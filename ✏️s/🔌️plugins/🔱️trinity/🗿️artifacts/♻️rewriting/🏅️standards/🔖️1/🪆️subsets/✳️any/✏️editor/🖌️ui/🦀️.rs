//! 🖌️ Rewriting editor UI helpers: action descriptors, UI value builders and the semantic rule-graph renderer the windows share.

use crate::editor::rewriting::window_config;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::WindowMeasure;
use semio_framework_os_kernel::Viewport2d;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_plugin::app::InteractionView;
use crate::editor::rewriting::ids::*;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as SemanticSurfaceKind;

//#region 🖌️UiHelpers
pub fn rewriting_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(TRINITY_REWRITING_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Binds window chrome through its retained renderer action descriptor.
pub fn rewriting_window_action(action: &str, args: Option<semio_framework_pack_json::Value>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionDescriptor { controller_id: TRINITY_REWRITING_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)) }
}

/// 🏷️ Admits resolved Rewriting text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "Rewriting UI label admission failed"))
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
//#endregion 🖌️UiHelpers

//#region 🔖️Render
pub fn rewriting_lod_json_for_window(cfg: &window_config::RewritingWindowConfig) -> String {
    let mode = cfg.lod_mode.as_str();
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        semio_framework_pack_json::json!({ "automatic": true }).to_string()
    } else {
        semio_framework_pack_json::json!({ "automatic": false, "forcedLabel": mode }).to_string()
    }
}

pub fn trinity_rewriting_lod_measure(window_id: &str, current_mode: &str) -> WindowMeasure {
    let mut items = vec![semio_framework_plugin::MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    let rows: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::parse(&semio_s_artifact_trinity_jack::editor::jack::lod::trinity_lod_scale_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_array().map(|values| values.to_vec())).unwrap_or_default();
    items.extend(rows.into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(semio_framework_plugin::MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: format!("{window_id}-lod"), label: Some("LOD".into()), value: current_mode.into(), items, on_change: rewriting_window_action("setLodMode", Some(semio_framework_pack_json::json!({ "windowId": window_id }))) }
}

/// 🕹️ `selection`/`hover` are left unset: `ArtifactApp::render` has no `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` gained one — see `📌️panels/🔍️inspection`'s doc comment
/// on `editor::jack` for the same framework-side gap) and this static scene isn't a `UiNode::Tree` the
/// wrapper's `stamp_and_cache_interaction_ui` post-pass would stamp either. The live node-graph host
/// reads domain "graph"'s `DomainSelection`/`DomainHover` directly, so the interactive surface stays
/// correct even though this snapshot doesn't carry it.
pub fn render_graph_snapshot(surface_id: &str, snapshot: &JackSnapshot, cfg: &window_config::RewritingWindowConfig, editable: bool) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let (nodes, edges, scene_viewport) = semio_s_artifact_trinity_jack::editor::jack::snapshot_to_workflow(snapshot).map_err(|error|semio_framework_plugin::PluginAssemblyError::new("trinity.child.unavailable",error.into_message()))?;
    let viewport = cfg.camera.as_ref().map_or(scene_viewport, |camera| Viewport2d { x: camera.x, y: camera.y, zoom: camera.zoom });
    semio_framework_plugin::scene_surface(
        surface_id,
        SemanticSurfaceKind::NodeGraph,
        &semio_framework_plugin::NodeGraphScene { lod_json: Some(rewriting_lod_json_for_window(cfg)), editable: editable.then_some(true), ..semio_framework_plugin::NodeGraphScene::base(nodes, edges, viewport) },
    )
}
//#endregion 🔖️Render
