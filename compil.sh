#!/bin/bash
cd ./rc_lib
echo "-- building lib"
if [ ! -d "./target/release/lib" ]; then
    mkdir target/release/lib
fi
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
echo "-- cmake"
cmake ./
make
if [ $# -ne 0 ] && [ "$1" = "run" ]; then
    ./main
fi