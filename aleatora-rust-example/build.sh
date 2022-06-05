#!/bin/sh
wasm-pack build --out-name main --target no-modules --no-typescript
cat post.js >> pkg/main.js

