#!/bin/bash
echo "build release & copy binary & cargo clean"
name="$(pwd)"					# get the current path as variable
name="${name%"${name##*[!/]}"}" # extglob-free multi-trailing-/ trim
name="${name##*/}"              # remove everything before the last /
cargo build --release
strip $CARGO_TARGET_DIR/release/$name
sudo cp $CARGO_TARGET_DIR/release/$name /usr/local/bin/ -v # copy the binary to th tools folder
sudo chmod u+x '/usr/local/bin/dh'
# cp $CARGO_TARGET_DIR/release/$name ~/.jbtools -v # copy the binary to th tools folder
# cargo clean
