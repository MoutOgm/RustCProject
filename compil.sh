#!/bin/bash

# Function to check if the script is running on Windows
is_windows() {
    case "$(uname -s)" in
        CYGWIN*|MINGW32*|MSYS*|MINGW*)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

cd ./rc_lib
echo "-- building lib"
cargo build --all --release
echo "-- copying *.a"

if [ ! -d "../lib" ]; then
    mkdir ../lib
fi

if is_windows; then
    cp -v ./target/release/*.lib ../lib/
else
    cp -v ./target/release/*.a ../lib/
fi

echo "-- cp *.hpp"
cp ./target/cxxbridge/utils.h ../cproject/includes/lib

cd ../

if [ $# -ne 0 ] && [ "$1" = "run" ] && [ -f "./main" ]; then
    echo "-- remove old main"
    rm ./main
fi

echo "-- cmake"
cmake ./
make

if [ $# -ne 0 ] && [ "$1" = "run" ]; then
    if [ -f "./main" ]; then
        ./main
        exit 1
    fi
    echo "Compilation echouée"
fi