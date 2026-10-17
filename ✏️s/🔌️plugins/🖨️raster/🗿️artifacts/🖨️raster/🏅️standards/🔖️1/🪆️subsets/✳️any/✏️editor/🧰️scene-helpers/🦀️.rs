//! 🧰️ Raster editor scene helpers: the reset document effect, the controller and tree constants, the layer and mask row identifiers, the document sync and asset JSON, the scene projection and the UI value builders.

use crate::config::RasterConfig;
use semio_s_artifact_raster_raster_model::{RasterLayerNode, RASTER_DOCUMENT_SCHEMA};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_pack_json::Value;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionFactory;
use pack::snapshot::ArtifactPack;
use semio_s_artifact_raster_raster_model::{find_layer, layer_blend_mode, layer_node_id, layer_visible, locate_layer, require_layer_edit};
use semio_framework_plugin::Fault;

/// 🧭️ Where a duplicate of layer `id` lands: next to its source, in the source coordinate frame, and only below an editable parent. Shared by the duplicate command and the inspection panel, which both ask the same question.
pub fn duplicate_layer_plan(document:&RasterSnapshot,id:&str)->Result<(Option<String>,usize),Fault> {
    let (parent,index)=locate_layer(&document.layers,id).ok_or_else(||Fault::from("raster-layer-not-found"))?;
    if let Some(parent)=&parent {require_layer_edit(&document.layers,parent,false).map_err(Fault::from)?;}
    Ok((parent,index+1))
}

/// 🌱️ Load effect for a whole document: example switches, JSON loads and imports replace the document through the
/// artifact's load path, never as mutation rows or history.
pub fn raster_reset_document_effect(document: &RasterSnapshot) -> semio_framework_plugin::Effect {
    let pack = <RasterSnapshot as ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr(&document.id.to_string(), RASTER_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

/// 🫳️ The adjacent normal-blend pair a merge-down of `layer_id` would bake: refuses unknown or lowest layers, backdrop-dependent siblings and protected layers. Shared by the merge-down command and the inspection panel, which both ask the same question.
pub fn merge_down_plan<'a>(document:&'a RasterSnapshot,layer_id:&str)->Result<(Option<String>,usize,&'a [RasterLayerNode]),Fault> {
    if layer_id.trim().is_empty()||layer_id.chars().count()>128 {return Err(Fault::from("raster.merge-target-invalid"));}
    let (parent,index)=locate_layer(&document.layers,layer_id).ok_or_else(||Fault::from("raster.merge-target-missing"))?;
    if index==0 {return Err(Fault::from("raster.merge-no-lower-layer"));}
    let siblings=match parent.as_deref().and_then(|id|find_layer(&document.layers,id)) {Some(RasterLayerNode::Group {children,..})=>children.as_slice(),_=>document.layers.as_slice()};
    let pair=&siblings[index-1..=index];
    if pair.iter().any(|layer|!layer_visible(layer)||layer_blend_mode(layer)!="normal"||matches!(layer,RasterLayerNode::Adjustment {..})) {return Err(Fault::from("raster.merge-backdrop-dependent"));}
    for layer in pair {semio_s_artifact_raster_raster_model::require_layer_edit(&document.layers,layer_node_id(layer),true).map_err(Fault::from)?;}
    Ok((parent,index-1,pair))
}

//#region 🔖️Constants
pub const RASTER_PLAY_CONTROLLER_ID: &str = "raster-play";

/// 🌳️ Prefix for every layer-tree row id — shared by the document/masks panels and the `moveLayer`
/// command (which needs to decode a `target_row_id` back into a layer/group id). App-wide tree-encoding
/// concern, not artifact data, so it lives here rather than in any single panel.
pub const RASTER_TREE_PREFIX: &str = "raster-play-layers";

/// 🕹️ The single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM interaction domain this app declares
/// (granularity `layer`, `HierarchyProvider::Topology`, method Pick) — the layer tree binds it and the
/// composite window references it.
pub const RASTER_INTERACTION_DOMAIN: &str = "layers";

pub const RASTER_INTERACTION_GRANULARITY: &str = "layer";
//#endregion 🔖️Constants

//#region 🔖️Document
/// 🌳️ Encodes a layer as its tree-row id — shared by the document/masks panels (which render rows) and
/// `moveLayer` (which decodes a drop target back into an id). More than one consumer, but this is UI row
/// encoding, not artifact data, so it stays app-level rather than in `crate::schema`.
pub fn layer_row_id(layer: &RasterLayerNode) -> String {
    semio_s_artifact_raster_raster_model::layer_node_id(layer).to_string()
}

pub fn mask_row_id(target_id: &str) -> String {
    format!("{RASTER_TREE_PREFIX}.mask.{target_id}")
}

/// 📡️ Document JSON for the WASM compositor, omitting embedded assets — mirrors premigration
/// `rasterDocumentToSyncJson`. Stays app-level next to {@link raster_scene}, its only caller.
///
/// 🛡️ Built field by field rather than from `RasterSnapshot`'s own `ToValue`, because the compositor
/// reads its pixels from `assetsJson` (resolved content) and must not also carry the `assets` HANDLE
/// pool. The layer forest itself — including a group's children and an adjustment's `params` owned
/// map — goes through the artifact's real derived codec.
pub fn document_sync_json(document: &RasterSnapshot) -> String {
    let mut fields = vec![("schema".to_string(), semio_framework_value::DslValue::String(document.schema.clone())), ("id".to_string(), semio_framework_value::DslValue::String(document.id.clone()))];
    if let Some(title) = &document.title {
        fields.push(("title".to_string(), semio_framework_value::DslValue::String(title.clone())));
    }
    fields.push(("layers".to_string(), semio_framework_value::DslValue::Array(document.layers.iter().map(semio_framework_value::ToValue::to_value).collect())));
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::DslValue::Object(fields)).to_string()
}

/// 🧩️ Resolves every asset handle on `document.assets` back to its real `RasterImageAsset` bytes
/// through the working-scene cache accessor (`crate::raster_asset`, ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — `document.assets` now stores composed
/// `s.stdio.semio.image` CHILD handles, not embedded bytes) — the ONE call site the WASM compositor's
/// real pixel bytes funnel through. A handle whose content is not (or no longer) cached is honestly
/// omitted rather than serialized as an empty/garbage blob (documented staleness gap, matches every
/// other exemplar in this ticket).
fn assets_json_from_document(document: &RasterSnapshot) -> String {
    let resolved: std::collections::BTreeMap<String, semio_s_artifact_raster_raster_model::RasterImageAsset> = document.assets.keys().filter_map(|asset_id| semio_s_artifact_raster_raster_io::raster_asset(&document.assets, asset_id).map(|asset| (asset_id.clone(), asset))).collect();
    let object: semio_framework_pack_json::Object = resolved.into_iter().map(|(id, asset)| (id, semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&asset)))).collect();
    semio_framework_pack_json::to_string(&Value::Object(object))
}

/// 🎞️ The composite and navigator read the same document, session style and framework selection.
pub fn raster_scene(document: &RasterSnapshot, runtime: &RasterConfig, active_utility: &str, view_mode: &str, selected_ids: &[String], hovered_id: Option<&str>) -> semio_framework_plugin::Paint2dScene {
    semio_framework_plugin::Paint2dScene {
        document_sync_json: document_sync_json(document),
        assets_json: assets_json_from_document(document),
        camera_json: semio_framework_pack_json::to_json_string(&runtime.camera),
        selection_json: semio_framework_pack_json::to_json_string(&selected_ids.to_vec()),
        hovered_id: hovered_id.map(str::to_string),
        active_utility: active_utility.into(),
        brush_size: runtime.brush_size,
        brush_opacity: runtime.brush_opacity,
        brush_color: runtime.brush_color.clone(),
        brush_hardness: runtime.brush_hardness,
        paint_target:runtime.paint_target.clone(),
        mask_value:runtime.mask_value,
        fill_tolerance:runtime.fill_tolerance,
        pixel_selection_json:runtime.pixel_selection.as_ref().map(semio_framework_pack_json::to_json_string),
        view_mode: view_mode.into(),
        composite_viewport_json: runtime.composite_viewport.as_ref().map(semio_framework_pack_json::to_json_string),
        lanes: Vec::new(),
    }
}

/// 🎬️ Builds the semantic-UI action binding dispatched through the raster app's single controller — the
/// one call site every panel/window *node* goes through. Not for `WindowMeasure` chrome, which rides the
/// renderer's own [`ActionDescriptor`] record — see [`raster_measure_action`].
pub fn raster_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    ActionFactory::new(RASTER_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🎚️ Builds the window-chrome `ActionDescriptor` a [`WindowMeasure`] carries — the measure tree is a
/// renderer-side record, not a semantic-UI node, so it takes the descriptor verbatim rather than the
/// fixed-capacity `(ActionId, Option<UiValue>)` pair [`raster_action`] returns.
pub fn raster_measure_action(action: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: RASTER_PLAY_CONTROLLER_ID.into(), action: action.into(), args: None }
}

/// 🏷️ Admits one resolved raster string into the semantic UI contract's fixed-capacity label owner —
/// every panel/window label goes through here rather than the renderer's unbounded `Label`.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster UI label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}
//#endregion 🔖️Document
