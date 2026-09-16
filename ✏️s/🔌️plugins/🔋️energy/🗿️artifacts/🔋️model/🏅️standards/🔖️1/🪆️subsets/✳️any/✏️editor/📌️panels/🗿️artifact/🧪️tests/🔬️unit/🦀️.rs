use super::*;
use crate::ENERGY_MODEL_DOCUMENT_SCHEMA;
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

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

/// 🪟️ One host window request for this body — exactly what a scroll, an expand or a collapse in the
/// React shell sends the guest on the next refresh.
fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: BODY_KEY.to_string(), node_key: node_key.to_string(), open, offset, rows }
}

/// 🪟️ Opens one container wide enough to show all of it — the "the user expanded this section" state.
fn opened(node_key: &str) -> TreeWindowRequest {
    request(node_key, Some(true), 0, 64)
}

fn hosted(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

/// ♻️ Every built row is projected AND RETIRED: an argument map dropped without retirement never
/// returns its credit, which would starve the panels assembled by the tests running beside this one.
fn panel(snapshot: &EnergyModelSnapshot, interaction: &EnergyModelInteractionSnapshot, locale: Locale, view: &ViewModel) -> String {
    let built = render(snapshot, interaction, locale, &TreeWindows::for_body(view, BODY_KEY)).expect("energy artifact tree assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: built }).expect("energy artifact tree projection")
}

/// 🏠️ The first paint: no host state at all, so every container opens at its author default and the
/// shared first-paint budget decides how much of each is materialised.
fn unhosted(snapshot: &EnergyModelSnapshot, interaction: &EnergyModelInteractionSnapshot, locale: Locale) -> String {
    let built = render(snapshot, interaction, locale, &TreeWindows::unhosted()).expect("energy artifact tree assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: built }).expect("energy artifact tree projection")
}

fn english(snapshot: &EnergyModelSnapshot) -> String {
    unhosted(snapshot, &nothing(), Locale::En)
}

fn tree_of(json: &str) -> serde_json::Value {
    serde_json::from_str(json).expect("the artifact projection is JSON")
}

/// 🔎️ The projected node carrying `key`, anywhere under the tree root.
fn node_at<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| node_at(child, key))
}

fn child_keys(node: &serde_json::Value) -> Vec<String> {
    node["children"].as_array().map(|children| children.iter().filter_map(|child| child["key"].as_str().map(str::to_owned)).collect()).unwrap_or_default()
}

/// 🪟️ The `{total, offset}` a container stamps — the full extent of its logical child list and where
/// the materialised slice starts.
fn window_of(node: &serde_json::Value) -> (usize, usize) {
    let window = &node["component"]["window"];
    let total = window["total"].as_u64().unwrap_or_else(|| panic!("a container stamps its window: {node}"));
    (total as usize, window["offset"].as_u64().unwrap_or(0) as usize)
}

fn section_key(suffix: &str) -> String {
    format!("{TREE_NAMESPACE}.{suffix}")
}

/// 🧫️ The demo's zone row children: no spaces, six surfaces.
const DEMO_ZONE_CHILDREN: usize = 6;

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
        assert!(json.contains(&section_key(section)), "section {section} missing from {json}");
    }
}

/// 🌳️ Every surface and window of the document reaches a row — the zones section and the zone row are
/// open by author default, so the first paint already shows the whole hierarchy of this fixture.
#[semio_framework_async_macros::async_test]
async fn every_zone_surface_and_window_is_a_row_keyed_by_its_entity_id() {
    let tree = tree_of(&english(&demo()));
    let zone = node_at(&tree, "1").expect("the zone row");
    assert_eq!(child_keys(zone), vec!["40", "41", "42", "43", "44", "45"], "every surface is a row keyed by its own EntityId");
    let south_wall = node_at(zone, "40").expect("the south wall row");
    assert_eq!(child_keys(south_wall), vec!["50", "51"], "both windows are rows nested under their host wall");
}

/// 🧾️ The catalogue sections key their rows by the raw `EntityId` too — read through the windows the
/// host opens them with, because they are closed by author default.
#[semio_framework_async_macros::async_test]
async fn the_catalogue_sections_key_their_rows_by_entity_id() {
    let snapshot = demo();
    let model = &snapshot.model;
    let view = hosted(vec![
        opened(&section_key("materials")),
        opened(&section_key("glazing-materials")),
        opened(&section_key("gas-materials")),
        opened(&section_key("constructions")),
        opened(&section_key("controls")),
        opened(&section_key("hvac")),
    ]);
    let tree = tree_of(&panel(&snapshot, &nothing(), Locale::En, &view));
    let keys = |suffix: &str| child_keys(node_at(&tree, &section_key(suffix)).unwrap_or_else(|| panic!("section {suffix} exists: {tree}")));
    assert!(keys("glazing-materials").contains(&"22".to_string()));
    assert!(keys("gas-materials").contains(&"23".to_string()));
    assert!(keys("controls").contains(&"62".to_string()));
    assert!(keys("hvac").contains(&"63".to_string()));
    assert_eq!(keys("materials").len(), model.materials.len());
    assert_eq!(keys("constructions").len(), model.constructions.len());
}

/// 🕹️ A pick row carries NO argument map: it declares its `granularity`, and the ONE
/// `interactionSelect` the tree root binds turns a click on the row's raw id into a pick. That is the
/// whole argument-arena law — a document of any size costs the arena one binding.
#[semio_framework_async_macros::async_test]
async fn every_entity_row_picks_into_the_energy_model_domain() {
    let tree = tree_of(&english(&demo()));
    let root_bindings = tree["bindings"].to_string();
    assert!(root_bindings.contains(INTERACTION_SELECT_ACTION_ID), "the tree itself binds the pick: {root_bindings}");
    assert_eq!(root_bindings.matches(INTERACTION_SELECT_ACTION_ID).count(), 1, "exactly ONE tree-level pick binding: {root_bindings}");
    assert!(tree.to_string().contains(ENERGY_MODEL_INTERACTION_DOMAIN));

    for (key, granularity) in [("1", ENERGY_GRANULARITY_ZONE), ("40", ENERGY_GRANULARITY_SURFACE), ("50", ENERGY_GRANULARITY_FENESTRATION)] {
        let row = node_at(&tree, key).unwrap_or_else(|| panic!("row {key} exists: {tree}"));
        assert_eq!(row["component"]["granularity"].as_str(), Some(granularity), "row {key} declares the granularity it picks at: {row}");
        assert!(row["bindings"].as_array().is_none_or(|bindings| bindings.is_empty()), "a pick row authors no binding of its own: {row}");
    }
    assert!(!tree.to_string().contains(DELETE_SURFACE_MARKER), "verbs belong to the inspector, never to a tree row");
}

const DELETE_SURFACE_MARKER: &str = "delete-surface";

/// 🌳️ A window is nested under its host surface, and a surface under its zone — the tree, not a flat
/// list, is what makes "which wall is this window in" readable without a second lookup.
#[semio_framework_async_macros::async_test]
async fn windows_nest_under_their_surface_and_surfaces_under_their_zone() {
    let tree = tree_of(&english(&demo()));
    let zone = node_at(&tree, "1").expect("the zone row");
    let south_wall = node_at(zone, "40").expect("the south wall is nested under its zone");
    assert_eq!(child_keys(south_wall), vec!["50", "51"], "both south windows are nested under the south wall");
    assert!(node_at(&tree, &section_key("zones")).is_some(), "the zones section reaches the rendered tree: {tree}");
}

#[semio_framework_async_macros::async_test]
async fn a_shading_surface_is_listed_when_the_model_has_one() {
    let view = hosted(vec![opened(&section_key("shading"))]);
    let tree = tree_of(&panel(&shaded(), &nothing(), Locale::En, &view));
    let section = node_at(&tree, &section_key("shading")).expect("the shading section");
    assert_eq!(child_keys(section), vec!["65"], "the south overhang is a row keyed by its own EntityId");
    assert_eq!(window_of(section).0, 1, "and the section reports the one shading surface it holds");

    let bare = tree_of(&panel(&demo(), &nothing(), Locale::En, &view));
    let empty = node_at(&bare, &section_key("shading")).expect("the shading section");
    assert_eq!(child_keys(empty), vec![section_key("shading.empty")], "case 600 has no shading surface, so the section holds only its placeholder");
    assert!(empty["component"]["window"].is_null(), "an empty container publishes no window: {empty}");
}

/// 🗓️ A schedule row is read-only: a `ScheduleId` lives in its own id space, so keying a pick by its
/// raw number would hand the inspector an `EntityId` that belongs to something else entirely.
///
/// 📍️ The SITE row is the deliberate exception and is asserted separately below — it has no
/// `EntityId` either, but it dispatches the framework's CLEARING pick rather than a target.
#[semio_framework_async_macros::async_test]
async fn schedule_rows_are_read_only() {
    let view = hosted(vec![opened(&section_key("schedules"))]);
    let tree = tree_of(&panel(&demo(), &nothing(), Locale::En, &view));
    let section = node_at(&tree, &section_key("schedules")).expect("the schedules section");
    let rows = section["children"].as_array().cloned().unwrap_or_default();
    assert_eq!(rows.len(), 5, "the five schedule families are listed: {section}");
    for row in &rows {
        assert!(row["bindings"].as_array().is_none_or(|bindings| bindings.is_empty()), "every schedule row must bind nothing: {row}");
        assert!(row["component"]["granularity"].is_null(), "and must not be a pick target: {row}");
        assert!(row["component"]["rowActions"].as_array().is_none_or(|actions| actions.is_empty()), "every schedule row must carry no row action: {row}");
    }
}

/// 📭️ An empty model still renders: an empty section shows its placeholder instead of refusing.
#[semio_framework_async_macros::async_test]
async fn an_empty_model_renders_placeholders_rather_than_refusing() {
    let view = hosted(vec![opened(&section_key("materials"))]);
    let tree = tree_of(&panel(&snapshot_of(&Model::default()), &nothing(), Locale::En, &view));
    assert!(node_at(&tree, &section_key("zones.empty")).is_some(), "{tree}");
    assert!(node_at(&tree, &section_key("materials.empty")).is_some(), "{tree}");
    assert!(tree.to_string().contains("None"), "the placeholder label is rendered");
}

//#region 🪟️WindowLaws
/// 🪟️ THE law this panel was rebuilt for: a document an order of magnitude past one viewport stamps
/// every container's FULL extent and materialises only the slice the host asked for. No section is
/// truncated, no `+N` row is invented, and the scrollbar the host paints spans the whole catalogue.
#[semio_framework_async_macros::async_test]
async fn an_oversized_document_stamps_every_containers_total_and_materialises_only_its_window() {
    let mut model = crate::examples::demo::model();
    let seed = model.materials[0].clone();
    for index in 0..400u32 {
        let mut copy = seed.clone();
        copy.id = crate::model::EntityId(10_000 + index);
        copy.name = format!("Filler {index}");
        model.materials.push(copy);
    }
    let materials = model.materials.len();
    let snapshot = snapshot_of(&model);
    let view = hosted(vec![request(&section_key("materials"), Some(true), 0, 10)]);
    let json = panel(&snapshot, &nothing(), Locale::En, &view);
    let tree = tree_of(&json);

    let section = node_at(&tree, &section_key("materials")).expect("the materials section");
    assert_eq!(window_of(section), (materials, 0), "the section reports the WHOLE catalogue, however little of it is built: {section}");
    let built = child_keys(section);
    assert_eq!(built.len(), 10, "exactly the ten rows the host asked for: {built:?}");
    let expected: Vec<String> = model.materials.iter().take(10).map(|material| energy_target_id(material.id)).collect();
    assert_eq!(built, expected, "keyed by the raw EntityId, in document order");

    let zone = node_at(&tree, "1").expect("the zone row");
    assert_eq!(window_of(zone).0, DEMO_ZONE_CHILDREN, "the nested zone container stamps its own total too: {zone}");

    assert!(!json.contains(".more"), "no continuation row survives anywhere: {json}");
    assert!(!json.contains("\"+"), "and no `+N` label either: {json}");
}

/// 🪟️ A closed container costs one stamped `total` and nothing else — that is what makes a document
/// with ten thousand entities cheap to paint, and what the host's expand arrow reads.
#[semio_framework_async_macros::async_test]
async fn a_closed_container_stamps_its_total_and_materialises_no_child() {
    let view = hosted(vec![request("1", Some(false), 0, 64)]);
    let tree = tree_of(&panel(&demo(), &nothing(), Locale::En, &view));
    let zone = node_at(&tree, "1").expect("the zone row");
    assert_eq!(window_of(zone).0, DEMO_ZONE_CHILDREN, "a closed zone still reports everything it owns: {zone}");
    assert!(child_keys(zone).is_empty(), "and materialises none of it: {zone}");

    // 🧾️ A section closed by AUTHOR default behaves identically without any host state at all.
    let first_paint = tree_of(&english(&demo()));
    let materials = node_at(&first_paint, &section_key("materials")).expect("the materials section");
    assert!(child_keys(materials).is_empty(), "a section that opens closed builds nothing: {materials}");
    assert!(window_of(materials).0 > 0, "but still reports its extent: {materials}");
}

/// 🪟️ Scrolling a nested group: the host names the surface and the slice, and exactly that slice of
/// its windows is materialised — keyed by raw `EntityId`, never renumbered by the offset.
#[semio_framework_async_macros::async_test]
async fn a_tree_window_request_materialises_exactly_its_slice_of_a_surface_group() {
    let south_wall = "40";
    let view = hosted(vec![request(south_wall, Some(true), 1, 1)]);
    let tree = tree_of(&panel(&demo(), &nothing(), Locale::En, &view));
    let wall = node_at(&tree, south_wall).expect("the south wall row");
    assert_eq!(window_of(wall), (2, 1), "the wall reports both its windows and that the slice starts at the second: {wall}");
    assert_eq!(child_keys(wall), vec!["51"], "exactly entries [1, 2) are built: {wall}");

    let whole = hosted(vec![opened(south_wall)]);
    let tree = tree_of(&panel(&demo(), &nothing(), Locale::En, &whole));
    assert_eq!(child_keys(node_at(&tree, south_wall).expect("the south wall row")), vec!["50", "51"], "and a wide window builds both");
}

/// 🪟️ The nesting a hand-paged workaround used to starve: a zone owning far more surfaces than one
/// viewport shows keeps EVERY surface reachable, and a surface's own windows are never collapsed into
/// a marker no row can expand (the defect the first browser probe of this ticket found).
#[semio_framework_async_macros::async_test]
async fn a_wide_zone_keeps_every_surface_reachable_and_no_window_is_ever_collapsed() {
    let mut model = crate::bestest::model("620").expect("ANSI/ASHRAE 140 case 620 is registered");
    let seed = model.surfaces[0].clone();
    for index in 0..120u32 {
        let mut copy = seed.clone();
        copy.id = crate::model::EntityId(20_000 + index);
        copy.name = format!("Filler wall {index}");
        model.surfaces.push(copy);
    }
    let surfaces = model.surfaces.len();
    let snapshot = snapshot_of(&model);

    // 🧭️ The reader scrolls to the tail of the zone: the last surfaces are built, the zone's total is
    // unchanged, and the east wall reached through its own window still owns its opening.
    let view = hosted(vec![request("1", Some(true), (surfaces - 4) as u32, 4), opened("41")]);
    let json = panel(&snapshot, &nothing(), Locale::En, &view);
    let tree = tree_of(&json);
    let zone = node_at(&tree, "1").expect("the zone row");
    assert_eq!(window_of(zone), (surfaces, surfaces - 4), "the zone reports every wall it owns and where the slice starts: {zone}");
    assert_eq!(child_keys(zone).len(), 4, "and builds exactly the four the host asked for: {zone}");

    let head = hosted(vec![request("1", Some(true), 0, 8), opened("41")]);
    let tree = tree_of(&panel(&snapshot, &nothing(), Locale::En, &head));
    let east_wall = node_at(&tree, "41").expect("the east wall row");
    assert_eq!(child_keys(east_wall), vec!["50"], "its own window is materialised, not collapsed: {east_wall}");
    assert_eq!(window_of(east_wall).0, 1);
}
//#endregion 🪟️WindowLaws

/// 🚨️ A surface whose construction does not resolve is dimmed rather than dropped — the tree is the
/// place a modeller notices a dangling reference.
#[semio_framework_async_macros::async_test]
async fn a_dangling_construction_reference_dims_its_surface_row() {
    let mut model = crate::examples::demo::model();
    model.surfaces[0].construction_id = crate::model::EntityId(9_999);
    let tree = tree_of(&english(&snapshot_of(&model)));
    let row = node_at(&tree, "40").expect("the south wall row");
    assert_eq!(row["component"]["dimmed"].as_bool(), Some(true), "a dangling construction reference dims the row: {row}");
}

#[semio_framework_async_macros::async_test]
async fn german_resolves_every_section_heading() {
    let json = unhosted(&demo(), &nothing(), Locale::De);
    assert!(json.contains("Zonen"), "{json}");
    assert!(json.contains("Standort"));
    assert!(json.contains("Konstruktionen"));
    assert!(json.contains("Anlagentechnik"));
    assert!(json.contains("Zeitpläne"));
}

/// 🎯️ The live selection and hover reach the builder (capped at one `UiFixedList` page) — the tree
/// never stores selection itself.
#[semio_framework_async_macros::async_test]
async fn a_wide_selection_still_assembles_the_whole_tree() {
    let ids: Vec<String> = (0..64).map(|index| index.to_string()).collect();
    assert_eq!(marked_ids(&ids).len(), MARKED_IDS_LIMIT);
    let interaction = EnergyModelInteractionSnapshot { selected_ids: ids, hovered_ids: vec!["40".into()] };
    let json = unhosted(&demo(), &interaction, Locale::En);
    assert!(json.contains(&format!("\"{TREE_NAMESPACE}\"")), "the tree still assembles under a wide selection: {json}");
    assert!(!json.contains(".more"), "and never with a continuation row: {json}");
}

/// 🧭️ The rows of a container are always its document order — a marked row is not floated to the top,
/// because a tree that reshuffles under the reader's cursor is unreadable.
#[semio_framework_async_macros::async_test]
async fn the_surface_rows_stay_in_document_order_whatever_is_marked() {
    let picked = EnergyModelInteractionSnapshot { selected_ids: vec!["44".into()], hovered_ids: Vec::new() };
    let tree = tree_of(&unhosted(&demo(), &picked, Locale::En));
    let zone = node_at(&tree, "1").expect("the zone row");
    assert_eq!(child_keys(zone), vec!["40", "41", "42", "43", "44", "45"], "the marked surface is rendered in place");
}

//#region 📍️SiteRowAndMarkers
/// 📍️ The site has no `EntityId`, so its row cannot be a domain target — but it is where the site
/// form is reached from. It dispatches the framework's own CLEARING pick (`merge: replace` with an
/// EMPTY target list), which is exactly the state the inspector renders its site form in.
#[semio_framework_async_macros::async_test]
async fn the_site_row_opens_the_site_form_by_clearing_the_selection() {
    let json = english(&demo());
    let tree = tree_of(&json);
    let row = node_at(&tree, &section_key("site.row")).unwrap_or_else(|| panic!("{json}"));
    let bindings = row["bindings"].to_string();
    assert!(bindings.contains(INTERACTION_SELECT_ACTION_ID), "the site row picks through the framework domain: {bindings}");
    assert!(bindings.contains(ENERGY_MODEL_INTERACTION_DOMAIN), "on the energy domain: {bindings}");
    assert!(bindings.contains("replace"), "replacing the selection: {bindings}");
    assert!(bindings.contains("[]"), "with NO targets, which is how the framework spells 'clear': {bindings}");
    assert!(row.to_string().contains("47.") || row.to_string().contains("°"), "and it still reports the site's own coordinates: {row}");
}

/// 🎯️ `PanelTreeBuilder::selected`/`::highlighted` are declared but do not paint in this SDK wave, so
/// the state also rides in the label — otherwise a reader cannot see that a 3d pick and a tree row
/// are the same selection.
#[semio_framework_async_macros::async_test]
async fn a_marked_row_carries_its_state_in_the_label() {
    let interaction = EnergyModelInteractionSnapshot { selected_ids: vec!["44".into()], hovered_ids: vec!["41".into()] };
    let tree = tree_of(&unhosted(&demo(), &interaction, Locale::En));
    let selected = node_at(&tree, "44").expect("the selected wall").to_string();
    let hovered = node_at(&tree, "41").expect("the hovered wall").to_string();
    let plain = node_at(&tree, "42").expect("an unmarked wall").to_string();
    assert!(selected.contains(MarkedAs::Selected.prefix().trim()), "the selected row is marked: {selected}");
    assert!(hovered.contains(MarkedAs::Hovered.prefix().trim()), "the hovered row is marked: {hovered}");
    assert!(!plain.contains(MarkedAs::Selected.prefix().trim()) && !plain.contains(MarkedAs::Hovered.prefix().trim()), "an unmarked row reads exactly as before: {plain}");
    assert_eq!(MarkedAs::Unmarked.prefix(), "", "an unmarked label is untouched");
}
//#endregion 📍️SiteRowAndMarkers
