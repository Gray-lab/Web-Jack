use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::jacklib::{self, NativeFunction};
use crate::memory::{Memory, WordSize};
use crate::parser::{parse_class, Bytecode, Command, Function, Segment};

struct StackFrame {
    function: Rc<RefCell<Function>>,
    next_line: usize,
}

impl StackFrame {
    fn new(function: Rc<RefCell<Function>>) -> StackFrame {
        StackFrame {
            function,
            next_line: 0,
        }
    }
}

#[wasm_bindgen]
pub struct Program {
    code: Bytecode,
    native_functions: HashMap<String, NativeFunction>,
    memory: Memory,
    call_stack: Vec<StackFrame>,
}

#[wasm_bindgen]
impl Program {
    /**
     * Initializes the program given a set of code and a configuration
     */
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Program {
        // set panic hook
        console_error_panic_hook::set_once();

        // intialize segment pointers for the main stack frame
        let sp = 256;
        let lcl = 300;
        let arg = 400;
        let this = 3000;
        let that = 4000;
        let memory = Memory::new(sp, lcl, arg, this, that);

        let code = parse_class(input);
        let string = format!("{:?}", code);
        console::log_1(&string.into());

        let mut native_functions: HashMap<String, NativeFunction> = HashMap::new();
        // Populate with standard library fuctions
        // Math library
        native_functions.insert("Math.multiply".into(), jacklib::multiply);
        native_functions.insert("Math.divide".into(), jacklib::divide);
        native_functions.insert("Math.min".into(), jacklib::jack_min);
        native_functions.insert("Math.max".into(), jacklib::jack_max);
        native_functions.insert("Math.sqrt".into(), jacklib::jack_sqrt);
        native_functions.insert("Math.pow".into(), jacklib::jack_pow);

        // String library
        native_functions.insert("String.new".into(), jacklib::string_new);
        native_functions.insert("String.new".into(), jacklib::string_new);

        // Array library

        // Output library
        native_functions.insert("Output.moveCursor".into(), jacklib::move_cursor);
        native_functions.insert("Output.printChar".into(), jacklib::print_char);
        native_functions.insert("Output.printString".into(), jacklib::print_string);
        native_functions.insert("Output.printInt".into(), jacklib::print_int);
        native_functions.insert("Output.println".into(), jacklib::println);
        native_functions.insert("Output.backSpace".into(), jacklib::move_cursor);

        // Screen library
        native_functions.insert("Screen.setColor".into(), jacklib::set_color);
        native_functions.insert("Screen.drawPixel".into(), jacklib::draw_pixel);
        native_functions.insert("Screen.clearScreen".into(), jacklib::clear_screen);
        native_functions.insert("Screen.fillScreen".into(), jacklib::fill_screen);
        native_functions.insert("Screen.drawLine".into(), jacklib::draw_line);
        native_functions.insert(
            "Screen.drawRectangleOutline".into(),
            jacklib::draw_rectangle_outline,
        );
        native_functions.insert("Screen.drawRectangle".into(), jacklib::draw_rectangle);
        native_functions.insert("Screen.drawCircle".into(), jacklib::draw_circle);

        // Keyboard library
        native_functions.insert("Keyboard.keyPressed".into(), jacklib::key_pressed);
        native_functions.insert("Keyboard.readChar".into(), jacklib::read_char);
        native_functions.insert("Keyboard.readLine".into(), jacklib::read_line);
        native_functions.insert("Keyboard.readInt".into(), jacklib::read_int);

        // Memory library
        native_functions.insert("Memory.peek".into(), jacklib::peek);
        native_functions.insert("Memory.poke".into(), jacklib::poke);
        native_functions.insert("Memory.alloc".into(), jacklib::alloc);
        native_functions.insert("Memory.deAlloc".into(), jacklib::de_alloc);

        // System library
        native_functions.insert("System.wait".into(), jacklib::wait);
        native_functions.insert("System.halt".into(), jacklib::halt);
        native_functions.insert("System.error".into(), jacklib::error);

        let main_function = code
            .functions
            .get("Main.main")
            .cloned()
            .expect("need to have a main function");

        let main_frame = StackFrame::new(main_function);

        let mut call_stack = Vec::new();
        call_stack.push(main_frame);

        Program {
            code,
            native_functions,
            memory,
            call_stack,
        }
    }

    /**
     * Steps though code in Program,
     * Returns true at a successful step and false if no more commands are available
     */
    pub fn step(&mut self, key: WordSize) -> bool {
        let mut frame = self.call_stack.last_mut().unwrap();

        // If there are no more instructions, return false and take no other action
        let length = frame.function.borrow().commands.len();
        if length <= frame.next_line {
            // console::log_1(&"Ding! Program is finished.".into());
            return false;
        }
        // The current command is cloned so that the stack frame can later be mutated
        // For example during a call or return command
        let current_command = &frame.function.borrow().commands[frame.next_line].clone();
        frame.next_line += 1;

        let command_string = format!("Executing {}:{:?}", frame.next_line - 1, current_command);
        console::log_1(&command_string.into());

        match &current_command.command {
            Command::Class(_identifier) => {
                ();
            }
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
                if first < second {
                    self.memory.push(Segment::Constant, -1);
                } else {
                    self.memory.push(Segment::Constant, 0);
                }
            }
            Command::Lt => {
                let first = self.memory.pop(Segment::Temp, 0);
                let second = self.memory.pop(Segment::Temp, 0);
                if first > second {
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
            Command::Function(name, num_vars) => {
                console::log_3(
                    &"In function".into(),
                    &name.into(),
                    &num_vars.to_string().into(),
                );
                // Push local variables
                for _i in 0..*num_vars {
                    self.memory.push(Segment::Constant, 0);
                }
            }
            Command::Call(name, num_args) => {
                // self.code holds all jack code, including user and library functions written in Jack
                if self.code.functions.contains_key(name) {
                    // Find the correct function
                    let callee = self.code.functions.get(name).cloned().unwrap();
                    // Build a stack frame for it in memory
                    let global_line_num = callee.borrow().start_line + frame.next_line - 1;
                    self.memory
                        .push_stack_frame(*num_args, global_line_num as WordSize);
                    // Build and push a stack frame for the virtual call stack
                    self.call_stack.push(StackFrame::new(callee));
                } else if self.native_functions.contains_key(name) {
                    // All other functions are native rust
                    let callee = self.native_functions.get(name).unwrap();
                    self.memory.push_stack_frame(*num_args, 0 as WordSize);
                    let return_value = callee(&mut self.memory, *num_args);
                    // Jack expects a return value for every function
                    self.memory.push(Segment::Constant, return_value);
                    self.memory.pop_stack_frame();
                } else {
                    panic!("Function {} not found", name);
                }
            }
            Command::Return => {
                self.memory.pop_stack_frame();
                self.call_stack.pop();
            }
        }
        true
    }

    pub fn ram_size(&self) -> usize {
        Memory::ram_size() as usize
    }

    /**
     * Sets the display to value at memory location display_word
     */
    pub fn set_display(&mut self, value: i32, offset: i32) {
        // let string = format!(
        //     "Setting display segment {} to {}",
        //     offset as WordSize, value as WordSize
        // );
        // console::log_1(&string.into());
        self.memory
            .set_display(value as WordSize, offset as WordSize);
    }

    /**
     * wrapper for Memory.display_size()
     * returns the length of the display memory array
     */
    pub fn display_size(&self) -> usize {
        Memory::display_size() as usize
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
