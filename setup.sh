#!/bin/bash

echo "========================="
echo "=== Updating packages ==="
echo "========================="

sudo apt update && sudo apt upgrade -y

echo "===================================================="
echo "=== Installing needed packages to run the system ==="
echo "===================================================="

echo "[*] Installing nodejs"
curl -sL https://deb.nodesource.com/setup_14.x | sudo -E bash -
sudo apt install -y nodejs

echo "[*] Installing rustc"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo "[*] Installing build dependencies for serial tty"
sudo apt install -y pkg-config libudev-dev

echo "============================================"
echo "=== Building first time all dependencies ==="
echo "============================================"

echo "[*] Installing npm packages"
pushd "$PWD/frontend"
npm i

echo "[*] Building all"
pushd -1
./install.sh
