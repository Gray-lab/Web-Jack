mod utils;

use std::collections::HashMap;

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

pub struct VMCommand {
    command: Command,
    line: usize,
}

type Index = usize;
type NumVars = usize;
type NumArgs = usize;
type LabelName = String;
type FunctionName = String;

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

fn parse_segment(seg_name: &str) -> Segment {
    match seg_name {
        "pointer" => Segment::Pointer,
        "constant" => Segment::Constant,
        "local" => Segment::Local,
        "argument" => Segment::Argument,
        "static" => Segment::Static,
        "this" => Segment::This,
        "that" => Segment::That,
        "temp" => Segment::Temp,
        otherwise => { 
            console::log_2(&otherwise.into(), &" is an invalid segment name".into());
            panic!("{} is not a valid segment name", otherwise);
        }
    }
}

// struct Class {
//     functions: HashMap<String, Function>,
// }

// struct Function {
//     arg_count: usize,
//     commands: Vec<VMCommand>,
//     parent_class: &Class,
//     label_table: HashMap<String, usize>
// }

// Need help figuring out how to send errors back
fn parse_vm_code(text: &str) -> Vec<VMCommand>{

    let mut tokens = Vec::new();

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
        
        let line_words: Vec<&str> = line.trim().split(" ").collect();
        let parsed_line: VMCommand;

        // Check the number of arguments in each line
        // The nested match statements could be factored out into VMCommand implementations.
        // Does Rust support polymorphic functions?
        match line_words.len() {
            1 => {
                match line_words[0] {
                    "add" => parsed_line = VMCommand {command: Command::Add, line: i},
                    "sub" => parsed_line = VMCommand {command: Command::Sub, line: i},
                    "neg" => parsed_line = VMCommand {command: Command::Neg, line: i},
                    "eq" => parsed_line = VMCommand {command: Command::Eq, line: i},
                    "gt" => parsed_line = VMCommand {command: Command::Gt, line: i},
                    "lt" => parsed_line = VMCommand {command: Command::Lt, line: i},
                    "and" => parsed_line = VMCommand {command: Command::And, line: i},
                    "or" => parsed_line = VMCommand {command: Command::Or, line: i},
                    "not" => parsed_line = VMCommand {command: Command::Not, line: i},
                    "return" => parsed_line = VMCommand {command: Command::Return, line: i},
                    otherwise => {
                        console::log_1(&"Invalid zero argument command".into());
                        panic!("Invalid zero argument command at line {}: {}", i, otherwise);
                    }
                }
            }
            2 => {
                match (line_words[0], line_words[1]) {
                    ("goto", label) => parsed_line = VMCommand {command: Command::GoTo(label.to_string()), line: i},
                    ("ifgoto", label) => parsed_line = VMCommand {command: Command::IfGoTo(label.to_string()), line: i},
                    ("label", label) => parsed_line = VMCommand {command: Command::Label(label.to_string()), line: i},
                    (otherwise, _) => {
                        console::log_1(&"Invalid one argument command".into());
                        panic!("Invalid one argument command at line {}: {}", i, otherwise);
                    }
                }
            }
            3 => {
                match (line_words[0], line_words[1], line_words[2].parse::<usize>().expect("Second argument should be parsable to an i32")) {
                    ("pop", segment, index) => parsed_line = VMCommand {command: Command::Pop(parse_segment(segment), index), line: i},
                    ("push", segment, index) => parsed_line = VMCommand {command: Command::Push(parse_segment(segment), index), line: i},
                    ("function", fn_name, num_vars) => parsed_line = VMCommand {command: Command::Function(fn_name.to_string(), num_vars), line: i},
                    ("call", fn_name, num_args) => parsed_line = VMCommand {command: Command::Call(fn_name.to_string(), num_args), line: i},
                    (otherwise, _, _) => {
                        console::log_1(&"Invalid two argument command".into());
                        panic!("Invalid two argument command at line {}: {}", i, otherwise);
                    }
                } 

            }
            otherwise => {
                console::log_2(&"Invalid syntax at line:".into(), &i.into());
                panic!("Invalid syntax at line {}. Expecting 0, 1, or two arguments, but was given {}", i, otherwise);
            }
        }
        tokens.push(parsed_line);
    }

    tokens
    
}


#[wasm_bindgen]
pub fn run(input: String) {
    let mut result = String::from("").to_owned();
    let words = parse_vm_code(&input);
    // for word in words {
    //     result.push_str(&word);
    //     result.push_str("\n");
    // }
    // result
}



/**
 * Memory array:
 * 0-16383 16 bit main memory (0x0000-0x3fff)
 * 16384-24575 16 bit screen (0x4000-0x5fff) -> pixel (r, c) is mapped onto the c%16 bit of the 
 * 16 bit word stored at Screen \[r * 32 + c / 16\]
 * This needs to be exposed to javascript to allow for screen display
 * 24576 is 16 bit value for keyboard press (0x6000)
 * This needs to be updated continuously to allow for user input
 */
#[wasm_bindgen]
pub struct Memory {
    ram: Vec<u16>,
    display: Vec<u16>,
    // keyboard: u16,
    // something to keep track of allocations on heap for when objects are implemented
    curr_idx: usize,
}

struct Block {
    pointer: u16,
    size: u16,
}

const RAM_SIZE: usize = 16384;
const DISPLAY_SIZE: usize = 8192; // 256x512
const KEYBOARD: usize = 24576;

#[wasm_bindgen]
impl Memory {
    pub fn initialize() -> Memory {
        let ram:Vec<u16> = (0..RAM_SIZE).map(|_i| 0x0000).collect();
        let display:Vec<u16> = (0..DISPLAY_SIZE).map(|_i| 0x0000).collect();
        // let keyboard = 0 as u16;
        let curr_idx = 0;

        Memory { ram, display, curr_idx }
        // Memory { ram, display, keyboard }
    }

    // pub fn peek(&self, index: usize) -> u16 {
    //     if index < RAM_SIZE {
    //         self.ram[index]
    //     } else if index < RAM_SIZE + DISPLAY_SIZE {
    //         self.display[index]
    //     } else if index == KEYBOARD {
    //         self.keyboard
    //     } else {
    //         panic!("Index out of bounds. Valid indexes range from 0 to {}", KEYBOARD);
    //     }
    // }

    pub fn poke(&mut self, index: usize, value: u16) {
        // should I accept a u32 and coerce it into a u16?
        self.ram[index] = value;
    }

    pub fn ram(&self) -> *const u16 {
        self.ram.as_ptr()
    }

    pub fn display(&self) -> *const u16 {
        self.display.as_ptr()
    }

    pub fn step(&mut self) {
        // console::log_1(&self.curr_idx.into());
        match self.display[self.curr_idx] {
            0x0000 => self.display[self.curr_idx] = 0x0001,
            0x0001 => self.display[self.curr_idx] = 0x0003,
            0x0003 => self.display[self.curr_idx] = 0x0007,
            0x0007 => self.display[self.curr_idx] = 0x000F,
            0x000F => self.display[self.curr_idx] = 0x001F,
            0x001F => self.display[self.curr_idx] = 0x003F,
            0x003F => self.display[self.curr_idx] = 0x007F,
            0x007F => self.display[self.curr_idx] = 0x00FF,
            0x00FF => self.display[self.curr_idx] = 0x01FF,
            0x01FF => self.display[self.curr_idx] = 0x03FF,
            0x03FF => self.display[self.curr_idx] = 0x07FF,
            0x07FF => self.display[self.curr_idx] = 0x0FFF,
            0x0FFF => self.display[self.curr_idx] = 0x1FFF,
            0x1FFF => self.display[self.curr_idx] = 0x3FFF,
            0x3FFF => self.display[self.curr_idx] = 0x7FFF,
            0x7FFF => self.display[self.curr_idx] = 0xFFFF,
            0xFFFF => {
                self.curr_idx += 1;
                self.display[self.curr_idx] = 0x0001;
            }
            _ => panic!("This is all wrong!")
        }
    }
    

    // pub fn keyboard(&self) -> u16 {
    //     self.keyboard
    // }

    pub fn ram_size(&self) -> u16 {
        self.ram.len() as u16
    }

    pub fn display_size(&self) -> u16 {
        self.display.len() as u16
    }

    /**
     * Allocates a block of memory of at least 'size' words
     * Returns the pointer to the block */
    fn allocate(size: u16) -> u16 {
        todo!()
    }

    /** 
     * Frees block of memory pointed to by 'pointer'
     */
    fn deallocate(pointer: u16) {
        todo!()
    }

}

struct Program{
    //code: Vec<Class>,
    code:Vec<VMCommand>,
    memory: Memory,
    current_line: usize,
    // call_stack: 
}

impl Program {
    /**
     * Initializes the program given a set of code and a configuration
     */
    fn initialize(code: Vec<VMCommand>) -> Program {
        let mut memory = Memory::initialize();
        // intialize segment pointers for the main stack frame
        memory.ram[0] = 256;   // SP
        memory.ram[1] = 300;   // LCL
        memory.ram[2] = 400;   // ARG
        memory.ram[3] = 3000;  // THIS
        memory.ram[4] = 4000;  // THAT
        let current_line = 0;

        Program { code, memory, current_line }
    }

    // step to the next line of code
    fn step() {
        todo!()
    }

    // return something for javascript to use?
    fn read_state() {
        todo!()
    }
}


// Global Stack
// There is a main frame where execution begins
// Before a function call, arguments are pushed to the current function's
// working stack.
// At a function/method call, the VM takes 5 words of memory to create a saved caller frame
// 1. return address
// 2. saved LCL
// 3. saved ARG
// 4. saved THIS
// 5. saved THAT
// Sets the argument segment to start at the first argument
// Sets the local segment to begin immediatelly after the saved frame
// Sets the working stack to begin immediatelly after that
// Transfers control to the callee





// classes
// static variables map to memory segment 'static'
// field variables map to memory segment 'this'
// pointer 0 is a pointer to the current objects 'this' segment
// pointer 1 is a pointer to 'that' segment

// So, each object has a pointer that either lives in static or local

// class instances
// they are mapped to memory 
// as long as the memory mapping is correct, the pointers will work out





// how to handle standard library calls - ultimately just functions that 



