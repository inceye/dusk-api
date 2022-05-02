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

//! Module, containing everything needed for function declaration and
//! binding it to callables and IDs

use crate::*;

/// A structure, that contains all information, the compiler needs to
/// know about function parameters.
///
/// The only required field is arg_type, but there are some optional
/// fields you might want to use.
#[derive(Clone, Debug)]
pub struct Parameter {

    /// The argument type, as it's [`TypeId`]
    ///
    /// Always required
    pub arg_type: TypeId,

    /// Argument can be of any type, don't check TypeId
    ///
    /// Default: false
    pub any_type: bool,

    /// If the argument should be just a response to the Interplug request
    /// by itself without any value bound to it, set trait_only to true
    ///
    /// All trait_only parameters *MUST* be placed in the beginning of
    /// the parameter list
    pub trait_only: bool,

    /// If any type is set to true, check if the type has some traits
    /// implemented
    ///
    /// If implements is set to Some, and trait_only is set to false,
    ///
    /// Implements can be a RequestEach, so it might be requesting that
    /// the type implements multiple traits with a single InterplugRequest
    pub implements: Option<InterplugRequest>,

    /// Allow to change the value of the argument
    ///
    /// *NOTE* Can only be used with arguments with no
    /// default value and allow_multiple set to false
    pub mutable: bool,

    /// Forbid for this parameter to be set with a
    /// positional arg instead of keyword arg
    pub keyword_only: bool,

    /// If you want to add ability to set the parameter, using a
    /// keyword, you might want to set the keyword to [`Some`],
    ///
    /// Default value is [`None`]
    pub keyword: Option<String>,

    /// If your keyword argument is optional, you can set
    /// it's default value to Some
    ///
    /// Default value is [`None`]
    pub default_value: Option<Object>,

    /// If you want the user to be able to pass multiple arguments
    /// with one keyword or just multiple positon arguments, you
    /// would need to set allow_multiple to true
    ///
    /// In this case, in the vector of arguments passed to the
    /// call function, all arguments for this [`Parameter`] will
    /// be grouped in a vector, at the position of this parameter
    /// at the vector of parameters
    ///
    /// *NOTE* Only one positional parameter may be multiple.
    /// If keyword parameters are multiple or there is already a
    /// multiple positional argument, there may be as many
    /// of multiple keyword parameters as you want, but the
    /// keywords will be required for all of them and can not
    /// be omitted by user.
    ///
    /// Default value is false
    pub allow_multiple: bool,

    /// If the parameter allows for multiple arguments to be
    /// passed in it, you may set max_amount of those arguments.
    ///
    /// Set to 0 to allow unlimited arguments
    ///
    /// Default value is 0
    pub max_amount: usize,
}

impl Default for Parameter {
    fn default () -> Parameter {
        Parameter {
            arg_type: TypeId::of::<u8>(),
            any_type: false,
            trait_only: false,
            implements: None,
            mutable: false,
            keyword_only: false,
            keyword: None,
            default_value: None,
            allow_multiple: false,
            max_amount: 0,
        }
    }
}

/// The struct that represents a keyword argument if it is passed
/// to the function with no_check_args set to true
#[derive(Debug)]
pub struct Kwarg {

    /// The keyword
    pub keyword: String,

    /// The actual argument value
    pub value: Object,
}

//pub struct TraitArg<'a> {
//}

/// Structure representing main characteristics of a function needed
/// for the program using a plugin, which implements it
///
/// A Function object contains
/// * function name
/// * a [`DuskCallable`] implementor to be used when calling the
/// function
/// * its id number that identifies it's place in the main function
/// Vector
/// * vector of parameter descriptions of the parameters
/// ([`Parameter`])
/// * its return [`TypeId`]
/// * whether or not the arguments should be checked or just passed
/// as is
/// * a vector of plugin dependencies this function has
#[derive(Clone, Debug)]
pub struct Function {

    /// Function name, as a reference to a static string. Mainly
    /// used to give user the ability to choose the function they
    /// want to use
    pub name: String,

    /// The callable that should be used when calling the function
    pub callable: Box<dyn DuskCallable>,

    /// Function ID, used to find this function in the main plugin
    /// function vector
    ///
    /// **Should always be the same for same functions in the newer
    /// releases, unless a new plugin version is submitted**
    pub fn_id: usize,

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

    /// If the function can not work without some optional
    /// interplug requests fulfilled, they must be included in
    /// this field when providing the function to the
    /// program that is using the plugin, so it knows if this
    /// function is available in the current setup or not.
    pub dependencies: Vec<InterplugRequest>,
}

impl Default for Function {
    fn default () -> Function {
        Function {
            name: "".to_string(),
            callable : Box::new(EmptyCallable{}),
            fn_id: 0,
            parameters: Vec::new(),
            return_type: TypeId::of::<u8>(),
            no_check_args: false,
            dependencies: Vec::new(),
        }
    }
}
