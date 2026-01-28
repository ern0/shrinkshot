#!/bin/bash
clear

BASE=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

TEST=f

$BASE/target/debug/shrinkshot \
    $BASE/test/${TEST}1-*.png \
    $BASE/test/${TEST}3-result.png

open $BASE/test/${TEST}3*.png
