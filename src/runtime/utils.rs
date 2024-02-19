use boa_engine::{Context, JsResult, JsValue};

pub fn formatter(data: &[JsValue], context: &mut Context) -> JsResult<String> {
    match data {
        [] => Ok(String::new()),
        [val] => Ok(val.to_string(context)?.to_std_string_escaped()),
        data => {
            let mut formatted = String::new();
            for rest in data.iter() {
                formatted.push_str(&format!(
                    " {}",
                    rest.to_string(context)?.to_std_string_escaped()
                ));
            }

            Ok(formatted)
        }
    }
}
