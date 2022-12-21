use crate::memory::WordSize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::console;

#[derive(Debug, Clone)]
pub struct VMCommand {
    pub command: Command,
    pub line: usize,
}

impl VMCommand {
    fn new(command: Command, line: usize) -> VMCommand {
        VMCommand { command, line }
    }
}

pub type Offset = WordSize;
type NumVars = WordSize;
type NumArgs = WordSize;
type LabelName = String;
type Identifier = String;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Command {
    Pop(Segment, Offset),
    Push(Segment, Offset),
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
    Function(Identifier, NumVars),
    Call(Identifier, NumArgs),
    Class(Identifier),
    Return,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub functions: HashMap<String, Rc<RefCell<Function>>>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub start_line: usize,
    pub num_vars: WordSize,
    pub commands: Vec<VMCommand>,
    pub label_table: HashMap<String, usize>,
}

impl Function {
    pub fn add_command(&mut self, command: VMCommand) {
        self.commands.push(command);
    }
}

// Need help figuring out how to send errors back
pub(crate) fn parse_bytecode(text: &str) -> Bytecode {
    // We will receive multiple classes in a single text string,
    // so each time Class... is encountered the previous class
    // needs to be closed and a new class initialized

    // Initialize class and function to be empty
    let mut program = Bytecode {
        functions: HashMap::new(),
    };

    // Initialize class to none
    let mut current_class = "";

    // Initialized to bring into scope
    let mut current_function: Option<Rc<RefCell<Function>>> = None;

    for (line_num, line) in text
        .lines()
        .map(|line| {
            // remove comments: delete any text between '//' and end of line'
            if let Some(i) = line.find("//") {
                &line[..i]
            } else {
                line
            }
        })
        .enumerate()
    {
        if line.trim().is_empty() {
            continue;
        }

        let line_words: Vec<&str> = line.trim().split(' ').collect();
        // Check the number of arguments in each line
        let parsed_line = match line_words.len() {
            1 => {
                match line_words[0] {
                    // I bet this could be a macro...
                    "add" => VMCommand::new(Command::Add, line_num),
                    "sub" => VMCommand::new(Command::Sub, line_num),
                    "neg" => VMCommand::new(Command::Neg, line_num),
                    "eq" => VMCommand::new(Command::Eq, line_num),
                    "gt" => VMCommand::new(Command::Gt, line_num),
                    "lt" => VMCommand::new(Command::Lt, line_num),
                    "and" => VMCommand::new(Command::And, line_num),
                    "or" => VMCommand::new(Command::Or, line_num),
                    "not" => VMCommand::new(Command::Not, line_num),
                    "return" => VMCommand::new(Command::Return, line_num),
                    otherwise => {
                        console::log_1(&"Invalid zero argument command".into());
                        panic!(
                            "Invalid zero argument command at line {}: {}",
                            line_num, otherwise
                        );
                    }
                }
            }
            2 => match (line_words[0], line_words[1]) {
                ("class", identifier) => {
                    current_class = identifier;
                    VMCommand::new(Command::Class(identifier.to_string()), line_num)
                }
                ("goto", label) => VMCommand::new(Command::GoTo(label.to_string()), line_num),
                ("if-goto", label) => VMCommand::new(Command::IfGoTo(label.to_string()), line_num),
                ("label", label) => {
                    let function = &current_function.clone().unwrap();
                    let label_location: usize = line_num - function.borrow().start_line;
                    if let Some(prev_label_location) = function
                        .borrow_mut()
                        .label_table
                        .insert(label.to_string(), label_location)
                    {
                        console::log_1(&"Duplicate label {}".into());
                        panic!(
                            "Duplicate label {} encountered on lines {} and {}",
                            label, label_location, prev_label_location
                        );
                    }
                    VMCommand::new(Command::Label(label.to_string()), line_num)
                }
                (otherwise, _) => {
                    console::log_1(&"Invalid one argument command".into());
                    panic!(
                        "Invalid one argument command at line {}: {}",
                        line_num, otherwise
                    );
                }
            },
            3 => {
                match (
                    line_words[0],
                    line_words[1],
                    line_words[2]
                        .parse::<WordSize>()
                        .expect("Second argument should be parsable to an i32"),
                ) {
                    ("pop", segment, index) => {
                        VMCommand::new(Command::Pop(parse_segment(segment), index), line_num)
                    }
                    ("push", segment, index) => {
                        VMCommand::new(Command::Push(parse_segment(segment), index), line_num)
                    }
                    ("function", fn_name, var_count) => {
                        // Initialize a new function
                        let f = Rc::new(RefCell::new(Function {
                            start_line: line_num,
                            num_vars: var_count,
                            commands: Vec::new(),
                            label_table: HashMap::new(),
                        }));

                        let name = format!("{}.{}", current_class, fn_name);

                        program.functions.insert(name.to_string(), f);

                        current_function = program.functions.get(&name).cloned();
                        VMCommand::new(Command::Function(name.to_string(), var_count), line_num)
                    }
                    ("call", fn_name, num_args) => {
                        VMCommand::new(Command::Call(fn_name.to_string(), num_args), line_num)
                    }
                    (otherwise, _, _) => {
                        console::log_1(&"Invalid two argument command".into());
                        panic!(
                            "Invalid two argument command at line {}: {}",
                            line_num, otherwise
                        );
                    }
                }
            }
            otherwise => {
                console::log_2(&"Invalid syntax at line:".into(), &line_num.into());
                panic!(
                    "Invalid syntax at line {}. Expecting 0, 1, or two arguments, but was given {}",
                    line_num, otherwise
                );
            }
        };

        if let Some(f) = &current_function {
            f.borrow_mut().add_command(parsed_line);
        }
    }

    program
}
