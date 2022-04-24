import init, { World } from "../pkg/snake_game.js";

init().then(_ => {
    const world = World.new();
    console.log("from index.js: " + world.width);
})

