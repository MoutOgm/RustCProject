#!/bin/bash
cd ./rc_lib
echo "-- building lib"
cargo build --release
echo "-- copying lib"
if [ ! -d "../lib" ]; then
    mkdir ../lib
fi
cp ./target/release/librc.a ../lib/
echo "-- create header.hpp"
cbindgen --config ../cbindgen.toml --crate rc --output ../cproject/includes/lib/rclib.hpp
cd ../
cmake ./
make
if [ $# -ne 0 ] && [ "$1" = "run" ]; then
    ./main
fi