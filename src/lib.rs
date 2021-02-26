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

/// A macro, which can be used to make exporting of a struct
/// easier
/// FIXME
///
/// # Example
///
/// ```
/// dusk_api::export_plugin!("test", "0.1.0", MyFreight);
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

/// A macro, which must be used for exporting a struct.
///
/// Every export must be done using this macro, to make sure
/// the plugin is compatible with the program using it
///
/// To learn more about structure, required to register the
/// plugins behavior, see [`Freight`] trait documentation
///
/// To learn how to do the same job easier automatically
/// see [`register_freight!`] macro documentation and
/// [`export_plugin!`] macro documentation
/// FIXME
///
/// # Example
/// ```
/// dusk_api::export_freight!("test", "0.1.0", register);
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
/// easier
///
/// # Example
///
///FIXME
/// ```
/// dusk_api::register_freight!(MyFreight, my_reg_fn);
/// dusk_api::export_freight!("test", "0.1.0", my_reg_fn);
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

/// A macro, that makes plugin importing a little bit easier
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
pub enum RuntimeError {

    /// Send a message, representing the runtime error, that occured
    /// while using a function.
    ///
    /// # Example
    /// ```
    /// fn call_function (
    ///     self: &mut Self,
    ///     _function_number: u64,
    ///     _args: Vec<&mut Box<dyn Any>>
    ///     ) -> Result<Box<dyn Any>, RuntimeError> {
    ///     Err(RuntimeError::Message{
    ///         msg: "You can't call an empty freight"
    ///     })
    /// }
    /// ```
    Message { msg: &'static str },
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
/// FIXME
pub enum InterplugRequest {

    /// An interplug request that *MUST* be fulfilled in order
    /// for the plugin to work at all
    Crucial {
        plugin: &'static str,
        version: Version,
    },

    /// An interplug request that must be fulfilled in order for
    /// the plugin to fully work, which means that without it
    /// some functions will be unavailable
    Optional {
        plugin: &'static str,
        version: Version,
    },

    /// An interlplug request that contains several interlplug
    /// requests, either of which *MUST* be fulfilled for the
    /// plugin to work at all
    Either {
        requests: Vec<InterplugRequest>,
    },

    /// An interlplug request that contains several interplug
    /// requests, either of which should be fulfilled for the
    /// plugin to fully work.
    OptionalEither {
        requests: Vec<InterplugRequest>,
    },

    /// An interplug request that contains several interplug
    /// requests, each of which *MUST* be fulfilled in order for
    /// the plugin to work
    Each {
        requests: Vec<InterplugRequest>,
    },

    /// An interplug request that contains several interplug
    /// requests, each of which should be fulfilled in for
    /// the plugin to fully work
    OptionalEach {
        requests: Vec<InterplugRequest>,
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
pub enum Limitation {

    /// Set the maximum allowed number, represetting some setting
    Top {
        setting: &'static str,
        limit: isize,
    },

    /// Set the minimum allowed number, representing some setting
    Bottom {
        setting: &'static str,
        limit: isize,
    },

    /// Reset the setting to default value (as if the main program
    /// has never set any value to the setting at all)
    Reset {
        setting: &'static str,
    },
}

/// Structure representing main characteristics of an object type
/// needed for the program, using the plugin, that either imports
/// or defines this type in case this type is not present in
/// the user program itself
///
/// A Type object contains
/// * type name, used for identifying this type
/// * its [`TypeId`] for Any trait to work properly
pub struct Type {

    /// Name for the [`TypeId`] owner to be reffered to as a static
    /// string
    pub name: &'static str,

    /// If an object of this type should have some functions, that
    /// can be called on it, they should be provided here. The 
    /// functions provided here should be called using the same
    /// [`Freight::call_function`] function, so they should
    /// all have unique numbers
    pub methods : Option<Vec<Function>>,

    /// All fields of an object of this type, user needs to be able
    /// to access, should be located here. The field name then
    /// will be the function name, functions return type is the 
    /// field type and the only argument of the function should 
    /// be of the type, the field is owned by. These functions
    /// are once again called by the same [`Freight::call_function`]
    /// function and should all have unique numbers over all functions
    /// called by [`Freight::call_function`]
    pub fields : Option<Vec<Function>>,

    /// [`TypeId`] object, gotten from the structure, being
    /// provided to the program, that is using the plugin
    ///
    /// See [`std::any::TypeId`] documentation to find out how
    /// to get a type id of a type
    pub type_id: TypeId,
}

///FIXME
pub struct Parameter {

    pub arg_type: TypeId,

    pub keyword: Option<&'static str>,

    pub default_value: Option<Box<dyn Any>>,

    pub optional: bool,

    pub mutable: bool,
}

impl Default for Parameter {//FIXME
    fn default () -> Parameter {
        Parameter {
            arg_type: TypeId::of::<u8>(),
            keyword: None,
            default_value: None,
            optional: false,
            mutable: false,
        }
    }
}

pub struct Kwarg {

    pub keyword: &'static str,

    pub value: Box<dyn Any>,
}

/// Structure representing main characteristics of a function needed
/// for the program using a plugin, which implements it
///
/// A Function object contains
/// * function name
/// * its id number to be used when calling the function
/// FIXME
/// * argument [`TypeId`]s it takes
/// * its return [`TypeId`]
pub struct Function {

    /// Function name, as a reference to a static string. Mainly
    /// used to give user the ability to choose the function they
    /// want to use
    pub name: &'static str,

    /// Function ID, used to call this function
    ///
    /// **Should always be the same for same functions in the newer
    /// releases, unless a new plugin version is submitted**
    pub number: u64,

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
    pub dependencies: Option<Vec<InterplugRequest>>,
}

impl Default for Function {
    fn default () -> Function {
        Function {
            name: "",
            number: 0,
            parameters: Vec::new(),
            return_type: TypeId::of::<u8>(),
            no_check_args: false,
            dependencies: None,
        }
    }
}

pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub release: usize,
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

/// Trait, that defines the plugin behavior
///
/// This trait must be implemented in plugins to allow the program,
/// that uses them to call any internal function of choice. For that
/// the trait has a method [`Freight::get_function_list`], that
/// provides argument types and return types, actually being used
/// under Any trait as well as the function name to refer to it and
/// its identification number, which is needed to call this function
///
/// The [`Freight::call_function`] method actually calls the function,
/// which matches the provided id.
///
/// # Example
/// ```
/// pub struct Test;
///
/// impl Freight for Test {
///     fn call_function (
///         self: &mut Self,
///         function_number: u64,
///         mut args: Vec<&mut Box<dyn Any>>
///         ) -> Result<Box<dyn Any>, RuntimeError> {
///
///         match function_number {
///             0 => return Ok(Box::new(
///                     args[0].downcast_mut::<String>()
///                         .unwrap()
///                         .clone())
///                 ),
///             _ => return Err(RuntimeError::Message{
///                     msg: "bad fn number"
///                 }),
///         }
///     }
///
///     fn get_function_list (self: &mut Self) -> Vec<Function> {
///         let mut result: Vec<Function> = Vec::new();
///         result.push(Function{
///             name: "copy",
///             number: 0,
///             arg_types: vec![TypeId::of::<String>()],
///             return_type: TypeId::of::<String>(),
///         });
///         return result;
///     }
///
///     fn get_type_list (self: &mut Self) -> Vec<Type> {
///         return Vec::new();
///     }
/// }
/// ```
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

    /// Function that is used to provide information about
    /// non standard types, a function from this plugin might take
    /// as an argument or return, so that the program using the
    /// plugin can take such non-standard objects from one
    /// function implemented in this plugin and pass it on into
    /// another function in this plugin
    ///
    /// To use it, just reimplement it to return a vector of
    /// such [`Type`] structure descriptions
    fn get_type_list (self: &mut Self) -> Vec<Type> {
        Vec::new()
    }

    /// Function that is used to provide information about internal
    /// functions of a plugin to the program using it, so it can
    /// choose the function it needs either by its name, argument
    /// types, return type or all of the above
    fn get_function_list (self: &mut Self) -> Vec<Function>;
    
    /// Function that is used to provide information about internal
    /// functions of a plugin that are named after binary operators
    /// and should be treated as such. These functions have to 
    /// always get exactly two arguments and they are called by the
    /// same function that calls any function [`Freight::call_function`]
    fn get_operator_list (self: &mut Self) -> Vec<Function> {
        Vec::new()
    }
    
    //FIXME
    fn get_backwards_compatibility (self: &mut Self) -> Option<Version> {
        None
    }

    /// Function that is used to call proxy the calls from the
    /// outside of a plugin to the internal functions and must
    /// implement function calling, by its number arguments,
    /// contained inside of [`Vec<Box<dyn Any>>`] and must return
    /// either a [`Box<dyn Any>`] representing  the returned value
    /// or a [`RuntimeError`]
    fn call_function (
        self: &mut Self,
        function_number: u64,
        args: Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, RuntimeError>;
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
pub struct FreightDeclaration {

    /// Rust compiler version as a static string
    pub rustc_version: &'static str,

    /// Api version as a static string
    pub api_version: &'static str,

    /// Version of the freight being imported
    pub freight_version: Version,
    
    /// The earliest version, for which the code was designed, this
    /// code can safely be run with the new plugin version
    pub backwards_compat_version: Version,

    /// Name of the freight being imported
    pub name: &'static str,

    /// Function that gets a [`FreightRegistrar`] trait implementor
    /// as an argument and calls its freight_register function
    /// to provide unexportable things, such as structs, in
    /// particular, [`Freight`] implementor structures
    pub register: fn (&mut dyn FreightRegistrar),
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

/// Structure representing an empty [`Freight`] implementor, needed
/// only for [`FreightProxy`] configuration
pub struct EmptyFreight;
impl Freight for EmptyFreight {
    fn call_function (
        self: &mut Self,
        _function_number: u64,
        _args: Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, RuntimeError> {

        Err(RuntimeError::Message{
            msg: "You can't call an empty freight"
        })
    }

    fn get_function_list (self: &mut Self) -> Vec<Function> {
        Vec::new()
    }
}

/// Functions, needed to configure [`FreightProxy`] structure
/// initially
impl FreightProxy {

    /// Function, used to build a [`FreightProxy`] object from a
    /// library path
    pub unsafe fn load (lib_path: &str)
        -> Result<FreightProxy, RuntimeError> {

        // Import the library
        // *FIXME* Get rid of unwrap
        let lib : std::rc::Rc<libloading::Library>
            = std::rc::Rc::new(
            libloading::Library::new(lib_path).unwrap());

        // Get the plugin declaration structure from this lib
        // *FIXME* Get rid of unwrap
        let declaration: FreightDeclaration = lib
            .get::<*mut FreightDeclaration>(b"freight_declaration\0")
            .unwrap()
            .read();

        // Check if the compiler and api versions match
        // If not -- immediately return an error
        if declaration.rustc_version != RUSTC_VERSION
            || declaration.api_version != API_VERSION
        {
            return Err(RuntimeError::Message{
                msg: "Version mismatch"
            });
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

    // Proxy function, that takes everything needed to call a function
    // and passes it on to the freight inside
    fn call_function (
        self: &mut Self,
        function_number: u64,
        args: Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, RuntimeError> {
        self.freight.call_function(function_number, args)
    }

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

    // Proxy function, that calls the function that gets function
    // list from the inside freight and returns the result
    fn get_function_list (self: &mut Self) -> Vec<Function> {
        self.freight.get_function_list()
    }

    // Proxy function, that calls the function that gets the backwards
    // compatibility version limit
    fn get_backwards_compatibility (self: &mut Self) -> Option<Version> {
        self.freight.get_backwards_compatibility()
    }

    // Proxy function, that calls the function that gets type
    // list from the inside freight and returns the result
    fn get_type_list (self: &mut Self) -> Vec<Type> {
        self.freight.get_type_list()
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
