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

//! The change log

/// Release 0.2.1
///
/// Add a new error type: [`crate::DuskError::DependencyError`] which can
/// notify the program that some interplugin dependency must be resolved
/// before the action can be performed, and carries an InterplugRequest,
/// identifying such dependency
///
pub mod r0_2_1 {}

/// Release 0.2.0 (2021-05-03)
///
/// Add more functionality to traits, such as not only getting all
/// functions at once, but getting functions, types, trait declarations
/// and even callables by names and IDs.
///
/// Split the project into different files, but make it in such way
/// as not to add another layer of incapsualtion (pub use each module)
///
pub mod r0_2_0 {}

/// Release 0.1.3 (2021-04-10)
///
/// Add pub use statements for DuskError and InterplugRequest enums
/// for easier accessibility. Also change name of requests a little
///
/// Swithch all strs for name and string representations for Strings
/// as they proved to be much more convenient to work with
///
/// Make function names of functions provided by autoimplemented
/// get_function_list (and names of types, provided by 
/// get_type_list) resemble actual identificators by which these
/// functions would be called, with complete path to the function
/// included in the name
///
pub mod r0_1_3 {}

/// Release 0.1.2 (2021-04-09)
///
/// Add Debug trait implementations for every single defined structure 
/// and add a rule to throw error on missing_debug_implementation
/// lint so that all the structures created in the future will always
/// implement Debug trait
///
/// Implement all needed ordering functions for the Version structure
/// so that the plugin versions were easily comparable
///
pub mod r0_1_2 {}

/// Release 0.1.1 (2021-04-09)
///
/// Add a changelog.
///
/// Update all documentation and add some more stylistic changes.
///
/// Make more lints give errors on compile and make all warnings lead
/// to compile error, as it is unacceptable for a library to have
/// any warnings.
///
/// Get rid of all unwraps, switching to error handling via match
/// statements.
///
pub mod r0_1_1 {}
