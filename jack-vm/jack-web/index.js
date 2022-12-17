import { Memory, Program } from "../pkg";
import { memory } from "../pkg/jack_vm_bg";

const BIT_ON_COLOR = "#000000";
const BIT_OFF_COLOR = "#DFDFDF";
const WORD_SIZE = 16; //bits

const testProgram =`class Main
function main 0
push constant 1
call Screen.setColor 1
pop temp 0
push constant 400
push constant 100
push constant 450
push constant 120
call Screen.drawRectangleOutline 4
pop temp 0
push constant 400
push constant 130
push constant 450
push constant 150
call Screen.drawRectangle 4
pop temp 0
push constant 20
push constant 20
push constant 15
call Screen.drawCircle 3
pop temp 0
push constant 500
push constant 250
push constant 200
call Screen.drawCircle 3
pop temp 0
label END
goto END
`;

// set lcl=260 so we can see it near sp at 256
const program = new Program(testProgram, 265);

// total memory size can be shown on a 768 x 512 grid. 
// If each memory cell is represented as a 2x2 pixel on a grid, it might even be legible!
console.log("before program")
const displaySize = program.display_size();
const displayPtr = program.display();

const ramSize = program.ram_size();
const ramPtr = program.ram();

// display dimensions in virtual pixels
const displayWidthPixels = 512;
const displayHeightPixels = 256;
const pixelSize = 2; // size of each virtual pixel in real pixels

console.log(`memory size: ${displaySize}`);
// console.log(`height in rows: ${height}`);
// console.log(`width in columns: ${width}`);
// console.log(`width in blocks: ${width_blocks}`);

// set display canvas size
const displayCanvas = document.getElementById("display-canvas");
displayCanvas.width = displayWidthPixels * pixelSize;
displayCanvas.height = displayHeightPixels * pixelSize;

const ctx = displayCanvas.getContext('2d', { willReadFrequently: true });
ctx.scale(pixelSize, pixelSize);

// from rustwasm tutorial
function bitIsSet(n, arr) {
    const word = Math.floor(n / 16);
    const mask = 1 << (n % 16);
    return (arr[word] & mask) === mask;
};

function drawMemory(widthPixels, heightPixels, pixelSize, memPtr, memSize) {
    const memArray = new Uint16Array(memory.buffer, memPtr, memSize)
    // pull pixels out of the canvas
    const id = ctx.getImageData(0, 0, displayCanvas.width, displayCanvas.height);
    // pixels is a Uint8ClampedArray, each pixel being 4 consecutive Uint8 values
    // representing r, g, b, and a respectively
    const pixels = id.data

    for (let i = 0; i < memSize * 16; i++) {
        if (bitIsSet(i, memArray)) {
            // set corresponding virtual pixel to red
            drawVirtualPixel(pixels, i, pixelSize, widthPixels, 0, 255, 0, 255)
        } else {
            // set corresponding virtual pixel to black
            drawVirtualPixel(pixels, i, pixelSize, widthPixels, 10, 10, 10, 255)
        }
    }

    // place pixels back in the canvas
    ctx.putImageData(id, 0, 0)
}

function drawVirtualPixel(pixels, index, pixelSize, widthPixels, r, g, b, a) {
    // index 0 ->
    for (let y = 0; y < pixelSize; y++) {
        for (let x = 0; x < pixelSize; x++) {
            const x_shift = x * 4
            const y_shift = Math.floor(index / widthPixels) * widthPixels * 4 * (pixelSize == 1? 0 : pixelSize) + y * widthPixels * 4 * pixelSize
            const offset = index * 4 * pixelSize + x_shift + y_shift// 4 values per pixel
            pixels[offset] = r
            pixels[offset + 1] = g
            pixels[offset + 2] = b
            pixels[offset + 3] = a
        }
    }
}

function updateRamList(start, end, memSize, memPtr) {
    const ramContainer = document.getElementById("ram-container");
    if (document.getElementById("ram-list")) {
        document.getElementById("ram-list").remove();
    }
    const outer = document.createElement('div');
    const memArray = new Int16Array(memory.buffer, memPtr, memSize)
    outer.setAttribute('id', 'ram-list')
    for (let i = start; i <= end; i++) {
        const inner = document.createElement('div');
        inner.innerHTML = `ram[${i}]:\t ${memArray[i]}`
        outer.appendChild(inner);
    }
    ramContainer.appendChild(outer)
}


// let i = 0
// let do_log = true
// setTimeout(() => {
//     do_log = false
// }, 10000)
function renderLoop() {
    drawMemory(displayWidthPixels, displayHeightPixels, pixelSize, displayPtr, displaySize);
    updateRamList(256, 270, ramSize, ramPtr)
    // program.set_display(i, i)
    // i++
    program.step()
    // if (i < displaySize) {
        // requestAnimationFrame(renderLoop)
    // }
    
    // if (do_log) {
    //     console.log(1)
    // }
}
// requestAnimationFrame(renderLoop)
setInterval(() => requestAnimationFrame(renderLoop), 100)



