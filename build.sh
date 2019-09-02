#!/bin/bash
echo "build release & copy binary & cargo clean"
name="$(pwd)"					# get the current path as variable
name="${name%"${name##*[!/]}"}" # extglob-free multi-trailing-/ trim
name="${name##*/}"              # remove everything before the last /
cargo build --release
cp $CARGO_TARGET_DIR/release/$name ~/.jbtools/dh -v # copy the binary to th tools folder
# sudo cp target/release/$name /usr/local/bin/ -v # copy the binary to th tools folder
# cargo clean
