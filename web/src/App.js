import './App.css';
import Program from "jack-web";
import memory from "jack-web";

function testProgram() {
  const testProgram = `
  function Main.main 0
  push constant 1 
  push constant 2 
  push constant 3 
  call Math.multiply 2
  add
  call Output.printInt 1
  pop temp 0 //remove return value from stack after do statement
  return
  push constant 0 
  `;

  // set lcl=260 so we can see it near sp at 256
  const program = new Program(testProgram, 265);

  // total memory size can be shown on a 768 x 512 grid.
  // If each memory cell is represented as a 2x2 pixel on a grid, it might even be legible!
  console.log("before program");
  const displaySize = program.display_size();
  const displayPtr = program.display();

  const ramSize = program.ram_size();
  const ramPtr = program.ram();

  // display dimensions in virtual pixels
  const displayWidthPixels = 512;
  const displayHeightPixels = 256;
  const pixelSize = 1; // size of each virtual pixel in real pixels

  console.log(`memory size: ${displaySize}`);
}

function App() {
  return (
    <div className="Jack-web">
      <header className="Jack-web-header">
        <p>
          Lets test a wasm binding!
          {testProgram()}
        </p>
      </header>

    </div>
  );
}

export default App;
