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

//! Module, containing everything needed to provide a type to
//! be used py other plugins

use crate::*;

/// Structure representing main characteristics of an object type
/// needed for the program, using the plugin, that either imports
/// or defines this type in case this type is not present in
/// the user program itself
///
/// A Type object contains
/// * type name, used for identifying this type
/// * its [`TypeId`] for Any trait to work properly
/// * its methods
/// * trait implementations for this type
/// * functions needed to access its fields
#[derive(Clone, Debug)]
pub struct Type {

    /// Name for the [`TypeId`] owner to be reffered to as a static
    /// string
    pub name: String,

    /// The **INTERNAL** id for the type, representing the position
    /// of the type in the type vector, **NOT** the native [`TypeId`]**
    pub tp_id: usize,

    pub generator : fn () -> Result<Box<dyn DkAny>, Error>,

    /// If an object of this type should have some functions, that
    /// can be called on it, they should be provided here. The function
    /// IDs of these functions must be unique over all other functions
    /// in the plugin
    pub methods : Vec<Function>,

    /// All fields of an object of this type, user needs to be able
    /// to access, should be located here. The field name then
    /// will be the function name, function's return type is the
    /// field type and the only argument of the function should
    /// be of the type, the field is owned by. The function
    /// IDs of these functions must be unique over all other functions
    /// in the plugin
    pub fields : Vec<Function>,

    /// All the traits that are implemented for this type
    pub trait_implementations : Vec<TraitImplementation>,

// XXX: native id is not really needed anymore, and is just a wee
// bit confusing because we already have another ID and all objects
// come in a structure along with the Type that describes them, so
// we don't really need another way to check what's inside the object
    /// [`TypeId`] object, gotten from the structure, being
    /// provided to the program, that is using the plugin
    ///
    /// See [`std::any::TypeId`] documentation to find out how
    /// to get a type id of a type
    pub native_id: TypeId,
}

impl Default for Type {
    fn default () -> Type {
        Type {
            name: "u8".to_string(),
            tp_id: U8_type_id,
            generator: U8::dk_new,
            methods: Vec::new(),
            fields: Vec::new(),
            trait_implementations: Vec::new(),
            native_id: TypeId::of::<U8>(),
        }
    }
}
