import { Memory } from "../pkg";
import { memory } from "../pkg/jack_vm_bg";

const CELL_SIZE = 2; //px
const BIT_ON_COLOR = "#000000";
const BIT_OFF_COLOR = "#DFDFDF";
const WORD_SIZE = 16; //bits

const programMemory = Memory.initialize();

const displaySize = programMemory.display_size();

// total memory size can be shown on a 768 x 512 grid. 
// If each memory cell is represented as a 2x2 pixel on a grid, it might even be legible!
const height = 256;
const width_blocks = displaySize / height;
const width = width_blocks * WORD_SIZE; // 48 blocks of 16 bits = 768

// console.log(`memory size: ${displaySize}`);
// console.log(`height in rows: ${height}`);
// console.log(`width in columns: ${width}`);
// console.log(`width in blocks: ${width_blocks}`);

const canvas = document.getElementById("memory-canvas");
canvas.width = width * (CELL_SIZE + 1);
canvas.height = height * (CELL_SIZE + 1);

const ctx = canvas.getContext('2d');

const getIndex = (row, col) => {
    return row * width + col;
}

// from rustwasm tutorial
const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 16);
    const mask = 1 << (n % 16);
    return (arr[byte] & mask) === mask;
    };

const drawDisplay = () => {
    // const ramPtr = programMemory.ram();
    const displayPtr = programMemory.display();
    // const keyBoard =  programMemory.keyboard();
    
    const display = new Uint16Array(memory.buffer, displayPtr, displaySize)

    // pull pixels out of the canvas
    // var id = ctx.getImageData(0, 0, canvas.width, canvas.height);
    // pixels is a Uint8ClampedArray, each pixel being 4 consecutive Uint8 values
    // representing r, g, b, and a respectively
    // var pixels = id.data

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const index = getIndex(row, col);
            // console.log(`row:${row}, block:${block}, index:${index}, value:${ram[index].toString(2)}`)
            ctx.fillStyle = bitIsSet(index, display)
                ? BIT_ON_COLOR 
                : BIT_OFF_COLOR;
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    // place pixels back in the canvas
    // ctx.putImageData(id, 0, 0)
}

const drawBlock = (row, block, value) => {
    // get an array of 16 bits from value (I guess this is the way to do it in JS (╯°□°）╯︵ ┻━┻)
    const bits_as_string = value.toString(2);

    // draw the 16 rectangles that represent those bits
    for (let i = 0; i <  WORD_SIZE; i++) {

        const bit = bits_as_string[i];
        // when JS casts the number to a string, it loses leading zeros
        ctx.fillStyle = bit === '0'|| bit === undefined
            ? BIT_OFF_COLOR 
            : BIT_ON_COLOR;
        ctx.fillRect(
            ((block * WORD_SIZE) + i ) * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
        );
    }
}

const renderLoop = () => {
    programMemory.step();

    drawDisplay();
    requestAnimationFrame(renderLoop);
}

drawDisplay();
requestAnimationFrame(renderLoop);



