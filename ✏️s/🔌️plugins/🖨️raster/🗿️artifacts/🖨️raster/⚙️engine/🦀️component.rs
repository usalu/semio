//! ⚙️ Raster artifact — headless compute (constitutional: engine).

use crate::artifacts::raster::{RasterImageAsset, RasterLayerNode, RasterLayerPatch, RasterProjection, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Constants
/// 📄️ The `semio` example document, handcrafted in the `.raster` DSL — {@link semio_example_document}/
/// {@link semio_example_json} are the only ways it should be consumed.
const SEMIO_RASTER_EXAMPLE_TEXT: &str = crate::artifacts::raster::dsl::SEMIO_RASTER_EXAMPLE_TEXT;

//#endregion 🔖️Constants

//#region 🔖️Register
/// 🗂️ Registers `RasterProjection`'s pack↔dsl codec, the raster 2D media export handler and the DWG
/// import handler. Called from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    register_pilot_languages();
    semio_framework_os::register_2d_export_handlers("2d.raster", "raster", raster_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.raster", raster_document_json_from_dwg);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::raster::RasterPlayApp>(RASTER_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "raster.document",
        extension: Some("raster"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::raster::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::raster::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::raster::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::raster::pack::COMPONENT_PROTOCOL_PATH),
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
        protocol: Some(crate::artifacts::raster::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::raster::pack::COMPONENT_PROTOCOL_PATH),
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

pub fn empty_raster_projection() -> RasterProjection {
    RasterProjection { schema: RASTER_DOCUMENT_SCHEMA.into(), id: "raster".into(), title: Some("Untitled".into()), layers: Vec::new(), assets: BTreeMap::new() }
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

pub fn empty_raster_document() -> RasterProjection {
    let mut document = empty_raster_projection();
    document.id = "empty".into();
    document.layers = vec![create_pixel_layer("Background", 512, 512)];
    document
}

/// 📄️ The `semio` example, parsed once from {@link SEMIO_RASTER_EXAMPLE_TEXT} — the source of truth for
/// every "semio" example call site (`setActiveExample`, tests). Falls back to the empty document if the
/// fixture ever fails to parse, matching the old JSON fixture's failure behavior.
pub fn semio_example_document() -> RasterProjection {
    <RasterProjection as store::DocumentDsl>::parse_dsl(SEMIO_RASTER_EXAMPLE_TEXT).unwrap_or_else(|_| empty_raster_document())
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

//#region 🔖️MediaExport
pub fn raster_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Raster", 1024, 1024)
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// 📥️ Rasterizes a DWG drawing's flat SVG projection into a single-layer raster document.
pub fn raster_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
    let (svg, width, height) = semio_framework_os::dwg_drawing_to_svg(drawing)?;
    let data = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height)?;
    let asset_key = create_raster_id("dwg-asset");
    let mut layer = create_pixel_layer("DWG Import", width, height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let mut assets = BTreeMap::new();
    assets.insert(asset_key, RasterImageAsset { mime: "image/png".into(), data });
    let document = RasterProjection { schema: RASTER_DOCUMENT_SCHEMA.into(), id: create_raster_id("dwg-import"), title: Some("DWG Import".into()), layers: vec![layer], assets };
    serde_json::to_value(&document).map_err(|error| error.to_string())
}

/// 📥️ Builds the whole-document replacement that appends the incoming `image:in` media as one new
/// pixel layer + embedded asset — `RasterOperation` has no granular "add asset" step (assets are
/// seeded with the document today), so this mirrors `raster_document_json_from_dwg`'s "compute the
/// whole next document, then `ReplaceDocument`" shape rather than inventing a narrower op. `png_base64`
/// is raw (unprefixed) base64 PNG bytes — the same convention `shooting_engine::shooting_photo_media`
/// produces on `photos:out` and the Vector→Raster run-crate converter produces for a draw `vector:out`
/// source.
pub fn raster_append_image_layer(document: &RasterProjection, png_base64: &str) -> RasterProjection {
    let asset_key = create_raster_id("image-in-asset");
    let mut layer = create_pixel_layer("Imported Image", 0, 0);
    if let RasterLayerNode::Pixel { image_key, width, height, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
        *width = None;
        *height = None;
    }
    let mut next = document.clone();
    next.assets.insert(asset_key, RasterImageAsset { mime: "image/png".into(), data: png_base64.to_string() });
    next.layers.push(layer);
    next
}
//#endregion 🔖️MediaImport

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `2d.raster`
/// `ArtifactKindSpec` literal `crate::artifacts::raster::artifact_kind` already declares, plus the
/// app-specific `image:in`/`image:out` ports (see below).
pub fn raster_io() -> semio_framework_core::AppIo {
    semio_framework_core::AppIo {
        document_schema: RASTER_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_core::MediaType { class: semio_framework_core::MediaClass::TwoD, form: semio_framework_core::MediaForm::Raster },
        ports: vec![raster_image_in_port(), raster_image_out_port()],
        export_formats: vec![semio_framework_core::OsMediaFormat::Svg, semio_framework_core::OsMediaFormat::Png],
        import_formats: vec![semio_framework_core::OsMediaFormat::Svg, semio_framework_core::OsMediaFormat::Png],
        artifact: semio_framework_core::ArtifactPresentation { id: "2d.raster".into(), name: "2D Raster".into(), dimension: "2d".into(), component_kind: "raster".into() },
    }
}

/// 🔌️ `image:in` — accepts raster imagery from upstream producers (e.g. draw's `vector:out`,
/// converted Vector→Raster) as a new composited layer. `Many`/optional: several upstream images may
/// feed in, and the port may sit unconnected.
pub fn raster_image_in_port() -> semio_framework_core::MediaPortSpec {
    semio_framework_core::MediaPortSpec {
        id: "image:in".into(),
        label: "Image".into(),
        direction: semio_framework_core::MediaPortDirection::In,
        media_type: semio_framework_core::MediaType { class: semio_framework_core::MediaClass::TwoD, form: semio_framework_core::MediaForm::Raster },
        kind_id: None,
        required: false,
        multiplicity: semio_framework_core::PortMultiplicity::Many,
    }
}

/// 🔌️ `image:out` — the raster document's current composited raster, as `2d.image` media (workflow
/// port surface; WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). `kind_id: "2d.image"` — the
/// shared framework-builtin interchange kind (declared on this app's `.artifact_kind(...)` below;
/// `shooting`'s `photos:out` declares the identical shape, harmless duplicate registrations).
pub fn raster_image_out_port() -> semio_framework_core::MediaPortSpec {
    semio_framework_core::MediaPortSpec {
        id: "image:out".into(),
        label: "Image".into(),
        direction: semio_framework_core::MediaPortDirection::Out,
        media_type: semio_framework_core::MediaType { class: semio_framework_core::MediaClass::TwoD, form: semio_framework_core::MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework_core::PortMultiplicity::Many,
    }
}

/// 🖼️ Composites the current raster document to a PNG `Media` payload for the `image:out` port —
/// reuses the same flat-SVG-then-rasterize pipeline `raster_document_json_from_dwg`'s inverse
/// (`dwg_drawing_to_svg`) doesn't apply here, so this renders the document's own composite instead:
/// `title_card_svg` is a placeholder card today (see `raster_document_json_to_svg`'s doc — real pixel
/// compositing is wgpu/canvas-host-side, out of this pure headless compute node's reach), rasterized
/// to PNG for a real, always-available (if generic) `image:out` value.
pub fn raster_composite_media(document: &RasterProjection) -> Result<semio_framework_core::Media, semio_framework_core::MediaError> {
    let value = serde_json::to_value(document).map_err(|error| semio_framework_core::MediaError::Payload("image:out".into(), error.to_string()))?;
    let (svg, _width, _height) = raster_document_json_to_svg(&value).map_err(|error| semio_framework_core::MediaError::Payload("image:out".into(), error))?;
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, 0, 0).map_err(|error| semio_framework_core::MediaError::Payload("image:out".into(), error))?;
    Ok(semio_framework_core::Media {
        media_type: semio_framework_core::MediaType { class: semio_framework_core::MediaClass::TwoD, form: semio_framework_core::MediaForm::Raster },
        payload: semio_framework_core::MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 },
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
        let document: RasterProjection = serde_json::from_value(value).expect("valid raster document");
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
        let document: RasterProjection = serde_json::from_value(value).expect("valid raster document");
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
        assert_eq!(asset.data, "aGVsbG8=");
    }

    #[test]
    fn raster_composite_media_exports_structured_2d_image_payload() {
        let document = empty_raster_document();
        let media = raster_composite_media(&document).expect("export image:out");
        let semio_framework_core::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.image");
        assert!(!json.is_empty());
    }
}
//#endregion 🧪️Tests
