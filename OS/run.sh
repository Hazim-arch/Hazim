#!/bin/bash

START_TIME=$SECONDS

clear

echo -e "${GREEN}$(toilet -f mono12 runner)${NC}"
echo "for LunerOS"

echo "[+] initng"
GREEN='\033[1;32m'
BOLD='\033[1m'
NC='\033[0m'

echo "${BOLD}[+]${NC} cleaning files"
cargo clean

echo "${BOLD}[+]${NC}building tree"
cargo build
if ! cargo build; then
    echo "[<!>] error: build failed"
    fahh
    exit 1
else
    echo "${GREEN}[+]${NC} seccess"
fi

echo "${BOLD}[+]${NC} building costome kernal triple"
cargo build --target x86_64-LunerOS.json

echo "[+] making bootimage"
cargo bootimage

DURATION=$(( SECONDS - START_TIME ))
echo "[+] executed in $DURATION s"

echo '[+] running'

qemu-system-x86_64 -drive format=raw,file=target/x86_64-LunerOS/debug/bootimage-LunerOS.bin -serial stdio