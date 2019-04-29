#!/usr/bin/env bash

set -x

INITIAL_DIR=$(dirname $0)/..

cargo-web deploy --release
du -h ./target/deploy/radial_dots_yew.wasm

wasm-opt -Oz \
    -o ./target/deploy/radial_dots_yew_compressed.wasm \
    ./target/deploy/radial_dots_yew.wasm

mv ./target/deploy/radial_dots_yew_compressed.wasm ./target/deploy/radial_dots_yew.wasm
du -h ./target/deploy/radial_dots_yew.wasm

#!/usr/bin/env bash
if [ -z "$(git status --porcelain)" ]; then 
    echo "working clean. deploying"

    DIR=$(mktemp -d)
    git clone $INITIAL_DIR $DIR
    cd $DIR

    git branch -D gh-pages || true
    git checkout -b gh-pages
    git push --force origin gh-pages

else 
    echo "working dir not clean -- not deploying"
    exit 1
fi
