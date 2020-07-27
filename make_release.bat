@echo off
cargo build --release
copy ".\\target\\release\\excalibur.exe" "..\\"