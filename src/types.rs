use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Main {
    grammars: Grammars,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Grammars {
    version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    exports: Vec<Export>,
    grammar: Vec<Grammar>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "serde_yaml::with::singleton_map_recursive::serialize")]
    #[serde(deserialize_with = "serde_yaml::with::singleton_map_recursive::deserialize")]
    shapes: Vec<Shape>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Export {
    name: String,
    r#type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    params: Vec<ExportParam>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExportParam {
    name: String,
    r#type: EType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum EType {
    F32,
    F64,
    I32,
    I64,
    C32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Shape {
    Polygon(Polygon),
    Rectangle(Rectangle),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Polygon {
    rule: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    ele: Option<f64>,
    // points: Vec<glam::Vec2>,
    points: Vec<(f64, f64)>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    skeleton: Option<Skeleton>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Rectangle {
    width: u32,
    height: u32,
    rule: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    // position: Option<glam::Vec3>,
    position: Option<(f64, f64, f64)>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Skeleton {
    offset: i32,
    indices: Vec<(i32, i32)>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Grammar {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    attr: Vec<Attr>,
    rule: Vec<Rule>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Attr {
    name: String,
    #[serde(flatten)]
    value: AttrValue,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<std::ops::Range<f64>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum AttrValue {
    #[serde(rename = "value")]
    Single(String),
    #[serde(rename = "values")]
    List(Vec<String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Rule {
    name: String,
    #[serde(serialize_with = "serde_yaml::with::singleton_map_recursive::serialize")]
    #[serde(deserialize_with = "serde_yaml::with::singleton_map_recursive::deserialize")]
    op: Vec<Op>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Op {
    Center(Center),
    Color(Color),
    Comp(Vec<Comp>),
    Copy(Copy),
    CornerCut(CornerCut),
    Extrude(Extrude),
    Hemisphere(Hemisphere),
    Hip(Hip),
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

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Center {
    axes_selector: EAxesSelector,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum EAxesSelector {
    XYZ,
    X,
    Y,
    Z,
    XY,
    XZ,
    YZ,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Copy {
    name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CornerCut {
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    r#type: ECornerCut,
    length: String,
}

#[derive(Debug, PartialEq, Default, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ECornerCut {
    #[default]
    Straight,
    Curve,
    NegativeCurve,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Hemisphere;

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Hip {
    angle: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InnerArch {
    // TODO: fill in fields
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InnerCircle;

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InnerSemiCircle;

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Insert {
    geometry_path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Pyramid {
    height: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RoofGable {
    angle: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RoofHip {
    angle: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Rotate {
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    x: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    y: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    z: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ShapeL {
    // TODO: fill in fields
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ShapeU {
    // TODO: fill in fields
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Taper {
    height: String,
    slope: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Translate {
    coord_system: ECoordSystem,
    mode: EMode,
    x: SizeDir,
    y: SizeDir,
    z: SizeDir,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum EMode {
    Absolute,
    Relative,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum ECoordSystem {
    World,
    Object,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Color {
    s: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Split {
    axis: EAxis,
    sizes: Vec<SizeDir>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Texture {
    path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SetupProjection {
    axes: EAxes,
    width: SizeDir,
    height: SizeDir,
    // m_executor_width: TExecutor,
    // m_executor_height: TExecutor,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Size {
    centered: bool,
    x: SizeDir,
    y: SizeDir,
    z: SizeDir,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SizeDir {
    value: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat: Option<bool>, // TODO: not sure if this should be distinct from r#type
    r#type: SizeType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum EAxis {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum SizeType {
    Absolute,
    Relative,
    Floating,
    Repeat,
    RepeatRelative,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Extrude {
    height: String, // TODO: should not be string
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Comp {
    name: CompName,
    value: String, // TODO: should not be string
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
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

fn is_default<T>(value: &T) -> bool
where
    T: Default + PartialEq + core::marker::Copy,
{
    value == &T::default()
}
