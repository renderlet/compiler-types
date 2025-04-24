use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Main {
    grammars: Grammars,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Grammars {
    version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    exports: Vec<Export>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serde_yaml::with::singleton_map_recursive::serialize")]
    #[serde(deserialize_with = "serde_yaml::with::singleton_map_recursive::deserialize")]
    shapes: Vec<Shape>,
    grammar: Vec<Grammar>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Export {
    name: String,
    r#type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    params: Vec<ExportParam>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExportParam {
    name: String,
    r#type: EType,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum EType {
    F32,
    F64,
    I32,
    I64,
    C32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Shape {
    Polygon(Polygon),
    Rectangle(Rectangle),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Polygon {
    rule: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    ele: Option<f32>,
    // points: Vec<glam::Vec2>,
    points: Vec<(f32, f32)>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    skeleton: Option<Skeleton>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Rectangle {
    width: u32,
    height: u32,
    rule: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    // position: Option<glam::Vec3>,
    position: Option<(f32, f32, f32)>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Skeleton {
    offset: i32,
    indices: Vec<(i32, i32)>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Grammar {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    attr: Vec<Attr>,
    rule: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Attr {
    name: String,
    value: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<std::ops::Range<f32>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Rule {
    name: String,
    #[serde(serialize_with = "serde_yaml::with::singleton_map_recursive::serialize")]
    #[serde(deserialize_with = "serde_yaml::with::singleton_map_recursive::deserialize")]
    op: Vec<Op>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Center {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Copy {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CornerCut {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Hemisphere {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InnerArch {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InnerCircle {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InnerSemiCircle {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Insert {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Pyramid {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RoofGable {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RoofHip {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Rotate {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ShapeL {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ShapeU {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Taper {
    // TODO: fill in fields
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Translate {
    coord_system: ECoordSystem,
    mode: EMode,
    x: SizeDir,
    y: SizeDir,
    z: SizeDir,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum EMode {
    Absolute,
    Relative,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum ECoordSystem {
    World,
    Object,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Color {
    s: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Split {
    axis: EAxis,
    sizes: Vec<SizeDir>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Texture {
    path: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SetupProjection {
    axes: EAxes,
    width: SizeDir,
    height: SizeDir,
    // m_executor_width: TExecutor,
    // m_executor_height: TExecutor,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Size {
    centered: bool,
    x: SizeDir,
    y: SizeDir,
    z: SizeDir,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SizeDir {
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat: Option<bool>, // TODO: not sure if this should be distinct from r#type
    r#type: SizeType,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum EAxis {
    X,
    Y,
    Z,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum SizeType {
    Absolute,
    Relative,
    Floating,
    Repeat,
    RepeatRelative,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Extrude {
    height: String, // TODO: should not be string
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Comp {
    name: CompName,
    value: String, // TODO: should not be string
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum CompName {
    Front,
    Right,
    Left,
    Back,
    Side,
    Top,
    Bottom,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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

