//! ✏️ Drawing artifact — document schema (the `2d.drawing` document type).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
// 🎭️ `fsm::statechart!` (used by `editor::drawing::commands::canvas_pointer_down`'s gesture machine) generates code
// containing `#[cfg(feature = "serde")]` gates meant for `fsm`'s OWN crate; macro hygiene splices
// that cfg check into the CALLING crate's feature list instead (a `fsm`/rustc macro-expansion
// limitation, not a real conditional-compilation bug here) — this crate declares no `serde` feature
// at all (the dependency is always-on), so rustc flags the value as unrecognized. Harmless, but a
// hard error under `-D warnings` without this crate-wide allow.

pub use store::ArtifactDsl;

pub const DRAWING_DOCUMENT_SCHEMA: &str = "drawing.document";
pub const DRAWING_BLEND_MODES: &[&str] = &["normal", "multiply", "screen", "overlay", "darken", "lighten", "colorDodge", "colorBurn", "hardLight", "softLight", "difference", "exclusion", "hue", "saturation", "color", "luminosity"];
pub const DRAWING_BOOLEAN_OPERATIONS: &[&str] = &["union", "difference", "intersection", "xor"];
pub const DRAWING_SHAPE_KINDS: &[&str] = &["rect", "ellipse", "circle", "line", "polygon"];
pub const DRAWING_UTILITY_IDS: &[&str] = &["selectMarquee", "selectLasso", "selectDirect", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"];

//#region 🔖️Domain
// No `#[dsl(keyword = ...)]` on `DrawingTransform`/`DrawingTraceParams`/`DrawingArtboard`: every field of
// these types is itself `#[dsl(block)]`, which already supplies the bare leading keyword from the
// FIELD's own name — an inner keyword too would double it (`transform { transform x=0 ... }`),
// same reasoning as `note`'s `NoteImageAsset`.
/// 🎥️ Camera pose (pan + zoom). Ephemeral view state owned by the `drawing` app runtime struct
/// (`DrawingConfig`), never a `DrawingSnapshot` field — see `.🧬semio/🦑️repo/🎫️tickets/26/07/31/
/// MOVE-DRAWING-PLUGIN-CAMERA-TO-RUNTIME-STATE`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for DrawingCamera {
    /// 🎯️ Matches the pre-migration `default_drawing_document` camera: centered on its 1024x1024 artboard.
    fn default() -> Self {
        Self { x: 512.0, y: 512.0, zoom: 0.75 }
    }
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingTransform {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    /// 📐️ Radians — `engine`'s compose/decompose matrix helpers call `.cos()`/`.sin()`
    /// directly on this field with no `to_radians()` conversion.
    #[dsl(angle = "rad")]
    pub rotation: f64,
}

// No keyword either: reached only through `Vec<GradientStop>` (a plain, un-tagged list) —
// `parse_record_body` self-terminates on the first unrecognized key regardless, the same reasoning
// verified for `note`'s `NoteImageAsset` nested inside a `Map` value slot.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct GradientStop {
    pub offset: f64,
    pub color: [f64; 4],
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
pub enum FillStyle {
    Solid {
        color: [f64; 4],
    },
    LinearGradient {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        #[dsl(table)]
        stops: Vec<GradientStop>,
    },
    RadialGradient {
        cx: f64,
        cy: f64,
        r: f64,
        #[dsl(table)]
        stops: Vec<GradientStop>,
    },
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct StrokeStyle {
    pub color: [f64; 4],
    pub width: f64,
    pub cap: String,
    pub join: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub dash: Option<Vec<f64>>,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingAttributes {
    // `fill` is a sum type (`FillStyle` has several tagged variants), so it uses
    // `#[dsl(statements, block)]` — see `dsl::DslVariants`'s doc comment on `OptionStatements`.
    // `stroke` is a single record type, so a plain `#[dsl(block)]` scalar Option suffices.
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(statements, block)]
    pub fill: Option<FillStyle>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub stroke: Option<StrokeStyle>,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingTraceParams {
    pub threshold: f64,
    pub simplify_epsilon: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingImageAsset {
    pub mime: String,
    pub data: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub width: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub height: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLayerBase {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    pub blend_mode: String,
    #[dsl(block)]
    pub transform: DrawingTransform,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(block)]
    pub attributes: DrawingAttributes,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingEllipse {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingCircle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingLine {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingPolygon {
    pub points: Vec<[f64; 2]>,
}

// Each body carries its own `#[dsl(keyword = ...)]` — required by the single-field tuple
// ("newtype") variants of `DrawingLayerNode` below, which delegate their entire `RecordSpec` to the
// inner body's own spec (see `dsl::__rt::newtype_variant_spec`) rather than wrapping it in one more
// layer. `base: DrawingLayerBase` carries BOTH `#[value(flatten)]` (splices into the JSON-shaped
// `ToValue`/`FromValue` tree) and `#[dsl(block)]` (the text/binary DSL grammar has no
// flatten-splice primitive; a bare nested `base { ... }` line is its declarative equivalent).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "shape")]
pub struct DrawingShapeBody {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub shape_kind: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub rect: Option<DrawingRect>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub ellipse: Option<DrawingEllipse>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub circle: Option<DrawingCircle>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub line: Option<DrawingLine>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub polygon: Option<DrawingPolygon>,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "path")]
pub struct DrawingPathBody {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    #[dsl(statements, block)]
    pub segments: Vec<PathSegment>,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "text")]
pub struct DrawingTextBody {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub x: f64,
    pub y: f64,
    pub content: String,
    pub size: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "image")]
pub struct DrawingImageBody {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub image_key: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "group")]
pub struct DrawingGroupBody {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    #[dsl(statements, block)]
    pub children: Vec<DrawingLayerNode>,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "boolean")]
pub struct DrawingBooleanBody {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub operation: String,
    pub children: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "trace")]
pub struct DrawingTraceBody {
    #[value(flatten)]
    #[cfg_attr(test, serde(flatten))]
    #[dsl(block)]
    pub base: DrawingLayerBase,
    pub source_key: String,
    #[dsl(block)]
    pub params: DrawingTraceParams,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind")]
#[cfg_attr(test, serde(tag = "kind"))]
pub enum DrawingLayerNode {
    #[value(rename = "shape")]
    #[cfg_attr(test, serde(rename = "shape"))]
    Shape(DrawingShapeBody),
    #[value(rename = "path")]
    #[cfg_attr(test, serde(rename = "path"))]
    Path(DrawingPathBody),
    #[value(rename = "text")]
    #[cfg_attr(test, serde(rename = "text"))]
    Text(DrawingTextBody),
    #[value(rename = "image")]
    #[cfg_attr(test, serde(rename = "image"))]
    Image(DrawingImageBody),
    #[value(rename = "group")]
    #[cfg_attr(test, serde(rename = "group"))]
    Group(DrawingGroupBody),
    #[value(rename = "boolean")]
    #[cfg_attr(test, serde(rename = "boolean"))]
    Boolean(DrawingBooleanBody),
    #[value(rename = "trace")]
    #[cfg_attr(test, serde(rename = "trace"))]
    Trace(DrawingTraceBody),
}

// 🖊️ Keywords/field order are a genuine SUBSET of SVG path data's absolute commands
// (`M`/`L`/`Q`/`C`/`A`/`Z`), each field `#[dsl(positional)]` so a segment prints as compact
// command-then-args tokens — `M 1.25,196.933 L 36.25,161.125 ... Z` — instead of `move to=1.25,196.933`.
// Field order per variant mirrors the SVG spec's own argument order (e.g. `A rx ry rotation
// large-arc-flag sweep-flag x,y`) so it reads as real SVG path syntax, just space- instead of
// comma/space-mixed-delimited between commands.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
pub enum PathSegment {
    #[dsl(key = "M")]
    Move {
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "L")]
    Line {
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "Q")]
    Quad {
        #[dsl(positional)]
        ctrl: [f64; 2],
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "C")]
    Cubic {
        #[dsl(positional)]
        ctrl1: [f64; 2],
        #[dsl(positional)]
        ctrl2: [f64; 2],
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "A")]
    Arc {
        #[dsl(positional)]
        rx: f64,
        #[dsl(positional)]
        ry: f64,
        /// 📐️ Degrees — `arc_segment_to_cubics`'s `rotation_deg` parameter calls `.to_radians()`
        /// on this value, matching SVG path data's `A rx ry x-axis-rotation ...` convention.
        #[dsl(positional)]
        #[dsl(angle = "deg")]
        rotation: f64,
        #[dsl(positional)]
        large_arc: bool,
        #[dsl(positional)]
        sweep: bool,
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "Z")]
    Close,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DrawingArtboard {
    pub width: f64,
    pub height: f64,
}

pub fn default_drawing_transform() -> DrawingTransform {
    DrawingTransform { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0, rotation: 0.0 }
}

pub fn default_drawing_trace_params() -> DrawingTraceParams {
    DrawingTraceParams { threshold: 0.5, simplify_epsilon: 1.5 }
}
pub use crate::schema::diff::DrawingDiff;
pub use crate::schema::mutations::DrawingMutation;
pub use crate::schema::snapshot::DrawingSnapshot;

//#endregion 🔖️Domain

//#region 🔖️ArtifactKind
/// 🏷️ The `2d.drawing` artifact kind declaration — lifted out of the old bundle manifest's
/// `.artifact_kind(...)` call so the app's manifest stitch can reuse it verbatim.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "2d.drawing".into(),
        name: "2D Drawing".into(),
        source_format: "drawing.document".into(),
        component_kind: "drawing".into(),
        dimension: "2d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        schema: "drawing.document".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.svg".into(), "stdio.png".into()],
        import_stdio_kinds: vec!["stdio.svg".into(), "stdio.png".into()],
    }
}

/// 🎯️ The `s.draw.drawing@1/*` surface dialect (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §2.1) — lives at the ARTIFACT level, not under `editor`/`viewer`, specifically so a
/// viewer file can read it without ever importing through the sibling editor module. `artifact_kind`
/// matches this file's own `definition()` capability row `"s.draw.schema.artifact"` → descriptor
/// `"s.draw.drawing"`; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const DRAWING_DIALECT: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: "s.draw.drawing", standard: semio_framework::StandardId("1"), subset: semio_framework::SubsetId::ANY };

/// 🔖️ This artifact's OLD capability-row definition (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) — kept per debt D1 (`📌️important.md`), not
/// deleted repo-wide until W6; `crate::editor::drawing::config::schema::register_app_schema()` is the
/// one exception, still called from `🖍️drawing/🦀️.rs`'s own `.setup()`: it registers the
/// `DrawingPlayApp` CONFIG/PRESENCE schema, an app-scope concern neither this nor the new declaration
/// tree (`artifact()`, below) has a field for. Superseded as the schema/io/surface registration
/// channel by `artifact()` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM) — this
/// function's only remaining reader is the `en`/`de` localized name pair (see `artifact()`'s own
/// `localization: &[]` doc).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.draw.drawing.standard.v1", "standard", "1", &[], None),
        ("s.draw.drawing.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.draw.drawing.schema.artifact", "schema", "s.draw.drawing", &[("schema", "s.draw.drawing")], None),
        ("s.draw.drawing.inference.artifact", "inference", "s.draw.drawing.inference", &[("schema", "s.draw.drawing.inference")], None),
        ("s.draw.drawing.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.draw.drawing.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.draw.drawing.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.draw.drawing.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.draw.drawing.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.draw.drawing.composer.dxf", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.draw.drawing.grammar.document", "grammar", "drawing.document", &[("grammar", "drawing.document")], None),
        ("s.draw.drawing.grammar.op", "grammar", "drawing.op", &[("grammar", "drawing.op")], None),
        ("s.draw.drawing.grammar.diff", "grammar", "drawing.diff", &[("grammar", "drawing.diff")], None),
        ("s.draw.drawing.grammar.pack", "grammar", "drawing.pack", &[("grammar", "drawing.pack")], None),
        ("s.draw.drawing.grammar.spr", "grammar", "drawing.spr", &[("grammar", "drawing.spr")], None),
        ("s.draw.drawing.codec.document.v1", "codec", "drawing.document:drawing", &[("codec", "drawing.document"), ("codec-extension", "16:drawing.document:drawing")], None),
        ("s.draw.drawing.localization.en", "localization", "Drawing", &[], Some(("en", "Drawing"))),
        ("s.draw.drawing.localization.de", "localization", "Zeichnung", &[], Some(("de", "Zeichnung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.draw.drawing")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🌳️ This artifact's declaration tree root (design.md §1/§2) — replaces the old `declaration()`
/// (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).languages(...)
/// .document_codec(...)` chain, deleted outright, no dual channel) as the ONLY registration channel
/// for schema/io/viewer/editor rows. `definition()` (old `ArtifactDefinition`/capability rows,
/// above) is kept per debt D1 — not deleted repo-wide until W6 — and `artifact_kind()` is kept
/// because `🦀️.rs`'s own `.activation(...)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME) still reads `artifact_kind().id`; neither has
/// any caller left in this function.
pub fn artifact<A: DrawingApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.draw.drawing").expect("canonical drawing kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait DrawingApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::drawing::DrawingPlayApp>>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::drawing::DrawingViewer>>>
{
}

impl<A> DrawingApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::drawing::DrawingPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::drawing::DrawingViewer>>>
{
}

//#endregion 🔖️ArtifactKind

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "🏅️standards/🔖️1/🦀️.rs"]
                mod v1_component;
                pub use v1_component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod any_component;
                        pub use any_component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs"]
                            pub mod owned;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            // 🚪️ Native codec facets (design.md §1 CORRECTION: unsplit, one `impl
                            // ArtifactDsl`/`ArtifactPack` per type, sits directly under `🚪️io/<facet>/
                            // <representation>/`, relocated from `🧬️schema/<facet>/<representation>/`).
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod structure {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod create_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/➕️create-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/➕️create-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/➕️create-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/➕️create-layer/🧪️tests/➕️appends-shape-b-0b0435/🦀️.rs"]
                                    mod tests_appends_shape_b_at_the_root;
                                }
                                #[path = "."]
                                pub mod delete_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/🚫️removes-group-a-41e1e0/🦀️.rs"]
                                    mod tests_removes_group_a_with_its_child;
                                }
                                #[path = "."]
                                pub mod duplicate_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/📋️duplicate-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/📋️duplicate-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/📋️duplicate-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/📋️duplicate-layer/🧪️tests/🚫️rejects-a-c88127/🦀️.rs"]
                                    mod tests_rejects_a_missing_source_layer;
                                }
                                #[path = "."]
                                pub mod reorder_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🔃reorder-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🔃reorder-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🔃reorder-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/🔃reorder-layer/🧪️tests/⬆️moves-shape-a-d7c515/🦀️.rs"]
                                    mod tests_moves_shape_a_above_shape_b;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod style {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod replace_layer_stroke {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🖊️replace-layer-stroke/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🖊️replace-layer-stroke/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🖊️replace-layer-stroke/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🖊️replace-layer-stroke/🧪️tests/🖊️adds-a-dashed-92bad7/🦀️.rs"]
                                    mod tests_adds_a_dashed_stroke;
                                }
                                #[path = "."]
                                pub mod replace_layer_fill {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🎨️replace-layer-fill/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🎨️replace-layer-fill/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🎨️replace-layer-fill/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🎨️replace-layer-fill/🧪️tests/🌈️solid-to-linear-9bdbe8/🦀️.rs"]
                                    mod tests_solid_to_linear_gradient;
                                }
                                #[path = "."]
                                pub mod set_layer_blend_mode {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🧪️tests/✖️normal-to-b12530/🦀️.rs"]
                                    mod tests_normal_to_multiply;
                                }
                                #[path = "."]
                                pub mod set_layer_opacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🎨️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🧪️tests/🌫️dims-shape-a-to-c25ad9/🦀️.rs"]
                                    mod tests_dims_shape_a_to_half;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transform {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod update_layer_transform {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔄️update-layer-transform/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔄️update-layer-transform/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔄️update-layer-transform/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔄️update-layer-transform/🧪️tests/📐️translates-and-f4316d/🦀️.rs"]
                                    mod tests_translates_and_scales_shape_a;
                                }
                                #[path = "."]
                                pub mod set_layer_boolean_operation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/🧪️tests/➖️union-to-subtract-845e1f/🦀️.rs"]
                                    mod tests_union_to_subtract;
                                }
                                #[path = "."]
                                pub mod update_layer_trace_params {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔍️update-layer-trace-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔍️update-layer-trace-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔍️update-layer-trace-params/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/🔍️update-layer-trace-params/🧪️tests/🔍️sharpens-the-117131/🦀️.rs"]
                                    mod tests_sharpens_the_trace;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod metadata {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod rename_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/✏️renames-shape-a-ba5eed/🦀️.rs"]
                                    mod tests_renames_shape_a_without_touching_its_id;
                                }
                                #[path = "."]
                                pub mod set_layer_visible {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🧪️tests/🙈️hides-shape-a/🦀️.rs"]
                                    mod tests_hides_shape_a;
                                }
                                #[path = "."]
                                pub mod set_layer_locked {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🧪️tests/🔒️locks-shape-a/🦀️.rs"]
                                    mod tests_locks_shape_a;
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::io::mutations::text::*;
            pub use crate::standards::v1::subsets::any::schema::mutations::{drawing_op_for_layer_field, patch_layer_field, DrawingMutation};
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
            pub use crate::standards::v1::subsets::any::schema::owned::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod drawing {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-layer/🦀️.rs"]
            pub mod add_layer;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️canvas-commit-draft/🦀️.rs"]
            pub mod canvas_commit_draft;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-double-click/🦀️.rs"]
            pub mod canvas_double_click;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚪️canvas-escape/🦀️.rs"]
            pub mod canvas_escape;
            #[path = "✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs"]
            pub mod canvas_pointer_move;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⬆️canvas-pointer-up/🦀️.rs"]
            pub mod canvas_pointer_up;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔀️combine-boolean/🦀️.rs"]
            pub mod combine_boolean;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📃️commit-document/🦀️.rs"]
            pub mod commit_document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-layer/🦀️.rs"]
            pub mod delete_layer;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️drop-layer-kind/🦀️.rs"]
            pub mod drop_layer_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-layer/🦀️.rs"]
            pub mod duplicate_layer;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-layer/🦀️.rs"]
            pub mod move_layer;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-layer/🦀️.rs"]
            pub mod patch_layer;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧵️patch-layers/🦀️.rs"]
            pub mod patch_layers;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪛️set-active-utility/🦀️.rs"]
            pub mod set_active_utility;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-camera-zoom/🦀️.rs"]
            pub mod set_camera_zoom;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧫️set-fixture-json/🦀️.rs"]
            pub mod set_fixture_json;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌫️set-selected-opacity/🦀️.rs"]
            pub mod set_selected_opacity;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📸️set-snapshot/🦀️.rs"]
            pub mod set_snapshot;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️toggle-layer-visible/🦀️.rs"]
            pub mod toggle_layer_visible;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs"]
                    pub mod canvas;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️layers/🦀️.rs"]
            pub mod layers;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs"]
            pub mod properties;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod drawing {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🦀️.rs"]
                    pub mod canvas;
                }
            }
        }
    }
}
