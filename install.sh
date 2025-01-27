#!/usr/bin/env bash

set -xe

# Check cargo is installed
if ! command -v cargo &> /dev/null
then
  echo "cargo is required to install buddai"
  exit
fi

cargo install --path .
echo "buddai installed successfully"
