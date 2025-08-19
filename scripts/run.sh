#!/bin/bash

# Assign the first argument to FILE_NAME.
# If no argument is provided, use "rust-learning" as the default.
FILE_NAME=${1:-rust-learning}

echo "Running the binary: $FILE_NAME"

# Use the --bin flag to specify which binary to run.
cargo run --bin $FILE_NAME