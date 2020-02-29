#!/bin/zsh

## Remember to add target if you haven't
# rustup target add rustup target add

## Dependencies
# brew install openssl

OPENSSL_DIR=/usr/local/opt/openssl@1.1 cargo build --target=x86_64-unknown-linux-gnu --release