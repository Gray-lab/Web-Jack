import * as wasm from "../pkg";

const pre = document.getElementById("test");
// const string = "This was a triumph\nMaking a note here, huge success\n//this is a comment\nthere was one here too -> //comment"
const string = "//starting with a comment \n push constant 4//comment\n \t\t   \n pop this 3 \n add"
const test_string = wasm.run(string);
pre.textContent = test_string;