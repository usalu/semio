//! 🚪️ IO s.note (1/✳️any) — registration now flows through 🎹️composer::register, called once from
//! this plugin's `.setup()` (see the artifact root's `declaration()`), not per-leaf register().

use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_framework::{DwgDrawing, DwgGeometry};
use semio_framework_plugin::{io_dispatch, Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io as semio_drawing_composer;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::write_svg_xml;
use serde_json::Value;

pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"] }
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::note::NoteSnapshot;
    use crate::artifacts::note::standards::v1::subsets::any::schema::NoteAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.note", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_DXF: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_PDF: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_SVG: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };


    pub struct NoteComposerComposition;

    impl ArtifactComposition for NoteComposerComposition {
        type Snapshot = NoteSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_DXF, DEP_JSON, DEP_PDF, DEP_PNG, DEP_SVG]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = NoteAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::note::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_DXF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::note::io::import::deserializers::artifacts::dxf::v_r12::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::note::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PDF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::note::io::import::deserializers::artifacts::pdf::v1_4::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::note::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_SVG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::note::io::import::deserializers::artifacts::svg::v1_1::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }

            }
            Err(ComposeError { message: "NoteComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️MediaExport
pub fn note_document_bounds(document: &NoteSnapshot) -> (u32, u32) {
    let mut max_x = 1024.0_f64;
    let mut max_y = 1024.0_f64;
    for block in crate::artifacts::note::schema::flatten_blocks(&document.blocks) {
        if !crate::artifacts::note::schema::block_visible(block) {
            continue;
        }
        let (x, y, width, height) = match block {
            NoteBlockNode::Text { x, y, width, height, .. }
            | NoteBlockNode::Image { x, y, width, height, .. }
            | NoteBlockNode::Table { x, y, width, height, .. }
            | NoteBlockNode::Math { x, y, width, height, .. }
            | NoteBlockNode::Ink { x, y, width, height, .. }
            | NoteBlockNode::Group { x, y, width, height, .. } => (*x, *y, *width, *height),
        };
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

/// 🧭️ Position/rotation of any block variant, lifted into a semio `SemioTransform` (Z-axis-only
/// rotation, matching the drawing subset's own svg-bridge convention — see its `matrix_to_semio_transform`).
fn note_block_transform(block: &NoteBlockNode) -> SemioTransform {
    let (x, y, rotation) = match block {
        NoteBlockNode::Text { x, y, rotation, .. }
        | NoteBlockNode::Image { x, y, rotation, .. }
        | NoteBlockNode::Table { x, y, rotation, .. }
        | NoteBlockNode::Math { x, y, rotation, .. }
        | NoteBlockNode::Ink { x, y, rotation, .. }
        | NoteBlockNode::Group { x, y, rotation, .. } => (*x, *y, *rotation),
    };
    let theta = rotation.to_radians();
    SemioTransform {
        translation: SemioPoint3 { x, y, z: 0.0 },
        rotation: SemioQuaternion { x: 0.0, y: 0.0, z: (theta / 2.0).sin(), w: (theta / 2.0).cos() },
        scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 },
    }
}

/// ▭ The outline-rectangle `PathSegment`s the deleted `note_block_to_svg` drew for its
/// image-without-asset and Table/Math/Group catch-all cases.
fn note_outline_rect_segments(width: f64, height: f64) -> Vec<PathSegment> {
    vec![
        PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } },
        PathSegment::LineTo { to: SemioPoint2 { x: width, y: 0.0 } },
        PathSegment::LineTo { to: SemioPoint2 { x: width, y: height } },
        PathSegment::LineTo { to: SemioPoint2 { x: 0.0, y: height } },
        PathSegment::Close,
    ]
}

/// 🎨️ Always-append style intern, mirroring semio/drawing's own svg-import `intern_style`
/// convention (see that bridge's module doc) — one named `DrawStyle` per call, referenced back by name.
fn note_intern_style(styles: &mut Vec<DrawStyle>, fill: Option<SemioRgba>, stroke: Option<SemioRgba>, stroke_width: Option<f64>) -> String {
    let name = format!("note-style-{}", styles.len());
    styles.push(DrawStyle { name: name.clone(), fill, stroke, stroke_width, opacity: None });
    name
}

/// 🔤️ Minimal, dependency-free base64 decoder (this repo's "no external libraries for runtime
/// purposes" rule — mirrors semio/drawing's own svg-import `base64_decode`) — unwraps a
/// `NoteImageAsset.data` `data:<mime>;base64,<payload>` URI into the raw bytes `DrawNode::Image`
/// needs.
fn note_asset_data_uri_bytes(data_uri: &str) -> Vec<u8> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let payload = data_uri.split_once(',').map(|(_, rest)| rest).unwrap_or(data_uri);
    let clean: Vec<u8> = payload.bytes().filter(|&b| b != b'=' && !b.is_ascii_whitespace()).collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    for chunk in clean.chunks(4) {
        let vals: Vec<u8> = match chunk.iter().map(|&b| val(b)).collect::<Option<Vec<u8>>>() {
            Some(vals) => vals,
            None => return out,
        };
        let n = vals.len();
        let combined = vals.iter().fold(0u32, |acc, &v| (acc << 6) | v as u32) << ((4 - n) * 6);
        out.push((combined >> 16) as u8);
        if n > 2 { out.push((combined >> 8) as u8); }
        if n > 3 { out.push(combined as u8); }
    }
    out
}

/// 🖍️ Maps one note block into a `DrawNode` (wrapped in a `Group` carrying its position/rotation
/// `SemioTransform`) — the real per-block domain mapping that replaces the deleted hand-rolled
/// `note_block_to_svg` SVG string emission. Text/Image/Ink map onto their natural `DrawNode`
/// counterpart; Table/Math/Group (no scene-graph equivalent in this subset) fall back to the same
/// plain outline rectangle the deleted code drew for its catch-all case.
fn draw_node_from_note_block(block: &NoteBlockNode, document: &NoteSnapshot, styles: &mut Vec<DrawStyle>) -> Option<DrawNode> {
    let transform = note_block_transform(block);
    let inner = match block {
        NoteBlockNode::Text { paragraphs, font_size, .. } => {
            let text = paragraphs.iter().map(|paragraph| paragraph.runs.iter().map(|run| run.text.as_str()).collect::<Vec<_>>().join("")).collect::<Vec<_>>().join("\n");
            DrawNode::Text { value: text, at: SemioPoint2 { x: 0.0, y: *font_size }, style: None }
        }
        NoteBlockNode::Image { image_key, width, height, .. } => match document.assets.get(image_key) {
            Some(asset) => DrawNode::Image { at: SemioPoint2::default(), width: *width, height: *height, mime: asset.mime.clone(), bytes: note_asset_data_uri_bytes(&asset.data) },
            None => {
                let style = note_intern_style(styles, None, Some(SemioRgba { r: 0.53, g: 0.53, b: 0.53, a: 1.0 }), Some(1.0));
                DrawNode::Path { segments: note_outline_rect_segments(*width, *height), style: Some(style) }
            }
        },
        NoteBlockNode::Ink { points, stroke_width, color, .. } => {
            if points.len() < 2 {
                return None;
            }
            let mut segments = vec![PathSegment::MoveTo { to: SemioPoint2 { x: points[0][0], y: points[0][1] } }];
            segments.extend(points.iter().skip(1).map(|point| PathSegment::LineTo { to: SemioPoint2 { x: point[0], y: point[1] } }));
            let stroke = SemioRgba { r: color[0] as f32, g: color[1] as f32, b: color[2] as f32, a: color[3] as f32 };
            let style = note_intern_style(styles, None, Some(stroke), Some(*stroke_width));
            DrawNode::Path { segments, style: Some(style) }
        }
        NoteBlockNode::Table { width, height, .. } | NoteBlockNode::Math { width, height, .. } | NoteBlockNode::Group { width, height, .. } => {
            let style = note_intern_style(styles, None, Some(SemioRgba { r: 0.53, g: 0.53, b: 0.53, a: 1.0 }), Some(1.0));
            DrawNode::Path { segments: note_outline_rect_segments(*width, *height), style: Some(style) }
        }
    };
    Some(DrawNode::Group { transform, children: vec![inner] })
}

/// 🧿️ Builds this document's `SemioDrawingSnapshot` — one flattened, visible-only layer whose
/// children are each block's `draw_node_from_note_block` mapping. This is the real snapshot
/// `note_document_to_svg` hands to `io_dispatch` (never a hand-rolled SVG string).
pub fn note_document_to_drawing_snapshot(document: &NoteSnapshot) -> SemioDrawingSnapshot {
    let (width, height) = note_document_bounds(document);
    let mut styles = Vec::new();
    let children: Vec<DrawNode> = crate::artifacts::note::schema::flatten_blocks(&document.blocks)
        .into_iter()
        .filter(|block| crate::artifacts::note::schema::block_visible(block))
        .filter_map(|block| draw_node_from_note_block(block, document, &mut styles))
        .collect();
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: width as f64, height: height as f64, background: None },
        styles,
        layers: vec![DrawLayer { id: "0".into(), name: "note".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

const NOTE_DRAWING_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const NOTE_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

/// 📌️ Registers the stdio semio/drawing subset's composer (schema descriptor + document codec +
/// validator + svg/dxf/pdf io entries) into the process-global `io` registry exactly once, so
/// `io_dispatch` below can resolve the drawing→svg bridge regardless of host-boot ordering (unit
/// tests included — nothing else in this test binary calls stdio's own `plugin()`/`register()`).
fn ensure_semio_drawing_bridge_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(semio_drawing_composer::register);
}

/// 📤️ note → svg, real bridge: builds this document's `SemioDrawingSnapshot`, then dispatches it
/// through stdio's registered semio/drawing→svg composer (`io_dispatch`, never a hand-rolled SVG
/// string) to obtain a real `SvgSnapshot`, which is printed back to XML text via svg's own
/// `write_svg_xml`.
pub fn note_document_to_svg(document: &NoteSnapshot) -> Result<(String, u32, u32), String> {
    ensure_semio_drawing_bridge_registered();
    let (width, height) = note_document_bounds(document);
    let drawing = note_document_to_drawing_snapshot(document);
    let drawing_bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&drawing);
    let key = IoKey {
        artifact_kind: NOTE_DRAWING_DIALECT.artifact_kind.to_string(),
        standard: NOTE_DRAWING_DIALECT.standard.0.to_string(),
        subset: NOTE_DRAWING_DIALECT.subset.0.to_string(),
        direction: IoDirection::Export,
        format_kind: NOTE_SVG_DIALECT.artifact_kind.to_string(),
        format_standard: NOTE_SVG_DIALECT.standard.0.to_string(),
        format_subset: NOTE_SVG_DIALECT.subset.0.to_string(),
    };
    let sources = [ErasedComposeSource { dialect: NOTE_DRAWING_DIALECT, payload: IoPayload::Binary(drawing_bytes) }];
    let composed = io_dispatch(&key, &sources).map_err(|error| format!("note→svg via semio/drawing bridge: {}", error.message))?;
    let svg_bytes = match composed.payload {
        IoPayload::Binary(bytes) => bytes,
        IoPayload::Text(_) => return Err("note→svg via semio/drawing bridge: expected a Binary (ArtifactPack) svg payload".into()),
    };
    let svg_snapshot = <semio_s_plugin_stdio::artifacts::svg::schema::snapshot::SvgSnapshot as store::ArtifactPack>::decode_pack(&svg_bytes)
        .map_err(|error| format!("note→svg via semio/drawing bridge: decode svg snapshot: {error:?}"))?;
    Ok((write_svg_xml(&svg_snapshot.doc), width, height))
}

pub fn note_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let document: NoteSnapshot = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    note_document_to_svg(&document)
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// 🕳️ Not rewired onto the semio/drawing bridge (see `stdio_gaps` in the W5b ticket report): the
/// drawing subset has no dwg-format io leaf yet (only svg/dxf/pdf), and routing through the
/// framework's `dwg_drawing_to_svg` + `io_dispatch(svg→drawing)` round trip would REGRESS this
/// path — `dwg_drawing_to_svg` only walks `LwPolyline` geometry (silently drops `Text` entities)
/// and `DrawNode::Text` has no font-size field to carry DWG's `height`. `ink_block_from_points`/
/// `text_block_from_dwg` below are real domain mappers over already-typed `DwgGeometry` fields
/// (not hand-rolled DWG byte manipulation — `semio_framework::dwg_from_bytes` does the actual
/// byte-level parse), kept as the honest, lossless choice until that bridge exists.
fn ink_block_from_points(points: &[[f64; 2]]) -> NoteBlockNode {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for point in points {
        min_x = min_x.min(point[0]);
        min_y = min_y.min(point[1]);
        max_x = max_x.max(point[0]);
        max_y = max_y.max(point[1]);
    }
    let local_points = points.iter().map(|point| [point[0] - min_x, point[1] - min_y]).collect();
    NoteBlockNode::Ink {
        id: crate::artifacts::note::schema::create_note_id("dwg-ink"),
        name: "Imported Stroke".into(),
        x: min_x,
        y: min_y,
        width: (max_x - min_x).max(1.0),
        height: (max_y - min_y).max(1.0),
        rotation: 0.0,
        visible: true,
        locked: false,
        points: local_points,
        stroke_width: 1.0,
        color: [0.0, 0.0, 0.0, 1.0],
    }
}

fn text_block_from_dwg(at: &[f64; 3], height: f64, rotation: f64, content: &str) -> NoteBlockNode {
    let font_size = if height > 0.0 { height } else { 12.0 };
    NoteBlockNode::Text {
        id: crate::artifacts::note::schema::create_note_id("dwg-text"),
        name: "Imported Text".into(),
        x: at[0],
        y: at[1],
        width: (content.chars().count() as f64 * font_size * 0.6).max(font_size),
        height: font_size * 1.4,
        rotation,
        visible: true,
        locked: false,
        paragraphs: vec![NoteTextParagraph { runs: vec![NoteTextRun { text: content.to_string(), bold: None, italic: None, underline: None, link: None }] }],
        font_size,
        font_weight: "normal".into(),
        align: "left".into(),
    }
}

pub fn note_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let mut document = crate::artifacts::note::schema::empty_note_snapshot();
    document.id = crate::artifacts::note::schema::create_note_id("dwg-import");
    document.title = Some("Imported Drawing".into());
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::Line { start, end } => {
                document.blocks.push(ink_block_from_points(&[[start[0], start[1]], [end[0], end[1]]]));
            }
            DwgGeometry::LwPolyline { closed, vertices, .. } => {
                if vertices.len() >= 2 {
                    let mut points = vertices.clone();
                    if *closed {
                        points.push(vertices[0]);
                    }
                    document.blocks.push(ink_block_from_points(&points));
                }
            }
            DwgGeometry::Text { at, height, rotation, content } => {
                document.blocks.push(text_block_from_dwg(at, *height, *rotation, content));
            }
            _ => {}
        }
    }
    serde_json::to_value(&document).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaImport

//#region 🧪️Tests
#[cfg(test)]
mod media_tests {
    use super::*;
    use crate::artifacts::note::{NoteImageAsset, NoteTableCell};
    use semio_framework::{DwgColor, DwgEntity, DwgLayer};

    /// 🧪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
    #[test]
    fn imports_dwg_polyline_and_text_into_note_blocks() {
        let drawing = DwgDrawing {
            layers: vec![DwgLayer::default()],
            entities: vec![
                DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], bulges: vec![0.0, 0.5, 0.0] } },
                DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [1.0, 2.0, 0.0], height: 2.5, rotation: 0.0, content: "semio".into() } },
            ],
            extmin: [0.0, 0.0, 0.0],
            extmax: [10.0, 10.0, 0.0],
        };
        let value = note_document_json_from_dwg(&drawing).unwrap();
        let document: NoteSnapshot = serde_json::from_value(value).unwrap();
        assert_eq!(document.schema, crate::artifacts::note::NOTE_DOCUMENT_SCHEMA);
        assert_eq!(document.blocks.len(), 2);
        let ink_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Ink { .. })).count();
        let text_count = document.blocks.iter().filter(|block| matches!(block, NoteBlockNode::Text { .. })).count();
        assert_eq!(ink_count, 1);
        assert_eq!(text_count, 1);
        if let Some(NoteBlockNode::Ink { points, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Ink { .. })) {
            assert_eq!(points.len(), 4);
        } else {
            panic!("expected ink block");
        }
        if let Some(NoteBlockNode::Text { paragraphs, .. }) = document.blocks.iter().find(|block| matches!(block, NoteBlockNode::Text { .. })) {
            assert_eq!(paragraphs[0].runs[0].text, "semio");
        } else {
            panic!("expected text block");
        }
    }

    /// 🧪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
    #[test]
    fn imports_empty_dwg_drawing_as_valid_empty_note_snapshot() {
        let drawing = DwgDrawing::default();
        let value = note_document_json_from_dwg(&drawing).unwrap();
        let document: NoteSnapshot = serde_json::from_value(value).unwrap();
        assert_eq!(document.schema, crate::artifacts::note::NOTE_DOCUMENT_SCHEMA);
        assert!(document.blocks.is_empty());
    }

    /// 🧪️ Real end-to-end proof `note_document_to_svg` goes through `io_dispatch` onto stdio's
    /// registered semio/drawing→svg composer (not a hand-rolled SVG string): text content, ink
    /// strokes, and the Table catch-all outline all have to survive a real `SemioDrawingSnapshot`
    /// build + a real svg-composer round trip to show up in the output. Relocated from the deleted
    /// `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
    #[test]
    fn document_to_svg_dispatches_through_semio_drawing_bridge() {
        let mut document = crate::artifacts::note::schema::empty_note_snapshot();
        document.blocks.push(NoteBlockNode::Text {
            id: "t1".into(),
            name: "Text".into(),
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 30.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            paragraphs: vec![NoteTextParagraph { runs: vec![NoteTextRun { text: "hello semio".into(), bold: None, italic: None, underline: None, link: None }] }],
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        });
        document.blocks.push(NoteBlockNode::Ink {
            id: "i1".into(),
            name: "Ink".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            points: vec![[0.0, 0.0], [5.0, 5.0]],
            stroke_width: 2.0,
            color: [1.0, 0.0, 0.0, 1.0],
        });
        document.blocks.push(NoteBlockNode::Table {
            id: "tb1".into(),
            name: "Table".into(),
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 40.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            columns: vec!["A".into()],
            rows: vec![vec![NoteTableCell { content: String::new() }]],
        });

        let (svg, width, height) = note_document_to_svg(&document).expect("svg export via io_dispatch");
        assert!(svg.starts_with("<svg"), "{svg}");
        assert!(svg.contains("hello semio"), "{svg}");
        assert!(width >= 1024 && height >= 1024);

        // Same pipeline through the JSON-wrapped entry point every io leaf/media handler actually calls.
        let json = serde_json::to_value(&document).unwrap();
        let (svg_via_json, _w, _h) = note_document_json_to_svg(&json).expect("json svg export via io_dispatch");
        assert_eq!(svg, svg_via_json);
    }

    /// 🧪️ Real proof an image block's asset bytes flow through `DrawNode::Image` (base64-decoded
    /// from the `NoteImageAsset.data` uri) and back out as a data uri on the svg side. Relocated
    /// from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
    #[test]
    fn document_to_svg_embeds_image_asset_bytes_as_data_uri() {
        let mut document = crate::artifacts::note::schema::empty_note_snapshot();
        document.assets.insert(
            "asset-1".into(),
            NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,AAECAw==".into(), width: Some(4.0), height: Some(4.0) },
        );
        document.blocks.push(NoteBlockNode::Image { id: "im1".into(), name: "Image".into(), x: 0.0, y: 0.0, width: 4.0, height: 4.0, rotation: 0.0, visible: true, locked: false, image_key: "asset-1".into() });

        let (svg, _w, _h) = note_document_to_svg(&document).expect("svg export via io_dispatch");
        assert!(svg.contains("data:image/png;base64,"), "{svg}");
    }

    /// 🧪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
    #[test]
    fn note_document_to_drawing_snapshot_flattens_visible_blocks_into_one_layer() {
        let mut document = crate::artifacts::note::schema::empty_note_snapshot();
        document.blocks.push(crate::artifacts::note::schema::create_block_by_kind("text", 5.0, 6.0));
        let mut hidden = crate::artifacts::note::schema::create_block_by_kind("text", 0.0, 0.0);
        if let NoteBlockNode::Text { visible, .. } = &mut hidden {
            *visible = false;
        }
        document.blocks.push(hidden);

        let drawing = note_document_to_drawing_snapshot(&document);
        assert_eq!(drawing.layers.len(), 1);
        let DrawNode::Group { children, .. } = &drawing.layers[0].root else { panic!("expected root Group") };
        assert_eq!(children.len(), 1, "hidden block must not be mapped into a DrawNode");
    }
}
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::note::standards::v1::subsets::any::schema::NoteComposer as NoteAnyComposer;
    use crate::artifacts::note::standards::v1::subsets::any::schema::NoteBuilder as NoteAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const NOTE_DIALECT: Dialect = Dialect { artifact_kind: "s.note", standard: StandardId("1"), subset: SubsetId("*") };
    const NOTE_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::note::NoteSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == NOTE_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => NoteAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => NoteAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "NoteComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == NOTE_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::note::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "NoteComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    fn compose_export_dxf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::note::io::export::serializers::artifacts::dxf::v_r12::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DXF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<NoteAnyComposer>(),
            ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_svg },
            ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_pdf },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_DXF_DIALECT, reads: &[NOTE_DIALECT], compose: compose_export_dxf },
        ]).as_slice()
    }

    //#region 🧪️Tests
    /// 🧪️ Reference pilot (hand-maintained, not regenerated by w15_add_export_entries.py) proving the
    /// export-entry pattern is genuinely correct, not just compiling -- a real round-trip through the
    /// typed registry. Every other artifact's export entries follow this exact mechanical shape.
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{register_composer_entries, io_resolve, io_dialects_for, IoKey, IoDirection};

        #[test]
        fn every_export_target_writes_its_own_dialect_and_reads_only_the_native_dialect() {
            for entry in entries().iter().skip(1) {
                assert_eq!(entry.reads, &[NOTE_DIALECT], "export entry writing {:?} should read only the native dialect", entry.writes);
            }
        }

        #[test]
        fn registered_json_export_entry_resolves_and_produces_valid_json_bytes() {
            register_composer_entries(entries());
            let key = IoKey {
                artifact_kind: NOTE_DIALECT.artifact_kind.to_string(),
                standard: NOTE_DIALECT.standard.0.to_string(),
                subset: NOTE_DIALECT.subset.0.to_string(),
                direction: IoDirection::Export,
                format_kind: EXPORT_JSON_DIALECT.artifact_kind.to_string(),
                format_standard: EXPORT_JSON_DIALECT.standard.0.to_string(),
                format_subset: EXPORT_JSON_DIALECT.subset.0.to_string(),
            };
            let resolved = io_resolve(&key).expect("note -> json export entry resolves through the typed registry");
            let snapshot = NoteAnyBuilder::empty().build().expect("empty note builds");
            let native_bytes = store::ArtifactPack::encode_pack(&snapshot);
            let sources = [ErasedComposeSource { dialect: NOTE_DIALECT, payload: IoPayload::Binary(native_bytes) }];
            let composed = (resolved.compose)(&sources).expect("compose succeeds");
            assert_eq!(composed.dialect, EXPORT_JSON_DIALECT);
            let IoPayload::Binary(bytes) = composed.payload else { panic!("expected binary json payload") };
            let value: serde_json::Value = serde_json::from_slice(&bytes).expect("export produced valid json bytes");
            assert!(value.is_object(), "expected a json object, got {value:?}");
        }

        #[test]
        fn dialects_for_export_direction_includes_every_target() {
            register_composer_entries(entries());
            let dialects = io_dialects_for(NOTE_DIALECT.artifact_kind, IoDirection::Export);
            for entry in entries().iter().skip(1) {
                assert!(dialects.contains(&entry.writes), "expected note's export dialects to include {:?}, got {:?}", entry.writes, dialects);
            }
        }
    }
    //#endregion 🧪️Tests
}
//#endregion 🚪️DerivedIoRegistry
