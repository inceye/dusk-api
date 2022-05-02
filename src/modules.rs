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

//! Module, containing everything needed to provide a module to
//! be used py other plugins

use crate::*;

/// A structure, that defines a module and may or may not contain
/// types, functions, submodules, trait definitions and
/// constants
#[derive(Clone, Debug)]
pub struct Module {

    /// The module name
    pub name: String,

    /// Module ID, used to find this module in the main plugin
    /// module vector of all modules (including nested modules)
    pub md_id: usize,

    /// A vector of types, presented in this module
    pub types: Vec<Type>,

    /// A vector of functions, implemented in this module
    /// (not including the functions that are used to
    /// get type fields and constants)
    pub functions: Vec<Function>,

    /// A vector of submodules, contained in this module
    pub submodules: Vec<Module>,

    /// A vector of trait definitions this module presents
    pub trait_definitions: Vec<TraitDefinition>,

    /// A vector of functions, that can be used to get some
    /// constants, defined in this module
    pub constants: Vec<Function>,
}

impl Default for Module {
    fn default () -> Module {
        Module {
            name: "".to_string(),
            md_id: 0,
            types: Vec::new(),
            functions: Vec::new(),
            submodules: Vec::new(),
            trait_definitions: Vec::new(),
            constants: Vec::new(),
        }
    }
}
