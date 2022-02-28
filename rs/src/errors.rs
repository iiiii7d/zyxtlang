use std::process::exit;
use ansi_term::Color::{Black, Red, Yellow};
use ansi_term::Style;
use crate::interpreter::Variable;
use crate::lexer::Position;

fn error_main(code: &str, message: String) -> ! {
    println!("{}", Black.on(Yellow).paint(format!(" Error {} ", code)).to_string()
    + &*Red.bold().paint(format!(" {}", message)).to_string());
    exit(0)
}

pub fn error_pos(pos: &Position) {
    print!("{}", Style::new().on(Red).bold().paint(format!(" {} ", pos)).to_string())
}

/* 0. Internal errors, have to do with the compiler-interpreter itself */
/// Rust error
pub fn error_0_0(stack: String) -> ! {
    error_main("0.0", format!("Internal error: \n{}", stack))
}

/// No file given
pub fn error_0_1() -> ! {
    error_main("0.1", format!("No file given"))
}

/* 1. File and I/O errors */
/// File does not exist
pub fn error_1_0(filename: String) {
    error_main("1.0", format!("File `{}` does not exist", filename))
}

/// file cannot be opened
pub fn error_1_1(filename: String) -> ! {
    error_main("1.1", format!("File `{}` cannot be opened", filename))
}

/* 2. Syntax errors */
/// parentheses not closed properly (try swapping)
pub fn error_2_0_0(paren1: char, paren2: char) -> ! {
    error_main("2.0.0", format!("Parentheses `{}` and `{}` not closed properly; try swapping them", paren1, paren2))
}
/// parentheses not closed properly (not closed)
pub fn error_2_0_1(paren: char) -> ! {
    error_main("2.0.1", format!("Parenthesis `{}` not closed", paren))
}
/// parentheses not closed properly (not opened)
pub fn error_2_0_2(paren: char) -> ! {
    error_main("2.0.2", format!("Parenthesis `{}` not opened", paren))
}

/// unexpected ident (generic)
pub fn error_2_1_0(ident: String) -> ! {
    error_main("2.1.0", format!("Unexpected ident `{}`", ident))
}
/// unexpected ident (lexer didnt recognise)
pub fn error_2_1_1(ident: String) -> ! {
    error_main("2.1.1", format!("Ident `{}` not recognised by lexer", ident))
}

/// assignment without variable name
pub fn error_2_2() -> ! {
    error_main("2.2", format!("Assignment without variable name"))
}

/// unfilled argument
pub fn error_2_3(func: String, index: usize) -> ! {
    error_main("2.3", format!("Unfilled argument #{} of {}", index, func))
}

/* 3. Variable & attribute errors */
/// Variable not defined
pub fn error_3_0(varname: String) -> ! {
    error_main("3.0", format!("Undefined variable `{}`", varname))
}

/* 4. Type errors */
/// Binary operator not implemented for type
pub fn error_4_0_0(operator: String, type1: String, type2: String) -> ! {
    error_main("4.0.0", format!("Operator {} not implemented for types `{}`, `{}`", operator, type1, type2))
}
/// Unary operator not implemented for type
pub fn error_4_0_1(operator: String, type_: String) -> ! {
    error_main("4.0.0", format!("Operator {} not implemented for type `{}`", operator, type_))
}

/// Binary operation unsuccessful
pub fn error_4_1_0(operator: String, value1: Variable, value2: Variable) -> ! {
    error_main("4.1", format!("Operator {} unsuccessful on `{}` (type `{}`), `{}` (type `{}`)",
                              operator, value1, value1.get_type_name(), value2, value2.get_type_name()))
}
/// Unary operation unsuccessful
pub fn error_4_1_1(operator: String, value: Variable) -> ! {
    error_main("4.1", format!("Operator {} unsuccessful on `{}` (type `{}`)",
                              operator, value, value.get_type_name()))
}