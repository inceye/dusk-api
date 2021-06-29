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

//! Module containing structures, traits and implementations, that
//! help to get data across between different functions and plugins

use crate::*;

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
