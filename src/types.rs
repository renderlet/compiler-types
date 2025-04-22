use crate::custom_formats;
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    exports: Vec<Export>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    shapes: Vec<Shape>,
    grammar: Vec<Grammar>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Export {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shape {}

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
    #[serde(with = "custom_formats::option_range")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<std::ops::Range<f32>>,
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
pub struct Center {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Copy {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CornerCut {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hemisphere {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerArch {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerCircle {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerSemiCircle {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Insert {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pyramid {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoofGable {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoofHip {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotate {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeL {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeU {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Taper {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translate {
    coord_system: ECoordSystem,
    mode: EMode,
    x: SizeDir,
    y: SizeDir,
    z: SizeDir,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EMode {
    Absolute,
    Relative,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum ECoordSystem {
    World,
    Object,
}

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

