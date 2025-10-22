use std::collections::HashMap;

use crate::error::RuntimeError;
use crate::expr::Literal;
use crate::token::Token;
use anyhow::Result;

/// Represents the environment in which variables and their values are stored.
/// Manages a stack of scopes, each of which is a mapping of variable names to their values.
#[derive(Debug)]
pub struct Environment {
    /// A stack of scopes, where each scope is a `HashMap` mapping variable names to their values.
    scopes: Vec<HashMap<String, Literal>>,
}

impl Environment {
    /// Creates a new `Environment` with a single, empty scope.
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    /// Adds a new scope to the environment.
    /// Scopes are used to manage variable lifetimes during block execution.
    pub fn add_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Removes the most recently added scope from the environment.
    /// Returns the removed scope as a `HashMap`.
    ///
    /// # Panics
    ///
    /// This function will panic if there are no scopes to remove.
    pub fn pop_scope(&mut self) -> HashMap<String, Literal> {
        self.scopes.pop().unwrap()
    }

    /// Defines a new variable in the current (innermost) scope.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to define.
    /// * `value` - The value of the variable.
    pub fn define(&mut self, name: String, value: Literal) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }

    /// Retrieves the value of a variable.
    /// Searches from the innermost scope outward.
    ///
    /// # Arguments
    ///
    /// * `name` - The token representing the variable name.
    ///
    /// # Returns
    ///
    /// A reference to the variable's value if it is found, otherwise a `RuntimeError`.
    pub fn get(&self, name: &Token) -> Result<&Literal, RuntimeError> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(&name.lexeme) {
                return Ok(value);
            }
        }
        Err(RuntimeError::new(
            format!("Undefined variable '{}'", name.lexeme),
            name.clone(),
        ))
    }

    /// Updates the value of an existing variable.
    /// Searches from the innermost scope outward.
    ///
    /// # Arguments
    ///
    /// * `name` - The token representing the variable name.
    /// * `value` - The new value of the variable.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `RuntimeError` if the variable is not defined.
    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), RuntimeError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name.lexeme) {
                scope.insert(name.lexeme.clone(), value.clone());
                return Ok(());
            }
        }
        Err(RuntimeError::new(
            format!("Undefined variable '{}'", name.lexeme),
            name.clone(),
        ))
    }
}
