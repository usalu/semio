//! 📤️ `s.stdio.semio/v1/drawing` → `pdf` (1.7) — one page the size of the canvas carrying every
//! visible layer as real content-stream operators, in the painting model the svg and png leaves
//! share: a `Path` fills (non-zero) then strokes (butt caps, miter joins, limit 4), a node without a
//! style fills black, `opacity` becomes an ExtGState alpha, a `Group`'s transform is a `cm` inside a
//! `q`/`Q` pair, and `Text` is shown in Helvetica 12 pt at its anchor. Drawing space is y-down and
//! PDF space y-up, so one base `cm` flips the page and every coordinate is written verbatim.
//! Quadratic segments are raised to cubics exactly; arcs become at most one cubic per quarter turn.
//!
//! 🔖 Documented lossiness: `Image` nodes are not embedded, and text is limited to Helvetica/WinAnsi
//! with no font program embedded.
//!
//! @see https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as store;

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_drawing::schema::geometry::{arc_to_cubics, semio_transform_affine};
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use {semio_plugin::ArtifactSerializer,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_pdf_lower::standards::v1_7::subsets::base::schema::snapshot::{PdfExtGState, PdfFont, PdfLineCap, PdfLineJoin, PdfOp, PdfPage, PdfTextString};
use semio_s_artifact_stdio_pdf_lower::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId::ANY };
const TEXT_FONT: &str = "F1";
const TEXT_SIZE: f64 = 12.0;

struct Painter<'a> {
    styles: &'a [DrawStyle],
    ops: Vec<PdfOp>,
    states: Vec<PdfExtGState>,
    uses_text: bool,
}

impl Painter<'_> {
    fn alpha_state(&mut self, fill_alpha: f64, stroke_alpha: f64) -> Option<String> {
        if fill_alpha >= 1.0 && stroke_alpha >= 1.0 {
            return None;
        }
        if let Some(state) = self.states.iter().find(|s| s.fill_alpha == Some(fill_alpha) && s.stroke_alpha == Some(stroke_alpha)) {
            return Some(state.id.clone());
        }
        let id = format!("GS{}", self.states.len() + 1);
        self.states.push(PdfExtGState { id: id.clone(), fill_alpha: Some(fill_alpha), stroke_alpha: Some(stroke_alpha), ..PdfExtGState::default() });
        Some(id)
    }

    fn path(&mut self, segments: &[PathSegment]) -> bool {
        let mut pen = [0.0, 0.0];
        let mut start = [0.0, 0.0];
        let mut drew = false;
        for segment in segments {
            match segment {
                PathSegment::MoveTo { to } => {
                    pen = [to.x, to.y];
                    start = pen;
                    self.ops.push(PdfOp::MoveTo { x: to.x, y: to.y });
                }
                PathSegment::LineTo { to } => {
                    pen = [to.x, to.y];
                    drew = true;
                    self.ops.push(PdfOp::LineTo { x: to.x, y: to.y });
                }
                PathSegment::CubicTo { c1, c2, to } => {
                    pen = [to.x, to.y];
                    drew = true;
                    self.ops.push(PdfOp::CurveTo { x1: c1.x, y1: c1.y, x2: c2.x, y2: c2.y, x3: to.x, y3: to.y });
                }
                PathSegment::QuadTo { c, to } => {
                    let (c1, c2) = ([pen[0] + 2.0 / 3.0 * (c.x - pen[0]), pen[1] + 2.0 / 3.0 * (c.y - pen[1])], [to.x + 2.0 / 3.0 * (c.x - to.x), to.y + 2.0 / 3.0 * (c.y - to.y)]);
                    pen = [to.x, to.y];
                    drew = true;
                    self.ops.push(PdfOp::CurveTo { x1: c1[0], y1: c1[1], x2: c2[0], y2: c2[1], x3: to.x, y3: to.y });
                }
                PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => {
                    for [c1, c2, end] in arc_to_cubics(pen, *rx, *ry, *x_rotation, *large_arc, *sweep, [to.x, to.y]) {
                        self.ops.push(PdfOp::CurveTo { x1: c1[0], y1: c1[1], x2: c2[0], y2: c2[1], x3: end[0], y3: end[1] });
                    }
                    pen = [to.x, to.y];
                    drew = true;
                }
                PathSegment::Close => {
                    pen = start;
                    self.ops.push(PdfOp::ClosePath);
                }
            }
        }
        drew
    }

    fn node(&mut self, node: &DrawNode) {
        let styles = self.styles;
        let find = |name: &Option<String>| name.as_deref().and_then(|name| styles.iter().find(|s| s.name == name));
        let black = SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
        match node {
            DrawNode::Group { transform, children } => {
                self.ops.push(PdfOp::Save);
                self.ops.push(PdfOp::Transform { matrix: semio_transform_affine(transform) });
                for child in children {
                    self.node(child);
                }
                self.ops.push(PdfOp::Restore);
            }
            DrawNode::Path { segments, style } => {
                let (fill, stroke, width, opacity) = match find(style) {
                    Some(s) => (s.fill, s.stroke.filter(|_| s.stroke_width.unwrap_or(1.0) > 0.0), s.stroke_width.unwrap_or(1.0), f64::from(s.opacity.unwrap_or(1.0))),
                    None => (Some(black), None, 1.0, 1.0),
                };
                if fill.is_none() && stroke.is_none() {
                    return;
                }
                self.ops.push(PdfOp::Save);
                let fill_alpha = fill.map_or(1.0, |c| (f64::from(c.a) * opacity).clamp(0.0, 1.0));
                let stroke_alpha = stroke.map_or(1.0, |c| (f64::from(c.a) * opacity).clamp(0.0, 1.0));
                if let Some(state) = self.alpha_state(fill_alpha, stroke_alpha) {
                    self.ops.push(PdfOp::SetExtGState { name: state });
                }
                if let Some(c) = fill {
                    self.ops.push(PdfOp::SetFillRgb { r: f64::from(c.r), g: f64::from(c.g), b: f64::from(c.b) });
                }
                if let Some(c) = stroke {
                    self.ops.extend([PdfOp::SetStrokeRgb { r: f64::from(c.r), g: f64::from(c.g), b: f64::from(c.b) }, PdfOp::SetLineWidth { width }, PdfOp::SetLineCap { cap: PdfLineCap::Butt }, PdfOp::SetLineJoin { join: PdfLineJoin::Miter }, PdfOp::SetMiterLimit { limit: 4.0 }]);
                }
                let drew = self.path(segments);
                self.ops.push(match (drew, fill.is_some(), stroke.is_some()) {
                    (false, _, _) => PdfOp::EndPath,
                    (true, true, true) => PdfOp::FillStroke,
                    (true, true, false) => PdfOp::Fill,
                    (true, false, _) => PdfOp::Stroke,
                });
                self.ops.push(PdfOp::Restore);
            }
            DrawNode::Text { value, at, style } => {
                let (fill, opacity) = find(style).map_or((black, 1.0), |s| (s.fill.unwrap_or(black), f64::from(s.opacity.unwrap_or(1.0))));
                self.uses_text = true;
                self.ops.extend([PdfOp::Save, PdfOp::Transform { matrix: [1.0, 0.0, 0.0, -1.0, at.x, at.y] }]);
                if let Some(state) = self.alpha_state((f64::from(fill.a) * opacity).clamp(0.0, 1.0), 1.0) {
                    self.ops.push(PdfOp::SetExtGState { name: state });
                }
                self.ops.extend([
                    PdfOp::SetFillRgb { r: f64::from(fill.r), g: f64::from(fill.g), b: f64::from(fill.b) },
                    PdfOp::BeginText,
                    PdfOp::SetFont { name: TEXT_FONT.into(), size: TEXT_SIZE },
                    PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] },
                    PdfOp::ShowText { text: PdfTextString::text(value.clone()) },
                    PdfOp::EndText,
                    PdfOp::Restore,
                ]);
            }
            DrawNode::Image { .. } => {}
        }
    }
}

/// 📄️ The drawing as a one-page vector PDF (see the module doc for the painting model).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn drawing_to_pdf(drawing: &SemioDrawingSnapshot) -> Result<PdfSnapshot, String> {
    let (width, height) = (drawing.canvas.width, drawing.canvas.height);
    if !(width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0) {
        return Err("semio/drawing→pdf: the canvas has no positive finite size".into());
    }
    let mut painter = Painter { styles: &drawing.styles, ops: vec![PdfOp::Save, PdfOp::Transform { matrix: [1.0, 0.0, 0.0, -1.0, 0.0, height] }], states: Vec::new(), uses_text: false };
    if let Some(background) = &drawing.canvas.background {
        painter.ops.extend([PdfOp::SetFillRgb { r: f64::from(background.r), g: f64::from(background.g), b: f64::from(background.b) }, PdfOp::Rectangle { x: 0.0, y: 0.0, width, height }, PdfOp::Fill]);
    }
    for layer in drawing.layers.iter().filter(|layer| layer.visible) {
        painter.node(&layer.root);
    }
    painter.ops.push(PdfOp::Restore);
    let mut page = PdfPage::new(width, height);
    page.content = painter.ops.into_iter().collect();
    let mut snapshot = PdfSnapshot { pages: vec![page], ext_g_states: painter.states, ..PdfSnapshot::default() };
    if painter.uses_text {
        snapshot.fonts.push(PdfFont::standard(TEXT_FONT, "Helvetica"));
    }
    Ok(snapshot)
}

//#region 🔖️Serializer
pub struct SemioDrawingToPdf;

impl ArtifactSerializer for SemioDrawingToPdf {
    type From = SemioDrawingSnapshot;
    type Into = PdfSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        drawing_to_pdf(from).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
//#endregion 🔖️Serializer

//#region 🔌️Conversion
/// 🧾️ This leaf's composer row, built once.
pub fn entries() -> &'static [semio_plugin::io::ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_plugin::io::ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_plugin::serializer_entry_of::<SemioDrawingToPdf>()]).as_slice()
}

/// 🔌️ Registers and declares the `drawing` → pdf composer row; a composition root that offers pdf names it next to the
/// subset's own export conversion.
pub const CONVERSION: semio_s_artifact_stdio_semio_base::SemioConversion = semio_s_artifact_stdio_semio_base::SemioConversion { declare, register };

fn declare(builder: semio_plugin::app::ArtifactDeclarationBuilder<semio_plugin::app::DeclarationReady>) -> semio_plugin::app::ArtifactDeclarationBuilder<semio_plugin::app::DeclarationReady> {
    static COMPOSERS: std::sync::OnceLock<Vec<semio_plugin::io::ComposerEntry>> = std::sync::OnceLock::new();
    builder.composers(semio_s_artifact_stdio_semio_base::semio_written(entries(), &COMPOSERS))
}

fn register() {
    semio_plugin::io::register_composer_entries(entries()).expect("static Stdio registration must be available and conflict-free");
}
//#endregion 🔌️Conversion

//#region 📄️Encoding
/// 📄️ The file bytes of `drawing` as a PDF declaring `version` (empty: 1.7) — the one pdf writer every domain artifact
/// that projects into a drawing calls.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_drawing_pdf(drawing: &SemioDrawingSnapshot, version: &str) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::writer::{serialize_document, DocumentTrailer, WriteOptions};
    use semio_s_artifact_stdio_pdf_lower::standards::v1_7::subsets::base::modules::lower::{lower_document, LowerOptions};
    let version = if version.is_empty() { "1.7" } else { version };
    if !version.bytes().all(|byte| byte.is_ascii_digit() || byte == b'.') {
        return Err(semio_s_artifact_stdio_pdf_lower::standards::v1_7::subsets::base::io::error::PdfEngineError::Malformed("declared PDF version is not numeric".into()).to_string());
    }
    let mut pdf = drawing_to_pdf(drawing)?;
    pdf.declared_version = version.into();
    let document = lower_document(&pdf, 1, LowerOptions::default()).map_err(|error| error.to_string())?;
    let trailer = DocumentTrailer { root: document.root, info: document.info, id: pdf.document_id, extra: Vec::new() };
    Ok(serialize_document(version, &document.objects, &trailer, &WriteOptions::default()))
}
//#endregion 📄️Encoding

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
