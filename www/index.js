import init, { World } from "../pkg/snake_game.js";

init().then(_ => {
    const CELL_SIZE = 20;

    const world = World.new();
    const world_width = world.width(); // avoid back and forth js-rust

    const canvas = document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");
   
    canvas.height = world_width * CELL_SIZE;
    canvas.width = world_width * CELL_SIZE; // world_widht: bad name dim would be better
    
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
    
    console.log("snake head, cell: " + world.snake_head_idx());
    drawWorld();
})

