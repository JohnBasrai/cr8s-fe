## Docker login with a PAT

1. **Create a GitHub PAT with `read:packages` scope:**

   * Go to [https://github.com/settings/tokens](https://github.com/settings/tokens)
   * Click **Generate new token (classic)**
   * (You may be asked to authenticate using your password, a 2FA code, or passkey)
   * Give the token a descriptive name (e.g., `Docker GHCR access`)
   * Set expiration as you prefer
   * Check the box for `read:packages` (minimum for pulling packages)
   * (Optional) Check `repo` if you also want repository access
   * Generate and **copy the token immediately** — you won’t see it again

2. **Log in to GHCR with Docker:**

```bash
docker logout ghcr.io  # clear any existing credentials
docker login ghcr.io
# When prompted:
# Username: your GitHub username (e.g., JohnBasrai)
# Password: paste your PAT token (not your GitHub password)
