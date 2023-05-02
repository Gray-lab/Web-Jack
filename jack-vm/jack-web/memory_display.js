// contains functions that were originally used to draw the canvas based on the display memory
const BIT_ON_COLOR = "#000000";
const BIT_OFF_COLOR = "#DFDFDF";
const WORD_SIZE = 16; //bits

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
