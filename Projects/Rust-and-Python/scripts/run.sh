#!/bin/bash

cd ./rust_lib || exit

# build python
cargo build --release

cd .. || exit

python python_app/app.py
