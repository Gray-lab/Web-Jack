use parser::Token;
use symbol_table::SymbolTable;
use parser::Name;

mod symbol_table;
mod parser;
mod config;

// enum SubRoutineKind {
//     Method, Function
// }

// enum SubRoutineType {
//     Void, Int, Char, Boolean, Name
// }

// struct LabelCounter {
//     if_labels: usize,
//     while_labels: usize,
// }

// struct CompilationEngine {
//     class_table: SymbolTable,
//     subroutine_table: SymbolTable,
//     current_class: Name,
//     field_var_count: isize,
//     current_subroutine: Name,
//     current_subroutine_kind: SubRoutineKind,
//     current_subroutine_type: SubRoutineType,
//     symbol_name:
//     symbol_type:
//     symbol_kind:
//     symbol_category:
//     label_counter: LabelCounter,


// }

// impl CompilationEngine {

//     pub fn compile(tokens: &Vec<Token>) -> String {
//         let token_iterator = tokens.iter().peekable();
//         todo!()
//     }

// }

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;
    use crate::parser::{parse, TokenType, Name};
    use crate::config::Config;

    use super::*;

    #[test]
    fn parse_comments() {
        let text = String::from(
        "// line comment
        line_comment ///comment * / */ /* */ //still a comment
        before_block_comment /* all the
        text in the block comment
        should be part of a comment
        and be ignored by the parser */ end_block
        below_block
        /**
         * testing
         */
        end_block");

        let result = parse(&text);

        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::new(TokenType::Identifier(Name("line_comment".to_string())), 2));
        expected.push(Token::new(TokenType::Identifier(Name("before_block_comment".to_string())), 3));
        expected.push(Token::new(TokenType::Identifier(Name("end_block".to_string())), 6));
        expected.push(Token::new(TokenType::Identifier(Name("below_block".to_string())), 7));
        expected.push(Token::new(TokenType::Identifier(Name("end_block".to_string())), 11));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_int_literals() {
        let text = String::from("0123456789
                                         test42
                                         123test");

        let result = parse(&text);

        // The expected is not syntactically correct, 
        // but that will be handled in the compiler
        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::new(TokenType::IntLiteral("0123456789".to_string()), 1));
        expected.push(Token::new(TokenType::Identifier(Name("test42".to_string())), 2));
        expected.push(Token::new(TokenType::IntLiteral("123".to_string()), 3));
        expected.push(Token::new(TokenType::Identifier(Name("test".to_string())), 3));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_string_literals() {
        let text = String::from(r#"let string = "a string literal";"#);

        let result = parse(&text);

        // The expected is not syntactically correct, 
        // but that will be handled in the compiler
        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::new(TokenType::Let, 1));
        expected.push(Token::new(TokenType::Identifier(Name("string".to_string())), 1));
        expected.push(Token::new(TokenType::Equals, 1));
        expected.push(Token::new(TokenType::StringLiteral("a string literal".to_string()), 1));
        expected.push(Token::new(TokenType::Semicolon, 1));

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_symbols() {
        let text = String::from("{} () [] . , ; + - * & | < > = ~ /");

        let result = parse(&text);

        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::new(TokenType::LeftBrace, 1));
        expected.push(Token::new(TokenType::RightBrace, 1));
        expected.push(Token::new(TokenType::LeftParen, 1));
        expected.push(Token::new(TokenType::RightParen, 1));
        expected.push(Token::new(TokenType::LeftBracket, 1));
        expected.push(Token::new(TokenType::RightBracket, 1));
        expected.push(Token::new(TokenType::Dot, 1));
        expected.push(Token::new(TokenType::Comma, 1));
        expected.push(Token::new(TokenType::Semicolon, 1));
        expected.push(Token::new(TokenType::Plus, 1));
        expected.push(Token::new(TokenType::Minus, 1));
        expected.push(Token::new(TokenType::Star, 1));
        expected.push(Token::new(TokenType::Ampersand, 1));
        expected.push(Token::new(TokenType::Bar, 1));
        expected.push(Token::new(TokenType::LessThan, 1));
        expected.push(Token::new(TokenType::GreaterThan, 1));
        expected.push(Token::new(TokenType::Equals, 1));
        expected.push(Token::new(TokenType::Tilde, 1));
        expected.push(Token::new(TokenType::Slash, 1));

        assert_eq!(result, expected);

    }

    #[test]
    fn parse_identifiers_and_keywords() {
        let text = String::from("class constructor function method field
                                         static var int char boolean void true
                                         false null this let do if else while return
                                         abcdefghijklmnopqrstuvwxyz_ABCDEPGHIJKLMOPQRSTUVWXYZ0123456789");

        let result = parse(&text);

        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::new(TokenType::Class, 1));
        expected.push(Token::new(TokenType::Constructor, 1));
        expected.push(Token::new(TokenType::Function, 1));
        expected.push(Token::new(TokenType::Method, 1));
        expected.push(Token::new(TokenType::Field, 1));
        expected.push(Token::new(TokenType::Static, 2));
        expected.push(Token::new(TokenType::Var, 2));
        expected.push(Token::new(TokenType::Int, 2));
        expected.push(Token::new(TokenType::Char, 2));
        expected.push(Token::new(TokenType::Boolean, 2));
        expected.push(Token::new(TokenType::Void, 2));
        expected.push(Token::new(TokenType::True, 2));
        expected.push(Token::new(TokenType::False, 3));
        expected.push(Token::new(TokenType::Null, 3));
        expected.push(Token::new(TokenType::This, 3));
        expected.push(Token::new(TokenType::Let, 3));
        expected.push(Token::new(TokenType::Do, 3));
        expected.push(Token::new(TokenType::If, 3));
        expected.push(Token::new(TokenType::Else, 3));
        expected.push(Token::new(TokenType::While, 3));
        expected.push(Token::new(TokenType::Return, 3));
        expected.push(Token::new(TokenType::Identifier(Name("abcdefghijklmnopqrstuvwxyz_ABCDEPGHIJKLMOPQRSTUVWXYZ0123456789".to_string())), 4));

        assert!(result == expected);

    }

    #[test]
    fn read_file_jack() {
        // requires directory 'test_dir' to contain file 'test1.jack'
        let mut args = Vec::new();
        args.push(String::from("arg1"));
        args.push(String::from("./test_dir/test1.jack"));

        let result_config = Config::build(&args).unwrap();

        let filepath = PathBuf::from_str(&args[1]).unwrap();

        assert!(result_config.file_paths.contains(&filepath));
    }

    #[test]
    fn read_directory_with_jack_files() {
        // requires directory 'test_dir' to contain files 'test1.jack' & 'test2.jack'
        let mut args = Vec::new();
        args.push(String::from("arg1"));
        args.push(String::from("./test_dir"));

        let result_config = Config::build(&args).unwrap();

        // both these .jack file paths should be saved in the config
        let filepath1 = PathBuf::from_str("./test_dir/test1.jack").unwrap();
        let filepath2 = PathBuf::from_str("./test_dir/test2.jack").unwrap();
        
        // this .txt file path should not be saved
        let badfilepath = PathBuf::from_str("./test_dir/test3.txt").unwrap();

        assert!(result_config.file_paths.contains(&filepath1));
        assert!(result_config.file_paths.contains(&filepath2));
        assert!(!result_config.file_paths.contains(&badfilepath));
    }
}
