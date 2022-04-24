# wasm-snake-game: Slide 7

Update snake. Just head for now :-)

On the Rust side:

```rust
#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        World { 
            width: WORLD_WIDTH,
            snake: Snake::new(10),
        }
    }

...

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        self.snake.body[0].0 = (snake_idx + 1) % (self.width * self.height);
    }
}

```

Call the wasm update() from Javascript
```js
    setInterval(() => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawWorld();
        drawSnake();

        world.update();
    }, refresh_rate);
```

