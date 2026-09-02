//! ✏️ Drawing artifact — document schema (the `2d.drawing` document type).

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
/// (`DrawingConfig`), never a `DrawingSnapshot` field — see `.🦑️repo/🎫️tickets/26/07/31/
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
pub use crate::artifacts::drawing::schema::diff::DrawingDiff;
pub use crate::artifacts::drawing::schema::mutations::DrawingMutation;
pub use crate::artifacts::drawing::schema::snapshot::DrawingSnapshot;

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
        export_stdio_kinds: vec!["stdio.svg", "stdio.png"],
        import_stdio_kinds: vec!["stdio.svg", "stdio.png"],
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
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.draw.standard.v1", "standard", "1", &[], None),
        ("s.draw.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.draw.schema.artifact", "schema", "s.draw.drawing", &[("schema", "s.draw.drawing")], None),
        ("s.draw.inference.artifact", "inference", "s.draw.drawing.inference", &[("schema", "s.draw.drawing.inference")], None),
        ("s.draw.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.draw.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.draw.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.draw.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.draw.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.draw.composer.dxf", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.draw.grammar.document", "grammar", "drawing.document", &[("grammar", "drawing.document")], None),
        ("s.draw.grammar.op", "grammar", "drawing.op", &[("grammar", "drawing.op")], None),
        ("s.draw.grammar.diff", "grammar", "drawing.diff", &[("grammar", "drawing.diff")], None),
        ("s.draw.grammar.pack", "grammar", "drawing.pack", &[("grammar", "drawing.pack")], None),
        ("s.draw.grammar.spr", "grammar", "drawing.spr", &[("grammar", "drawing.spr")], None),
        ("s.draw.codec.document.v1", "codec", "drawing.document:drawing", &[("codec", "drawing.document"), ("extension", "drawing")], None),
        ("s.draw.localization.en", "localization", "Drawing", &[], Some(("en", "Drawing"))),
        ("s.draw.localization.de", "localization", "Zeichnung", &[], Some(("de", "Zeichnung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.draw")?);
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
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::DrawApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.draw.drawing").expect("canonical drawing kind"), localization: &[], standards: vec![crate::artifacts::drawing::standards::v1::standard()] }
}
//#endregion 🔖️ArtifactKind
