#!/bin/bash
# This is an installation script for auto_dr
if [[ $EUID -ne 0 ]]; then
  echo "This script must be run as root." >&2
  exit 1
fi

if ! [ -x "$(command -v cargo)" ]; then
  echo 'You need to have cargo installed to run this script.' >&2
  exit 1
fi

echo "Pulling newest version." &&
  git pull &&
  echo "Building from source." &&
  cargo build --release &&
  echo 'Copying executable to "/usr/bin/".' &&
  sudo cp ./target/release/auto_dr /usr/bin/core_perl/ &&
  echo "Removing compilation artifacts." &&
  sudo rm -rf ./target &&
  echo "Installation of auto_dr finished."
