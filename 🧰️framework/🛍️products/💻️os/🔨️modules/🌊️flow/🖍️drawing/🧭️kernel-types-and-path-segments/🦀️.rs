//! 🧭️ Kernel types and path segments of the drawing kernel: the drawing kind, handle, styles, affine transform, node, scene and the JSON form of path segments. See [`super`].

use super::*;

// #region 🔖️KernelTypes
/// 🧭️ Drawing entity kind carried by a handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum DrawingKind {
    Rect,
    Ellipse,
    Circle,
    Line,
    Polygon,
    Path,
    Text,
    Group,
}

/// 🧭️ Opaque content-addressed drawing handle (hex-encoded OS engine key).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DrawingHandle(pub String);

impl DrawingHandle {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 🔁️ Hand-written: tuple structs are a shape `#[derive(ToValue, FromValue)]` cannot express.
impl semio_framework_value::ToValue for DrawingHandle {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&self.0)
    }
}

impl semio_framework_value::FromValue for DrawingHandle {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Self(semio_framework_value::FromValue::from_value(value)?))
    }
}

/// 🎨️ Gradient color stop.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub struct GradientStop {
    pub offset: f64,
    pub color: [f64; 4],
}

/// 🪣️ Fill style.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum FillStyle {
    Solid { color: [f64; 4] },
    LinearGradient { x1: f64, y1: f64, x2: f64, y2: f64, stops: Vec<GradientStop> },
    RadialGradient { cx: f64, cy: f64, r: f64, stops: Vec<GradientStop> },
}

/// 🖌️ Stroke style.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub struct StrokeStyle {
    pub color: [f64; 4],
    pub width: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    #[serde(default)]
    #[value(default)]
    pub dash: Vec<f64>,
}

/// 🔚️ Line cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// 🔗️ Line join.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// 📐️ Affine 2D transform `[a,b,c,d,e,f]` mapping `(x,y)` to `(a*x+c*y+e, b*x+d*y+f)`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Affine2D(pub [f64; 6]);

/// 🔁️ Hand-written: tuple structs are a shape `#[derive(ToValue, FromValue)]` cannot express.
impl semio_framework_value::ToValue for Affine2D {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&self.0)
    }
}

impl semio_framework_value::FromValue for Affine2D {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Self(semio_framework_value::FromValue::from_value(value)?))
    }
}

impl Default for Affine2D {
    fn default() -> Self {
        Self([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }
}

impl Affine2D {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn translate(dx: f64, dy: f64) -> Self {
        Self([1.0, 0.0, 0.0, 1.0, dx, dy])
    }

    pub fn rotate(angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self([cos, sin, -sin, cos, 0.0, 0.0])
    }

    pub fn scale(sx: f64, sy: f64) -> Self {
        Self([sx, 0.0, 0.0, sy, 0.0, 0.0])
    }

    pub fn multiply(self, other: Self) -> Self {
        let [a, b, c, d, e, f] = self.0;
        let [a2, b2, c2, d2, e2, f2] = other.0;
        Self([a * a2 + c * b2, b * a2 + d * b2, a * c2 + c * d2, b * c2 + d * d2, a * e2 + c * f2 + e, b * e2 + d * f2 + f])
    }

    pub fn transform_point(self, point: semio_framework_2d::Vec2) -> semio_framework_2d::Vec2 {
        let [a, b, c, d, e, f] = self.0;
        [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
    }
}

/// 🧩️ Scene-graph node variants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DrawingNode {
    Rect { x: f64, y: f64, width: f64, height: f64 },
    Ellipse { cx: f64, cy: f64, rx: f64, ry: f64 },
    Circle { cx: f64, cy: f64, r: f64 },
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
    Polygon { points: Vec<semio_framework_2d::Vec2> },
    Path {
        #[serde(with = "path_segments")]
        segments: Vec<semio_framework_2d::PathSegment>,
    },
    Text { x: f64, y: f64, content: String, size: f64 },
    Group { children: Vec<String> },
}

/// 🖼️ Flattened scene node for preview and export.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneNode {
    pub transform: Affine2D,
    pub node: DrawingNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<StrokeStyle>,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "optional_path_segments")]
    pub clip: Option<Vec<semio_framework_2d::PathSegment>>,
}

fn default_opacity() -> f64 {
    1.0
}

/// 🎬️ Serializable drawing scene transfer type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrawingScene {
    pub width: f64,
    pub height: f64,
    pub nodes: Vec<SceneNode>,
}

impl Default for DrawingScene {
    fn default() -> Self {
        Self { width: 512.0, height: 512.0, nodes: Vec::new() }
    }
}
// #endregion 🔖️KernelTypes

// #region 🔖️PathSegmentJson
/// 🛤️ Serde conversion type of [`semio_framework_2d::PathSegment`]. This kernel is the only serde user of the shared
/// path vocabulary, so it owns the camelCase JSON shape (`kind` tag, `largeArc`) instead of the geometry kernel.
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(super) enum PathSegmentJson {
    Move { to: semio_framework_2d::Vec2 },
    Line { to: semio_framework_2d::Vec2 },
    Quad { ctrl: semio_framework_2d::Vec2, to: semio_framework_2d::Vec2 },
    Cubic { ctrl1: semio_framework_2d::Vec2, ctrl2: semio_framework_2d::Vec2, to: semio_framework_2d::Vec2 },
    Arc { rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool, to: semio_framework_2d::Vec2 },
    Close,
}

impl From<&semio_framework_2d::PathSegment> for PathSegmentJson {
    fn from(segment: &semio_framework_2d::PathSegment) -> Self {
        use semio_framework_2d::PathSegment as P;
        match *segment {
            P::Move { to } => Self::Move { to },
            P::Line { to } => Self::Line { to },
            P::Quad { ctrl, to } => Self::Quad { ctrl, to },
            P::Cubic { ctrl1, ctrl2, to } => Self::Cubic { ctrl1, ctrl2, to },
            P::Arc { rx, ry, rotation, large_arc, sweep, to } => Self::Arc { rx, ry, rotation, large_arc, sweep, to },
            P::Close => Self::Close,
        }
    }
}

impl From<PathSegmentJson> for semio_framework_2d::PathSegment {
    fn from(segment: PathSegmentJson) -> Self {
        match segment {
            PathSegmentJson::Move { to } => Self::Move { to },
            PathSegmentJson::Line { to } => Self::Line { to },
            PathSegmentJson::Quad { ctrl, to } => Self::Quad { ctrl, to },
            PathSegmentJson::Cubic { ctrl1, ctrl2, to } => Self::Cubic { ctrl1, ctrl2, to },
            PathSegmentJson::Arc { rx, ry, rotation, large_arc, sweep, to } => Self::Arc { rx, ry, rotation, large_arc, sweep, to },
            PathSegmentJson::Close => Self::Close,
        }
    }
}

/// 🧵️ `#[serde(with)]` codec of a path-segment list through [`PathSegmentJson`].
mod path_segments {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(segments: &[semio_framework_2d::PathSegment], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(segments.iter().map(super::PathSegmentJson::from))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<semio_framework_2d::PathSegment>, D::Error> {
        Ok(Vec::<super::PathSegmentJson>::deserialize(deserializer)?.into_iter().map(Into::into).collect())
    }
}

/// 🧵️ `#[serde(with)]` codec of an optional path-segment list through [`PathSegmentJson`].
pub(super) mod optional_path_segments {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(segments: &Option<Vec<semio_framework_2d::PathSegment>>, serializer: S) -> Result<S::Ok, S::Error> {
        segments.as_ref().map(|segments| segments.iter().map(super::PathSegmentJson::from).collect::<Vec<_>>()).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Vec<semio_framework_2d::PathSegment>>, D::Error> {
        Ok(Option::<Vec<super::PathSegmentJson>>::deserialize(deserializer)?.map(|segments| segments.into_iter().map(Into::into).collect()))
    }
}
// #endregion 🔖️PathSegmentJson
