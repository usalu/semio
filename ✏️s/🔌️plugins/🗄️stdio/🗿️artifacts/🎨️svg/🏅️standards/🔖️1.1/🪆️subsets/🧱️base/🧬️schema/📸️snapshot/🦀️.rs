//! 🧬️ SvgSnapshot schema — persistent fields + real codecs.

use crate::STDIO_SVG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, xml_document_to_text_checked, XmlAttr, XmlDocument, XmlNode};

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg")]
pub struct SvgSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub doc: XmlDocument,
}

impl Default for SvgSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: XmlDocument { root: Some(XmlNode::Element { name: "svg".into(), attrs: Vec::new(), children: Vec::new() }), doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() } }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SvgCodec




impl SvgSnapshot {
    /// 🧠️ Returns the lossless logical SVG state used by diff and mutation laws.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn semantic_projection(&self) -> Self {
        self.clone()
    }

    /// 📥️ Parses SVG UTF-8 into its lossless logical XML model.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_utf8(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("svg source is not UTF-8: {error}"))?;
        Ok(Self { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: parse_svg_xml(text)? })
    }

    /// 🛡️ Verifies the shared natural SVG identity without materializing a second document.
    pub fn validate_natural(&self) -> Result<(), String> {
        if self.schema != STDIO_SVG_DOCUMENT_SCHEMA {
            return Err(format!("svg schema must be {STDIO_SVG_DOCUMENT_SCHEMA}"));
        }
        match &self.doc.root {
            Some(XmlNode::Element { name, .. }) if name == "svg" || name.ends_with(":svg") => Ok(()),
            Some(XmlNode::Element { .. }) => Err("root element must be svg".into()),
            _ => Err("svg document requires root element".into()),
        }
    }

    /// 📤️ Deterministically materializes SVG from the logical XML model.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_utf8(&self) -> Result<Vec<u8>, String> {
        self.validate_natural()?;
        Ok(xml_document_to_text_checked(&self.doc)?.into_bytes())
    }
}
//#endregion 🔖️SvgCodec

//#region 🔖️NumberGrammar






// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fmt_num(v: f64) -> String {
    v.to_string()
}
//#endregion 🔖️NumberGrammar

//#region 🔖️Geometry
/// 📐️ Parsed `viewBox="min-x min-y width height"`.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ViewBox {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn view_box_to_string(v: &ViewBox) -> String {
    format!("{} {} {} {}", fmt_num(v.min_x), fmt_num(v.min_y), fmt_num(v.width), fmt_num(v.height))
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn points_to_string(points: &[(f64, f64)]) -> String {
    points.iter().map(|(x, y)| format!("{},{}", fmt_num(*x), fmt_num(*y))).collect::<Vec<_>>().join(" ")
}
//#endregion 🔖️Geometry

//#region 🔖️Transform
/// ✖️ 2D affine matrix `[a c e; b d f; 0 0 1]`, matching SVG's `matrix(a,b,c,d,e,f)` layout.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Matrix2D {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Matrix2D {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn identity() -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 }
    }
    /// ✖️ Composes `self ∘ other` (apply `other` first, then `self`) -- matches SVG's
    /// left-to-right `transform="A B"` list semantics, where the combined matrix is `A * B`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn multiply(&self, other: &Matrix2D) -> Matrix2D {
        Matrix2D {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            e: self.a * other.e + self.c * other.f + self.e,
            f: self.b * other.e + self.d * other.f + self.f,
        }
    }
}

/// 📜 One entry of a `transform="..."` list. Kept as a typed op list (rather than collapsed
/// eagerly into a single matrix) so the original function-call structure round-trips; compose via
/// `transform_ops_to_matrix` whenever a single resolved affine matrix is needed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "op", rename_all = "camelCase")]
pub enum TransformOp {
    Matrix {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    },
    Translate {
        x: f64,
        #[value(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
    },
    Scale {
        x: f64,
        #[value(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
    },
    Rotate {
        angle: f64,
        #[value(default, skip_serializing_if = "Option::is_none")]
        center: Option<(f64, f64)>,
    },
    SkewX {
        angle: f64,
    },
    SkewY {
        angle: f64,
    },
}

impl TransformOp {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_matrix(&self) -> Matrix2D {
        match *self {
            TransformOp::Matrix { a, b, c, d, e, f } => Matrix2D { a, b, c, d, e, f },
            TransformOp::Translate { x, y } => Matrix2D { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: x, f: y.unwrap_or(0.0) },
            TransformOp::Scale { x, y } => Matrix2D { a: x, b: 0.0, c: 0.0, d: y.unwrap_or(x), e: 0.0, f: 0.0 },
            TransformOp::Rotate { angle, center } => {
                let (sin, cos) = angle.to_radians().sin_cos();
                let rot = Matrix2D { a: cos, b: sin, c: -sin, d: cos, e: 0.0, f: 0.0 };
                match center {
                    None => rot,
                    Some((cx, cy)) => {
                        let t1 = Matrix2D { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: cx, f: cy };
                        let t2 = Matrix2D { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: -cx, f: -cy };
                        t1.multiply(&rot).multiply(&t2)
                    }
                }
            }
            TransformOp::SkewX { angle } => Matrix2D { a: 1.0, b: 0.0, c: angle.to_radians().tan(), d: 1.0, e: 0.0, f: 0.0 },
            TransformOp::SkewY { angle } => Matrix2D { a: 1.0, b: angle.to_radians().tan(), c: 0.0, d: 1.0, e: 0.0, f: 0.0 },
        }
    }
}

/// ✖️ Composes an entire transform list into one resolved matrix (fold in list order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn transform_ops_to_matrix(ops: &[TransformOp]) -> Matrix2D {
    ops.iter().fold(Matrix2D::identity(), |acc, op| acc.multiply(&op.to_matrix()))
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn transform_list_to_string(ops: &[TransformOp]) -> String {
    ops.iter()
        .map(|op| match op {
            TransformOp::Matrix { a, b, c, d, e, f } => format!("matrix({},{},{},{},{},{})", fmt_num(*a), fmt_num(*b), fmt_num(*c), fmt_num(*d), fmt_num(*e), fmt_num(*f)),
            TransformOp::Translate { x, y: None } => format!("translate({})", fmt_num(*x)),
            TransformOp::Translate { x, y: Some(y) } => format!("translate({},{})", fmt_num(*x), fmt_num(*y)),
            TransformOp::Scale { x, y: None } => format!("scale({})", fmt_num(*x)),
            TransformOp::Scale { x, y: Some(y) } => format!("scale({},{})", fmt_num(*x), fmt_num(*y)),
            TransformOp::Rotate { angle, center: None } => format!("rotate({})", fmt_num(*angle)),
            TransformOp::Rotate { angle, center: Some((cx, cy)) } => format!("rotate({},{},{})", fmt_num(*angle), fmt_num(*cx), fmt_num(*cy)),
            TransformOp::SkewX { angle } => format!("skewX({})", fmt_num(*angle)),
            TransformOp::SkewY { angle } => format!("skewY({})", fmt_num(*angle)),
        })
        .collect::<Vec<_>>()
        .join(" ")
}
//#endregion 🔖️Transform

//#region 🔖️PathData
/// 🖊️ One command of the `d` attribute mini-language. `relative` distinguishes the lower-case
/// (relative-to-current-point) form from the upper-case (absolute) form -- both are kept typed
/// rather than pre-resolved to absolute coordinates, since resolving requires walking the whole
/// path with a running current-point/start-point state that belongs to a renderer, not the parser.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "cmd", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PathCommand {
    MoveTo { x: f64, y: f64, relative: bool },
    LineTo { x: f64, y: f64, relative: bool },
    HorizontalLineTo { x: f64, relative: bool },
    VerticalLineTo { y: f64, relative: bool },
    CurveTo { x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64, relative: bool },
    SmoothCurveTo { x2: f64, y2: f64, x: f64, y: f64, relative: bool },
    QuadraticCurveTo { x1: f64, y1: f64, x: f64, y: f64, relative: bool },
    SmoothQuadraticCurveTo { x: f64, y: f64, relative: bool },
    Arc { rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool, x: f64, y: f64, relative: bool },
    ClosePath,
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn path_data_to_string(cmds: &[PathCommand]) -> String {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn letter(base: char, relative: bool) -> char {
        if relative {
            base.to_ascii_lowercase()
        } else {
            base
        }
    }
    let mut parts = Vec::with_capacity(cmds.len());
    for cmd in cmds {
        parts.push(match cmd {
            PathCommand::MoveTo { x, y, relative } => format!("{} {} {}", letter('M', *relative), fmt_num(*x), fmt_num(*y)),
            PathCommand::LineTo { x, y, relative } => format!("{} {} {}", letter('L', *relative), fmt_num(*x), fmt_num(*y)),
            PathCommand::HorizontalLineTo { x, relative } => format!("{} {}", letter('H', *relative), fmt_num(*x)),
            PathCommand::VerticalLineTo { y, relative } => format!("{} {}", letter('V', *relative), fmt_num(*y)),
            PathCommand::CurveTo { x1, y1, x2, y2, x, y, relative } => {
                format!("{} {} {} {} {} {} {}", letter('C', *relative), fmt_num(*x1), fmt_num(*y1), fmt_num(*x2), fmt_num(*y2), fmt_num(*x), fmt_num(*y))
            }
            PathCommand::SmoothCurveTo { x2, y2, x, y, relative } => format!("{} {} {} {} {}", letter('S', *relative), fmt_num(*x2), fmt_num(*y2), fmt_num(*x), fmt_num(*y)),
            PathCommand::QuadraticCurveTo { x1, y1, x, y, relative } => format!("{} {} {} {} {}", letter('Q', *relative), fmt_num(*x1), fmt_num(*y1), fmt_num(*x), fmt_num(*y)),
            PathCommand::SmoothQuadraticCurveTo { x, y, relative } => format!("{} {} {}", letter('T', *relative), fmt_num(*x), fmt_num(*y)),
            PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative } => {
                format!("{} {} {} {} {} {} {} {}", letter('A', *relative), fmt_num(*rx), fmt_num(*ry), fmt_num(*x_axis_rotation), *large_arc as u8, *sweep as u8, fmt_num(*x), fmt_num(*y))
            }
            PathCommand::ClosePath => "Z".to_string(),
        });
    }
    parts.join(" ")
}
//#endregion 🔖️PathData

//#region 🔖️Style
/// 🎨 The subset of presentation properties this artifact understands both as plain XML attributes
/// (`fill="red"`) and as `style="fill: red"` CSS-like declarations (style wins on conflict, matching
/// CSS cascade precedence over presentation attributes). `extra_style` losslessly retains any
/// `style=""` declaration this artifact doesn't specifically model, so a `style` attribute never
/// silently loses content it didn't recognize.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PresentationAttrs {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke_opacity: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_style: Vec<(String, String)>,
}



/// ↩️ Returns `true` if `name` is a recognized presentation property (and was applied).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_presentation_attr(p: &mut PresentationAttrs, name: &str, value: &str) -> bool {
    match name {
        "fill" => p.fill = Some(value.to_string()),
        "stroke" => p.stroke = Some(value.to_string()),
        "stroke-width" => p.stroke_width = Some(value.to_string()),
        "opacity" => p.opacity = Some(value.to_string()),
        "fill-opacity" => p.fill_opacity = Some(value.to_string()),
        "stroke-opacity" => p.stroke_opacity = Some(value.to_string()),
        "font-family" => p.font_family = Some(value.to_string()),
        "font-size" => p.font_size = Some(value.to_string()),
        _ => return false,
    }
    true
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_presentation_attrs(attrs: &mut Vec<XmlAttr>, p: &PresentationAttrs) {
    let mut push = |name: &str, value: &Option<String>| {
        if let Some(v) = value {
            attrs.push(XmlAttr { name: name.to_string(), value: v.clone() });
        }
    };
    push("fill", &p.fill);
    push("stroke", &p.stroke);
    push("stroke-width", &p.stroke_width);
    push("opacity", &p.opacity);
    push("fill-opacity", &p.fill_opacity);
    push("stroke-opacity", &p.stroke_opacity);
    push("font-family", &p.font_family);
    push("font-size", &p.font_size);
    if !p.extra_style.is_empty() {
        let decls: Vec<String> = p.extra_style.iter().map(|(k, v)| format!("{k}: {v}")).collect();
        attrs.push(XmlAttr { name: "style".to_string(), value: decls.join("; ") });
    }
}
//#endregion 🔖️Style

//#region 🔖️CommonAttrs
/// 🧬 Attributes shared by (almost) every SVG element: `id`, `class`, `transform`, the
/// presentation-attribute subset, and an `extra_attrs` escape hatch for anything else on the
/// element this artifact doesn't specifically model (namespaced attrs, `xmlns`, custom `data-*`,
/// unrecognized presentation properties, ...) -- kept verbatim so nothing is ever silently dropped.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CommonAttrs {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<TransformOp>>,
    #[value(default)]
    pub presentation: PresentationAttrs,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_attrs: Vec<XmlAttr>,
}

impl CommonAttrs {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Self::default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn with_transform(mut self, ops: Vec<TransformOp>) -> Self {
        self.transform = Some(ops);
        self
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn with_fill(mut self, v: impl Into<String>) -> Self {
        self.presentation.fill = Some(v.into());
        self
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn with_stroke(mut self, v: impl Into<String>) -> Self {
        self.presentation.stroke = Some(v.into());
        self
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn with_stroke_width(mut self, v: impl Into<String>) -> Self {
        self.presentation.stroke_width = Some(v.into());
        self
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn with_opacity(mut self, v: impl Into<String>) -> Self {
        self.presentation.opacity = Some(v.into());
        self
    }
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_common_attrs(attrs: &mut Vec<XmlAttr>, common: &CommonAttrs) {
    if let Some(id) = &common.id {
        attrs.push(XmlAttr { name: "id".into(), value: id.clone() });
    }
    if let Some(class) = &common.class {
        attrs.push(XmlAttr { name: "class".into(), value: class.clone() });
    }
    if let Some(t) = &common.transform {
        attrs.push(XmlAttr { name: "transform".into(), value: transform_list_to_string(t) });
    }
    push_presentation_attrs(attrs, &common.presentation);
    attrs.extend(common.extra_attrs.iter().cloned());
}
//#endregion 🔖️CommonAttrs

//#region 🔖️TypedElementModel


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attr_f64(attrs: &[XmlAttr], name: &str, default: f64) -> Result<f64, String> {
    match attr_val(attrs, name) {
        None => Ok(default),
        Some(v) => v.trim().parse::<f64>().map_err(|_| format!("attribute '{name}' is not a number: '{v}'")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attr_f64_opt(attrs: &[XmlAttr], name: &str) -> Result<Option<f64>, String> {
    match attr_val(attrs, name) {
        None => Ok(None),
        Some(v) => v.trim().parse::<f64>().map(Some).map_err(|_| format!("attribute '{name}' is not a number: '{v}'")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attr_string_opt(attrs: &[XmlAttr], name: &str) -> Option<String> {
    attr_val(attrs, name).map(|s| s.to_string())
}

/// ✂️ Strips an XML namespace prefix (`xlink:href` -> `href`) for TYPED-ELEMENT DISPATCH ONLY;
/// `Unknown` and attribute passthrough always keep the original, fully-qualified name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

/// 🌳 Typed SVG 1.1 element tree. Elements outside this typed set (and any element this session
/// chose not to model in depth) fall into `Unknown` -- name/attrs/children kept byte-for-byte, so
/// parsing never drops or corrupts content outside the typed surface.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SvgElement {
    Svg {
        common: CommonAttrs,
        #[value(default, skip_serializing_if = "Option::is_none")]
        view_box: Option<ViewBox>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        width: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        height: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        xmlns: Option<String>,
        #[value(default)]
        children: Vec<SvgElement>,
    },
    Rect {
        common: CommonAttrs,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[value(default, skip_serializing_if = "Option::is_none")]
        rx: Option<f64>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        ry: Option<f64>,
    },
    Circle {
        common: CommonAttrs,
        cx: f64,
        cy: f64,
        r: f64,
    },
    Ellipse {
        common: CommonAttrs,
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
    },
    Line {
        common: CommonAttrs,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
    Polyline {
        common: CommonAttrs,
        points: Vec<(f64, f64)>,
    },
    Polygon {
        common: CommonAttrs,
        points: Vec<(f64, f64)>,
    },
    Path {
        common: CommonAttrs,
        d: Vec<PathCommand>,
    },
    Group {
        common: CommonAttrs,
        #[value(default)]
        children: Vec<SvgElement>,
    },
    Text {
        common: CommonAttrs,
        #[value(default, skip_serializing_if = "Option::is_none")]
        x: Option<f64>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
        #[value(default)]
        children: Vec<SvgElement>,
    },
    Tspan {
        common: CommonAttrs,
        #[value(default, skip_serializing_if = "Option::is_none")]
        x: Option<f64>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
        #[value(default)]
        children: Vec<SvgElement>,
    },
    Defs {
        common: CommonAttrs,
        #[value(default)]
        children: Vec<SvgElement>,
    },
    LinearGradient {
        common: CommonAttrs,
        #[value(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        x1: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        y1: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        x2: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        y2: Option<String>,
        #[value(default)]
        children: Vec<SvgElement>,
    },
    RadialGradient {
        common: CommonAttrs,
        #[value(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        cx: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        cy: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        r: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        fx: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        fy: Option<String>,
        #[value(default)]
        children: Vec<SvgElement>,
    },
    Stop {
        common: CommonAttrs,
        offset: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        stop_color: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        stop_opacity: Option<String>,
    },
    Use {
        common: CommonAttrs,
        href: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        x: Option<f64>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        width: Option<f64>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        height: Option<f64>,
    },
    /// 🚪 Escape hatch: any element name outside the typed set above, kept byte-for-byte.
    Unknown {
        name: String,
        #[value(default)]
        attrs: Vec<XmlAttr>,
        #[value(default)]
        children: Vec<SvgElement>,
    },
    TextNode(String),
    CData(String),
    Comment(String),
    ProcessingInstruction {
        target: String,
        data: String,
    },
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn convert_children(children: &[XmlNode]) -> Result<Vec<SvgElement>, String> {
    children.iter().map(svg_element_from_xml_node).collect()
}

/// 🌳 Converts one generic (lossless) `XmlNode` into the typed SVG model. Dispatches on the local
/// (namespace-prefix-stripped) tag name against the typed set; anything else becomes `Unknown`
/// (with the ORIGINAL, still-prefixed name preserved) rather than being dropped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn svg_element_from_xml_node(node: &XmlNode) -> Result<SvgElement, String> {
    match node {
        XmlNode::Text { text } => Ok(SvgElement::TextNode(text.clone())),
        XmlNode::CData { text } => Ok(SvgElement::CData(text.clone())),
        XmlNode::Comment { text } => Ok(SvgElement::Comment(text.clone())),
        XmlNode::ProcessingInstruction { target, data } => Ok(SvgElement::ProcessingInstruction { target: target.clone(), data: data.clone() }),
        XmlNode::Element { name, attrs, children } => match local_name(name) {
            "svg" => {
                let common = parse_common_attrs(attrs, &["viewBox", "width", "height", "xmlns"]);
                let view_box = match attr_val(attrs, "viewBox") {
                    Some(v) => Some(parse_view_box(v)?),
                    None => None,
                };
                Ok(SvgElement::Svg { common, view_box, width: attr_string_opt(attrs, "width"), height: attr_string_opt(attrs, "height"), xmlns: attr_string_opt(attrs, "xmlns"), children: convert_children(children)? })
            }
            "rect" => {
                let common = parse_common_attrs(attrs, &["x", "y", "width", "height", "rx", "ry"]);
                Ok(SvgElement::Rect {
                    common,
                    x: attr_f64(attrs, "x", 0.0)?,
                    y: attr_f64(attrs, "y", 0.0)?,
                    width: attr_f64(attrs, "width", 0.0)?,
                    height: attr_f64(attrs, "height", 0.0)?,
                    rx: attr_f64_opt(attrs, "rx")?,
                    ry: attr_f64_opt(attrs, "ry")?,
                })
            }
            "circle" => {
                let common = parse_common_attrs(attrs, &["cx", "cy", "r"]);
                Ok(SvgElement::Circle { common, cx: attr_f64(attrs, "cx", 0.0)?, cy: attr_f64(attrs, "cy", 0.0)?, r: attr_f64(attrs, "r", 0.0)? })
            }
            "ellipse" => {
                let common = parse_common_attrs(attrs, &["cx", "cy", "rx", "ry"]);
                Ok(SvgElement::Ellipse { common, cx: attr_f64(attrs, "cx", 0.0)?, cy: attr_f64(attrs, "cy", 0.0)?, rx: attr_f64(attrs, "rx", 0.0)?, ry: attr_f64(attrs, "ry", 0.0)? })
            }
            "line" => {
                let common = parse_common_attrs(attrs, &["x1", "y1", "x2", "y2"]);
                Ok(SvgElement::Line { common, x1: attr_f64(attrs, "x1", 0.0)?, y1: attr_f64(attrs, "y1", 0.0)?, x2: attr_f64(attrs, "x2", 0.0)?, y2: attr_f64(attrs, "y2", 0.0)? })
            }
            "polyline" => {
                let common = parse_common_attrs(attrs, &["points"]);
                let points = match attr_val(attrs, "points") {
                    Some(v) => parse_points(v)?,
                    None => Vec::new(),
                };
                Ok(SvgElement::Polyline { common, points })
            }
            "polygon" => {
                let common = parse_common_attrs(attrs, &["points"]);
                let points = match attr_val(attrs, "points") {
                    Some(v) => parse_points(v)?,
                    None => Vec::new(),
                };
                Ok(SvgElement::Polygon { common, points })
            }
            "path" => {
                let common = parse_common_attrs(attrs, &["d"]);
                let d = match attr_val(attrs, "d") {
                    Some(v) => parse_path_data(v)?,
                    None => Vec::new(),
                };
                Ok(SvgElement::Path { common, d })
            }
            "g" => Ok(SvgElement::Group { common: parse_common_attrs(attrs, &[]), children: convert_children(children)? }),
            "text" => {
                let common = parse_common_attrs(attrs, &["x", "y"]);
                Ok(SvgElement::Text { common, x: attr_f64_opt(attrs, "x")?, y: attr_f64_opt(attrs, "y")?, children: convert_children(children)? })
            }
            "tspan" => {
                let common = parse_common_attrs(attrs, &["x", "y"]);
                Ok(SvgElement::Tspan { common, x: attr_f64_opt(attrs, "x")?, y: attr_f64_opt(attrs, "y")?, children: convert_children(children)? })
            }
            "defs" => Ok(SvgElement::Defs { common: parse_common_attrs(attrs, &[]), children: convert_children(children)? }),
            "linearGradient" => {
                let common = parse_common_attrs(attrs, &["id", "x1", "y1", "x2", "y2"]);
                Ok(SvgElement::LinearGradient {
                    common,
                    id: attr_string_opt(attrs, "id"),
                    x1: attr_string_opt(attrs, "x1"),
                    y1: attr_string_opt(attrs, "y1"),
                    x2: attr_string_opt(attrs, "x2"),
                    y2: attr_string_opt(attrs, "y2"),
                    children: convert_children(children)?,
                })
            }
            "radialGradient" => {
                let common = parse_common_attrs(attrs, &["id", "cx", "cy", "r", "fx", "fy"]);
                Ok(SvgElement::RadialGradient {
                    common,
                    id: attr_string_opt(attrs, "id"),
                    cx: attr_string_opt(attrs, "cx"),
                    cy: attr_string_opt(attrs, "cy"),
                    r: attr_string_opt(attrs, "r"),
                    fx: attr_string_opt(attrs, "fx"),
                    fy: attr_string_opt(attrs, "fy"),
                    children: convert_children(children)?,
                })
            }
            "stop" => {
                let common = parse_common_attrs(attrs, &["offset", "stop-color", "stop-opacity"]);
                Ok(SvgElement::Stop { common, offset: attr_string_opt(attrs, "offset").unwrap_or_default(), stop_color: attr_string_opt(attrs, "stop-color"), stop_opacity: attr_string_opt(attrs, "stop-opacity") })
            }
            "use" => {
                let common = parse_common_attrs(attrs, &["href", "xlink:href", "x", "y", "width", "height"]);
                let href = attr_string_opt(attrs, "href").or_else(|| attr_string_opt(attrs, "xlink:href")).unwrap_or_default();
                Ok(SvgElement::Use { common, href, x: attr_f64_opt(attrs, "x")?, y: attr_f64_opt(attrs, "y")?, width: attr_f64_opt(attrs, "width")?, height: attr_f64_opt(attrs, "height")? })
            }
            _ => Ok(SvgElement::Unknown { name: name.clone(), attrs: attrs.clone(), children: convert_children(children)? }),
        },
    }
}

/// 🌳 Lowers the typed model back into the generic (lossless) `XmlNode` tree that the xml codec's
/// text/binary writers already know how to serialize.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn svg_element_to_xml_node(el: &SvgElement) -> XmlNode {
    match el {
        SvgElement::TextNode(t) => XmlNode::Text { text: t.clone() },
        SvgElement::CData(t) => XmlNode::CData { text: t.clone() },
        SvgElement::Comment(t) => XmlNode::Comment { text: t.clone() },
        SvgElement::ProcessingInstruction { target, data } => XmlNode::ProcessingInstruction { target: target.clone(), data: data.clone() },
        SvgElement::Svg { common, view_box, width, height, xmlns, children } => {
            let mut attrs = Vec::new();
            if let Some(vb) = view_box {
                attrs.push(XmlAttr { name: "viewBox".into(), value: view_box_to_string(vb) });
            }
            if let Some(w) = width {
                attrs.push(XmlAttr { name: "width".into(), value: w.clone() });
            }
            if let Some(h) = height {
                attrs.push(XmlAttr { name: "height".into(), value: h.clone() });
            }
            if let Some(x) = xmlns {
                attrs.push(XmlAttr { name: "xmlns".into(), value: x.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "svg".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Rect { common, x, y, width, height, rx, ry } => {
            let mut attrs =
                vec![XmlAttr { name: "x".into(), value: fmt_num(*x) }, XmlAttr { name: "y".into(), value: fmt_num(*y) }, XmlAttr { name: "width".into(), value: fmt_num(*width) }, XmlAttr { name: "height".into(), value: fmt_num(*height) }];
            if let Some(rx) = rx {
                attrs.push(XmlAttr { name: "rx".into(), value: fmt_num(*rx) });
            }
            if let Some(ry) = ry {
                attrs.push(XmlAttr { name: "ry".into(), value: fmt_num(*ry) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "rect".into(), attrs, children: vec![] }
        }
        SvgElement::Circle { common, cx, cy, r } => {
            let mut attrs = vec![XmlAttr { name: "cx".into(), value: fmt_num(*cx) }, XmlAttr { name: "cy".into(), value: fmt_num(*cy) }, XmlAttr { name: "r".into(), value: fmt_num(*r) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "circle".into(), attrs, children: vec![] }
        }
        SvgElement::Ellipse { common, cx, cy, rx, ry } => {
            let mut attrs = vec![XmlAttr { name: "cx".into(), value: fmt_num(*cx) }, XmlAttr { name: "cy".into(), value: fmt_num(*cy) }, XmlAttr { name: "rx".into(), value: fmt_num(*rx) }, XmlAttr { name: "ry".into(), value: fmt_num(*ry) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "ellipse".into(), attrs, children: vec![] }
        }
        SvgElement::Line { common, x1, y1, x2, y2 } => {
            let mut attrs = vec![XmlAttr { name: "x1".into(), value: fmt_num(*x1) }, XmlAttr { name: "y1".into(), value: fmt_num(*y1) }, XmlAttr { name: "x2".into(), value: fmt_num(*x2) }, XmlAttr { name: "y2".into(), value: fmt_num(*y2) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "line".into(), attrs, children: vec![] }
        }
        SvgElement::Polyline { common, points } => {
            let mut attrs = vec![XmlAttr { name: "points".into(), value: points_to_string(points) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "polyline".into(), attrs, children: vec![] }
        }
        SvgElement::Polygon { common, points } => {
            let mut attrs = vec![XmlAttr { name: "points".into(), value: points_to_string(points) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "polygon".into(), attrs, children: vec![] }
        }
        SvgElement::Path { common, d } => {
            let mut attrs = vec![XmlAttr { name: "d".into(), value: path_data_to_string(d) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "path".into(), attrs, children: vec![] }
        }
        SvgElement::Group { common, children } => {
            let mut attrs = Vec::new();
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "g".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Text { common, x, y, children } => {
            let mut attrs = Vec::new();
            if let Some(x) = x {
                attrs.push(XmlAttr { name: "x".into(), value: fmt_num(*x) });
            }
            if let Some(y) = y {
                attrs.push(XmlAttr { name: "y".into(), value: fmt_num(*y) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "text".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Tspan { common, x, y, children } => {
            let mut attrs = Vec::new();
            if let Some(x) = x {
                attrs.push(XmlAttr { name: "x".into(), value: fmt_num(*x) });
            }
            if let Some(y) = y {
                attrs.push(XmlAttr { name: "y".into(), value: fmt_num(*y) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "tspan".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Defs { common, children } => {
            let mut attrs = Vec::new();
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "defs".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::LinearGradient { common, id, x1, y1, x2, y2, children } => {
            let mut attrs = Vec::new();
            if let Some(id) = id {
                attrs.push(XmlAttr { name: "id".into(), value: id.clone() });
            }
            if let Some(v) = x1 {
                attrs.push(XmlAttr { name: "x1".into(), value: v.clone() });
            }
            if let Some(v) = y1 {
                attrs.push(XmlAttr { name: "y1".into(), value: v.clone() });
            }
            if let Some(v) = x2 {
                attrs.push(XmlAttr { name: "x2".into(), value: v.clone() });
            }
            if let Some(v) = y2 {
                attrs.push(XmlAttr { name: "y2".into(), value: v.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "linearGradient".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::RadialGradient { common, id, cx, cy, r, fx, fy, children } => {
            let mut attrs = Vec::new();
            if let Some(id) = id {
                attrs.push(XmlAttr { name: "id".into(), value: id.clone() });
            }
            if let Some(v) = cx {
                attrs.push(XmlAttr { name: "cx".into(), value: v.clone() });
            }
            if let Some(v) = cy {
                attrs.push(XmlAttr { name: "cy".into(), value: v.clone() });
            }
            if let Some(v) = r {
                attrs.push(XmlAttr { name: "r".into(), value: v.clone() });
            }
            if let Some(v) = fx {
                attrs.push(XmlAttr { name: "fx".into(), value: v.clone() });
            }
            if let Some(v) = fy {
                attrs.push(XmlAttr { name: "fy".into(), value: v.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "radialGradient".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Stop { common, offset, stop_color, stop_opacity } => {
            let mut attrs = vec![XmlAttr { name: "offset".into(), value: offset.clone() }];
            if let Some(v) = stop_color {
                attrs.push(XmlAttr { name: "stop-color".into(), value: v.clone() });
            }
            if let Some(v) = stop_opacity {
                attrs.push(XmlAttr { name: "stop-opacity".into(), value: v.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "stop".into(), attrs, children: vec![] }
        }
        SvgElement::Use { common, href, x, y, width, height } => {
            let mut attrs = vec![XmlAttr { name: "href".into(), value: href.clone() }];
            if let Some(x) = x {
                attrs.push(XmlAttr { name: "x".into(), value: fmt_num(*x) });
            }
            if let Some(y) = y {
                attrs.push(XmlAttr { name: "y".into(), value: fmt_num(*y) });
            }
            if let Some(w) = width {
                attrs.push(XmlAttr { name: "width".into(), value: fmt_num(*w) });
            }
            if let Some(h) = height {
                attrs.push(XmlAttr { name: "height".into(), value: fmt_num(*h) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "use".into(), attrs, children: vec![] }
        }
        SvgElement::Unknown { name, attrs, children } => XmlNode::Element { name: name.clone(), attrs: attrs.clone(), children: children.iter().map(svg_element_to_xml_node).collect() },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn svg_document_to_typed(doc: &XmlDocument) -> Result<SvgElement, String> {
    match &doc.root {
        Some(node) => svg_element_from_xml_node(node),
        None => Err("svg document has no root element".into()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn typed_to_svg_document(root: &SvgElement, doctype: Option<semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype>) -> XmlDocument {
    XmlDocument { root: Some(svg_element_to_xml_node(root)), doctype, declaration: None, prolog: Vec::new(), epilog: Vec::new() }
}
//#endregion 🔖️TypedElementModel

//#region 🔖️NodePath
/// 🧭 A child-index chain from the document root, used by the mutation vocabulary to address a
/// node inside `SvgSnapshot.doc` without needing the full typed model (mutations operate on the
/// persisted, always-lossless `XmlDocument`, not the typed view).
pub type NodePath = Vec<usize>;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn node_at<'a>(doc: &'a XmlDocument, path: &[usize]) -> Result<&'a XmlNode, String> {
    let mut node = doc.root.as_ref().ok_or("document has no root element")?;
    for &idx in path {
        match node {
            XmlNode::Element { children, .. } => {
                node = children.get(idx).ok_or_else(|| format!("child index {idx} out of range"))?;
            }
            _ => return Err("path descends into a non-element node".into()),
        }
    }
    Ok(node)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn node_at_mut<'a>(doc: &'a mut XmlDocument, path: &[usize]) -> Result<&'a mut XmlNode, String> {
    let mut node = doc.root.as_mut().ok_or("document has no root element")?;
    for &idx in path {
        match node {
            XmlNode::Element { children, .. } => {
                node = children.get_mut(idx).ok_or_else(|| format!("child index {idx} out of range"))?;
            }
            _ => return Err("path descends into a non-element node".into()),
        }
    }
    Ok(node)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn element_attr<'a>(node: &'a XmlNode, name: &str) -> Option<&'a str> {
    match node {
        XmlNode::Element { attrs, .. } => attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str()),
        _ => None,
    }
}

/// 🏷️ Sets `name` to `value` (updating the existing attribute IN PLACE if present, so its position
/// in the attribute list is preserved -- only a genuinely new attribute gets appended); `None`
/// removes it. Update-in-place (rather than remove-then-append) matters for `SetAttribute`'s
/// apply/inverse round trip to reproduce the exact original attribute order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_element_attr(node: &mut XmlNode, name: &str, value: Option<String>) {
    if let XmlNode::Element { attrs, .. } = node {
        match value {
            Some(v) => match attrs.iter_mut().find(|a| a.name == name) {
                Some(existing) => existing.value = v,
                None => attrs.push(XmlAttr { name: name.to_string(), value: v }),
            },
            None => attrs.retain(|a| a.name != name),
        }
    }
}
//#endregion 🔖️NodePath

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests



#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests






