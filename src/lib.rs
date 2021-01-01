use std::any::{Any, TypeId};

pub static API_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[macro_export]
macro_rules! export_freight {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static freight_declaration: $crate::FreightDeclaration
            = $crate::FreightDeclaration {
                rustc_version: $crate::RUSTC_VERSION,
                api_version: $crate::API_VERSION,
                register: $register,
            };
    };
}

#[derive(Debug)]
pub enum RuntimeError {
    Message { msg: &'static str },
}

pub struct Function {
    pub name: String,
    pub number: u64,
    pub arg_types: Vec<TypeId>,
    pub return_type: TypeId,
}

pub struct Type {
    pub name: String,
    pub type_id: TypeId,
}

pub struct FreightDeclaration {
    pub rustc_version: &'static str,
    pub api_version: &'static str,
    pub register: unsafe extern "C" fn (&mut dyn FreightRegistrar),
}

pub struct FreightProxy {
    pub freight: Box<dyn Freight>,
    pub lib: std::rc::Rc<libloading::Library>,
    pub name: String,
    pub version: String,
}

pub trait Freight {
    fn call_function (
        self: &mut Self,
        function_number: u64,
        args: Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, RuntimeError>;

    fn get_function_list(self: &mut Self) -> Vec<Function>;

    fn get_type_list(self: &mut Self) -> Vec<Type>;
}

struct EmptyFreight;
impl Freight for EmptyFreight {
    fn call_function (
        self: &mut Self,
        _function_number: u64,
        _args: Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, RuntimeError> {

        Err(RuntimeError::Message{
            msg: "You can't call an empty trait"
        })
    }

    fn get_function_list(self: &mut Self) -> Vec<Function> {
        Vec::new()
    }

    fn get_type_list(self: &mut Self) -> Vec<Type> {
        Vec::new()
    }
}

pub trait FreightRegistrar {
    fn register_freight(
        self: &mut Self,
        name: &str,
        version: &str,
        freight: Box<dyn Freight>,
        );
}

impl FreightProxy {
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
            name: "".to_string(),
            version: "".to_string(),
        };

        (declaration.register)(&mut result);

        Ok(result)
    }
}

impl Freight for FreightProxy {
    fn call_function (
        self: &mut Self,
        function_number: u64,
        args: Vec<&mut Box<dyn Any>>
        ) -> Result<Box<dyn Any>, RuntimeError> {
        self.freight.call_function(function_number, args)
    }

    fn get_function_list(self: &mut Self) -> Vec<Function> {
        self.freight.get_function_list()
    }

    fn get_type_list(self: &mut Self) -> Vec<Type> {
        self.freight.get_type_list()
    }
}

impl FreightRegistrar for FreightProxy {
    fn register_freight(
        self: &mut Self,
        name: &str,
        version: &str,
        freight: Box<dyn Freight>,
        ) {
        self.freight = freight;
        self.name = name.to_string();
        self.version = version.to_string();
    }
}
