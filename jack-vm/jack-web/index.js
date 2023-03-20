import { Memory, Program } from "../pkg";
import { memory } from "../pkg/jack_vm_bg";

const BIT_ON_COLOR = "#000000";
const BIT_OFF_COLOR = "#DFDFDF";
const WORD_SIZE = 16; //bits

const testProgram = `
function Main.main 3
push constant 1
pop local 1
push constant 10
push constant 20
call Output.moveCursor 2
pop temp 0
push constant 22
call String.new 1
push constant 80
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 121
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 107
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 121
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 111
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 116
call String.appendChar 2
call Output.printString 1
pop temp 0
label WHILE_EXP0
push local 2
push constant 0
eq
not
if-goto WHILE_END0
call Keyboard.keyPressed 0
pop local 2
push local 1
push constant 1499
push constant 1
sub
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 1
pop local 1
label IF_FALSE0
push local 1
push constant 1
add
pop local 1
goto WHILE_EXP0
label WHILE_END0
push constant 10
push constant 20
call Output.moveCursor 2
pop temp 0
push constant 25
call String.new 1
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 32
call String.appendChar 2
call Output.printString 1
pop temp 0
push local 1
call SnakeGame.new 1
pop local 0
push local 0
call SnakeGame.run 1
pop temp 0
push local 0
call SnakeGame.dispose 1
pop temp 0
push constant 0
return

function Mod.mod 1
push argument 0
push argument 0
push argument 1
call Math.divide 2
push argument 1
call Math.multiply 2
sub
pop local 0
push local 0
return

function Random.new 0
push constant 3
call Memory.alloc 1
pop pointer 0
push argument 0
pop this 2
push constant 1499
pop this 0
push constant 19
pop this 1
push pointer 0
return
function Random.randomRange 1
push argument 0
pop pointer 0
push pointer 0
call Random.random 1
pop local 0
push local 0
push argument 1
call Mod.mod 2
return
function Random.random 0
push argument 0
pop pointer 0
push this 1
push this 2
call Math.multiply 2
push this 0
call Mod.mod 2
pop this 2
push this 2
return

function Segment.new 0
push constant 3
call Memory.alloc 1
pop pointer 0
push argument 0
pop this 0
push argument 1
pop this 1
push constant 0
pop this 2
push pointer 0
return
function Segment.getX 0
push argument 0
pop pointer 0
push this 0
return
function Segment.getY 0
push argument 0
pop pointer 0
push this 1
return
function Segment.getPrev 0
push argument 0
pop pointer 0
push this 2
return
function Segment.setPrev 0
push argument 0
pop pointer 0
push argument 1
pop this 2
push constant 0
return
function Segment.draw 0
push argument 0
pop pointer 0
push constant 0
not
call Screen.setColor 1
pop temp 0
push this 0
push constant 1
add
push this 1
push constant 1
add
push this 0
push argument 1
add
push constant 1
sub
push this 1
push argument 1
add
push constant 1
sub
call Screen.drawRectangle 4
pop temp 0
push constant 0
return
function Segment.erase 0
push argument 0
pop pointer 0
push constant 0
call Screen.setColor 1
pop temp 0
push this 0
push constant 1
add
push this 1
push constant 1
add
push this 0
push argument 1
add
push constant 1
sub
push this 1
push argument 1
add
push constant 1
sub
call Screen.drawRectangle 4
pop temp 0
push constant 0
return
function Segment.dispose 0
push argument 0
pop pointer 0
push pointer 0
call Memory.deAlloc 1
pop temp 0
push constant 0
return

function Snake.new 0
push constant 4
call Memory.alloc 1
pop pointer 0
push argument 4
pop this 3
push this 3
push argument 0
push argument 1
push argument 2
push argument 3
push argument 5
call SnakeBoard.new 6
pop this 2
push this 2
call SnakeBoard.setBoard 1
pop temp 0
push this 2
push constant 594
call SnakeBoard.getX 2
push this 2
push constant 594
call SnakeBoard.getY 2
call Segment.new 2
pop this 0
push this 2
push constant 593
call SnakeBoard.getX 2
push this 2
push constant 593
call SnakeBoard.getY 2
call Segment.new 2
pop this 1
push this 1
push this 0
call Segment.setPrev 2
pop temp 0
push this 0
push this 3
call Segment.draw 2
pop temp 0
push this 1
push this 3
call Segment.draw 2
pop temp 0
push this 2
call SnakeBoard.makeApple 1
pop temp 0
push pointer 0
return
function Snake.step 5
push argument 0
pop pointer 0
push argument 1
push constant 1
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push this 0
call Segment.getX 1
pop local 0
push this 0
call Segment.getY 1
push this 3
sub
pop local 1
label IF_FALSE0
push argument 1
push constant 2
eq
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push this 0
call Segment.getX 1
push this 3
add
pop local 0
push this 0
call Segment.getY 1
pop local 1
label IF_FALSE1
push argument 1
push constant 3
eq
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push this 0
call Segment.getX 1
pop local 0
push this 0
call Segment.getY 1
push this 3
add
pop local 1
label IF_FALSE2
push argument 1
push constant 4
eq
if-goto IF_TRUE3
goto IF_FALSE3
label IF_TRUE3
push this 0
call Segment.getX 1
push this 3
sub
pop local 0
push this 0
call Segment.getY 1
pop local 1
label IF_FALSE3
push this 2
push local 0
push local 1
call SnakeBoard.getBoardVal 3
push constant 1
eq
if-goto IF_TRUE4
goto IF_FALSE4
label IF_TRUE4
push pointer 0
call Snake.gameOver 1
pop temp 0
label IF_FALSE4
push local 0
push local 1
call Segment.new 2
pop local 2
push this 0
push local 2
call Segment.setPrev 2
pop temp 0
push local 2
pop this 0
push this 0
push this 3
call Segment.draw 2
pop temp 0
push this 2
push local 0
push local 1
call SnakeBoard.getBoardVal 3
push constant 2
eq
not
if-goto IF_TRUE5
goto IF_FALSE5
label IF_TRUE5
push this 2
push this 1
call Segment.getX 1
push this 1
call Segment.getY 1
push constant 0
call SnakeBoard.setBoardVal 4
pop temp 0
push this 1
push this 3
call Segment.erase 2
pop temp 0
push this 1
call Segment.getPrev 1
pop local 3
push this 1
call Segment.dispose 1
pop temp 0
push local 3
pop this 1
goto IF_END5
label IF_FALSE5
push this 2
call SnakeBoard.makeApple 1
pop temp 0
push this 2
push constant 1
call SnakeBoard.drawScore 2
pop temp 0
label IF_END5
push this 2
push local 0
push local 1
push constant 1
call SnakeBoard.setBoardVal 4
pop temp 0
push constant 100
call Sys.wait 1
pop temp 0
push constant 0
return
function Snake.gameOver 0
push argument 0
pop pointer 0
push constant 10
push constant 22
call Output.moveCursor 2
pop temp 0
push constant 9
call String.new 1
push constant 71
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 109
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 79
call String.appendChar 2
push constant 118
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
call Output.printString 1
pop temp 0
call Sys.halt 0
pop temp 0
push constant 0
return
function Snake.dispose 1
push argument 0
pop pointer 0
push this 1
call Segment.getPrev 1
push constant 0
eq
not
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push this 1
call Segment.getPrev 1
pop local 0
push local 0
call Segment.dispose 1
pop temp 0
label IF_FALSE0
push pointer 0
call Memory.deAlloc 1
pop temp 0
push constant 0
return

function SnakeBoard.new 0
push constant 14
call Memory.alloc 1
pop pointer 0
push argument 4
pop this 5
push argument 3
pop this 6
push argument 0
pop this 4
push this 5
pop this 8
push this 6
pop this 10
push this 5
push argument 2
add
push this 4
sub
pop this 7
push this 6
push argument 1
add
push this 4
sub
pop this 9
push argument 2
push this 4
call Math.divide 2
pop this 1
push argument 1
push this 4
call Math.divide 2
pop this 2
push this 1
push this 2
call Math.multiply 2
pop this 3
push this 3
call Array.new 1
pop this 0
push argument 5
call Random.new 1
pop this 11
push argument 5
call Random.new 1
pop this 12
push constant 0
pop this 13
push pointer 0
push constant 0
call SnakeBoard.drawScore 2
pop temp 0
push pointer 0
return
function SnakeBoard.drawScore 0
push argument 0
pop pointer 0
push this 13
push argument 1
add
pop this 13
push constant 20
push constant 50
call Output.moveCursor 2
pop temp 0
push constant 6
call String.new 1
push constant 83
call String.appendChar 2
push constant 99
call String.appendChar 2
push constant 111
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 58
call String.appendChar 2
call Output.printString 1
pop temp 0
push this 13
call Output.printInt 1
pop temp 0
push constant 0
return
function SnakeBoard.setBoard 3
push argument 0
pop pointer 0
push constant 0
pop local 0
push constant 0
not
call Screen.setColor 1
pop temp 0
label WHILE_EXP0
push local 0
push this 3
lt
not
if-goto WHILE_END0
push pointer 0
push local 0
call SnakeBoard.getX 2
pop local 1
push pointer 0
push local 0
call SnakeBoard.getY 2
pop local 2
push local 1
push this 8
eq
push local 1
push this 7
eq
or
push local 2
push this 10
eq
or
push local 2
push this 9
eq
or
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push local 0
push this 0
add
push constant 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 1
push constant 1
add
push local 2
push constant 1
add
push local 1
push this 4
add
push constant 1
sub
push local 2
push this 4
add
push constant 1
sub
call Screen.drawRectangle 4
pop temp 0
goto IF_END0
label IF_FALSE0
push local 0
push this 0
add
push constant 0
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 1
push local 2
call Screen.drawPixel 2
pop temp 0
label IF_END0
push local 0
push constant 1
add
pop local 0
goto WHILE_EXP0
label WHILE_END0
push constant 0
return
function SnakeBoard.getBoardVal 0
push argument 0
pop pointer 0
push pointer 0
push argument 1
push argument 2
call SnakeBoard.getI 3
push this 0
add
pop pointer 1
push that 0
return
function SnakeBoard.setBoardVal 0
push argument 0
pop pointer 0
push pointer 0
push argument 1
push argument 2
call SnakeBoard.getI 3
push this 0
add
push argument 3
pop temp 0
pop pointer 1
push temp 0
pop that 0
push constant 0
return
function SnakeBoard.makeApple 5
push argument 0
pop pointer 0
push this 11
push this 1
call Random.randomRange 2
pop local 0
push this 12
push this 2
call Random.randomRange 2
pop local 1
push local 1
push this 1
call Math.multiply 2
push local 0
add
pop local 2
push pointer 0
push local 2
call SnakeBoard.getX 2
pop local 3
push pointer 0
push local 2
call SnakeBoard.getY 2
pop local 4
push pointer 0
push local 3
push local 4
call SnakeBoard.getBoardVal 3
push constant 0
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push pointer 0
push local 3
push local 4
push constant 2
call SnakeBoard.setBoardVal 4
pop temp 0
push local 3
push this 4
push constant 2
call Math.divide 2
add
push local 4
push this 4
push constant 2
call Math.divide 2
add
push this 4
push constant 3
call Math.divide 2
call Screen.drawCircle 3
pop temp 0
goto IF_END0
label IF_FALSE0
push pointer 0
call SnakeBoard.makeApple 1
pop temp 0
label IF_END0
push constant 0
return
function SnakeBoard.getX 2
push argument 0
pop pointer 0
push argument 1
push this 1
call Math.divide 2
pop local 1
push argument 1
push this 1
push local 1
call Math.multiply 2
sub
pop local 0
push local 0
push this 4
call Math.multiply 2
push this 5
add
return
function SnakeBoard.getY 1
push argument 0
pop pointer 0
push argument 1
push this 1
call Math.divide 2
pop local 0
push local 0
push this 4
call Math.multiply 2
push this 6
add
return
function SnakeBoard.getI 2
push argument 0
pop pointer 0
push argument 1
push this 5
sub
push this 4
call Math.divide 2
pop local 0
push argument 2
push this 6
sub
push this 4
call Math.divide 2
pop local 1
push local 1
push this 1
call Math.multiply 2
push local 0
add
return

function SnakeGame.new 0
push constant 10
call Memory.alloc 1
pop pointer 0
push constant 8
pop this 2
push constant 29
push this 2
call Math.multiply 2
pop this 3
push constant 41
push this 2
call Math.multiply 2
pop this 4
push constant 2
push this 2
call Math.multiply 2
pop this 5
push constant 6
push this 2
call Math.multiply 2
pop this 6
push constant 2
pop this 1
push constant 0
pop this 7
push constant 50
pop this 9
push constant 20
pop this 8
push this 3
push this 4
push this 5
push this 6
push this 2
push argument 0
call Snake.new 6
pop this 0
push pointer 0
return
function SnakeGame.dispose 0
push argument 0
pop pointer 0
push this 0
call Snake.dispose 1
pop temp 0
push pointer 0
call Memory.deAlloc 1
pop temp 0
push constant 0
return
function SnakeGame.run 3
push argument 0
pop pointer 0
push constant 0
pop local 1
push constant 0
pop local 2
label WHILE_EXP0
push local 1
if-goto WHILE_END0
label WHILE_EXP1
push local 0
push constant 0
eq
not
if-goto WHILE_END1
call Keyboard.keyPressed 0
pop local 0
push this 0
push this 1
push local 2
call Snake.step 3
pop temp 0
goto WHILE_EXP1
label WHILE_END1
push local 0
push constant 81
eq
if-goto IF_TRUE0
goto IF_FALSE0
label IF_TRUE0
push constant 0
not
pop local 1
label IF_FALSE0
push local 0
push constant 131
eq
if-goto IF_TRUE1
goto IF_FALSE1
label IF_TRUE1
push constant 1
pop this 1
label IF_FALSE1
push local 0
push constant 132
eq
if-goto IF_TRUE2
goto IF_FALSE2
label IF_TRUE2
push constant 2
pop this 1
label IF_FALSE2
push local 0
push constant 133
eq
if-goto IF_TRUE3
goto IF_FALSE3
label IF_TRUE3
push constant 3
pop this 1
label IF_FALSE3
push local 0
push constant 130
eq
if-goto IF_TRUE4
goto IF_FALSE4
label IF_TRUE4
push constant 4
pop this 1
label IF_FALSE4
label WHILE_EXP2
push local 0
push constant 0
eq
if-goto WHILE_END2
call Keyboard.keyPressed 0
pop local 0
push this 0
push this 1
push local 2
call Snake.step 3
pop temp 0
goto WHILE_EXP2
label WHILE_END2
goto WHILE_EXP0
label WHILE_END0
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
  // const id = ctx.getImageData(0, 0, displayCanvas.width, displayCanvas.height);

  // console.log(id.data)

  const id = ctx.createImageData(displayCanvas.width, displayCanvas.height);

  // console.log(id_create.data)

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
      pixels[offset + 3] = 255;
    } else {
      // set corresponding virtual pixel to black
      const offset = i * 4;
      pixels[offset] = 10;
      pixels[offset + 1] = 10;
      pixels[offset + 2] = 10;
      pixels[offset + 3] = 255;
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
updateRam("pointers", 0, 45, ramSize, ramPtr);
updateRam("global-stack", 256, 350, ramSize, ramPtr);
updateRam("heap", 16000, 16383, ramSize, ramPtr);

// If drawMemory is disabled, fonts will not be rendered! 
// font printing has not been implemented for the wasm canvas

function renderLoop() {
  for (let i = 0; i < 15; i++) {
    program.step(currentKey);
  }
  // drawMemory();
  updateRam("pointers", 0, 45, ramSize, ramPtr);
  updateRam("global-stack", 256, 350, ramSize, ramPtr);
  updateRam("heap", 16000, 16383, ramSize, ramPtr);
}
