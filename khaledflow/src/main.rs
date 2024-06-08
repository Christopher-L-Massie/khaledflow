// Importing the lexer, parser, and interpreter modules
mod lexer;
mod parser;
mod interpreter;

// Importing necessary items from the standard library and our modules
use std::fs; // For reading files
use lexer::Lexer; // For tokenizing input
use parser::Parser; // For parsing tokens into statements
use interpreter::Interpreter; // For interpreting and executing statements

/// The main function, entry point of the program
fn main() {
    let input = fs::read_to_string("test_scripts/output.dj")
        .expect("Failed to read script file");

    // Create a new lexer with the input string
    let lexer = Lexer::new(&input);

    // Create a new parser with the lexer
    let mut parser = Parser::new(lexer);

    // Parse the tokens into a series of statements
    let statements = parser.parse();

    // Create a new interpreter
    let mut interpreter = Interpreter::new();

    // Interpret and execute the statements
    interpreter.interpret(statements);
}