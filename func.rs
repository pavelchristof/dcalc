//! Differentiable functions.

/// An Differentiable function.
#[deriving(Clone)]
pub enum DiffFunc {
    Exp,
    Ln,
    Sin,
    Cos,
    
    Constant(f64),
    Power(f64),
    
    Plus  { left: ~DiffFunc, right: ~DiffFunc },
    Minus { left: ~DiffFunc, right: ~DiffFunc },
    Mul   { left: ~DiffFunc, right: ~DiffFunc },
    Div   { left: ~DiffFunc, right: ~DiffFunc },
    Compose { outer: ~DiffFunc, inner: ~DiffFunc }
}

impl DiffFunc {
    /// Converts a function to a string.
    pub fn to_str(&self, arg: &str) -> ~str {
        match *self {
            Exp => format!("exp({})", arg),
            Ln  => format!("ln({})", arg),
            Sin => format!("sin({})", arg),
            Cos => format!("cos({})", arg),
            
            Constant(f) => f.to_str(),
            Power(f) => format!("({}^{})", arg, f),
            
            Plus { left: ref l, right: ref r } => format!("({} + {})", l.to_str(arg), r.to_str(arg)),
            Minus { left: ref l, right: ref r } => format!("({} - {})", l.to_str(arg), r.to_str(arg)),
            Mul { left: ref l, right: ref r } => format!("({} * {})", l.to_str(arg), r.to_str(arg)),
            Div { left: ref l, right: ref r } => format!("({} / {})", l.to_str(arg), r.to_str(arg)),
            Compose { outer: ref o, inner: ref i } => o.to_str(i.to_str(arg))
        }
    }
    
    /// Computes the derivative.
    pub fn derivative(&self) -> DiffFunc {
        match *self {
            Exp => Exp,
            Ln  => Div { left: ~Constant(1.0), right: ~Power(1.0) },
            Sin => Cos,
            Cos => Mul { left: ~Constant(-1.0), right: ~Sin },
            
            Constant(_) => Constant(0.0),
            Power(f) => Mul { left: ~Constant(f), right: ~Power(f - 1.0) },
            
            Plus { left: ref l, right: ref r } => Plus { left: ~l.derivative(), right: ~r.derivative() },
            Minus { left: ref l, right: ref r } => Minus { left: ~l.derivative(), right: ~r.derivative() },
            Mul { left: ref l, right: ref r } => Plus {
                left: ~Mul { left: ~l.derivative(), right: r.clone() },
                right: ~Mul { left: l.clone(), right: ~r.derivative() }
            },
            Div { left: ref l, right: ref r } => Div {
                left: ~Minus {
                    left: ~Mul { left: ~l.derivative(), right: r.clone() },
                    right: ~Mul { left: l.clone(), right: ~r.derivative() }
                },
                right: ~Compose {
                    outer: ~Power(2.0),
                    inner: r.clone()
                }
            },
            Compose { outer: ref o, inner: ref i } => Mul {
                left: ~Compose {
                    outer: ~o.derivative(),
                    inner: i.clone()
                },
                right: ~i.derivative()
            }
        }
    }
}
