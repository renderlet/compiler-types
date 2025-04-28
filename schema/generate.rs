use compiler_types::Main;
use std::fs;

fn main() {
    let schema = schemars::schema_for!(Main);
    let output = serde_json::to_string_pretty(&schema).unwrap();
    fs::write("schema/schema.json", output).expect("Unable new schema");
}
