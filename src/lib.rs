/// Crate, that is used while building a plugin system as a common
/// dependency by both plugin and plugin user to define the plugin
/// behavior and safely import and use the plugin
///
/// To learn how to create a plugin and export functions from it see
/// [`export_freight!`] macro documentation
///
/// To learn how to import and use plugins see [`FreightProxy`]
/// documentation

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

/// A macro, which must be used for exporting a struct.
///
/// Every export must be done using this macro, to make sure
/// the plugin is compatible with the program using it
///
/// To learn more about structure, required to register the
/// plugin's behavior, see [`Freight`] trait documentation
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
                name: $name,
                register: $register,
            };
    };
}

/// Enum, representing a message passed to the program using the
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

/// Structure representing main characteristics of a function needed
/// for the program using a plugin, which implements it
///
/// A Function object contains
/// * function name
/// * it's id number to be used when calling the function
/// * argument [`TypeId`]s it takes
/// * it's return [`TypeId`]
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

    /// [`TypeId`]s of arguments, this function expects to find
    /// inside of a Vector of [`Any`] trait implementors
    ///
    /// See [`std::any::Any`] documentation to find out more about
    /// storing an [`Any`] trait implementor and getting back
    /// from a [`Box<dyn Any>`]
    pub arg_types: Vec<TypeId>,

    /// The [`TypeId`] of the returned [`Any`] trait implementor
    ///
    /// See [`std::any::Any`] documentation to find out more about
    /// storing an [`Any`] trait implementor and getting back
    /// from a [`Box<dyn Any>`]
    pub return_type: TypeId,
}

/// Structure representing main characteristics of an object type
/// needed for the program, using the plugin, that either imports
/// or defines this type in case this type is not present in
/// the user program itself
///
/// A Type object contains
/// * type name, used for identifying this type
/// * it's [`TypeId`] for Any trait to work properly
pub struct Type {

    /// Name for the [`TypeId`] owner to be reffered to as a static
    /// string
    pub name: &'static str,

    /// [`TypeId`] object, gotten from the structure, being
    /// provided to the program, that is using the plugin
    ///
    /// See [`std::any::TypeId`] documentation to find out how
    /// to get a type id of a type
    pub type_id: TypeId,
}

/// Trait, that defines the plugin behavior
///
/// This trait must be implemented in plugins to allow the program,
/// that uses them to call any internal function of choice. For that
/// the trait has a method [`Freight::get_function_list`], that
/// provides argument types and return types, actually being used
/// under Any trait as well as the function name to refer to it and
/// it's identification number, which is needed to call this function
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

    /// Function that is used to call proxy the calls from the
    /// outside of a plugin to the internal functions and must
    /// implement function calling, by it's number arguments,
    /// contained inside of [`Vec<Box<dyn Any>>`] and must return
    /// either a [`Box<dyn Any>`] representing  the returned value
    /// or a [`RuntimeError`]
    fn call_function (
        self: &mut Self,
        function_number: u64,
        args: Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, RuntimeError>;

    /// Function that is used to provide information about internal
    /// functions of a plugin to the program using it, so it can
    /// choose the function it needs either by it's name, argument
    /// types, return type or all of the above
    fn get_function_list (self: &mut Self) -> Vec<Function>;

    /// Function that is used to provide information about
    /// non standard types, a function from this plugin might take
    /// as an argument or return, so that the program using the
    /// plugin can take such non-standard objects from one
    /// function implemented in this plugin and pass it on into
    /// another function in this plugin
    fn get_type_list (self: &mut Self) -> Vec<Type>;
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
/// in plugins. And it's fields are only read by
/// [`FreightProxy::load`] function when loading the plugin
pub struct FreightDeclaration {

    /// Rust compiler version as a static string
    pub rustc_version: &'static str,

    /// Api version as a static string
    pub api_version: &'static str,

    /// Version of the freight being imported
    pub freight_version: &'static str,

    /// Name of the freight being imported
    pub name: &'static str,

    /// Function that gets a [`FreightRegistrar`] trait implementor
    /// as an argument and calls it's freight_register function
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

    /// Imported freight's name as a static string
    pub name: &'static str,

    /// Imported freight's version as a static string
    pub version: &'static str,
}

/// Structure representing an empty [`Freight`] implementor, needed
/// for only for [`FreightProxy`] configuration
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

    fn get_type_list (self: &mut Self) -> Vec<Type> {
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

        let lib = std::rc::Rc::new(
            libloading::Library::new(lib_path).unwrap());

        let declaration: FreightDeclaration = lib
            .get::<*mut FreightDeclaration>(b"freight_declaration\0")
            .unwrap()
            .read();

        if declaration.rustc_version != RUSTC_VERSION
            || declaration.api_version != API_VERSION
        {
            return Err(RuntimeError::Message{
                msg: "Version mismatch"
            });
        }

        let mut result = FreightProxy {
            freight: Box::new(EmptyFreight{}),
            lib: lib,
            name: declaration.name,
            version: declaration.freight_version,
        };

        (declaration.register)(&mut result);

        Ok(result)
    }
}

/// Implementation of trait [`Freight`] for the proxy structure, so we
/// can call exact same functions from it
impl Freight for FreightProxy {
    fn call_function (
        self: &mut Self,
        function_number: u64,
        args: Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, RuntimeError> {
        self.freight.call_function(function_number, args)
    }

    fn get_function_list (self: &mut Self) -> Vec<Function> {
        self.freight.get_function_list()
    }

    fn get_type_list (self: &mut Self) -> Vec<Type> {
        self.freight.get_type_list()
    }
}

/// Implementation of [`FreightRegistrar`] trait for the proxy
/// structure, so that we can call register function on it without
/// any third party structure
impl FreightRegistrar for FreightProxy {
    fn register_freight (
        self: &mut Self,
        freight: Box<dyn Freight>,
        ) {
        self.freight = freight;
    }
}
