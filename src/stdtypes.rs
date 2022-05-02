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



use crate::*;

pub const U8_type_id : usize = 0;

pub struct U8 {
    dk_obj_core: ObjCore,
    data: u8,
}

impl U8 {
    fn init () -> Type {
        Type {
            name: "u8".to_string(),
            tp_id: U8_type_id,
            generator: U8::dk_new,
            methods: Vec::new(),
            fields: Vec::new(),
            trait_implementations: Vec::new(),
            native_id: TypeId::of::<U8>(),
        }
    }
}

impl Clone for U8 {
    fn clone (
        self: &U8,
    ) -> U8 {

        U8 {
            dk_obj_core: ObjCore::new(),
            data: self.data,
        }
    }
}

impl DkGen for U8 {
    fn dk_new () -> Result<Box<dyn DkAny>, Error> {
        Ok(Box::new(U8 {
            dk_obj_core: ObjCore::new(),
            data: 0u8
        }))
    }
}

impl DkRefCount for U8 {
    fn dk_incref (
        self: &Self,
    ) -> Result<usize, Error> {

        self.dk_obj_core.incref()
    }

    fn dk_decref (
        self: &Self,
    ) -> Result<usize, Error> {

        self.dk_obj_core.decref()
    }
}

impl DkRWLock for U8 {
    fn dk_lock_ex (
        self: &Self,
    ) -> Result<(), Error> {

        self.dk_obj_core.lock_ex()
    }

    fn dk_try_lock_ex (
        self: &Self,
    ) -> Result<bool, Error> {

        self.dk_obj_core.try_lock_ex()
    }

    fn dk_lock (
        self: &Self,
    ) -> Result<(), Error> {

        self.dk_obj_core.lock()
    }

    fn dk_try_lock (
        self: &Self,
    ) -> Result<bool, Error> {

        self.dk_obj_core.try_lock()
    }

    fn dk_unlock (
        self: &Self,
    ) -> Result<(), Error> {

        self.dk_obj_core.unlock()
    }
}

impl DkGet for U8 {
    fn dk_get (
        self: &Self,
    ) -> Result<Box<dyn DkAny>, Error> {

        self.data.to_dk()
    }
}

impl DkSet for U8 {
    fn dk_set (
        self: &mut Self,
        new_data: &Box<dyn DkAny>,
    ) -> Result<(), Error> {

        let new_data_any: &dyn Any = &*new_data as &dyn Any;
        match new_data_any.downcast_ref::<U8>() {
            Some(new_data_object) => {
                self.data = new_data_object.data;
                return Ok(());
            },
            _ => {
                return Err(TypeError(
                        "Expected an object of type U8".to_string()
                ));
            },
        }
    }
}

impl DkDump for U8 {
    fn dk_dump (
        self: &Self,
    ) -> Result<Vec<u8>, Error> {

        let mut result: Vec<u8> = Vec::new();
        result.push(self.data);
        Ok(result)
    }
}

impl DkLoad for U8 {
    fn dk_load (
        self: &mut Self,
        new_data: Vec<u8>,
        cursor: &mut usize,
    ) -> Result<(), Error> {

        self.data = new_data[*cursor];
        *cursor += 1;
        Ok(())
    }
}

impl ToDk for u8 {
    fn to_dk (
        self: &Self,
    ) -> Result<Box<dyn DkAny>, Error> {
        Ok(Box::new(U8 {
            dk_obj_core: ObjCore::new(),
            data: *self,
        }))
    }

    fn to_dk_object (
        self: &Self,
    ) -> Result<Object, Error> {
        Ok(U8 {
            dk_obj_core: ObjCore::new(),
            data: *self,
        }.to_object(&U8_type, 3 << SIZE_SHIFT)?)
    }
}
