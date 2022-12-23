import { Memory, Program } from "../pkg";
import { memory } from "../pkg/jack_vm_bg";

const BIT_ON_COLOR = "#000000";
const BIT_OFF_COLOR = "#DFDFDF";
const WORD_SIZE = 16; //bits

const testProgram = `
function Main.main 4
push constant 18 
call String.new 1
push constant 72 
call String.appendChar 2
push constant 111 
call String.appendChar 2
push constant 119 
call String.appendChar 2
push constant 32 
call String.appendChar 2
push constant 109 
call String.appendChar 2
push constant 97 
call String.appendChar 2
push constant 110 
call String.appendChar 2
push constant 121 
call String.appendChar 2
push constant 32 
call String.appendChar 2
push constant 110 
call String.appendChar 2
push constant 117 
call String.appendChar 2
push constant 109 
call String.appendChar 2
push constant 98 
call String.appendChar 2
push constant 101 
call String.appendChar 2
push constant 114 
call String.appendChar 2
push constant 115 
call String.appendChar 2
push constant 63 
call String.appendChar 2
push constant 32 
call String.appendChar 2
call Keyboard.readInt 1
pop local 1 //bind topmost stack value to length
push local 1 //value of length
call Array.new 1
pop local 0 //bind topmost stack value to a
push constant 0 
pop local 2 //bind topmost stack value to i
label WHILE0
push local 2 //value of i
push local 1 //value of length
lt
not
if-goto WHILE1
push local 0 //a
push local 2 //value of i
add
push constant 16 
call String.new 1
push constant 69 
call String.appendChar 2
push constant 110 
call String.appendChar 2
push constant 116 
call String.appendChar 2
push constant 101 
call String.appendChar 2
push constant 114 
call String.appendChar 2
push constant 32 
call String.appendChar 2
push constant 97 
call String.appendChar 2
push constant 32 
call String.appendChar 2
push constant 110 
call String.appendChar 2
push constant 117 
call String.appendChar 2
push constant 109 
call String.appendChar 2
push constant 98 
call String.appendChar 2
push constant 101 
call String.appendChar 2
push constant 114 
call String.appendChar 2
push constant 58 
call String.appendChar 2
push constant 32 
call String.appendChar 2
call Keyboard.readInt 1
pop temp 0 //save expression result to temp location
pop pointer 1 //set pointer to array+exp
push temp 0 //get stored value
pop that 0 //set array location to that value
push local 3 //value of sum
push local 0 //pointer to a
push local 2 //value of i
add
pop pointer 1 
push that 0 //value of a[exp]
add
pop local 3 //bind topmost stack value to sum
push local 2 //value of i
push constant 1 
add
pop local 2 //bind topmost stack value to i
goto WHILE0
label WHILE1
push constant 15 
call String.new 1
push constant 84 
call String.appendChar 2
push constant 104 
call String.appendChar 2
push constant 101 
call String.appendChar 2
push constant 32 
call String.appendChar 2
push constant 97 
call String.appendChar 2
push constant 118 
call String.appendChar 2
push constant 101 
call String.appendChar 2
push constant 114 
call String.appendChar 2
push constant 97 
call String.appendChar 2
push constant 103 
call String.appendChar 2
push constant 101 
call String.appendChar 2
push constant 32 
call String.appendChar 2
push constant 105 
call String.appendChar 2
push constant 115 
call String.appendChar 2
push constant 32 
call String.appendChar 2
call Output.printString 1
pop temp 0 //remove return value from stack after do statement
push local 3 //value of sum
push local 1 //value of length
call Math.divide 2
call Output.printInt 1
pop temp 0 //remove return value from stack after do statement
return
push constant 0 
`;

const program = new Program(testProgram);

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
console.log(`display ptr: ${displayPtr}`);
// console.log(`height in rows: ${height}`);
// console.log(`width in columns: ${width}`);
// console.log(`width in blocks: ${width_blocks}`);

// set display canvas size
const displayCanvas = document.getElementById("display-canvas");
displayCanvas.width = displayWidthPixels * pixelSize;
displayCanvas.height = displayHeightPixels * pixelSize;

const ctx = displayCanvas.getContext("2d", {
  willReadFrequently: true,
  alpha: false,
});
ctx.scale(pixelSize, pixelSize);

// from rustwasm tutorial
function bitIsSet(n, arr) {
  const word = Math.floor(n / 16);
  const mask = 1 << n % 16;
  return (arr[word] & mask) === mask;
}

function drawMemory(widthPixels, heightPixels, pixelSize, memPtr, memSize) {
  console.log(memSize)
  console.log(memory.buffer)
  console.log(memPtr)
  const memArray = new Uint16Array(memory.buffer, memPtr, memSize);
  // pull pixels out of the canvas
  const id = ctx.getImageData(0, 0, displayCanvas.width, displayCanvas.height);
  // pixels is a Uint8ClampedArray, each pixel being 4 consecutive Uint8 values
  // representing r, g, b, and a respectively
  const pixels = id.data;

  for (let i = 0; i < memSize * 16; i++) {
    if (bitIsSet(i, memArray)) {
      // set corresponding virtual pixel to red
      const offset = i * 4;
      pixels[offset] = 0;
      pixels[offset + 1] = 255;
      pixels[offset + 2] = 0;
      // drawVirtualPixel(pixels, i, pixelSize, widthPixels, 0, 255, 0, 255);
    } else {
      // set corresponding virtual pixel to black
      // drawVirtualPixel(pixels, i, pixelSize, widthPixels, 10, 10, 10, 255);
      const offset = i * 4;
      pixels[offset] = 10;
      pixels[offset + 1] = 10;
      pixels[offset + 2] = 10;
    }
  }

  // place pixels back in the canvas
  ctx.putImageData(id, 0, 0);
}

function drawVirtualPixel(pixels, index, pixelSize, widthPixels, r, g, b, a) {
  // index 0 ->
  for (let y = 0; y < pixelSize; y++) {
    for (let x = 0; x < pixelSize; x++) {
      const x_shift = x * 4;
      const y_shift =
        Math.floor(index / widthPixels) *
          widthPixels *
          4 *
          (pixelSize == 1 ? 0 : pixelSize) +
        y * widthPixels * 4 * pixelSize;
      const offset = index * 4 * pixelSize + x_shift + y_shift; // 4 values per pixel
      pixels[offset] = r;
      pixels[offset + 1] = g;
      pixels[offset + 2] = b;
      pixels[offset + 3] = a;
    }
  }
}

function updateRamList(start, end, memSize, memPtr) {
  const ramContainer = document.getElementById("ram-container");
  if (document.getElementById("ram-list")) {
    document.getElementById("ram-list").remove();
  }
  const outer = document.createElement("div");
  const memArray = new Int16Array(memory.buffer, memPtr, memSize);
  outer.setAttribute("id", "ram-list");
  for (let i = start; i <= end; i++) {
    const inner = document.createElement("div");
    inner.innerHTML = `ram[${i}]:\t ${memArray[i]}`;
    outer.appendChild(inner);
  }
  ramContainer.appendChild(outer);
}

// Keyboard input listeners

const input_map = {
  Enter: 128,
  Backspace: 129,
  ArrowLeft: 130,
  ArrowUp: 131,
  ArrowRight: 132,
  ArrowDown: 133,
  Home: 134,
  End: 135,
  PageUp: 136,
  PageDown: 137,
  Insert: 138,
  Delete: 139,
  Escape: 140,
  F1: 141,
  F2: 142,
  F3: 143,
  F4: 144,
  F5: 145,
  F6: 146,
  F7: 147,
  F8: 148,
  F9: 149,
  F10: 150,
  F11: 151,
  F12: 152,
};

let currentKey = 0;
const body = document.querySelector("body");
body.addEventListener("keydown", (event) => {
  console.log(event.key);
  if (event.key.length === 1) {
    currentKey = event.key.charCodeAt(0);
  } else if (event.key in input_map) {
    currentKey = input_map[event.key];
  }
});

body.addEventListener("keyup", (event) => {
  if (event.key.length === 1 || event.key in input_map) {
    currentKey = 0;
  }
});

let i = 0;
let do_log = true;
// setTimeout(() => {
//     do_log = false
// }, 10000)

drawMemory(
  displayWidthPixels,
  displayHeightPixels,
  pixelSize,
  displayPtr,
  displaySize
);
updateRamList(256, 270, ramSize, ramPtr);

function renderLoop() {
  // program.set_display(i, i)
  // i++
  if (program.step(currentKey)) {
    drawMemory(
      displayWidthPixels,
      displayHeightPixels,
      pixelSize,
      displayPtr,
      displaySize
    );
  }
  updateRamList(16300, 16383, ramSize, ramPtr);
  //   console.log(`Read key ${currentKey}`);
  // if (i < displaySize) {
  // requestAnimationFrame(renderLoop)
  // }

  // if (do_log) {
  // console.log(i)
  // i ++
  // }
}
// requestAnimationFrame(renderLoop)
setInterval(() => requestAnimationFrame(renderLoop), 10);
