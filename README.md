## Jack-VM
---
A web based VM for the Jack language from nand2tetris, running a game of snake!
![snake works!](./img/snake.png)

This program was inspired while I was working through the nand2tetris course. The course is all about virtually building up a computing system from scratch starting from the logic gates all the way to stack machine bytecode running on a virtual machine. The creators of a class provide the virtual machine emulator to run this bytecode, but using this felt somehow incomplete. If I had already taken the time to write an assembler and both the frontend and backend of the compiler, wouldn't it be awesome to also write a virtual machine to run it? Wouldn't it be even cooler to have it work on the web so that others who do nand2tetris could easily share their Jack programs with others?

Toward the second half of my time at Recurse Center I decided to kill two birds with one stone and build this vm in Rust, compiling to webassembly to make it runnable in a browser. The memory and registers are faithfully emulated as specified in the nand2tetris course and the core Jack library has been implemented in a combination of Rust and Jack. 

---
Memory values displayed while running a version of pong, showing the registers, stack, and heap. 
![pong and memory](./img/pong.png)

The display was originally also implemented as a linear memory array that was then read into a canvas component on the website, but updating the canvas in this way was too slow. The canvas is instead now updated directly in the wasm code when any draw or output functions are called. 

I would not have been able to do this without the encouragement and help of my batch mates at Recurse Center. I especially want to thank Mary, who paired with me many times and contributed her remarkable skills in languges, compilers, Rust, and programming in general. 

### Code organization
jack-vm/src contains all the Rust code for the core library. The entry point is the program.rs file. First, a Program struct is created. The constructor for this takes a string of Jack bytecode and parses it into the vm instructions using the logic in parser.rs.  Library functions from jacklib.rs are placed into a global function hashmap and any user functions or methods are placed into a separate hashmap. A step method can then be called on the Program struct to execute the next instruction, updating the display and memory. While most of the library functions are implemented in Rust, the keyboard entry functions are implemented in Jack. This avoided the need for defeloping an interrupt handler. Instead, the VM is able to use the Jack implemented functions to check any updates to the memory location associated with the keyboard input with each execution step. 

The memory.rs file handles the implementation of the memory struct, which emulates the 16 bit memory of the Hack cpu from nand2tetris. The first 16 elements are virtual registers, including the stack, static, local, argument, and object pointers. The memory implementation includes a number of methods to handle the global stack, view and change values in memory, and update the display memory (which I decided to keep for now, even though the display is mostly handled with a direct wasm binding to an html canvas).

The charmap.rs file contains character bit mappings for the standard Jack font. Since the original display was implemented as a direct memory mapping where an on bit represented a lit pixel, the font is rendered by bitwise manipulation of the display memory. With the use of a canvas it might be easier to use the built-in font support, but it would not be nearly as satisfying. :)

/jack-vm/jack-web includes index.html and index.js files that serve as a simple testing front-end. Display is largely handled via the direct wasm canvas binding, but the js file still contains the code for reading the display memory directly from wasm and bitwise drawing it to the canvas. Likewise, the memory values are also read directly from the wasm memory using an Int16Array. Currently there is no front end input for the bytecode, which is entered into the testProgram string.

### Future Goals:
- Set up Svelte front end for the VM
- Rewrite Jack compiler in Rust so it can be added as a wasm binding or a webworker
- Add codemirror components for editing of code in the browser
- Add options for range of memory visualization
