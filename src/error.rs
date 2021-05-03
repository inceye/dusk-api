// Copyright (C) 2021 by Andy Gozas <andy@gozas.me>
//
// This file is part of Dusk API.
//
// Dusk API is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Dusk API is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Dusk API.  If not, see <https://www.gnu.org/licenses/>.

//! Module, containing everything needed for error handling

pub use DuskError::*;

/// Enum, that represents a message passed to the program using the
/// plugin when the function fails
#[derive(Debug)]
pub enum DuskError {

    /// Send a message, representing the runtime error, that occured
    /// while using a function.
    ///
    /// # Example
    /// ```
    /// fn call_function (
    ///     self: &mut Self,
    ///     _function_number: u64,
    ///     _args: Vec<&mut Box<dyn DuskObject>>
    ///     ) -> Result<Box<dyn DuskObject>, DuskError> {
    ///     Err(DuskError::RuntimeError (
    ///         "You can't call an empty freight"
    ///     ))
    /// }
    /// ```

    /// Plugin library loading failed
    LoadingError (libloading::Error),

    /// Plugin import failed
    ImportError (String),

    /// An argument of wrong type received
    TypeError (String),

    /// An argument of wrong value was received
    ValueError (String),

    /// An OS error occured
    OsError (String),

    /// At some point some value check failed
    AssertionError (String),

    /// Index out of bounds
    IndexError (String),

    /// Code is trying to divide by zero
    ZeroDivisionError (String),

    /// Out of memory
    OverflowError (String),

    /// Called function is not implemented
    NotImplementedError (String),

    /// Other error occured during runtime
    RuntimeError (String),
}
