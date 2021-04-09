// Plugin API used in Dusk project
//
// Copyright (C) 2021 by Andy Gozas <andy@gozas.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![deny(warnings)]

#![allow(unused_parens)]

#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(missing_copy_implementations)]
//#![warn(missing_debug_implementations)]
#![warn(variant_size_differences)]
#![warn(keyword_idents)]
#![warn(anonymous_parameters)]

#![warn(missing_abi)]

#![warn(meta_variable_misuse)]
#![warn(semicolon_in_expressions_from_macros)]
#![warn(absolute_paths_not_starting_with_crate)]

#![warn(missing_crate_level_docs)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(invalid_html_tags)]
#![warn(non_ascii_idents)]
#![warn(pointer_structural_match)]
#![warn(private_doc_tests)]
#![warn(single_use_lifetimes)]
#![warn(unaligned_references)]

//! Crate, that is used while building a plugin system as a common
//! dependency by both plugin and plugin user to define the plugin
//! behavior and safely import and use the plugin
//!
//! # Plugin Side
//!
//! To quickly learn how to create a plugin and export functions from it see
//! [`export_freight!`] macro documentation
//!
//! # Importer Side
//!
//! To quickly learn how to import and use plugins see [`FreightProxy`]
//! documentation

use std::any::{Any, TypeId};

pub mod changelog;

/// Api version parameter, passed from the build script.
///
/// For the program that uses the plugin to work correctly it
/// has to use the same version of api, which is ensured by embedding
/// it as a static variable
pub static API_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Rust compiler version parameter, passed from the compiler.
///
/// If plugin is compiled with a different rust compiler version
/// it may be incompatible with the program using it, so before
/// proceeding to use the plugin a version check is needed.
///
/// for this to work, build script should set this environmental
/// variable, which can be done like this
///
/// # build.rs
/// ```
/// extern crate rustc_version;
///
/// fn main() {
///     let version = rustc_version::version().unwrap();
///     println!("cargo:rustc-env=RUSTC_VERSION={}", version);
/// }
/// ```
///
/// # Cargo.toml
/// ```
/// [build-dependencies]
/// rustc_version = "0.3.0"
/// ```
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// A structure that holds a representation of plugin version
/// for easy comparing and storing.
///
/// The ordering is as follows
///
/// * Major
/// * Minor
/// * Release
/// * Build
///
/// e.g in 1.2.3.4, 1 is major, 2 is minor, 3 is release and 4 
/// is build
#[derive(Copy, Clone)]
pub struct Version {

    /// Major version number
    pub major: usize,

    /// Minor version number
    pub minor: usize,
    
    /// Release version number
    pub release: usize,

    /// Build version number
    pub build: usize,
}

impl Default for Version {
    fn default () -> Version {
        Version {
            major: 0,
            minor: 0,
            release: 0,
            build: 0,
        }
    }
}

/// A structure, exported by plugin, containing some package details
/// and register function
///
/// This structure contains the rust compiler version, the plugin was
/// compiled with, api version it uses, the plugin name and version
/// and the actual function, that is used to register the plugin.
///
/// The function is only needed to pass a structure, that implements
/// trait Freight to the [`FreightRegistrar::register_freight`] as
/// structures can not be put into static variables, but static
/// functions can.
///
/// This structure must only be built by [`export_freight!`] macro
/// in plugins. And its fields are only read by
/// [`FreightProxy::load`] function when loading the plugin
#[derive(Copy, Clone)]
pub struct FreightDeclaration {

    /// Rust compiler version as a static string
    pub rustc_version: &'static str,

    /// Api version as a static string
    pub api_version: &'static str,

    /// Version of the freight being imported
    pub freight_version: Version,
    
    /// The earliest plugin version, for which the code could have 
    /// been designed, and still be run with this version
    /// of plugin, with same results
    pub backwards_compat_version: Version,

    /// Name of the freight being imported
    pub name: &'static str,

    /// Function that gets a [`FreightRegistrar`] trait implementor
    /// as an argument and calls its freight_register function
    /// to provide unexportable things, such as structs, in
    /// particular, [`Freight`] implementor structures
    pub register: fn (&mut dyn FreightRegistrar),
}

/// A macro, which **MUST** be used for exporting a struct.
///
/// Every export must be done using this macro, or it's 
/// wrapper [`export_plugin`] to make sure the plugin is 
/// compatible with the program using it
///
/// To learn more about structure, required to register the
/// plugins behavior, see [`Freight`] trait documentation
///
/// To learn how to do the same job easier automatically
/// see [`register_freight!`] macro documentation and
/// [`export_plugin!`] macro documentation
///
/// # Example
/// ```
/// dusk_api::export_freight!(
///     "test",
///     Version {major: 1, minor: 23, ..Default::default() }, 
///     register);
///
/// pub fn register (registrar: &mut dyn FreightRegistrar) {
///     registrar.register_freight(Box::new(MyFreight));
/// }
///
/// pub struct MyFreight;
///
/// impl Freight for MyFreight {
///     // Your implementation here
/// }
/// ```
///
/// If you want to also specify the backwards compatibility
/// version, you should use the same macro with four 
/// arguments, inserting backwards compatibility version
/// right after the plugin version
///
/// # Example
/// ```
/// dusk_api::export_freight!(
///     "test",
///     Version {major: 1, minor: 23, ..Default::default() }, 
///     Version {major: 0, minor: 6, ..Default::default() }, 
///     register);
///
/// pub fn register (registrar: &mut dyn FreightRegistrar) {
///     registrar.register_freight(Box::new(MyFreight));
/// }
///
/// pub struct MyFreight;
///
/// impl Freight for MyFreight {
///     // Your implementation here
/// }
/// ```
#[macro_export]
macro_rules! export_freight {
    ($name:expr, $version:expr, $register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static freight_declaration: $crate::FreightDeclaration
            = $crate::FreightDeclaration {
                rustc_version: $crate::RUSTC_VERSION,
                api_version: $crate::API_VERSION,
                freight_version: $version,
                backwards_compat_version: $version,
                name: $name,
                register: $register,
            };
    };
    ($name:expr, $version:expr, $back_version:expr, $register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static freight_declaration: $crate::FreightDeclaration
            = $crate::FreightDeclaration {
                rustc_version: $crate::RUSTC_VERSION,
                api_version: $crate::API_VERSION,
                freight_version: $version,
                backwards_compat_version: $back_version,
                name: $name,
                register: $register,
            };
    };
}

/// A macro, which can be used to create a registry function
/// for your freight easier
///
/// # Example
///
/// ```
/// dusk_api::register_freight!(MyFreight, my_reg_fn);
/// dusk_api::export_freight!(
///     "test",
///     Version {major: 1, minor: 23, ..Default::default() }, 
///     Version {major: 0, minor: 6, ..Default::default() }, 
///     register);
///
/// pub struct MyFreight;
///
/// impl Freight for MyFreight {
///     // Your implementation here
/// }
/// ```
#[macro_export]
macro_rules! register_freight {
    ($freight: expr, $name: ident) => {
        #[doc(hidden)]
        pub fn $name (registrar: &mut dyn $crate::FreightRegistrar) {
            registrar.register_freight(Box::new($freight));
        }
    };
}

/// A macro, which can be used to make exporting of a struct
/// easier 
///
/// Can be used both with and without the backwards
/// compatibility version argument (for more info see
/// [`export_freight!`]
///
/// # Example
///
/// ```
/// dusk_api::export_plugin!(
///     "test",
///     Version {major: 1, minor: 23, ..Default::default() }, 
///     register);
///
/// pub struct MyFreight;
///
/// impl Freight for MyFreight {
///     // Your implementation here
/// }
/// ```
///
/// With backwards compatibility version:
///
/// # Example
///
/// ```
/// dusk_api::export_plugin!(
///     "test",
///     Version {major: 1, minor: 23, ..Default::default() }, 
///     Version {major: 0, minor: 6, ..Default::default() }, 
///     register);
///
/// pub struct MyFreight;
///
/// impl Freight for MyFreight {
///     // Your implementation here
/// }
/// ```
#[macro_export]
macro_rules! export_plugin {
    ($name: expr, $version: expr, $freight: ident) => {
        $crate::register_freight!($freight, freight_registry_function);
        $crate::export_freight!($name, $version, freight_registry_function);
    };
    ($name: expr, $version: expr, $back_version: expr, $freight: ident) => {
        $crate::register_freight!($freight, freight_registry_function);
        $crate::export_freight!($name, $version, $back_version, freight_registry_function);
    };
}

/// A macro, that makes plugin importing a little bit easier
///
/// # Safety
///
/// This macro is **UNSAFE** as it **CAN NOT** check whether
/// the plugin has beed exported correctly or not, and 
/// using it on a plugin that is in any way corrupted might
/// lead to segmentation fault or undefined behavior
///
/// # Example
///
/// ```
/// let mut my_f_proxy: FreightProxy =
///     import_plugin!("/bin/libtest-plug.so").unwrap();
///
/// println!("{}, {}", my_f_proxy.name, my_f_proxy.version);
/// let fnlist: Vec<Function> = my_f_proxy.get_function_list();
/// for func in fnlist {
///     println!("{}, {}", func.name, func.number);
/// }
/// ```
#[macro_export]
macro_rules! import_plugin {
    ($lib: expr) => {
        unsafe{
            $crate::FreightProxy::load($lib)
        }
    };
}

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
    ///     _args: Vec<&mut Box<dyn Any>>
    ///     ) -> Result<Box<dyn Any>, DuskError> {
    ///     Err(DuskError::RuntimeError (
    ///         "You can't call an empty freight"
    ///     ))
    /// }
    /// ```

    /// Plugin library loading failed
    LoadingError (libloading::Error),
    
    /// Plugin import failed
    ImportError (&'static str),

    /// An argument of wrong type received
    TypeError (&'static str),

    /// An argument of wrong value was received
    ValueError (&'static str),

    /// An OS error occured
    OsError (&'static str),

    /// At some point some value check failed
    AssertionError (&'static str),

    /// Index out of bounds
    IndexError (&'static str),

    /// Code is trying to divide by zero
    ZeroDivisionError (&'static str),

    /// Out of memory
    OverflowError (&'static str),

    /// Called function is not implemented
    NotImplementedError (&'static str),

    /// Other error occured during runtime
    RuntimeError (&'static str),
}

/// Enum, that represents an interplugin request and either contains
/// a [`InterplugRequest::Crucial`] plugin request (must be provided
/// in order for the plugin to work or an
/// [`InterplugRequest::Optional`] plugin request which may be
/// denied
///
/// For more complex situations, when several plugins might provide
/// similar functionality, [`InterplugRequest::Either`] may be used
/// to provide several requests, each of which may be fulfilled
/// for the plugin to work correctly. In case this  functionality
/// may also be provided by several different plugins together,
/// [`InterplugRequest::Each`] should be used.
///
/// If the request is optional, the final decision to provide it
/// or not to provide it is supposed to be made by the user. For
/// example, if user needs some function from a plugin, that
/// requires an optional interplug request to be fulfilled, they
/// just add it to the dependencies, so the program, that provides
/// the dependencies, when seeing this request finds out that
/// the plugin that was requested was already loaded earlier,
/// so it might as well provide it to the requesting plugin.
#[derive(Clone)]
pub enum InterplugRequest {

    /// Request for a specific plugin with a specific version
    /// and name, and make sure the functions with ids provided
    /// have all dependencies fulfilled
    PlugRequest {

        /// The string, that identifies the plugin
        plugin: &'static str,

        /// The list of function IDs that need their dependencies
        /// fulfilled
        fn_ids: Vec<usize>,

        /// The plugin version, with which the actuall version
        /// has to at least be compatible
        version: Version,
    },

    /// Request for any implementor of a specific trait from
    /// a specific plugin with a specific version, and make 
    /// sure the functions with ids provided have all 
    /// dependencies fulfilled (function IDs are local trait 
    /// IDs -- not global IDs)
    TraitRequest {

        /// String that identifies the plugin, conataining the 
        /// trait definition
        plugin: &'static str,

        /// Trait name
        trait_name: &'static str,

        /// In trait function IDs of the functions that need 
        /// their dependencies fulfilled
        fn_ids: Vec<usize>,

        /// The version of the plugin containing the trait 
        /// definition
        version: Version,
    },

    /// Request for a specific plugin with a specific version
    /// and name, and make sure all of it's functions have
    /// their dependencies fulfilled
    PlugRequestAll {
        
        /// The string, that identifies the plugin
        plugin: &'static str,

        /// The plugin version, with which the actuall version
        /// has to at least be compatible
        version: Version,
    },

    /// Request for any implementor of a specific trait from
    /// a specific plugin with a specific version, and make 
    /// sure all of it's functions have their dependencies 
    /// fulfilled
    TraitRequestAll {

        /// String that identifies the plugin, conataining the 
        /// trait definition
        plugin: &'static str,

        /// Trait name
        trait_name: &'static str,

        /// The version of the plugin containing the trait 
        /// definition
        version: Version,
    },

    /// An interlplug request that contains several interlplug
    /// requests, either of which *MUST* be fulfilled for the
    /// plugin to work at all
    Either {
        
        /// A vector of the requests, either of which has to 
        /// be fulfilled
        requests: Vec<InterplugRequest>,
    },

    /// An interplug request that contains several interplug
    /// requests, each of which *MUST* be fulfilled in order for
    /// the plugin to work
    Each {

        /// A vector of the requests, all of which have to 
        /// be fulfilled
        requests: Vec<InterplugRequest>,
    },

    /// An interplug request that *MUST* be fulfilled in order
    /// for the plugin to work at all
    Crucial {

        /// A box, containing the actual request
        request: Box<InterplugRequest>,
    },

    /// An interplug request that must be fulfilled in order for
    /// the plugin to fully work, which means that without it
    /// some functions will be unavailable
    Optional {

        /// A box, containing the actual request
        request: Box<InterplugRequest>,
    },
}

/// Enum that represents a system limitation, that a plugin either
/// needs to know to work correctly, or should be notified of in
/// case main program wants to limit some settings
///
/// When initiating the plugin, using [`Freight::init`], a vector
/// of limitations can be passed to the plugin, to set such limits
/// as number of cpu threads, memory working directories, etc.
/// If for example the main program started to do some multithreading
/// itself, it may notify the plugin using [`Freight::update_limitations`]
/// that the maximum amount of threads it can use was lowered from
/// the previous amount to 1, or if the main program does not care
/// about the amount of threads anymore, and lets the plugin decide
/// by itself which amount it wants to use, it can send a
/// [`Limitation::Reset`] to it.
#[derive(Copy, Clone)]
pub enum Limitation {

    /// Set the maximum allowed number, represetting some setting
    Top {

        /// The name of the setting
        setting: &'static str,

        /// The value to which we want to set it
        limit: isize,
    },

    /// Set the minimum allowed number, representing some setting
    Bottom {

        /// The name of the setting
        setting: &'static str,

        /// The value to which we want to set it
        limit: isize,
    },

    /// Reset the setting to default value (as if the main program
    /// has never set any value to the setting at all)
    Reset {

        /// The name of the setting
        setting: &'static str,
    },
}

/// A structure, that contains all information, the compiler needs to
/// know about function parameters.
///
/// The only required field is arg_type, but there are some optional
/// fields you might want to use.
#[derive(Clone)]
pub struct Parameter {

    /// The argument type, as it's [`TypeId`]
    ///
    /// Always required
    pub arg_type: TypeId,

    /// Argument can be of any type, don't check TypeId
    ///
    /// Default: false
    pub any_type: bool,

    /// If any type is set to true, check if the type
    /// has some traits implemented
    pub implements: Option<Vec<InterplugRequest>>,

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
    pub keyword: Option<&'static str>,

    /// If your keyword argument is optional, you can set
    /// it's default value to Some
    ///
    /// Default value is [`None`]
    pub default_value: Option<&'static Box<dyn Any>>,

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
pub struct Kwarg {

    /// The keyword
    pub keyword: &'static str,

    /// The actual argument value
    pub value: &'static mut Box<dyn Any>,
}

/// A trait that defines the behavior of a function wrapper, used
/// to call functions imported from plugins
pub trait DuskCallable: CallableClone {
    
    /// The function that takes arguments, processes them and any
    /// data that is stored in the implementor struct and calls
    /// the underlying function, returning it's result
    fn call (
        self: &mut Self,
        args: &Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, DuskError>;
}

/// The trait, containing a function that clones the existing
/// DuskCallable implementor
pub trait CallableClone {

    /// The function that returns a new box, of [`DuskCallable`]
    /// implementor
    fn clone_box (self: &Self) -> Box<dyn DuskCallable>;
}

impl <T> CallableClone for T
where
    T: 'static + DuskCallable + Clone
{
    fn clone_box (self: &Self) -> Box<dyn DuskCallable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DuskCallable> {
    fn clone (self: &Self) -> Box<dyn DuskCallable> {
        self.clone_box()
    }
}

/// Simplest Dusk callable implementor -- only contains one function
/// that gets the argument vector as an argument and simply passes
/// the arguments and returned Result
#[derive(Copy, Clone)]
pub struct SimpleCallable {
    underlying_fn: 
        fn (&Vec<&mut Box<dyn Any>>) 
            -> Result<Box<dyn Any>, DuskError>,
}

impl DuskCallable for SimpleCallable {
    fn call (
        self: &mut Self,
        args: &Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, DuskError> {

        (self.underlying_fn)(args)
    }
}

/// Dusk callable that not only holds a function pointer, but
/// also a vector of args to pass to the underlying function
/// as well as the arguments provided to the call function
#[derive(Copy, Clone)]
pub struct ConstArgsCallable {
    const_args: &'static Vec<&'static Box<dyn Any>>,
    underlying_fn: 
        fn (
            &Vec<&Box<dyn Any>>, 
            &Vec<&mut Box<dyn Any>>,
        ) -> Result<Box<dyn Any>, DuskError>,
}

impl DuskCallable for ConstArgsCallable {
    fn call (
        self: &mut Self,
        args: &Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, DuskError> {

        (self.underlying_fn)(&self.const_args.clone(), args)
    }
}

/// A default callable: does not call anything, always returns
/// [`DuskError::NotImplementedError`]
#[derive(Copy, Clone)]
pub struct EmptyCallable;

impl DuskCallable for EmptyCallable {
    fn call (
        self: &mut Self,
        _args: &Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, DuskError> {

        Err(DuskError::NotImplementedError (
            "Called function is not implemented"
        ))
    }
}


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
#[derive(Clone)]
pub struct TraitFunctionDefinition {

    /// Function name, as a reference to a static string. Mainly
    /// used to give user the ability to choose the function they
    /// want to use
    pub name: &'static str,

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
    /// from a [`Box<dyn Any>`]
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
            name: "",
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
#[derive(Clone)]
pub struct Function {

    /// Function name, as a reference to a static string. Mainly
    /// used to give user the ability to choose the function they
    /// want to use
    pub name: &'static str,

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
    /// from a [`Box<dyn Any>`]
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
            name: "",
            callable : Box::new(EmptyCallable{}),
            fn_id: 0,
            parameters: Vec::new(),
            return_type: TypeId::of::<u8>(),
            no_check_args: false,
            dependencies: Vec::new(),
        }
    }
}

/// Structure representing main characteristics of a function needed
/// for the program using a plugin, which implements it
///
/// A TraitFunction object contains
/// * its number in trait
/// * the underlying function
#[derive(Clone)]
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
#[derive(Clone)]
pub struct TraitDefinition {

    /// The trait's name
    pub name: &'static str,

    /// The method definitions
    pub methods: Vec<TraitFunctionDefinition>,
}

/// A trait implementation, that contains name of the trait
/// being implemented and a vector of the trait method
/// implementations
#[derive(Clone)]
pub struct TraitImplementation {

    /// Trait name (containing full path to it in the plugin
    /// where it came from)
    pub name: &'static str,

    /// Methods being implemented
    pub methods: Vec<TraitFunction>,
}

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
#[derive(Clone)]
pub struct Type {

    /// Name for the [`TypeId`] owner to be reffered to as a static
    /// string
    pub name: &'static str,

    /// The **INTERNAL** id for the type, representing the position
    /// of the type in the type vector, **NOT** the native [`TypeId`]**
    pub tp_id: usize,

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
            name: "",
            tp_id: 0,
            methods: Vec::new(),
            fields: Vec::new(),
            trait_implementations: Vec::new(),
            native_id: TypeId::of::<u8>(),
        }
    }
}

/// A structure, that defines a module and may or may not contain
/// types, functions, submodules, trait definitions and 
/// constants
#[derive(Clone)]
pub struct Module {

    /// The module name
    pub name: &'static str,

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

/// TODO: trait proxy not perfect for the cause yet
#[derive(Clone)]
pub struct TraitProxy {
    
    /// The name of the trait
    pub trait_name: &'static str,

    /// The plugin where it came from
    pub freight_proxy: std::rc::Rc<FreightProxy>,

    /// The vector, linking IDs of the Trait functions to the actual
    /// general plugin function IDs
    pub function_links: Vec<usize>,
}

/// Trait, that defines the plugin behavior
///
/// This trait must be implemented in plugins to allow the program,
/// that uses them to call any internal function of choice. For that
/// the trait has a method [`Freight::get_function_list`], that
/// provides argument types and return types, actually being used
/// under Any trait as well as the function name to refer to it and
/// its identification number, which is needed to call this function
///
/// # Example
/// TODO
pub trait Freight {

    /// Function that is ran when importing the plugin, which
    /// may be reimplememented in a plugin if it needs to set up
    /// some things before doing any other actions
    ///
    /// If some system limitations should be applied, they must be
    /// included as an argument. If the plugin needs to use other
    /// plugins, it should request them as a Vector of
    /// [`InterplugRequest`]
    fn init (self: &mut Self, _limitations: &Option<Vec<Limitation>>)
        -> Vec<InterplugRequest> {
        Vec::new()
    }

    /// Function that updates system limitations
    fn update_limitations (self: &mut Self, _limitations: &Vec<Limitation>) {
        ()
    }

    /// Function that replies to the interplugin request by
    /// providing the requested plugin
    fn interplug_provide (
        self: &mut Self,
        _request: InterplugRequest,
        _freight_proxy: std::rc::Rc<FreightProxy>,
        ) {}

    /// Function that replies to the interplugin request by
    /// by informing it that the request was denied
    fn interplug_deny (
        self: &mut Self,
        _request: InterplugRequest,
        ) {}

    /// The function that is used to provide the main module / 
    /// modules of the plugin. Any function, constant or type
    /// are defined inside those modules
    fn get_module_list (self: &mut Self) -> Vec<Module>;
    
    /// The function that is used to proveid the functions that
    /// implement all the binary operators this plugin provides
    fn get_operator_list (self: &mut Self) -> Vec<Function> {
        Vec::new()
    }

    /// The function has to provide a vector of **ALL** functions
    /// that this plugin holds **PLACED IN SUCH WAY THAT ID IS
    /// EQUAL TO THE POSITION IN THE VECTOR** 
    ///
    /// **DO NOT REIMPLEMENT IT UNLESS YOU KNOW WHAT YOU ARE 
    /// DOING**
    ///
    /// This vector should contain **ALL** functions from **ALL**
    /// modules **AND ALL OF THEIR SUBMODULES**, including **ALL
    /// OF THE TYPE METHODS AND FIELD FUNCTIONS**, 
    /// **ALL FUNCTIONS USED FOR CONSTANTS** and including 
    /// **ALL OF THE BINARY OPERATOR FUNCTIONS**
    fn get_function_list (self: &mut Self) -> Vec<Function> {
        let all_modules: Vec<Module> = self.get_module_list();
        let mut parents: Vec<Module>;
        let mut par_progress: Vec<usize>;
        let mut result: Vec<Function> = Vec::new();
        for module in all_modules {
            parents = Vec::new();
            par_progress = Vec::new();
            parents.push(module.clone());
            par_progress.push(0);
            'par: while parents.len() > 0 {
                if (*par_progress.last().unwrap() < 
                    parents.last().unwrap().submodules.len()) {

                    parents.push(parents.last().unwrap().submodules[
                        *par_progress.last().unwrap()].clone());
                    *par_progress.last_mut().unwrap() += 1;
                    par_progress.push(0);
                    continue 'par;
                }
                par_progress.pop();
                'fun: for def_fun in &parents.last().unwrap().functions {
                    if def_fun.fn_id < result.len() {
                        result[def_fun.fn_id] = def_fun.clone();
                        continue 'fun;
                    }
                    for _i in result.len()..def_fun.fn_id {
                        result.push(Default::default());
                    }
                    result.push(def_fun.clone());
                }
                'con: for def_fun in &parents.last().unwrap().constants {
                    if def_fun.fn_id < result.len() {
                        result[def_fun.fn_id] = def_fun.clone();
                        continue 'con;
                    }
                    for _i in result.len()..def_fun.fn_id {
                        result.push(Default::default());
                    }
                    result.push(def_fun.clone());
                }
                for def_type in parents.pop().unwrap().types {
                    'met: for def_met in def_type.methods {
                        if def_met.fn_id < result.len() {
                            result[def_met.fn_id] = def_met.clone();
                            continue 'met;
                        }
                        for _i in result.len()..def_met.fn_id {
                            result.push(Default::default());
                        }
                        result.push(def_met.clone());
                    }
                    'fil: for def_fil in def_type.fields {
                        if def_fil.fn_id < result.len() {
                            result[def_fil.fn_id] = def_fil.clone();
                            continue 'fil;
                        }
                        for _i in result.len()..def_fil.fn_id {
                            result.push(Default::default());
                        }
                        result.push(def_fil.clone());
                    }
                    for def_trt in def_type.trait_implementations {
                        'trtmet: for def_met in def_trt.methods {
                            if def_met.function.fn_id < result.len() {
                                result[def_met.function.fn_id] = def_met.function.clone();
                                continue 'trtmet;
                            }
                            for _i in result.len()..def_met.function.fn_id {
                                result.push(Default::default());
                            }
                            result.push(def_met.function.clone());
                        }
                    }
                }
            }
        }
        return result;
    }
    
    /// The function has to provide a vector of **ALL** types
    /// that this plugin holds **PLACED IN SUCH WAY THAT ID IS
    /// EQUAL TO THE POSITION IN THE VECTOR** 
    ///
    /// **DO NOT REIMPLEMENT IT UNLESS YOU KNOW WHAT YOU ARE 
    /// DOING**
    ///
    /// This vector should contain **ALL** types from **ALL**
    /// modules **AND ALL OF THEIR SUBMODULES**
    fn get_type_list (self: &mut Self) -> Vec<Type> {
        let all_modules: Vec<Module> = self.get_module_list();
        let mut parents: Vec<Module>;
        let mut par_progress: Vec<usize>;
        let mut result: Vec<Type> = Vec::new();
        for module in all_modules {
            parents = Vec::new();
            par_progress = Vec::new();
            parents.push(module.clone());
            par_progress.push(0);
            'par: while parents.len() > 0 {
                if (*par_progress.last().unwrap() < 
                    parents.last().unwrap().submodules.len()) {

                    parents.push(parents.last().unwrap().submodules[
                        *par_progress.last().unwrap()].clone());
                    *par_progress.last_mut().unwrap() += 1;
                    par_progress.push(0);
                    continue 'par;
                }
                par_progress.pop();
                'typ: for def_type in parents.pop().unwrap().types {
                    if def_type.tp_id < result.len() {
                        result[def_type.tp_id] = def_type.clone();
                        continue 'typ;
                    }
                    for _i in result.len()..def_type.tp_id {
                        result.push(Default::default());
                    }
                    result.push(def_type.clone());
                }
            }
        }
        return result;
    }
}

/// Structure representing an empty [`Freight`] implementor, needed
/// only for [`FreightProxy`] configuration
#[derive(Copy, Clone)]
pub struct EmptyFreight;
impl Freight for EmptyFreight {
    fn get_module_list (self: &mut Self) -> Vec<Module> {
        Vec::new()
    }
}

/// Trait to be implemented on structs, which are used to register
/// or store the imported plugins
///
/// This trait is only needed for internal usage and as a reference
/// for the plugins, so that they can define a function that takes a
/// [`FreightRegistrar`] implementor as an argument, so that when
/// the plugin is imported the function is called on it and
/// some unexportable things such as structures could be provided,
/// which in this particular case is a [`Freight`] implementor
/// structure
pub trait FreightRegistrar {

    /// Function that gets a [`Freight`] implementor passed as an
    /// argument and is used to use it wherever it is needed in the
    /// [`FreightRegistrar`] implementor
    fn register_freight (
        self: &mut Self,
        freight: Box<dyn Freight>,
        );
}

/// A structure, that contains a Freight object and is used to import
/// and use it safely
///
/// This structure is a [`Freight`] trait implementor and
/// [`FreightRegistrar`] trait implementor. It provides
/// [`FreightProxy::load`] function that is used to build the
/// [`FreightProxy`] from a library path
///
/// To learn more about the functions you may call on the
/// [`FreightProxy`], see [`Freight`] trait documentation
///
/// # Example
///
/// ```
/// let mut my_f_proxy: FreightProxy = unsafe{
///     FreightProxy::load("/bin/libtest_plug.so").expect("fail")
/// };
/// println!("{}, {}", my_f_proxy.name, my_f_proxy.version);
/// let fnlist: Vec<Function> = my_f_proxy.get_function_list();
/// for func in fnlist {
///     println!("{}, {}", func.name, func.number);
/// }
/// ```
pub struct FreightProxy {

    /// Imported freight, solely for internal purposes
    pub freight: Box<dyn Freight>,

    /// Lib this freight was imported from to make sure this
    /// structure does not outlive the library it was imported from
    pub lib: std::rc::Rc<libloading::Library>,

    /// Imported freights name as a static string
    pub name: &'static str,

    /// Imported freights version 
    pub version: Version,

    /// The earliest version, for which the code was designed, this
    /// code can safely be run with the new plugin version
    pub backwards_compat_version: Version,
}

/// Functions, needed to configure [`FreightProxy`] structure
/// initially
impl FreightProxy {

    /// Function, used to build a [`FreightProxy`] object from a
    /// library path
    pub unsafe fn load (lib_path: &str)
        -> Result<FreightProxy, DuskError> {

        // Import the library
        let lib : std::rc::Rc<libloading::Library>;
        match libloading::Library::new(lib_path) {
            Ok(library) => lib = std::rc::Rc::new(library),
            Err(lib_err) => return(Err(DuskError::LoadingError (lib_err))),
        }

        // Get the plugin declaration structure from this lib
        let declaration: FreightDeclaration;
        match lib.get::<*mut FreightDeclaration>(
            b"freight_declaration\0") {

            Ok(decl) => declaration = decl.read(),
            Err(lib_err) => return(Err(DuskError::LoadingError (lib_err))),
        }

        // Check if the compiler and api versions match
        // If not -- immediately return an error
        if declaration.rustc_version != RUSTC_VERSION {
            return Err(DuskError::ImportError (
                "Compiler version mismatch"
            ));
        }

        if declaration.api_version != API_VERSION {
            return Err(DuskError::ImportError (
                "Dusk API version mismatch"
            ));
        }

        // Make a new FreightProxy with all values that are
        // already available
        let mut result: FreightProxy = FreightProxy {
            freight: Box::new(EmptyFreight{}),
            lib: lib,
            name: declaration.name,
            version: declaration.freight_version,
            backwards_compat_version: 
                declaration.backwards_compat_version,
        };

        // Call the function, imported in the plugin declaration
        // and pass the FreightProxy to it as an argument
        // so it sets the internal freight variable to a
        // correct value
        (declaration.register)(&mut result);

        // Return the result
        Ok(result)
    }
}

// Implementation of trait Freight for the proxy structure, so we
// can call exact same functions from it
impl Freight for FreightProxy {

    // Proxy function, that calls the internal freights init function
    // and returns its plugin dependencies
    fn init (self: &mut Self, limitations: &Option<Vec<Limitation>>)
        -> Vec<InterplugRequest> {
        self.freight.init(limitations)
    }

    // Proxy function that takes the list of new system limitations
    // and passes it to the plugin
    fn update_limitations (self: &mut Self, limitations: &Vec<Limitation>) {
        self.freight.update_limitations(limitations)
    }

    // Proxy function for replying to an interplugin dependency
    // request by providing the requested plugin
    fn interplug_provide (
        self: &mut Self,
        request: InterplugRequest,
        freight_proxy: std::rc::Rc<FreightProxy>,
        ) {
        self.freight.interplug_provide(request, freight_proxy);
    }

    // Proxy function for replying to an interplugin dependency
    // request by informing it of request denial
    fn interplug_deny (
        self: &mut Self,
        request: InterplugRequest,
        ) {
        self.freight.interplug_deny(request);
    }

    fn get_module_list (self: &mut Self) -> Vec<Module> {
        self.freight.get_module_list()
    }
}

// Implementation of FreightRegistrar trait for the proxy
// structure, so that we can call register function on it without
// any third party structure
impl FreightRegistrar for FreightProxy {

    // The function that simply takes a freight implementor
    // as an argument and passes it into the inside freight
    // variable
    fn register_freight (
        self: &mut Self,
        freight: Box<dyn Freight>,
        ) {
        self.freight = freight;
    }
}
