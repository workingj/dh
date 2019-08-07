#!/bin/bash
cargo b --release
cp $CARGO_TARGET_DIR/release/dh ~/.jbtools/dh/
