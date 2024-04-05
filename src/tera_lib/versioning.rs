use serde_json::value::{to_value, Value};
use std::collections::HashMap;
pub fn version(_: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    Ok(to_value(env!("VERGEN_GIT_DESCRIBE")).unwrap())
}
