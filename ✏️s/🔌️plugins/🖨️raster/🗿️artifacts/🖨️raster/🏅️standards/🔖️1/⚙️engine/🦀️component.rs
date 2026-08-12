//! ⚙️ Raster artifact — headless compute (constitutional: engine).

use crate::artifacts::raster::{RasterImageAsset, RasterLayerNode, RasterLayerPatch, RasterSnapshot, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use base64::Engine;
use semio_framework::{io::io_compose_via, io_dispatch, Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Constants
/// 📄️ The `semio` example document, handcrafted in the `.raster` DSL — {@link semio_example_document}/
/// {@link semio_example_json} are the only ways it should be consumed.
const SEMIO_RASTER_EXAMPLE_TEXT: &str = crate::artifacts::raster::dsl::SEMIO_RASTER_EXAMPLE_TEXT;

//#endregion 🔖️Constants

//#region 🔖️Register
/// 🗂️ Registers `RasterSnapshot`'s pack↔dsl codec, the raster 2D media export handler and the DWG
/// import handler. Called from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::raster::composer::register();

    register_pilot_languages();
    register_artifact_schema();
    crate::apps::raster::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::raster::RasterPlayApp>(RASTER_DOCUMENT_SCHEMA);
}

pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::raster::schema::raster_artifact_schema_descriptor());
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "raster.document",
        extension: Some("raster"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::raster::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::raster::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("raster.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "raster.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::raster::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::raster::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("raster.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "raster.document.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::raster::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::raster::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("raster.document.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "raster.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("raster.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "raster.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("raster.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️DocumentHelpers
pub fn create_raster_id(prefix: &str) -> String {
    let next = {
        let hex = blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex();
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("{prefix}-{next}")
}

pub fn empty_raster_snapshot() -> RasterSnapshot {
    RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: "raster".into(), title: Some("Untitled".into()), layers: Vec::new(), assets: BTreeMap::new() }
}

//#region 🔖️Tree
pub fn layer_node_id(layer: &RasterLayerNode) -> &str {
    match layer {
        RasterLayerNode::Pixel { id, .. } | RasterLayerNode::Group { id, .. } | RasterLayerNode::Adjustment { id, .. } => id,
    }
}

pub fn layer_name(layer: &RasterLayerNode) -> &str {
    match layer {
        RasterLayerNode::Pixel { name, .. } | RasterLayerNode::Group { name, .. } | RasterLayerNode::Adjustment { name, .. } => name,
    }
}

pub fn layer_visible(layer: &RasterLayerNode) -> bool {
    match layer {
        RasterLayerNode::Pixel { visible, .. } | RasterLayerNode::Group { visible, .. } | RasterLayerNode::Adjustment { visible, .. } => *visible,
    }
}

pub fn layer_opacity(layer: &RasterLayerNode) -> f32 {
    match layer {
        RasterLayerNode::Pixel { opacity, .. } | RasterLayerNode::Group { opacity, .. } | RasterLayerNode::Adjustment { opacity, .. } => *opacity,
    }
}

pub fn find_layer<'a>(layers: &'a [RasterLayerNode], target_id: &str) -> Option<&'a RasterLayerNode> {
    for layer in layers {
        if layer_node_id(layer) == target_id {
            return Some(layer);
        }
        if let RasterLayerNode::Group { children, .. } = layer {
            if let Some(found) = find_layer(children, target_id) {
                return Some(found);
            }
        }
    }
    None
}

/// 🧭️ Finds a layer's parent-group id (`None` at the root) and its index among its siblings.
pub fn locate_layer(layers: &[RasterLayerNode], target_id: &str) -> Option<(Option<String>, usize)> {
    fn walk(layers: &[RasterLayerNode], parent: Option<&str>, target_id: &str) -> Option<(Option<String>, usize)> {
        for (index, layer) in layers.iter().enumerate() {
            if layer_node_id(layer) == target_id {
                return Some((parent.map(str::to_string), index));
            }
            if let RasterLayerNode::Group { id, children, .. } = layer {
                if let Some(found) = walk(children, Some(id), target_id) {
                    return Some(found);
                }
            }
        }
        None
    }
    walk(layers, None, target_id)
}

pub fn flatten_raster_layers(layers: &[RasterLayerNode]) -> Vec<&RasterLayerNode> {
    let mut out = Vec::new();
    fn visit<'a>(layers: &'a [RasterLayerNode], out: &mut Vec<&'a RasterLayerNode>) {
        for layer in layers {
            out.push(layer);
            if let RasterLayerNode::Group { children, .. } = layer {
                visit(children, out);
            }
        }
    }
    visit(layers, &mut out);
    out
}
//#endregion 🔖️Tree

fn create_pixel_layer(name: &str, width: u32, height: u32) -> RasterLayerNode {
    RasterLayerNode::Pixel { id: create_raster_id("layer"), name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(width), height: Some(height), image_key: None }
}

fn create_group_layer() -> RasterLayerNode {
    RasterLayerNode::Group { id: create_raster_id("group"), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() }
}

fn create_adjustment_layer() -> RasterLayerNode {
    RasterLayerNode::Adjustment {
        id: create_raster_id("adjust"),
        name: "Adjustment".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        adjustment_kind: "brightnessContrast".into(),
        params: BTreeMap::new(),
    }
}

pub fn create_layer_of_kind(kind: &str) -> RasterLayerNode {
    match kind {
        "group" => create_group_layer(),
        "adjustment" => create_adjustment_layer(),
        _ => create_pixel_layer("Layer", 512, 512),
    }
}

pub fn empty_raster_document() -> RasterSnapshot {
    let mut document = empty_raster_snapshot();
    document.id = "empty".into();
    document.layers = vec![create_pixel_layer("Background", 512, 512)];
    document
}

pub fn semio_fixture_snapshot() -> RasterSnapshot {
    let mut assets = BTreeMap::new();
    assets.insert(
        "semio-emblem".into(),
        RasterImageAsset { mime: "image/png".into(), data: base64::engine::general_purpose::STANDARD.decode("iVBORw0KGgo=").unwrap_or_default() },
    );
    let mut params = BTreeMap::new();
    params.insert("brightness".into(), dsl::to_dsl_value(&serde_json::json!(0.12)).expect("dsl value"));
    params.insert("contrast".into(), dsl::to_dsl_value(&serde_json::json!(0.08)).expect("dsl value"));
    RasterSnapshot {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: "semio-demo".into(),
        title: Some("Semio Raster Demo".into()),
        layers: vec![
            RasterLayerNode::Pixel {
                id: "backdrop".into(),
                name: "Backdrop".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(1024),
                height: Some(1024),
                image_key: Some("semio-emblem".into()),
            },
            RasterLayerNode::Adjustment {
                id: "brighten".into(),
                name: "Brighten".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                adjustment_kind: "brightnessContrast".into(),
                params,
            },
        ],
        assets,
    }
}

/// 📄️ The `semio` example document used by the app manifest and tests.
pub fn semio_example_document() -> RasterSnapshot {
    semio_fixture_snapshot()
}

/// 📄️ JSON re-serialization of {@link semio_example_document}, for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`) — out of scope to change,
/// since it is defined in `framework/plugin`.
pub fn semio_example_json() -> String {
    serde_json::to_string(&semio_example_document()).expect("serialize semio example document")
}

/// 📄️ Duplicates a layer subtree with freshly minted ids (a new document node, not an operation inverse).
pub fn clone_layer(layer: &RasterLayerNode) -> RasterLayerNode {
    match layer {
        RasterLayerNode::Pixel { name, visible, opacity, blend_mode, transform, mask, width, height, image_key, .. } => RasterLayerNode::Pixel {
            id: create_raster_id("layer"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            width: *width,
            height: *height,
            image_key: image_key.clone(),
        },
        RasterLayerNode::Group { name, visible, opacity, blend_mode, transform, mask, children, .. } => RasterLayerNode::Group {
            id: create_raster_id("group"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            children: children.iter().map(clone_layer).collect(),
        },
        RasterLayerNode::Adjustment { name, visible, opacity, blend_mode, transform, adjustment_kind, params, .. } => RasterLayerNode::Adjustment {
            id: create_raster_id("adjust"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            adjustment_kind: adjustment_kind.clone(),
            params: params.clone(),
        },
    }
}

/// 🩹️ Builds a sparse {@link RasterLayerPatch} for a `patchLayer`/`patchLayers` field write.
pub fn layer_patch_for_field(field: &str, value: &Value, prior: &RasterLayerNode) -> Option<RasterLayerPatch> {
    let mut patch = RasterLayerPatch::default();
    let opacity_of = layer_opacity(prior) as f64;
    match field {
        "name" => patch.name = Some(value.as_str().unwrap_or("").into()),
        "visible" => patch.visible = Some(value.as_bool().unwrap_or_else(|| !layer_visible(prior))),
        "opacity" => patch.opacity = Some(value.as_f64().unwrap_or(opacity_of) as f32),
        "blendMode" => patch.blend_mode = Some(value.as_str().unwrap_or("normal").into()),
        "transformX" => patch.transform_x = Some(value.as_f64().unwrap_or(0.0)),
        "transformY" => patch.transform_y = Some(value.as_f64().unwrap_or(0.0)),
        "width" => patch.width = Some(value.as_u64().unwrap_or(512) as u32),
        "height" => patch.height = Some(value.as_u64().unwrap_or(512) as u32),
        "adjustmentKind" => patch.adjustment_kind = Some(value.as_str().unwrap_or("brightnessContrast").into()),
        _ => return None,
    }
    Some(patch)
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️SemioBridge
/// 🌉️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT (W5b): raster
/// is dual-natured — its vector-shaped exports (svg) go through `s.stdio.semio/v1/drawing`, its
/// pixel asset surface (png) goes through `s.stdio.semio/v1/image` — both real stdio bridges, never
/// hand-rolled SVG/DWG bytes. Only the SVG→pixels render step has no stdio equivalent (a genuine
/// vector-rasterizer gap, reported in `stdio_gaps`); it stays on `semio_framework_os`'s real
/// usvg/resvg renderer, whose OUTPUT is then canonicalized through the real png↔semio/image codec.
const SEMIO_DRAWING_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const SEMIO_IMAGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };
const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

/// 📌️ w5b-close fix: registers stdio's `semio` v1 engine (drawing/image/… subset composers) and
/// stdio's `png` engine into the process-global `io` registry exactly once, so `io_dispatch`/
/// `io_compose_via` below resolve regardless of host-boot ordering — a bare `cargo test` process
/// never runs the plugin-host boot path that would normally call this. Mirrors 🗒️note's/📏️layout's/
/// 🌍️gis's own `ensure_..._registered()` helper (w5b-verify-report.md flagged 🖍️draw as missing this
/// exact pattern; raster was missing it too, surfaced by `cargo test` once the crate compiled).
fn ensure_stdio_semio_and_png_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::register();
        semio_s_plugin_stdio::artifacts::png::engine::register();
    });
}

/// 🗝️ Mirrors `io::IoKey::from_owner_counterpart` (private to the io module) for the four fixed
/// owner/counterpart pairs this bridge dispatches.
fn semio_io_key(owner: &Dialect, direction: IoDirection, counterpart: &Dialect) -> IoKey {
    IoKey {
        artifact_kind: owner.artifact_kind.into(),
        standard: owner.standard.0.into(),
        subset: owner.subset.0.into(),
        direction,
        format_kind: counterpart.artifact_kind.into(),
        format_standard: counterpart.standard.0.into(),
        format_subset: counterpart.subset.0.into(),
    }
}

fn semio_transform_from_raster(transform: &RasterTransform) -> SemioTransform {
    let half = transform.rotation.to_radians() / 2.0;
    SemioTransform {
        translation: SemioPoint3 { x: transform.x, y: transform.y, z: 0.0 },
        rotation: SemioQuaternion { x: 0.0, y: 0.0, z: half.sin(), w: half.cos() },
        scale: SemioPoint3 { x: transform.scale_x, y: transform.scale_y, z: 1.0 },
    }
}

/// 🖼️ Builds one real `DrawNode` per visible pixel layer (its embedded asset bytes, positioned/
/// scaled/rotated by the layer's own `RasterTransform`), recursing into group layers; adjustment
/// layers carry no geometry of their own and are honestly skipped.
fn draw_node_for_raster_layer(layer: &RasterLayerNode, assets: &BTreeMap<String, RasterImageAsset>) -> Option<DrawNode> {
    match layer {
        RasterLayerNode::Pixel { visible, transform, width, height, image_key, .. } => {
            if !*visible {
                return None;
            }
            let asset = image_key.as_ref().and_then(|key| assets.get(key))?;
            let w = width.unwrap_or(0) as f64;
            let h = height.unwrap_or(0) as f64;
            if w <= 0.0 || h <= 0.0 {
                return None;
            }
            Some(DrawNode::Group {
                transform: semio_transform_from_raster(transform),
                children: vec![DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: w, height: h, mime: asset.mime.clone(), bytes: asset.data.clone() }],
            })
        }
        RasterLayerNode::Group { visible, transform, children, .. } => {
            if !*visible {
                return None;
            }
            let kids: Vec<DrawNode> = children.iter().filter_map(|child| draw_node_for_raster_layer(child, assets)).collect();
            if kids.is_empty() {
                return None;
            }
            Some(DrawNode::Group { transform: semio_transform_from_raster(transform), children: kids })
        }
        RasterLayerNode::Adjustment { .. } => None,
    }
}

/// 🧬️ Builds a real `SemioDrawingSnapshot` from a raster document's own layer stack (its own
/// domain document model), replacing the `title_card_svg` placeholder.
fn drawing_snapshot_from_raster(document: &RasterSnapshot) -> SemioDrawingSnapshot {
    let mut max_x = 0.0f64;
    let mut max_y = 0.0f64;
    for layer in &document.layers {
        if let RasterLayerNode::Pixel { transform, width, height, .. } = layer {
            max_x = max_x.max(transform.x + width.unwrap_or(0) as f64);
            max_y = max_y.max(transform.y + height.unwrap_or(0) as f64);
        }
    }
    let canvas = DrawCanvas { width: if max_x > 0.0 { max_x } else { 1024.0 }, height: if max_y > 0.0 { max_y } else { 1024.0 }, background: None };
    let children: Vec<DrawNode> = document.layers.iter().filter_map(|layer| draw_node_for_raster_layer(layer, &document.assets)).collect();
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas,
        styles: Vec::new(),
        layers: vec![DrawLayer { id: document.id.clone(), name: document.title.clone().unwrap_or_default(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

/// 🧬️ Converts a legacy `DwgDrawing`'s line-shaped entities into a real `DrawNode::Path` tree —
/// typed geometry, not hand-formatted SVG `<path d="…">` strings.
fn drawing_snapshot_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> SemioDrawingSnapshot {
    let width = (drawing.extmax[0] - drawing.extmin[0]).max(1.0);
    let height = (drawing.extmax[1] - drawing.extmin[1]).max(1.0);
    let to_point = |v: &[f64; 2]| SemioPoint2 { x: v[0] - drawing.extmin[0], y: height - (v[1] - drawing.extmin[1]) };
    let mut children = Vec::new();
    for entity in &drawing.entities {
        let (vertices, closed): (Vec<[f64; 2]>, bool) = match &entity.geometry {
            semio_framework_os::DwgGeometry::LwPolyline { vertices, closed, .. } => (vertices.clone(), *closed),
            semio_framework_os::DwgGeometry::Polyline3d { vertices, closed } => (vertices.iter().map(|v| [v[0], v[1]]).collect(), *closed),
            semio_framework_os::DwgGeometry::Line { start, end } => (vec![[start[0], start[1]], [end[0], end[1]]], false),
            _ => continue,
        };
        if vertices.is_empty() {
            continue;
        }
        let mut segments = Vec::with_capacity(vertices.len() + 1);
        segments.push(PathSegment::MoveTo { to: to_point(&vertices[0]) });
        for vertex in &vertices[1..] {
            segments.push(PathSegment::LineTo { to: to_point(vertex) });
        }
        if closed {
            segments.push(PathSegment::Close);
        }
        children.push(DrawNode::Path { segments, style: None });
    }
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width, height, background: None },
        styles: Vec::new(),
        layers: vec![DrawLayer { id: "0".into(), name: "dwg-import".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

/// 🚪️ Dispatches `s.stdio.semio/v1/drawing` → `s.stdio.svg` through stdio's real SVG serializer
/// (`io_dispatch`), then prints the composed `SvgSnapshot` as bare XML (`write_svg_xml`, NOT
/// `ArtifactDsl::print_dsl` — w5b-close fix: `print_dsl` wraps the text in stdio's `.semio`
/// envelope preamble, which `raster_document_json_from_dwg`'s `semio_framework_os::
/// rasterize_svg_to_png_base64` call below then fails to parse as XML at all ("unknown token at
/// 1:1"); every downstream consumer of this function's return value wants a bare `<svg>…</svg>`
/// document, matching 🗒️note's/🖍️draw's own `write_svg_xml` usage for the identical bridge).
fn dispatch_drawing_to_svg(snapshot: &SemioDrawingSnapshot) -> Result<String, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(snapshot));
    let key = semio_io_key(&SEMIO_DRAWING_DIALECT, IoDirection::Export, &SVG_DIALECT);
    let composed = io_dispatch(&key, &[ErasedComposeSource { dialect: SEMIO_DRAWING_DIALECT, payload }]).map_err(|error| error.message)?;
    let IoPayload::Binary(svg_bytes) = composed.payload else { return Err("s.stdio.svg composer returned a non-binary payload".into()) };
    let svg_snapshot = <SvgSnapshot as store::ArtifactPack>::decode_pack(&svg_bytes).map_err(|error| format!("{error:?}"))?;
    Ok(semio_s_plugin_stdio::artifacts::svg::schema::snapshot::write_svg_xml(&svg_snapshot.doc))
}

/// 🚪️ Dispatches real `png` bytes → `s.stdio.semio/v1/image` through stdio's real PNG deserializer
/// (`io_dispatch`) — the honest, structured way to learn a decoded image's real width/height/pixels.
fn semio_image_from_png_bytes(raw_png_bytes: &[u8]) -> Result<SemioImageSnapshot, String> {
    ensure_stdio_semio_and_png_registered();
    let png_snapshot = semio_s_plugin_stdio::artifacts::png::engine::decode_png(raw_png_bytes)?;
    let payload = IoPayload::Binary(<PngSnapshot as store::ArtifactPack>::encode_pack(&png_snapshot));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Import, &PNG_DIALECT);
    let composed = io_dispatch(&key, &[ErasedComposeSource { dialect: PNG_DIALECT, payload }]).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.semio image composer returned a non-binary payload".into()) };
    <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))
}

/// 🚪️ Dispatches `s.stdio.semio/v1/image` → real `png` bytes through stdio's real PNG serializer
/// (`io_dispatch`) plus its own real byte encoder — never a hand-rolled PNG writer.
fn png_bytes_from_semio_image(image: &SemioImageSnapshot) -> Result<Vec<u8>, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<SemioImageSnapshot as store::ArtifactPack>::encode_pack(image));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Export, &PNG_DIALECT);
    let composed = io_dispatch(&key, &[ErasedComposeSource { dialect: SEMIO_IMAGE_DIALECT, payload }]).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.png composer returned a non-binary payload".into()) };
    let png_snapshot = <PngSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))?;
    semio_s_plugin_stdio::artifacts::png::engine::encode_png(&png_snapshot)
}

/// 🌉️🌉️ Round-trips raw PNG bytes through `s.stdio.semio/v1/image` (import then export) via the
/// real 2-hop `io_compose_via` seam — canonicalizes a renderer's raw output through stdio's own
/// codec rather than trusting it verbatim.
fn canonicalize_png_bytes(raw_png_bytes: &[u8]) -> Result<Vec<u8>, String> {
    ensure_stdio_semio_and_png_registered();
    let png_snapshot = semio_s_plugin_stdio::artifacts::png::engine::decode_png(raw_png_bytes)?;
    let payload = IoPayload::Binary(<PngSnapshot as store::ArtifactPack>::encode_pack(&png_snapshot));
    let hub_key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Import, &PNG_DIALECT);
    let target_key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Export, &PNG_DIALECT);
    let composed = io_compose_via(&hub_key, &target_key, &[ErasedComposeSource { dialect: PNG_DIALECT, payload }]).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.png composer returned a non-binary payload".into()) };
    let png_snapshot = <PngSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))?;
    semio_s_plugin_stdio::artifacts::png::engine::encode_png(&png_snapshot)
}
//#endregion 🔖️SemioBridge

//#region 🔖️MediaExport
/// 📤️ Real vector export: the document's visible layer stack becomes a `SemioDrawingSnapshot`
/// (real geometry, own domain model), composed into real SVG text via stdio's `s.stdio.semio/v1/
/// drawing` → `s.stdio.svg` bridge (`io_dispatch`) — no more `title_card_svg` placeholder.
pub fn raster_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let document: RasterSnapshot = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    let drawing = drawing_snapshot_from_raster(&document);
    let svg = dispatch_drawing_to_svg(&drawing)?;
    Ok((svg, drawing.canvas.width.round().max(1.0) as u32, drawing.canvas.height.round().max(1.0) as u32))
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// 📥️ Rewires the DWG import path onto real stdio bridges: the DWG entities become a real
/// `SemioDrawingSnapshot` (`drawing_snapshot_from_dwg`), composed to real SVG text via
/// `s.stdio.semio/v1/drawing` (`io_dispatch`). SVG→pixels still needs a real vector renderer — no
/// stdio bridge does that (reported `stdio_gaps`) — so `semio_framework_os`'s real usvg/resvg
/// renderer stays, but its raw PNG bytes are then canonicalized through the real
/// `s.stdio.semio/v1/image` ↔ png round trip (`canonicalize_png_bytes`) instead of being trusted
/// verbatim, which also recovers the real decoded width/height for the new pixel layer.
pub fn raster_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
    let drawing_snapshot = drawing_snapshot_from_dwg(drawing);
    let svg = dispatch_drawing_to_svg(&drawing_snapshot)?;
    let fallback_width = drawing_snapshot.canvas.width.round().max(1.0) as u32;
    let fallback_height = drawing_snapshot.canvas.height.round().max(1.0) as u32;
    let rendered = semio_framework_os::rasterize_svg_to_png_base64(&svg, fallback_width, fallback_height)?;
    let raw_bytes = base64::engine::general_purpose::STANDARD.decode(rendered.as_bytes()).map_err(|error| error.to_string())?;
    let (data, width, height) = match semio_image_from_png_bytes(&raw_bytes).and_then(|image| Ok((png_bytes_from_semio_image(&image)?, image.width, image.height))) {
        Ok((bytes, width, height)) => (bytes, width, height),
        Err(_) => (raw_bytes, fallback_width, fallback_height),
    };
    let asset_key = create_raster_id("dwg-asset");
    let mut layer = create_pixel_layer("DWG Import", width, height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let mut assets = BTreeMap::new();
    assets.insert(asset_key, RasterImageAsset { mime: "image/png".into(), data });
    let document = RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: create_raster_id("dwg-import"), title: Some("DWG Import".into()), layers: vec![layer], assets };
    serde_json::to_value(&document).map_err(|error| error.to_string())
}

/// 📥️ Builds the whole-document replacement that appends the incoming `image:in` media as one new
/// pixel layer + embedded asset — `RasterMutation` has no granular "add asset" step (assets are
/// seeded with the document today), so this mirrors `raster_document_json_from_dwg`'s "compute the
/// whole next document, then `ReplaceDocument`" shape rather than inventing a narrower op. `png_base64`
/// is raw (unprefixed) base64 PNG bytes — the same convention `shooting_engine::shooting_photo_media`
/// produces on `photos:out` and the Vector→Raster run-crate converter produces for a draw `vector:out`
/// source. Real decode through `s.stdio.semio/v1/image` (`semio_image_from_png_bytes`) recovers the
/// real width/height instead of leaving them unset, and re-encodes through the real serializer
/// instead of storing the caller's bytes verbatim.
pub fn raster_append_image_layer(document: &RasterSnapshot, png_base64: &str) -> RasterSnapshot {
    let asset_key = create_raster_id("image-in-asset");
    let raw_bytes = base64::engine::general_purpose::STANDARD.decode(png_base64.as_bytes()).unwrap_or_default();
    let (data, width, height) = match semio_image_from_png_bytes(&raw_bytes).and_then(|image| Ok((png_bytes_from_semio_image(&image)?, image.width, image.height))) {
        Ok((bytes, width, height)) => (bytes, Some(width), Some(height)),
        Err(_) => (raw_bytes, None, None),
    };
    let mut layer = create_pixel_layer("Imported Image", width.unwrap_or(0), height.unwrap_or(0));
    if let RasterLayerNode::Pixel { image_key, width: layer_width, height: layer_height, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
        *layer_width = width;
        *layer_height = height;
    }
    let mut next = document.clone();
    next.assets.insert(asset_key, RasterImageAsset { mime: "image/png".into(), data });
    next.layers.push(layer);
    next
}
//#endregion 🔖️MediaImport

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `2d.raster`
/// `ArtifactKindSpec` literal `crate::artifacts::raster::artifact_kind` already declares, plus the
/// app-specific `image:in`/`image:out` ports (see below).
pub fn raster_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        document_schema: RASTER_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework::MediaType { class: semio_framework::MediaClass::TwoD, form: semio_framework::MediaForm::Raster },
        ports: vec![raster_image_in_port(), raster_image_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: "2d.raster".into(), name: "2D Raster".into(), dimension: "2d".into(), component_kind: "raster".into() },
    }
}

/// 🔌️ `image:in` — accepts raster imagery from upstream producers (e.g. draw's `vector:out`,
/// converted Vector→Raster) as a new composited layer. `Many`/optional: several upstream images may
/// feed in, and the port may sit unconnected.
pub fn raster_image_in_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "image:in".into(),
        label: "Image".into(),
        direction: semio_framework::MediaPortDirection::In,
        media_type: semio_framework::MediaType { class: semio_framework::MediaClass::TwoD, form: semio_framework::MediaForm::Raster },
        kind_id: None,
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🔌️ `image:out` — the raster document's current composited raster, as `2d.image` media (workflow
/// port surface; WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). `kind_id: "2d.image"` — the
/// shared framework-builtin interchange kind (declared on this app's `.artifact_kind(...)` below;
/// `shooting`'s `photos:out` declares the identical shape, harmless duplicate registrations).
pub fn raster_image_out_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "image:out".into(),
        label: "Image".into(),
        direction: semio_framework::MediaPortDirection::Out,
        media_type: semio_framework::MediaType { class: semio_framework::MediaClass::TwoD, form: semio_framework::MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🖼️ Composites the current raster document to a PNG `Media` payload for the `image:out` port —
/// `raster_document_json_to_svg` now renders the document's real layer stack (not a placeholder
/// title card) via the `s.stdio.semio/v1/drawing` bridge; the vector→pixels render step still has
/// no stdio bridge (real pixel compositing is wgpu/canvas-host-side, out of this pure headless
/// compute node's reach — see that function's own doc), so its raw renderer output is canonicalized
/// through the real `s.stdio.semio/v1/image` ↔ png round trip (`canonicalize_png_bytes`, the
/// `io_compose_via` 2-hop seam) before leaving this port.
pub fn raster_composite_media(document: &RasterSnapshot) -> Result<semio_framework::Media, semio_framework::MediaError> {
    let value = serde_json::to_value(document).map_err(|error| semio_framework::MediaError::Payload("image:out".into(), error.to_string()))?;
    let (svg, width, height) = raster_document_json_to_svg(&value).map_err(|error| semio_framework::MediaError::Payload("image:out".into(), error))?;
    let rendered = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).map_err(|error| semio_framework::MediaError::Payload("image:out".into(), error))?;
    let raw_bytes = base64::engine::general_purpose::STANDARD.decode(rendered.as_bytes()).map_err(|error| semio_framework::MediaError::Payload("image:out".into(), error.to_string()))?;
    let canonical = canonicalize_png_bytes(&raw_bytes).map_err(|error| semio_framework::MediaError::Payload("image:out".into(), error))?;
    let png_base64 = base64::engine::general_purpose::STANDARD.encode(canonical);
    Ok(semio_framework::Media {
        media_type: semio_framework::MediaType { class: semio_framework::MediaClass::TwoD, form: semio_framework::MediaForm::Raster },
        payload: semio_framework::MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 },
    })
}
//#endregion 🔖️Io

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_dwg_polyline_into_raster_document() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(semio_framework_os::DwgEntity {
            layer,
            color: semio_framework_os::DwgColor::ByLayer,
            geometry: semio_framework_os::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]], bulges: vec![0.0, 0.0, 0.0, 0.0] },
        });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [10.0, 10.0, 0.0];
        let value = raster_document_json_from_dwg(&drawing).expect("dwg import");
        let document: RasterSnapshot = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert_eq!(asset.mime, "image/png");
        assert!(!asset.data.is_empty());
    }

    #[test]
    fn imports_empty_dwg_into_blank_raster_document() {
        let drawing = semio_framework_os::DwgDrawing::default();
        let value = raster_document_json_from_dwg(&drawing).expect("empty dwg import");
        let document: RasterSnapshot = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, width, height, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        assert_eq!(*width, Some(1));
        assert_eq!(*height, Some(1));
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert!(!asset.data.is_empty());
    }

    #[test]
    fn raster_io_declares_image_in_and_image_out() {
        let io = raster_io();
        assert_eq!(io.document_schema, RASTER_DOCUMENT_SCHEMA);
        assert_eq!(io.artifact.id, "2d.raster");
        assert!(io.ports.iter().any(|p| p.id == "image:in"));
        let out_port = raster_image_out_port();
        assert_eq!(out_port.kind_id.as_deref(), Some("2d.image"));
    }

    #[test]
    fn raster_append_image_layer_inserts_layer_and_asset() {
        let document = empty_raster_document();
        let before_layers = document.layers.len();
        let next = raster_append_image_layer(&document, "aGVsbG8=");
        assert_eq!(next.layers.len(), before_layers + 1);
        let RasterLayerNode::Pixel { image_key, .. } = next.layers.last().unwrap() else { panic!("expected pixel layer") };
        let asset = next.assets.get(image_key.as_ref().unwrap()).expect("asset inserted");
        assert_eq!(asset.data, b"hello".to_vec());
    }

    #[test]
    fn raster_composite_media_exports_structured_2d_image_payload() {
        let document = empty_raster_document();
        let media = raster_composite_media(&document).expect("export image:out");
        let semio_framework::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.image");
        assert!(!json.is_empty());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent artifact engine — owns the full artifact; `snapshot()` is its persisted subset.
pub struct RasterEngine {
    artifact: crate::artifacts::raster::schema::RasterArtifact,
    snapshot: RasterSnapshot,
}

impl RasterEngine {
    /// 🏗️ Seeds the engine from a persisted snapshot.
    pub fn new(snapshot: RasterSnapshot) -> Self {
        let artifact = crate::artifacts::raster::schema::RasterArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    /// 📸️ Consumes the engine and returns its persisted snapshot.
    pub fn into_snapshot(self) -> RasterSnapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine

