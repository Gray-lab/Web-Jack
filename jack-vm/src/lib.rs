mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

type Index = usize;
type NumVars = usize;
type NumArgs = usize;
type LabelName = String;
type FunctionName = String;

struct VMToken {
    command: Command,
    line: usize,
}

#[derive(PartialEq)]
pub enum Command {
    Pop(Segment, Index), 
    Push(Segment, Index),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And, 
    Or, 
    Not,
    GoTo(LabelName),
    IfGoTo(LabelName),
    Label(LabelName),
    Function(FunctionName, NumVars),
    Call(FunctionName, NumArgs),
    Return,
}

#[derive(PartialEq)]
pub enum Segment {
    // Stack Pointer
    Pointer,
    Constant,
    Local,
    Argument,
    Static,
    This,
    That,
    Temp,
}
// Need help figuring out how to send errors back
fn parse_vm_code(text: &str) /*-> Vec<VMToken>*/{

    // let mut tokens = Vec::new();

    for (i, line) in text.lines().map(|line| {
        // remove comments:
        // delete any text between '//' and end of line'
        if let Some(i) = line.find("//") {
            // console::log_1(&"Inside if\n".into());
            &line[..i]
        } else {
            // console::log_1(&"Inside else\n".into());
            line
        }
    }).enumerate() {
        // console.log the line
        let js: JsValue = line.into();
        console::log_1(&js);

        if line.trim().is_empty() {
            continue;
        }

        // Check the number of arguments in each line
        let line_words: Vec<&str> = line.trim().split(" ").collect();

        match line_words.len() {
            1 => console::log_1(&"Line length 1\n".into()),
            2 => console::log_1(&"Line length 2\n".into()),
            3 => console::log_1(&"Line length 3\n".into()),
            _ => console::log_1(&"Something Else :(\n".into()),
        }
    }
    // tokens.push(VMToken{command:Command::Add, line:0});
    // tokens
    // words
    
}

// fn test_token() -> Vec<VMToken> {
//     let mut tokens = Vec::new();
//     tokens.push(VMToken{command:Command::Add, line:0});
//     tokens.push(VMToken{command:Command::Sub, line:0});
//     tokens.push(VMToken{command:Command::Eq, line:0});
//     tokens


#[wasm_bindgen]
pub fn run(input: String) {
    use web_sys::console;

    let mut result = String::from("").to_owned();
    let words = parse_vm_code(&input);
    // for word in words {
    //     result.push_str(&word);
    //     result.push_str("\n");
    // }
    // result
}