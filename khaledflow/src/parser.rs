use crate::lexer::{Lexer, Token, TokenKind};

/// Enum representing the different kinds of statements that can be produced by the parser.
#[derive(Debug)]
pub enum Statement {
    VariableDeclaration { name: String, value: String }, // Represents a variable declaration statement
    Output { name: String },
}

/// The parser struct, responsible for turning a series of tokens into a series of statements.
pub struct Parser<'a> {
    lexer: Lexer<'a>, // The lexer providing tokens to the parser
    current_token: Token, // The current token being processed
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given lexer.
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    /// Parses the input tokens and produces a vector of statements.
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.current_token.kind != TokenKind::EOF {
            let statement = self.parse_statement();
            statements.push(statement);
        }
        statements
    }

    /// Parses a single statement from the current token.
    fn parse_statement(&mut self) -> Statement {
        match self.current_token.kind {
            TokenKind::VariableDeclaration => {
                self.advance();
                let name = self.expect_identifier();
                let value = self.expect_literal();
                Statement::VariableDeclaration { name, value }
            }
            TokenKind::Output => {
                self.advance();
                let name = self.expect_identifier();
                Statement::Output { name }
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    // Expects the current token to be an identifier and return its lexeme.
    fn expect_identifier(&mut self) -> String {
        if let TokenKind::Identifier = self.current_token.kind {
            let name = self.current_token.lexeme.clone();
            self.advance();
            name
        } else {
            panic!("Expected identifier, found {:?}", self.current_token);
        }
    }

    /// Expects the current token to be a literal and returns its lexeme.
    fn expect_literal(&mut self) -> String {
        match self.current_token.kind {
            TokenKind::StringLiteral | TokenKind::NumberLiteral | TokenKind::FloatLiteral => {
                let value = self.current_token.lexeme.clone();
                self.advance();
                value
            }
            _ => panic!("Expected literal, found {:?}", self.current_token),
        }
    }

    /// Advances the parser to the next token from the lexer.
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}