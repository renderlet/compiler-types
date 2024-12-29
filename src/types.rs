use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    grammars: Grammars,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grammars {
    version: String,
    grammar: Vec<Grammar>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grammar {
    attr: Vec<Attr>,
    rule: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attr {
    name: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    name: String,
    op: Vec<Op>,
}

#[derive(Debug)]
pub enum Op {
    Center(Center),
    Color(Color),
    Comp(Vec<Comp>),
    Copy(Copy),
    CornerCut(CornerCut),
    Extrude(Extrude),
    Hemisphere(Hemisphere),
    InnerArch(InnerArch),
    InnerCircle(InnerCircle),
    InnerSemiCircle(InnerSemiCircle),
    Insert(Insert),
    Offset(Offset),
    Pyramid(Pyramid),
    RoofGable(RoofGable),
    RoofHip(RoofHip),
    Rotate(Rotate),
    SetupProjection(SetupProjection),
    ShapeL(ShapeL),
    ShapeU(ShapeU),
    Size(Size),
    Split(Split),
    Taper(Taper),
    Texture(Texture),
    Translate(Translate),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Center {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Copy {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CornerCut {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hemisphere {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerArch {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerCircle {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerSemiCircle {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Insert {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pyramid {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoofGable {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoofHip {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotate {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeL {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeU {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Taper {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translate {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Split {
    axis: EAxis,
    sizes: Vec<SizeDir>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupProjection {
    axes: EAxes,
    width: SizeDir,
    height: SizeDir,
    // m_executor_width: TExecutor,
    // m_executor_height: TExecutor,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    centered: bool,
    x: SizeDir,
    y: SizeDir,
    z: SizeDir,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeDir {
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat: Option<bool>, // TODO: not sure if this should be distinct from r#type
    r#type: SizeType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EAxis {
    X,
    Y,
    Z,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EAxes {
    #[serde(rename = "scope.xy")]
    ScopeXY,
    ScopeXZ,
    ScopeYX,
    ScopeYZ,
    ScopeZX,
    ScopeZY,
    WorldXY,
    WorldXZ,
    WorldYX,
    WorldYZ,
    WorldZX,
    WorldZY,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SizeType {
    Absolute,
    Relative,
    Floating,
    Repeat,
    RepeatRelative,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extrude {
    height: String, // TODO: should not be string
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comp {
    name: CompName,
    value: String, // TODO: should not be string
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompName {
    Front,
    Right,
    Left,
    Back,
    Side,
    Top,
    Bottom,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offset {
    distance: String, // TODO: should not be string
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<String>, // TODO: should not be string
    inside: String,   // TODO: should not be string
}

// pub enum Expression<T> {
//     Value(T),
//     // Variable
//     // Expression(??),
// }

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        let input = include_str!("../tests/a.yaml");

        let grammars: Main = serde_yaml::from_str(input).unwrap();
        let output = serde_yaml::to_string(&grammars).unwrap(); //.replace("  ", "    ");

        let output = format(&output);

        assert_eq!(input, output);
    }

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
}
