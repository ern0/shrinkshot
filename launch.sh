#!/bin/bash
clear

BASE=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

$BASE/target/debug/shrinkshot \
    $BASE/test/f1-*.png \
    $BASE/test/f3-result.png

open $BASE/test/f3*.png
