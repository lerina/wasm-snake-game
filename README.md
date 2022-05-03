# wasm-snake-game: Slide 16
## Refactor update


- extract the direction logic from Update into a new method `gen_next_snake_cell()`

```rust
    fn gen_next_snake_cell(&self) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

...

        }
```

- avoid using costly modulo operations and manage the warping around the world bounds

```
         match self.snake.direction {
            Direction::Right => {
                let right_bound = (row + 1) * self.width;
                if snake_idx + 1 == right_bound {
                    SnakeCell(right_bound - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            },
          Direction::Left => {
                let left_bound = row * self.width;
                if snake_idx == left_bound {
                    SnakeCell(left_bound + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            },
            Direction::Up => {
                let upper_bound = snake_idx - (row * self.width);
                if snake_idx == upper_bound {
                    SnakeCell((self.size - self.width) + upper_bound)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            },
            Direction::Down => {
                let lower_bound = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == lower_bound {
                    SnakeCell(lower_bound - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
```

- rename update() to step()and use the SnakeCell returned by gen_next_snake_cell()
```rust
    pub fn step(&mut self) {
        let next_cell = self.gen_next_snake_cell();
        self.snake.body[0] = next_cell;
    }
```

- finally rename update to step in index.ts
Change world.update() to world.step()

```ts
function update() {
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            world.step();
            draw_all();
            
            requestAnimationFrame(update);
        }, 1000 / fps);
    }
```
