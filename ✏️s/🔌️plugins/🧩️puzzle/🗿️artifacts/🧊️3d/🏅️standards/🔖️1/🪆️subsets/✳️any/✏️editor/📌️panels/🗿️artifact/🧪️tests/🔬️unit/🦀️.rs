use super::*;
use crate::editor::puzzle3d::terminology::puzzle3d_labels;
use crate::editor::puzzle3d::{empty_fixture, nakagin_fixture};
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

const RETIREMENT_DRAIN_STEPS: usize = 4096;

fn native() -> &'static Puzzle3dLabels {
    puzzle3d_labels(&ViewModel { terminology: semio_framework_plugin::Terminology::Native, ..Default::default() }).expect("admitted host axis")
}

/// ♻️ Stands in for the reactor's own one-page-per-turn retirement pump, which no unit test has: a
/// released tree hands its child backings and its `UiValue` pages to the retirement authorities, and
/// only closing them returns the process-wide admission credit the next build needs.
fn drain_retired_ui_owners() {
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if ui::close_built_node_page_one() {
            break;
        }
    }
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if ui::close_ui_value_page_one() {
            break;
        }
    }
}

/// 🪟️ One host window request for this body — exactly what a scroll, an expand or a collapse in the
/// React shell sends the guest on the next refresh.
fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: BODY_KEY.to_string(), node_key: node_key.to_string(), open, offset, rows }
}

fn hosted(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

fn section_key(suffix: &str) -> String {
    format!("{ROOT}.{suffix}")
}

/// ♻️ Every built row is projected AND RETIRED: an argument map dropped without retirement never
/// returns its credit, which would starve the panels assembled by the tests running beside this one.
fn panel(fixture: &Puzzle3dFixture, view: &ViewModel) -> String {
    drain_retired_ui_owners();
    let built = render(fixture, native(), &TreeWindows::for_body(view, BODY_KEY)).expect("puzzle3d artifact tree assembly");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: built }).expect("puzzle3d artifact tree projection");
    drain_retired_ui_owners();
    json
}

/// 🏠️ The first paint: no host state at all, so every container opens at its author default and the
/// shared first-paint budget decides how much of each is materialised.
fn first_paint(fixture: &Puzzle3dFixture) -> String {
    drain_retired_ui_owners();
    let built = render(fixture, native(), &TreeWindows::unhosted()).expect("puzzle3d artifact tree assembly");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: built }).expect("puzzle3d artifact tree projection");
    drain_retired_ui_owners();
    json
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

/// 🏗️ A synthetic document of `objects` objects carrying `vortices` vortices each — the shape the flagship
/// Nakagin fixture has (180 objects, ≈2 vortices per object) and the shape a `DOCUMENT_OBJECT_SLOTS`-scale
/// document has, without depending on either asset's contents.
fn scaled_fixture(objects: usize, vortices: usize) -> Puzzle3dFixture {
    let mut fixture = empty_fixture();
    fixture.objects = (0..objects)
        .map(|index| Puzzle3dObject {
            id: format!("object-{index}"),
            label: None,
            object_kind: Some("capsule".into()),
            origin: [f64::from(index as u32), 0.0, 0.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: (0..vortices).map(|slot| Puzzle3dVortex { id: format!("vortex-{slot}"), vortex_kind: Some("edge".into()), ..Default::default() }).collect(),
            hidden: false,
            locked: false,
        })
        .collect();
    fixture
}

#[test]
fn the_panel_tab_declares_the_framework_artifact_slot_and_this_body_key() {
    let tab = definition();
    assert_eq!(tab.body_key.as_deref(), Some(BODY_KEY));
    assert_eq!(tab.group, PanelGroup::Workbench);
    assert_eq!(tab.kind, PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()));
}

/// 🌳️ The four document sections are always assembled, whatever the host asked for.
#[test]
fn the_outliner_assembles_its_four_document_sections() {
    let tree = tree_of(&first_paint(&nakagin_fixture()));
    assert_eq!(tree["children"].as_array().map(Vec::len), Some(4), "the outliner keeps its four document sections: {tree}");
    for suffix in ["objects", "references", "target-volumes", "attractions"] {
        assert!(node_at(&tree, &section_key(suffix)).is_some(), "section {suffix} missing from {tree}");
    }
}

//#region 🪟️WindowLaws
/// 🪟️ THE law this panel was rebuilt for: the flagship document (180 objects, 358 vortices) stamps
/// every container's FULL extent and materialises only the slice the host asked for. Nothing is
/// truncated, no `+N` row is invented, and the scrollbar the host paints spans the whole document.
#[test]
fn an_oversized_document_stamps_every_containers_total_and_materialises_only_its_window() {
    let fixture = nakagin_fixture();
    assert!(fixture.objects.len() >= 100, "the Nakagin fixture must be the large document: {} objects", fixture.objects.len());
    let nested = fixture.objects.iter().find(|object| !object.vortices.is_empty()).expect("Nakagin objects carry vortices").clone();
    let view = hosted(vec![request(&section_key("objects"), Some(true), 0, 10), request(&nested.id, Some(true), 0, 8)]);
    let json = panel(&fixture, &view);
    let tree = tree_of(&json);

    let section = node_at(&tree, &section_key("objects")).expect("the objects section");
    assert_eq!(window_of(section), (fixture.objects.len(), 0), "the section reports the WHOLE document, however little of it is built: {section}");
    let built = child_keys(section);
    assert_eq!(built.len(), 10, "exactly the ten rows the host asked for: {built:?}");
    let expected: Vec<String> = fixture.objects.iter().take(10).map(|object| object.id.clone()).collect();
    assert_eq!(built, expected, "keyed by the raw object id, in document order");

    let group = node_at(&tree, &nested.id).expect("the opened object row");
    assert_eq!(window_of(group).0, nested.vortices.len(), "the nested vortex container stamps its own total too: {group}");
    assert!(child_keys(group).len() <= nested.vortices.len().min(8), "and materialises no more than its slice: {group}");

    for suffix in ["references", "target-volumes", "attractions"] {
        let closed = node_at(&tree, &section_key(suffix)).expect("every section is assembled");
        assert!(closed["component"]["window"]["total"].as_u64().is_some() || fixture_section_len(&fixture, suffix) == 0, "section {suffix} publishes its extent: {closed}");
    }

    assert!(!json.contains(".more"), "no continuation row survives anywhere: {json}");
    assert!(!json.contains("\"+"), "and no `+N` label either: {json}");
}

fn fixture_section_len(fixture: &Puzzle3dFixture, suffix: &str) -> usize {
    match suffix {
        "references" => fixture.references.len(),
        "target-volumes" => fixture.target_volumes.len(),
        _ => fixture.attractions.len(),
    }
}

/// 🪟️ A closed container costs one stamped `total` and nothing else — that is what makes a document
/// with thousands of entities cheap to paint, and what the host's expand arrow reads.
#[test]
fn a_closed_container_stamps_its_total_and_materialises_no_child() {
    let fixture = scaled_fixture(40, 3);
    let view = hosted(vec![request(&section_key("objects"), Some(false), 0, 64)]);
    let tree = tree_of(&panel(&fixture, &view));
    let section = node_at(&tree, &section_key("objects")).expect("the objects section");
    assert_eq!(window_of(section), (40, 0), "a closed section still reports everything it owns: {section}");
    assert!(child_keys(section).is_empty(), "and materialises none of it: {section}");

    // 🧾️ An object row is closed by AUTHOR default, so its vortices cost one stamp and no row at all.
    let open = hosted(vec![request(&section_key("objects"), Some(true), 0, 4)]);
    let tree = tree_of(&panel(&fixture, &open));
    let row = node_at(&tree, "object-0").expect("the first object row");
    assert_eq!(window_of(row), (3, 0), "a folded object row still reports its vortices: {row}");
    assert!(child_keys(row).is_empty(), "and builds none of them: {row}");
}

/// 🪟️ Scrolling: the host names the container and the slice, and exactly that slice is materialised —
/// keyed by the raw entity id, never renumbered by the offset.
#[test]
fn a_tree_window_request_materialises_exactly_its_slice() {
    let fixture = scaled_fixture(200, 4);
    let view = hosted(vec![request(&section_key("objects"), Some(true), 120, 6)]);
    let tree = tree_of(&panel(&fixture, &view));
    let section = node_at(&tree, &section_key("objects")).expect("the objects section");
    assert_eq!(window_of(section), (200, 120), "the section reports all 200 objects and that the slice starts at 120: {section}");
    let expected: Vec<String> = (120..126).map(|index| format!("object-{index}")).collect();
    assert_eq!(child_keys(section), expected, "exactly entries [120, 126) are built: {section}");

    // 🪺️ A nested group scrolls the same way, independently of its parent.
    let nested = hosted(vec![request(&section_key("objects"), Some(true), 0, 2), request("object-1", Some(true), 2, 2)]);
    let tree = tree_of(&panel(&fixture, &nested));
    let row = node_at(&tree, "object-1").expect("the second object row");
    assert_eq!(window_of(row), (4, 2), "the object reports all four vortices and the slice start: {row}");
    assert_eq!(child_keys(row), vec!["object-1:vortex-2".to_string(), "object-1:vortex-3".to_string()], "exactly entries [2, 4) are built: {row}");
}

/// 🕹️ A pick row carries NO argument map: it declares its `granularity`, and the ONE
/// `interactionSelect` the tree root binds turns a click on the row's raw id into a pick. That is the
/// whole argument-arena law — a document of any size costs the arena one binding, where 180 per-row
/// argument maps refused at the eleventh object (wave B44 §6.2, `outliner entityRows=0`).
#[test]
fn every_row_is_a_domain_pick_target_and_only_the_tree_binds_the_pick() {
    let mut fixture = scaled_fixture(2, 2);
    fixture.references.push(crate::editor::puzzle3d::Puzzle3dReference {
        id: "reference-1".into(),
        source: crate::editor::puzzle3d::Puzzle3dReferenceSource { url: "/reference/plan.png".into(), media_kind: Some("image".into()) },
        ..Default::default()
    });
    fixture.target_volumes.push(Puzzle3dTargetVolume { id: "volume-1".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, hidden: false, locked: false });
    fixture.attractions.push(Puzzle3dAttraction { id: "attraction-1".into(), attracting: "object-0".into(), attracted: "object-1".into(), ..Default::default() });
    let view = hosted(vec![
        request(&section_key("objects"), Some(true), 0, 8),
        request("object-0", Some(true), 0, 8),
        request(&section_key("references"), Some(true), 0, 8),
        request(&section_key("target-volumes"), Some(true), 0, 8),
        request(&section_key("attractions"), Some(true), 0, 8),
    ]);
    let json = panel(&fixture, &view);
    let tree = tree_of(&json);

    let root_bindings = tree["bindings"].to_string();
    assert!(root_bindings.contains(semio_framework_plugin::INTERACTION_SELECT_ACTION_ID), "the tree itself binds the pick: {root_bindings}");
    assert_eq!(root_bindings.matches(semio_framework_plugin::INTERACTION_SELECT_ACTION_ID).count(), 1, "exactly ONE tree-level pick binding: {root_bindings}");
    assert!(json.contains(PUZZLE3D_INTERACTION_DOMAIN), "and it names this app's interaction domain: {json}");

    for (key, granularity) in [
        ("object-0", PUZZLE3D_GRANULARITY_OBJECT),
        ("object-0:vortex-0", PUZZLE3D_GRANULARITY_VORTEX),
        ("reference-1", PUZZLE3D_GRANULARITY_REFERENCE),
        ("volume-1", PUZZLE3D_GRANULARITY_TARGET_VOLUME),
        ("attraction-1", PUZZLE3D_GRANULARITY_ATTRACTION),
    ] {
        let row = node_at(&tree, key).unwrap_or_else(|| panic!("row {key} exists: {tree}"));
        assert_eq!(row["component"]["granularity"].as_str(), Some(granularity), "row {key} declares the granularity it picks at: {row}");
        assert!(row["bindings"].as_array().is_none_or(|bindings| bindings.is_empty()), "a pick row authors no activation binding of its own: {row}");
    }

    // 🙈️ Row actions are NOT pick bindings — hide/lock survive the migration on every row that had them.
    for key in ["object-0", "reference-1", "volume-1"] {
        let row = node_at(&tree, key).expect("the flagged row");
        assert_eq!(row["component"]["rowActions"].as_array().map(Vec::len), Some(2), "row {key} keeps its hide/lock row actions: {row}");
    }
}

/// 🏠️ The first paint has no host state at all: the author defaults decide what is open and the shared
/// viewport budget decides how much of it is built, so a flagship document still paints about one
/// viewport of rows and stops — without ever truncating what the sections REPORT.
#[test]
fn the_first_paint_draws_one_viewport_and_still_reports_the_whole_document() {
    let fixture = nakagin_fixture();
    let json = first_paint(&fixture);
    let tree = tree_of(&json);
    let section = node_at(&tree, &section_key("objects")).expect("the objects section");
    assert_eq!(window_of(section).0, fixture.objects.len(), "the cold section still reports every object: {section}");
    let built = child_keys(section).len();
    assert!(built > 0, "the objects section opens by author default and must build rows: {section}");
    assert!(built <= semio_framework_plugin::TREE_WINDOW_DEFAULT_ROWS as usize, "the cold paint stays inside one viewport budget, built {built}");
    for suffix in ["references", "target-volumes", "attractions"] {
        let folded = node_at(&tree, &section_key(suffix)).expect("every section is assembled");
        assert!(child_keys(folded).is_empty(), "section {suffix} is folded by author default and builds nothing: {folded}");
    }
    assert!(!json.contains(".more"), "the cold paint invents no continuation row: {json}");
    assert!(!json.contains("\"+"), "and no `+N` label: {json}");
}
//#endregion 🪟️WindowLaws

/// 🔁️ One row action's `setSelectionFlag` args, flattened to `(flag, value)` — the two entries the
/// reducer reads (`🎮️commands/🔖️set-selection-flag/🦀️.rs`).
fn flag_binding(row_action: &ui::RowAction) -> (String, bool) {
    let Some(UiValue::Map(map)) = row_action.action.args.as_ref() else {
        panic!("a hide/lock row action must carry setSelectionFlag args");
    };
    let (mut flag, mut value) = (String::new(), None);
    let mut cursor = map.iter();
    while let Some((key, entry)) = cursor.advance() {
        match (key.as_str(), entry) {
            ("flag", UiValue::Text(text)) => flag = text.as_str().to_string(),
            ("value", UiValue::Bool(bit)) => value = Some(*bit),
            _ => {}
        }
    }
    (flag, value.expect("a hide/lock row action must carry an explicit value"))
}

/// 🔁️ Every hide/lock row action of one built tree, keyed by its row.
fn flag_bindings(node: &BuiltNode, rows: &mut Vec<(String, String, bool)>) {
    if let ui::Component::TreeItem(props) = &node.component {
        for row_action in props.row_actions.iter() {
            let (flag, value) = flag_binding(row_action);
            rows.push((node.key.as_str().to_string(), flag, value));
        }
    }
    for child in node.children.iter() {
        flag_bindings(child, rows);
    }
}

fn opened_everywhere() -> ViewModel {
    hosted(vec![
        request(&section_key("objects"), Some(true), 0, 8),
        request(&section_key("references"), Some(true), 0, 8),
        request(&section_key("target-volumes"), Some(true), 0, 8),
    ])
}

/// 🙈️ The outliner's inline hide/lock toggles must ASK FOR THE INVERSE of the row's current flag —
/// `📓️2026-09-09-user-feature-checklist.md` §17/summary #8: `flag_args` hardcoded `value: true`, so
/// "Show"/"Unlock" re-sent the state the row was already in and an outliner-hidden object could never
/// be un-hidden from the row that hid it. Asserted for all three row kinds that carry the toggles
/// (object / reference / target volume) in both states, so a regression in any one of them fails.
#[test]
fn outliner_hide_and_lock_rows_dispatch_the_inverse_of_the_current_flag() {
    let view = opened_everywhere();
    for flagged in [false, true] {
        drain_retired_ui_owners();
        let mut fixture = empty_fixture();
        fixture.objects.push(Puzzle3dObject {
            id: "object-1".into(),
            label: None,
            object_kind: Some("Object".into()),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: Vec::new(),
            hidden: flagged,
            locked: flagged,
        });
        fixture.references.push(crate::editor::puzzle3d::Puzzle3dReference {
            id: "reference-1".into(),
            source: crate::editor::puzzle3d::Puzzle3dReferenceSource { url: "/reference/plan.png".into(), media_kind: Some("image".into()) },
            hidden: flagged,
            locked: flagged,
            ..Default::default()
        });
        fixture.target_volumes.push(Puzzle3dTargetVolume { id: "volume-1".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, hidden: flagged, locked: flagged });
        let page = render(&fixture, native(), &TreeWindows::for_body(&view, BODY_KEY)).expect("a three-row outliner tree must be admitted");
        let mut rows = Vec::new();
        flag_bindings(&page, &mut rows);
        assert_eq!(rows.len(), 6, "one hide plus one lock row action per object/reference/target-volume row: {rows:?}");
        for (key, flag, value) in &rows {
            assert_eq!(*value, !flagged, "row {key}'s {flag} toggle must ask for {} while the row is {flagged}", !flagged);
        }
        for expected in ["object-1", "reference-1", "volume-1"] {
            assert_eq!(rows.iter().filter(|(key, _, _)| key == expected).count(), 2, "row {expected} lost a hide/lock action: {rows:?}");
        }
        drop(page);
    }
    drain_retired_ui_owners();
}

/// 🔁️ The checklist's own QA, closed as a loop: hide an object through the outliner's inline row
/// action, re-render, then UN-hide it through the SAME row — for `hidden` and for `locked`. The
/// sibling law above pins one render's args; this one pins that feeding those args to the reducer the
/// row names (`setSelectionFlag`'s explicit `{entity, ids}` path, i.e.
/// [`apply_puzzle3d_selection_flag`]) and re-rendering yields the OPPOSITE request, so the second
/// click undoes the first. With the old hardcoded `value: true` the row asked for `true` on both
/// passes and the object could never come back.
///
/// [`apply_puzzle3d_selection_flag`]: crate::editor::puzzle3d::apply_puzzle3d_selection_flag
#[test]
fn an_outliner_flag_row_undoes_itself_on_the_second_click() {
    let view = opened_everywhere();
    let mut fixture = empty_fixture();
    fixture.objects.push(Puzzle3dObject {
        id: "object-1".into(),
        label: None,
        object_kind: Some("Object".into()),
        origin: [0.0, 0.0, 0.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
    });
    let requested = |fixture: &Puzzle3dFixture, flag: &str| {
        drain_retired_ui_owners();
        let page = render(fixture, native(), &TreeWindows::for_body(&view, BODY_KEY)).expect("a one-row outliner tree must be admitted");
        let mut rows = Vec::new();
        flag_bindings(&page, &mut rows);
        let value = rows.iter().find(|(key, rendered, _)| key == "object-1" && rendered == flag).map(|(_, _, value)| *value).unwrap_or_else(|| panic!("the outliner row must offer a {flag} toggle: {rows:?}"));
        drop(page);
        value
    };
    let state = |fixture: &Puzzle3dFixture, flag: &str| {
        let object = fixture.objects.first().expect("the one object survives every flag write");
        if flag == "locked" {
            object.locked
        } else {
            object.hidden
        }
    };
    for flag in ["hidden", "locked"] {
        for expected in [true, false] {
            let asked = requested(&fixture, flag);
            assert_eq!(asked, expected, "the outliner's {flag} row must ask for {expected} while the object is {}", !expected);
            crate::editor::puzzle3d::apply_puzzle3d_selection_flag(&mut fixture, "object", &["object-1".to_string()], flag, asked);
            assert_eq!(state(&fixture, flag), expected, "clicking the outliner's own {flag} row must reach {expected}");
        }
    }
    drain_retired_ui_owners();
}
