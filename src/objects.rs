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
//! help to move data between different functions and plugins



use crate::*;



pub const READ_FORBID       : u32 = 0x00000001;
pub const WRITE_FORBID      : u32 = 0x00000002;
pub const CLONE_FORBID      : u32 = 0x00000004;
pub const DELETE_FORBID     : u32 = 0x00000008;
pub const LOCK_FORBID       : u32 = 0x00000010;
pub const ELOCK_FORBID      : u32 = 0x00000020;
pub const DUMP_FORBID       : u32 = 0x00000040;
pub const LOAD_FORBID       : u32 = 0x00000080;

pub const ALL_PERM          : u32 = 0x00000000;
pub const PERM_MASK         : u32 = 0x0000FFFF;

pub const SIZE_MASK         : u32 = 0x00FF0000;
pub const SIZE_SHIFT        : u32 = 0x00000010;

const LOCK_NANO_SLEEP       : u32 = 0x00000100;



#[derive(Debug)]
pub struct Object {
    data: std::ptr::NonNull<Box<dyn DkAny>>,
    phantom: std::marker::PhantomData<Box<dyn DkAny>>,
    data_type: &'static Type,
    flags: u32,
}

#[derive(Debug)]
pub struct ObjCore {
    rc: std::sync::atomic::AtomicUsize,
    lck: std::sync::atomic::AtomicUsize,
}

#[derive(Debug)]
pub struct ObjGuard<'a> {
    data_obj: &'a Object,
}

#[derive(Debug)]
pub struct ObjGuardMut<'a> {
    data_obj: &'a mut Object,
}



pub trait DkGen {
    fn dk_new (
        freight: Object
        ) -> Result<Box<dyn DkAny>, Error>
        where Self: Sized;
}

pub trait DkRefCount {
    fn dk_incref (
        self: &Self,
    ) -> Result<usize, Error>;

    fn dk_decref (
        self: &Self,
    ) -> Result<usize, Error>;
}

pub trait DkRWLock {
    fn dk_lock_ex (
        self: &Self,
    ) -> Result<(), Error>;

    fn dk_try_lock_ex (
        self: &Self,
    ) -> Result<bool, Error>;

    fn dk_lock (
        self: &Self,
    ) -> Result<(), Error>;

    fn dk_try_lock (
        self: &Self,
    ) -> Result<bool, Error>;

    fn dk_unlock (
        self: &Self,
    ) -> Result<(), Error>;
}

pub trait DkGet {
    fn dk_get (
        self: &Self,
    ) -> Result<Box<dyn DkAny>, Error>;
}

pub trait DkSet {
    fn dk_set (
        self: &mut Self,
        new_data: &Box<dyn DkAny>,
    ) -> Result<(), Error>;
}

pub trait DkDump {
    fn dk_dump (
        self: &Self,
    ) -> Result<Vec<u8>, Error>;
}

pub trait DkLoad {
    fn dk_load (
        self: &mut Self,
        new_data: Vec<u8>,
        cursor: &mut usize,
    ) -> Result<(), Error>;
}

/// A trait, implementors of which may be passed as arguments
pub trait DkAny : Any + DkGen + DkRefCount + DkRWLock + DkGet + DkSet + DkDump + DkLoad
{
    fn to_object (
        self: &Self,
        data_type: &'static Type,
        flags: u32,
    ) -> Result<Object, Error>
    where Self: Sized + Clone
    {

        Ok(Object::new(
            Box::new(self.clone()),
            data_type,
            flags,
        ))
    }
}

pub trait ToDk {
    fn to_dk (
        self: &Self,
    ) -> Result<Box<dyn DkAny>, Error>;

    fn to_dk_object (
        self: &Self,
    ) -> Result<Object, Error>;
}



impl std::fmt::Debug for dyn DkAny {
    fn fmt (
        self: &Self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        f.pad("DkAny")
    }
}

impl <T> DkAny for T
where
    T: 'static + Any + DkGen + DkRefCount + DkRWLock + DkGet + DkSet + DkDump + DkLoad
{}

impl Object {

    pub fn new (
        data: Box<dyn DkAny>,
        data_type: &'static Type,
        flags: u32,
    ) -> Object {

        let boxed = Box::new(data);

        Object {
            data: std::ptr::NonNull::new(Box::into_raw(boxed)).unwrap(),
            data_type: data_type,
            flags: flags,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn get_flags (
        self: &Object,
    ) -> Result<u32, Error> {

        return Ok(self.flags);
    }

    pub fn set_flags (
        self: &mut Object,
        flags: u32,
    ) -> Result<(), Error> {

        self.flags = flags;
        return Ok(());
    }

    pub fn flags_has_bits (
        self: &Object,
        hasbits: u32,
    ) -> Result<bool, Error> {

        if self.flags & hasbits == hasbits {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn flags_set_bits (
        self: &mut Object,
        setbits: u32,
    ) -> Result<(), Error> {

        self.flags |= setbits;
        return Ok(());
    }

    pub fn flags_clear_bits (
        self: &mut Object,
        clrbits: u32,
    ) -> Result<(), Error> {

        self.flags &= !clrbits;
        return Ok(());
    }

    pub fn flags_clear_set_bits (
        self: &mut Object,
        clrbits: u32,
        setbits: u32,
    ) -> Result<(), Error> {

        self.flags &= !clrbits;
        self.flags |= setbits;
        return Ok(());
    }

    pub fn get_underlying_data (
        self: &Object,
    ) -> Result<Box<dyn DkAny>, Error> {

        // TODO: read lock
        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        Ok(inner.dk_get()?)
    }

    pub fn set_underlying_data (
        self: &mut Object,
        new_data: &Box<dyn DkAny>,
    ) -> Result<(), Error> {

        // TODO: exclusive lock
        let inner: &mut Box<dyn DkAny> = unsafe { self.data.as_mut() };
        Ok(inner.dk_set(new_data)?)
    }

    pub fn get_ref (
        self: &Object,
    ) -> Result<ObjGuard<'_>, Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_lock()?;
        // TODO: use dk_get to get the data
        Ok(ObjGuard { data_obj: &self })
    }

    pub fn get_mut (
        self: &mut Object,
    ) -> Result<ObjGuardMut<'_>, Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_lock_ex()?;
        // TODO: use dk_get to get the data
        Ok(ObjGuardMut { data_obj: self })
    }
}

impl DkRefCount for Object {
    fn dk_incref (
        self: &Self,
    ) -> Result<usize, Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_incref()
    }

    fn dk_decref (
        self: &Self,
    ) -> Result<usize, Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_decref()
    }
}

impl DkRWLock for Object {
    fn dk_lock_ex (
        self: &Self,
    ) -> Result<(), Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_lock_ex()
    }

    fn dk_try_lock_ex (
        self: &Self,
    ) -> Result<bool, Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_try_lock_ex()
    }

    fn dk_lock (
        self: &Self,
    ) -> Result<(), Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_lock()
    }

    fn dk_try_lock (
        self: &Self,
    ) -> Result<bool, Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_try_lock()
    }

    fn dk_unlock (
        self: &Self,
    ) -> Result<(), Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_unlock()
    }
}

impl DkGet for Object {
    fn dk_get (
        self: &Self,
    ) -> Result<Box<dyn DkAny>, Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_get()
    }
}

impl DkSet for Object {
    fn dk_set (
        self: &mut Self,
        new_data: &Box<dyn DkAny>,
    ) -> Result<(), Error> {

        let inner: &mut Box<dyn DkAny> = unsafe { self.data.as_mut() };
        inner.dk_set(new_data)
    }
}

impl DkDump for Object {
    fn dk_dump (
        self: &Self,
    ) -> Result<Vec<u8>, Error> {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };
        inner.dk_dump()
    }
}

impl DkLoad for Object {
    fn dk_load (
        self: &mut Self,
        new_data: Vec<u8>,
        cursor: &mut usize,
    ) -> Result<(), Error> {

        let inner: &mut Box<dyn DkAny> = unsafe { self.data.as_mut() };
        inner.dk_load(new_data, cursor)
    }
}

impl ObjCore {

    pub fn new () -> ObjCore {
        ObjCore {
            rc: std::sync::atomic::AtomicUsize::new(1),
            lck: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn get_ref (
        self: &ObjCore,
    ) -> Result<usize, Error> {

        Ok(self.rc.load(
                std::sync::atomic::Ordering::Acquire
        ))
    }

    pub fn incref (
        self: &ObjCore,
    ) -> Result<usize, Error> {

        let old_rc: usize = self.rc.fetch_add(
            1,
            std::sync::atomic::Ordering::Relaxed
            );

        if old_rc >= isize::MAX as usize {
            return Err(OverflowError(
                    "Reference counter overflow".to_string()
                    ));
        }

        return Ok(old_rc);
    }

    pub fn decref (
        self: &ObjCore,
    ) -> Result<usize, Error> {

        let old_rc: usize = self.rc.fetch_sub(
            1,
            std::sync::atomic::Ordering::Release
            );

        return Ok(old_rc);
    }

    pub fn is_locked (
        self: &ObjCore,
    ) -> Result<bool, Error> {

        if (self.lck.load(
                std::sync::atomic::Ordering::Acquire) != 0) {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn is_ex_locked (
        self: &ObjCore,
    ) -> Result<bool, Error> {

        if (self.lck.load(
                std::sync::atomic::Ordering::Acquire) == 1) {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn try_lock (
        self: &ObjCore,
    ) -> Result<bool, Error> {

        let mut oldlck: usize;
        loop {
            oldlck = self.lck.load(
                std::sync::atomic::Ordering::Acquire
                );
            if oldlck >= isize::MAX as usize {
                return Err(OverflowError(
                        "Lock counter overflow".to_string()
                        ));
            }
            if oldlck == 1 {
                return Ok(false);
            } else if oldlck % 2 != 0 {
                return Err(RuntimeError(
                        "Invalid lock value".to_string()
                ));
            }
            let result = self.lck.compare_exchange(
                oldlck,
                oldlck + 2,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Release,
            );
            match result {
                Ok(_value) => {
                    return Ok(true);
                },
                Err(_value) => {
                    continue;
                },
            }
        }
    }

    pub fn try_lock_ex (
        self: &ObjCore,
    ) -> Result<bool, Error> {

        let result = self.lck.compare_exchange(
            0,
            1,
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Release,
        );
        match result {
            Ok(_value) => {
                return Ok(true);
            },
            Err(_value) => {
                return Ok(false);
            },
        }
    }

    pub fn lock (
        self: &ObjCore,
    ) -> Result<(), Error> {

        loop {
            if self.try_lock()? {
                return Ok(());
            }
            std::thread::sleep(std::time::Duration::new(
                    0,
                    LOCK_NANO_SLEEP
                    ));
        }
    }

    pub fn lock_ex (
        self: &ObjCore,
    ) -> Result<(), Error> {

        loop {
            if self.try_lock_ex()? {
                return Ok(());
            }
            std::thread::sleep(std::time::Duration::new(0, LOCK_NANO_SLEEP));
        }
    }

    pub fn unlock (
        self: &ObjCore,
    ) -> Result<(), Error> {

        let mut oldlck: usize;
        let mut difference: usize;
        loop {
            oldlck = self.lck.load(std::sync::atomic::Ordering::Acquire);
            if oldlck == 0 {
                return Err(RuntimeError(
                        "Trying to unlock an unlocked mutex lock".to_string()
                ));
            } else if oldlck == 1 {
                difference = 1;
            } else if oldlck % 2 == 0 {
                difference = 2;
            } else {
                return Err(RuntimeError(
                        "Invalid lock value".to_string()
                ));
            }

            let result = self.lck.compare_exchange(
                oldlck,
                oldlck - difference,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Release,
            );
            match result {
                Ok(_value) => {
                    return Ok(());
                },
                Err(_value) => {
                    continue;
                },
            }
        }
    }
}

impl Clone for Object {
    fn clone (
        self: &Object,
    ) -> Object {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };

        // FIXME: in case incref returns error, return a none object
        inner.dk_incref().unwrap();

        Object { 
            data: self.data,
            data_type: self.data_type,
            phantom: std::marker::PhantomData,
            flags: self.flags,
        }
    }
}

impl Drop for Object {
    fn drop (
        self: &mut Object,
    ) {

        let inner: &Box<dyn DkAny> = unsafe { self.data.as_ref() };

        if inner.dk_decref().unwrap() != 1 {
            return;
        }

        std::sync::atomic::fence(std::sync::atomic::Ordering::Acquire);
        unsafe { Box::from_raw(self.data.as_ptr()); }
    }
}

unsafe impl Send for Object {}
unsafe impl Sync for Object {}

impl core::ops::Deref for ObjGuardMut<'_> {
    type Target = Box<dyn DkAny>;

    fn deref(&self) -> &Self::Target {
        let inner: &Box<dyn DkAny> = unsafe { self.data_obj.data.as_ref() };
        inner
    }
}

impl core::ops::DerefMut for ObjGuardMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let inner: &mut Box<dyn DkAny> = unsafe { self.data_obj.data.as_mut() };
        inner
    }
}

impl core::ops::Deref for ObjGuard<'_> {
    type Target = Box<dyn DkAny>;

    fn deref(&self) -> &Self::Target {
        let inner: &Box<dyn DkAny> = unsafe { self.data_obj.data.as_ref() };
        inner
    }
}

impl Drop for ObjGuardMut<'_> {
    fn drop(self: &mut Self) {
        let inner: &mut Box<dyn DkAny> = unsafe { self.data_obj.data.as_mut() };
        // TODO: use dk_set to set the new data
        inner.dk_unlock();
        // data object should drop by itself and call decref in process
    }
}

impl Drop for ObjGuard<'_> {
    fn drop(self: &mut Self) {
        let inner: &Box<dyn DkAny> = unsafe { self.data_obj.data.as_ref() };
        inner.dk_unlock();
        // data object should drop by itself and call decref in process
    }
}
