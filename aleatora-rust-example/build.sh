#!/bin/sh
wasm-pack build --out-name main --target no-modules --no-typescript &&
cat post.js >> pkg/main.js &&
../../alternator/bundle.py "" assets/ &&
mv bundles/assets/* pkg &&
rmdir bundles/assets bundles

