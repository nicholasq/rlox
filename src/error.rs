use crate::token::Token;

/// Represents a runtime error that occurs during the execution of the interpreter.
/// Contains an error message and the token associated with the error.
#[derive(Debug)]
pub struct RuntimeError {
    /// The error message describing what went wrong.
    pub message: String,
    /// The token associated with the error, providing context such as the location in the source code.
    pub token: Option<Token>,
}

impl RuntimeError {
    /// Creates a new `RuntimeError` instance.
    ///
    /// # Arguments
    ///
    /// * `message` - A string describing the error.
    /// * `token` - The token associated with the error.
    pub fn new(message: String, token: Option<Token>) -> Self {
        Self { message, token }
    }
}

/// Macro to construct a `RuntimeError`.
///
/// # Arguments
///
/// * `$message` - A string literal describing the error.
/// * `$token` - The token associated with the error.
#[macro_export]
macro_rules! runtime_err {
    ($message:expr, $token:expr) => {
        RuntimeError::new($message.to_string(), $token)
    };
}
