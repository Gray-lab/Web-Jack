State to manage:
- jackcode string
- bytecode string
- compiled/not compiled
- program
- running/not running
- ram cells...

Events/interactions
- compile button: sends jackcode string to compiler and loads result into bytecode string (compiler doesn't yet exist as a wasm binding, lol)
- init: initializes Program with bytecode input
- step: calls Program.step()
- run: calls setInterval(() => requestAnimationFrame(renderLoop), 0)
- stop: terminates setInterval

Keyboard -> when Running, keyboard events should go to the runtime keyboard on-down handler
Otherwise, keyboard output should go to the selected codemirror instance