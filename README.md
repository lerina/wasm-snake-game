# wasm-snake-game: Slide 14

## Refactoring the direction code

- first some helpers
```rust
#[wasm_bindgen]
impl World {
...
    
    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        //  COLUMN       ,     ROW
        (idx / self.width, idx % self.height)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        // WHOLE ROWS + remaining COLUMNS
        (row * self.width) + col
    }

    pub fn update(&mut self) {
...
```

- decluter Update

```rust
    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (row, col) = self.index_to_cell(snake_idx);

        let (new_row, new_col) = match self.snake.direction {
            Direction::Right => {
                (row, (col + 1) % self.width)
            },
            Direction::Left => {
                (row, (col - 1) % self.width)
            },
            Direction::Up => {
                ((row - 1) % self.width, col)
            },
            Direction::Down => {
                ((row +1) % self.width, col)
            },
        };

        let next_idx = self.cell_to_index(new_row, new_col);
        self.set_snake_head(next_idx);
    } //^-- update()
...
```

## fixing a bug in index.ts
- First we group the various drawing 
```ts
    
    function draw_all(){
        drawWorld();
        drawSnake();
    }
```
- then we fix the bug caused by drawing before updating the world which
had the adverse effect when changing direction at the borders of the world.

```ts    
    function update() {
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            world.update();
            draw_all();
            
            requestAnimationFrame(update);
        }, 1000 / fps);
    }
    
    update();
```
