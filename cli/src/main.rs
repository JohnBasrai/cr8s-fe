use anyhow::{anyhow, bail, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use once_cell::sync::Lazy;
use std::env;
use std::ffi::OsStr;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use strum_macros::Display;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

/// cr8s quickstart utility
#[derive(Parser)]
#[command(name = "quickstart", version, about = "Start or stop cr8s services")]
struct Cli {
    /// Override log level (e.g. error, warn, info, debug, trace)
    #[arg(long, value_enum)]
    log_level: Option<LogLevel>,

    /// Echo commands instead of running them
    #[arg(long)]
    dry_run: bool,

    /// Developer mode (base image from local docker registery)
    #[arg(long)]
    dev: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, Debug, ValueEnum, Display)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Subcommand)]
enum Commands {
    /// Start backend services and initialize the environment
    Start(StartOptions),

    /// Stop all containers and remove volumes
    Shutdown,

    /// Wait for frontend (trunk) to finish compiling
    Wait(WaitOptions),
}

#[derive(Args)]
struct WaitOptions {
    /// Timeout in seconds to wait for frontend readiness
    #[arg(long, value_name = "SECONDS", default_value_t = 60)]
    timeout: u64,
}

#[derive(Copy, Clone, Debug, ValueEnum, Display)]
#[clap(rename_all = "lower")] // ensures lowercase parsing
enum LintMode {
    // ---
    /// Skip all lint checks for fast startup
    None,

    /// Run cargo fmt + cargo clippy
    Basic,

    /// Run comprehensive lint checks (fmt + clippy + audit + outdated)
    Full,
}

#[derive(Args)]
#[command(
    about = "Start backend services and initialize the environment",
    long_about = None,
    after_help =
        r#"        If no flags are specified, performs a fast startup using cached Docker
        layers and existing containers.
        "#
)]
struct StartOptions {
    /// Lint mode to use
    #[arg(long)]
    lint: LintMode,

    /// Rebuild server without Docker cache (local images only)
    #[arg(long)]
    no_cache: bool,

    /// Pull base images from registry before building
    #[arg(long)]
    force_pull: bool,

    /// Recreate all containers (keep Docker cache)
    #[arg(long, conflicts_with = "fresh")]
    force_rebuild: bool,

    /// Nuclear option: shutdown + no-cache + force-pull + force-recreate
    #[arg(long, conflicts_with_all = ["force_rebuild", "force_pull", "no_cache"])]
    fresh: bool,
}

fn main() -> Result<()> {
    // ---
    let cli = Cli::parse();

    let filter = if let Some(level) = cli.log_level {
        println!(
            "🔧 Effective log level: {}",
            level.to_string().to_lowercase()
        );
        EnvFilter::new(level.to_string().to_lowercase())
    } else {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        println!(
            "🔧 Effective log level from RUST_LOG or default: {}",
            env_filter.to_string()
        );
        env_filter
    };

    tracing_subscriber::fmt().with_env_filter(filter).init();

    tracing::info!("cr8s-fe version: {}", *VERSION);

    setup_env_for_compose(cli.dev)?;

    match cli.command {
        // ---
        Commands::Start(opts) => start(opts, cli.dry_run),
        Commands::Shutdown => {
            info!("Stopping services...");
            info!("🧹 Cleaning up dev volumes...");

            run_shell("docker compose down -v", cli.dry_run)?;

            let target_path = CR8S_SCRATCH_DIR.join("dev-target");
            let cargo_path = CR8S_SCRATCH_DIR.join("dev-cargo");

            let cleanup_result = run_shell(
                format!(
                    "sudo rm -rf {:?}/dev-target {:?}/dev-cargo",
                    target_path, cargo_path
                )
                .as_str(),
                cli.dry_run,
            );

            if let Err(e) = cleanup_result {
                warn!(
                    "Failed to clean up {} dev volumes: {e:#}",
                    CR8S_SCRATCH_DIR.display()
                );
            }
            Ok(())
        }
        Commands::Wait(opts) => wait_for_frontend(opts.timeout, cli.dry_run),
    }
}

fn start(opts: StartOptions, dry_run: bool) -> Result<()> {
    // ---

    if opts.fresh {
        info!("🧨 Performing fresh startup...");
        run("docker", ["compose", "down", "-v"], dry_run)?;
    } else {
        info!("💬 Skipping docker compose down...");
    }

    run_lint_checks(&opts.lint, dry_run)?;

    {
        let no_cache = if opts.no_cache || opts.fresh {
            "--no-cache"
        } else {
            ""
        };
        let cr8s_version = get_cr8s_version();
        let fe_base_image = get_env_no_default("FE_BASE_IMAGE")?;
        let fe_server_image = get_env_no_default("FE_SERVER_IMAGE")?;

        info!(
            "🔨 Building cr8s-fe-server image with base:{}...",
            fe_base_image
        );

        run_shell(
            format!(
                "docker build {no_cache} \
                 --build-arg FE_BASE_IMAGE={fe_base_image} \
                 --build-arg CR8S_VERSION={cr8s_version} \
                 -f Dockerfile.fe-server \
                 -t {fe_server_image} \
                 ."
            )
            .as_str(),
            dry_run,
        )?;
    }

    if opts.fresh || opts.force_pull {
        info!("🐳 Pulling new images...");
        run_shell("docker compose pull postgres redis server", dry_run)?;
    }

    info!("🐳 Start all services...");
    run_shell("docker compose up -d", dry_run)?;

    info!("⏳ Waiting for services to be healthy...");
    run_shell("docker compose up --wait", dry_run)?;

    info!("🗄️  Loading database schema...");
    run_shell("docker compose run -q --rm cli load-schema", dry_run)?;

    info!("👤 Creating default admin user...");
    run_shell(
        "docker compose run -q --rm cli create-user \
        --username admin@example.com \
        --password password123 \
        --roles admin,editor,viewer",
        dry_run,
    )?;

    info!("🚀 Starting backend server...");
    run_shell("docker compose up -d server", dry_run)?;

    info!("✅ Backend services ready");
    info!("✅ Frontend trunk compile has started but is not complete!");
    Ok(())
}

fn wait_for_frontend(timeout_secs: u64, dry_run: bool) -> Result<()> {
    // ---

    info!("Waiting for frontend to be ready...");

    let timeout_ms = timeout_secs * 1000;
    run_shell(
        &format!(
            "npx wait-on http://localhost:8080 \
             --timeout {timeout_ms}  \
             --interval 2000 \
             --delay 1000 \
             --window 1000 \
             --verbose"
        ),
        dry_run,
    )?;
    Ok(())
}

fn setup_env_for_compose(is_dev: bool) -> Result<()> {
    // ---
    let rust_dev_image = get_env_with_default(
        "RUST_DEV_IMAGE",
        "ghcr.io/johnbasrai/cr8s/rust-dev:1.83.0-rev5",
    );

    tracing::info!("env: RUST_DEV_IMAGE={rust_dev_image}");
    tracing::info!("env: CR8S_VERSION={}", get_cr8s_version());

    let defaults = if !is_dev {
        // ---
        let be_cli_image = format!("ghcr.io/johnbasrai/cr8s/cr8s-cli:{}", get_cr8s_version());
        let be_server_image = format!("ghcr.io/johnbasrai/cr8s/cr8s-server:{}", get_cr8s_version());

        [
            ("RUST_DEV_IMAGE", rust_dev_image.clone()),
            ("FE_BASE_IMAGE", rust_dev_image),
            ("BE_CLI_IMAGE", be_cli_image),
            ("FE_SERVER_IMAGE", "cr8s-fe-server".to_string()),
            ("BE_SERVER_IMAGE", be_server_image),
        ]
    } else {
        [
            ("RUST_DEV_IMAGE", rust_dev_image.clone()),
            ("FE_BASE_IMAGE", rust_dev_image),
            ("BE_CLI_IMAGE", "cr8s-cli-dev".to_string()),
            ("FE_SERVER_IMAGE", "cr8s-fe-server".to_string()),
            ("BE_SERVER_IMAGE", "cr8s-server-dev".to_string()),
        ]
    };

    for (key, default) in defaults {
        // ---
        let value = match env::var(key) {
            Ok(value) => value,
            Err(_) => {
                env::set_var(key, default.clone());
                default
            }
        };
        tracing::info!("env: {}={}", key, value);
    }
    Ok(())
}

/// Logs the command to be run, with optional `(dry-run)` suffix.
///
/// Supports both: `program + args` or `one shell string`
#[macro_export]
macro_rules! log_command {
    // form: program + args
    ($dry_run:expr, $program:expr, $args:expr) => {{
        let args_vec: Vec<_> = $args.iter().map(|s| s.to_string()).collect();
        if $dry_run {
            tracing::info!("→ Running(dry-run): {} {}", $program, args_vec.join(" "));
        } else {
            tracing::info!("→ Running: {} {}", $program, args_vec.join(" "));
        }
    }};

    // form: single shell command string
    ($dry_run:expr, $cmd:expr) => {{
        if $dry_run {
            tracing::info!("→ Running(dry-run): {}", $cmd);
        } else {
            tracing::info!("→ Running: {}", $cmd);
        }
    }};
}

fn run<I, S>(program: &str, args: I, dry_run: bool) -> Result<ExitStatus>
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<OsStr> + std::fmt::Display,
{
    let args_vec: Vec<String> = args.clone().into_iter().map(|s| s.to_string()).collect();

    log_command!(dry_run, program, &args_vec);
    if dry_run {
        return Ok(ExitStatus::from_raw(0));
    }

    let status = Command::new(program).args(args).status()?;

    if !status.success() {
        error!(
            "❌ Command `{}` failed with exit code {:?}",
            program,
            status.code()
        );
        Err(anyhow::anyhow!(
            "Command `{}` failed with exit code: {:?}",
            program,
            status.code()
        ))
    } else {
        Ok(status)
    }
}

fn run_shell(command: &str, dry_run: bool) -> Result<ExitStatus> {
    // ---
    log_command!(dry_run, command);

    if dry_run {
        return Ok(ExitStatus::from_raw(0));
    }

    let status = Command::new("/bin/bash").arg("-c").arg(command).status()?;

    if !status.success() {
        error!("❌ Shell command failed with exit code {:?}", status.code());
        Err(anyhow::anyhow!(
            "Shell command failed with exit code: {:?}",
            status.code()
        ))
    } else {
        Ok(status)
    }
}

fn get_env_with_default(key: &str, def: &str) -> String {
    // ---
    match std::env::var(key) {
        Ok(value) => value.to_string(),
        Err(_) => def.to_string(),
    }
}

fn get_env_no_default(key: &str) -> Result<String> {
    // ---
    match std::env::var(key) {
        Ok(value) => Ok(value.to_string()),
        Err(e) => Err(anyhow!("get_env_no_default: {key}:{e}")),
    }
}

fn get_cr8s_version() -> String {
    // ---
    // This is the version of cr8s BE that we are targeting.
    get_env_with_default("CR8S_VERSION", "0.5.1")
}

fn extract_version_from_cargo_toml(path: &str) -> Result<String> {
    // --
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // ---
        let line = line?;
        if let Some(rest) = line.strip_prefix("version") {
            if let Some(ver) = rest.split('"').nth(1) {
                return Ok(ver.to_string());
            }
        }
    }

    bail!("❌ Could not extract version from {path}")
}

fn run_lint_checks(mode: &LintMode, dry_run: bool) -> Result<()> {
    // ---
    use std::fs::write;
    use std::os::unix::fs::PermissionsExt;

    let mut body = vec![
        "#!/bin/bash",
        "set -euo pipefail",
        "echo '🧹 Running lint checks...'",
    ];

    // All options do this (Ok, not none)
    body.push("cargo fmt --all -- --check");

    match mode {
        LintMode::None => return Ok(()), // no-op
        LintMode::Basic => {
            body.push("cargo clippy --all-targets -- -D warnings");
        }
        LintMode::Full => {
            body.push("cargo clippy --all-targets --all-features -- -D warnings");
            body.push("(cargo audit || true)");
            body.push("(cargo outdated || true)");
        }
    }

    let run_checks_file = "run-checks.sh";
    let script = body.join("\n") + "\n";
    write(&run_checks_file, script)?;
    std::fs::set_permissions(&run_checks_file, std::fs::Permissions::from_mode(0o755))?;

    // `setup_env_for_compose` already called so will not fail.
    let rust_dev_image = get_env_no_default("RUST_DEV_IMAGE")?;

    info!("Generating lint script: {}", run_checks_file);
    info!("Running lint checks using container: {}", rust_dev_image);

    let target_path = CR8S_SCRATCH_DIR.join("dev-target");
    let cargo_path = CR8S_SCRATCH_DIR.join("dev-cargo");

    let result = run_shell(
        format!(
            r#"docker run --rm -u root -v "$PWD:$PWD" \
               -v {:?}/dev-target:/app/target \
               -v {:?}/dev-cargo:/usr/local/cargo/registry \
               -w $PWD {rust_dev_image} \
               ./{run_checks_file}"#,
            target_path, cargo_path
        )
        .as_str(),
        dry_run,
    );
    run_shell(format!("cat {run_checks_file}").as_str(), false)?;
    let _status = std::fs::remove_file(run_checks_file); // always attempt cleanup
    result.map(|_status| ())
}

pub static VERSION: Lazy<String> = Lazy::new(|| {
    // ---
    extract_version_from_cargo_toml("Cargo.toml").unwrap_or_else(|e| format!("unknown ({e})"))
});

pub static CR8S_SCRATCH_DIR: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("CR8S_SCRATCH_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/var/tmp"))
});
