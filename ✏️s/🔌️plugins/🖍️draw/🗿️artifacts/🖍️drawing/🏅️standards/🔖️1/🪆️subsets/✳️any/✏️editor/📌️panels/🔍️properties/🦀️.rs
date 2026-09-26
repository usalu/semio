//! 🔍️ Selection-aware, localized layer inspection through undoable semantic commands.
use crate::editor::drawing::terminology::DrawingPlayLabels;
use crate::editor::drawing::{drawing_play_action, ui_value_list, ui_value_map, ui_value_text, ui_value_number};
use crate::schema::{selected_drawing_layers, layer_base, rgba_to_hex};
use crate::{DrawingLayerNode, DrawingSnapshot, FillStyle, PathSegment};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{BuiltNode, Label, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText};
use semio_framework_ui_contract as ui;

pub const DRAWING_PLAY_BODY_PROPERTIES: &str = "drawing.play.properties";
const ROOT: &str = "drawing-inspector";

pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native("Inspection", "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(DRAWING_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}

fn error() -> PluginAssemblyError {
    PluginAssemblyError::new("drawing.inspector.capacity", "Drawing inspector exceeds its UI capacity")
}

fn text(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(error)
}


struct Field {
    key: &'static str,
    label: LabelText,
    value: String,
    kind: InputKind,
    toggle: bool,
    min: Option<f64>,
    max: Option<f64>,
}

fn fields(layer: &DrawingLayerNode, labels: &DrawingPlayLabels) -> Vec<Field> {
    let base = layer_base(layer);
    let fill = match &base.attributes.fill { Some(FillStyle::Solid { color }) => rgba_to_hex(*color), _ => String::new() };
    let mut rows = Vec::new();
    for (key, label, value, kind, toggle, min, max) in [
        ("name", labels.name, base.name.clone(), InputKind::Text, false, None, None),
        ("visible", labels.visible, base.visible.to_string(), InputKind::Text, true, None, None),
        ("locked", labels.locked, base.locked.to_string(), InputKind::Text, true, None, None),
        ("opacity", labels.opacity, base.opacity.to_string(), InputKind::Number, false, Some(0.0), Some(1.0)),
        ("blendMode", labels.blend_mode, base.blend_mode.clone(), InputKind::Text, false, None, None),
        ("fillEnabled", labels.fill_enabled, base.attributes.fill.is_some().to_string(), InputKind::Text, true, None, None),
        ("strokeEnabled", labels.stroke_enabled, base.attributes.stroke.is_some().to_string(), InputKind::Text, true, None, None),
        ("strokeColor", labels.stroke_color, base.attributes.stroke.as_ref().map(|stroke| rgba_to_hex(stroke.color)).unwrap_or_else(|| "#000000".into()), InputKind::Color, false, None, None),
        ("fillColor", labels.fill, fill, InputKind::Color, false, None, None),
        ("strokeWidth", labels.stroke_width, base.attributes.stroke.as_ref().map_or(0.0, |stroke| stroke.width).to_string(), InputKind::Number, false, Some(0.0), None),
        ("strokeCap", labels.stroke_cap, base.attributes.stroke.as_ref().map_or("butt", |stroke| stroke.cap.as_str()).into(), InputKind::Text, false, None, None),
        ("strokeJoin", labels.stroke_join, base.attributes.stroke.as_ref().map_or("miter", |stroke| stroke.join.as_str()).into(), InputKind::Text, false, None, None),
        ("strokeDash", labels.stroke_dash, base.attributes.stroke.as_ref().and_then(|stroke| stroke.dash.as_ref()).map_or_else(String::new, |dash| dash.iter().map(f64::to_string).collect::<Vec<_>>().join(" ")), InputKind::Text, false, None, None),
        ("transformX", labels.position_x, base.transform.x.to_string(), InputKind::Number, false, None, None),
        ("transformY", labels.position_y, base.transform.y.to_string(), InputKind::Number, false, None, None),
        ("transformScaleX", labels.scale_x, base.transform.scale_x.to_string(), InputKind::Number, false, None, None),
        ("transformScaleY", labels.scale_y, base.transform.scale_y.to_string(), InputKind::Number, false, None, None),
        ("transformRotation", labels.rotation, base.transform.rotation.to_degrees().to_string(), InputKind::Number, false, None, None),
    ] {
        rows.push(Field { key, label, value, kind, toggle, min, max });
    }
    if let DrawingLayerNode::Trace(trace) = layer {
        rows.push(Field { key: "traceThreshold", label: labels.trace_threshold, value: trace.params.threshold.to_string(), kind: InputKind::Number, toggle: false, min: Some(0.0), max: Some(1.0) });
        rows.push(Field { key: "traceSimplify", label: labels.simplify, value: trace.params.simplify_epsilon.to_string(), kind: InputKind::Number, toggle: false, min: Some(0.0), max: None });
    }
    if let DrawingLayerNode::Boolean(boolean) = layer {
        rows.push(Field { key: "booleanOperation", label: labels.boolean_operation, value: boolean.operation.clone(), kind: InputKind::Text, toggle: false, min: None, max: None });
    }
    rows
}

fn field_row(document: &DrawingSnapshot, field: &Field, selected: &[&DrawingLayerNode], mixed: bool, labels: &DrawingPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([
        ("field", ui_value_text(if field.key == "transformRotation" { "rotationDegrees" } else { field.key })?),
        ("layerIds", ui_value_list(selected.iter().map(|layer| ui_value_text(&layer_base(layer).id)).collect::<UiAssemblyResult<Vec<_>>>()?)?),
    ])?;
    let (action, args) = drawing_play_action("patchLayers", Some(args))?;
    let args = args.ok_or_else(error)?;
    let id = format!("{ROOT}.{}.input", field.key);
    let disabled = selected.iter().any(|layer| crate::schema::drawing_layer_is_locked(document, &layer_base(layer).id)) && !matches!(field.key, "locked" | "visible");
    let control = if field.toggle {
        ui::toggle(field.value == "true").try_id(&id).map_err(|_| error())?.try_label(field.label.as_str()).map_err(|_| error())?.disabled(disabled)
            .try_on_with(Trigger::Change, action, args).map_err(|_| error())?.try_build().map_err(|_| error())?
    } else if matches!(field.key, "blendMode" | "booleanOperation" | "strokeCap" | "strokeJoin") {
        let choices: &[(&str, LabelText)] = if field.key == "blendMode" {
            &[("normal", labels.blend_normal), ("multiply", labels.blend_multiply), ("screen", labels.blend_screen), ("overlay", labels.blend_overlay), ("darken", labels.blend_darken), ("lighten", labels.blend_lighten)]
        } else if field.key == "strokeCap" {
            &[("butt", labels.cap_butt), ("round", labels.stroke_round), ("square", labels.cap_square)]
        } else if field.key == "strokeJoin" {
            &[("miter", labels.join_miter), ("round", labels.stroke_round), ("bevel", labels.join_bevel)]
        } else { &[("union", labels.boolean_union), ("intersect", labels.boolean_intersect), ("subtract", labels.boolean_subtract), ("exclude", labels.boolean_exclude)] };
        let mut input = ui::select(text(if mixed { "" } else { &field.value })?).try_id(&id).map_err(|_| error())?.try_label(field.label.as_str()).map_err(|_| error())?.disabled(disabled);
        for (value, label) in choices { input = input.try_item(text(value)?, ui::Label(text(label.as_str())?)).map_err(|_| error())?; }
        input.try_on_with(Trigger::Change, action, args).map_err(|_| error())?.try_build().map_err(|_| error())?
    } else {
        let mut input = ui::input(field.kind).value(text(if mixed { "" } else { &field.value })?).try_id(&id).map_err(|_| error())?.try_label(field.label.as_str()).map_err(|_| error())?.disabled(disabled).commit(text("blur")?);
        if mixed { input = input.placeholder(ui::Label(text(labels.mixed.as_str())?)); }
        if let Some(min) = field.min { input = input.min(min); }
        if let Some(max) = field.max { input = input.max(max); }
        if field.kind == InputKind::Number { input = input.step(if matches!(field.key, "opacity" | "traceThreshold") { 0.01 } else { 0.1 }); }
        input.try_on_with(Trigger::Change, action, args).map_err(|_| error())?.try_build().map_err(|_| error())?
    };
    ui::tree_item(ui::Label(text(field.label.as_str())?)).try_id(format!("{ROOT}.{}", field.key)).map_err(|_| error())?.try_child(control).map_err(|_| error())?.try_build().map_err(|_| error())
}

fn node_row(layer_id: &str, index: usize, segment: &PathSegment, join_target: Option<usize>, disabled: bool, labels: &DrawingPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let id = format!("{ROOT}.node.{index}");
    let mut row = ui::tree_item(ui::Label(text(&format!("{} {}", labels.anchor.as_str(), index + 1))?)).try_id(&id).map_err(|_| error())?;
    let mut points = Vec::new();
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Arc { to, .. } => points.push(("anchor", labels.anchor, *to)),
        PathSegment::Quad { to, ctrl } => points.extend([("anchor", labels.anchor, *to), ("control1", labels.control_one, *ctrl)]),
        PathSegment::Cubic { to, ctrl1, ctrl2 } => points.extend([("anchor", labels.anchor, *to), ("control1", labels.control_one, *ctrl1), ("control2", labels.control_two, *ctrl2)]),
        PathSegment::Close => {}
    }
    for (point, label, coordinates) in points {
        for (axis, value) in [("x", coordinates[0]), ("y", coordinates[1])] {
            let edit = ui_value_map([("axis", ui_value_text(axis)?), ("index", ui_value_number(index as f64)), ("kind", ui_value_text("coordinate")?), ("point", ui_value_text(point)?), ("value", ui_value_number(value))])?;
            let args = ui_value_map([("edit", edit), ("layerId", ui_value_text(layer_id)?)])?;
            let (action, args) = drawing_play_action("editPath", Some(args))?;
            let input = ui::input(InputKind::Number).value(text(&value.to_string())?).step(0.1).commit(text("blur")?).disabled(disabled)
                .try_id(format!("{id}.{point}.{axis}")).map_err(|_| error())?.try_label(format!("{} {}", label.as_str(), axis.to_uppercase())).map_err(|_| error())?
                .try_on_with(Trigger::Change, action, args.ok_or_else(error)?).map_err(|_| error())?.try_build().map_err(|_| error())?;
            row = row.try_child(input).map_err(|_| error())?;
        }
    }
    if !disabled {
        if let Some(other) = join_target {
            let edit = ui_value_map([("kind",ui_value_text("join")?),("index",ui_value_number(index as f64)),("other",ui_value_number(other as f64))])?;
            let args = ui_value_map([("edit",edit),("layerId",ui_value_text(layer_id)?)])?;
            let button = semio_framework_plugin::tree_item_with_action(format!("{id}.join"),ui::Label(text(labels.join_contour.as_str())?),None,drawing_play_action("editPath",Some(args))?)?;
            row = row.try_child(button).map_err(|_| error())?;
        }
        let mut actions = vec![("open", labels.open_contour), ("close", labels.close_contour)];
        if !matches!(segment, PathSegment::Close) { actions.push(("delete", labels.delete_node)); }
        if matches!(segment, PathSegment::Line { .. } | PathSegment::Quad { .. } | PathSegment::Cubic { .. } | PathSegment::Arc { .. }) { actions.push(("split", labels.split_segment)); }
        if index == 0 { actions.push(("reverse", labels.reverse_path)); }
        for (kind, label) in actions {
            let mut fields = vec![("kind", ui_value_text(kind)?)];
            if kind != "reverse" { fields.push(("index", ui_value_number(index as f64))); }
            if kind == "split" { fields.push(("t", ui_value_number(0.5))); }
            let args = ui_value_map([("edit", ui_value_map(fields)?), ("layerId", ui_value_text(layer_id)?)])?;
            let button = semio_framework_plugin::tree_item_with_action(format!("{id}.{kind}"), ui::Label(text(label.as_str())?), None, drawing_play_action("editPath", Some(args))?)?;
            row = row.try_child(button).map_err(|_| error())?;
        }
        if !matches!(segment, PathSegment::Move { .. } | PathSegment::Close) {
            for (target,label) in [("line",labels.straighten_segment),("cubic",labels.curve_segment)] {
                if matches!((segment,target),(PathSegment::Line { .. },"line") | (PathSegment::Cubic { .. },"cubic")) { continue; }
                let edit = ui_value_map([("kind",ui_value_text("convert")?),("index",ui_value_number(index as f64)),("target",ui_value_text(target)?)])?;
                let args = ui_value_map([("edit",edit),("layerId",ui_value_text(layer_id)?)])?;
                let button = semio_framework_plugin::tree_item_with_action(format!("{id}.convert.{target}"),ui::Label(text(label.as_str())?),None,drawing_play_action("editPath",Some(args))?)?;
                row = row.try_child(button).map_err(|_| error())?;
            }
        }
    }
    row.try_build().map_err(|_| error())
}

fn fill_input(layer_id: &str, id: &str, label: LabelText, kind: InputKind, value: &str, edit: semio_framework_plugin::UiValue, disabled: bool) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("layerId",ui_value_text(layer_id)?),("edit",edit)])?;
    let (action,args) = drawing_play_action("editFill",Some(args))?;
    let mut input = ui::input(kind).value(text(value)?).disabled(disabled).commit(text("blur")?)
        .try_id(id).map_err(|_| error())?.try_label(label.as_str()).map_err(|_| error())?;
    if kind == InputKind::Number { input = input.step(0.01); }
    input.try_on_with(Trigger::Change,action,args.ok_or_else(error)?).map_err(|_| error())?.try_build().map_err(|_| error())
}

fn fill_controls(layer_id: &str, fill: Option<&FillStyle>, disabled: bool, labels: &DrawingPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let kind = match fill { None => "none",Some(FillStyle::Solid { .. }) => "solid",Some(FillStyle::LinearGradient { .. }) => "linearGradient",Some(FillStyle::RadialGradient { .. }) => "radialGradient" };
    let args = ui_value_map([("layerId",ui_value_text(layer_id)?),("edit",ui_value_map([("kind",ui_value_text("type")?),("value",ui_value_text(kind)?)])?)])?;
    let (action,args) = drawing_play_action("editFill",Some(args))?;
    let mut select = ui::select(text(kind)?).disabled(disabled).try_id("drawing-inspector.fill.type").map_err(|_| error())?.try_label(labels.fill_type.as_str()).map_err(|_| error())?;
    for (value,label) in [("none",labels.fill_none),("solid",labels.fill_solid),("linearGradient",labels.fill_linear),("radialGradient",labels.fill_radial)] { select = select.try_item(text(value)?,ui::Label(text(label.as_str())?)).map_err(|_| error())?; }
    let select = select.try_on_with(Trigger::Change,action,args.ok_or_else(error)?).map_err(|_| error())?.try_build().map_err(|_| error())?;
    let mut row = ui::tree_item(ui::Label(text(labels.fill_type.as_str())?)).try_id("drawing-inspector.fill.controls").map_err(|_| error())?.try_child(select).map_err(|_| error())?;
    let coordinates = match fill {
        Some(FillStyle::LinearGradient { x1,y1,x2,y2,.. }) => vec![("x1",*x1,labels.gradient_start_x),("y1",*y1,labels.gradient_start_y),("x2",*x2,labels.gradient_end_x),("y2",*y2,labels.gradient_end_y)],
        Some(FillStyle::RadialGradient { cx,cy,r,.. }) => vec![("cx",*cx,labels.gradient_center_x),("cy",*cy,labels.gradient_center_y),("r",*r,labels.gradient_radius)],
        _ => Vec::new(),
    };
    for (axis,value,label) in coordinates {
        let edit = ui_value_map([("kind",ui_value_text("coordinate")?),("axis",ui_value_text(axis)?),("value",ui_value_number(value))])?;
        row = row.try_child(fill_input(layer_id,&format!("{ROOT}.fill.{axis}"),label,InputKind::Number,&value.to_string(),edit,disabled)?).map_err(|_| error())?;
    }
    if let Some(FillStyle::Solid { color }) = fill {
        let edit = ui_value_map([("kind",ui_value_text("alpha")?),("value",ui_value_number(color[3]))])?;
        row = row.try_child(fill_input(layer_id,"drawing-inspector.fill.alpha",labels.opacity,InputKind::Number,&color[3].to_string(),edit,disabled)?).map_err(|_| error())?;
    }
    if let Some(fill) = fill {
        let stops = crate::schema::fill::stops(fill);
        if !disabled && !stops.is_empty() && stops.len() < 64 {
            let offset = stops.windows(2).max_by(|a,b| (a[1].offset-a[0].offset).total_cmp(&(b[1].offset-b[0].offset))).map_or(0.5,|pair| (pair[0].offset+pair[1].offset)/2.0);
            let args = ui_value_map([("layerId",ui_value_text(layer_id)?),("edit",ui_value_map([("kind",ui_value_text("addStop")?),("offset",ui_value_number(offset))])?)])?;
            row = row.try_child(semio_framework_plugin::tree_item_with_action("drawing-inspector.fill.add",ui::Label(text(labels.add_stop.as_str())?),None,drawing_play_action("editFill",Some(args))?)?).map_err(|_| error())?;
        }
    }
    row.try_build().map_err(|_| error())
}

fn stop_row(layer_id: &str, index: usize, stop: &crate::GradientStop, removable: bool, disabled: bool, labels: &DrawingPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let id = format!("{ROOT}.fill.stop.{index}");
    let mut row = ui::tree_item(ui::Label(text(&format!("{} {}",labels.gradient_stop.as_str(),index+1))?)).try_id(&id).map_err(|_| error())?;
    for (kind,label,input,value) in [("color",labels.fill,InputKind::Color,rgba_to_hex(stop.color)),("alpha",labels.opacity,InputKind::Number,stop.color[3].to_string()),("offset",labels.stop_position,InputKind::Number,stop.offset.to_string())] {
        let value_arg = if input == InputKind::Color { ui_value_text(&value)? } else { ui_value_number(if kind == "alpha" { stop.color[3] } else { stop.offset }) };
        let edit = ui_value_map([("kind",ui_value_text(kind)?),("index",ui_value_number(index as f64)),("value",value_arg)])?;
        row = row.try_child(fill_input(layer_id,&format!("{id}.{kind}"),label,input,&value,edit,disabled)?).map_err(|_| error())?;
    }
    if removable && !disabled {
        let args = ui_value_map([("layerId",ui_value_text(layer_id)?),("edit",ui_value_map([("kind",ui_value_text("removeStop")?),("index",ui_value_number(index as f64))])?)])?;
        row = row.try_child(semio_framework_plugin::tree_item_with_action(format!("{id}.remove"),ui::Label(text(labels.remove_stop.as_str())?),None,drawing_play_action("editFill",Some(args))?)?).map_err(|_| error())?;
    }
    row.try_build().map_err(|_| error())
}

pub fn render(document: &DrawingSnapshot, ids: &[String], labels: &DrawingPlayLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let selected = selected_drawing_layers(document, ids);
    let Some(first) = selected.first() else { return semio_framework_plugin::built_text_node(Label::data(labels.select_hint.as_str())).map_err(|_| error()) };
    let all_fields = selected.iter().map(|layer| fields(layer, labels)).collect::<Vec<_>>();
    let rows = fields(first, labels).into_iter().filter(|field| all_fields.iter().all(|fields| fields.iter().any(|other| other.key == field.key))).collect::<Vec<_>>();
    let mut actions = vec![("group", labels.group), ("duplicate", labels.duplicate), ("delete", labels.delete), ("bringToFront", labels.bring_front), ("sendToBack", labels.send_back), ("alignLeft", labels.align_left), ("alignCenter", labels.align_center), ("alignRight", labels.align_right), ("alignTop", labels.align_top), ("alignMiddle", labels.align_middle), ("alignBottom", labels.align_bottom), ("distributeHorizontal", labels.distribute_horizontal), ("distributeVertical", labels.distribute_vertical)];
    if selected.iter().any(|layer| matches!(layer,DrawingLayerNode::Shape(_))) && selected.iter().all(|layer| matches!(layer,DrawingLayerNode::Shape(_) | DrawingLayerNode::Path(_)) && !crate::schema::drawing_layer_is_locked(document,&layer_base(layer).id)) { actions.insert(0,("toPath",labels.convert_to_path)); }
    let mut tree = PanelTreeBuilder::new(ROOT)?.window_section(windows, ROOT, Some(ui::Label(text(&format!("{} · {}", labels.layer.as_str(), selected.len()))?)), true, &rows, |field| {
        let mixed = all_fields.iter().any(|fields| fields.iter().any(|other| other.key == field.key && other.value != field.value));
        field_row(document, field, &selected, mixed, labels)
    })?.window_section(windows, "drawing-inspector.arrange", Some(ui::Label(text(labels.arrange.as_str())?)), true, &actions, |(operation, label)| {
        let args = ui_value_map([("operation", ui_value_text(operation)?)])?;
        semio_framework_plugin::tree_item_with_action(format!("{ROOT}.{operation}"), ui::Label(text(label.as_str())?), None, drawing_play_action("editSelection", Some(args))?)
    })?;
    if selected.len() == 1 {
        let base = layer_base(first);
        let disabled = crate::schema::drawing_layer_is_locked(document,&base.id);
        tree = tree.window_section(windows,"drawing-inspector.fill",Some(ui::Label(text(labels.fill.as_str())?)),true,&[()],|_| fill_controls(&base.id,base.attributes.fill.as_ref(),disabled,labels))?;
        if let Some(fill) = base.attributes.fill.as_ref() {
            let stops = crate::schema::fill::stops(fill);
            let rows = stops.iter().enumerate().collect::<Vec<_>>();
            if !rows.is_empty() { tree = tree.window_section(windows,"drawing-inspector.fill.stops",Some(ui::Label(text(labels.gradient_stops.as_str())?)),true,&rows,|(index,stop)| stop_row(&base.id,*index,stop,stops.len()>2,disabled,labels))?; }
        }
        if let DrawingLayerNode::Path(path) = first {
            let mut joins = vec![None;path.segments.len()];
            let mut closed = false;
            for (index,segment) in path.segments.iter().enumerate().rev() {
                if matches!(segment,PathSegment::Close) { closed=true; }
                if matches!(segment,PathSegment::Move { .. }) {
                    if index>0 && !closed && !matches!(path.segments[index-1],PathSegment::Close) { joins[index-1]=Some(index); }
                    closed=false;
                }
            }
            let nodes = path.segments.iter().enumerate().collect::<Vec<_>>();
            let disabled = crate::schema::drawing_layer_is_locked(document, &path.base.id);
            tree = tree.window_section(windows, "drawing-inspector.nodes", Some(ui::Label(text(labels.nodes.as_str())?)), true, &nodes, |(index, segment)| node_row(&path.base.id, *index, segment, joins[*index], disabled, labels))?;
        }
    }
    tree.build()
}

#[cfg(test)]
#[path = "🧪️tests/🎛️selection/🦀️.rs"]
mod tests;
