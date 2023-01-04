import { Memory, Program } from "../pkg";
import { memory } from "../pkg/jack_vm_bg";

const BIT_ON_COLOR = "#000000";
const BIT_OFF_COLOR = "#DFDFDF";
const WORD_SIZE = 16; //bits

const testProgram = `
function Main.main 1
push constant 1
call String.new 1
pop local 0
push local 0
call String.doubleQuote 0
call String.appendChar 2
pop temp 0
push constant 0
push constant 63
call Output.moveCursor 2
pop temp 0
push constant 66
call Output.printChar 1
pop temp 0
push constant 22
push constant 0
call Output.moveCursor 2
pop temp 0
push constant 67
call Output.printChar 1
pop temp 0
push constant 22
push constant 63
call Output.moveCursor 2
pop temp 0
push constant 68
call Output.printChar 1
pop temp 0
push constant 65
call Output.printChar 1
pop temp 0
push constant 2
push constant 0
call Output.moveCursor 2
pop temp 0
push constant 10
call String.new 1
push constant 48
call String.appendChar 2
push constant 49
call String.appendChar 2
push constant 50
call String.appendChar 2
push constant 51
call String.appendChar 2
push constant 52
call String.appendChar 2
push constant 53
call String.appendChar 2
push constant 54
call String.appendChar 2
push constant 55
call String.appendChar 2
push constant 56
call String.appendChar 2
push constant 57
call String.appendChar 2
call Output.printString 1
pop temp 0
call Output.println 0
pop temp 0
push constant 53
call String.new 1
push constant 65
call String.appendChar 2
push constant 66
call String.appendChar 2
push constant 67
call String.appendChar 2
push constant 68
call String.appendChar 2
push constant 69
call String.appendChar 2
push constant 70
call String.appendChar 2
push constant 71
call String.appendChar 2
push constant 72
call String.appendChar 2
push constant 73
call String.appendChar 2
push constant 74
call String.appendChar 2
push constant 75
call String.appendChar 2
push constant 76
call String.appendChar 2
push constant 77
call String.appendChar 2
push constant 78
call String.appendChar 2
push constant 79
call String.appendChar 2
push constant 80
call String.appendChar 2
push constant 81
call String.appendChar 2
push constant 82
call String.appendChar 2
push constant 83
call String.appendChar 2
push constant 84
call String.appendChar 2
push constant 85
call String.appendChar 2
push constant 86
call String.appendChar 2
push constant 87
call String.appendChar 2
push constant 88
call String.appendChar 2
push constant 89
call String.appendChar 2
push constant 90
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 98
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 100
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 102
call String.appendChar 2
push constant 103
call String.appendChar 2
push constant 104
call String.appendChar 2
push constant 105
call String.appendChar 2
push constant 106
call String.appendChar 2
push constant 107
call String.appendChar 2
push constant 108
call String.appendChar 2
push constant 109
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 111
call String.appendChar 2
push constant 112
call String.appendChar 2
push constant 113
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 118
call String.appendChar 2
push constant 119
call String.appendChar 2
push constant 120
call String.appendChar 2
push constant 121
call String.appendChar 2
push constant 122
call String.appendChar 2
call Output.printString 1
pop temp 0
call Output.println 0
pop temp 0
push constant 30
call String.new 1
push constant 33
call String.appendChar 2
push constant 35
call String.appendChar 2
push constant 36
call String.appendChar 2
push constant 37
call String.appendChar 2
push constant 38
call String.appendChar 2
push constant 39
call String.appendChar 2
push constant 40
call String.appendChar 2
push constant 41
call String.appendChar 2
push constant 42
call String.appendChar 2
push constant 43
call String.appendChar 2
push constant 44
call String.appendChar 2
push constant 45
call String.appendChar 2
push constant 46
call String.appendChar 2
push constant 47
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 59
call String.appendChar 2
push constant 60
call String.appendChar 2
push constant 61
call String.appendChar 2
push constant 62
call String.appendChar 2
push constant 63
call String.appendChar 2
push constant 64
call String.appendChar 2
push constant 91
call String.appendChar 2
push constant 93
call String.appendChar 2
push constant 94
call String.appendChar 2
push constant 95
call String.appendChar 2
push constant 96
call String.appendChar 2
push constant 123
call String.appendChar 2
push constant 124
call String.appendChar 2
push constant 125
call String.appendChar 2
push constant 126
call String.appendChar 2
call Output.printString 1
pop temp 0
push local 0
call Output.printString 1
pop temp 0
call Output.println 0
pop temp 0
push constant 12345
neg
call Output.printInt 1
pop temp 0
call Output.backSpace 0
pop temp 0
push constant 6789
call Output.printInt 1
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

// console.log(`memory size: ${displaySize}`);
// console.log(`display ptr: ${displayPtr}`);
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
updateRam("pointers", 0, 45, ramSize, ramPtr);
updateRam("global-stack", 256, 350, ramSize, ramPtr);
updateRam("heap", 16000, 16383, ramSize, ramPtr);

function renderLoop() {
  if (program.step(currentKey)) {
    drawMemory(
      displayWidthPixels,
      displayHeightPixels,
      pixelSize,
      displayPtr,
      displaySize
    );
  }
  updateRam("pointers", 0, 45, ramSize, ramPtr);
  updateRam("global-stack", 256, 350, ramSize, ramPtr);
  updateRam("heap", 16000, 16383, ramSize, ramPtr);
}
