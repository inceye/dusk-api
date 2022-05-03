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

//! Module, containing everything needed to register and use a
//! plugin

use crate::*;

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
/// ``` rust, ignore
/// let mut my_f_proxy: FreightProxy = unsafe{
///     FreightProxy::load("/bin/libtest_plug.so").expect("fail")
/// };
/// println!("{}, {}", my_f_proxy.name, my_f_proxy.version);
/// let fnlist: Vec<Function> = my_f_proxy.get_function_list();
/// for func in fnlist {
///     println!("{}, {}", func.name, func.number);
/// }
/// ```
#[derive(Debug)]
pub struct FreightProxy {

    /// Imported freight, solely for internal purposes
    freight: Box<dyn Freight>,

    /// Lib this freight was imported from to make sure this
    /// structure does not outlive the library it was imported from
    lib: Option<std::rc::Rc<libloading::Library>>,

    /// Imported freights name as a static string
    pub name: String,

    /// Imported freights version
    pub version: Version,

    /// The earliest version, for which the code was designed, this
    /// code can safely be run with the new plugin version
    pub backwards_compat_version: Version,

    callables: Option<Vec<Box<dyn DuskCallable>>>,

    functions: Option<Vec<Function>>,

    types: Option<Vec<Type>>,

    trait_definitions: Option<Vec<TraitDefinition>>,

    modules: Option<Vec<Module>>,

    functions_by_name: Option<std::collections::HashMap<String, Vec<usize>>>,

    types_by_name: Option<std::collections::HashMap<String, Vec<usize>>>,

    types_by_native_id: Option<std::collections::HashMap<TypeId, usize>>,

    trait_definitions_by_name: Option<std::collections::HashMap<String, Vec<usize>>>,

    modules_by_name: Option<std::collections::HashMap<String, Vec<usize>>>,
}

/// Functions, needed to configure [`FreightProxy`] structure
/// initially
impl FreightProxy {

    /// Function, used to build a [`FreightProxy`] object from a
    /// library path
    pub unsafe fn load (
        lib_path: &str,
    ) -> Result<FreightProxy, Error> {

        // Import the library
        let lib : std::rc::Rc<libloading::Library>;
        match libloading::Library::new(lib_path) {
            Ok(library) => lib = std::rc::Rc::new(library),
            Err(lib_err) => return(Err(LoadingError (lib_err))),
        }

        // Get the plugin declaration structure from this lib
        let declaration: FreightDeclaration;
        match lib.get::<*mut FreightDeclaration>(
            b"freight_declaration\0") {

            Ok(decl) => declaration = decl.read(),
            Err(lib_err) => return(Err(LoadingError (lib_err))),
        }

        let mut result: FreightProxy =
            FreightProxy::load_from_declaration(&declaration)?;
        result.lib = Some(lib);

        // Return the result
        Ok(result)
    }

    /// # Warning
    /// This is an internal function. This is reserved for
    /// future use in dawn builder and must never be used
    /// for any purpose other than creating the program that
    /// would link together the final program in case of
    /// building for systems that do not allow for runtime
    /// dynamic library loading. In cases that do not
    /// require static linking use the [`FreightProxy::load`]
    /// function instead.
    ///
    /// # Unsafe, Use at your own risk
    /// You may
    /// also use this function when implementing your own
    /// library loading method, but beware,
    /// [`FreightProxy`] implements a way of keeping
    /// track of the library in use so it is offloaded then
    /// and only then when it is not being used by anything
    /// anymore. If you implement your own library loading
    /// method, you will have to keep track of that library
    /// yourself so it is not offloaded when still in use
    /// which will result in undefined behaviour, and it
    /// is not kept in memory when there are no more
    /// FreightProxies that are derived from it which will
    /// result in a memory leak.
    ///
    /// # Use
    /// This function is used on the importer side instead
    /// of the regular load function when loading a
    /// [`FreightProxy`] from a [`FreightDeclaration`] that
    /// has either already been loaded from the runtime loaded
    /// dynamic library or is found in the statically or
    /// load-on-startup dynamically loaded library.
    ///
    pub fn load_from_declaration (
        declaration: &FreightDeclaration,
    ) -> Result<FreightProxy, Error> {

        // Check if the compiler and api versions match
        // If not -- immediately return an error
        if declaration.rustc_version != RUSTC_VERSION {
            return Err(ImportError (
                    "Compiler version mismatch".to_string()
            ));
        }

        if declaration.api_version != API_VERSION {
            return Err(ImportError (
                    "Dusk API version mismatch".to_string()
            ));
        }

        return FreightProxy::load_from_declaration_no_vcheck(&declaration);
    }

    /// # Warning
    /// This is an internal function, it does the same
    /// thing as [`FreightProxy::load_from_declaration`]
    /// but without checking neither the API version nor
    /// the compiler version. Please check documentation for
    /// [`FreightProxy::load_from_declaration`] before use.
    ///
    pub fn load_from_declaration_no_vcheck (
        declaration: &FreightDeclaration,
    ) -> Result<FreightProxy, Error> {

        // Make a new FreightProxy with all values that are
        // already available
        let mut result: FreightProxy = FreightProxy {
            freight: Box::new(EmptyFreight{}),
            lib: None,
            name: declaration.name,
            version: declaration.freight_version,
            backwards_compat_version: declaration.backwards_compat_version,
            callables: None,
            functions: None,
            types: None,
            trait_definitions: None,
            modules: None,
            functions_by_name: None,
            types_by_name: None,
            types_by_native_id: None,
            trait_definitions_by_name: None,
            modules_by_name: None,
        };

        // Call the function, imported in the plugin declaration
        // and pass the FreightProxy to it as an argument
        // so it sets the internal freight variable to a
        // correct value
        (declaration.register)(&mut result);

        return Ok(result);
    }
}

macro_rules! remember_or_create {
    ($self: ident, $memory: ident, $get_list: ident) => {

        match &$self.$memory {
            Some(list) => return Ok(list.clone()),
            None => {
                $self.$memory = Some($self.freight.$get_list()?);
                return Ok($self.$memory.as_ref().unwrap().clone());
            },
        }
    }
}

macro_rules! find_by_id {
    ($name: expr, $id: expr, $self: ident, $memory: ident, $get_list: ident, $self_fn: ident) => {

        return match &$self.$memory {
            Some(list) => {
                if list.len() > $id {
                    if (!list[$id].name.eq(&"".to_string())) {
                        return Ok(list[$id].clone());
                    }
                }
                return Err(IndexError(
                        format!(
                            "{} with index {} does not exist",
                            $name,
                            $id,
                        )));
            },
            None => {
                $self.$memory = Some($self.freight.$get_list()?);
                $self.$self_fn($id)
            },
        }
    }
}

macro_rules! find_by_name {
    ($type: ident, $id: expr, $self: ident, $memory: ident,
     $get_list: ident, $get_by_id: ident, $self_fn: ident) => {

        return match &mut $self.$memory {
            Some(hash_map) => {
                let mut res: Vec<$type> = Vec::new();
                for id in hash_map.entry($id.clone()).or_default().clone() {
                    res.push($self.$get_by_id(id)?);
                }
                return Ok(res);
            },
            None => {
                let mut hash_map: std::collections::HashMap<String, Vec<usize>> =
                    std::collections::HashMap::new();
                let mut idx: usize = 0;
                for item in $self.$get_list()? {
                    hash_map.entry(item.name)
                        .or_default()
                        .push(idx);
                    idx += 1;
                }
                $self.$memory = Some(hash_map);
                $self.$self_fn(&$id)
            }
        }
    }
}

// Implementation of trait Freight for the proxy structure, so we
// can call exact same functions from it
impl Freight for FreightProxy {

    // Proxy function, that calls the internal freights init function
    // and returns its plugin dependencies
    fn init (
        self: &mut Self,
        limitations: &Option<Vec<Limitation>>,
    ) -> Vec<InterplugRequest> {

        self.freight.init(limitations)
    }

    // Proxy function that takes the list of new system limitations
    // and passes it to the plugin
    fn update_limitations (
        self: &mut Self,
        limitations: &Vec<Limitation>,
    ) {

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

    fn get_root_modules (self: &mut Self) -> Vec<Module> {
        self.freight.get_root_modules()
    }

    fn get_operator_list (self: &mut Self) -> Vec<Function> {
        self.freight.get_operator_list()
    }

    fn get_callable_list (
        self: &mut Self,
    ) -> Result<Vec<Box<dyn DuskCallable>>, Error> {

        match &self.callables {
            Some(list) => return Ok(list.clone()),
            None => {
                let tmp_functions: Vec<Function> = self.get_function_list()?;
                let mut tmp_callables: Vec<Box<dyn DuskCallable>> = Vec::new();
                for function in tmp_functions {
                    tmp_callables.push(function.callable.clone());
                }
                self.callables = Some(tmp_callables.clone());
                return Ok(tmp_callables);
            },
        }
    }

    fn get_callable_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<Box<dyn DuskCallable>, Error> {

        let function: Function = self.get_function_by_id(id)?;
        return Ok(function.callable);
    }

    fn get_function_list (
        self: &mut Self,
    ) -> Result<Vec<Function>, Error> {

        remember_or_create!(self, functions, get_function_list);
    }

    fn get_function_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<Function, Error> {

        find_by_id!("Function", id, self, functions, get_function_list, get_function_by_id);
    }

    fn get_functions_by_name (
        self: &mut Self,
        name: &String,
    ) -> Result<Vec<Function>, Error> {

        find_by_name!(Function, name, self, functions_by_name, get_function_list,
            get_function_by_id, get_functions_by_name)
    }

    fn get_type_list (
        self: &mut Self,
    ) -> Result<Vec<Type>, Error> {

        remember_or_create!(self, types, get_type_list);
    }

    fn get_type_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<Type, Error> {

        find_by_id!("Type", id, self, types, get_type_list, get_type_by_id);
    }

    fn get_type_by_native_id (
        self: &mut Self,
        native_id: TypeId,
    ) -> Result<Type, Error> {

        match &self.types_by_native_id {
            Some(hash_map) => {
                match hash_map.get(&native_id) {
                    Some(id) => {
                        let id_clone = id.clone();
                        self.get_type_by_id(id_clone)
                    },
                    None => Err(IndexError(
                            format!(
                                "Could not find type with native id {:#?} in list",
                                native_id,
                            ))),
                }

            },
            None => {
                let mut hash_map: std::collections::HashMap<TypeId, usize> =
                    std::collections::HashMap::new();
                let mut idx: usize = 0;
                for item in self.get_type_list()? {
                    hash_map.insert(item.native_id.clone(), idx);
                    idx += 1;
                }
                self.types_by_native_id = Some(hash_map);
                self.get_type_by_native_id(native_id)
            }
        }

    }

    fn get_types_by_name (
        self: &mut Self,
        name: &String,
    ) -> Result<Vec<Type>, Error> {

        find_by_name!(Type, name, self, types_by_name, get_type_list,
            get_type_by_id, get_types_by_name)
    }

    fn get_trait_definition_list (
        self: &mut Self,
    ) -> Result<Vec<TraitDefinition>, Error> {

        remember_or_create!(self, trait_definitions, get_trait_definition_list);
    }

    fn get_trait_definition_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<TraitDefinition, Error> {

        find_by_id!("Trait", id, self, trait_definitions,
            get_trait_definition_list, get_trait_definition_by_id);
    }

    fn get_trait_definitions_by_name (
        self: &mut Self,
        name: &String,
    ) -> Result<Vec<TraitDefinition>, Error> {

        find_by_name!(TraitDefinition, name, self, trait_definitions_by_name,
            get_trait_definition_list, get_trait_definition_by_id,
            get_trait_definitions_by_name)
    }

    fn get_module_list (
        self: &mut Self,
    ) -> Result<Vec<Module>, Error> {

        remember_or_create!(self, modules, get_module_list);
    }

    fn get_module_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<Module, Error> {

        find_by_id!("Module", id, self, modules, get_module_list, get_module_by_id);
    }

    fn get_modules_by_name (
        self: &mut Self,
        name: &String,
    ) -> Result<Vec<Module>, Error> {

        find_by_name!(Module, name, self, modules_by_name, get_module_list,
            get_module_by_id, get_modules_by_name)
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
