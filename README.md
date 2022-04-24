# wasm-snake-game: Slide 5

Draw the snake.  Only the head for now :-)

The fillRect() method draws a "filled" rectangle.
The default color of the fill is black.

```js
    function drawSnake() {
        const snake_idx = world.snake_head_idx();
        const col = snake_idx % world_width;
        const row = Math.floor(snake_idx / world_width);
        
        ctx.beginPath();

        ctx.fillRect(
            col * CELL_SIZE,            // x: upper-left corner
            row * CELL_SIZE,            // y: upper-left corner
            CELL_SIZE, CELL_SIZE);      // width, height

        ctx.stroke();
    }
```

Then you draw the snake after drawing the world

```js
    drawWorld();
    drawSnake();
```


