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
    let built = render(snapshot, &selecting(ids), locale).expect("energy inspector assembly");
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
    assert_eq!(component_at(&json, "energy-model-inspection.material.roughness.value"), "text", "roughness has no mutation kind, so it is reported rather than offered");
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

#[semio_framework_async_macros::async_test]
async fn a_selected_construction_lists_its_layers_by_material_name() {
    let json = english(&["30"]);
    assert_eq!(component_at(&json, "energy-model-inspection.construction.layer.0.value"), "text", "{json}");
    assert!(json.contains("Wood Siding") || json.contains("Plasterboard"), "a layer reads as its material's name: {json}");
}

/// 🌡️ Every thermostat control carries the WHOLE `set-thermostat-setpoints` payload minus its own
/// key, because that verb replaces all four fields at once and the host merges only one value.
#[semio_framework_async_macros::async_test]
async fn a_selected_thermostat_carries_the_whole_setpoint_payload_per_control() {
    let json = english(&["62"]);
    let tree: serde_json::Value = serde_json::from_str(&json).expect("JSON");
    let control = node_at(&tree, "energy-model-inspection.thermostat.heating-throttle.input").unwrap_or_else(|| panic!("{json}"));
    let bindings = control["bindings"].to_string();
    assert!(bindings.contains(SET_THERMOSTAT_SETPOINTS_ACTION_ID), "{bindings}");
    assert!(bindings.contains("coolingSchedule") && bindings.contains("heatingSchedule") && bindings.contains("coolingThrottleRangeK"), "the other three fields travel with the edit: {bindings}");
    assert!(!bindings.contains("heatingThrottleRangeK"), "the edited key is the one the host merges: {bindings}");
    assert_eq!(component_at(&json, "energy-model-inspection.thermostat.heating-schedule.select"), "select");
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
