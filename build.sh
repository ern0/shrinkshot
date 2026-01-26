#!/bin/bash
clear

BASE=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $BASE/v2-rust

cargo build
