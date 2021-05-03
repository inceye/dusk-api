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

//! Module, containing everything needed to declare traits to be
//! used by other plugins

use crate::*;

/// Structure that holds all the characteristics of a trait 
/// function that need to be known when actually implementing
/// it or importing its implementor.
///
/// A TraitFunctionDefinition object contains
/// * function name
/// * its trait id number that identifies it's place in the trait
/// implementation function Vector
/// * vector of parameter descriptions of the parameters 
/// ([`Parameter`])
/// * its return [`TypeId`]
/// * whether or not the arguments should be checked or just passed 
/// as is
#[derive(Clone, Debug)]
pub struct TraitFunctionDefinition {

    /// Function name, as a reference to a static string. Mainly
    /// used to give user the ability to choose the function they
    /// want to use
    pub name: String,

    /// Function ID, used to find this function in the trait 
    /// implementation function vector
    ///
    /// **Should always be the same for same functions in the newer
    /// releases, unless a new plugin version is submitted**
    pub fn_trait_id: u64,

    /// A vector of function parameter definitions, as objects 
    /// of type [`Parameter`]
    ///
    /// This field contains all information, compiler needs to 
    /// know about argument amount, types and keywords in case
    /// 
    pub parameters: Vec<Parameter>,

    /// The [`TypeId`] of the returned [`Any`] trait implementor
    ///
    /// See [`std::any::Any`] documentation to find out more about
    /// storing an [`Any`] trait implementor and getting back
    /// from a [`Box<dyn DuskObject>`]
    pub return_type: TypeId,

    /// Bool that identifyes whether or not should the compiler
    /// ignore argument types, and just hand them over to the
    /// function as is.
    ///
    /// In this case, the keywords will not be checked, so all
    /// keyword arguments will be provided as objects of type
    /// [`Kwarg`]
    /// 
    /// If the function might take different arguments in different
    /// situations, or even have unlimited amount of arguments,
    /// sometimes it is easier to make one function that would
    /// parse the arguments and decide how to deal with them. For 
    /// such function, compiler will not check the argument types
    /// nor amount of them.
    pub no_check_args: bool,
}

impl Default for TraitFunctionDefinition {
    fn default () -> TraitFunctionDefinition {
        TraitFunctionDefinition {
            name: "".to_string(),
            fn_trait_id: 0,
            parameters: Vec::new(),
            return_type: TypeId::of::<u8>(),
            no_check_args: false,
        }
    }
}

/// Structure representing main characteristics of a function needed
/// for the program using a plugin, which implements it
///
/// A TraitFunction object contains
/// * its number in trait
/// * the underlying function
#[derive(Clone, Debug)]
pub struct TraitFunction {

    /// Function ID, used to call this function
    ///
    /// **Should always be the same for same functions in the newer
    /// releases, unless a new plugin version is submitted**
    pub fn_trait_id: u64,

    /// The underlying function that contains everything else
    /// you need to know
    pub function: Function,
}

impl Default for TraitFunction {
    fn default () -> TraitFunction {
        TraitFunction {
            fn_trait_id: 0,
            function: Default::default(),
        }
    }
}

/// A trait definition, that contains the defined trait's name
/// and a vector of definitions of it's functions
#[derive(Clone, Debug)]
pub struct TraitDefinition {

    /// The trait's name
    pub name: String,

    /// Trait definition ID
    pub td_id: usize,

    /// The method definitions
    pub methods: Vec<TraitFunctionDefinition>,
}

impl Default for TraitDefinition {
    fn default () -> TraitDefinition {
        TraitDefinition {
            name: "".to_string(),
            td_id: 0,
            methods: Vec::new(),
        }
    }
}

/// A trait implementation, that contains name of the trait
/// being implemented and a vector of the trait method
/// implementations
#[derive(Clone, Debug)]
pub struct TraitImplementation {

    /// Trait name (containing full path to it in the plugin
    /// where it came from)
    pub name: String,

    /// Methods being implemented
    pub methods: Vec<TraitFunction>,
}

/// TODO: trait proxy not perfect for the cause yet
#[derive(Clone, Debug)]
pub struct TraitProxy {

    /// The name of the trait
    pub trait_name: String,

    /// The plugin where it came from
    pub freight_proxy: std::rc::Rc<FreightProxy>,

    /// The vector, linking IDs of the Trait functions to the actual
    /// general plugin function IDs
    pub function_links: Vec<usize>,
}
