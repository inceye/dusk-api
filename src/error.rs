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

use crate::*;
pub use DuskError::*;

/// Enum, that represents a message passed to the program using the
/// plugin when the function fails.
///
/// Interplugin communication is meant to be panic safe, so that 
/// the program does not just completely fail just because one plugin
/// failed. Instead if the plugin fails to perform any action, instead
/// of the normal return value, it should return [`DuskError`], so 
/// that it could be parsed by the program, and considered when making
/// the decision to either try and fix the problem by providing 
/// dependencies, trying another plugin, or notifying user in the most
/// comprehensible way possible of what the problem is and how it might
/// possibly be solved.
///
/// # Example
/// ```
/// fn add (
///     a: u8, 
///     b: u8,
/// ) -> Result<u8, dusk_api::DuskError> {
///     
///     if ((255 - b) < a) {
///         return Err(dusk_api::OverflowError(
///             format!(
///                 "Result ({}) does not fit in one byte",
///                 a as usize + b as usize,
///                 )));
///     }
///     return Ok(a + b);
/// }
///
/// fn add_many (
///     lst: Vec<u8>,
/// ) -> Result<u8, dusk_api::DuskError> {
///
///     let mut res: u8 = 0;
///     for elm in lst {
///         res = add(elm, res)?;
///     }
///     return Ok(res);
/// }
///
/// assert!(add(1, 2).is_ok());
/// assert!(add_many(vec![1, 2, 3]).is_ok());
///
/// assert!(!add(100, 200).is_ok());
/// assert!(!add_many(vec![100, 200, 30]).is_ok());
/// ```
#[derive(Debug)]
pub enum DuskError {

    /// Plugin library loading failed
    LoadingError (libloading::Error),

    /// Plugin did not receive a dependency that is crucial for the
    /// requested action
    DependencyError (InterplugRequest),

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
