// Enum representing the different kinds of tokens that can be produced by the lexer.
#[derive(Debug, PartialEq)] // Deriving Debug and PartialEq for better error messages and comparisons
pub enum TokenKind {
    VariableDeclaration, // Represents the `ANOTHER_ONE` keyword
    Output, // Represents the `SING` keyword
    Identifier, // Represents an identifer (variable names)
    StringLiteral, // Represents a string literal
    NumberLiteral, // Represents an integer literal
    FloatLiteral, // Represents a floating-point literal
    EOF, // Represents the end of the input
}

// A token produced by the lexer, consisting of a kind and the lexeme (text) it was generated from.
#[derive(Debug)] // Deriving Debug for better error messages
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

// The lexer struct, responsible for turning an input string into a series of tokens.
pub struct Lexer<'a> {
    input: &'a str, // The input string to be tokenized
    position: usize, // The current position within the input string
    current_char: Option<char>, // The current character being processed
}



impl<'a> Lexer<'a> {
    /// Creates a new lexer for the given input string.
    pub fn new(input: &'a str) -> Self {
        let lexer = Self {
            input,
            position: 0,
            current_char: input.chars().next(),
        };
        lexer
    }

    /// Advances the lexer to the next character in the input string.
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.chars().nth(self.position);
    }

    // Produces the next token from the input string
    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            return match c {
                // Skip whitespace characters
                ' ' | '\n' | '\t' | '\r' => {
                    self.advance();
                    continue;
                }
                // Handle the string literals
                '"' => self.string_literal(),
                // Handle identifiers and keywords
                c if c.is_alphabetic() => self.identifier(),
                // Handle numeric literals
                c if c.is_digit(10) => self.number_literal(),
                // Default case for unrecognized characters
                _ => {
                    self.advance();
                    Token {
                        kind: TokenKind::EOF,
                        lexeme: String::new(),
                    }
                }
            };
        }

        // Return an EOF token if there are no more characters to process
        Token {
            kind: TokenKind::EOF,
            lexeme: String::new(),
        }
    }

    /// Produces a string literal token.
    fn string_literal(&mut self) -> Token {
        let mut value = String::new();
        self.advance(); // Skip the opening quote
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance(); // Skip the closing quote
                break;
            }
            value.push(c);
            self.advance();
        }
        Token {
            kind: TokenKind::StringLiteral,
            lexeme: value,
        }
    }

    // Produces an identifier token or a keyword token.
    fn identifier(&mut self) -> Token {
        let mut value = String::new();
        while let Some(c) = self.current_char {
            if !c.is_alphabetic() && c != '_' {
                break;
            }
            value.push(c);
            self.advance();
        }

        let kind = match value.as_str() {
            "ANOTHER_ONE" => TokenKind::VariableDeclaration,
            "SING" => TokenKind::Output,
            _ => TokenKind::Identifier,
        };

        Token {
            kind,
            lexeme: value,
        }
    }

    /// Produces a number literal token, either integer or floating-point.
    fn number_literal(&mut self) -> Token {
        let mut value = String::new();
        let mut is_float = false;

        while let Some(c) = self.current_char {
            if c == '.' {
                is_float = true;
            } else if !c.is_digit(10) {
                break;
            }
            value.push(c);
            self.advance();
        }

        if is_float {
            Token {
                kind: TokenKind::FloatLiteral,
                lexeme: value,
            }
        } else {
            Token {
                kind: TokenKind::NumberLiteral,
                lexeme: value,
            }
        }
    }

}