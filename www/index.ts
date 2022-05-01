import init, { World } from "./pkg/snake_game.js";
import {rnd} from "./utils/rnd.js";

init().then(_ => {
    const CELL_SIZE = 20;
    const refresh_rate = 100;
    const WORLD_WIDTH = 8;
    const WORLD_HEIGHT = 8;
    const SNAKE_SPAWN_IDX = rnd(WORLD_WIDTH * WORLD_HEIGHT);

    const world = World.new(WORLD_WIDTH, WORLD_HEIGHT, SNAKE_SPAWN_IDX);

    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");
   
    canvas.width = WORLD_WIDTH * CELL_SIZE;
    canvas.height = WORLD_HEIGHT * CELL_SIZE;

    function drawWorld() {
        ctx.beginPath();

        // mk column: mv on the y axis
        for (let y=0; y <= canvas.height; y+=CELL_SIZE) {
            ctx.moveTo(y, 0);
            ctx.lineTo(y, canvas.height ); 
        }
        
        // mk row: mv on the x axis
        for (let x=0; x <= canvas.width; x+=CELL_SIZE) {
            ctx.moveTo(0, x);
            ctx.lineTo(canvas.width, x); 
        }
        

        ctx.stroke();
    }
    

    function drawSnake() {
        const snake_idx = world.snake_head_idx();
        const col = snake_idx % WORLD_WIDTH;
        const row = Math.floor(snake_idx / WORLD_HEIGHT);
        
        ctx.beginPath();

        ctx.fillRect(
            col * CELL_SIZE, 
            row * CELL_SIZE, 
            CELL_SIZE, CELL_SIZE);

        ctx.stroke();
    }

    //drawWorld();
    //drawSnake();
    
    function update() {
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            drawWorld();
            drawSnake();
            world.update();
            
            requestAnimationFrame(update);
        }, refresh_rate);
    }
    
    update();
})

