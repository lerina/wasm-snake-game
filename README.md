# wasm-snake-game Slide 2

1. Using wee_alloc: The Wasm-Enabled, Elfin Allocator.

> wee_alloc is focused on targeting WebAssembly, producing a small .wasm code size, and having a simple, correct implementation. It is geared towards code that makes a handful of initial dynamically sized allocations, and then performs its heavy lifting without any further allocations

SOURCE: [wee_alloc](https://github.com/rustwasm/wee_alloc)

2. isolating the html from the wasm for better debug experience

Previously in index.html
```html
<body>
<script type="module">
  import init, { World } from './pkg/snake_game.js';

  async function run() {

    await init();

    const world = World.new();
    console.log("width: " + world.width);
  }

  run();
</script>
</body>
```
now in index.html

```html
<body>

<script src="./bootstrap.js"></script>
</body>
```

Errors can be better traced  by adding some level of indirection :-)

```js
import("./index.js")
  .catch(e => console.error("Error importing index.js :", e))
```

Finally we reach the code that loads the wasm file
using init.then() rather than async/await

```js
import init, { World } from "../pkg/snake_game.js";

init().then(_ => {
    const world = World.new();
    console.log("from index.js: " + world.width);
})

```
