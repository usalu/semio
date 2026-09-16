use super::*;
use crate::ENERGY_MODEL_DOCUMENT_SCHEMA;

fn snapshot_of(model: &Model) -> EnergyModelSnapshot {
    crate::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None)
}

/// 🧫️ ANSI/ASHRAE 140 case 600 — one zone, six surfaces, two windows, the material/glazing/gas
/// catalogues, four constructions, one thermostat, one ideal-loads system.
fn demo() -> EnergyModelSnapshot {
    snapshot_of(&crate::examples::demo::model())
}

/// 🧫️ Case 610 adds the south overhang, so the shading section has something in it.
fn shaded() -> EnergyModelSnapshot {
    snapshot_of(&crate::bestest::model("610").expect("ANSI/ASHRAE 140 case 610 is registered"))
}

fn nothing() -> EnergyModelInteractionSnapshot {
    EnergyModelInteractionSnapshot::default()
}

fn panel(snapshot: &EnergyModelSnapshot, interaction: &EnergyModelInteractionSnapshot, locale: Locale) -> String {
    let built = render(snapshot, interaction, locale).expect("energy artifact tree assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: built }).expect("energy artifact tree projection")
}

fn english(snapshot: &EnergyModelSnapshot) -> String {
    panel(snapshot, &nothing(), Locale::En)
}

/// 🪙️ `panel_page_rows()` reads the PROCESS-WIDE `UiValue` argument arena, which every other panel
/// test in this binary spends from CONCURRENTLY — so any law about which rows survive is asserted
/// against an EXPLICIT budget, never against whatever the shared arena happens to admit that run.
///
/// ♻️ And every built row is projected AND RETIRED: an argument map dropped without retirement never
/// returns its credit, which would starve the panels assembled by the tests running beside this one.
fn retire(items: UiFixedList<BuiltNode>) -> serde_json::Value {
    let root = PanelTreeBuilder::new(TREE_NAMESPACE).expect("namespace").section(format!("{TREE_NAMESPACE}.probe"), None, true, items).expect("probe section").build().expect("probe tree");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root }).expect("probe projection");
    let probe: serde_json::Value = serde_json::from_str(&json).expect("the projection is JSON");
    // 🌳️ The built root is the TREE; its one child is the probe section that actually holds the rows.
    node_at(&probe, &format!("{TREE_NAMESPACE}.probe")).cloned().unwrap_or(probe)
}

/// 🔎️ The projected node carrying `key`, anywhere under the probe root.
fn node_at<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| node_at(child, key))
}

fn child_keys(node: &serde_json::Value) -> Vec<String> {
    node["children"].as_array().map(|children| children.iter().filter_map(|child| child["key"].as_str().map(str::to_owned)).collect()).unwrap_or_default()
}

/// 🏘️ The one zone row of a BESTEST fixture, built against an explicit budget and retired.
fn zone_tree(snapshot: &EnergyModelSnapshot, interaction: &EnergyModelInteractionSnapshot, rows: usize) -> serde_json::Value {
    let mut budget = PanelRowBudget::new(rows);
    let built = zones_section(&snapshot.model, interaction, Locale::En, &mut budget).expect("the zones section assembles");
    let probe = retire(built);
    node_at(&probe, "1").cloned().expect("the zone row")
}

/// 🧫️ The demo's whole zones demand: one zone, six surfaces, two windows.
const DEMO_ZONES_DEMAND: usize = 9;

#[semio_framework_async_macros::async_test]
async fn the_panel_tab_declares_the_framework_artifact_slot_and_this_body_key() {
    let tab = definition();
    assert_eq!(tab.body_key.as_deref(), Some(BODY_KEY));
    assert_eq!(tab.group, PanelGroup::Workbench);
    assert_eq!(tab.kind, PanelTabKind::App(semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()));
}

/// 🌳️ Every family the tree claims to list is present, keyed by its RAW `EntityId`, and the section
/// keys are namespaced so a row key can never collide with one.
#[semio_framework_async_macros::async_test]
async fn every_section_is_assembled_over_the_document() {
    let json = english(&demo());
    for section in ["site", "zones", "shading", "materials", "glazing-materials", "gas-materials", "constructions", "loads", "controls", "hvac", "schedules"] {
        assert!(json.contains(&format!("{TREE_NAMESPACE}.{section}")), "section {section} missing from {json}");
    }
}

/// 🌳️ Every surface and window of the document reaches a row once the zones section has its whole
/// demand — asserted against an explicit budget, not the shared arena.
#[semio_framework_async_macros::async_test]
async fn every_zone_surface_and_window_is_a_row_keyed_by_its_entity_id() {
    let zone = zone_tree(&demo(), &nothing(), DEMO_ZONES_DEMAND);
    assert_eq!(child_keys(&zone), vec!["40", "41", "42", "43", "44", "45"], "every surface is a row keyed by its own EntityId");
    let south_wall = node_at(&zone, "40").expect("the south wall row");
    assert_eq!(child_keys(south_wall), vec!["50", "51"], "both windows are rows nested under their host wall");
}

/// 🕹️ Every entity row binds exactly ONE action — the framework-reserved `interactionSelect` in this
/// editor's own domain — and no row carries a second verb (the argument-arena law).
/// 🧾️ The catalogue sections key their rows by the raw `EntityId` too — asserted through the
/// section builders with their own budgets, for the same arena reason.
#[semio_framework_async_macros::async_test]
async fn the_catalogue_sections_key_their_rows_by_entity_id() {
    let snapshot = demo();
    let model = &snapshot.model;
    let keys = |built: UiFixedList<BuiltNode>| child_keys(&retire(built));
    let mut budget = PanelRowBudget::new(32);
    assert!(keys(glazing_materials_section(model, Locale::En, &mut budget).expect("glazing")).contains(&"22".to_string()));
    assert!(keys(gas_materials_section(model, Locale::En, &mut budget).expect("gases")).contains(&"23".to_string()));
    assert!(keys(controls_section(model, Locale::En, &mut budget).expect("controls")).contains(&"62".to_string()));
    assert!(keys(hvac_section(model, Locale::En, &mut budget).expect("hvac")).contains(&"63".to_string()));
    assert_eq!(keys(materials_section(model, Locale::En, &mut budget).expect("materials")).len(), model.materials.len());
    assert_eq!(keys(constructions_section(model, Locale::En, &mut budget).expect("constructions")).len(), model.constructions.len());
}

#[semio_framework_async_macros::async_test]
async fn every_entity_row_picks_into_the_energy_model_domain() {
    let snapshot = demo();
    let mut budget = PanelRowBudget::new(DEMO_ZONES_DEMAND);
    let built = zones_section(&snapshot.model, &nothing(), Locale::En, &mut budget).expect("the zones section assembles");
    let json = retire(built).to_string();
    assert!(json.contains("interactionSelect"), "{json}");
    assert!(json.contains(ENERGY_MODEL_INTERACTION_DOMAIN));
    assert!(json.contains(ENERGY_GRANULARITY_SURFACE) && json.contains(ENERGY_GRANULARITY_FENESTRATION) && json.contains(ENERGY_GRANULARITY_ZONE));
    assert!(!json.contains(DELETE_SURFACE_MARKER), "verbs belong to the inspector, never to a tree row: {json}");
}

const DELETE_SURFACE_MARKER: &str = "delete-surface";

/// 🌳️ A window is nested under its host surface, and a surface under its zone — the tree, not a flat
/// list, is what makes "which wall is this window in" readable without a second lookup.
#[semio_framework_async_macros::async_test]
async fn windows_nest_under_their_surface_and_surfaces_under_their_zone() {
    let zone = zone_tree(&demo(), &nothing(), DEMO_ZONES_DEMAND);
    let south_wall = node_at(&zone, "40").expect("the south wall is nested under its zone");
    assert_eq!(child_keys(south_wall), vec!["50", "51"], "both south windows are nested under the south wall");
    let json = english(&demo());
    assert!(json.contains(&format!("{TREE_NAMESPACE}.zones")), "the zones section reaches the rendered tree: {json}");
}

#[semio_framework_async_macros::async_test]
async fn a_shading_surface_is_listed_when_the_model_has_one() {
    let snapshot = shaded();
    let built = shading_section(&snapshot.model, Locale::En, &mut PanelRowBudget::new(4)).expect("the shading section assembles");
    assert_eq!(child_keys(&retire(built)), vec!["65"], "the south overhang is a row keyed by its own EntityId");
    let bare = demo();
    let empty = shading_section(&bare.model, Locale::En, &mut PanelRowBudget::new(4)).expect("empty section");
    assert!(child_keys(&retire(empty)).is_empty(), "case 600 has no shading surface");
}

/// 🗓️ A schedule row is read-only: a `ScheduleId` lives in its own id space, so keying a pick by its
/// raw number would hand the inspector an `EntityId` that belongs to something else entirely.
#[semio_framework_async_macros::async_test]
async fn schedules_and_the_site_are_read_only_rows() {
    let snapshot = demo();
    let built = render(&snapshot, &nothing(), Locale::En).expect("tree assembles");
    for section_key in [format!("{TREE_NAMESPACE}.schedules"), format!("{TREE_NAMESPACE}.site")] {
        let section = built.children.iter().find(|section| section.key.as_str() == section_key.as_str()).unwrap_or_else(|| panic!("section {section_key} exists"));
        for row in section.children.iter() {
            assert!(row.bindings.is_empty(), "read-only row {} must bind nothing", row.key.as_str());
            if let semio_framework_plugin::plugin_app_close_prelude::Component::TreeItem(props) = &row.component {
                assert!(props.row_actions.is_empty(), "read-only row {} must carry no row action", row.key.as_str());
            }
        }
    }
}

/// 📭️ An empty model still renders: each entity section shows its placeholder instead of refusing.
#[semio_framework_async_macros::async_test]
async fn an_empty_model_renders_placeholders_rather_than_refusing() {
    let json = english(&snapshot_of(&Model::default()));
    assert!(json.contains(&format!("{TREE_NAMESPACE}.zones.empty")), "{json}");
    assert!(json.contains(&format!("{TREE_NAMESPACE}.materials.empty")));
    assert!(json.contains("None"), "the placeholder label is rendered");
}

/// 🪙️ Max-min fair paging: every section that fits keeps all its rows, only the widest truncate, and
/// the quotas never exceed the page.
#[semio_framework_async_macros::async_test]
async fn section_quotas_are_max_min_fair_and_never_exceed_the_page() {
    let demands = [0, 90, 1, 12, 1, 1, 4, 2, 1, 1, 0];
    let page = 40;
    let quotas = section_quotas(demands, page);
    assert!(quotas.iter().sum::<usize>() <= page, "{quotas:?} overspends the page");
    for index in 0..SECTIONS {
        assert!(quotas[index] <= demands[index], "section {index} was given more rows than it wants");
    }
    for index in [2usize, 4, 5, 6, 7, 8, 9] {
        assert_eq!(quotas[index], demands[index], "small section {index} must keep every row");
    }
    assert!(quotas[1] < demands[1], "only the widest section truncates");
    let roomy = section_quotas(demands, 1000);
    assert_eq!(roomy, demands, "a page that fits everything truncates nothing");
}

/// ➕️ A document past the page closes each truncated section with a continuation row instead of
/// failing an argument admission mid-row.
#[semio_framework_async_macros::async_test]
async fn an_oversized_document_pages_with_continuation_rows() {
    let mut model = crate::examples::demo::model();
    let seed = model.materials[0].clone();
    for index in 0..400u32 {
        let mut copy = seed.clone();
        copy.id = crate::model::EntityId(10_000 + index);
        copy.name = format!("Filler {index}");
        model.materials.push(copy);
    }
    let json = english(&snapshot_of(&model));
    assert!(json.contains(&format!("{TREE_NAMESPACE}.materials.more")), "a truncated section closes with its continuation row: {json}");
}

/// 🚨️ A surface whose construction does not resolve is dimmed rather than dropped — the tree is the
/// place a modeller notices a dangling reference.
#[semio_framework_async_macros::async_test]
async fn a_dangling_construction_reference_dims_its_surface_row() {
    let mut model = crate::examples::demo::model();
    model.surfaces[0].construction_id = crate::model::EntityId(9_999);
    let snapshot = snapshot_of(&model);
    let zone = zone_tree(&snapshot, &nothing(), DEMO_ZONES_DEMAND);
    let row = node_at(&zone, "40").expect("the south wall row");
    assert_eq!(row["component"]["dimmed"].as_bool(), Some(true), "a dangling construction reference dims the row: {row}");
}

#[semio_framework_async_macros::async_test]
async fn german_resolves_every_section_heading() {
    let json = panel(&demo(), &nothing(), Locale::De);
    assert!(json.contains("Zonen"), "{json}");
    assert!(json.contains("Standort"));
    assert!(json.contains("Konstruktionen"));
    assert!(json.contains("Anlagentechnik"));
    assert!(json.contains("Zeitpläne"));
}

/// 🎯️ The live selection and hover reach the builder (capped at one `UiFixedList` page) — the tree
/// never stores selection itself.
#[semio_framework_async_macros::async_test]
async fn a_wide_selection_marks_only_its_first_page() {
    let ids: Vec<String> = (0..64).map(|index| index.to_string()).collect();
    assert_eq!(marked_ids(&ids).len(), MARKED_IDS_LIMIT);
    let interaction = EnergyModelInteractionSnapshot { selected_ids: ids, hovered_ids: vec!["40".into()] };
    let json = panel(&demo(), &interaction, Locale::En);
    assert!(json.contains(&format!("\"{TREE_NAMESPACE}\"")), "the tree still assembles under a wide selection: {json}");
}

/// 🧾️ The demands table is what the quota split is computed from, so it has to count the nested rows
/// the zones section actually materialises.
#[semio_framework_async_macros::async_test]
async fn section_demands_count_every_interactive_row_including_the_nested_ones() {
    let model = crate::examples::demo::model();
    let demands = section_demands(&model);
    assert_eq!(demands[0], 0, "the site section binds no argument map");
    assert_eq!(demands[1], model.zones.len() + model.spaces.len() + model.surfaces.len() + model.fenestrations.len());
    assert_eq!(demands[10], 0, "the schedules section binds no argument map");
}

/// 🎯️ THE defect the first browser probe found: under a tight page the first surface's own windows
/// collapsed into a `…windows.more` marker no tree row can expand, so a user could not reach a window
/// while its host wall was selected. The marked surface now claims its whole demand first.
#[semio_framework_async_macros::async_test]
async fn a_marked_surface_keeps_its_windows_under_a_tight_page() {
    // 🧫️ Case 620 puts one window on the east wall (41) and one on the west wall (43), so a page
    // with room for exactly ONE window shows which wall the allocation favours.
    let snapshot = snapshot_of(&crate::bestest::model("620").expect("ANSI/ASHRAE 140 case 620 is registered"));
    let tight = 1 + 6 + 1;
    let picked = EnergyModelInteractionSnapshot { selected_ids: vec!["43".into()], hovered_ids: Vec::new() };
    let zone = zone_tree(&snapshot, &picked, tight);
    assert_eq!(child_keys(&zone), vec!["40", "41", "42", "43", "44", "45"], "breadth first: every wall keeps its own row");
    assert_eq!(child_keys(node_at(&zone, "43").expect("the marked west wall")), vec!["51"], "the MARKED wall's window survives the truncation");
    assert!(!child_keys(node_at(&zone, "41").expect("the east wall")).contains(&"50".to_string()), "the unmarked wall's window is the one that gives way");

    // 🧭️ With nothing marked the depth goes to the first wall that wants it, in document order.
    let unmarked = zone_tree(&snapshot, &nothing(), tight);
    assert_eq!(child_keys(node_at(&unmarked, "41").expect("the east wall")), vec!["50"], "document order decides when the domain marks nothing");
}

/// 🎯️ With nothing marked the page still goes to the surfaces in document order — the marked-first
/// pass must not reorder the rendered rows, only who survives a truncation.
#[semio_framework_async_macros::async_test]
async fn the_surface_rows_stay_in_document_order_whatever_is_marked() {
    let picked = EnergyModelInteractionSnapshot { selected_ids: vec!["44".into()], hovered_ids: Vec::new() };
    let zone = zone_tree(&demo(), &picked, DEMO_ZONES_DEMAND);
    assert_eq!(child_keys(&zone), vec!["40", "41", "42", "43", "44", "45"], "the marked surface is served first but rendered in place");
}
