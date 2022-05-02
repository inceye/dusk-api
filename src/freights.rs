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

//! Module, containing everything needed to register an exportable
//! freight and fill it with functionality, or to use another plugin
//! functionality

use crate::*;

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
    fn init (
        self: &mut Self,
        _limitations: &Option<Vec<Limitation>>,
    ) -> Vec<InterplugRequest> {

        Vec::new()
    }

    /// Function that updates system limitations
    fn update_limitations (
        self: &mut Self,
        _limitations: &Vec<Limitation>,
    ) {
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
    fn top_modules (self: &mut Self) -> Vec<Module>;

    /// The function that is used to provide the functions that
    /// implement all the binary operators this plugin provides
    fn get_operator_list (self: &mut Self) -> Vec<Function> {
        Vec::new()
    }

    /// Get the list of callables, extracted from the function list
    /// in ID order
    fn get_callable_list (
        self: &mut Self,
    ) -> Result<Vec<Box<dyn DuskCallable>>, Error> {

        match self.get_function_list() {
            Ok(list) => {
                let mut result: Vec<Box<dyn DuskCallable>> =
                    Vec::new();
                for function in list {
                    result.push(function.callable.clone());
                }
                return Ok(result);
            },
            Err(err) => return Err(err),
        }
    }

    /// Get callable by the ID of the function it is located in
    fn get_callable_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<Box<dyn DuskCallable>, Error> {

        match self.get_function_list() {
            Ok(list) => {
                if (list.len() <= id) {
                    return Err(IndexError(
                            format!(
                                "Callable with index {} does not exist",
                                id,
                            )));
                }
                if (list[id].name.eq(&"".to_string())) {
                    return Err(IndexError(
                            format!(
                                "Callable with index {} does not exist",
                                id,
                            )));
                }
                return Ok(list[id].callable.clone());
            },
            Err(err) => return Err(err),
        }
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
    ///
    fn get_function_list (
        self: &mut Self,
    ) -> Result<Vec<Function>, Error> {

        let all_modules: Vec<Module>;

        match self.get_module_list() {
            Err(err) => return Err(err),
            Ok(modules) => all_modules = modules,
        }

        let mut result: Vec<Function> = Vec::new();
        let mut result_unsorted: Vec<Function> = Vec::new();
        for def_fun in self.get_operator_list() {
            if (def_fun.name.eq(&"".to_string())) {
                return Err(ImportError(
                        format!(
                            "{}",
                            "Operators can not have empty names",
                        )));
            }
            result_unsorted.push(def_fun.clone());
        }
        for module in all_modules {
            for def_fun in module.functions {
                if (def_fun.name.eq(&"".to_string())) {
                    return Err(ImportError(
                            format!(
                                "{}",
                                "Functions can not have empty names",
                            )));
                }
                result_unsorted.push(def_fun.clone());
                result_unsorted.last_mut().unwrap().name = format!(
                    "{}::{}",
                    module.name,
                    def_fun.name);
            }
            for def_con in module.constants {
                if (def_con.name.eq(&"".to_string())) {
                    return Err(ImportError(
                            format!(
                                "{}",
                                "Functions can not have empty names",
                            )));
                }
                result_unsorted.push(def_con.clone());
                result_unsorted.last_mut().unwrap().name = format!(
                    "@{}::{}",
                    module.name,
                    def_con.name);
            }
            for def_type in module.types {
                if (def_type.name.eq(&"".to_string())) {
                    return Err(ImportError(
                            format!(
                                "{}",
                                "Types can not have empty names",
                            )));
                }
                for def_met in def_type.methods {
                    if (def_met.name.eq(&"".to_string())) {
                        return Err(ImportError(
                                format!(
                                    "{}",
                                    "Type methods can not have empty names",
                                )));
                    }
                    result_unsorted.push(def_met.clone());
                    result_unsorted.last_mut().unwrap().name = format!(
                        "{}::{}::{}",
                        module.name,
                        def_type.name,
                        def_met.name);
                }
                for def_fil in def_type.fields {
                    if (def_fil.name.eq(&"".to_string())) {
                        return Err(ImportError(
                                format!(
                                    "{}",
                                    "Type fields can not have empty names",
                                )));
                    }
                    result_unsorted.push(def_fil.clone());
                    result_unsorted.last_mut().unwrap().name = format!(
                        "@{}::{}::{}",
                        module.name,
                        def_type.name,
                        def_fil.name);
                }
                for def_trt in def_type.trait_implementations {
                    for def_met in def_trt.methods {
                        if (def_met.function.name.eq(&"".to_string())) {
                            return Err(ImportError(
                                    format!(
                                        "{}",
                                        "Type methods can not have empty names",
                                    )));
                        }
                        result_unsorted.push(def_met.function.clone());
                        result_unsorted.last_mut().unwrap().name = format!(
                            "{}::{}::{}",
                            module.name,
                            def_type.name,
                            def_met.function.name);
                    }
                }
            }
        }

        for def_met in result_unsorted {
            if def_met.fn_id < result.len() {
                if (!def_met.name.eq(&"".to_string())) {
                    return Err(ImportError(
                            format!(
                                "Several functions with same id ({}) found",
                                def_met.fn_id,
                            )));
                }
                result[def_met.fn_id] =
                    def_met.clone();

                continue;
            }
            for _i in result.len()..def_met.fn_id {
                result.push(Default::default());
            }
            result.push(def_met.clone());
        }

        return Ok(result);
    }

    /// Get function by its ID
    fn get_function_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<Function, Error> {

        match self.get_function_list() {
            Ok(list) => {
                if (list.len() <= id) {
                    return Err(IndexError(
                            format!(
                                "Function with index {} does not exist",
                                id,
                            )));
                }
                if (list[id].name.eq(&"".to_string())) {
                    return Err(IndexError(
                            format!(
                                "Function with index {} does not exist",
                                id,
                            )));
                }
                return Ok(list[id].clone());
            },
            Err(err) => return Err(err),
        }
    }

    /// Get function by its name
    fn get_functions_by_name (
        self: &mut Self,
        name: &String,
    ) -> Result<Vec<Function>, Error> {

        let mut res: Vec<Function> = Vec::new();
        match self.get_function_list() {
            Ok(list) => {
                for function in list {
                    if (function.name.eq(name)) {
                        res.push(function.clone());
                    }
                }
                return Ok(res);
            },
            Err(err) => return Err(err),
        }
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
    ///
    fn get_type_list (
        self: &mut Self,
    ) -> Result<Vec<Type>, Error> {

        let all_modules: Vec<Module>;

        match self.get_module_list() {
            Err(err) => return Err(err),
            Ok(modules) => all_modules = modules,
        }

        let mut result: Vec<Type> = Vec::new();
        for module in all_modules {
            for mut def_type in module.types {
                if (def_type.name.eq(&"".to_string())) {
                    return Err(ImportError(
                            format!(
                                "{}",
                                "Types can not have empty names",
                            )));
                }
                def_type.name = format!(
                    "{}::{}",
                    module.name,
                    def_type.name);
                if def_type.tp_id < result.len() {
                    if (!def_type.name.eq(&"".to_string())) {
                        return Err(ImportError(
                                format!(
                                    "Several types with same id ({}) found",
                                    def_type.tp_id,
                                )));
                    }
                    result[def_type.tp_id] = def_type.clone();
                    continue;
                }
                for _i in result.len()..def_type.tp_id {
                    result.push(Default::default());
                }
                result.push(def_type.clone());
            }
        }
        return Ok(result);
    }

    /// Get type by its ID
    fn get_type_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<Type, Error> {

        match self.get_type_list() {
            Ok(list) => {
                if (list.len() <= id) {
                    return Err(IndexError(
                            format!(
                                "Type with index {} does not exist",
                                id,
                            )));
                }
                if (list[id].name.eq(&"".to_string())) {
                    return Err(IndexError(
                            format!(
                                "Type with index {} does not exist",
                                id,
                            )));
                }
                return Ok(list[id].clone());
            },
            Err(err) => return Err(err),
        }
    }

    /// Get type by its name
    fn get_types_by_name (
        self: &mut Self,
        name: &String,
    ) -> Result<Vec<Type>, Error> {

        let mut res: Vec<Type> = Vec::new();
        match self.get_type_list() {
            Ok(list) => {
                for one_type in list {
                    if (one_type.name.eq(name)) {
                        res.push(one_type.clone());
                    }
                }
                return Ok(res);
            },
            Err(err) => return Err(err),
        }
    }

    /// Get type by its native ID
    fn get_type_by_native_id (
        self: &mut Self,
        native_id: TypeId,
    ) -> Result<Type, Error> {

        match self.get_type_list() {
            Ok(list) => {
                for one_type in list {
                    if (one_type.native_id.eq(&native_id)) {
                        return Ok(one_type.clone());
                    }
                }
                return Err(IndexError(
                        format!(
                            "Could not find type with native id {:#?} in list",
                            native_id,
                        )));
            },
            Err(err) => return Err(err),
        }
    }

    /// Get a vector of all trait definitions provided by the plugin,
    /// composed in such way that the type's ID corresponds to its
    /// position in this vector. Also change the name to the full name,
    /// containing the name of the module it is located in.
    fn get_trait_definition_list (
        self: &mut Self,
    ) -> Result<Vec<TraitDefinition>, Error> {

        let all_modules: Vec<Module>;

        match self.get_module_list() {
            Err(err) => return Err(err),
            Ok(modules) => all_modules = modules,
        }

        let mut result: Vec<TraitDefinition> = Vec::new();
        for module in all_modules {
            for mut def_trt in module.trait_definitions {
                if (def_trt.name.eq(&"".to_string())) {
                    return Err(ImportError(
                            format!(
                                "{}",
                                "Traits can not have empty names",
                            )));
                }
                def_trt.name = format!(
                    "{}::{}",
                    module.name,
                    def_trt.name);
                if def_trt.trait_id < result.len() {
                    if (!def_trt.name.eq(&"".to_string())) {
                        return Err(ImportError(
                                format!(
                                    "Several traits with same id ({}) found",
                                    def_trt.trait_id,
                                )));
                    }
                    result[def_trt.trait_id] = def_trt.clone();
                    continue;
                }
                for _i in result.len()..def_trt.trait_id {
                    result.push(Default::default());
                }
                result.push(def_trt.clone());
            }
        }
        return Ok(result);
    }

    /// Get trait definition by its ID
    fn get_trait_definition_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<TraitDefinition, Error> {

        match self.get_trait_definition_list() {
            Ok(list) => {
                if (list.len() <= id) {
                    return Err(IndexError(
                            format!(
                                "Trait with index {} does not exist",
                                id,
                            )));
                }
                if (list[id].name.eq(&"".to_string())) {
                    return Err(IndexError(
                            format!(
                                "Trait with index {} does not exist",
                                id,
                            )));
                }
                return Ok(list[id].clone());
            },
            Err(err) => return Err(err),
        }
    }

    /// Get trait definition by its name
    fn get_trait_definitions_by_name (
        self: &mut Self,
        name: &String,
    ) -> Result<Vec<TraitDefinition>, Error> {

        let mut res: Vec<TraitDefinition> = Vec::new();
        match self.get_trait_definition_list() {
            Ok(list) => {
                for trait_definition in list {
                    if (trait_definition.name.eq(name)) {
                        res.push(trait_definition.clone());
                    }
                }
                return Ok(res);
            },
            Err(err) => return Err(err),
        }
    }

    /// Get all modules, provided by the plugin as a vector, where
    /// module's ID corresponds to its location in that vector.
    ///
    /// The list must contain all the modules, unwrapping all nesting
    /// and changing names to full names, containing names of modules
    /// higher in the tree
    fn get_module_list (
        self: &mut Self,
    ) -> Result<Vec<Module>, Error> {

        let top_modules: Vec<Module> = self.top_modules();
        let mut parents: Vec<Module>;
        let mut par_progress: Vec<usize>;
        let mut result: Vec<Module> = Vec::new();
        for module in top_modules {
            if (module.name.eq(&"".to_string())){
                return Err(ImportError(
                    format!(
                        "{}",
                        "Modules can not have empty names",
                    )));
            }
            parents = Vec::new();
            par_progress = Vec::new();
            parents.push(module.clone());
            par_progress.push(0);
            while parents.len() > 0 {

                let tmp_name: String =
                    parents.last().unwrap().name.clone();

                if (*par_progress.last().unwrap() <
                    parents.last().unwrap().submodules.len())
                {

                    parents.push(parents.last().unwrap().submodules[
                        *par_progress.last().unwrap()].clone());

                    if (parents.last().unwrap().name.eq(
                            &"".to_string(),
                    )) {

                        return Err(ImportError(
                            format!(
                                "{}",
                                "Modules can not have empty names",
                            )));
                    }

                    parents.last_mut().unwrap().name = format!(
                        "{}::{}",
                        tmp_name,
                        parents.last().unwrap().name);

                    *par_progress.last_mut().unwrap() += 1;
                    par_progress.push(0);
                    continue;
                }

                par_progress.pop();
                if parents.last().unwrap().md_id < result.len() {
                    if (result[parents.last().unwrap().md_id].name.ne(
                            &"".to_string(),
                    )) {

                        return Err(ImportError(
                            format!(
                                "Several modules with same id ({}) found",
                                parents.last().unwrap().md_id,
                            )));
                    }
                    result[parents.last().unwrap().md_id] =
                        parents.last().unwrap().clone();

                    continue;
                }
                for _i in result.len()..parents.last().unwrap().md_id {
                    result.push(Default::default());
                }
                result.push(parents.pop().unwrap());
            }
        }
        return Ok(result);
    }

    /// Get module by its ID
    fn get_module_by_id (
        self: &mut Self,
        id: usize,
    ) -> Result<Module, Error> {

        match self.get_module_list() {
            Ok(list) => {
                if (list.len() <= id) {
                    return Err(IndexError(
                            format!(
                                "Module with index {} does not exist",
                                id,
                            )));
                }
                if (list[id].name.eq(&"".to_string())) {
                    return Err(IndexError(
                            format!(
                                "Module with index {} does not exist",
                                id,
                            )));
                }
                return Ok(list[id].clone());
            },
            Err(err) => return Err(err),
        }
    }

    /// Get module by its full name
    fn get_modules_by_name (
        self: &mut Self,
        name: &String,
    ) -> Result<Vec<Module>, Error> {

        let mut res: Vec<Module> = Vec::new();
        match self.get_module_list() {
            Ok(list) => {
                for module in list {
                    if (module.name.eq(name)) {
                        res.push(module.clone());
                    }
                }
                return Ok(res);
            },
            Err(err) => return Err(err),
        }
    }
}

impl std::fmt::Debug for dyn Freight {
    fn fmt (
        self: &Self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        f.pad("Freight")
    }
}

/// Structure representing an empty [`Freight`] implementor, needed
/// only for [`FreightProxy`] configuration
#[derive(Copy, Clone, Debug)]
pub struct EmptyFreight;
impl Freight for EmptyFreight {
    fn top_modules (self: &mut Self) -> Vec<Module> {
        Vec::new()
    }
}
