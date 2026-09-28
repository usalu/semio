//! 🔍️ Localized selection properties and foreground controls through semantic commands.
use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::{raster_action, ui_label, ui_value_list, ui_value_map, ui_value_text};
use crate::standards::v1::subsets::any::schema::{find_layer, layer_name, layer_node_id, layer_opacity, layer_blend_mode, layer_transform, layer_visible, layer_locked, layer_protection};
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

fn value(layer: &RasterLayerNode, field: &str, document: &RasterDocument) -> String {
    match field {
        "name" => layer_name(layer).into(),
        "brightness" | "contrast" => if let RasterLayerNode::Adjustment {params,..}=layer {params.get(field).and_then(dsl::DslValue::as_f64).unwrap_or(0.0).to_string()} else {String::new()},
        "visible" => layer_visible(layer).to_string(),
        "locked" => layer_locked(layer).to_string(),
        "opacity" => layer_opacity(layer).to_string(),
        "blendMode" => layer_blend_mode(layer).into(),
        "transformScaleX"|"transformScaleY"|"transformRotation"|"transformShearX"=>{
            let Ok(controls)=semio_framework_pixels::compositing::frames::decompose(layer_transform(layer).as_affine()) else {return String::new();};
            match field {"transformScaleX"=>controls.scale_x,"transformScaleY"=>controls.scale_y,"transformRotation"=>controls.rotation,_=>controls.shear_x}.to_string()
        }
        "transformX" => layer_transform(layer).x.to_string(),
        "transformY" => layer_transform(layer).y.to_string(),
        "width" => if let RasterLayerNode::Pixel { width, .. } = layer { width.unwrap_or(512).to_string() } else { String::new() },
        "maskPresent" | "maskEnabled" | "maskInvert" | "maskLinked" => {
            let (RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) = layer else { return String::new(); };
            match field {
                "maskPresent" => mask.is_some(),
                "maskEnabled" => mask.as_ref().is_some_and(|mask| mask.enabled),
                "maskLinked" => mask.as_ref().is_some_and(|mask| mask.linked),
                _ => mask.as_ref().is_some_and(|mask| mask.invert),
            }.to_string()
        }
        "maskX" | "maskY" | "maskScaleX" | "maskScaleY" | "maskRotation" | "maskShearX" | "maskWidth" | "maskHeight" => {
            let (RasterLayerNode::Pixel { mask: Some(mask), .. } | RasterLayerNode::Group { mask: Some(mask), .. }) = layer else { return String::new(); };
            let image = mask.image_key.as_ref().and_then(|key| document.assets.get(key)).and_then(|asset| asset.local_owner::<crate::SemioImageSnapshot>());
            match field {
                "maskX" => mask.transform.x.to_string(),
                "maskY" => mask.transform.y.to_string(),
                "maskScaleX" | "maskScaleY" | "maskRotation" | "maskShearX" => {
                    let Ok(controls)=semio_framework_pixels::compositing::frames::decompose(mask.transform.as_affine()) else {return String::new();};
                    match field {"maskScaleX"=>controls.scale_x,"maskScaleY"=>controls.scale_y,"maskRotation"=>controls.rotation,_=>controls.shear_x}.to_string()
                },
                "maskWidth" => mask.width.or_else(|| image.map(|image| image.width)).unwrap_or(512).to_string(),
                _ => mask.height.or_else(|| image.map(|image| image.height)).unwrap_or(512).to_string(),
            }
        }
        "height" => if let RasterLayerNode::Pixel { height, .. } = layer { height.unwrap_or(512).to_string() } else { String::new() },
        _ => String::new(),
    }
}

fn field_row(field: &str, label: LabelText, selected: &[&RasterLayerNode], labels: &RasterPlayLabels, document: &RasterDocument) -> UiAssemblyResult<BuiltNode> {
    let initial = value(selected[0], field, document);
    let mixed = selected.iter().skip(1).any(|layer| value(layer, field, document) != initial);
    let args = ui_value_map([("field", ui_value_text(field)?), ("layerIds", ui_value_list(selected.iter().map(|layer| ui_value_text(layer_node_id(layer))).collect::<UiAssemblyResult<Vec<_>>>()?)?)])?;
    let (action, args) = raster_action("patchLayers", Some(args))?;
    let args = args.ok_or_else(capacity)?;
    let id = format!("{ROOT}.{field}.input");
    let mut control = if matches!(field, "locked" | "visible" | "maskPresent" | "maskEnabled" | "maskInvert" | "maskLinked") {
        ui::toggle(initial == "true").try_id(&id).map_err(|_| capacity())?.try_label(label.as_str()).map_err(|_| capacity())?.try_on_with(Trigger::Change, action, args).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?
    } else if field == "blendMode" {
        let mut input = ui::select(text(if mixed { "" } else { &initial })?).try_id(&id).map_err(|_| capacity())?.try_label(label.as_str()).map_err(|_| capacity())?;
        for (key,name) in [("normal",labels.blend_normal),("multiply",labels.blend_multiply),("screen",labels.blend_screen),("overlay",labels.blend_overlay),("darken",labels.blend_darken),("lighten",labels.blend_lighten),("colorDodge",labels.blend_color_dodge),("colorBurn",labels.blend_color_burn),("hardLight",labels.blend_hard_light),("softLight",labels.blend_soft_light),("difference",labels.blend_difference),("exclusion",labels.blend_exclusion),("hue",labels.blend_hue),("saturation",labels.blend_saturation),("color",labels.blend_color),("luminosity",labels.blend_luminosity)] {
            input = input.try_item(text(key)?, ui_label(name.as_str())?).map_err(|_| capacity())?;
        }
        input.try_on_with(Trigger::Change, action, args).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?
    } else {
        let kind = if field == "name" { InputKind::Text } else { InputKind::Number };
        let mut input = ui::input(kind).value(text(if mixed { "" } else { &initial })?).try_id(&id).map_err(|_| capacity())?.try_label(label.as_str()).map_err(|_| capacity())?.commit(text("blur")?);
        if mixed { input = input.placeholder(ui_label(labels.mixed.as_str())?); }
        if matches!(field,"brightness"|"contrast") {input=input.min(-1.0).max(1.0).step(0.01);}
        if field == "opacity" { input = input.min(0.0).max(1.0).step(0.01); }
        if matches!(field, "width" | "height" | "maskWidth" | "maskHeight") { input = input.min(1.0).max(16384.0).step(1.0); }
        if matches!(field, "maskScaleX" | "maskScaleY" | "maskShearX" | "transformScaleX" | "transformScaleY" | "transformShearX") { input = input.step(0.01); }
        input.try_on_with(Trigger::Commit, action, args).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?
    };
    control.disabled=selected.iter().any(|layer|layer_protection(&document.layers,layer_node_id(layer)).is_none_or(|policy|match field {
        "visible"=>false,
        "locked"=>!policy.can_change_lock,
        "transformX"|"transformY"|"transformScaleX"|"transformScaleY"|"transformRotation"|"transformShearX"|"width"|"height"=>!policy.structural,
        _=>!policy.editable,
    }));
    let disabled=control.disabled;
    let mut row=ui::tree_item(ui_label(label.as_str())?).try_id(format!("{ROOT}.{field}")).map_err(|_| capacity())?.try_child(control).map_err(|_| capacity())?;
    if disabled {row=row.description(text(labels.locked_hint.as_str())?);}
    row.try_build().map_err(|_|capacity())
}

pub fn render(document: &RasterDocument, runtime: &RasterConfig, selected_ids: &[String], labels: &RasterPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let selected: Vec<_> = selected_ids.iter().filter_map(|id| find_layer(&document.layers, id)).collect();
    let mut rows = Vec::new();
    if !selected.is_empty() {
        for (field, label) in [("name", labels.name), ("visible", labels.visible), ("locked", labels.locked), ("opacity", labels.opacity), ("blendMode", labels.blend_mode)] {
            rows.push(field_row(field, label, &selected, labels, document)?);
        }
        if selected.iter().all(|layer| matches!(layer, RasterLayerNode::Pixel { .. } | RasterLayerNode::Group { .. })) {
            rows.push(field_row("transformX",labels.position_x,&selected,labels,document)?);
            rows.push(field_row("transformY",labels.position_y,&selected,labels,document)?);
            for (field,label) in [("transformScaleX",labels.scale_x),("transformScaleY",labels.scale_y),("transformRotation",labels.rotation),("transformShearX",labels.shear_x)] {rows.push(field_row(field,label,&selected,labels,document)?);}
            rows.push(field_row("maskPresent", labels.mask_present, &selected, labels, document)?);
            if selected.iter().all(|layer| value(layer, "maskPresent", document) == "true") {
                rows.push(field_row("maskEnabled", labels.mask_enabled, &selected, labels, document)?);
                rows.push(field_row("maskInvert", labels.mask_invert, &selected, labels, document)?);
                rows.push(field_row("maskLinked", labels.mask_linked, &selected, labels, document)?);
                for (field,label) in [("maskX",labels.mask_x),("maskY",labels.mask_y),("maskScaleX",labels.mask_scale_x),("maskScaleY",labels.mask_scale_y),("maskRotation",labels.mask_rotation),("maskShearX",labels.mask_shear_x),("maskWidth",labels.mask_width),("maskHeight",labels.mask_height)] {
                    rows.push(field_row(field,label,&selected,labels,document)?);
                }
            }
        }
        if selected.iter().all(|layer|matches!(layer,RasterLayerNode::Adjustment {adjustment_kind,..} if adjustment_kind=="brightnessContrast")) {
            rows.push(field_row("brightness",labels.brightness,&selected,labels,document)?);
            rows.push(field_row("contrast",labels.contrast,&selected,labels,document)?);
        }
        if selected.iter().all(|layer| matches!(layer, RasterLayerNode::Pixel { .. })) {
            rows.push(field_row("width", labels.width, &selected, labels, document)?);
            rows.push(field_row("height", labels.height, &selected, labels, document)?);
        }
    }
    if selected.len()==1 {
        let id=layer_node_id(selected[0]);
        let duplicate=crate::editor::raster::commands::duplicate_layer::plan(document,id).is_ok();
        let delete=layer_protection(&document.layers,id).is_some_and(|policy|policy.structural);
        for (key,command,label,enabled) in [("duplicate","duplicateLayer",labels.duplicate_layer,duplicate),("delete","deleteLayer",labels.delete_layer,delete)] {
            let (action,args)=raster_action(command,Some(ui_value_map([("layerId",ui_value_text(id)?)])?))?;
            let mut row=ui::tree_item(ui_label(label.as_str())?).try_id(format!("{ROOT}.{key}")).map_err(|_|capacity())?.disabled(!enabled);
            if !enabled {row=row.description(text(labels.locked_hint.as_str())?);}
            rows.push(row.try_on_with(Trigger::Activate,action,args.ok_or_else(capacity)?).map_err(|_|capacity())?.try_build().map_err(|_|capacity())?);
        }
        let supported=crate::editor::raster::commands::merge_down::plan(document,id).is_ok();
        let (action,args)=raster_action("mergeDown",Some(ui_value_map([("layerId",ui_value_text(id)?)])?))?;
        rows.push(ui::tree_item(ui_label(labels.merge_down.as_str())?).try_id(format!("{ROOT}.merge-down")).map_err(|_|capacity())?.description(text(labels.merge_down_hint.as_str())?).disabled(!supported)
            .try_on_with(Trigger::Activate,action,args.ok_or_else(capacity)?).map_err(|_|capacity())?.try_build().map_err(|_|capacity())?);
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
