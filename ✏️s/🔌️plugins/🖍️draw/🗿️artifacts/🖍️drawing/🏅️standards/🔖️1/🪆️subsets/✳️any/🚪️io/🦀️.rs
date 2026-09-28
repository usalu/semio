//! 🚪️ IO s.draw (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec plus every
//! foreign hop, aggregated from the typed `Serializer<DrawingSnapshot>`/`Deserializer<DrawingSnapshot>`
//! leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`. Replaces the old hand-rolled
//! `ArtifactComposition`/`ComposerEntry` dispatch chain (`derived_composition`/`io_registry`,
//! deleted outright) — all io now goes exclusively through the `io_mechanism` registry (design.md
//! rule 3). `import_stdio_kinds`/`export_stdio_kinds` (old, zero callers even before this pass)
//! deleted alongside them.

//#region 🔖️SemioBridge
use crate::schema::{drawing_layer_world_bounds, flatten_drawing_document_to_scene_nodes, flatten_drawing_layers, DrawingSceneNode};
use crate::{DrawingSnapshot, FillStyle, PathSegment};
/// 🌉️ Relocated verbatim from the `⚙️engine` directory (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 5: sniff/codec dispatch and
/// cross-format bridge functions live in `🚪️io/`).
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{
    DrawCanvas as SemioDrawCanvas, DrawLayer as SemioDrawLayer, DrawNode as SemioDrawNode, DrawStyle as SemioDrawStyle, PathSegment as SemioPathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
};
fn resolve_drawing_document_artboard(doc: &DrawingSnapshot) -> (u32, u32) {
    if let Some(artboard) = &doc.artboard {
        return (artboard.width.max(1.0).round() as u32, artboard.height.max(1.0).round() as u32);
    }
    let mut max_x: f64 = 1024.0;
    let mut max_y: f64 = 1024.0;
    for layer in flatten_drawing_layers(&doc.layers) {
        if let Some((x, y, width, height)) = drawing_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

/// 🌉️ [DrawingTransform]'s 6-value affine matrix → semio's [SemioTransform] (Z-only rotation
/// quaternion, axis scale, zero-z translation) — the same decomposition stdio's own svg↔drawing
/// bridge applies on its side (`matrix_to_semio_transform` in that leaf).
fn matrix_to_semio_transform(matrix: [f64; 6]) -> SemioTransform {
    let transform = crate::schema::drawing_matrix_to_transform(matrix);
    SemioTransform {
        translation: SemioPoint3 { x: transform.x, y: transform.y, z: 0.0 },
        rotation: SemioQuaternion { x: 0.0, y: 0.0, z: (transform.rotation / 2.0).sin(), w: (transform.rotation / 2.0).cos() },
        scale: SemioPoint3 { x: transform.scale_x, y: transform.scale_y, z: 1.0 },
    }
}

/// ✏️ Drawing's own [PathSegment] → semio's [SemioPathSegment] — same SVG-command grammar, field
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

/// 🎨️ [FillStyle::Solid]/[StrokeStyle] → [SemioRgba] — `DrawingStyle` is solid-color-only, so
/// gradients have no representable equivalent and are honestly dropped (matching the pre-migration
/// SVG renderer's own gradient fallback: no fill, not a fabricated flat color).
fn solid_fill_to_semio_rgba(fill: &FillStyle) -> Option<SemioRgba> {
    match fill {
        FillStyle::Solid { color } => Some(SemioRgba { r: color[0] as f32, g: color[1] as f32, b: color[2] as f32, a: color[3] as f32 }),
        FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => None,
    }
}

/// 🎨️ Interns one [DrawingSceneNode]'s fill/stroke/opacity as a named [SemioDrawStyle] and returns
/// its name, or `None` when the node carries no representable presentation at all. 🕳️ stdio_gap:
/// `blend_mode`/`fill_rule` have no `DrawingStyle` field and `Group`/`Image` nodes have no opacity
/// slot at all (only `Path`/`Text` reference a style) — both honestly dropped, not fabricated.
fn intern_semio_style(styles: &mut Vec<SemioDrawStyle>, node: &DrawingSceneNode) -> Option<String> {
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
/// [flatten_drawing_document_to_scene_nodes] for image scene nodes) into real mime + bytes.
fn decode_data_uri_bytes(uri: &str) -> Option<(String, Vec<u8>)> {
    let rest = uri.strip_prefix("data:")?;
    let (meta, data) = rest.split_once(',')?;
    let mime = meta.split(';').next().unwrap_or("application/octet-stream").to_string();
    let bytes = base64_codec::base64_standard_decode(data).ok()?;
    Some((mime, bytes))
}

/// 🖍️ One [DrawingSceneNode] → semio's recursive [SemioDrawNode]: each becomes its own `Group`
/// carrying the node's baked world transform, wrapping exactly one Path/Text/Image leaf (mirrors
/// the pre-migration SVG renderer's own `<g transform="matrix(...)"><path/></g>` shape).
fn semio_drawing_node_from_scene_node(node: &DrawingSceneNode, styles: &mut Vec<SemioDrawStyle>) -> Option<SemioDrawNode> {
    let style = intern_semio_style(styles, node);
    if let Some(text) = &node.text {
        let children = semio_s_2d::text::drawing_text_lines(&text.content).enumerate().filter(|(_, line)| !line.is_empty()).map(|(index, line)| {
            SemioDrawNode::Text { value: line.to_owned(), at: SemioPoint2 { x: 0.0, y: text.size + index as f64 * text.size * semio_s_2d::text::DRAWING_TEXT_LINE_HEIGHT }, style: style.clone() }
        }).collect();
        return Some(SemioDrawNode::Group { transform: matrix_to_semio_transform(node.transform), children });
    }
    let leaf = if let Some(image) = &node.image {
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
/// side of drawing's domain↔semio bridge. [flatten_drawing_document_to_scene_nodes] has already resolved
/// booleans/traces/curve-flattening, so every scene node here is a concrete leaf.
pub fn drawing_document_to_semio_drawing(doc: &DrawingSnapshot) -> SemioDrawingSnapshot {
    let (width, height) = resolve_drawing_document_artboard(doc);
    let mut styles = Vec::new();
    let children: Vec<SemioDrawNode> = flatten_drawing_document_to_scene_nodes(doc).iter().filter_map(|node| semio_drawing_node_from_scene_node(node, &mut styles)).collect();
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: SemioDrawCanvas { width: width as f64, height: height as f64, background: None },
        styles,
        layers: vec![SemioDrawLayer { id: "root".into(), name: doc.title.clone().unwrap_or_else(|| "root".into()), visible: true, root: SemioDrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

/// 🎨️ Exports the authored scene through the typed SVG serializer.
pub fn drawing_document_to_svg(doc: &DrawingSnapshot) -> Result<(String, u32, u32), String> {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    export::svg::v1_1::any::drawing_document_to_svg(doc)
}

pub fn drawing_document_json_to_svg(value: &dsl::DslValue) -> Result<(String, u32, u32), String> {
    let doc: DrawingSnapshot = dsl::FromValue::from_value(value.clone()).map_err(|error: dsl::ValueError| error.to_string())?;
    drawing_document_to_svg(&doc)
}
//#endregion 🔖️SemioBridge

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{DrawingMutation, DrawingSnapshot, DRAWING_DIALECT, DRAWING_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<DrawingSnapshot, export::svg::v1_1::any::DrawingIntoSvg>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::pdf::v1_4::any::DrawingIntoPdf>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::png::v1_2::any::DrawingIntoPng>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::json::v_rfc8259::any::DrawingIntoJson>(DRAWING_DIALECT),
                    deserializer_entry::<DrawingSnapshot, import::json::v_rfc8259::any::JsonIntoDraw>(DRAWING_DIALECT),
                    deserializer_entry::<DrawingSnapshot, import::svg::v1_1::any::SvgIntoDraw>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::dwg::v_ac1018::any::DrawingIntoDwg>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::dxf::v_r12::any::DrawingIntoDxf>(DRAWING_DIALECT),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            // 🎯️ `LanguagePair { text: None, binary: None }` for every facet: a documented,
            // deliberate scope-narrowing matching every other subset already on the new tree
            // (stdio binary/txt, sequence) — `NativeCodecs`'s own doc calls this a legal, supported
            // shape. The underlying `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` codecs these
            // would point at are unchanged, independently implemented (see `📸️snapshot/`,
            // `🧬️mutations/` siblings), and independently tested either way.
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<DrawingSnapshot, DrawingMutation>(DRAWING_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
