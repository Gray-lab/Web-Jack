<script>
  import { onMount } from "svelte";

  export let bindings;
  let { memory, Program } = bindings;
  console.log(Program);
  console.log(memory);

  let name = "world";
  let count = 0;

  const testProgram = `
  function Main.main 0
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
  goto END`;

  let canvas;
  let ctx;
  let program;
  // display dimensions in virtual pixels
  const width = 512;
  const height = 256;
  const pixelRatio = 1; // size of each virtual pixel in real pixels

  function renderLoop() {
    for (let i = 0; i < 5; i++) {
      program.step(0);
    }
    // drawMemory();
    //   updateRam("pointers", 0, 45, ramSize, ramPtr);
    //   updateRam("global-stack", 256, 350, ramSize, ramPtr);
    //   updateRam("heap", 16000, 16383, ramSize, ramPtr);
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

  function onKeyDown(e) {
    console.log(e.key);
  }

  onMount(() => {
    ctx = canvas.getContext("2d", {
      willReadFrequently: true,
      alpha: false,
    });
    canvas.setAttribute("id", "display-canvas");

    program = new Program(testProgram, ctx, canvas);

    // // total memory size can be shown on a 768 x 512 grid.
    // // If each memory cell is represented as a 2x2 pixel on a grid, it might even be legible!
    const displaySize = program.display_size();
    const displayPtr = program.display();

    const ramSize = program.ram_size();
    const ramPtr = program.ram();
  });

  // // from rustwasm tutorial
  // function bitIsSet(n, arr) {
  //   const word = Math.floor(n / 16);
  //   const mask = 1 << n % 16;
  //   return (arr[word] & mask) === mask;
  // }

  // const displayArray = new Uint16Array(memory.buffer, displayPtr, displaySize);

  // function drawMemory() {
  //   // pull pixels out of the canvas
  //   // const id = ctx.getImageData(0, 0, displayCanvas.width, displayCanvas.height);

  //   // console.log(id.data)

  //   const id = ctx.createImageData(displayCanvas.width, displayCanvas.height);

  //   // console.log(id_create.data)

  //   // pixels is a Uint8ClampedArray, each pixel being 4 consecutive Uint8 values
  //   // representing r, g, b, and a respectively

  //   const pixels = id.data;

  //   for (let i = 0; i < displaySize * 16; i++) {
  //     if (bitIsSet(i, displayArray)) {
  //       // set corresponding virtual pixel to green
  //       const offset = i * 4;
  //       pixels[offset] = 0;
  //       pixels[offset + 1] = 255;
  //       pixels[offset + 2] = 0;
  //       pixels[offset + 3] = 255;
  //     } else {
  //       // set corresponding virtual pixel to black
  //       const offset = i * 4;
  //       pixels[offset] = 10;
  //       pixels[offset + 1] = 10;
  //       pixels[offset + 2] = 10;
  //       pixels[offset + 3] = 255;
  //     }
  //   }
  //   // place pixels back in the canvas
  //   ctx.putImageData(id, 0, 0);
  // }

  // function updateRam(id, start, end, memSize, memPtr) {
  //   const ramContainer = document.getElementById(id);
  //   if (document.getElementById(id + "ram")) {
  //     document.getElementById(id + "ram").remove();
  //   }
  //   const outer = document.createElement("div");
  //   const memArray = new Int16Array(memory.buffer, memPtr, memSize);
  //   outer.setAttribute("id", id + "ram");
  //   for (let i = start; i <= end; i++) {
  //     const inner = document.createElement("div");
  //     inner.innerHTML = `ram[${i}]:\t ${memArray[i]}`;
  //     outer.appendChild(inner);
  //   }
  //   ramContainer.appendChild(outer);
  // }

  // let currentKey = 0;
  // const body = document.querySelector("body");
  // body.addEventListener("keydown", (event) => {
  //   console.log(event.key);
  //   if (event.key.length === 1) {
  //     currentKey = event.key.charCodeAt(0);
  //   } else if (event.key in input_map) {
  //     currentKey = input_map[event.key];
  //   }
  // });

  // body.addEventListener("keyup", (event) => {
  //   if (event.key.length === 1 || event.key in input_map) {
  //     currentKey = 0;
  //   }
  // });

  // // Button listeners
  // const stepButton = document.getElementById("step-button");
  // stepButton.addEventListener("click", (event) => {
  //   renderLoop();
  // });

  // const runButton = document.getElementById("run-button");
  // runButton.addEventListener("click", (event) => {
  //   setInterval(() => requestAnimationFrame(renderLoop), 0);
  // });

  // // // Render the starting state
  // drawMemory(
  //   displayWidthPixels,
  //   displayHeightPixels,
  //   pixelSize,
  //   displayPtr,
  //   displaySize
  // );
  // updateRam("pointers", 0, 45, ramSize, ramPtr);
  // updateRam("global-stack", 256, 350, ramSize, ramPtr);
  // updateRam("heap", 16000, 16383, ramSize, ramPtr);

  // // // There is some weird bug here with text not displaying if updateRam and drawMemory are disabled
</script>

<h1>Hello {name}!</h1>
<input bind:value={name} />

<button on:click={() => (count += 1)}>
  Clicks: {count}
</button>

<div id="container">
  <div>
    <canvas
      bind:this={canvas}
      width={width * pixelRatio}
      height={height * pixelRatio}
      style="width: {width}px; height: {height}px;"
    />
    <br />
    <button on:click={() => renderLoop()}>Step</button>
    <button
      on:click={() => setInterval(() => requestAnimationFrame(renderLoop), 0)}
      >Run</button
    >
  </div>
  <div class="ram" id="pointers" />
  <div class="ram" id="global-stack" />
  <div class="ram" id="heap" />
</div>

<svelte:window on:keydown|preventDefault={onKeyDown} />
