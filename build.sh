#!/bin/bash
echo "compiling Rust dynamic lib"
rm ./lib/hello.h && rm ./lib/libhello.so
cd ./lib/hello || exit
cargo build --release
echo "creating header file"
cbindgen --config cbindgen.toml --crate hello --output hello.h --lang c
mv hello.h ./../
mv ./target/release/libhello.so ./../
