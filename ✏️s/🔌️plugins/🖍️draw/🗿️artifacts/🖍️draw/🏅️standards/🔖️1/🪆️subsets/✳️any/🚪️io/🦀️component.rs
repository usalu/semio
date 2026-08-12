//! 🚪️ IO s.draw (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from the artifact root's `declaration()`), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"] }

//#region 🔖️SemioBridge
/// 🌉️ Relocated verbatim from the `⚙️engine` directory (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 5: sniff/codec dispatch and
/// cross-format bridge functions live in `🚪️io/`).
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use crate::artifacts::draw::{DrawSnapshot, FillStyle, PathSegment};
use crate::artifacts::draw::schema::{draw_layer_world_bounds, flatten_draw_document_to_scene_nodes, flatten_draw_layers, DrawSceneNode};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
    DrawCanvas as SemioDrawCanvas, DrawLayer as SemioDrawLayer, DrawNode as SemioDrawNode, DrawStyle as SemioDrawStyle, PathSegment as SemioPathSegment, SemioDrawingSnapshot,
    STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
};
use semio_s_plugin_stdio::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::write_svg_xml;
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;

/// 🕳️ stdio_gap: `s.stdio.semio/v1/drawing` bridges only to svg/dxf/pdf (per the master plan's
/// format lattice — dwg lives under `s.stdio.semio/v1/cad`, standard `ac1024`, a different hub
/// entirely). There is no route from `SemioDrawingSnapshot` to DWG bytes today, so this plugin's
/// former ad-hoc `draw_document_json_to_dwg_bytes`/`draw_document_json_from_dwg` pair was deleted
/// outright rather than hand-rolling DWG again — see `w5b-w-report.md` `stdio_gaps`.
const SEMIO_DRAWING_DIALECT: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework::StandardId("v1"), subset: semio_framework::SubsetId("drawing") };
const SVG_DIALECT: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: "s.stdio.svg", standard: semio_framework::StandardId("1.1"), subset: semio_framework::SubsetId::ANY };

/// 📌️ W5b-close fix: registers stdio's semio/drawing subset composer (svg/dxf/pdf io entries)
/// into the process-global `io` registry exactly once, so `io_dispatch` below resolves the
/// drawing→svg bridge regardless of host-boot ordering — a bare `cargo test` process never runs
/// the plugin-host boot path that would normally call this. Mirrors 🗒️note's/📏️layout's/🌍️gis's
/// own `ensure_..._registered()` helper (w5b-verify-report.md §6b flagged draw as the one sibling
/// that had not added this, causing `draw_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing`
/// and `draw_io_declares_vector_out_and_export_media_covers_both_ports` to fail with "no composer
/// registered").
fn ensure_semio_drawing_bridge_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io::register);
}

fn resolve_draw_document_artboard(doc: &DrawSnapshot) -> (u32, u32) {
    if let Some(artboard) = &doc.artboard {
        return (artboard.width.max(1.0).round() as u32, artboard.height.max(1.0).round() as u32);
    }
    let mut max_x: f64 = 1024.0;
    let mut max_y: f64 = 1024.0;
    for layer in flatten_draw_layers(&doc.layers) {
        if let Some((x, y, width, height)) = draw_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

/// 🌉️ [DrawTransform]'s 6-value affine matrix → semio's [SemioTransform] (Z-only rotation
/// quaternion, axis scale, zero-z translation) — the same decomposition stdio's own svg↔drawing
/// bridge applies on its side (`matrix_to_semio_transform` in that leaf).
fn matrix_to_semio_transform(matrix: [f64; 6]) -> SemioTransform {
    let transform = crate::artifacts::draw::schema::draw_matrix_to_transform(matrix);
    SemioTransform {
        translation: SemioPoint3 { x: transform.x, y: transform.y, z: 0.0 },
        rotation: SemioQuaternion { x: 0.0, y: 0.0, z: (transform.rotation / 2.0).sin(), w: (transform.rotation / 2.0).cos() },
        scale: SemioPoint3 { x: transform.scale_x, y: transform.scale_y, z: 1.0 },
    }
}

/// ✏️ Draw's own [PathSegment] → semio's [SemioPathSegment] — same SVG-command grammar, field
/// renames only (no geometry recomputed).
fn to_semio_path_segment(segment: &PathSegment) -> SemioPathSegment {
    match *segment {
        PathSegment::Move { to } => SemioPathSegment::MoveTo { to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Line { to } => SemioPathSegment::LineTo { to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Quad { ctrl, to } => SemioPathSegment::QuadTo { c: SemioPoint2 { x: ctrl[0], y: ctrl[1] }, to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Cubic { ctrl1, ctrl2, to } => SemioPathSegment::CubicTo { c1: SemioPoint2 { x: ctrl1[0], y: ctrl1[1] }, c2: SemioPoint2 { x: ctrl2[0], y: ctrl2[1] }, to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => SemioPathSegment::ArcTo { rx, ry, x_rotation: rotation, large_arc, sweep, to: SemioPoint2 { x: to[0], y: to[1] } },
        PathSegment::Close => SemioPathSegment::Close,
    }
}

/// 🎨️ [FillStyle::Solid]/[StrokeStyle] → [SemioRgba] — `DrawStyle` is solid-color-only, so
/// gradients have no representable equivalent and are honestly dropped (matching the pre-migration
/// SVG renderer's own gradient fallback: no fill, not a fabricated flat color).
fn solid_fill_to_semio_rgba(fill: &FillStyle) -> Option<SemioRgba> {
    match fill {
        FillStyle::Solid { color } => Some(SemioRgba { r: color[0] as f32, g: color[1] as f32, b: color[2] as f32, a: color[3] as f32 }),
        FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => None,
    }
}

/// 🎨️ Interns one [DrawSceneNode]'s fill/stroke/opacity as a named [SemioDrawStyle] and returns
/// its name, or `None` when the node carries no representable presentation at all. 🕳️ stdio_gap:
/// `blend_mode`/`fill_rule` have no `DrawStyle` field and `Group`/`Image` nodes have no opacity
/// slot at all (only `Path`/`Text` reference a style) — both honestly dropped, not fabricated.
fn intern_semio_style(styles: &mut Vec<SemioDrawStyle>, node: &DrawSceneNode) -> Option<String> {
    let fill = node.fill.as_ref().and_then(solid_fill_to_semio_rgba);
    let stroke = node.stroke.as_ref().map(|style| SemioRgba { r: style.color[0] as f32, g: style.color[1] as f32, b: style.color[2] as f32, a: style.color[3] as f32 });
    let stroke_width = node.stroke.as_ref().map(|style| style.width);
    let opacity = if (node.opacity - 1.0).abs() > f64::EPSILON { Some(node.opacity as f32) } else { None };
    if fill.is_none() && stroke.is_none() && opacity.is_none() {
        return None;
    }
    let name = format!("style{}", styles.len());
    styles.push(SemioDrawStyle { name: name.clone(), fill, stroke, stroke_width, opacity });
    Some(name)
}

/// 🖼️ Decodes one `data:<mime>;base64,<data>` URI (as built by
/// [flatten_draw_document_to_scene_nodes] for image scene nodes) into real mime + bytes.
fn decode_data_uri_bytes(uri: &str) -> Option<(String, Vec<u8>)> {
    let rest = uri.strip_prefix("data:")?;
    let (meta, data) = rest.split_once(',')?;
    let mime = meta.split(';').next().unwrap_or("application/octet-stream").to_string();
    let bytes = BASE64.decode(data).ok()?;
    Some((mime, bytes))
}

/// 🖍️ One [DrawSceneNode] → semio's recursive [SemioDrawNode]: each becomes its own `Group`
/// carrying the node's baked world transform, wrapping exactly one Path/Text/Image leaf (mirrors
/// the pre-migration SVG renderer's own `<g transform="matrix(...)"><path/></g>` shape).
fn semio_draw_node_from_scene_node(node: &DrawSceneNode, styles: &mut Vec<SemioDrawStyle>) -> Option<SemioDrawNode> {
    let style = intern_semio_style(styles, node);
    let leaf = if let Some(text) = &node.text {
        SemioDrawNode::Text { value: text.content.clone(), at: SemioPoint2 { x: 0.0, y: text.size }, style }
    } else if let Some(image) = &node.image {
        let (mime, bytes) = decode_data_uri_bytes(&image.src).unwrap_or_default();
        SemioDrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: image.width, height: image.height, mime, bytes }
    } else {
        let segments: Vec<SemioPathSegment> = node.segments.iter().map(to_semio_path_segment).collect();
        if segments.is_empty() {
            return None;
        }
        SemioDrawNode::Path { segments, style }
    };
    Some(SemioDrawNode::Group { transform: matrix_to_semio_transform(node.transform), children: vec![leaf] })
}

/// 🌉️ Builds a real [SemioDrawingSnapshot] from this plugin's own domain document — the semio hub
/// side of draw's domain↔semio bridge. [flatten_draw_document_to_scene_nodes] has already resolved
/// booleans/traces/curve-flattening, so every scene node here is a concrete leaf.
pub fn draw_document_to_semio_drawing(doc: &DrawSnapshot) -> SemioDrawingSnapshot {
    let (width, height) = resolve_draw_document_artboard(doc);
    let mut styles = Vec::new();
    let children: Vec<SemioDrawNode> = flatten_draw_document_to_scene_nodes(doc).iter().filter_map(|node| semio_draw_node_from_scene_node(node, &mut styles)).collect();
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: SemioDrawCanvas { width: width as f64, height: height as f64, background: None },
        styles,
        layers: vec![SemioDrawLayer { id: "root".into(), name: doc.title.clone().unwrap_or_else(|| "root".into()), visible: true, root: SemioDrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

/// @emoji 🌉️ Serializes a draw document to SVG markup and raster dimensions by building a real
/// [SemioDrawingSnapshot] and dispatching through stdio's real semio/drawing↔svg bridge
/// (`io_dispatch`) — replaces the deleted hand-rolled SVG string builder.
pub fn draw_document_to_svg(doc: &DrawSnapshot) -> Result<(String, u32, u32), String> {
    ensure_semio_drawing_bridge_registered();
    let (width, height) = resolve_draw_document_artboard(doc);
    let semio_drawing = draw_document_to_semio_drawing(doc);
    let key = semio_framework::IoKey {
        artifact_kind: SEMIO_DRAWING_DIALECT.artifact_kind.into(),
        standard: SEMIO_DRAWING_DIALECT.standard.0.into(),
        subset: SEMIO_DRAWING_DIALECT.subset.0.into(),
        direction: semio_framework::IoDirection::Export,
        format_kind: SVG_DIALECT.artifact_kind.into(),
        format_standard: SVG_DIALECT.standard.0.into(),
        format_subset: SVG_DIALECT.subset.0.into(),
    };
    let sources = [semio_framework::ErasedComposeSource { dialect: SEMIO_DRAWING_DIALECT, payload: semio_framework::IoPayload::Binary(<SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(&semio_drawing)) }];
    let composed = semio_framework::io_dispatch(&key, &sources).map_err(|error| error.message)?;
    let bytes = match composed.payload {
        semio_framework::IoPayload::Binary(bytes) => bytes,
        semio_framework::IoPayload::Text(text) => text.into_bytes(),
    };
    let svg = <SvgSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))?;
    Ok((write_svg_xml(&svg.doc), width, height))
}

pub fn draw_document_json_to_svg(value: &serde_json::Value) -> Result<(String, u32, u32), String> {
    let doc: DrawSnapshot = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    draw_document_to_svg(&doc)
}
//#endregion 🔖️SemioBridge
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::draw::DrawSnapshot;
    use crate::artifacts::draw::standards::v1::subsets::any::schema::DrawAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.draw", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_DXF: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_PDF: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_SVG: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };


    pub struct DrawComposerComposition;

    impl ArtifactComposition for DrawComposerComposition {
        type Snapshot = DrawSnapshot;
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
                    let analysis = DrawAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::draw::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_DXF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::draw::io::import::deserializers::artifacts::dxf::v_r12::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::draw::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PDF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::draw::io::import::deserializers::artifacts::pdf::v1_4::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::draw::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_SVG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::draw::io::import::deserializers::artifacts::svg::v1_1::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }

            }
            Err(ComposeError { message: "DrawComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated verbatim from the `⚙️engine` directory (rule 5). The artifact root's `declaration()`
/// calls `…::io::io_registry::entries()` (qualified — see that file's own `🔖️ArtifactKind` region);
/// the root's own shadowing `io_registry` (returns `&'static [&'static ComposerEntry]`, a different
/// type) must never be confused with this module's `entries() -> &'static [ComposerEntry]`.
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::draw::standards::v1::subsets::any::schema::DrawComposer as DrawAnyComposer;
    use crate::artifacts::draw::standards::v1::subsets::any::schema::DrawBuilder as DrawAnyBuilder;

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
    const DRAW_DIALECT: Dialect = Dialect { artifact_kind: "s.draw", standard: StandardId("1"), subset: SubsetId("*") };
    const DRAW_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::draw::DrawSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == DRAW_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => DrawAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => DrawAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "DrawComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == DRAW_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::draw::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "DrawComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    /// 🌉️ Builds a real `SemioDrawingSnapshot` from the rebuilt native snapshot and dispatches through
    /// stdio's real semio/drawing↔svg bridge (`io_dispatch`) — replaces the previous degenerate
    /// `📤️export/🧵️serializers/…/svg` leaf, which only ever wrapped this artifact's own DSL text
    /// disguised as SVG bytes.
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let semio_drawing = crate::artifacts::draw::io::draw_document_to_semio_drawing(&snapshot);
        const SEMIO_DRAWING_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
        let key = semio_framework_plugin::IoKey {
            artifact_kind: SEMIO_DRAWING_DIALECT.artifact_kind.into(),
            standard: SEMIO_DRAWING_DIALECT.standard.0.into(),
            subset: SEMIO_DRAWING_DIALECT.subset.0.into(),
            direction: semio_framework_plugin::IoDirection::Export,
            format_kind: EXPORT_SVG_DIALECT.artifact_kind.into(),
            format_standard: EXPORT_SVG_DIALECT.standard.0.into(),
            format_subset: EXPORT_SVG_DIALECT.subset.0.into(),
        };
        let hub_source = ErasedComposeSource { dialect: SEMIO_DRAWING_DIALECT, payload: IoPayload::Binary(store::ArtifactPack::encode_pack(&semio_drawing)) };
        let composed = semio_framework_plugin::io_dispatch(&key, std::slice::from_ref(&hub_source))?;
        Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: composed.payload, diagnostics: composed.diagnostics, confidence: IoConfidence::Medium })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::draw::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::draw::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::draw::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::draw::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    fn compose_export_dxf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::draw::io::export::serializers::artifacts::dxf::v_r12::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DXF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<DrawAnyComposer>(),
            ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[DRAW_DIALECT], compose: compose_export_svg },
            ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[DRAW_DIALECT], compose: compose_export_pdf },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[DRAW_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[DRAW_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[DRAW_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_DXF_DIALECT, reads: &[DRAW_DIALECT], compose: compose_export_dxf },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::draw::schema::{create_draw_image_layer, create_draw_shape_layer_rect, default_draw_document, default_layer_base};
    use crate::artifacts::draw::{DrawImageAsset, DrawLayerNode, DrawTextBody, StrokeStyle};

    /// 🌉️ Ported from the pre-migration `draw_document_to_svg_renders_shape_text_image_and_gradient_nodes`
    /// (same shape/text/image/gradient coverage) onto the new `SemioDrawingSnapshot`→`io_dispatch`
    /// bridge — decodes the real bridged SVG back into stdio's own typed `SvgElement` tree instead
    /// of substring-matching hand-rolled markup, since the markup is no longer hand-rolled.
    #[test]
    fn draw_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing() {
        use semio_s_plugin_stdio::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::{parse_svg_xml, svg_element_from_xml_node, SvgElement};

        let mut rect = create_draw_shape_layer_rect("Rect");
        if let DrawLayerNode::Shape(shape) = &mut rect {
            shape.base.attributes.fill = Some(FillStyle::Solid { color: [1.0, 0.0, 0.0, 0.5] });
            shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: "round".into(), join: "round".into(), dash: None });
        }
        let mut gradient_rect = create_draw_shape_layer_rect("Gradient");
        if let DrawLayerNode::Shape(shape) = &mut gradient_rect {
            shape.base.attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0, stops: Vec::new() });
        }
        let text = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("T"), x: 0.0, y: 0.0, content: "<a & b>".into(), size: 12.0 });
        let mut assets = std::collections::BTreeMap::new();
        assets.insert("img".to_string(), DrawImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(4), height: Some(4) });
        let image = create_draw_image_layer("Image", "img");

        let mut doc = default_draw_document("svg-test", None);
        doc.layers = vec![rect, gradient_rect, text, image];
        doc.assets = assets;
        doc.artboard = None;

        let (svg_text, width, height) = draw_document_to_svg(&doc).expect("svg export via semio/drawing bridge");
        assert!(width >= 1 && height >= 1);

        let reparsed = parse_svg_xml(&svg_text).expect("bridged svg reparses");
        let root = svg_element_from_xml_node(reparsed.root.as_ref().expect("svg root")).expect("typed svg root");
        let layer_children = match &root {
            SvgElement::Svg { children, .. } => match &children[0] {
                SvgElement::Group { children, .. } => children,
                other => panic!("expected layer group, got {other:?}"),
            },
            other => panic!("expected <svg> root, got {other:?}"),
        };
        assert_eq!(layer_children.len(), 4, "rect, gradient rect, text, image");
        let leaf = |index: usize| match &layer_children[index] {
            SvgElement::Group { children, .. } => &children[0],
            other => panic!("expected node wrapper group, got {other:?}"),
        };

        match leaf(0) {
            SvgElement::Path { common, .. } => assert!(common.presentation.fill.as_deref().is_some_and(|fill| fill.starts_with("rgba(255,")), "{:?}", common.presentation.fill),
            other => panic!("expected filled rect path, got {other:?}"),
        }
        match leaf(1) {
            SvgElement::Path { common, .. } => assert!(common.presentation.fill.is_none(), "gradients have no semio/drawing equivalent — dropped, not fabricated"),
            other => panic!("expected gradient rect path, got {other:?}"),
        }
        match leaf(2) {
            SvgElement::Text { children, .. } => assert_eq!(children, &vec![SvgElement::TextNode("<a & b>".into())]),
            other => panic!("expected text node, got {other:?}"),
        }
        match leaf(3) {
            SvgElement::Unknown { name, attrs, .. } => {
                assert_eq!(name, "image");
                let href = attrs.iter().find(|attr| attr.name == "href").expect("image href attr");
                assert!(href.value.starts_with("data:image/png;base64,"));
            }
            other => panic!("expected image node, got {other:?}"),
        }

        let json_error = draw_document_json_to_svg(&serde_json::json!({"bad": true}));
        assert!(json_error.is_err());
    }
}
//#endregion 🧪️Tests
