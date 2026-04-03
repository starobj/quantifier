#!/usr/bin/env bash

# Clear STDOUT.
clear

# Build the crate.
cargo build

# Test the crate.
cargo test

# Clear STDOUT.
clear

# Run the lexer on a sample.
cat samples/sum.sk | ./target/debug/sklex
