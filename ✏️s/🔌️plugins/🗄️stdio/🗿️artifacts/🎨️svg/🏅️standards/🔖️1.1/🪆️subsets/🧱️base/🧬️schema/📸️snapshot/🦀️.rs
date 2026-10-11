//! 🧬️ SvgSnapshot schema — persistent fields + real codecs.

use crate::STDIO_SVG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr;
#[path="🧩️document/🦀️.rs"]
pub mod document;
pub use document::*;

//#region 🔖️Snapshot
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg")]
pub struct SvgSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub doc: SvgDocument,
}

impl Default for SvgSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: SvgDocument { root: Some(SvgNode::Element { name: "svg".into(), attrs: Vec::new(), children: Vec::new() }), doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() } }
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

    /// 🛡️ Verifies the shared natural SVG identity without materializing a second document.
    pub fn validate_natural(&self) -> Result<(), String> {
        if self.schema != STDIO_SVG_DOCUMENT_SCHEMA {
            return Err(format!("svg schema must be {STDIO_SVG_DOCUMENT_SCHEMA}"));
        }
        match &self.doc.root {
            Some(SvgNode::Element { name, .. }) if name == "svg" || name.ends_with(":svg") => Ok(()),
            Some(SvgNode::Element { .. }) => Err("root element must be svg".into()),
            _ => Err("svg document requires root element".into()),
        }
    }


}
//#endregion 🔖️SvgCodec

//#region 🔖️NumberGrammar






// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9

//#endregion 🔖️NumberGrammar

//#region 🔖️Geometry
/// 📐️ Parsed `viewBox="min-x min-y width height"`.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct ViewBox {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9




// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9

//#endregion 🔖️Geometry

//#region 🔖️Transform
/// ✖️ 2D affine matrix `[a c e; b d f; 0 0 1]`, matching SVG's `matrix(a,b,c,d,e,f)` layout.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

//#endregion 🔖️Transform

//#region 🔖️PathData
/// 🖊️ One command of the `d` attribute mini-language. `relative` distinguishes the lower-case
/// (relative-to-current-point) form from the upper-case (absolute) form -- both are kept typed
/// rather than pre-resolved to absolute coordinates, since resolving requires walking the whole
/// path with a running current-point/start-point state that belongs to a renderer, not the parser.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

//#endregion 🔖️PathData

//#region 🔖️Style
/// 🎨 The subset of presentation properties this artifact understands both as plain XML attributes
/// (`fill="red"`) and as `style="fill: red"` CSS-like declarations (style wins on conflict, matching
/// CSS cascade precedence over presentation attributes). `extra_style` losslessly retains any
/// `style=""` declaration this artifact doesn't specifically model, so a `style` attribute never
/// silently loses content it didn't recognize.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9

//#endregion 🔖️Style

//#region 🔖️CommonAttrs
/// 🧬 Attributes shared by (almost) every SVG element: `id`, `class`, `transform`, the
/// presentation-attribute subset, and an `extra_attrs` escape hatch for anything else on the
/// element this artifact doesn't specifically model (namespaced attrs, `xmlns`, custom `data-*`,
/// unrecognized presentation properties, ...) -- kept verbatim so nothing is ever silently dropped.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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

//#endregion 🔖️CommonAttrs

//#region 🔖️TypedElementModel


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


/// ✂️ Strips an XML namespace prefix (`xlink:href` -> `href`) for TYPED-ELEMENT DISPATCH ONLY;
/// `Unknown` and attribute passthrough always keep the original, fully-qualified name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


/// 🌳 Typed SVG 1.1 element tree. Elements outside this typed set (and any element this session
/// chose not to model in depth) fall into `Unknown` -- name/attrs/children kept byte-for-byte, so
/// parsing never drops or corrupts content outside the typed surface.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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


/// 🌳 Converts one generic (lossless) `SvgNode` into the typed SVG model. Dispatches on the local
/// (namespace-prefix-stripped) tag name against the typed set; anything else becomes `Unknown`
/// (with the ORIGINAL, still-prefixed name preserved) rather than being dropped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


/// 🌳 Lowers the typed model back into the generic (lossless) `SvgNode` tree that the xml codec's
/// text/binary writers already know how to serialize.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9

//#endregion 🔖️TypedElementModel

//#region 🔖️NodePath
/// 🧭 A child-index chain from the document root, used by the mutation vocabulary to address a
/// node inside `SvgSnapshot.doc` without needing the full typed model (mutations operate on the
/// persisted, always-lossless `SvgDocument`, not the typed view).
pub type NodePath = Vec<usize>;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn node_at<'a>(doc: &'a SvgDocument, path: &[usize]) -> Result<&'a SvgNode, String> {
    let mut node = doc.root.as_ref().ok_or("document has no root element")?;
    for &idx in path {
        match node {
            SvgNode::Element { children, .. } => {
                node = children.get(idx).ok_or_else(|| format!("child index {idx} out of range"))?;
            }
            _ => return Err("path descends into a non-element node".into()),
        }
    }
    Ok(node)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn node_at_mut<'a>(doc: &'a mut SvgDocument, path: &[usize]) -> Result<&'a mut SvgNode, String> {
    let mut node = doc.root.as_mut().ok_or("document has no root element")?;
    for &idx in path {
        match node {
            SvgNode::Element { children, .. } => {
                node = children.get_mut(idx).ok_or_else(|| format!("child index {idx} out of range"))?;
            }
            _ => return Err("path descends into a non-element node".into()),
        }
    }
    Ok(node)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn element_attr<'a>(node: &'a SvgNode, name: &str) -> Option<&'a SvgAttributeValue> {
    match node {
        SvgNode::Element { attrs, .. } => attrs.iter().find(|a| a.name == name).map(|a| &a.value),
        _ => None,
    }
}

/// 🏷️ Sets `name` to `value` (updating the existing attribute IN PLACE if present, so its position
/// in the attribute list is preserved -- only a genuinely new attribute gets appended); `None`
/// removes it. Update-in-place (rather than remove-then-append) matters for `SetAttribute`'s
/// apply/inverse round trip to reproduce the exact original attribute order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_element_attr(node: &mut SvgNode, name: &str, value: Option<SvgAttributeValue>) {
    if let SvgNode::Element { attrs, .. } = node {
        match value {
            Some(v) => match attrs.iter_mut().find(|a| a.name == name) {
                Some(existing) => existing.value = v,
                None => attrs.push(SvgAttr { name: name.to_string(), value: v }),
            },
            None => attrs.retain(|a| a.name != name),
        }
    }
}
//#endregion 🔖️NodePath

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests




//#endregion 🧪️Tests







