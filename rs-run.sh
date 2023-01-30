#!/bin/bash

cargo build --release
time for i in $(seq 0 100); do ./target/release/rust-vs-cpp 12; done
