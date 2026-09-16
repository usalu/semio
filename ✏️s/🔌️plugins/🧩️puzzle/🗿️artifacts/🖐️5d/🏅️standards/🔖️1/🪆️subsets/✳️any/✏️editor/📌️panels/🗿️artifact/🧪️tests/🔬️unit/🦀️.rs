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
    Puzzle5dScene { document, runtime: crate::editor::puzzle5d::config::Puzzle5dRuntime::default(), active_utility: String::new() }
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
    let windows = semio_framework_plugin::TreeWindows::unhosted();
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
    let body = serde_json::to_string(&tree).expect("serialize the outliner body");
    assert!(!body.contains(".more"), "a windowed body carries no continuation key");
    assert!(!body.contains("\"+"), "a windowed body carries no `+N` label");
    drop(tree);
    drain_retired_ui_owners();
}

/// 🪟️ (b) A closed container stamps its total and materialises nothing — both the author default
/// (`fasteners`, every part's grip list) and an explicit host close (`parts`).
#[test]
fn a_closed_outliner_container_stamps_its_total_and_builds_no_row() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_PARTS, SCALE_GRIPS, SCALE_FASTENERS);
    let view = windows_for(vec![request(PARTS_SECTION, Some(false), 0, 32)]);
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, BODY_KEY);
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
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, BODY_KEY);
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
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, BODY_KEY);
    let tree = render(&scene, labels(), &windows).expect("a domain-bound outliner must be admitted");
    assert_eq!(tree.bindings.len(), 1, "the tree root carries exactly one interactionSelect binding");
    assert_eq!(tree.bindings.iter().next().expect("root binding").action.as_str(), semio_framework_plugin::INTERACTION_SELECT_ACTION_ID);
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
//#endregion 🪟️WindowLaws
