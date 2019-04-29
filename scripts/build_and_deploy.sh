#!/usr/bin/env bash

set -x

INITIAL_DIR=$(realpath $(dirname $0)/..)

cargo-web deploy --release
du -h ./target/deploy/radial_dots_yew.wasm

wasm-opt -Oz \
    -o ./target/deploy/radial_dots_yew_compressed.wasm \
    ./target/deploy/radial_dots_yew.wasm

mv ./target/deploy/radial_dots_yew_compressed.wasm ./target/deploy/radial_dots_yew.wasm
du -h ./target/deploy/radial_dots_yew.wasm

!/usr/bin/env bash
if [ -z "$(git status --porcelain)" ]; then 
    echo "working clean. deploying"

    cd $(mktemp -d)

    git init
    git remote add origin git@github.com:Adjective-Object/radial-dots-yew.git
    git checkout -b gh-pages
    cp "$INITIAL_DIR"/target/deploy/* .
    git add .
    git commit -m "auto build"
    git status
    tree
    git push --force origin gh-pages

else 
    echo "working dir not clean -- not deploying"
    exit 1
fi
