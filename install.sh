#!/bin/bash

cargo build --release --target-dir=target/

function get_abs_path() {
    echo "$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
}

function create_link() {
    local abs_path=$(get_abs_path "$1")
    chmod +x $abs_path
    ln -si "$abs_path" /usr/local/bin/pomo
}

create_link ./target/release/pomo

rm -rf ./target
