# Brainfuck in WebAssembly

This project is built with wasm-pack and rust. 

To build the project, install wasm-pack and run `wasm-pack build --target web`.
The interpreter features 256 cells of memory. Each cell is a unsigned 8-bit integer.

References
- [Brainfuck on Wikipedia](https://en.wikipedia.org/wiki/Brainfuck)
