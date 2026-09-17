use super::*;
use crate::editor::puzzle5d::unit_tests::context::*;

#[test]
fn document_tree_lists_the_seeded_parts_section() {
    let mut app = app();
    assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-document.parts"));
}

//#region 🪟️WindowLaws
const RETIREMENT_DRAIN_STEPS: usize = 4096;
/// 🏗️ Past `TREE_WINDOW_DEFAULT_ROWS` and past `UI_BUILT_CHILDREN_MAX` in every container, so each
/// law measures a document that cannot fit one paint.
const SCALE_PARTS: usize = 200;
const SCALE_GRIPS: usize = 160;
const SCALE_FASTENERS: usize = 150;

/// ♻️ Stands in for the reactor's own one-page-per-turn retirement pump: only closing the released
/// child backings and `UiValue` pages returns the process-wide credit the next build needs.
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

fn labels() -> &'static Puzzle5dLabels {
    crate::editor::puzzle5d::terminology::puzzle5d_labels(&semio_framework_plugin::ViewModel::default()).expect("an admitted host label axis")
}

/// 🏗️ A synthetic document of `parts` parts carrying `grips` grips each, plus `fasteners` fasteners.
fn scaled_scene(parts: usize, grips: usize, fasteners: usize) -> Puzzle5dScene {
    let mut document = crate::editor::puzzle5d::empty_document();
    document.parts = (0..parts)
        .map(|part| Puzzle5dPart {
            id: format!("part-{part}"),
            part_kind: "capsule".into(),
            anchor: Default::default(),
            part_2d: Default::default(),
            part_3d: Default::default(),
            grips: (0..grips).map(|grip| Puzzle5dGrip { id: format!("grip-{grip}"), grip_kind: "socket".into(), grip_2d: Default::default(), grip_3d: Default::default() }).collect(),
        })
        .collect();
    document.fasteners = (0..fasteners)
        .map(|index| Puzzle5dFastener {
            id: format!("fastener-{index}"),
            source: puzzle5d_grip_full_id(&format!("part-{index}"), "grip-0"),
            target: puzzle5d_grip_full_id(&format!("part-{}", index + 1), "grip-0"),
            fastener_kind: None,
            gap: 0.0,
            shift: 0.0,
            rise: 0.0,
            rotation: 0.0,
            turn: 0.0,
            tilt: 0.0,
            x: 0.0,
            y: 0.0,
        })
        .collect();
    Puzzle5dScene { document, runtime: crate::editor::puzzle5d::config::Puzzle5dRuntime::default(), active_utility: String::new(), interaction: Default::default() }
}

fn window_of(node: &BuiltNode) -> Option<ui::TreeWindow> {
    match &node.component {
        ui::Component::TreeSection(props) => props.window,
        ui::Component::TreeItem(props) => props.window,
        _ => None,
    }
}

fn granularity_of(node: &BuiltNode) -> Option<String> {
    match &node.component {
        ui::Component::TreeItem(props) => props.granularity.as_ref().map(|value| value.as_str().to_string()),
        _ => None,
    }
}

/// 🧾️ The body as JSON. A built tree's children travel as retained pages, so the projection helper —
/// not `serde_json` on the root — is what walks and retires them.
fn body_json(tree: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).expect("retire the rendered body")
}

fn child_of<'a>(node: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    node.children.iter().find(|child| child.key.as_str() == key).unwrap_or_else(|| panic!("container {key} must exist"))
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> semio_framework_plugin::TreeWindowRequest {
    semio_framework_plugin::TreeWindowRequest { body_key: BODY_KEY.into(), node_key: node_key.into(), open, offset, rows }
}

fn windows_for(requests: Vec<semio_framework_plugin::TreeWindowRequest>) -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel { tree_windows: requests, ..Default::default() }
}

/// 🪟️ (a) An oversized document: every container stamps its FULL extent and materialises no more
/// than its slice, and the body carries neither a `.more` key nor a `+N` label.
#[test]
fn the_outliner_stamps_every_container_and_never_pages() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_PARTS, SCALE_GRIPS, SCALE_FASTENERS);
    let windows = TreeWindows::unhosted();
    let tree = render(&scene, labels(), &windows).expect("an oversized outliner must be admitted");
    let parts = child_of(&tree, PARTS_SECTION);
    let fasteners = child_of(&tree, FASTENERS_SECTION);
    assert_eq!(window_of(parts).expect("the parts section must stamp its window").total as usize, SCALE_PARTS);
    assert_eq!(window_of(fasteners).expect("the fasteners section must stamp its window").total as usize, SCALE_FASTENERS);
    assert!(!parts.children.is_empty() && parts.children.len() <= semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX);
    for part in parts.children.iter() {
        assert_eq!(window_of(part).expect("every part row stamps its grip window").total as usize, SCALE_GRIPS);
        assert_eq!(part.children.len(), 0, "a closed part row materialises no grip");
    }
    let body = body_json(tree);
    assert!(!body.contains(".more"), "a windowed body carries no continuation key");
    assert!(!body.contains("\"+"), "a windowed body carries no `+N` label");
    drain_retired_ui_owners();
}

/// 🪟️ (b) A closed container stamps its total and materialises nothing — both the author default
/// (`fasteners`, every part's grip list) and an explicit host close (`parts`).
#[test]
fn a_closed_outliner_container_stamps_its_total_and_builds_no_row() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_PARTS, SCALE_GRIPS, SCALE_FASTENERS);
    let view = windows_for(vec![request(PARTS_SECTION, Some(false), 0, 32)]);
    let windows = TreeWindows::for_body(&view, BODY_KEY);
    let tree = render(&scene, labels(), &windows).expect("a closed outliner must be admitted");
    for (section, total) in [(PARTS_SECTION, SCALE_PARTS), (FASTENERS_SECTION, SCALE_FASTENERS)] {
        let container = child_of(&tree, section);
        assert_eq!(window_of(container).expect("a closed container still stamps its window").total as usize, total);
        assert_eq!(container.children.len(), 0, "a closed container materialises no child");
    }
    drop(tree);
    drain_retired_ui_owners();
}

/// 🪟️ (c) A host window materialises exactly `[offset, offset + rows)`, keyed by raw entity id —
/// for a section and for a nested grip list alike.
#[test]
fn a_host_window_materialises_exactly_its_slice_keyed_by_raw_id() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_PARTS, SCALE_GRIPS, SCALE_FASTENERS);
    let (offset, rows) = (50u32, 6u32);
    let view = windows_for(vec![request(PARTS_SECTION, Some(true), offset, rows), request("part-52", Some(true), 10, 4), request(FASTENERS_SECTION, Some(true), 3, 2)]);
    let windows = TreeWindows::for_body(&view, BODY_KEY);
    let tree = render(&scene, labels(), &windows).expect("a windowed outliner must be admitted");
    let parts = child_of(&tree, PARTS_SECTION);
    assert_eq!(window_of(parts).expect("stamped window").offset, offset);
    let keys: Vec<String> = parts.children.iter().map(|child| child.key.as_str().to_string()).collect();
    assert_eq!(keys, (offset..offset + rows).map(|index| format!("part-{index}")).collect::<Vec<_>>());
    let opened = child_of(parts, "part-52");
    assert_eq!(window_of(opened).expect("stamped grip window").offset, 10);
    let grips: Vec<String> = opened.children.iter().map(|child| child.key.as_str().to_string()).collect();
    assert_eq!(grips, (10..14).map(|index| puzzle5d_grip_full_id("part-52", &format!("grip-{index}"))).collect::<Vec<_>>());
    let fasteners = child_of(&tree, FASTENERS_SECTION);
    assert_eq!(fasteners.children.iter().map(|child| child.key.as_str().to_string()).collect::<Vec<_>>(), vec!["fastener-3".to_string(), "fastener-4".to_string()]);
    drop(tree);
    drain_retired_ui_owners();
}

/// 🎯️ (d) Pick rows carry a granularity and no binding of their own; the tree root carries exactly
/// one `interactionSelect` binding for the whole domain.
#[test]
fn outliner_pick_rows_are_domain_bound_without_a_per_row_binding() {
    drain_retired_ui_owners();
    let scene = scaled_scene(8, 4, 3);
    let view = windows_for(vec![request(PARTS_SECTION, Some(true), 0, 8), request("part-0", Some(true), 0, 4), request(FASTENERS_SECTION, Some(true), 0, 3)]);
    let windows = TreeWindows::for_body(&view, BODY_KEY);
    let tree = render(&scene, labels(), &windows).expect("a domain-bound outliner must be admitted");
    assert_eq!(tree.bindings.len(), 1, "the tree root carries exactly one interactionSelect binding");
    assert_eq!(tree.bindings.iter().next().expect("root binding").action.name.as_str(), semio_framework_plugin::INTERACTION_SELECT_ACTION_ID);
    let parts = child_of(&tree, PARTS_SECTION);
    for part in parts.children.iter() {
        assert_eq!(granularity_of(part).as_deref(), Some(PUZZLE5D_GRANULARITY_PART));
        assert_eq!(part.bindings.len(), 0, "part row {} must carry no per-row binding", part.key.as_str());
        for grip in part.children.iter() {
            assert_eq!(granularity_of(grip).as_deref(), Some(PUZZLE5D_GRANULARITY_GRIP));
            assert_eq!(grip.bindings.len(), 0);
        }
    }
    for fastener in child_of(&tree, FASTENERS_SECTION).children.iter() {
        assert_eq!(granularity_of(fastener).as_deref(), Some(PUZZLE5D_GRANULARITY_FASTENER));
        assert_eq!(fastener.bindings.len(), 0);
    }
    drop(tree);
    drain_retired_ui_owners();
}

/// 🧾️ Every node record one body spends — the tree root, every section, every part group and every
/// leaf row counts once, exactly as `SurfaceReconcileLimits::max_nodes` counts them.
fn node_records(node: &BuiltNode) -> usize {
    1 + node.children.iter().map(node_records).sum::<usize>()
}

/// 🔑️ Every node key in one body. A windowed container is addressed by its authored key, so two
/// containers (or two rows) sharing one key inside a body make the host's `TreeWindowRequest`
/// ambiguous — the SDK refuses it.
fn node_keys(node: &BuiltNode, keys: &mut Vec<String>) {
    keys.push(node.key.as_str().to_string());
    for child in node.children.iter() {
        node_keys(child, keys);
    }
}

/// 🧾️ The whole-document law: one container holds more entries than a whole `UI_DOCUMENT_NODES` arena
/// AND every container — both sections and two dozen part groups — is open at once, each asking for
/// more rows than the arena could hold. Per-container clamps alone do not save this body: only the
/// SDK's body-wide node ledger does. Every container must still stamp its FULL total, the body must
/// reconcile inside `UI_DOCUMENT_NODES` records, and nothing may be closed off with a `+N`.
#[test]
fn a_whole_open_document_stamps_every_total_and_stays_inside_the_body_node_ceiling() {
    drain_retired_ui_owners();
    let (parts_len, grips_len, fasteners_len) = (ui::UI_DOCUMENT_NODES + 72, 24, ui::UI_DOCUMENT_NODES + 16);
    let scene = scaled_scene(parts_len, grips_len, fasteners_len);
    let mut requests = vec![request(PARTS_SECTION, Some(true), 0, 512), request(FASTENERS_SECTION, Some(true), 0, 512)];
    // 🔑️ A nested container is addressed by its window PATH: the parts section, then the part row.
    requests.extend((0..24).map(|index| request(&format!("{PARTS_SECTION}{}part-{index}", ui::TREE_WINDOW_PATH_SEPARATOR), Some(true), 0, 512)));
    let view = windows_for(requests);
    let windows = TreeWindows::for_body(&view, BODY_KEY);
    let tree = render(&scene, labels(), &windows).expect("a fully open oversized outliner must be admitted");
    for (section, total) in [(PARTS_SECTION, parts_len), (FASTENERS_SECTION, fasteners_len)] {
        let container = child_of(&tree, section);
        assert_eq!(window_of(container).expect("every container stamps its window").total as usize, total, "container {section} must stamp its FULL total however few rows it could afford");
    }
    for part in child_of(&tree, PARTS_SECTION).children.iter() {
        assert_eq!(window_of(part).expect("an open part group stamps its own window").total as usize, grips_len, "part group {} must stamp its full grip total", part.key.as_str());
    }
    let records = node_records(&tree);

    // 🔑️ One body, one key per node: a duplicate would make a host window request ambiguous.
    let mut keys = Vec::new();
    node_keys(&tree, &mut keys);
    let mut unique = keys.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), keys.len(), "every node key in this body is unique ({} of {} distinct)", unique.len(), keys.len());
    assert!(records <= ui::UI_DOCUMENT_NODES, "the whole open document must reconcile inside one surface arena, spent {records} of {}", ui::UI_DOCUMENT_NODES);
    let body = body_json(tree);
    assert!(!body.contains(".more"), "no continuation row closes an exhausted container: {body:.400}");
    assert!(!body.contains("\"+"), "and no `+N` label either: {body:.400}");
    drain_retired_ui_owners();
}
//#endregion 🪟️WindowLaws

//#region 🏷️LabelAndFlagLaws
/// 🏷️ A part row reads its AUTHORED label, not its kind id, and duplicate kinds get numbered roots.
#[test]
fn part_rows_read_authored_labels_and_numbered_peers() {
    let mut document = crate::editor::puzzle5d::empty_document();
    document.parts = vec![
        Puzzle5dPart { id: "a".into(), part_kind: "capsule".into(), anchor: Default::default(), part_2d: Default::default(), part_3d: crate::editor::puzzle5d::Puzzle5dPart3d { label: Some("Capsule".into()), ..Default::default() }, grips: Vec::new() },
        Puzzle5dPart { id: "b".into(), part_kind: "capsule".into(), anchor: Default::default(), part_2d: Default::default(), part_3d: crate::editor::puzzle5d::Puzzle5dPart3d { label: Some("Capsule 2".into()), ..Default::default() }, grips: Vec::new() },
        Puzzle5dPart { id: "c".into(), part_kind: "capsule".into(), anchor: Default::default(), part_2d: Default::default(), part_3d: Default::default(), grips: Vec::new() },
    ];
    assert_eq!(puzzle5d_part_display_label(&document.parts[0], &document), "Capsule");
    assert_eq!(puzzle5d_part_display_label(&document.parts[2], &document), "capsule", "an unlabelled part falls back to its kind's catalogue name");
    assert_eq!(crate::editor::puzzle5d::puzzle5d_next_part_label(&document.parts, &document, "capsule"), "Capsule 3", "the highest numbered peer is 2, so the next one is 3");
    assert_eq!(crate::editor::puzzle5d::puzzle5d_next_part_label(&document.parts, &document, "core"), "core", "the first instance of a kind takes the bare catalogue name");
}

/// 🙈️ Every part row carries exactly the two hide/lock toggles, each asking for the INVERSE of the
/// state the row is in — the hardcoded-`true` defect puzzle 3d carried would re-apply the same state.
#[test]
fn part_rows_carry_inverting_hide_and_lock_toggles() {
    drain_retired_ui_owners();
    let mut scene = scaled_scene(2, 0, 0);
    scene.document.parts[0].part_2d.hidden = Some(true);
    scene.document.parts[0].part_2d.locked = Some(true);
    let view = windows_for(vec![request(PARTS_SECTION, Some(true), 0, 2)]);
    let tree = render(&scene, labels(), &TreeWindows::for_body(&view, BODY_KEY)).expect("an outliner with row actions must be admitted");
    let parts = child_of(&tree, PARTS_SECTION);
    for row in parts.children.iter() {
        let actions = match &row.component {
            ui::Component::TreeItem(props) => props.row_actions.len(),
            _ => panic!("a part row is a tree item"),
        };
        assert_eq!(actions, 2, "part row {} carries show/hide and lock/unlock", row.key.as_str());
        assert_eq!(row.bindings.len(), 0, "row actions are not bindings; the pick binding stays on the root");
    }
    let body = body_json(tree);
    assert!(body.contains("\"value\":false") || body.contains("\"value\": false"), "the already-hidden row must ask for `hidden:false`: {body:.900}");
    drain_retired_ui_owners();
}
//#endregion 🏷️LabelAndFlagLaws
