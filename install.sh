#!/bin/bash
set -e

REPO="credkellar-boop/insta-repo"
echo "[*] Fetching latest release of Insta-Repo..."

# Get latest release data from GitHub API
LATEST_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url.*linux-amd64" | cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
    echo "[-] Could not find a suitable binary for your architecture."
    exit 1
fi

echo "[*] Downloading $LATEST_URL..."
curl -sL $LATEST_URL -o insta-repo

chmod +x insta-repo
sudo mv insta-repo /usr/local/bin/

echo "[+] Insta-Repo installed successfully to /usr/local/bin/insta-repo!"
