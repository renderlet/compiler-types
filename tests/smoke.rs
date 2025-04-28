use similar_asserts::assert_eq;

use compiler_types::Main;

fn format(input: &str) -> String {
    use yaml_rust::{YamlEmitter, YamlLoader};
    let docs = YamlLoader::load_from_str(input).unwrap();
    let mut ouput = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut ouput);
        emitter.dump(&docs[0]).unwrap();
    }
    ouput.to_string()
}

fn test(path: &str) {
    let path = format!("./tests/{path}.yaml");
    let input = std::fs::read_to_string(path).unwrap();

    let grammars: Main = serde_yaml::from_str(&input).unwrap();
    let output = serde_yaml::to_string(&grammars).unwrap();

    let output = format(&output);

    assert_eq!(input, output);
}

#[test]
fn a() {
    test("a");
}

#[test]
fn b() {
    test("b");
}

#[test]
fn c() {
    test("c");
}

#[test]
fn d() {
    test("d");
}
