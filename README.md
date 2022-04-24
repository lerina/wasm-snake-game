# wasm-snake-game: Slide 6

Refresh canvas


```js
import init, { World } from "../pkg/snake_game.js";

init().then(_ => {
    const CELL_SIZE = 20;
    const refresh_rate = 100;

    const world = World.new();

...

    drawWorld();
    drawSnake();

    setInterval(() => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawWorld();
        drawSnake();

    }, refresh_rate);
})
```


