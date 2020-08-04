@echo off
cargo build --release
copy ".\\target\\release\\excalibur.exe" ".\\release"
copy ".\\config.toml" ".\\release"
copy ".\\.env" ".\\release"