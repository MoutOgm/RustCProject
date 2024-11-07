#!/bin/bash
cd ./rc_lib
echo "-- building lib"
cargo build --all --release
cargo run --release -p rc
echo "-- copying *.a"
if [ ! -d "../lib" ]; then
    mkdir ../lib
fi
cp -v ./target/release/*.a ../lib/
echo "-- cp *.hpp"
cp -r ./target/release/lib ../cproject/includes
cd ../
if [ $# -ne 0 ] && [ "$1" = "run" ] && [ -f "./main" ]; then
    echo "-- remove old main"
    rm ./main
fi
echo "-- cmake"
cmake ./
make
if [ $# -ne 0 ] && [ "$1" = "run" ]; then
    if [  -f "./main" ]; then
        ./main
        exit 1
    fi
    echo "Compilation echouée"
fi