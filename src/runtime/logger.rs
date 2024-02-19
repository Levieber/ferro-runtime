use crate::runtime::utils::formatter;
use boa_engine::{JsValue, NativeFunction};

pub fn prepare_logger() -> NativeFunction {
    unsafe {
        NativeFunction::from_closure(move |_, args, context| {
            println!("{}", formatter(args, context).unwrap());
            Ok(JsValue::undefined())
        })
    }
}
