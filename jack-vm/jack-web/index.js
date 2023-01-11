import { Memory, Program } from "../pkg";
import { memory } from "../pkg/jack_vm_bg";

const BIT_ON_COLOR = "#000000";
const BIT_OFF_COLOR = "#DFDFDF";
const WORD_SIZE = 16; //bits

const testProgram = `
function Main.main 0
push constant 0
push constant 220
push constant 511
push constant 220
call Screen.drawLine 4
pop temp 0
push constant 280
push constant 90
push constant 410
push constant 220
call Screen.drawRectangle 4
pop temp 0
push constant 0
call Screen.setColor 1
pop temp 0
push constant 350
push constant 120
push constant 390
push constant 219
call Screen.drawRectangle 4
pop temp 0
push constant 292
push constant 120
push constant 332
push constant 150
call Screen.drawRectangle 4
pop temp 0
push constant 0
not
call Screen.setColor 1
pop temp 0
push constant 360
push constant 170
push constant 3
call Screen.drawCircle 3
pop temp 0
push constant 280
push constant 90
push constant 345
push constant 35
call Screen.drawLine 4
pop temp 0
push constant 345
push constant 35
push constant 410
push constant 90
call Screen.drawLine 4
pop temp 0
push constant 140
push constant 60
push constant 30
call Screen.drawCircle 3
pop temp 0
push constant 140
push constant 26
push constant 140
push constant 6
call Screen.drawLine 4
pop temp 0
push constant 163
push constant 35
push constant 178
push constant 20
call Screen.drawLine 4
pop temp 0
push constant 174
push constant 60
push constant 194
push constant 60
call Screen.drawLine 4
pop temp 0
push constant 163
push constant 85
push constant 178
push constant 100
call Screen.drawLine 4
pop temp 0
push constant 140
push constant 94
push constant 140
push constant 114
call Screen.drawLine 4
pop temp 0
push constant 117
push constant 85
push constant 102
push constant 100
call Screen.drawLine 4
pop temp 0
push constant 106
push constant 60
push constant 86
push constant 60
call Screen.drawLine 4
pop temp 0
push constant 117
push constant 35
push constant 102
push constant 20
call Screen.drawLine 4
pop temp 0
push constant 0
return
`;

const program = new Program(testProgram);

// total memory size can be shown on a 768 x 512 grid.
// If each memory cell is represented as a 2x2 pixel on a grid, it might even be legible!
const displaySize = program.display_size();
const displayPtr = program.display();

const ramSize = program.ram_size();
const ramPtr = program.ram();

// display dimensions in virtual pixels
const displayWidthPixels = 512;
const displayHeightPixels = 256;
const pixelSize = 1; // size of each virtual pixel in real pixels

// set display canvas size
const displayCanvas = document.getElementById("display-canvas");
displayCanvas.width = displayWidthPixels * pixelSize;
displayCanvas.height = displayHeightPixels * pixelSize;

const ctx = displayCanvas.getContext("2d", {
  willReadFrequently: true,
  alpha: false,
});

// from rustwasm tutorial
function bitIsSet(n, arr) {
  const word = Math.floor(n / 16);
  const mask = 1 << n % 16;
  return (arr[word] & mask) === mask;
}

const displayArray = new Uint16Array(memory.buffer, displayPtr, displaySize);

function drawMemory() {
  // pull pixels out of the canvas
  const id = ctx.getImageData(0, 0, displayCanvas.width, displayCanvas.height);
  // pixels is a Uint8ClampedArray, each pixel being 4 consecutive Uint8 values
  // representing r, g, b, and a respectively
  const pixels = id.data;

  for (let i = 0; i < displaySize * 16; i++) {
    if (bitIsSet(i, displayArray)) {
      // set corresponding virtual pixel to green
      const offset = i * 4;
      pixels[offset] = 0;
      pixels[offset + 1] = 255;
      pixels[offset + 2] = 0;
    } else {
      // set corresponding virtual pixel to black
      const offset = i * 4;
      pixels[offset] = 10;
      pixels[offset + 1] = 10;
      pixels[offset + 2] = 10;
    }
  }
  // place pixels back in the canvas
  ctx.putImageData(id, 0, 0);
}

function updateRam(id, start, end, memSize, memPtr) {
  const ramContainer = document.getElementById(id);
  if (document.getElementById(id + "ram")) {
    document.getElementById(id + "ram").remove();
  }
  const outer = document.createElement("div");
  const memArray = new Int16Array(memory.buffer, memPtr, memSize);
  outer.setAttribute("id", id + "ram");
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


// Button listeners
const stepButton = document.getElementById("step-button");
stepButton.addEventListener("click", (event) => {
  renderLoop();
});

const runButton = document.getElementById("run-button");
runButton.addEventListener("click", (event) => {
  setInterval(() => requestAnimationFrame(renderLoop), 0);
});

// Render the starting state
drawMemory(
  displayWidthPixels,
  displayHeightPixels,
  pixelSize,
  displayPtr,
  displaySize
);
// updateRam("pointers", 0, 45, ramSize, ramPtr);
// updateRam("global-stack", 256, 350, ramSize, ramPtr);
// updateRam("heap", 16000, 16383, ramSize, ramPtr);

function renderLoop() {
  for (let i = 0; i<20; i++) {
    program.step(currentKey)
  }
  drawMemory();
  // updateRam("pointers", 0, 45, ramSize, ramPtr);
  // updateRam("global-stack", 256, 350, ramSize, ramPtr);
  // updateRam("heap", 16000, 16383, ramSize, ramPtr);
}


