#!/bin/sh

wat=./example.wat

export ENV_WAT_PATH="${wat}"

./target/release/basic_cpuinfo
