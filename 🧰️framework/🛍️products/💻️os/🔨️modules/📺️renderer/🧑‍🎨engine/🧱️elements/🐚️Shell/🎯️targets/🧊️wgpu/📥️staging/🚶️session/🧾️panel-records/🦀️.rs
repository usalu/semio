//! 🚶️ `shell-session` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 🧾️ Projects one SHELL-OWNED `UiNode` subtree into the `ui_contract` records a retained panel
/// document carries, pre-order so every parent precedes its children. W1h could only publish the
/// Commands panel by re-expressing it as `WindowMeasure`s (a `Select`/`Toggle` vocabulary with no
/// text, no button and no nesting); this is the general assembler, so a shell-owned panel body is
/// authored once as a `UiNode` — the same dialect React's `📌️ChromePanels/🟦️.tsx` hands its
/// `buildSettings*Tree` output straight to the Interpreter in — and published as-is.
///
/// A node's `key` is its own authored id when it has one (so hits and bindings address the control by
/// the id React uses), else a positional `n<seq>` key inside the surface, which is stable because the
/// walk order is.
pub(crate) fn panel_ui_records(surface_id: &str, root: &UiNode) -> Result<Vec<ui_contract::UiNodeRecord>, String> {
    let mut projection = PanelProjection { surface_id, records: Vec::new(), key_seq: 0 };
    projection.node(root)?;
    projection.records.into_iter().collect::<Option<Vec<_>>>().ok_or_else(|| format!("panel '{surface_id}' projection left a reserved record unplaced"))
}

/// 🖥️ One Display leaf's body: a single `Tree` under a stack the panel can address — the shape
/// every other shell-owned leaf publishes, so the two Display leaves need no new paint path.
/// 🪟️ Whether a shell-owned body streams a windowed Tree section — such a body IS its scroll viewport (the window streams against
/// it, its spacers give it the whole extent), so it publishes as a scroll root (`panel_ui_scroll_records`).
pub(crate) fn ui_node_is_windowed(node: &UiNode) -> bool {
    match node {
        UiNode::Tree(tree) => tree.sections.iter().any(|section| section.window.is_some()),
        UiNode::Stack(stack) => stack.children.iter().any(ui_node_is_windowed),
        _ => false,
    }
}

/// 📝️ One staged command argument as a labelled row — [`staged_arg_row`] dispatching `stageCommandArg`, so the
/// value buffers in {@link ShellState::staged_command_args} and nothing fires until Execute.
pub(crate) fn staged_command_arg_row(command_key: &str, arg: &semio_framework::ActionArgDef, value: Option<&Value>, selection: &HashMap<String, semio_framework::DomainSelection>, terminology: Terminology, locale: Locale) -> UiTreeItemNode {
    let id = format!("command.{}.arg.{}", command_key.replace(':', "."), arg.id);
    let stage = ActionDescriptor { controller_id: "framework".into(), action: "stageCommandArg".into(), args: crate::action_args_json!({ "command": command_key.to_string(), "arg": arg.id.clone() }) };
    staged_arg_row(id.clone(), &id, arg, value, &stage, selection, terminology, locale)
}

/// 📝️ One staged argument's row — the wgpu twin of React's `renderStagedArgControl` (`🛠️ShellHelpers/🟦️.tsx`), shared
/// by the command palette's staged form and a window's Actions form. `stage` is `stageCommandArg`/`stageActionArg`
/// with the row's own coordinates; every editor sends it, so the value buffers and nothing fires until Execute.
///
/// 🎛️ One control per scalar kind — text, number, a stepper, a slider or dial, a toggle, a select. Every number control
/// carries the facets [`semio_framework::ActionArgDef::number_facets`] derives (the one mapping the time-travel editor
/// shares): look and axis, travel or key range, step, stored and shown unit, display factor, precision, detents painted
/// as ticks, and the schema's hard limits, whose localized refusal names the crossed bound and which make a slider's
/// travel soft — a staged value is never clamped to the travel. The multi-part kinds open child rows of one control
/// each, so every part is its own focus stop with its own accessible name: a segmented choice's pressed buttons
/// (`option`), a vector's labelled axes (`index`/`dims`, spliced into the staged tuple by [`staged_arg_value`]), and a
/// reference list's removable chips plus its "use current selection" button, which stages the live selection of the
/// argument's domain.
#[allow(clippy::too_many_arguments, reason = "one staged row: its two ids, the definition, the value, its stage verb and the live selection")]
pub(crate) fn staged_arg_row(row_id: String, control_id: &str, arg: &semio_framework::ActionArgDef, value: Option<&Value>, stage: &ActionDescriptor, selection: &HashMap<String, semio_framework::DomainSelection>, terminology: Terminology, locale: Locale) -> UiTreeItemNode {
    use ui_wgpu::wgpu::component::ui::UiControlNode as Control;
    let label = arg.label.resolve(terminology, locale).to_string();
    let description = arg.description.as_ref().map(|description| description.resolve(terminology, locale).to_string());
    let with = |extra: Value| ActionDescriptor { controller_id: stage.controller_id.clone(), action: stage.action.clone(), args: staged_arg_args(stage.args.as_ref(), extra) };
    let text = value.map(|value| value.as_str().map(ToOwned::to_owned).unwrap_or_else(|| value.to_string())).unwrap_or_default();
    let number = value.and_then(staged_number);
    let facets = arg.number_facets(locale);
    let child = |id: String, label: String, control: Control| UiTreeItemNode { id, label: Label::data(label), control: Some(control), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) };
    let field = |id: String, input_kind: &str, value: String, accessibility_label: Option<String>, facets: Option<&semio_framework::ActionArgNumberFacets>, on_change: ActionDescriptor| {
        Control::Input(UiInputNode {
            id,
            input_kind: input_kind.into(),
            value,
            placeholder: Some(Label::data(label.clone())),
            accessibility_label: accessibility_label.map(Label::data),
            commit: Some("blur".into()),
            min: facets.and_then(|facets| facets.min),
            max: facets.and_then(|facets| facets.max),
            step: facets.and_then(|facets| facets.step),
            accept: None,
            precision: facets.and_then(|facets| facets.precision),
            snaps: facets.map(|facets| facets.snaps.clone()).unwrap_or_default(),
            display_factor: facets.and_then(|facets| facets.display_factor),
            limits: facets.map(|facets| facets.limits.clone()),
            on_change,
            on_submit: None,
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence::default(),
            menu: None,
            ..Default::default()
        })
    };
    let (control, mut items) = match arg.control() {
        semio_framework::ActionArgControl::Select { options } => (
            Some(Control::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: control_id.to_string(),
                value: text,
                items: options.iter().map(|option| UiSelectItem { value: option.value.clone(), label: Label::data(option.label.resolve(terminology, locale)) }).collect(),
                placeholder: Some(Label::data(label.clone())),
                on_change: stage.clone(),
                menu: None,
            })),
            None,
        ),
        semio_framework::ActionArgControl::Segmented { options } => (
            None,
            Some(
                options
                    .iter()
                    .map(|option| {
                        let option_label = option.label.resolve(terminology, locale).to_string();
                        child(
                            format!("{row_id}.option.{}", option.value),
                            option_label.clone(),
                            Control::Toggle(UiToggleNode {
                                appearance: ui_contract::ToggleAppearance::Button,
                                id: format!("{control_id}.option.{}", option.value),
                                icon_id: IconName::Check,
                                text: Some(Label::data(option_label)),
                                on_change: with(serde_json::json!({ "option": option.value })),
                                presence: UiPresence { selected: text == option.value, ..UiPresence::default() },
                                menu: None,
                            }),
                        )
                    })
                    .collect(),
            ),
        ),
        semio_framework::ActionArgControl::Toggle => (
            Some(Control::Toggle(UiToggleNode {
                appearance: ui_contract::ToggleAppearance::Button,
                id: control_id.to_string(),
                icon_id: IconName::Check,
                text: Some(Label::data(label.clone())),
                on_change: stage.clone(),
                presence: UiPresence { selected: value.and_then(Value::as_bool).unwrap_or(text == "true"), ..UiPresence::default() },
                menu: None,
            })),
            None,
        ),
        semio_framework::ActionArgControl::Slider { .. } | semio_framework::ActionArgControl::Dial { .. } => {
            let facets = facets.unwrap_or_default();
            let min = facets.min.unwrap_or(0.0);
            (
                Some(Control::Slider(UiSliderNode {
                    id: control_id.to_string(),
                    value: number.unwrap_or(min),
                    min,
                    max: facets.max.unwrap_or(0.0),
                    step: facets.step.unwrap_or(1.0),
                    unit: facets.unit,
                    snaps: facets.snaps,
                    appearance: facets.appearance,
                    scale: facets.scale,
                    precision: facets.precision,
                    display_unit: facets.display_unit,
                    display_factor: facets.display_factor,
                    limits: Some(facets.limits),
                    on_change: stage.clone(),
                    ..Default::default()
                })),
                None,
            )
        }
        semio_framework::ActionArgControl::Stepper { .. } => {
            let facets = facets.unwrap_or_default();
            let unit = facets.shown_unit().map(ToOwned::to_owned);
            (
                Some(Control::NumberStepper(UiNumberStepperNode {
                    id: control_id.to_string(),
                    value: number.or(facets.min).unwrap_or(0.0),
                    step: facets.step.unwrap_or(1.0),
                    uniform: true,
                    min: facets.min,
                    max: facets.max,
                    precision: facets.precision,
                    snaps: facets.snaps,
                    unit,
                    display_factor: facets.display_factor,
                    limits: Some(facets.limits),
                    on_absolute: stage.clone(),
                    on_delta: ActionDescriptor { controller_id: String::new(), action: String::new(), args: None },
                    ..Default::default()
                })),
                None,
            )
        }
        semio_framework::ActionArgControl::Number { .. } => (Some(field(control_id.to_string(), "number", text, None, facets.as_ref(), stage.clone())), None),
        semio_framework::ActionArgControl::Vector { dims, .. } => {
            let dims = dims as usize;
            let facets = facets.unwrap_or_default();
            let mut tuple: Vec<f64> = value.and_then(Value::as_array).map(|components| components.iter().map(|component| staged_number(component).unwrap_or(0.0)).collect()).unwrap_or_default();
            tuple.resize(dims, 0.0);
            let axes = ["x", "y", "z", "w"];
            let items = (0..dims)
                .map(|index| {
                    let axis = axes.get(index).map_or_else(|| index.to_string(), |axis| (*axis).to_string());
                    let on_change = with(serde_json::json!({ "index": index, "dims": dims, "tuple": tuple }));
                    let shown = match (facets.display_factor, facets.precision) {
                        (None, Some(digits)) => ui_contract::format_ui_number_fixed(tuple[index], digits),
                        _ => ui_contract::format_ui_number(tuple[index]),
                    };
                    child(format!("{row_id}.{axis}"), axis.clone(), field(format!("{control_id}.{axis}"), "number", shown, Some(format!("{label} {axis}")), Some(&facets), on_change))
                })
                .collect();
            (None, Some(items))
        }
        semio_framework::ActionArgControl::Color { alpha } => {
            let mut rgba = [0.0, 0.0, 0.0, 1.0];
            for (slot, component) in rgba.iter_mut().zip(value.and_then(Value::as_array).into_iter().flatten()) {
                *slot = staged_number(component).unwrap_or(0.0).clamp(0.0, 1.0);
            }
            let color = serde_json::json!({ "color": { "alpha": alpha, "rgba": &rgba[..if alpha { 4 } else { 3 }] } });
            let hex = LocalizedLabel::native("Hex", "Hex").resolve(terminology, locale).to_string();
            let opacity = LocalizedLabel::native("Opacity", "Deckkraft").resolve(terminology, locale).to_string();
            let mut items = vec![
                child(format!("{row_id}.swatch"), label.clone(), field(format!("{control_id}.swatch"), "color", ui_contract::ui_color_hex(&rgba, false), Some(label.clone()), None, with(color.clone()))),
                child(format!("{row_id}.hex"), hex.clone(), field(format!("{control_id}.hex"), "text", ui_contract::ui_color_hex(&rgba, alpha), Some(hex), None, with(color.clone()))),
            ];
            if alpha {
                items.push(child(format!("{row_id}.alpha"), opacity, Control::Slider(UiSliderNode { id: format!("{control_id}.alpha"), value: rgba[3], min: 0.0, max: 1.0, step: 0.01, unit: None, snaps: Vec::new(), on_change: with(color), presence: UiPresence::default(), menu: None, ..Default::default() })));
            }
            (None, Some(items))
        }
        semio_framework::ActionArgControl::Reference { domain, granularity, many, id_type, .. } => {
            let text = |id: &Value| semio_framework::reference_id_text(&semio_framework_value::DslValue::from(id));
            let ids: Vec<String> = match value {
                Some(Value::Array(ids)) => ids.iter().filter_map(text).collect(),
                Some(id) => text(id).into_iter().collect(),
                None => Vec::new(),
            };
            let references = |ids: Vec<String>| {
                let mut values = ids.iter().filter_map(|id| id_type.id_value(id)).map(Value::from);
                if many { Value::Array(values.collect()) } else { values.next().unwrap_or(Value::Null) }
            };
            let use_label = LocalizedLabel::native("Use current selection", "Aktuelle Auswahl verwenden").resolve(terminology, locale).to_string();
            let remove = LocalizedLabel::native("Remove {item}", "{item} entfernen").resolve(terminology, locale).to_string();
            let empty = LocalizedLabel::native("Nothing selected", "Nichts ausgewählt").resolve(terminology, locale).to_string();
            let picked = reference_selection_ids(selection, domain.as_deref(), granularity.as_deref());
            let mut items: Vec<UiTreeItemNode> = ids
                .iter()
                .map(|id| {
                    let remaining = references(ids.iter().filter(|other| *other != id).cloned().collect());
                    child(
                        format!("{row_id}.reference.{id}"),
                        id.clone(),
                        Control::Button(UiButtonNode { id: Some(format!("{control_id}.remove.{id}")), icon_id: IconName::X, label: Label::data(remove.replace("{item}", id)), action: with(serde_json::json!({ "value": remaining })), style: None, presence: UiPresence::default(), menu: None }),
                    )
                })
                .collect();
            if ids.is_empty() {
                items.push(UiTreeItemNode { id: format!("{row_id}.empty"), label: Label::data(empty), dimmed: Some(true), ..UiTreeItemNode::base(String::new(), Label::data(String::new())) });
            }
            let use_selection = UiButtonNode {
                id: Some(format!("{control_id}.useSelection")),
                icon_id: IconName::Crosshair,
                label: Label::data(use_label.clone()),
                action: with(serde_json::json!({ "value": references(picked.clone()) })),
                style: None,
                presence: UiPresence { state: if picked.is_empty() { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
                menu: None,
            };
            items.push(child(format!("{row_id}.useSelection"), use_label, Control::Button(use_selection)));
            (None, Some(items))
        }
        _ => (Some(field(control_id.to_string(), "text", text, None, None, stage.clone())), None),
    };
    if arg.nullable {
        let clear_label = match locale {
            Locale::En => "Clear",
            Locale::De => "Leeren",
        };
        let cleared = matches!(value, Some(Value::Null));
        let clear = UiButtonNode {
            id: Some(format!("{control_id}.clear")),
            icon_id: IconName::X,
            label: Label::data(format!("{clear_label} {label}")),
            action: with(serde_json::json!({ "value": Value::Null })),
            style: None,
            presence: UiPresence { state: if cleared { ui_wgpu::wgpu::component::ui::UiState::Disabled } else { ui_wgpu::wgpu::component::ui::UiState::Normal }, ..UiPresence::default() },
            menu: None,
        };
        items.get_or_insert_with(Vec::new).push(child(format!("{row_id}.clear"), clear_label.to_string(), Control::Button(clear)));
    }
    UiTreeItemNode { id: row_id, label: Label::data(label), description, control, default_open: items.as_ref().map(|_| true), items, ..UiTreeItemNode::base(String::new(), Label::data(String::new())) }
}

/// 🧮️ A staged value read as a number — a number, or the text a number field committed.
pub(super) fn staged_number(value: &Value) -> Option<f64> {
    value.as_f64().or_else(|| value.as_str().and_then(|text| text.trim().parse::<f64>().ok())).filter(|number| number.is_finite())
}

/// 🧩️ A stage verb's own coordinates (`window`/`action`/`arg` or `command`/`arg`) with one editor part's extra keys.
fn staged_arg_args(base: Option<&DslValue>, extra: Value) -> Option<DslValue> {
    let mut args = base.map(dsl_value_as_json).and_then(|args| args.as_object().cloned()).unwrap_or_default();
    if let Value::Object(extra) = extra {
        args.extend(extra);
    }
    Some(semio_framework_value::DslValue::from(Value::Object(args)))
}

/// 🎯️ The ids "use current selection" stages for a reference argument: the live selection of its `domain` (or, when
/// it names none, of the one domain that selects anything), kept only at its `granularity` when it declares one.
pub(crate) fn reference_selection_ids(selection: &HashMap<String, semio_framework::DomainSelection>, domain: Option<&str>, granularity: Option<&str>) -> Vec<String> {
    let picked = match domain {
        Some(domain) => selection.get(domain),
        None => {
            let mut live = selection.values().filter(|selected| !selected.ids.is_empty());
            let first = live.next();
            if live.next().is_some() {
                None
            } else {
                first
            }
        }
    };
    picked.filter(|selected| granularity.is_none_or(|granularity| selected.granularity == granularity)).map(|selected| selected.ids.clone()).unwrap_or_default()
}

//#endregion 🧾️PanelRecordsAssembler
//#region 🛍️AppCatalogueAttempt
/// 🛍️ The "fetch the app-static catalogue once per app instance" rule, owned in ONE place because it
/// has to hold on BOTH outcomes.
///
/// `recorded` is the instance the catalogue was last ATTEMPTED for, not the one it last succeeded
/// for. `true` means the caller now owns that attempt and must perform the fetch; every later call
/// for the same instance answers `false` whether the fetch succeeded, faulted or never published.
/// Recording only successes made a failed catalogue re-fetch once per settled command, and the
/// second fetch of `framework.section.catalogue` never returned from its own turn — which wedged
/// `boot_shell` before the canvas ever bound a pointer listener
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md`).
///
/// A new instance always re-claims, so switching app instance still refetches exactly once.
pub(crate) fn claim_app_catalogue_fetch(recorded: &mut Option<u32>, instance_id: u32) -> bool {
    if *recorded == Some(instance_id) {
        return false;
    }
    *recorded = Some(instance_id);
    true
}

//#endregion 🛍️AppCatalogueAttempt
//#endregion ShellLifecycle

