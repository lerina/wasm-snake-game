# wasm-snake-game: Slide 20

## Adding the food (reward)
We shall hold the food index in reward_cell
put is at index 10 for now

```rust
#[wasm_bindgen]
pub struct World {
    ...
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        World {
            ...            
            next_cell: None,
            reward_cell: 10,
        }
    }
```

##draw the food

- on the Ts side draw a rect at the specified index from wasm 
 
```ts
    function drawReward(){
        const rewardIdx= world.reward_cell();
        const x = rewardIdx % worldWidth;
        const y = Math.floor(rewardIdx / worldWidth);

        ctx.beginPath();
        ctx.fillStyle = "#ff0000";
        ctx.fillRect(
            x * CELL_SIZE,
            y * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        );
        ctx.stroke();
    }
```

- update the rendering 

```ts
    function draw_all(){
        drawWorld();
        drawSnake();
        drawReward();
    }
```

