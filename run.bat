@echo off
cls
cargo build
copy ".\\target\\debug\\excalibur.exe" ".\\run\\"
cd run
excalibur.exe