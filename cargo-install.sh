#!/usr/bin/env bash
CFLAGS=${CFLAGS:-"-Os -march=native -mtune=native"}
CXXFLAGS=${CXXFLAGS:-"-Os -march=native -mtune=native"}
RUSTFLAGS=${RUSTFLAGS:-"-C target-cpu=native"}

export CFLAGS CXXFLAGS RUSTFLAGS

cargo install --path .
