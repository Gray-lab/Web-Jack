import { Memory, Program } from "../pkg";
import { memory } from "../pkg/jack_vm_bg";
import { snakeProgram, sevenProgram } from "./bytecode";

let program = new Program(sevenProgram);
let ramSize = program.ram_size();
let ramPtr = program.ram();

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
const loadButton = document.getElementById("load-button");
loadButton.addEventListener("click", (event) => {
  program = new Program(snakeProgram);
  ramSize = program.ram_size();
  ramPtr = program.ram();
});

const stepButton = document.getElementById("step-button");
stepButton.addEventListener("click", (event) => {
  renderLoop();
});

const runButton = document.getElementById("run-button");
runButton.addEventListener("click", (event) => {
  const interval = setInterval(() => {
    requestAnimationFrame(renderLoop);
    if (program.finished) {
      console.log("Program finished. Exiting renderLoop.");
      clearInterval(interval);
    }
  }, 0);
});

const stopButton = document.getElementById("stop-button");
stopButton.addEventListener("click", (event) => {
  program.end();
});

// Initialize ram display
updateRam("pointers", 0, 45, ramSize, ramPtr);
updateRam("global-stack", 256, 350, ramSize, ramPtr);
updateRam("heap", 16000, 16383, ramSize, ramPtr);

// Program loop callback
function renderLoop() {
  for (let i = 0; i < 25; i++) {
    program.step(currentKey)
  }
  console.log("here")
  updateRam("pointers", 0, 45, ramSize, ramPtr);
  updateRam("global-stack", 256, 350, ramSize, ramPtr);
  updateRam("heap", 16000, 16383, ramSize, ramPtr);
}
