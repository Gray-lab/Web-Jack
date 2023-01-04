use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;

use crate::jacklib::{self, NativeFunction};
use crate::memory::{Memory, WordSize};
use crate::parser::{parse_bytecode, Bytecode, Command, Function, Segment};

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

        // some library functions are implemented in jack
        // Keyboard.readChar
        // Keyboard.readLine
        // Keyboard.readInt
        let jack_library_functions = String::from("function Keyboard.readChar 2
            call Keyboard.keyPressed 0
            pop local 1
            push local 1
            pop local 0
            push local 1
            push constant 0
            eq
            not
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            label WHILE_EXP0
            push local 1
            push local 0
            eq
            not
            if-goto WHILE_END0
            call Keyboard.keyPressed 0
            pop local 0
            goto WHILE_EXP0
            label WHILE_END0
            label IF_FALSE0
            label WHILE_EXP1
            push local 0
            push constant 0
            eq
            not
            if-goto WHILE_END1
            call Keyboard.keyPressed 0
            pop local 0
            goto WHILE_EXP1
            label WHILE_END1
            label WHILE_EXP2
            call Keyboard.keyPressed 0
            push local 0
            eq
            not
            if-goto WHILE_END2
            goto WHILE_EXP2
            label WHILE_END2
            push local 0
            push constant 128
            lt
            if-goto IF_TRUE1
            goto IF_FALSE1
            label IF_TRUE1
            push local 0
            call Output.printChar 1
            pop temp 0
            label IF_FALSE1
            push local 0
            return
            function Keyboard.readLine 2
            push argument 0
            call Output.printString 1
            pop temp 0
            push constant 64
            call String.new 1
            pop local 0
            call Keyboard.readChar 0
            pop local 1
            label WHILE_EXP0
            push local 1
            call String.newLine 0
            eq
            if-goto WHILE_END0
            push local 1
            call String.backSpace 0
            eq
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            call Output.backSpace 0
            pop temp 0
            push local 0
            call String.eraseLastChar 1
            pop temp 0
            label IF_FALSE0
            push local 1
            push constant 128
            lt
            if-goto IF_TRUE1
            goto IF_FALSE1
            label IF_TRUE1
            push local 0
            push local 1
            call String.appendChar 2
            pop temp 0
            label IF_FALSE1
            call Keyboard.readChar 0
            pop local 1
            goto WHILE_EXP0
            label WHILE_END0
            call Output.println 0
            pop temp 0
            push local 0
            return
            function Keyboard.readInt 3
            push constant 0
            pop local 2
            push argument 0
            call Output.printString 1
            pop temp 0
            push constant 64
            call String.new 1
            pop local 0
            call Keyboard.readChar 0
            pop local 1
            label WHILE_EXP0
            push local 1
            push constant 45
            eq
            push local 2
            push constant 0
            eq
            and
            push local 1
            push constant 47
            gt
            push local 1
            push constant 58
            lt
            and
            or
            push local 1
            push constant 129
            eq
            or
            not
            if-goto WHILE_END0
            push local 1
            push constant 129
            eq
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            call Output.backSpace 0
            pop temp 0
            push local 0
            call String.eraseLastChar 1
            pop temp 0
            push local 2
            push constant 0
            gt
            if-goto IF_TRUE1
            goto IF_FALSE1
            label IF_TRUE1
            push local 2
            push constant 1
            sub
            pop local 2
            label IF_FALSE1
            goto IF_END0
            label IF_FALSE0
            push local 0
            push local 1
            call String.appendChar 2
            pop temp 0
            push local 2
            push constant 1
            add
            pop local 2
            label IF_END0
            call Keyboard.readChar 0
            pop local 1
            goto WHILE_EXP0
            label WHILE_END0
            call Output.println 0
            pop temp 0
            push local 0
            call String.intValue 1
            pop local 2
            push local 0
            call String.dispose 1
            pop temp 0
            push local 2
            return
        ");

        // their bytecode is appended to the input file
        let linked_input = format!("{}\n{}", input, jack_library_functions);
       
        let code = parse_bytecode(&linked_input);

        // let string = format!("{:?}", code);
        // console_log!("{}", string);

        // Populate with standard library fuctions
        let mut native_functions: HashMap<String, NativeFunction> = HashMap::new();
  
        // Math library
        native_functions.insert("Math.multiply".into(), jacklib::multiply);
        native_functions.insert("Math.divide".into(), jacklib::divide);
        native_functions.insert("Math.min".into(), jacklib::jack_min);
        native_functions.insert("Math.max".into(), jacklib::jack_max);
        native_functions.insert("Math.sqrt".into(), jacklib::jack_sqrt);
        native_functions.insert("Math.pow".into(), jacklib::jack_pow);
        native_functions.insert("Math.abs".into(), jacklib::jack_abs);
        native_functions.insert("Mod.mod".into(), jacklib::jack_mod);


        // String library
        native_functions.insert("String.new".into(), jacklib::string_new);
        native_functions.insert("String.dispose".into(), jacklib::string_dispose);
        native_functions.insert("String.length".into(), jacklib::string_length);
        native_functions.insert("String.charAt".into(), jacklib::char_at);
        native_functions.insert("String.setCharAt".into(), jacklib::set_char_at);
        native_functions.insert("String.appendChar".into(), jacklib::append_char);
        native_functions.insert("String.eraseLastChar".into(), jacklib::erase_last_char);
        native_functions.insert("String.intValue".into(), jacklib::int_value);
        native_functions.insert("String.setInt".into(), jacklib::set_int);
        native_functions.insert("String.backSpace".into(), jacklib::string_backspace);
        native_functions.insert("String.doubleQuote".into(), jacklib::double_quote);
        native_functions.insert("String.newLine".into(), jacklib::new_line);

        // Array library
        native_functions.insert("Array.new".into(), jacklib::array_new);
        native_functions.insert("Array.dispose".into(), jacklib::array_dispose);

        // Output library
        native_functions.insert("Output.moveCursor".into(), jacklib::move_cursor);
        native_functions.insert("Output.printChar".into(), jacklib::print_char);
        native_functions.insert("Output.printString".into(), jacklib::print_string);
        native_functions.insert("Output.printInt".into(), jacklib::print_int);
        native_functions.insert("Output.println".into(), jacklib::println);
        native_functions.insert("Output.backSpace".into(), jacklib::output_backspace);

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
        native_functions.insert("Memory.peek".into(), jacklib::jack_peek);
        native_functions.insert("Memory.poke".into(), jacklib::jack_poke);
        native_functions.insert("Memory.alloc".into(), jacklib::alloc);
        native_functions.insert("Memory.deAlloc".into(), jacklib::de_alloc);

        // System library
        native_functions.insert("Sys.wait".into(), jacklib::wait);
        native_functions.insert("Sys.halt".into(), jacklib::halt);
        native_functions.insert("Sys.error".into(), jacklib::error);

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
     * Execute next bytecode command.
     * Returns true if display was updated, otherwise returns false.
     */
    pub fn step(&mut self, key: WordSize) -> bool {
        let mut frame = match self.call_stack.last_mut() {
            Some(frame) => frame,
            None => return false,
        };

        // If there are no more instructions, return false and take no other action
        let length = frame.function.borrow().commands.len();
        if length <= frame.next_line {
            return false;
        }

        self.memory.display_updated = false;
        self.memory.keyboard = key;

        // The current command is cloned so that the stack frame can later be mutated
        // For example during a call or return command
        let current_command = &frame.function.borrow().commands[frame.next_line].clone();
        frame.next_line += 1;

        // This prints the current command to the web console for debugging
        // let command_string = format!("Executing {}:{:?}", frame.next_line - 1, current_command);
        // console_log!("{}", command_string);

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
            Command::Function(_, num_vars) => {
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
        self.memory.display_updated
    }

    pub fn ram_size(&self) -> usize {
        Memory::ram_size() as usize
    }

    /**
     * wrapper for Memory.display_size()
     * returns the length of the display memory array
     */
    pub fn display_size(&self) -> usize {
        Memory::display_size() as usize
    }

    /**
     * Sets the display to value at memory location display_word
     */
    pub fn set_display(&mut self, value: i32, offset: i32) {
        self.memory
            .set_display(value as WordSize, offset as WordSize);
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
