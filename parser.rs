//! Function parser.

use func;
use tokenizer;
use func::{DiffFunc};
use monad::ResultMonad;
use tokenizer::{Token, Tokenizer};

/// Parses a string into a ~DiffFunc.
pub struct Parser<R> {
    priv tokenizer: ~Tokenizer<R>
}

impl<R: Reader> Parser<R> {
    /// Returns a new parser.
    pub fn new(tokenizer: ~Tokenizer<R>) -> Parser<R> {
        Parser::<R> { tokenizer: tokenizer }
    }
    
    /// Creates and runs the parser
    pub fn parse(reader: R) -> Result<~DiffFunc, ~str> {
        let tokenizer = ~Tokenizer::<R>::new(reader);
        let mut parser = Parser::<R>::new(tokenizer);
        parser.run()
    }
    
    /// Runs the parser and returns an function.
    pub fn run(&mut self) -> Result<~DiffFunc, ~str> {
        match self.statement() {
            Ok(f)  => {
                // Expect an eof.
                if self.tokenizer.eof() {
                    Ok(f)
                } else {
                    Err(format!("Expected eof, got {}.", self.tokenizer.peek().to_str()))
                }
            },
            Err(s) => Err(s)
        }
    }
    
    /// Parses an statement.
    fn statement(&mut self) -> Result<~DiffFunc, ~str> {
        // Just an expression for now.
        self.expression()
    }
    
    /// Parses an expression.
    fn expression(&mut self) -> Result<~DiffFunc, ~str> {
        self.pm_ops()
    }
    
    /// Parses plus-minus level operations.
    fn pm_ops(&mut self) -> Result<~DiffFunc, ~str> {
        self.md_ops().bind(|left| self.pm_ops_tail(left))
    }
    
    /// Parses a plus-minus level operations tail.
    fn pm_ops_tail(&mut self, left: ~DiffFunc) -> Result<~DiffFunc, ~str> {
        match self.tokenizer.peek() {
            // + #md_ops #pm_ops_tail
            Some(tokenizer::Plus) => {
                self.tokenizer.take();
                self.md_ops().bind_with(left, |left, right| self.pm_ops_tail(
                    ~func::Plus { left: left, right: right }
                ))
            },
            
            // - #md_ops #pm_ops_tail
            Some(tokenizer::Minus) => {
                self.tokenizer.take();
                self.md_ops().bind_with(left, |left, right| self.pm_ops_tail(
                    ~func::Minus { left: left, right: right }
                ))
            },
            
            // e
            _ => Ok(left)
        }
    }
    
    /// Parses multiply-divide level operations.
    fn md_ops(&mut self) -> Result<~DiffFunc, ~str> {
        self.power_ops().bind(|left| self.md_ops_tail(left))
    }
    
    /// Parses multiply-divide level operations tail.
    fn md_ops_tail(&mut self, left: ~DiffFunc) -> Result<~DiffFunc, ~str> {
        match self.tokenizer.peek() {
            // * #power_ops #md_ops_tail
            Some(tokenizer::Mul) => {
                self.tokenizer.take();
                self.power_ops().bind_with(left, |left, right| self.md_ops_tail(
                    ~func::Mul { left: left, right: right }
                ))
            },
            
            // / #power_ops #md_ops_tail
            Some(tokenizer::Div) => {
                self.tokenizer.take();
                self.power_ops().bind_with(left, |left, right| self.md_ops_tail(
                    ~func::Div { left: left, right: right }
                ))
            },
            
            // e
            _ => Ok(left)
        }
    }
    
    /// Parses power operations.
    fn power_ops(&mut self) -> Result<~DiffFunc, ~str> {
        self.operand().bind(|left| self.power_ops_tail(left))
    }
    
    /// Parses power operations tail.
    fn power_ops_tail(&mut self, left: ~DiffFunc) -> Result<~DiffFunc, ~str> {
        match self.tokenizer.peek() {
            // ^ #operand #power_ops_tail
            Some(tokenizer::Power) => {
                self.tokenizer.take();
                self.operand().bind_with(left, |left, right| self.power_ops_tail(
                    ~func::Compose {
                        outer: ~func::Exp,
                        inner: ~func::Mul {
                            left: ~func::Compose {
                                outer: ~func::Ln,
                                inner: left
                            },
                            right: right
                        }
                    }
                ))
            }
            
            // e
            _ => Ok(left)
        }
    }
    
    /// Parses an operand: a number, an 'x' identifier, a build-in function or an bracketed expression.
    fn operand(&mut self) -> Result<~DiffFunc, ~str> {
        match self.tokenizer.peek() {
            // Number
            Some(tokenizer::Number(f)) => {
                self.tokenizer.take();
                Ok(~func::Constant(f))
            },
            
            // Id
            Some(tokenizer::Id(s)) => {
                if s == ~"x" {
                    self.tokenizer.take();
                    Ok(~func::Power(1.0))
                } else {
                    Err(format!("Invalid identifier. Use 'x' as the variable name."))
                }
            },
            
            // Exp #operand
            Some(tokenizer::Exp) => {
                self.tokenizer.take();
                self.operand().bind(|expr| Ok(
                    ~func::Compose {
                        outer: ~func::Exp,
                        inner: expr
                    }
                ))
            },
            
            // Ln #operand
            Some(tokenizer::Ln) => {
                self.tokenizer.take();
                self.operand().bind(|expr| Ok(
                    ~func::Compose {
                        outer: ~func::Ln,
                        inner: expr
                    }
                ))
            },
            
            // Sin #operand
            Some(tokenizer::Sin) => {
                self.tokenizer.take();
                self.operand().bind(|expr| Ok(
                    ~func::Compose {
                        outer: ~func::Sin,
                        inner: expr
                    }
                ))
            },
            
            // Cos #operand
            Some(tokenizer::Cos) => {
                self.tokenizer.take();
                self.operand().bind(|expr| Ok(
                    ~func::Compose {
                        outer: ~func::Cos,
                        inner: expr
                    }
                ))
            },
            
            // Tg #operand
            Some(tokenizer::Tg) => {
                self.tokenizer.take();
                self.operand().bind(|expr| Ok(
                    ~func::Compose {
                        outer: ~func::Div {
                            left: ~func::Sin,
                            right: ~func::Cos
                        },
                        inner: expr
                    }
                ))
            },
            
            // Ctg #operand
            Some(tokenizer::Ctg) => {
                self.tokenizer.take();
                self.operand().bind(|expr| Ok(
                    ~func::Compose {
                        outer: ~func::Div {
                            left: ~func::Cos,
                            right: ~func::Sin
                        },
                        inner: expr
                    }
                ))
            },
            
            // #bracket_expr
            Some(tokenizer::OpenBracket) => self.bracket_expr(),
            
            // errors
            Some(t) => Err(format!("Expected an operand, got {}.", t.to_str())),
            None    => Err(~"Expected an operand, got eof.")
        }
    }
    
    /// Parses a bracketed expression.
    fn bracket_expr(&mut self) -> Result<~DiffFunc, ~str> {
        self.expect(tokenizer::OpenBracket).bind(|_| 
            self.expression().bind(|expr|
                self.expect(tokenizer::CloseBracket).bind_with(expr, |expr, _|
                    Ok(expr)
                )
            )
        )
    }
    
    /// Consumes a token or fails.
    fn expect(&mut self, token: Token) -> Result<(), ~str> {
        match self.tokenizer.peek() {
            Some(t) => {
                if token == t {
                    self.tokenizer.take();
                    Ok(())
                } else {
                    Err(format!("Expected {} token, got {}.", token.to_str(), t.to_str()))
                }
            },
            None => Err(format!("Expected {} token, got eof.", token.to_str()))
        }
    }
}
