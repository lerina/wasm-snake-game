# wasm-snake-game: Slide 12
We are going to randomly spawn the snake in the World.  

There are many ways to do this:
- We can use Rand or js_sys to access Javascript's Math.random 
- Or we can directly expose Math.random to Rust ourself without add new crates.

## Using Rand in WebAssembly

Under the hood, the rand crate uses the operating system's native random functions to seed the random number generator.   
This initial entropy seed comes from the hardware noise. 
However, when Rust programs inside virtual machines like WebAssembly, 
it does not have access to native random hardware.

When the rand crate is compiled for WebAssembly, 
it has a special feature to use JavaScript's random function to seed the generator. 
This works when your WebAssembly application is running inside a JavaScript host, 
such as in a web browser or in Node.js. To enable that, do the following in your Cargo.toml.

```toml
[dependencies]
rand = "0.8.5"
getrandom = { version = "0.2", features = ["js"] }
```

[Get random number](https://www.secondstate.io/articles/wasi-access-system-resources/)
[source](https://rust-by-example-ext.com/rand.html)  
[The Rust Rand Book](https://rust-random.github.io/book/intro.html)  

Adding two crates (+ dependencies) just to get access to one method is not very fun.  
So we'll pass on the rand crate.

## Using js_sys

- add js-sys dependency in Cargo.toml

```toml
[dependencies]
wasm-bindgen = "0.2.63"
wee_alloc = "0.4.5"
js-sys = "0.3.57"
```

- make it available

```
use js_sys::Math;

... 
    
#[wasm_bindgen]
impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let snake_start_idx = Math::random() * (width * height) as f64;
        World { 
            width,
            height,
            snake: Snake::new(snake_start_idx as usize),
        }
    }
...
```

At this point we dont need anything other that the random method. 
To add a dependency  just for random may be overkill.   
So again, we'll pass on the js-sys crate for now.

## Making Math.random available to our Rust-wasm code

Javascript should be limited to the rendering as much as possible, 
but running code inside a browser does not play well with accessing system 
resources for obvious security reasons.  
So relying on javascript's Math.random() makes sense for our purpose.

in ./utils/rnd.ts

```ts
export function rnd(max: number) : number {
    return Math.floor(Math.random() * max);
}
```
in ./index.ts we make rnd() available to wasm by importing it

```js
import init, { World } from "./pkg/snake_game.js";
import {rnd} from "./utils/rnd.ts";

init().then((wasm) => {
    ...
    const WORLD_WIDTH = 8;
    const WORLD_HEIGHT = 8;
    const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH)
    
    ...
    
    update();
})

```
 in ../src/lib.rs

```rust
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

...

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}
...

```
Finally let the typescript compiler know about our new rnd.ts

```json
{
  "compilerOptions": {
    "target": "ES6",
    "module": "ES6",
    "esModuleInterop": true,
  },
  "files": [
    "index.ts",
    "utils/rnd.ts",
  ],
  "$schema": "https://json.schemastore.org/tsconfig",
  "display": "Recommended"
}
```

 



