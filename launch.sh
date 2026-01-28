#!/bin/bash
clear

BASE=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

$BASE/v2-rust/target/debug/shrinkshot \
    $BASE/test/a1-*.png \
    $BASE/test/a3-result.png

open $BASE/test/a3*.png
