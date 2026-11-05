//! 🌈️ Native pixel operations dispatch the same semantic commands as every other host.
use super::*;
use semio_framework_plugin::UiValue;
use semio_s_artifact_raster_raster_editor_session::{ui_value_bool, ui_value_number};

fn action(key: &str, command: &str, label: LabelText, id: &str, enabled: bool, mut args: Vec<(&'static str, UiValue)>) -> UiAssemblyResult<BuiltNode> {
    args.push(("layerId", ui_value_text(id)?));
    let target = raster_action(command, Some(ui_value_map(args)?))?;
    ui::tree_item(ui_label(label.as_str())?).try_id(format!("{ROOT}.{key}")).map_err(|_| capacity())?.disabled(!enabled).target(activation_target(target)?).try_build().map_err(|_| capacity())
}

fn control(key: &str, label: LabelText, input: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    ui::tree_item(ui_label(label.as_str())?).try_id(format!("{ROOT}.{key}")).map_err(|_| capacity())?.try_child(input).map_err(|_| capacity())?.try_build().map_err(|_| capacity())
}

pub(super) fn render(document: &RasterDocument, config: &RasterConfig, layer: &RasterLayerNode, labels: &RasterPlayLabels) -> UiAssemblyResult<Vec<BuiltNode>> {
    let id = layer_node_id(layer);
    let pixel = matches!(layer, RasterLayerNode::Pixel { .. });
    let masked = matches!(layer, RasterLayerNode::Pixel { mask: Some(_), .. } | RasterLayerNode::Group { mask: Some(_), .. });
    let editable = layer_visible(layer) && layer_protection(&document.layers, id).is_some_and(|policy| policy.editable);
    let enabled = editable && config.paint_target == "pixels";
    let mut rows = Vec::new();
    if pixel || config.paint_target == "mask" && matches!(layer, RasterLayerNode::Group { .. }) {
        rows.push(action("fill-selection", "fillSelection", labels.fill_selection, id, editable && if config.paint_target == "mask" { masked } else { pixel }, vec![])?);
    }
    if !pixel { return Ok(rows); }
    let filters = [("invert", labels.invert_pixels), ("grayscale", labels.grayscale), ("clear", labels.clear_pixels), ("flipHorizontal", labels.flip_horizontal), ("flipVertical", labels.flip_vertical), ("brightness", labels.brightness), ("contrast", labels.contrast), ("saturation", labels.saturation), ("gamma", labels.gamma), ("threshold", labels.threshold), ("posterize", labels.posterize), ("blur", labels.blur), ("sharpen", labels.sharpen)];
    for (key, filter, label) in [("invert", "invert", labels.invert_pixels), ("grayscale", "grayscale", labels.grayscale), ("clear", "clear", labels.clear_pixels), ("flip-horizontal", "flipHorizontal", labels.flip_horizontal), ("flip-vertical", "flipVertical", labels.flip_vertical)] {
        rows.push(action(key, "applyFilter", label, id, enabled, vec![("filter", ui_value_text(filter)?), ("amount", ui_value_number(0.0))])?);
    }
    for (key, operation, label) in [("rotate-clockwise", "rotateClockwise", labels.rotate_clockwise), ("rotate-counterclockwise", "rotateCounterclockwise", labels.rotate_counterclockwise)] {
        rows.push(action(key, "transformImage", label, id, enabled, vec![("operation", ui_value_text(operation)?), ("x", ui_value_number(0.0)), ("y", ui_value_number(0.0)), ("width", ui_value_number(0.0)), ("height", ui_value_number(0.0)), ("bilinear", ui_value_bool(false))])?);
    }
    let (command, args) = raster_action("setFilterKind", Some(ui_value_map([])?))?;
    let mut picker = ui::select(text(&config.filter_kind)?).try_id(format!("{ROOT}.filter-kind.input")).map_err(|_| capacity())?.try_label(labels.filter.as_str()).map_err(|_| capacity())?;
    for (kind, label) in filters { picker = picker.try_item(text(kind)?, ui_label(label.as_str())?).map_err(|_| capacity())?; }
    rows.push(control("filter-kind", labels.filter, picker.try_on_with(Trigger::Change, command, args.ok_or_else(capacity)?).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?)?);
    if let Some(Some((min, max))) = crate::config::filter_range(&config.filter_kind) {
        let (command, args) = raster_action("setFilterAmount", Some(ui_value_map([])?))?;
        let step = if matches!(config.filter_kind.as_str(), "posterize" | "blur" | "threshold") { 1.0 } else { 0.01 };
        rows.push(control("filter-amount", labels.filter_amount, ui::input(InputKind::Number).value(text(&config.filter_amount.to_string())?).min(min).max(max).step(step).commit(text("blur")?).try_id(format!("{ROOT}.filter-amount.input")).map_err(|_| capacity())?.try_label(labels.filter_amount.as_str()).map_err(|_| capacity())?.try_on_with(Trigger::Commit, command, args.ok_or_else(capacity)?).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?)?);
    }
    rows.push(action("apply-filter", "applyFilter", labels.apply_filter, id, enabled && crate::config::valid_filter_options(&config.filter_kind, config.filter_amount), vec![("filter", ui_value_text(&config.filter_kind)?), ("amount", ui_value_number(config.filter_amount))])?);
    let grid = &config.image_grid;
    for (field, label, value, min) in [("width", labels.pixel_width, grid.width, 1.0), ("height", labels.pixel_height, grid.height, 1.0), ("x", labels.crop_x, grid.x, 0.0), ("y", labels.crop_y, grid.y, 0.0)] {
        let (command, args) = raster_action("setImageGrid", Some(ui_value_map([("field", ui_value_text(field)?)])?))?;
        let key = format!("image-grid-{field}");
        let input = ui::input(InputKind::Number).value(text(&value.to_string())?).min(min).max(16_384.0).step(1.0).commit(text("blur")?).try_id(format!("{ROOT}.{key}.input")).map_err(|_| capacity())?.try_label(label.as_str()).map_err(|_| capacity())?.try_on_with(Trigger::Commit, command, args.ok_or_else(capacity)?).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?;
        rows.push(control(&key, label, input)?);
    }
    let (command, args) = raster_action("setImageGrid", Some(ui_value_map([("field", ui_value_text("bilinear")?)])?))?;
    let input = ui::toggle(grid.bilinear).try_id(format!("{ROOT}.image-grid-bilinear.input")).map_err(|_| capacity())?.try_label(labels.bilinear.as_str()).map_err(|_| capacity())?.try_on_with(Trigger::Change, command, args.ok_or_else(capacity)?).map_err(|_| capacity())?.try_build().map_err(|_| capacity())?;
    rows.push(control("image-grid-bilinear", labels.bilinear, input)?);
    let extent_valid = semio_framework_pixels::editing::validate_extent(grid.width, grid.height).is_ok();
    let (source_width, source_height) = match layer {
        RasterLayerNode::Pixel { image_key, width, height, .. } => {
            let image = image_key.as_ref().and_then(|key| document.assets.get(key)).and_then(|asset| asset.local_owner::<semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot>());
            (image.map(|image| image.width).or(*width).unwrap_or(512), image.map(|image| image.height).or(*height).unwrap_or(512))
        }
        _ => unreachable!(),
    };
    for (key, operation, label) in [("resize-pixels", "resize", labels.resize_pixels), ("crop-pixels", "crop", labels.crop_pixels)] {
        let crop = operation == "crop";
        let fits = !crop || u64::from(grid.x) + u64::from(grid.width) <= u64::from(source_width) && u64::from(grid.y) + u64::from(grid.height) <= u64::from(source_height);
        rows.push(action(key, "transformImage", label, id, enabled && extent_valid && fits, vec![("operation", ui_value_text(operation)?), ("x", ui_value_number(if crop { grid.x } else { 0 })), ("y", ui_value_number(if crop { grid.y } else { 0 })), ("width", ui_value_number(grid.width)), ("height", ui_value_number(grid.height)), ("bilinear", ui_value_bool(!crop && grid.bilinear))])?);
    }
    Ok(rows)
}
