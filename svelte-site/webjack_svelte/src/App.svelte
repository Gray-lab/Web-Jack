<script>
  import { onMount } from "svelte";
  import CodeMirror, { basicSetup } from "./CodeMirror.svelte";

  export let bindings;
  let { memory, Program } = bindings;
  console.log(Program);
  console.log(memory);

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

  let bytecode = "hi there";
  let jackcode = "hello jack";

  function changeHandler({ detail: { tr } }) {
    console.log("change", tr.changes.toJSON());
  }

  function renderLoop() {
    for (let i = 0; i < 5; i++) {
      program.step(0);
    }
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

  function onKeyUp(e) {
    
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

  // // Button listeners
  // const stepButton = document.getElementById("step-button");
  // stepButton.addEventListener("click", (event) => {
  //   renderLoop();
  // });

  // const runButton = document.getElementById("run-button");
  // runButton.addEventListener("click", (event) => {
  //   setInterval(() => requestAnimationFrame(renderLoop), 0);
  // });

  // updateRam("pointers", 0, 45, ramSize, ramPtr);
  // updateRam("global-stack", 256, 350, ramSize, ramPtr);
  // updateRam("heap", 16000, 16383, ramSize, ramPtr);
</script>

<!-- <div id="container">
  <div>
    <br />
  </div>
  <div class="ram" id="pointers" />
  <div class="ram" id="global-stack" />
  <div class="ram" id="heap" />
</div> -->

<title>Hi</title>
<body>
  <div class="container">
    <div class="top">
      <div class="left_top">
        <p>Web_Jack</p>
      </div>
      <div class="right_top">
        <canvas
          bind:this={canvas}
          width={width * pixelRatio}
          height={height * pixelRatio}
          style="width: {width}px; height: {height}px;"
        />
      </div>
    </div>
    <div class="bottom">
      <div class="cm-container">
        <div class="cm">
          <CodeMirror
            doc={"Edit me!\nAnd here is the second line!!"}
            bind:docStore={jackcode}
            extensions={basicSetup}
            on:change={changeHandler}
          />
        </div>
        <div class="btn-container">
          <button class="btn">Button 1</button>
          <button class="btn">Button 2</button>
          <button class="btn">Button 3</button>
        </div>
      </div>
      <div class="cm-container">
        <div class="cm">
          <CodeMirror
            doc={testProgram}
            bind:docStore={bytecode}
            extensions={basicSetup}
            on:change={changeHandler}
          />
        </div>
        <div class="btn-container">
          <button class="btn" on:click={() => renderLoop()}>Step</button>
          <button
            class="btn"
            on:click={() =>
              setInterval(() => requestAnimationFrame(renderLoop), 0)}
            >Run</button
          >
        </div>
      </div>
      <div class="empty" />
      <div class="empty" />
    </div>
  </div>
</body>
<!-- <svelte:window on:keydown|preventDefault={onKeyDown} /> -->

<style>
  body {
    height: 100%;
    margin: 5;
    padding: 5;
    background-color: #616161;
  }
  .container {
    height: 100%;
    display: grid;
    grid-template-rows: 280px;
    grid-template-columns: 100%;
  }
  .top {
    height: 100%;
    display: flex;
    flex-direction: row;
    grid-row: 1;
    grid-column: 1;
  }
  .bottom {
    height: 100%;
    display: flex;
    flex-direction: row;
    grid-row: 2;
    grid-column: 1;
  }
  .left_top {
    flex: 1;
    background-color: #eee;
    padding: 10px;
  }
  .right_top {
    flex: 0 0 512px;
    background-color: #ddd;
    padding: 10px;
  }
  .cm-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 10px;
  }
  .cm {
    flex: 1;
    border: 1px solid #ccc;
    margin-bottom: 10px;
  }
  .btn-container {
    display: flex;
    flex-direction: row;
    justify-content: right;
  }
  .btn {
    margin-right: 10px;
    background-color: #323232;
    color: white;
    padding: 5px 10px;
    border: none;
    border-radius: 3px;
    cursor: pointer;
  }
  .empty {
    flex: 1;
    background-color: #eee;
    padding: 10px;
  }
</style>
