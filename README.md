# wasm-snake-game: Slide 8

RECAP

- src/lib.rs

```rust
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const WORLD_WIDTH: usize = 8;
const WORLD_HEIGHT: usize = 8;

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    fn new(spawn_index: usize) -> Self {
        Snake{
            body: vec!(SnakeCell(spawn_index)),
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    height: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        World { 
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize { 
        self.height 
    }
    
    pub fn dim(&self) -> usize {
        self.width * self.height
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        self.snake.body[0].0 = (snake_idx + 1) % self.dim();
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

```

- www/index.js

```js
import init, { World } from "../pkg/snake_game.js";

init().then(_ => {
    const CELL_SIZE = 20;
    const refresh_rate = 100;

    const world = World.new();
    const world_width = world.width(); // avoid back and forth js-rust

    const canvas = document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");
   
    canvas.height = world_width * CELL_SIZE;
    canvas.width = world_width * CELL_SIZE; // world_widht: bad name dim would be better

    function drawWorld() {
        ctx.beginPath();

        // mk column: mv on the y axis
        for (let y=0; y <= canvas.height; y+=CELL_SIZE) {
            ctx.moveTo(y, 0);
            ctx.lineTo(y, canvas.height ); 
        }
        
        // mk row: mv on the x axis
        for (let x=0; x <= canvas.width; x+=CELL_SIZE) {
            ctx.moveTo(0, x);
            ctx.lineTo(canvas.width, x); 
        }
        

        ctx.stroke();
    }
    

    function drawSnake() {
        const snake_idx = world.snake_head_idx();
        const col = snake_idx % world_width;
        const row = Math.floor(snake_idx / world_width);
        
        ctx.beginPath();

        ctx.fillRect(
            col * CELL_SIZE, 
            row * CELL_SIZE, 
            CELL_SIZE, CELL_SIZE);

        ctx.stroke();
    }

    drawWorld();
    drawSnake();

    setInterval(() => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawWorld();
        drawSnake();
        world.update();

    }, refresh_rate);
})

```

- index.html

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="generator" content="pandoc" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes" />
  <title>index</title>
  <link rel="stylesheet" href="snake-game.css" />
</head>
<body>
    <div class="content-wrapper">
        <canvas id="snake-canvas"></canvas>
    </div>

 <script src="./bootstrap.js"></script>
</body>
</html>

```

- snake-game.css

```css
.content-wrapper {
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    position:absolute;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
}

.game-panel {
    margin-bottom: 20px;
}

.flex {
    display: flex;
}

.label {
    font-weight: bold;
    margin-right: 10px;
}
```

- Cargo.toml

```toml
[package]
name = "snake_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
wasm-bindgen = "0.2.63"
wee_alloc = "0.4.5"

[lib]
crate-type = ["cdylib"]

[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```


