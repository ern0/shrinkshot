#!/bin/bash
clear

BASE=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

$BASE/v2-rust/target/debug/shrinkshot \
    $BASE/test/d1-*.png \
    $BASE/test/d3-result.png

# open $BASE/test/d3*.png
