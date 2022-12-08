use wasm_bindgen::prelude::*;
use web_sys::console;
use std::collections::HashMap;

use crate::memory::{Memory, WordSize};
use crate::parser::{parse_vm_code, Command, Segment, VMCommand};

#[wasm_bindgen]
pub struct Program {
    //code: Vec<Class>,
    code: Vec<VMCommand>,
    memory: Memory,
    next_line: usize,
    label_table: HashMap<String, usize>,
    // call_stack:
}

#[wasm_bindgen]
impl Program {
    /**
     * Initializes the program given a set of code and a configuration
     */
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str, lcl:i16) -> Program {
        // intialize segment pointers for the main stack frame
        let sp = 256;
        // let lcl = 300;
        let arg = 400;
        let this = 3000;
        let that = 4000;
        let memory = Memory::new(sp, lcl, arg, this, that);
        let next_line = 0;

        let code = parse_vm_code(input);
        for command in &code {
            let string = format!("{:?}", command);
            console::log_1(&string.into())
        
        }
        let label_table: HashMap<String, usize> = HashMap::new();

        Program {
            code,
            memory,
            next_line,
            label_table
        }
    }

    pub fn step(&mut self) {
        // Handle the end of program by doing nothing when step() is called
        console::log_1(&self.next_line.into());
        if &self.code.len() - 1 < self.next_line {
            return;
        }

        let current_command = &self.code[self.next_line];
        self.next_line += 1;

        match &current_command.command {
            Command::Pop(seg, idx) => {
                self.memory.pop(*seg, *idx);
            }
            Command::Push(seg, idx) => {
                self.memory.push(*seg, *idx);
            }
            Command::Add => {
                let sum = self.memory.pop(Segment::Temp, 0) + self.memory.pop(Segment::Temp, 0);
                self.memory.push(Segment::Constant, sum);
            }
            Command::Sub => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                let sum = second - first;
                self.memory.push(Segment::Constant, sum);
            }
            Command::Neg => {
                let val = self.memory.pop(Segment::Temp, 0);
                let neg = -val;
                self.memory.push(Segment::Constant, neg);
            },
            Command::Eq => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                if first == second {
                    self.memory.push(Segment::Constant, -1);
                } else {
                    self.memory.push(Segment::Constant, 0);
                }
            }
            Command::Gt => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                if first == second {
                    self.memory.push(Segment::Constant, -1);
                } else {
                    self.memory.push(Segment::Constant, 0);
                }
            },
            Command::Lt => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                if first == second {
                    self.memory.push(Segment::Constant, -1);
                } else {
                    self.memory.push(Segment::Constant, 0);
                }
            },
            Command::And => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                let and = first & second;
                self.memory.push(Segment::Constant, and);
            },
            Command::Or => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                let or = first | second;
                self.memory.push(Segment::Constant, or);
            },
            Command::Not => {
                let val = self.memory.pop(Segment::Temp, 0);
                let not = !val;
                self.memory.push(Segment::Constant, not);
            },
            Command::GoTo(label) => {
                self.next_line = match self.label_table.get(label) {
                    Some(line) => *line,
                    None => panic!("GoTo an unknown label encountered on line {}", self.next_line - 1),
                }
            },
            Command::IfGoTo(label) => {
                if self.memory.pop(Segment::Temp, 0) != 0 {
                    self.next_line = match self.label_table.get(label) {
                        Some(line) => *line,
                        None => panic!("GoTo an unknown label encountered on line {}", self.next_line - 1),
                    }
                }
            },
            Command::Label(label) => {
                match self.label_table.insert(label.clone(), self.next_line) {
                    Some(prev_label_location) => {
                        console::log_1(&"Duplicate label {}".into());
                        panic!("Duplicate label {} encountered on lines {} and {}", label, self.next_line - 1, prev_label_location - 1);
                    }
                    None => ()
                }
            },
            Command::Function(name, num_vars) => todo!(),
            Command::Call(name, num_args) => todo!(),
            Command::Return => todo!(),
        }
    }

    pub fn ram_size(&self) -> usize {
        self.memory.ram_size() as usize
    }

    // /**
    //  * wrapper for Memory.test_display()
    //  * fills the bits of the display memory
    //  */
    // pub fn test_display(&mut self) {
    //     self.working_memory.test_display();
    // }

    /**
     * wrapper for Memory.display_size()
     * returns the length of the display memory array
     */
    pub fn display_size(&self) -> usize {
        self.memory.display_size() as usize
    }

    /**
     * wrapper for Memory.ram()
     * returns a pointer to the start of the ram memory segment
     */
    pub fn ram(&self) -> *const WordSize {
        self.memory.ram()
    }

    /**
     * wrapper for Memory.display()
     * returns a pointer to the start of the display memory segment
     */
    pub fn display(&self) -> *const WordSize {
        self.memory.display()
    }

    /**
     * wrapper for Memory.keyboard()
     * returns the contents of the keyboard memory segment
     */
    pub fn keyboard(&self) -> WordSize {
        self.memory.keyboard()
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
