# Wowthing Uploader written in Rust

## Compile on Linux

```sh
cargo build --target-dir "dist/linux/"
```

## Compile on Windows

due to WSL vs. Windows issues we have to target a windows native output folder
when building:

```sh
cargo.exe build --target-dir "D:\rust_out"
```

## Run

```bash
D:\rust_out\debug\wowthing_upload_rust.exe --config "D:\wow\wowthing_upload_rust\config.toml"
```
