# wasm-snake-game: Slide 4

Build the snake

Instead of having a meaningless vector of usize, a little inderection here
will make the code more pleasing to the eyes.

```rust
struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}
```
Next we make a "constructor" for the Snake
```rust
impl Snake {
    fn new(spawn_index: usize) -> Self {
        Snake{
            body: vec!(SnakeCell(spawn_index)),
        }
    }
}
```

Update the world

```rust
#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        World { 
            width: WORLD_WIDTH,
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
```

and add a getter for the Snake's head

```rust
    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }
}
```
