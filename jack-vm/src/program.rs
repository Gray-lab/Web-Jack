use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::memory::{Memory, WordSize};
use crate::parser::{parse_class, Command, Function, Segment, VMClass};

#[wasm_bindgen]
pub struct Program {
    class: VMClass,
    memory: Memory,
    call_stack: Vec<StackFrame>,
}

struct StackFrame {
    function: Rc<RefCell<Function>>,
    next_line: usize,
}

#[wasm_bindgen]
impl Program {
    /**
     * Initializes the program given a set of code and a configuration
     */
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str, lcl: i16) -> Program {
        // console::log_1(&"at start of program::new".into());
        // intialize segment pointers for the main stack frame
        let sp = 256;
        // let lcl = 300;
        let arg = 400;
        let this = 3000;
        let that = 4000;
        let memory = Memory::new(sp, lcl, arg, this, that);

        let class = parse_class(input);
        let string = format!("{:?}", class);
        console::log_1(&string.into());

        let main_function = class
            .functions
            .get("main")
            .cloned()
            .expect("need to have a main function");
        let main_frame = StackFrame {
            function: main_function,
            next_line: 0,
        };
        let mut call_stack = Vec::new();
        call_stack.push(main_frame);

        Program {
            class,
            memory,
            call_stack,
        }
    }

    pub fn step(&mut self) {
        // Handle the end of program by doing nothing when step() is called
        let mut frame = self.call_stack.last_mut().unwrap();

        let current_command = &frame.function.borrow().commands[frame.next_line];
        frame.next_line += 1;

        match &current_command.command {
            Command::Class(_) => panic!("Should not have a class command in the parsed code"),
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
            }
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
            }
            Command::Lt => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                if first == second {
                    self.memory.push(Segment::Constant, -1);
                } else {
                    self.memory.push(Segment::Constant, 0);
                }
            }
            Command::And => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                let and = first & second;
                self.memory.push(Segment::Constant, and);
            }
            Command::Or => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                let or = first | second;
                self.memory.push(Segment::Constant, or);
            }
            Command::Not => {
                let val = self.memory.pop(Segment::Temp, 0);
                let not = !val;
                self.memory.push(Segment::Constant, not);
            }
            Command::GoTo(label) => {
                frame.next_line = match frame.function.borrow().label_table.get(label) {
                    Some(line) => *line,
                    None => panic!(
                        "GoTo an unknown label encountered on line {}",
                        frame.next_line - 1
                    ),
                }
            }
            Command::IfGoTo(label) => {
                if self.memory.pop(Segment::Temp, 0) != 0 {
                    frame.next_line = match frame.function.borrow().label_table.get(label) {
                        Some(line) => *line,
                        None => panic!(
                            "GoTo an unknown label encountered on line {}",
                            frame.next_line - 1
                        ),
                    }
                }
            }
            Command::Label(_) => (),
            Command::Function(name, num_vars) => console::log_3(
                &"Hi from function".into(),
                &name.into(),
                &num_vars.to_string().into(),
            ),
            Command::Call(name, num_args) => {}
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
