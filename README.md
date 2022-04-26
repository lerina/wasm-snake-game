# wasm-snake-game: Slide 10

Testing plain vanilla typescript

- Rename index.js and make a single modification by add type for the canvas element

```ts
import init, { World } from "../pkg/snake_game.js";

init().then(_ => {
    const CELL_SIZE = 20;
    const refresh_rate = 100;

    const world = World.new();
    const world_width = world.width(); // avoid back and forth js-rust

    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
 ...

```

- create a config file for ts compiler for our convinience

```json
{
  "compilerOptions": {
    "target": "ES6",
    "module": "ES6",
    "esModuleInterop": true,
  },
  "files": [
    "index.ts",
  ],
  "$schema": "https://json.schemastore.org/tsconfig",
  "display": "Recommended"
}
```

- update our build script

```bash
#!/bin/sh

set -ex

wasm-pack build --target web


# tsc --module ES6 --target ES6 www/index.ts

# using config file in www/tsconfig.json
tsc -p ./www/


printf '%s\n' "serving page at: http://127.0.0.1:8080"
#python3 -m http.server

cp -fr pkg www/
http -a 127.0.0.1 -p 8080 www/
```


