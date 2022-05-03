# wasm-snake-game: Slide 17

## Body follows the head

- add Clone trait to SnakeCell

```rust
#[derive(Copy, Clone)]
pub struct SnakeCell(usize);

```

- After moving the head, shift each other cells to the one in front of it.

```rust
    pub fn step(&mut self) {
        let tmp = self.snake.body.clone();
        let len = self.snake.body.len();

        // Set head new pos
        let next_cell = self.gen_next_snake_cell();
        self.snake.body[0] = next_cell;

        // Shift body
        for i in 1..len {
            self.snake.body[i] = SnakeCell(tmp[i - 1].0);
        }
    }
```
