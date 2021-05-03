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

//! Module, containing everything needed to export a callable

use crate::*;

/// A trait that defines the behavior of a function wrapper, used
/// to call functions imported from plugins
pub trait DuskCallable: CallableClone {

    /// The function that takes arguments, processes them and any
    /// data that is stored in the implementor struct and calls
    /// the underlying function, returning it's result
    fn call (
        self: &mut Self,
        args: &Vec<&mut Box<dyn DuskObject>>
    ) -> Result<Box<dyn DuskObject>, DuskError>;
}

impl std::fmt::Debug for dyn DuskCallable {
    fn fmt (
        self: &Self, 
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        f.pad("DuskCallable")
    }
}

/// The trait, containing a function that clones the existing
/// DuskCallable implementor
pub trait CallableClone {

    /// The function that returns a new box, of [`DuskCallable`]
    /// implementor
    fn dusk_callable_clone_box (
        self: &Self,
    ) -> Box<dyn DuskCallable>;
}

impl <T> CallableClone for T
where
    T: 'static + DuskCallable + Clone
{
    fn dusk_callable_clone_box (
        self: &Self,
    ) -> Box<dyn DuskCallable> {

        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DuskCallable> {
    fn clone (self: &Self) -> Box<dyn DuskCallable> {
        self.dusk_callable_clone_box()
    }
}

/// Simplest Dusk callable implementor -- only contains one function
/// that gets the argument vector as an argument and simply passes
/// the arguments and returned Result
#[derive(Copy, Clone)]
pub struct SimpleCallable {
    underlying_fn: 
        fn (&Vec<&mut Box<dyn DuskObject>>) 
            -> Result<Box<dyn DuskObject>, DuskError>,
}

impl DuskCallable for SimpleCallable {
    fn call (
        self: &mut Self,
        args: &Vec<&mut Box<dyn DuskObject>>
    ) -> Result<Box<dyn DuskObject>, DuskError> {

        (self.underlying_fn)(args)
    }
}

impl std::fmt::Debug for SimpleCallable {
    fn fmt (
        self: &Self, 
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        f.debug_struct("SimpleCallable")
            .finish()
    }
}

/// Dusk callable that not only holds a function pointer, but
/// also a vector of args to pass to the underlying function
/// as well as the arguments provided to the call function
#[derive(Clone)]
pub struct ConstArgsCallable {
    const_args: Vec<Box<dyn DuskObject>>,
    underlying_fn: 
        fn (
            &Vec<Box<dyn DuskObject>>, 
            &Vec<&mut Box<dyn DuskObject>>,
        ) -> Result<Box<dyn DuskObject>, DuskError>,
}

impl DuskCallable for ConstArgsCallable {
    fn call (
        self: &mut Self,
        args: &Vec<&mut Box<dyn DuskObject>>
    ) -> Result<Box<dyn DuskObject>, DuskError> {

        (self.underlying_fn)(&self.const_args.clone(), args)
    }
}

impl std::fmt::Debug for ConstArgsCallable {
    fn fmt (
        self: &Self, 
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        f.debug_struct("ConstArgsCallable")
            .field("const_args", &self.const_args)
            .finish()
    }
}

/// A default callable: does not call anything, always returns
/// [`DuskError::NotImplementedError`]
#[derive(Copy, Clone, Debug)]
pub struct EmptyCallable;

impl DuskCallable for EmptyCallable {
    fn call (
        self: &mut Self,
        _args: &Vec<&mut Box<dyn DuskObject>>
    ) -> Result<Box<dyn DuskObject>, DuskError> {

        Err(DuskError::NotImplementedError (
                "Called function is not implemented".to_string()
        ))
    }
}
