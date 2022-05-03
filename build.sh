#!/bin/sh

set -ex

wasm-pack build --target web

cp -fr pkg www/

# tsc --module ES6 --target ES6 www/index.ts

# using config file in www/tsconfig.json
tsc -p ./www/


printf '%s\n' "serving page at: http://127.0.0.1:8080"
#python3 -m http.server

http -a 127.0.0.1 -p 8080 www/
