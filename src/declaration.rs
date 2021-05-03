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

//! Module, containing everything needed for plugin export

use crate::*;

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
#[derive(Clone)]
pub struct FreightDeclaration {

    /// Rust compiler version as a static string
    pub rustc_version: String,

    /// Api version as a static string
    pub api_version: String,

    /// Version of the freight being imported
    pub freight_version: Version,

    /// The earliest plugin version, for which the code could have 
    /// been designed, and still be run with this version
    /// of plugin, with same results
    pub backwards_compat_version: Version,

    /// Name of the freight being imported
    pub name: String,

    /// Function that gets a [`FreightRegistrar`] trait implementor
    /// as an argument and calls its freight_register function
    /// to provide unexportable things, such as structs, in
    /// particular, [`Freight`] implementor structures
    pub register: fn (&mut dyn FreightRegistrar),
}

impl std::fmt::Debug for FreightDeclaration {
    fn fmt (
        self: &Self, 
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        f.debug_struct("FreightDeclaration")
            .field("rustc_version", &self.rustc_version)
            .field("api_version", &self.api_version)
            .field("freight_version", &self.freight_version)
            .field("backwards_compat_version", 
                &self.backwards_compat_version)
            .field("name", &self.name)
            .finish()
    }
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
