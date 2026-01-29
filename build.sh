#!/bin/bash
clear

BASE=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $BASE

rm -f $BASE/target/release/shrinkshot
cargo build --release
if [ -f $BASE/target/release/shrinkshot ]; then
    cp $BASE/target/release/shrinkshot /usr/local/bin
fi
