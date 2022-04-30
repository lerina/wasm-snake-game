# wasm-snake-game: Slide 11


source: [Interfacing Rust and JavaScript](https://rustwasm.github.io/docs/book/game-of-life/implementing.html#interfacing-rust-and-javascript)

JavaScript's garbage-collected heap — where Objects, Arrays, and DOM nodes are allocated —   
is distinct from  
WebAssembly's linear memory space, where our Rust values live. 

WebAssembly currently has no direct access to the garbage-collected heap (as of April 2018, this is expected to change with the "Interface Types" proposal). 

JavaScript, on the other hand, can read and write to the WebAssembly linear memory space, but only as an ArrayBuffer of scalar values (u8, i32, f64, etc...). WebAssembly functions also take and return scalar values. 

These are the building blocks from which all WebAssembly and JavaScript communication is constituted.

`wasm_bindgen` defines a common understanding of how to work with compound structures across this boundary.  
It involves boxing Rust structures, and wrapping the pointer in a JavaScript class for usability, or indexing into a table of JavaScript objects from Rust. 

wasm_bindgen is very convenient, but it does not remove the need to consider our data representation, and what values and structures are passed across this boundary. Instead, think of it as a tool for implementing the interface design you choose.

When designing an interface between WebAssembly and JavaScript, we want to optimize for the following properties:

1. Minimizing copying into and out of the WebAssembly linear memory. Unnecessary copies impose unnecessary overhead.
2. Minimizing serializing and deserializing. Similar to copies, serializing and deserializing also imposes overhead, and often imposes copying as well. If we can pass opaque handles to a data structure — instead of serializing it on one side, copying it into some known location in the WebAssembly linear memory, and deserializing on the other side — we can often reduce a lot of overhead. wasm_bindgen helps us define and work with opaque handles to JavaScript Objects or boxed Rust structures.

As a general rule of thumb, a good JavaScript↔WebAssembly interface design 
is often one where large, long-lived data structures are implemented 
as Rust types that live in the WebAssembly linear memory, 
and are exposed to JavaScript as opaque handles. 

JavaScript calls exported WebAssembly functions that take these opaque handles, 
transform their data, perform heavy computations, query the data, 
and ultimately return a small, copy-able result.  

By only returning the small result of the computation, 
we avoid copying and/or serializing everything back and forth between 
the JavaScript garbage-collected heap and the WebAssembly linear memory.


## Take away 

At this point we have enough information to continue our snake-game.

The rendering and configurable data will be in our Js/Ts file. 
Everything else will be in the wasm side.

---

## Further Information
### rule of thumb: 
1. Use WebAssembly for business logic, computationally intensive tasks, such as games, image manipulation, math, physics, audio effects, etc....

2. You still generally want to use Javascript for a lot of things on the web. Such as Dom Manipulation (modifying/building views and UIs), blogs, ecommerce websites, using most Web APIs, etc...

3. js data lives within JavaScript’s heap, which is currently inaccessible from WebAssembly’s linear memory space (this, too, will be fixed upon implementation of the interface types proposal: five years on going)

### To remember
a better implementation would instead have our table data and the entirety of our sorting logic reside inside WebAssembly. Rather than hitting an endpoint for the table data from JavaScript, we would do so from Rust, perhaps via some exposed-to-Javascript handle. This precludes the need for any copying, since all of our data now lives and is manipulated entirely within WebAssembly’s memory space.

Without the need to copy, things should be much faster. 

Furthermore, while WebAssembly cannot currently access JavaScript’s heap, JavaScript can access WebAssembly’s linear memory space via the WebAssembly.Memory API. The buffer prototype property of the WebAssembly.Memory object returns the buffer contained in the memory.. 
wasm-bindgen exports the memory of our WebAssembly instance as a standard module, meaning we can import just as easily as we can anything else.

```js
// From JavaScript, we can easily import WebAssembly's memory.
import { memory } from "../../rust/pkg/wasm_bg";

```

see:  
[WebAssembly.Memory.prototype.buffer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Memory/buffer)  
[Rendering to Canvas Directly from Memory](https://rustwasm.github.io/book/game-of-life/implementing.html#rendering-to-canvas-directly-from-memory)

### hazards to avoid

We don't want to copy the whole universe 
into and out of the WebAssembly linear memory on every tick. 

We do not want to allocate objects for every cell in the universe, 
nor do we want to impose a cross-boundary call to read and write each cell.





