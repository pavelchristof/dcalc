//! Provides a monad trait and implementation for Result.

/// A result monad trait.
pub trait ResultMonad<T1, E> {
    fn bind<T2>(self, |T1| -> Result<T2, E>) -> Result<T2, E>;
    fn bind_with<T2, A>(self, A, |A, T1| -> Result<T2, E>) -> Result<T2, E>;
}

impl<T1, E> ResultMonad<T1, E> for Result<T1, E> {
    fn bind<T2>(self, f: |T1| -> Result<T2, E>) -> Result<T2, E> {
        match self {
            Ok(t)  => f(t),
            Err(e) => Err(e)
        }
    }
    
    fn bind_with<T2, A>(self, a: A, f: |A, T1| -> Result<T2, E>) -> Result<T2, E> {
        match self {
            Ok(t)  => f(a, t),
            Err(e) => Err(e)
        }
    }
}
