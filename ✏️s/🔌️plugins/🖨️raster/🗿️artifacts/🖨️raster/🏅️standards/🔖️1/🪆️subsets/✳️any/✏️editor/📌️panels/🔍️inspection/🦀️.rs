//! 🔍️ Localized selection properties and foreground controls through semantic commands.
use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::{raster_action, ui_label, ui_value_list, ui_value_map, ui_value_text};
use crate::standards::v1::subsets::any::schema::{find_layer, layer_name, layer_node_id, layer_opacity, layer_blend_mode, layer_transform, layer_visible};
use crate::{RasterLayerNode, RasterSnapshot as RasterDocument};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{BuiltNode, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_INSPECTION_ID};
use semio_framework_ui_contract as ui;

pub const RASTER_PLAY_BODY_PROPERTIES: &str = "raster.play.properties";
const ROOT: &str = "raster-inspector";

pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label: LocalizedLabel::native("Inspection", "Inspektion"), group: PanelGroup::Details, body_key: Some(RASTER_PLAY_BODY_PROPERTIES.into()), children: Vec::new() }
}

fn capacity() -> PluginAssemblyError { PluginAssemblyError::new("raster.inspector.capacity", "Raster inspection exceeds its UI capacity") }
fn text(value: &str) -> UiAssemblyResult<UiText> { UiText::try_from_str(value).ok_or_else(capacity) }

fn value(layer: &RasterLayerNode, field: &str) -> String {
    match field {
        "name" => layer_name(layer).into(),
        "visible" => layer_visible(layer).to_string(),
        "opacity" => layer_opacity(layer).to_string(),
        "blendMode" => layer_blend_mode(layer).into(),
        "transformX" => layer_transform(layer).x.to_string(),
        "transformY" => layer_transform(layer).y.to_string(),
        "width" => if let RasterLayerNode::Pixel { width, .. } = layer { width.unwrap_or(512).to_string() } else { String::new() },
        "height" => if let RasterLayerNode::Pixel { height, .. } = layer { height.unwrap_or(512).to_string() } else { String::new() },
        _ => String::new(),
    }
}

fn field_row(field: &str, label: LabelText, selected: &[&RasterLayerNode], labels: &RasterPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let initial = value(selected[0], field);
    let mixed = selected.iter().skip(1).any(|layer| value(layer, field) != initial);
    let args = ui_value_map([("field", ui_value_text(field)?), ("layerIds", ui_value_list(selected.iter().map(|layer| ui_value_text(layer_node_id(layer))).collect::<UiAssemblyResult<Vec<_>>>()?)?)])?;
    let (action, args) = raster_action("patchLayers", Some(args))?;
    let args = args.ok_or_else(capacity)?;
    let id = format!("{ROOT}.{field}.input");
    let control = if field == "visible" {
        ui::toggle(initial == "true").try_id(&id).map_err(|_| capacity())?.try_label(label.as_str()).map_err(|_| capacity())?.try_on_with(Trigger::Change, action, args).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?
    } else if field == "blendMode" {
        let mut input = ui::select(text(if mixed { "" } else { &initial })?).try_id(&id).map_err(|_| capacity())?.try_label(label.as_str()).map_err(|_| capacity())?;
        for (key, name) in [("normal", labels.blend_normal), ("multiply", labels.blend_multiply), ("screen", labels.blend_screen), ("darken", labels.blend_darken), ("lighten", labels.blend_lighten), ("difference", labels.blend_difference)] {
            input = input.try_item(text(key)?, ui_label(name.as_str())?).map_err(|_| capacity())?;
        }
        input.try_on_with(Trigger::Change, action, args).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?
    } else {
        let kind = if field == "name" { InputKind::Text } else { InputKind::Number };
        let mut input = ui::input(kind).value(text(if mixed { "" } else { &initial })?).try_id(&id).map_err(|_| capacity())?.try_label(label.as_str()).map_err(|_| capacity())?.commit(text("blur")?);
        if mixed { input = input.placeholder(ui_label(labels.mixed.as_str())?); }
        if field == "opacity" { input = input.min(0.0).max(1.0).step(0.01); }
        if matches!(field, "width" | "height") { input = input.min(1.0).max(16384.0).step(1.0); }
        input.try_on_with(Trigger::Change, action, args).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?
    };
    ui::tree_item(ui_label(label.as_str())?).try_id(format!("{ROOT}.{field}")).map_err(|_| capacity())?.try_child(control).map_err(|_| capacity())?.try_build().map_err(|_| capacity())
}

pub fn render(document: &RasterDocument, runtime: &RasterConfig, selected_ids: &[String], labels: &RasterPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let selected: Vec<_> = selected_ids.iter().filter_map(|id| find_layer(&document.layers, id)).collect();
    let mut rows = Vec::new();
    if !selected.is_empty() {
        for (field, label) in [("name", labels.name), ("visible", labels.visible), ("opacity", labels.opacity), ("blendMode", labels.blend_mode), ("transformX", labels.position_x), ("transformY", labels.position_y)] {
            rows.push(field_row(field, label, &selected, labels)?);
        }
        if selected.iter().all(|layer| matches!(layer, RasterLayerNode::Pixel { .. })) {
            rows.push(field_row("width", labels.width, &selected, labels)?);
            rows.push(field_row("height", labels.height, &selected, labels)?);
        }
    }
    let (action, args) = raster_action("setBrushColor", Some(ui_value_map([])?))?;
    let input = ui::input(InputKind::Color).value(text(&runtime.brush_color)?).try_id(format!("{ROOT}.foreground.input")).map_err(|_| capacity())?.try_label(labels.foreground.as_str()).map_err(|_| capacity())?
        .try_on_with(Trigger::Change, action, args.ok_or_else(capacity)?).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?;
    rows.push(ui::tree_item(ui_label(labels.foreground.as_str())?).try_id(format!("{ROOT}.foreground")).map_err(|_| capacity())?.try_child(input).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?);
    PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.properties"), Some(ui_label(labels.inspection.as_str())?), true, semio_framework_plugin::ui_node_list(rows.into_iter().map(Ok))?)?.build()
}

#[cfg(test)]
#[path = "🧪️tests/🎛️selection/🦀️.rs"]
mod tests;
