# wasm-snake-game: Slide 15

## Add a body to the snake

- Update our snake to have a body

```rust
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i)); 
        }

        Snake{
            body,
            direction: Direction::Right,
        }
    }
}
```
- helper to get body length

```rust
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }
```

- Expose the snake body vector to JS as a ref 

This is what we want

```
    pub fn snake_body(&self) -> &Vec<SnakeCell> {
      &self.snake.body
    }
```

But we can't return a ref to Js (borrow can't be checked so its not allowed)
the error message is `cannot return a borrowed ref with #[wasm_bindgen]`

- The solution is to use a raw pointer. We are telling the Borrow checker that we 
take responsability to not break borrowing rules on the Js side :-)

```rust
    // Solution is to use a raw pointer (*const) to the first element of our vector
    // Borrow checker will not apply the rules.
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

```

## on the Js/Ts side

- Get the pointer to our buffer that is within wasmMemory

```ts
    const snakeCellPtr = world.snake_cells_ptr();
```

- Create a view from Js/Ts of our wasm memory using Uint32Array

```ts
    const snakeLen = world.snake_length();
    const snakeCells = new Uint32Array( 
                                wasm.memory.buffer, 
                                snakeCellPtr, 
                                snakeLen);
```
The Uint32Array typed array represents an array of 32-bit unsigned integers in 
the platform byte order. The contents are initialized to 0. 
Once established, you can reference elements in the array using the object's methods, 
or using standard array index syntax (that is, using bracket notation).



- Update drawSnake to draw the body not just the head.

```ts
    function drawSnake() {
        //const snake_idx = world.snake_head_idx();
        
        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            world.snake_cells_ptr(),
            world.snake_length(),
        );

        snakeCells.forEach(cellIdx => {
            const x = cellIdx % worldWidth;
            const y = Math.floor(cellIdx / worldWidth);
            
            ctx.beginPath();

            ctx.fillRect(
                x * CELL_SIZE, 
                y * CELL_SIZE, 
                CELL_SIZE, CELL_SIZE);
        });

        ctx.stroke();
    }
```

## Bug in build
we were compiling our typescript before
copying pkg into our www folder.

Here is the correct order

```bash
#!/bin/sh

set -ex

wasm-pack build --target web

cp -fr pkg www/

# tsc --module ES6 --target ES6 www/index.ts

# using config file in www/tsconfig.json
tsc -p ./www/


printf '%s\n' "serving page at: http://127.0.0.1:8080"
#python3 -m http.server

http -a 127.0.0.1 -p 8080 www/
```
