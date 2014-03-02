//! Module used to simplify functions.
//! Pretty hacky.

use func::{DiffFunc, Exp, Ln, Sin, Cos, Constant, Power, Plus, Minus, Mul, Div, Compose};

/// Simplifies something.
pub trait Simplify {
    fn simplify(&self) -> Self;
    fn simplify_step(&self) -> Self;
}

impl Simplify for DiffFunc {
    /// Constructs a new, simplified function.
    fn simplify(&self) -> DiffFunc {
        let mut old = self.clone();
        let mut new = old.simplify_step();
        while old != new {
            old = new;
            new = old.simplify_step();
        }
        new
    }
    
    /// Constructs a new, simplified function.
    fn simplify_step(&self) -> DiffFunc {
        match *self {
            Exp => Exp,
            Ln  => Ln,
            Sin => Sin,
            Cos => Cos,
            
            Constant(f) => Constant(f),
            Power(f)    => if f == 0.0 { Constant(1.0) } else { Power(f) },
            
            Plus { left: ref l, right: ref r } => {
                let l = l.simplify();
                let r = r.simplify();
                
                match (l, r) {
                    (Constant(x), Constant(y)) => Constant(x + y),
                    (Constant(0.0), r) => r,
                    (l, Constant(0.0)) => l,
                    (f1, Plus { left: f2, right: f3 }) => Plus { left: ~Plus { left: ~f1, right: f2 }, right: f3 },
                    (l, r) => if l == r { 
                        Mul { left: ~Constant(2.0), right: ~l } 
                    } else { 
                        Plus { left: ~l, right: ~r } 
                    }
                }
            },
            
            Minus { left: ref l, right: ref r } => {
                let l = l.simplify();
                let r = r.simplify();
                
                match (l, r) {
                    (Constant(x), Constant(y)) => Constant(x - y),
                    (Constant(0.0), r) => Mul { left: ~Constant(-1.0), right: ~r },
                    (l, Constant(0.0)) => l,
                    (l, r) => Minus { left: ~l, right: ~r }
                }
            },
            
            Mul { left: ref l, right: ref r } => {
                let l = l.simplify();
                let r = r.simplify();
                
                match (l, r) {
                    (Constant(x), Constant(y)) => Constant(x * y),
                    (Constant(1.0), r) => r,
                    (l, Constant(1.0)) => l,
                    (Constant(0.0), _) => Constant(0.0),
                    (_, Constant(0.0)) => Constant(0.0),
                    (Power(a), Power(b)) => Power(a + b),
                    (f, Mul { left: ~Power(a), right: ~Power(b) }) => Mul { left: ~Power(a + b), right: ~f },
                    (Power(a), Mul { left: f, right: ~Power(b) }) => Mul { left: ~Power(a + b), right: f },
                    (Power(a), Mul { left: ~Power(b), right: f }) => Mul { left: ~Power(a + b), right: f },
                    (Mul { left: ~Power(a), right: ~Power(b) }, f) => Mul { left: ~Power(a + b), right: ~f },
                    (Mul { left: ~Power(a), right: f }, Power(b)) => Mul { left: ~Power(a + b), right: f },
                    (Mul { left: f, right: ~Power(a) }, Power(b)) => Mul { left: ~Power(a + b), right: f },
                    (Compose { outer: ~Exp, inner: f1 }, Compose { outer: ~Exp, inner: f2 }) => 
                        Compose { outer: ~Exp, inner: ~Plus { left: f1, right: f2 } },
                    (f1, Mul { left: f2, right: f3 }) => Mul { left: ~Mul { left: ~f1, right: f2 }, right: f3 },
                    (l, r) => Mul { left: ~l, right: ~r }
                }
            },
            
            Div { left: ref l, right: ref r } => {
                let l = l.simplify();
                let r = r.simplify();
                
                match (l, r) {
                    (Constant(x), Constant(y)) => Constant(x / y),
                    (l, Constant(1.0)) => l,
                    (l, r) => Div { left: ~l, right: ~r }
                }
            },
            
            Compose { outer: ref o, inner: ref i } => {
                let o = o.simplify();
                let i = i.simplify();
                
                match (o, i) {
                    (Power(a), Power(b)) => Power(a * b),
                    (Exp, Ln) => Power(1.0),
                    (Exp, Compose { outer: ~Ln, inner: f }) => *f,
                    (Exp, Mul {
                        left: ~Compose {
                            outer: ~Ln,
                            inner: f1
                        },
                        right: ~Constant(c)
                    }) => Compose { outer: ~Power(c), inner: f1 },
                    (Exp, Mul {
                        left: ~Constant(c),
                        right: ~Compose {
                            outer: ~Ln,
                            inner: f1
                        }
                    }) => Compose { outer: ~Power(c), inner: f1 },
                    (Ln, Exp) => Power(1.0),
                    (Ln, Compose { outer: ~Exp, inner: f }) => *f,
                    (o, i) => Compose { outer: ~o, inner: ~i }
                }
            }
        }
    }
}
