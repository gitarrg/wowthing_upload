#!/bin/bash

cargo.exe build --release --target-dir "D:\rust_out"
cp "/mnt/d/rust_out/release/wowthing_upload_rust.exe" .
