# wasm-snake-game: Slide 18

## Prevent the snake from doing a 180°

- gen_next_snake_cell has a new parameter `direction: &Direction`

```rust
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

         match direction {
            Direction::Right => {
                let right_bound = (row + 1) * self.width;
                if snake_idx + 1 == right_bound {
                    SnakeCell(right_bound - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            },
            
            ...

```

- update change_snake_dir()

```rust
    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);

        // can't 180
        if self.snake.body[1].0 == next_cell.0 { return;}

        self.snake.direction = direction;
    }
```

- and step()

```rust
    pub fn step(&mut self) {
        let tmp = self.snake.body.clone();
        let next_cell = self.gen_next_snake_cell(&self.snake.direction);

    ...
```
