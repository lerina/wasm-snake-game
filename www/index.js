import init, { World } from "../pkg/snake_game.js";

init().then(_ => {
    const world = World.new();
    const canvas = document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");
    
    console.log("from index.js: " + world.width());
})

