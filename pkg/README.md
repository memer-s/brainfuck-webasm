# Brainfuck in WebAssembly

This project is built with wasm-pack and rust. 

To build the project, install wasm-pack and run `wasm-pack build --target web`.
The interpreter features 30000 cells of memory (as to be faithful to the original). Each cell is an unsigned 8-bit integer.

References:
- [Brainfuck on Wikipedia](https://en.wikipedia.org/wiki/Brainfuck)
- [brain-lang/brainfuck](https://github.com/brain-lang/brainfuck)

----

### TODO:
- [ ] Implement knobs or inputs to change execution speed, and other emulation settings.
- [ ] Saving code.
- [ ] Cursor for currently executing code / A visual way to see where the Program Counter exists during program execution.
