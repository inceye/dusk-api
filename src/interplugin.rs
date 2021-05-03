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

//! Module, containing traits and structures needed for proper
//! data transfer between plugins, as well as establishing connections
//! between them.

use crate::*;
pub use InterplugRequest::*;

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
#[derive(Clone, Debug)]
pub enum InterplugRequest {

    /// Request for a specific plugin with a specific version
    /// and name, and make sure the functions with ids provided
    /// have all dependencies fulfilled
    PlugRequest {

        /// The string, that identifies the plugin
        plugin: String,

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
        plugin: String,

        /// Trait name
        trait_name: String,

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
        plugin: String,

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
        plugin: String,

        /// Trait name
        trait_name: String,

        /// The version of the plugin containing the trait 
        /// definition
        version: Version,
    },

    /// An interlplug request that contains several interlplug
    /// requests, either of which *MUST* be fulfilled for the
    /// plugin to work at all
    RequestEither {

        /// A vector of the requests, either of which has to 
        /// be fulfilled
        requests: Vec<InterplugRequest>,
    },

    /// An interplug request that contains several interplug
    /// requests, each of which *MUST* be fulfilled in order for
    /// the plugin to work
    RequestEach {

        /// A vector of the requests, all of which have to 
        /// be fulfilled
        requests: Vec<InterplugRequest>,
    },

    /// An interplug request that *MUST* be fulfilled in order
    /// for the plugin to work at all
    RequestCrucial {

        /// A box, containing the actual request
        request: Box<InterplugRequest>,
    },

    /// An interplug request that must be fulfilled in order for
    /// the plugin to fully work, which means that without it
    /// some functions will be unavailable
    RequestOptional {

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
#[derive(Clone, Debug)]
pub enum Limitation {

    /// Set the maximum allowed number, represetting some setting
    Top {

        /// The name of the setting
        setting: String,

        /// The value to which we want to set it
        limit: isize,
    },

    /// Set the minimum allowed number, representing some setting
    Bottom {

        /// The name of the setting
        setting: String,

        /// The value to which we want to set it
        limit: isize,
    },

    /// Reset the setting to default value (as if the main program
    /// has never set any value to the setting at all)
    Reset {

        /// The name of the setting
        setting: String,
    },
}

/// A trait, implementors of which may be passed as arguments
pub trait DuskObject : Any + DuskObjClone {}

impl std::fmt::Debug for dyn DuskObject {
    fn fmt (
        self: &Self, 
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        f.pad("DuskObject")
    }
}

/// The trait, containing a function that clones the existing
/// DuskObject implementor
pub trait DuskObjClone {

    /// The function that returns a new box, of [`DuskCallable`]
    /// implementor
    fn dusk_object_clone_box (
        self: &Self,
    ) -> Box<dyn DuskObject>;
}

impl <T> DuskObjClone for T
where
    T: 'static + DuskObject + Clone
{
    fn dusk_object_clone_box (
        self: &Self,
    ) -> Box<dyn DuskObject> {

        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DuskObject> {
    fn clone (self: &Self) -> Box<dyn DuskObject> {
        self.dusk_object_clone_box()
    }
}

impl <T> DuskObject for T 
where 
    T: 'static + Any + Clone
{}
