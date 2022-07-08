# Brainfuck in WebAssembly

This project is built with wasm-pack and rust. 

To build the project, install wasm-pack and run `wasm-pack build --target web`.

References

- [https://en.wikipedia.org/wiki/Brainfuck](Wikipedia brainfuck)

The interpreter features 256 cells of memory (to be faithful to the original). Each cell is a unsigned 8-bit integer.
