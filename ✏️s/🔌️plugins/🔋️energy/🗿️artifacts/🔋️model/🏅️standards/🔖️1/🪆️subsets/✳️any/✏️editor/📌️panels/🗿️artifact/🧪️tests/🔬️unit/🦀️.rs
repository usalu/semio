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
    assert!(json.contains("\"1\""), "the single zone is keyed by its own EntityId");
    assert!(json.contains("\"40\"") && json.contains("\"45\""), "the six surfaces are keyed by their own EntityIds");
    assert!(json.contains("\"50\"") && json.contains("\"51\""), "both windows are rows");
    assert!(json.contains("\"22\"") && json.contains("\"23\""), "the glazing material and the gas gap are rows");
    assert!(json.contains("\"62\""), "the thermostat is a row");
    assert!(json.contains("\"63\""), "the ideal-loads system is a row");
}

/// 🕹️ Every entity row binds exactly ONE action — the framework-reserved `interactionSelect` in this
/// editor's own domain — and no row carries a second verb (the argument-arena law).
#[semio_framework_async_macros::async_test]
async fn every_entity_row_picks_into_the_energy_model_domain() {
    let json = english(&demo());
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
    let snapshot = demo();
    let built = render(&snapshot, &nothing(), Locale::En).expect("tree assembles");
    let zones_key = format!("{TREE_NAMESPACE}.zones");
    let zones = built.children.iter().find(|section| section.key.as_str() == zones_key).expect("the zones section is a child of the tree root");
    let zone = zones.children.iter().next().expect("the zones section has its one zone row");
    assert_eq!(zone.key.as_str(), "1", "the zone row is keyed by its EntityId");
    let south_wall = zone.children.iter().find(|row| row.key.as_str() == "40").expect("the south wall is nested under its zone");
    assert!(south_wall.children.iter().any(|row| row.key.as_str() == "50"), "the south window is nested under the south wall");
}

#[semio_framework_async_macros::async_test]
async fn a_shading_surface_is_listed_when_the_model_has_one() {
    let json = english(&shaded());
    assert!(json.contains("\"65\""), "the south overhang is a row: {json}");
    assert!(json.contains(ENERGY_GRANULARITY_SHADING));
    let bare = english(&demo());
    assert!(!bare.contains("\"65\""), "case 600 has no shading surface");
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
    let built = render(&snapshot, &nothing(), Locale::En).expect("tree assembles");
    let zones_key = format!("{TREE_NAMESPACE}.zones");
    let zones = built.children.iter().find(|section| section.key.as_str() == zones_key).expect("zones section");
    let zone = zones.children.iter().next().expect("the zones section has its one zone row");
    let row = zone.children.iter().find(|row| row.key.as_str() == "40").expect("the south wall row");
    let semio_framework_plugin::plugin_app_close_prelude::Component::TreeItem(props) = &row.component else { panic!("an entity row is a tree item") };
    assert_eq!(props.dimmed, Some(true), "a dangling construction reference dims the row");
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
