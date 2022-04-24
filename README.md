# wasm-snake-game: Slide 9

- setTimeout()

The global setTimeout() method sets a timer which executes a function or specified piece of code once the timer expires.
This gives a little time for the page to load before jumping into the "gameloop"

- requestAnimationFrame()

Tells the browser that you wish to perform an animation and requests that the browser calls a specified function to update an animation before the next repaint.

- update
Our callback routine, that must itself call requestAnimationFrame() again 
since we want to animate another frame at the next repaint. 
requestAnimationFrame() is 1 shot.



```js
import init, { World } from "../pkg/snake_game.js";

init().then(_ => {
    const CELL_SIZE = 20;
    const refresh_rate = 100;

...

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
```

see also:  
[CREATE A PROPER GAME LOOP](https://spicyyoghurt.com/tutorials/html5-javascript-game-development/create-a-proper-game-loop-with-requestanimationframe)  
[MDN: Anatomy of a video game](https://developer.mozilla.org/en-US/docs/Games/Anatomy)  

