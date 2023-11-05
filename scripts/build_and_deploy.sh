#!/usr/bin/env bash

set -x

INITIAL_DIR=$(realpath $(dirname $0)/..)

# TODO: cargo-web is deprecated. Use trunk build here instead
trunk build --release
du -h ./dist/radial_dots.wasm

wasm-opt -Oz \
    -o ./dist/radial_dots_compressed.wasm \
    ./dist/radial_dots.wasm

mv ./dist/radial_dots_compressed.wasm ./dist/radial_dots.wasm
du -h ./dist/radial_dots.wasm

if [ -z "$(git status --porcelain)" ]; then 
    echo "working clean. deploying"

    cd $(mktemp -d)

    git init
    git remote add origin git@github.com:Adjective-Object/radial-dots-yew.git
    git checkout -b gh-pages
    cp "$INITIAL_DIR"/dist/* .
    git add .
    git commit -m "auto build"
    git status
    tree
    git push --force origin gh-pages

else 
    echo "working dir not clean -- not deploying"
    exit 1
fi
