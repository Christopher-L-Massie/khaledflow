// Importing necessary modules from the standard library and our parser module
use std::collections::HashMap;
use crate::parser::Statement;

// The interpreter struct, responsible for executing a series of statements.
pub struct Interpreter {
    // A hashmap to store variables and their values
    variables: HashMap<String, String>
}

impl Interpreter {
    /// Creates a new interpreter.
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    // Interprets a vector of statements, executing each one in order.
    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement {
                // Handle variable declaration statements
                Statement::VariableDeclaration { name, value } => {
                    self.variables.insert(name, value);
                }
                // Handle output statements
                Statement::Output { name } => {
                    if let Some(value) = self.variables.get(&name) {
                        println!("{}", value);
                    } else {
                        eprintln!("Error: Undefined variable {}", name);
                    }
                }
            }
        }
    }
}