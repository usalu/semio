use super::*;
use crate::ENERGY_MODEL_DOCUMENT_SCHEMA;

fn snapshot_of(model: &Model) -> EnergyModelSnapshot {
    crate::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None)
}

/// 🧫️ ANSI/ASHRAE 140 case 600 — zone 1, surfaces 40..45, windows 50/51, materials 10..21, glazing
/// 22, gas gap 23, constructions 30..33, thermostat 62.
fn demo() -> EnergyModelSnapshot {
    snapshot_of(&crate::examples::demo::model())
}

fn selecting(ids: &[&str]) -> EnergyModelInteractionSnapshot {
    EnergyModelInteractionSnapshot { selected_ids: ids.iter().map(|id| (*id).to_string()).collect(), ..Default::default() }
}

fn panel(snapshot: &EnergyModelSnapshot, ids: &[&str], locale: Locale) -> String {
    panel_with(snapshot, ids, &EnergyModelConfig::default(), locale)
}

fn panel_with(snapshot: &EnergyModelSnapshot, ids: &[&str], config: &EnergyModelConfig, locale: Locale) -> String {
    let built = render(snapshot, &selecting(ids), config, locale).expect("energy inspector assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: built }).expect("energy inspector projection")
}

fn english(ids: &[&str]) -> String {
    panel(&demo(), ids, Locale::En)
}

/// 🔎️ The projected node carrying `key`, anywhere under the body.
fn node_at<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| node_at(child, key))
}

fn component_at(json: &str, key: &str) -> String {
    let tree: serde_json::Value = serde_json::from_str(json).expect("the inspector projection is JSON");
    let node = node_at(&tree, key).unwrap_or_else(|| panic!("no node keyed {key} in {json}"));
    node["component"]["type"].as_str().expect("every projected component names its type").to_owned()
}

/// 🌲️ The layout trap: the React `Interpreter` maps a `Component::Tree` onto `TreeDataItem`s and
/// DROPS every non-tree-item child, so a form control nested in one reaches the host and is never
/// drawn. This panel renders none — ever.
fn carries_a_tree(json: &str) -> bool {
    json.contains("\"tree\"") || json.contains("\"treeSection\"") || json.contains("\"treeItem\"")
}

#[semio_framework_async_macros::async_test]
async fn the_panel_tab_declares_the_framework_inspection_slot_and_this_body_key() {
    let tab = definition();
    assert_eq!(tab.body_key.as_deref(), Some(BODY_KEY));
    assert_eq!(tab.group, PanelGroup::Details);
}

/// 🟫️ A selected surface offers every editable field of the record plus its derived geometry, and
/// never a tree node.
#[semio_framework_async_macros::async_test]
async fn a_selected_surface_renders_its_whole_editable_record() {
    let json = english(&["40"]);
    assert_eq!(component_at(&json, "energy-model-inspection.surface.name.input"), "input", "{json}");
    assert_eq!(component_at(&json, "energy-model-inspection.surface.class.select"), "select");
    assert_eq!(component_at(&json, "energy-model-inspection.surface.boundary.select"), "select");
    assert_eq!(component_at(&json, "energy-model-inspection.surface.construction.select"), "select");
    assert_eq!(component_at(&json, "energy-model-inspection.surface.sun-exposed.toggle"), "toggle");
    assert_eq!(component_at(&json, "energy-model-inspection.surface.wind-exposed.toggle"), "toggle");
    assert_eq!(component_at(&json, "energy-model-inspection.surface.multiplier.input"), "input");
    assert_eq!(component_at(&json, "energy-model-inspection.surface.area.value"), "text", "a derived quantity is reported, never offered as a control");
    assert!(json.contains(SET_SURFACE_PROPERTY_ACTION_ID), "every control dispatches the surface property verb");
    assert!(json.contains("exteriorWall"), "the class select offers this editor's own class vocabulary");
    assert!(!carries_a_tree(&json), "{json}");
}

/// 🪟️ A selected window offers all fifteen scalars plus its glazing construction — the fields the
/// `diff_fenestrations` step this ticket added makes reachable at all.
#[semio_framework_async_macros::async_test]
async fn a_selected_window_renders_every_fenestration_field() {
    let json = english(&["50"]);
    for row in ["name", "u-value", "area", "height", "sill-height", "frame", "divider", "overhang-depth", "overhang-offset", "fin-depth", "fin-offset"] {
        assert!(json.contains(&format!("energy-model-inspection.fenestration.{row}.input")), "{row} missing from {json}");
    }
    assert_eq!(component_at(&json, "energy-model-inspection.fenestration.shgc.slider"), "slider");
    assert_eq!(component_at(&json, "energy-model-inspection.fenestration.vlt.slider"), "slider");
    assert_eq!(component_at(&json, "energy-model-inspection.fenestration.glazing.select"), "select");
    assert!(json.contains(SET_FENESTRATION_PROPERTY_ACTION_ID));
    assert!(json.contains("\"uValueWM2K\""), "the binding names the field it patches");
    assert!(!carries_a_tree(&json), "{json}");
}

/// 🧾️ The law the tree layout broke: a field row must reach the host as the CONTROL component the
/// renderer knows how to draw, inside its labelled field row, bound to its own command.
#[semio_framework_async_macros::async_test]
async fn a_window_u_value_row_is_a_bound_number_input_inside_a_field_row() {
    let json = english(&["50"]);
    let tree: serde_json::Value = serde_json::from_str(&json).expect("the inspector projection is JSON");
    let control = node_at(&tree, "energy-model-inspection.fenestration.u-value.input").unwrap_or_else(|| panic!("{json}"));
    assert_eq!(control["component"]["type"].as_str(), Some("input"), "{control}");
    assert_eq!(control["component"]["kind"].as_str(), Some("number"), "{control}");
    let bindings = control["bindings"].to_string();
    assert!(bindings.contains(SET_FENESTRATION_PROPERTY_ACTION_ID), "{bindings}");
    assert!(bindings.contains("uValueWM2K"), "the binding names the field it patches: {bindings}");
    assert!(bindings.contains("\"50\""), "the binding addresses the selected window: {bindings}");
    assert!(!bindings.contains("\"value\""), "the control's own value is merged by the host, never authored here: {bindings}");
    let row = node_at(&tree, "energy-model-inspection.fenestration.u-value").unwrap_or_else(|| panic!("{json}"));
    assert_eq!(row["component"]["type"].as_str(), Some("container"), "the input sits inside its labelled field row: {row}");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_zone_renders_its_five_fields_and_a_delete_verb() {
    let json = english(&["1"]);
    assert_eq!(component_at(&json, "energy-model-inspection.zone.name.input"), "input", "{json}");
    assert_eq!(component_at(&json, "energy-model-inspection.zone.volume.input"), "input");
    assert_eq!(component_at(&json, "energy-model-inspection.zone.multiplier.input"), "input");
    assert_eq!(component_at(&json, "energy-model-inspection.zone.conditioned.toggle"), "toggle");
    assert_eq!(component_at(&json, "energy-model-inspection.zone.floor-area.toggle"), "toggle");
    assert!(json.contains(SET_ZONE_PROPERTY_ACTION_ID));
    assert_eq!(component_at(&json, "energy-model-inspection.actions.delete"), "button", "a zone can be deleted from the inspector");
    assert!(json.contains(crate::editor::model::DELETE_ZONE_ACTION_ID));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_material_mixes_inputs_and_bounded_sliders() {
    let json = english(&["10"]);
    assert_eq!(component_at(&json, "energy-model-inspection.material.conductivity.input"), "input", "{json}");
    assert_eq!(component_at(&json, "energy-model-inspection.material.thermal-absorptance.slider"), "slider");
    assert_eq!(component_at(&json, "energy-model-inspection.material.solar-absorptance.slider"), "slider");
    assert_eq!(component_at(&json, "energy-model-inspection.material.visible-absorptance.slider"), "slider");
    assert!(json.contains(SET_MATERIAL_PROPERTY_ACTION_ID));
    assert_eq!(component_at(&json, "energy-model-inspection.material.name.input"), "input", "`set-material-property` carries text now, so a material can be renamed");
    assert_eq!(component_at(&json, "energy-model-inspection.material.roughness.select"), "select", "`change-material-roughness` landed, so roughness is a real control");
    assert!(json.contains("mediumRough"), "the roughness select offers this editor's own vocabulary: {json}");
}

/// 🧊️ Lane A's `change-glazing-material-*`/`change-gas-material-*` kinds landed, so both catalogues
/// are editable — except the four reflectances and the infrared transmittance, which still have no
/// mutation kind and are therefore reported rather than offered as a control that would refuse.
#[semio_framework_async_macros::async_test]
async fn glazing_and_gas_materials_expose_exactly_the_fields_a_mutation_kind_names() {
    let glazing = english(&["22"]);
    assert_eq!(component_at(&glazing, "energy-model-inspection.glazing.thickness.input"), "input", "{glazing}");
    assert_eq!(component_at(&glazing, "energy-model-inspection.glazing.solar-transmittance.slider"), "slider");
    assert_eq!(component_at(&glazing, "energy-model-inspection.glazing.emissivity-front.slider"), "slider");
    assert!(glazing.contains(SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID));
    assert_eq!(component_at(&glazing, "energy-model-inspection.glazing.solar-reflectance.value"), "text", "a field with no mutation kind is reported, never offered");
    assert!(!carries_a_tree(&glazing), "{glazing}");
    let gas = english(&["23"]);
    assert_eq!(component_at(&gas, "energy-model-inspection.gas.kind.select"), "select", "{gas}");
    assert_eq!(component_at(&gas, "energy-model-inspection.gas.thickness.input"), "input");
    assert!(gas.contains(SET_GAS_MATERIAL_PROPERTY_ACTION_ID));
    assert!(gas.contains("argon"), "the gas select offers this editor's own fill-gas vocabulary");
}

/// 🧱️ The construction form: an EDITABLE layer stack (a select per layer, a remove button per layer,
/// one appending select) plus the U-value that stack implies, read straight from the engine's own
/// `material::construction_u_value`.
#[semio_framework_async_macros::async_test]
async fn a_selected_construction_edits_its_layer_stack_and_reports_its_u_value() {
    let json = english(&["30"]);
    assert_eq!(component_at(&json, "energy-model-inspection.construction.name.input"), "input", "{json}");
    assert_eq!(component_at(&json, "energy-model-inspection.construction.layer.0.select"), "select", "a layer is re-pointed, not just reported: {json}");
    assert_eq!(component_at(&json, "energy-model-inspection.construction.layer.0.remove.button"), "button");
    assert_eq!(component_at(&json, "energy-model-inspection.construction.add-layer.select"), "select");
    assert!(json.contains(SET_CONSTRUCTION_PROPERTY_ACTION_ID), "every layer control dispatches the construction verb");
    assert!(json.contains("replaceLayer:0"), "the layer select names the slot it replaces: {json}");
    assert!(json.contains("removeLayer"), "the remove button carries its own index: {json}");
    assert!(json.contains("Wood Siding") || json.contains("Plasterboard"), "a layer reads as its material's name: {json}");
    assert!(!carries_a_tree(&json), "{json}");

    // 🔥️ The U-value row reports exactly what the engine computes for that stack.
    let model = crate::examples::demo::model();
    let construction = model.constructions.iter().find(|entry| entry.id.0 == 30).expect("construction 30");
    let layers: Vec<crate::model::Material> = construction.layer_material_ids.iter().filter_map(|layer| model.materials.iter().find(|material| material.id == *layer).cloned()).collect();
    if layers.len() == construction.layer_material_ids.len() && !layers.is_empty() {
        let expected = crate::material::construction_u_value(&layers, crate::material::R_FILM_INTERIOR_M2K_W, crate::material::R_FILM_EXTERIOR_M2K_W);
        assert!(json.contains(&format!("{expected:.3}")), "the U-value row prints the engine's own number ({expected:.3}): {json}");
    }
}

/// 🌡️ THE law the shape-only predecessor got backwards: it asserted the edited key was ABSENT,
/// certifying exactly the shape that made every setpoint edit write the bridge's default. What
/// matters is not the shape but the ROUND TRIP — build the row, merge `value` the way the react host
/// does, run it through `command_from_action`, and check the decoded command carries the typed
/// number.
#[semio_framework_async_macros::async_test]
async fn a_thermostat_control_round_trips_the_typed_number_through_the_action_bridge() {
    let json = english(&["62"]);
    let tree: serde_json::Value = serde_json::from_str(&json).expect("JSON");
    let control = node_at(&tree, "energy-model-inspection.thermostat.heating-throttle.input").unwrap_or_else(|| panic!("{json}"));
    let bindings = control["bindings"].to_string();
    assert!(bindings.contains(SET_THERMOSTAT_SETPOINTS_ACTION_ID), "{bindings}");
    assert!(bindings.contains("heatingThrottleRangeK"), "the whole record travels, the edited key included: {bindings}");
    assert!(bindings.contains("\"field\""), "and `field` names the slot the host-merged value belongs in: {bindings}");
    assert_eq!(component_at(&json, "energy-model-inspection.thermostat.heating-schedule.select"), "select");

    let command = bridged(control, "4.5");
    let model = crate::examples::demo::model();
    let thermostat = model.thermostats.iter().find(|entry| entry.id.0 == 62).expect("thermostat 62");
    assert_eq!(
        command,
        crate::editor::model::EnergyModelEditorCommand::SetThermostatSetpoints {
            thermostat: 62,
            heating_schedule: thermostat.heating_setpoint_schedule_id.0,
            cooling_schedule: thermostat.cooling_setpoint_schedule_id.0,
            heating_throttle_range_k: 4.5,
            cooling_throttle_range_k: thermostat.cooling_throttle_range_k,
        },
        "the control's own binding, merged as the host merges it, decodes to the typed number"
    );
}

/// 📍️ Same round trip for the site's five-scalar payload.
#[semio_framework_async_macros::async_test]
async fn a_site_control_round_trips_the_typed_number_through_the_action_bridge() {
    let mut model = crate::examples::demo::model();
    model.site.north_axis_deg = 15.0;
    let json = panel(&snapshot_of(&model), &[], Locale::En);
    let tree: serde_json::Value = serde_json::from_str(&json).expect("JSON");
    let control = node_at(&tree, "energy-model-inspection.site.north-axis.input").unwrap_or_else(|| panic!("{json}"));
    let command = bridged(control, "30");
    assert_eq!(
        command,
        crate::editor::model::EnergyModelEditorCommand::SetSite {
            latitude_deg: model.site.latitude_deg,
            longitude_deg: model.site.longitude_deg,
            elevation_m: model.site.elevation_m,
            time_zone_hours: model.site.time_zone_hours,
            north_axis_deg: 30.0,
        },
        "the north-axis control writes 30.0, not the bridge's 0.0 default: {json}"
    );
}

/// 🌉️ Runs ONE projected control through the exact wire path a click takes: its authored args, the
/// host's `{value}` merge (`🗣️Interpreter/🟦️.tsx` `dispatchDeclarativeControlAction`), then
/// `command_from_action`. Nothing here is a shape assertion — the point is what the bridge DECODES.
fn bridged(control: &serde_json::Value, value: &str) -> crate::editor::model::EnergyModelEditorCommand {
    use semio_framework_plugin::ArtifactEditor;
    let binding = control["bindings"].as_array().and_then(|bindings| bindings.first()).unwrap_or_else(|| panic!("the control carries a binding: {control}"));
    let action = binding["action"]["name"].as_str().unwrap_or_else(|| panic!("the binding names its action: {binding}")).to_string();
    let scalar = |value: &serde_json::Value| match value {
        serde_json::Value::String(text) => text.clone(),
        other => other.to_string(),
    };
    let mut entries: Vec<(String, semio_framework_plugin::DslValue)> = match &binding["args"] {
        serde_json::Value::Object(map) => map.iter().map(|(key, value)| (key.clone(), semio_framework_plugin::DslValue::String(scalar(value)))).collect(),
        serde_json::Value::Array(pairs) => pairs
            .iter()
            .filter_map(|pair| {
                let entry = pair.as_array()?;
                Some((entry.first()?.as_str()?.to_string(), semio_framework_plugin::DslValue::String(scalar(entry.get(1)?))))
            })
            .collect(),
        _ => Vec::new(),
    };
    entries.retain(|(key, _)| key != "value");
    entries.push(("value".to_string(), semio_framework_plugin::DslValue::String(value.to_string())));
    <crate::editor::model::EnergyModelEditor as ArtifactEditor>::command_from_action(&action, Some(&semio_framework_plugin::DslValue::Object(entries))).unwrap_or_else(|fault| panic!("{action}: {fault:?}"))
}

/// 📍️ Nothing selected: the document summary plus the SITE, which is a singleton with no
/// `EntityId` and therefore has nowhere else to be edited from.
#[semio_framework_async_macros::async_test]
async fn nothing_selected_renders_the_document_summary_and_the_editable_site() {
    let json = english(&[]);
    assert!(json.contains("energy-model-inspection.summary.zones"), "{json}");
    assert_eq!(component_at(&json, "energy-model-inspection.site.latitude.input"), "input", "the site is editable from the summary");
    assert!(json.contains(SET_SITE_ACTION_ID));
    assert!(!json.contains(SET_SURFACE_PROPERTY_ACTION_ID));
    assert!(!carries_a_tree(&json), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn an_id_no_collection_owns_falls_back_to_the_summary() {
    let json = english(&["99999"]);
    assert!(json.contains("energy-model-inspection.summary.zones"), "{json}");
    let unparsable = english(&["not-a-number"]);
    assert!(unparsable.contains("energy-model-inspection.summary.zones"), "{unparsable}");
}

#[semio_framework_async_macros::async_test]
async fn a_multi_selection_headers_the_count_and_inspects_the_first() {
    let json = english(&["40", "41", "50"]);
    assert!(json.contains("3 selected"), "{json}");
    for id in ["40", "41", "50"] {
        assert!(json.contains(&format!("\"{id}\"")));
    }
    assert!(json.contains("energy-model-inspection.surface.name.input"), "the form belongs to the first resolvable id");
}

#[semio_framework_async_macros::async_test]
async fn german_resolves_every_field_label_the_inspector_binds() {
    let snapshot = demo();
    assert!(panel(&snapshot, &["40"], Locale::De).contains("Randbedingung"));
    assert!(panel(&snapshot, &["50"], Locale::De).contains("Brüstungshöhe (m)"));
    assert!(panel(&snapshot, &["1"], Locale::De).contains("Konditioniert"));
    assert!(panel(&snapshot, &[], Locale::De).contains("Standort"));
}

//#region 🎨️ResultsSection
/// 🎨️ Lane D's `set-result-field` had no rendered control anywhere. It has one now, in a Results
/// section the inspector shows with NOTHING selected and under EVERY entity form — so the colour
/// field is one click away whatever the reader is looking at.
#[semio_framework_async_macros::async_test]
async fn every_body_carries_the_result_field_selector() {
    for ids in [&[][..], &["40"][..], &["50"][..], &["1"][..], &["10"][..], &["22"][..], &["23"][..], &["30"][..], &["62"][..]] {
        let json = english(ids);
        assert_eq!(component_at(&json, "energy-model-inspection.results.field.select"), "select", "selection {ids:?} lost the result-field control: {json}");
        assert!(json.contains(crate::editor::model::modes::edit::windows::simulation::SET_RESULT_FIELD_ACTION_ID), "selection {ids:?}: the control dispatches the colour verb");
        for field in crate::editor::model::results::ResultField::ALL {
            assert!(json.contains(field.id()), "selection {ids:?}: the select offers {}", field.id());
        }
        assert!(!carries_a_tree(&json), "{json}");
    }
}

/// 🎨️ The select reads back the CONFIG's current field, and authors no `field` argument — the bridge
/// prefers a named `field` over the host-merged `value`, so authoring one would pin every pick to the
/// value already held and make the control inert.
#[semio_framework_async_macros::async_test]
async fn the_result_field_select_reads_the_config_and_leaves_value_to_the_host() {
    use crate::editor::model::results::ResultField;
    let mut config = EnergyModelConfig::default();
    config.result_field = ResultField::SolarAbsorbed.id().into();
    let json = panel_with(&demo(), &[], &config, Locale::En);
    let tree: serde_json::Value = serde_json::from_str(&json).expect("JSON");
    let control = node_at(&tree, "energy-model-inspection.results.field.select").unwrap_or_else(|| panic!("{json}"));
    assert_eq!(control["component"]["value"].as_str(), Some(ResultField::SolarAbsorbed.id()), "the select shows the field the config holds: {control}");
    let bindings = control["bindings"].to_string();
    assert!(!bindings.contains("\"field\""), "no `field` is authored, or the host's merged value would be ignored: {bindings}");

    let command = bridged(control, ResultField::ConductionGain.id());
    assert_eq!(command, crate::editor::model::EnergyModelEditorCommand::SetResultField { field: ResultField::ConductionGain.id().into() });
    assert!(panel_with(&demo(), &[], &config, Locale::De).contains("Solare Absorption"), "and the labels are authored in both languages");
}

/// 🚧️ The review's second finding: `interzone` needs a partner, so the boundary select no longer
/// offers it and a partner picker does the job instead — round-tripped through the bridge, not
/// asserted as a shape.
#[semio_framework_async_macros::async_test]
async fn a_surface_form_picks_its_interzone_partner_and_never_offers_a_partnerless_interzone() {
    let json = english(&["40"]);
    let tree: serde_json::Value = serde_json::from_str(&json).expect("JSON");
    let boundary = node_at(&tree, "energy-model-inspection.surface.boundary.select").unwrap_or_else(|| panic!("{json}"));
    assert!(!boundary.to_string().contains("interzone"), "a kind that always refuses without a partner is not offered: {boundary}");
    assert!(boundary.to_string().contains("outdoorAir"), "the kinds a lone select CAN apply are still offered: {boundary}");

    let partner = node_at(&tree, "energy-model-inspection.surface.partner.select").unwrap_or_else(|| panic!("no partner picker in {json}"));
    let command = bridged(partner, "41");
    assert_eq!(
        command,
        crate::editor::model::EnergyModelEditorCommand::SetSurfaceProperty { surface: 40, property: "interzonePartner".into(), value: "41".into(), partner_surface: 0 },
        "picking a neighbour is what makes a surface interzone"
    );
}
//#endregion 🎨️ResultsSection
