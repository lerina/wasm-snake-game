import init, { World, Direction } from "./pkg/snake_game.js";
import { rnd } from "./utils/rnd.js";
init().then(wasm => {
    const CELL_SIZE = 20;
    const fps = 5;
    const worldWidth = 8;
    const worldHeight = 8;
    const snakeSpawnIdx = rnd(worldWidth * worldHeight);
    const world = World.new(worldWidth, snakeSpawnIdx);
    const canvas = document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");
    canvas.width = worldWidth * CELL_SIZE;
    canvas.height = worldHeight * CELL_SIZE;
    const snakeCellPtr = world.snake_cells_ptr();
    const snakeLen = world.snake_length();
    const snakeCells = new Uint32Array(wasm.memory.buffer, snakeCellPtr, snakeLen);
    document.addEventListener("keydown", e => {
        switch (e.code) {
            case "ArrowUp":
                world.change_snake_dir(Direction.Up);
                break;
            case "ArrowRight":
                world.change_snake_dir(Direction.Right);
                break;
            case "ArrowDown":
                world.change_snake_dir(Direction.Down);
                break;
            case "ArrowLeft":
                world.change_snake_dir(Direction.Left);
                break;
        }
    });
    function drawWorld() {
        ctx.beginPath();
        // mk column: mv on the y axis
        for (let y = 0; y <= canvas.height; y += CELL_SIZE) {
            ctx.moveTo(y, 0);
            ctx.lineTo(y, canvas.height);
        }
        // mk row: mv on the x axis
        for (let x = 0; x <= canvas.width; x += CELL_SIZE) {
            ctx.moveTo(0, x);
            ctx.lineTo(canvas.width, x);
        }
        ctx.stroke();
    }
    function drawSnake() {
        //const snake_idx = world.snake_head_idx();
        const snakeCells = new Uint32Array(wasm.memory.buffer, world.snake_cells_ptr(), world.snake_length());
        snakeCells.forEach((cellIdx, i) => {
            const x = cellIdx % worldWidth;
            const y = Math.floor(cellIdx / worldWidth);
            // snake has purple head 
            ctx.fillStyle = i === 0 ? "#7878dd" : "#343434";
            ctx.beginPath();
            ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        });
        ctx.stroke();
    }
    function draw_all() {
        drawWorld();
        drawSnake();
    }
    function update() {
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            world.step();
            draw_all();
            requestAnimationFrame(update);
        }, 1000 / fps);
    }
    update();
});
