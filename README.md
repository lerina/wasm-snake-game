# wasm-snake-game: Slide 3

build the snake grid

This is one way, but it feels cluttered

```js
    function drawWorld() {
        ctx.beginPath();
        
        // I find this unnesseraly cluttering to the eyes and demanding on the cpu
        // mk column: mv on the y axis
        for (let y=0; y <= world_width; y++) {
            ctx.moveTo(y * CELL_SIZE, 0);
            ctx.lineTo(y * CELL_SIZE, world_width * CELL_SIZE); 
        }

        // mk row: mv on the x axis
        for (let x=0; x < world_width + 1; x++) {
            ctx.moveTo(0, CELL_SIZE * x);
            ctx.lineTo(world_width * CELL_SIZE, CELL_SIZE * x); 
        }


        ctx.stroke();
    }

```

But this version is clear and avoid performing calculations in a loop

```js
    function drawWorld() {
        ctx.beginPath();
        

        // mk column: mv on the y axis
        for (let y=0; y <= canvas.height; y+=CELL_SIZE) {
            ctx.moveTo(y, 0);
            ctx.lineTo(y, canvas.height ); 
        }
        
        // mk row: mv on the x axis
        // I find this unnesseraly cluttering to the eyes and demanding on the cpu
        for (let x=0; x <= canvas.width; x+=CELL_SIZE) {
            ctx.moveTo(0, x);
            ctx.lineTo(canvas.width, x); 
        }
        

        ctx.stroke();
    }

```
