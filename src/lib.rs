use std::any::{Any, TypeId};

pub static API_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[macro_export]
macro_rules! export_plugin {
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

pub enum RuntimeError {
    Message { msg: String },
}

#[allow(dead_code)]
pub struct Function {
    name: String,
    number: u64,
    arg_types: Vec<TypeId>,
    return_type: TypeId,
}

#[allow(dead_code)]
pub struct Type {
    name: String,
    type_id: TypeId,
}

pub struct FreightDeclaration {
    pub rustc_version: &'static str,
    pub api_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn FreightRegistrar),
}

pub struct FreightProxy {
    freight: Box<dyn Freight>,
    lib: std::rc::Rc<libloading::Library>,
    name: String,
    version: String,
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

pub trait FreightRegistrar {
    fn register_freight(
        self: &mut Self,
        name: &str,
        version: &str,
        freight: Box<dyn Freight>,
        );
}

impl FreightProxy {
    pub unsafe fn load (self: &mut Self, lib_path: &str)
        -> std::io::Result<()> {

        self.lib = std::rc::Rc::new(
            libloading::Library::new(lib_path).unwrap());

        let declaration: FreightDeclaration = self.lib
            .get::<*mut FreightDeclaration>(b"freight_declaration\0")
            .unwrap()
            .read();

        if declaration.rustc_version != RUSTC_VERSION
            || declaration.api_version != API_VERSION
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Version mismatch",
            ));
        }

        (declaration.register)(self);

        Ok(())
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
