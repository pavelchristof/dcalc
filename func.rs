//! Differentiable functions.

/// An differentiable function.
pub trait DiffFunc {
    fn to_str(&self, arg: &str) -> ~str;
    fn clone(&self) -> ~DiffFunc;
    fn derivative(&self) -> ~DiffFunc;
}

/// A constant function.
pub struct Constant {
    value: f64
}

impl DiffFunc for Constant {
    fn to_str(&self, _: &str) -> ~str {
        self.value.to_str()
    }

    fn clone(&self) -> ~DiffFunc {
        ~Constant { value: self.value } as ~DiffFunc
    }

    fn derivative(&self) -> ~DiffFunc {
        ~Constant { value: 0.0 } as ~DiffFunc
    }
}

/// An identity function.
pub struct Identity;

impl DiffFunc for Identity {
    fn to_str(&self, arg: &str) -> ~str {
        arg.to_owned()
    }
    
    fn clone(&self) -> ~DiffFunc {
        ~Identity as ~DiffFunc
    }
    
    fn derivative(&self) -> ~DiffFunc {
        ~Constant { value: 1.0 } as ~DiffFunc
    }
}

/// An exponential function.
pub struct Exp;

impl DiffFunc for Exp {
    fn to_str(&self, arg: &str) -> ~str {
        format!("exp({})", arg)
    }

    fn clone(&self) -> ~DiffFunc {
        ~Exp as ~DiffFunc
    }

    fn derivative(&self) -> ~DiffFunc {
        ~Exp as ~DiffFunc
    }
}

/// Natural logarithm.
pub struct Ln;

impl DiffFunc for Ln {
    fn to_str(&self, arg: &str) -> ~str {
        format!("ln({})", arg)
    }

    fn clone(&self) -> ~DiffFunc {
        ~Ln as ~DiffFunc
    }

    fn derivative(&self) -> ~DiffFunc {
        ~Div {
            left: ~Constant { value: 1.0 } as ~DiffFunc,
            right: ~Identity as ~DiffFunc
        } as ~DiffFunc
    }
}

/// Sinus function.
pub struct Sin;

impl DiffFunc for Sin {
    fn to_str(&self, arg: &str) -> ~str {
        format!("sin({})", arg)
    }

    fn clone(&self) -> ~DiffFunc {
        ~Sin as ~DiffFunc
    }

    fn derivative(&self) -> ~DiffFunc {
        ~Cos as ~DiffFunc
    }
}

/// Cosinus function.
pub struct Cos;

impl DiffFunc for Cos {
    fn to_str(&self, arg: &str) -> ~str {
        format!("cos({})", arg)
    }

    fn clone(&self) -> ~DiffFunc {
        ~Cos as ~DiffFunc
    }

    fn derivative(&self) -> ~DiffFunc {
        ~Mul {
            left: ~Constant { value: -1.0 } as ~DiffFunc,
            right: ~Sin as ~DiffFunc
        } as ~DiffFunc
    }
}

/// A power function.
pub struct Power {
    exponent: f64
}

impl DiffFunc for Power {
    fn to_str(&self, arg: &str) -> ~str {
        format!("({}^{})", arg, self.exponent)
    }

    fn clone (&self) -> ~DiffFunc {
        ~Power { exponent: self.exponent } as ~DiffFunc
    }

    fn derivative(&self) -> ~DiffFunc {
        ~Mul {
            left: ~Constant { value: self.exponent } as ~DiffFunc,
            right: ~Power { exponent: self.exponent - 1.0 } as ~DiffFunc
        } as ~DiffFunc
    }
}

/// A plus operator.
pub struct Plus {
    left: ~DiffFunc,
    right: ~DiffFunc
}

impl DiffFunc for Plus {
    fn to_str(&self, arg: &str) -> ~str {
        format!("({} + {})", self.left.to_str(arg), self.right.to_str(arg))
    }

    fn clone(&self) -> ~DiffFunc {
        ~Plus { left: self.left.clone(), right: self.right.clone() } as ~DiffFunc
    }
    
    fn derivative(&self) -> ~DiffFunc {
        ~Plus { left: self.left.derivative(), right: self.right.derivative() } as ~DiffFunc
    }
}

/// A minus operator.
pub struct Minus {
    left: ~DiffFunc,
    right: ~DiffFunc
}

impl DiffFunc for Minus {
    fn to_str(&self, arg: &str) -> ~str {
        format!("({} - {})", self.left.to_str(arg), self.right.to_str(arg))
    }

    fn clone(&self) -> ~DiffFunc {
        ~Minus { left: self.left.clone(), right: self.right.clone() } as ~DiffFunc
    }
    
    fn derivative(&self) -> ~DiffFunc {
        ~Minus { left: self.left.derivative(), right: self.right.derivative() } as ~DiffFunc
    }
}

/// A multiply operator.
pub struct Mul {
    left: ~DiffFunc,
    right: ~DiffFunc
}

impl DiffFunc for Mul {
    fn to_str(&self, arg: &str) -> ~str {
        format!("({} * {})", self.left.to_str(arg), self.right.to_str(arg))
    }    

    fn clone(&self) -> ~DiffFunc {
        ~Mul { left: self.left.clone(), right: self.right.clone() } as ~DiffFunc
    }

    fn derivative(&self) -> ~DiffFunc {
        ~Plus { 
            left: ~Mul {
                left: self.left.derivative(),
                right: self.right.clone()
            } as ~DiffFunc,
            right: ~Mul {
                left: self.left.clone(),
                right: self.right.derivative()
            } as ~DiffFunc
        } as ~DiffFunc
    }
}

/// A divide operator.
pub struct Div {
    left: ~DiffFunc,
    right: ~DiffFunc
}

impl DiffFunc for Div {
    fn to_str(&self, arg: &str) -> ~str {
        format!("({} / {})", self.left.to_str(arg), self.right.to_str(arg))
    }

    fn clone(&self) -> ~DiffFunc {
        ~Div { left: self.left.clone(), right: self.right.clone() } as ~DiffFunc
    }

    fn derivative(&self) -> ~DiffFunc {
        ~Div {
            left: ~Minus {
                left: ~Mul {
                    left: self.left.derivative(),
                    right: self.right.clone()
                } as ~DiffFunc,
                right: ~Mul {
                    left: self.left.clone(),
                    right: self.right.derivative()
                } as ~DiffFunc
            } as ~DiffFunc,
            right: ~Compose {
                outer: ~Power {
                    exponent: 2.0
                } as ~DiffFunc,
                inner: self.right.clone()
            } as ~DiffFunc
        } as ~DiffFunc
    }
}

/// Function composition.
pub struct Compose {
    outer: ~DiffFunc,
    inner: ~DiffFunc
}

impl DiffFunc for Compose {
    fn to_str(&self, arg: &str) -> ~str {
        self.outer.to_str(self.inner.to_str(arg))
    }

    fn clone(&self) -> ~DiffFunc {
        ~Compose { outer: self.outer.clone(), inner: self.inner.clone() } as ~DiffFunc
    }

    fn derivative(&self) -> ~DiffFunc {
        ~Mul {
            left: ~Compose {
                outer: self.outer.derivative(),
                inner: self.inner.clone()
            } as ~DiffFunc,
            right: self.inner.derivative()
        } as ~DiffFunc
    }
}
