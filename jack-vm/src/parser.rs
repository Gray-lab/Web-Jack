use web_sys::console;

#[derive(Debug)]
pub struct VMCommand {
    pub command: Command,
    pub line: usize,
}

impl VMCommand {
    fn new(command:Command, line:usize) -> VMCommand {
        VMCommand { command, line }
    }
}

pub type Index = usize;
type NumVars = usize;
type NumArgs = usize;
type LabelName = String;
type FunctionName = String;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
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
//     label_table: HashMap<String, usize>
// }

// Need help figuring out how to send errors back
pub(crate) fn parse_vm_code(text: &str) -> Vec<VMCommand>{

    let mut tokens = Vec::new();

    for (i, line) in text.lines().map(|line| {
        // remove comments: delete any text between '//' and end of line'
        if let Some(i) = line.find("//") {
            &line[..i]
        } else {
            line
        }
    }).enumerate() {
        // console.log the line
        console::log_1(&line.into());

        if line.trim().is_empty() {
            continue;
        }
        
        let line_words: Vec<&str> = line.trim().split(" ").collect();
        // Check the number of arguments in each line
        // The nested match statements could be factored out into VMCommand implementations.
        // Does Rust support polymorphic functions?
        let parsed_line = match line_words.len() {
            1 => {
                match line_words[0] {
                    // I bet this could be a macro...
                    "add" => VMCommand::new(Command::Add, i),
                    "sub" => VMCommand::new(Command::Sub, i),
                    "neg" => VMCommand::new(Command::Neg, i),
                    "eq" => VMCommand::new(Command::Eq, i),
                    "gt" => VMCommand::new(Command::Gt, i),
                    "lt" => VMCommand::new(Command::Lt, i),
                    "and" => VMCommand::new(Command::And, i),
                    "or" => VMCommand::new(Command::Or, i),
                    "not" => VMCommand::new(Command::Not, i),
                    "return" => VMCommand::new(Command::Return, i),
                    otherwise => {
                        console::log_1(&"Invalid zero argument command".into());
                        panic!("Invalid zero argument command at line {}: {}", i, otherwise);
                    }
                }
            }
            2 => {
                match (line_words[0], line_words[1]) {
                    ("goto", label) => VMCommand::new(Command::GoTo(label.to_string()), i),
                    ("ifgoto", label) => VMCommand::new(Command::IfGoTo(label.to_string()),i),
                    ("label", label) => VMCommand::new(Command::Label(label.to_string()),i),
                    (otherwise, _) => {
                        console::log_1(&"Invalid one argument command".into());
                        panic!("Invalid one argument command at line {}: {}", i, otherwise);
                    }
                }
            }
            3 => {
                match (line_words[0], line_words[1], line_words[2].parse::<usize>().expect("Second argument should be parsable to an i32")) {
                    ("pop", segment, index) => VMCommand::new(Command::Pop(parse_segment(segment), index), i),
                    ("push", segment, index) => VMCommand::new(Command::Push(parse_segment(segment), index), i),
                    ("function", fn_name, num_vars) => VMCommand::new(Command::Function(fn_name.to_string(), num_vars), i),
                    ("call", fn_name, num_args) => VMCommand::new(Command::Call(fn_name.to_string(), num_args), i),
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
        };
        tokens.push(parsed_line);
    }

    tokens
    
}