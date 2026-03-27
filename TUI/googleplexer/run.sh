#!/bin/bash

echo -e "${GREEN}[!]${NC}${BOLD} initing ${NC}"
DESABLE= false
GREEN='\033[1;32m'
RED='\033[1;31m'
BOLD='\033[1m'
NC='\033[0m'
case $1 in
    -t)
        echo -e "${GREEN}[+]${NC}${BOLD} cleaning files${NC}"
        cmake clean
        echo -e "${GREEN}[+]${NC}${BOLD} runnning tests${NC}"
        cmake test  || echo -e "${RED}[<!>]${BOLD} ${BOLD}an error happened${NC}"
        exit
    ;;
    -b)
        echo -e "${GREEN}[+]${NC}${BOLD} cleaning files${NC}"
        cmake clean
        echo -e "${GREEN}[+]${NC}${BOLD} building tree${NC}"
        cmake build 
        exit
    ;;
    -d)
        echo -e "${GREEN}[+]${NC}${BOLD} desabled extras${NC}"
        DESABLE= true
    ;;
    --tree)
        echo -e "${GREEN}[+]${NC}${BOLD} treeing with depth level 3${NC}"
        tree -L 3
        exit
    ;;
    --cat) 
        echo -e "${GREEN}[+]${NC}${BOLD} cating all files in current directory${NC}"
        cat *
        echo "\n"
        exit
    ;;
esac

START_TIME=$SECONDS
echo -e "\n  ██▄████  ${GREEN}██    ██  ██▄████▄  ██▄████▄   ▄████▄    ██▄████ ${NC}" 
echo -e "  ██▀      ${GREEN}██    ██  ██▀   ██  ██▀   ██  ██▄▄▄▄██   ██▀     ${NC}" 
echo -e "  ██       ${GREEN}██    ██  ██    ██  ██    ██  ██▀▀▀▀▀▀   ██      ${NC}" 
echo -e "  ██       ${GREEN}██▄▄▄███  ██    ██  ██    ██  ▀██▄▄▄▄█   ██      ${NC}" 
echo -e "  ▀▀        ${GREEN}▀▀▀▀ ▀▀  ▀▀    ▀▀  ▀▀    ▀▀    ▀▀▀▀▀    ▀▀      ${NC}" 
echo "for Googleplexer (GPX)"

echo -e "${RED}[!]${NC}${BOLD} deleting folder 'build'${NC}"
rm -rf build

echo -e "${GREEN}[+]${NC}${BOLD} building tree${NC}"
cmake -B build
cmake --build build
if ! cmake build ; then
    echo -e "${RED}[<!>]${NC}${BOLD} error: build failed${NC}"
    fahh
    exit 1
else
    echo -e "${GREEN}[+]${NC}${BOLD} seccess${NC}"
fi

DURATION=$(( SECONDS - START_TIME ))
echo -e "${GREEN}[+]${NC}${BOLD} executed in ${DURATION}s ${NC}"
sleep 0.1

echo -e "${GREEN}[+]${NC}${BOLD} running${NC}"
echo -e "${BOLD}─┤ output ├────────────────────────────────────────────────────────────────────────────────────────────────────────────────────${NC}\n"
./build/googleplexer
echo -e "\n\n${BOLD}───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────${NC}"
sleep 1
if [ $DESABLE ]; then
    echo -e "${GREEN}[+]${NC}${BOLD} do you want to clean [c] or test [t]${NC}" 
    read debig
        if [ $debig == "c" ]; then
        cmake clean
        clear
        exit

    elif [ $debig == "t" ]; then
        cmake clear
        cmake test
        exit    
    else
        echo -e "${GREEN}[+]${NC}${BOLD} ok :D${NC}"
        exit 
    fi
fi