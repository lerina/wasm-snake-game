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

    fn index_to_xy(&self, idx: usize) -> (usize, usize) {
        //  x            ,     y
        (idx % self.width, idx / self.width)
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        // WHOLE ROWS    + remaining COLUMNS gives index
        (y * self.width) + x
    }

    pub fn update(&mut self) {
...
```

- decluter Update

```rust
    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (x, y) = self.index_to_xy(snake_idx);

        let (new_x, new_y) = match self.snake.direction {
            Direction::Right => {
                ((x + 1) % self.width, y)
            },
            Direction::Left => {
                ((x - 1) % self.width, y)
            },
            Direction::Up => {
                (x, (y - 1) % self.width)
            },
            Direction::Down => {
                (x, (y +1) % self.width)
            },
        };

        let next_idx = self.xy_to_index(new_x, new_y);
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
