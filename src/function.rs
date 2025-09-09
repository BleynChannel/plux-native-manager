use std::{marker::PhantomData, sync::Arc};

use plux_rs::{
    function::{Arg, Function, FunctionOutput},
    variable::Variable,
};
use libloading::{Library, Symbol};

pub struct NativeFunction<T: Send + Sync> {
    library: Arc<Library>,
    name: String,
    inputs: Vec<Arg>,
    output: Option<Arg>,
    phantom_data: PhantomData<T>,
}

impl<T: Send + Sync> NativeFunction<T> {
    pub fn new<S>(library: Arc<Library>, name: S, inputs: Vec<Arg>, output: Option<Arg>) -> Self
    where
        S: Into<String>,
    {
        Self {
            library: library.clone(),
            name: name.into(),
            inputs,
            output,
            phantom_data: PhantomData {},
        }
    }
}

impl<T: Send + Sync> Function for NativeFunction<T> {
    type Output = FunctionOutput;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn inputs(&self) -> Vec<Arg> {
        self.inputs.clone()
    }

    fn output(&self) -> Option<Arg> {
        self.output.clone()
    }

    fn call(&self, args: &[Variable]) -> Self::Output {
        // let a = args[0].try_parse_ref::<u32>()?;
        // let b = args[1].try_parse_ref::<u32>()?;

        // let f: Symbol<T> = unsafe { self.library.get(self.name.as_bytes()).unwrap() };
        // let result = unsafe { f(a.clone(), b.clone()) };

        // Ok(Some(Variable::U32(result as u32)))
		
		Ok(None)
    }
}

// #[test]
// fn test_request() {
//     let library = Arc::new(unsafe { libloading::Library::new("target/debug/libnative_manager.so").unwrap() });
//     let request = NativeFunction::<fn(u32, u32) -> u32>::new(library, "add", vec![Arg::U32, Arg::U32], Some(Arg::U32));

//     let result = request.call(&[Variable::U32(1), Variable::U32(2)]).unwrap();
//     assert_eq!(result.unwrap(), Variable::U32(3));
// }