//! Tokenizer.

use std::io::buffered::BufferedReader;
use std::str::from_char;
use std::ascii::StrAsciiExt;

/// A token.
#[deriving(Clone, ToStr, Eq)]
pub enum Token {
    Number(f64),
    Id(~str),
    
    // Brackets
    OpenBracket,
    CloseBracket,
    
    // Operators.
    Plus,
    Minus,
    Mul,
    Div,
    Power,
    
    // Built-in functions.
    Exp,
    Ln,
    Sin,
    Cos,
    Tg,
    Ctg
}

/// The tokenizer.
pub struct Tokenizer<R> {
    priv reader: BufferedReader<R>,
    priv token: Option<Token>,
    priv ch: Option<char>,
    priv failed: bool
}

/// What should we do with an invalid token.
pub enum InvalidTokenFix {
    Ignore,
    UseToken(Token),
    Fail
}

/// Raised when an invalid token is encountered.
condition! {
    pub invalid_token : ~str -> InvalidTokenFix;
}

impl<R: Reader> Tokenizer<R> {
    /// Creates a new tokenizer using the given reader.
    pub fn new(reader: R) -> Tokenizer<R> {
        Tokenizer::<R> { 
            reader: BufferedReader::<R>::new(reader), 
            token: None,
            ch: None,
            failed: false,
        }
    }

    /// Return whether the Tokenizer has reached the end of the stream or failed.
    pub fn eof(&mut self) -> bool {
        self.failed || self.peek().is_none()
    }

    /// Returns and consumes the next token.
    pub fn take(&mut self) -> Option<Token> {
        if self.token.is_none() {
            self.token = self.read_token()
        }
        
        self.token.take()
    }
    
    /// Returns the next token without consuming it.
    pub fn peek(&mut self) -> Option<Token> {
        if self.token.is_none() {
            self.token = self.read_token()
        }
        
        self.token.clone()
    }
    
    /// Reads the next token.
    fn read_token(&mut self) -> Option<Token> {
        // Don't even try if we failed before.
        if self.failed {
            return None
        }
    
        // Ignore whitespace.
        while self.peek_char().map_or(false, |ch| ch.is_whitespace()) {
            self.take_char();
        }
        
        match self.take_char() {
            // Single character tokens.
            Some('(') => Some(OpenBracket),
            Some(')') => Some(CloseBracket),
            Some('+') => Some(Plus),
            Some('-') => {
                // Peek the next non-whitespace character.
                while self.peek_char().map_or(false, |ch| ch.is_whitespace()) {
                    self.take_char();
                }
                if self.peek_char().map_or(false, |ch| ch.is_digit()) {
                    self.read_number('-')
                } else {
                    Some(Minus)
                }
            },
            Some('*') => Some(Mul),
            Some('/') => Some(Div),
            Some('^') => Some(Power),
            
            // Multi character tokens.
            Some(ch)  => {
                // Is this a number?
                if ch.is_digit() {
                    self.read_number(ch)
                }
                // Is this an identifier?
                else if ch.is_alphabetic() || ch == '_' {
                    self.read_id(ch)
                }
                // Otherwise this is not a valid token.
                else {
                    self.handle_invalid_token(from_char(ch))
                }
            },
            
            // EOF or an error.
            None      => None
        }
    }
    
    /// Reads an number.
    fn read_number(&mut self, ch: char) -> Option<Token> {
        // Read all digits, dot characters or 'e' characters.
        let mut s = from_char(ch);
        while self.peek_char().map_or(false, |c| c.is_digit() || c == '.' || c == 'e') {
            s.push_char(self.take_char().unwrap());
        }
        
        // Try to parse an float.
        match from_str::<f64>(s) {
            Some(f) => Some(Number(f)),
            None    => self.handle_invalid_token(s)
        }
    }
    
    /// Reads an identifier.
    fn read_id(&mut self, ch: char) -> Option<Token> {
        // Read all alphanumerics and '_' characters.
        let mut s = from_char(ch);
        while self.peek_char().map_or(false, |c| c.is_alphanumeric() || c == '_') {
            s.push_char(self.take_char().unwrap());
        }
    
        match s.to_ascii_lower() {
            // Match build-in functions.
            ~"exp" => Some(Exp),
            ~"ln"  => Some(Ln),
            ~"sin" => Some(Sin),
            ~"cos" => Some(Cos),
            ~"tg"  => Some(Tg),
            ~"ctg" => Some(Ctg),
            
            // Otherwise it must be an identifier.
            _      => Some(Id(s))
        }
    }
    
    /// Handles an invalid token.
    fn handle_invalid_token(&mut self, s: ~str) -> Option<Token> {
        match invalid_token::cond.raise(s) {
            // Ignore the failed and try to read another token.
            Ignore => self.read_token(),
            
            // Use an user provided token.
            UseToken(token) => Some(token),
            
            // Or just fail.
            Fail => {
                self.failed = true;
                None
            }
        }
    }
    
    /// Returns and consumes the next char.
    fn take_char(&mut self) -> Option<char> {
        if self.ch.is_none() {
            self.read_char()
        }
        
        self.ch.take()
    }
    
    /// Returns the next char without consuming it.
    fn peek_char(&mut self) -> Option<char> {
        if self.ch.is_none() {
            self.read_char()
        }
        
        self.ch.clone()
    }
    
    /// Reads the next char and stores it in the 'ch' field.
    fn read_char(&mut self) {
        self.ch = self.reader.read_char()
    }
}
