#!/bin/bash

FILE_NAME=${1:-rust-learning}

echo "creating new file: $FILE_NAME"

touch "./src/bin/$FILE_NAME.rs"

cat "./src/bin/hello_world.rs" >> "./src/bin/$FILE_NAME.rs"