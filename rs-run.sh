#!/bin/bash

cargo build --release
time ./target/release/rust-vs-cpp 12
