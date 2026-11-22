//! 🌳️ LAW: a mounted `Tree` answers the WAI-ARIA tree keyboard and the modifier selection React's `Tree` does, keeps one roving tab stop,
//! never mirrors (React forces `inline="ltr"`), and paints its section header and indent guides the way React's `TreeSection` and
//! `TreeDocumentGutter` do.
//!
//! Oracle: `🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧫️fixtures/⌨️roving/🔣️.json`, the neutral vectors `🌳️Tree/⌨️roving/🦀️.rs` is also measured
//! against; this file replays the single-select key cases through the real router and arena.

use super::*;
use semio_ui_declaration::layout::ActionDescriptor;
use semio_ui_declaration::ui::{UiPresence, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use semio_ui_locale::Label;
use semio_ui_theme::geometry::Rect;
use semio_ui_wgpu_events::{EventModifiers, PointerButton};
use semio_ui_wgpu_hosting::scene_slots::{SceneHost, ScenePaintCursor, ScenePaintStep, SceneSlot};
use semio_framework_test_fixtures::fixture_text;
use serde_json::Value;

struct EmptySceneHost;

impl SceneHost for EmptySceneHost {
    fn paint_slot_step(&mut self, _slot: &SceneSlot<'_>, _cursor: &mut ScenePaintCursor, _draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) -> ScenePaintStep {
        ScenePaintStep::Fault
    }
}

fn fixture() -> Value {
    serde_json::from_str(fixture_text!("../../../../../../../🧱️elements/🌳️Tree/🧪️tests/🧫️fixtures/⌨️roving/🔣️.json")).expect("the neutral tree-roving vectors parse")
}

fn activation(id: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: "law".into(), action: format!("activate-{id}"), args: None }
}

fn item(value: &Value, domain: bool) -> UiTreeItemNode {
    let id = value["id"].as_str().expect("id");
    let mut node = UiTreeItemNode::base(id, Label::data(value["label"].as_str().expect("label")));
    node.action = Some(activation(id));
    if domain {
        node.granularity = Some("piece".into());
    }
    if let Some(children) = value["items"].as_array() {
        node.items = Some(children.iter().map(|child| item(child, domain)).collect());
        node.default_open = Some(value["expanded"].as_bool().unwrap_or(false));
    }
    node
}

fn tree_node(items: Vec<UiTreeItemNode>, label: &str, default_open: bool, domain: bool) -> UiNode {
    let tree = UiNode::Tree(UiTreeNode {
        presentation: Default::default(),
        sections: vec![UiTreeSectionNode { header_toolbar: None, id: "s".into(), label: Some(Label::data(label)), default_open: Some(default_open), presence: UiPresence::default(), items, window: None }],
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: domain.then(|| "outliner.objects".to_string()),
    });
    UiNode::Stack(UiStackNode { id: Some("panel".into()), direction: "vertical".into(), gap: None, padding: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None, children: vec![tree] })
}

fn fixture_tree(law: &Value, domain: bool) -> UiNode {
    tree_node(law["tree"].as_array().expect("tree").iter().map(|entry| item(entry, domain)).collect(), "Section", true, domain)
}

fn drive_layout(ui: &mut UiFixture, atlas: &mut FontAtlas, pool: &semio_framework_async::WorkerPool) {
    ui.set_viewport("fixture", 320.0, 240.0);
    assert!(ui.settle_layout("fixture", pool, atlas), "tree layout never settled");
}

fn drive_frame(ui: &mut Ui, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    for _ in 0..131_072 {
        match ui.frame_into_step::<EmptySceneHost>("fixture", Rect::new(0.0, 0.0, 320.0, 240.0), atlas, icons, None, draw, ui_contract::UI_WORKER_RETIREMENT_POLICY).0 {
            UiFrameStep::Pending => {}
            UiFrameStep::Ready => return,
            step => panic!("tree paint answered {step:?}"),
        }
    }
    panic!("tree paint never settled");
}

struct Mounted {
    ui: UiFixture,
    atlas: FontAtlas,
    pool: semio_framework_async::WorkerPool,
}

fn mount(node: &UiNode) -> Mounted {
    let mut ui = UiFixture::new(semio_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    ui.apply_tree("fixture", node);
    drive_layout(&mut ui, &mut atlas, &pool);
    Mounted { ui, atlas, pool }
}

fn row_node(tree: &UiTree, key: &str) -> NodeId {
    fn walk(tree: &UiTree, parent: NodeId, key: &str) -> Option<NodeId> {
        tree.children(parent).find_map(|child| if tree.authored_tree_item(child).is_some_and(|item| item.id == key) { Some(child) } else { walk(tree, child, key) })
    }
    walk(tree, tree.root.expect("root"), key).unwrap_or_else(|| panic!("row {key} is mounted"))
}

fn focused_key(ui: &Ui) -> Option<String> {
    let window = ui.windows.get("fixture").expect("window");
    window.router.focus.focused.and_then(|id| window.tree.authored_tree_item(id).map(|item| item.id.clone()))
}

fn expanded_keys(ui: &Ui, law: &Value) -> Vec<String> {
    fn walk(tree: &UiTree, items: &[Value], out: &mut Vec<String>) {
        for entry in items {
            let Some(children) = entry["items"].as_array() else { continue };
            let key = entry["id"].as_str().expect("id");
            if tree.disclosure_open(row_node(tree, key)) == Some(true) {
                out.push(key.to_owned());
            }
            walk(tree, children, out);
        }
    }
    let mut out = Vec::new();
    walk(&ui.windows.get("fixture").expect("window").tree, law["tree"].as_array().expect("tree"), &mut out);
    out.sort();
    out
}

fn key(ui: &mut Ui, name: &str, modifiers: EventModifiers) -> Vec<UiCommand> {
    ui.dispatch_event("fixture", UiEvent::KeyDown { key: name.to_owned(), modifiers })
}

fn activated(commands: &[UiCommand]) -> Vec<String> {
    commands.iter().filter_map(|command| match command {
        UiCommand::App { intent, .. } => intent.action_name().strip_prefix("activate-").map(str::to_owned),
        _ => None,
    }).collect()
}

fn held(names: &[Value]) -> EventModifiers {
    let has = |name: &str| names.iter().any(|entry| entry == name);
    EventModifiers { shift: has("shift"), ctrl: has("ctrl"), meta: has("meta"), alt: has("alt") }
}

fn focus_row(m: &mut Mounted, key: &str) {
    let window = m.ui.windows.get_mut("fixture").expect("window");
    let row = row_node(&window.tree, key);
    let _ = window.router.focus.set_focus(&mut window.tree, Some(row), true);
}

#[test]
fn the_shared_single_select_key_vectors_drive_the_real_router_and_arena() {
    let law = fixture();
    for case in law["cases"].as_array().expect("cases") {
        if case["multiple"].as_bool().expect("multiple") || case["steps"].as_array().expect("steps").iter().any(|step| step.get("click").is_some()) {
            continue;
        }
        let name = case["name"].as_str().expect("name");
        let mut m = mount(&fixture_tree(&law, false));
        focus_row(&mut m, case["focus"].as_str().expect("focus"));
        let mut clicked: Vec<String> = Vec::new();
        let mut clock = 0.0;
        for (index, step) in case["steps"].as_array().expect("steps").iter().enumerate() {
            if step["reset"].as_bool().unwrap_or(false) {
                clock += 5.0;
                let window = m.ui.windows.get_mut("fixture").expect("window");
                let _ = window.router.advance_clock(&mut window.tree, clock);
            }
            let modifiers = held(step["modifiers"].as_array().map(Vec::as_slice).unwrap_or(&[]));
            let commands = key(&mut m.ui, step["key"].as_str().expect("key"), modifiers);
            clicked.extend(activated(&commands));
            let expect = &step["expect"];
            let at = format!("{name} / step {index}");
            if let Some(focus) = expect["focus"].as_str() {
                assert_eq!(focused_key(&m.ui).as_deref(), Some(focus), "{at}: focus");
            }
            if expect.get("expanded").is_some() {
                let wanted: Vec<String> = expect["expanded"].as_array().expect("expanded").iter().map(|entry| entry.as_str().expect("id").to_owned()).collect();
                assert_eq!(expanded_keys(&m.ui, &law), wanted, "{at}: expanded");
            }
            if expect.get("clicked").is_some() {
                let wanted: Vec<String> = expect["clicked"].as_array().expect("clicked").iter().map(|entry| entry.as_str().expect("id").to_owned()).collect();
                assert_eq!(clicked, wanted, "{at}: the activations the keys emitted");
            }
        }
        let _ = &m.pool;
        let _ = &m.atlas;
    }
}

#[test]
fn a_tree_is_one_tab_stop_and_remembers_the_row_that_last_held_focus() {
    let law = fixture();
    let mut m = mount(&fixture_tree(&law, false));
    let first = key(&mut m.ui, "Tab", EventModifiers::default());
    assert!(matches!(first.last(), Some(UiCommand::FocusChanged { node: Some(_), .. })) && focused_key(&m.ui).is_none(), "the first Tab stop is the section header, a button of its own: {first:?}");
    let _ = key(&mut m.ui, "Tab", EventModifiers::default());
    assert_eq!(focused_key(&m.ui).as_deref(), Some("fruit"), "the next Tab lands on the tree's first row when nothing is selected or remembered");
    let _ = key(&mut m.ui, "ArrowDown", EventModifiers::default());
    let _ = key(&mut m.ui, "ArrowDown", EventModifiers::default());
    assert_eq!(focused_key(&m.ui).as_deref(), Some("banana"));
    let _ = key(&mut m.ui, "Tab", EventModifiers::default());
    let _ = key(&mut m.ui, "Tab", EventModifiers { shift: true, ..EventModifiers::default() });
    assert_eq!(focused_key(&m.ui).as_deref(), Some("banana"), "the one tab stop is the row that last held focus, not the first row");
}

#[test]
fn left_and_right_are_never_mirrored_in_a_right_to_left_window() {
    let law = fixture();
    let mut m = mount(&fixture_tree(&law, false));
    m.ui.set_window_flow("fixture", ui_contract::UiFlow { inline: ui_contract::FlowInline::Rtl, block: ui_contract::FlowBlock::Down });
    focus_row(&mut m, "fruit");
    let _ = key(&mut m.ui, "ArrowRight", EventModifiers::default());
    assert_eq!(focused_key(&m.ui).as_deref(), Some("apple"), "Right enters an open branch whatever the window direction: React forces its Tree to LTR");
}

fn click_row(m: &mut Mounted, control_id: &str, modifiers: EventModifiers) -> Vec<UiCommand> {
    let rect = m.ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == control_id).unwrap_or_else(|| panic!("hit {control_id}")).rect;
    let (x, y) = (rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    let mut commands = m.ui.dispatch_event("fixture", UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers });
    commands.extend(m.ui.dispatch_event("fixture", UiEvent::PointerUp { x, y, button: PointerButton::Primary, modifiers }));
    commands
}

#[test]
fn a_repeated_press_on_a_row_yields_to_the_double_click_and_does_not_fold_or_activate_again() {
    let law = fixture();
    let mut m = mount(&fixture_tree(&law, false));
    let mut draw = DrawList::default();
    drive_frame(&mut m.ui, &mut m.atlas, None, &mut draw);
    let first = click_row(&mut m, "tree.label.fruit", EventModifiers::default());
    assert_eq!(activated(&first), vec!["fruit".to_string()], "the first press activates once");
    assert!(expanded_keys(&m.ui, &law).is_empty(), "and folds the open branch");
    drive_layout(&mut m.ui, &mut m.atlas, &m.pool);
    drive_frame(&mut m.ui, &mut m.atlas, None, &mut draw);
    let second = click_row(&mut m, "tree.label.fruit", EventModifiers::default());
    assert!(activated(&second).is_empty(), "the second press of a double-click activates nothing: React's event.detail > 1 rule");
    assert!(expanded_keys(&m.ui, &law).is_empty(), "and does not unfold the row again");
}

fn domain_binding(surface: &str) -> semio_ui_action::UiIntentBindings {
    let hover = ui_contract::ActionId::try_v1("outliner", "interactionHover").expect("hover id");
    semio_ui_action::UiIntentBindings {
        address: semio_ui_action::UiIntentAddress { surface: surface.into(), revision: 0, node: 1, node_key: "row".into() },
        bindings: vec![(ui_contract::Trigger::HoverPreview, hover)],
        args: vec![(ui_contract::Trigger::HoverPreview, semio_framework_value::DslValue::Object(vec![("domain".into(), semio_framework_value::DslValue::String("outliner.objects".into()))]))],
    }
}

fn picks(commands: &[UiCommand]) -> Vec<(String, String)> {
    commands.iter().filter_map(|command| match command {
        UiCommand::App { intent, .. } if intent.action_name() == "interactionSelect" => {
            let semio_framework_value::DslValue::Object(fields) = intent.input.as_ref()? else { return None };
            let text = |name: &str| fields.iter().find_map(|(key, value)| (key == name).then(|| if let semio_framework_value::DslValue::String(text) = value { text.clone() } else { String::new() }));
            Some((text("merge")?, text("targets")?))
        }
        _ => None,
    }).collect()
}

#[test]
fn a_domain_bound_row_picks_with_shift_range_and_ctrl_toggle_through_interaction_select() {
    let flat: Vec<UiTreeItemNode> = ["a", "b", "c", "d"].iter().map(|id| {
        let mut row = UiTreeItemNode::base(*id, Label::data(*id));
        row.granularity = Some("piece".into());
        row
    }).collect();
    let mut m = mount(&tree_node(flat, "Pieces", true, true));
    for id in ["a", "b", "c", "d"] {
        let window = m.ui.windows.get_mut("fixture").expect("window");
        let row = row_node(&window.tree, id);
        window.tree.node_mut(row).expect("row").intent = Some(domain_binding("surface"));
    }
    let mut draw = DrawList::default();
    drive_frame(&mut m.ui, &mut m.atlas, None, &mut draw);
    let plain = click_row(&mut m, "tree.label.b", EventModifiers::default());
    assert_eq!(picks(&plain), vec![("replace".to_string(), r#"[{"granularity":"piece","id":"b"}]"#.to_string())], "a bare press replaces the selection");
    let ctrl = click_row(&mut m, "tree.label.d", EventModifiers { ctrl: true, ..EventModifiers::default() });
    assert_eq!(picks(&ctrl), vec![("invertive".to_string(), r#"[{"granularity":"piece","id":"d"}]"#.to_string())], "Ctrl inverts just the pressed row");
    let shift = click_row(&mut m, "tree.label.c", EventModifiers { shift: true, ..EventModifiers::default() });
    assert_eq!(picks(&shift), vec![("replace".to_string(), r#"[{"granularity":"piece","id":"c"},{"granularity":"piece","id":"d"}]"#.to_string())], "Shift ranges from the anchor to the pressed row");
    let keyed = {
        focus_row(&mut m, "a");
        key(&mut m.ui, "ArrowDown", EventModifiers { shift: true, ..EventModifiers::default() })
    };
    assert_eq!(focused_key(&m.ui).as_deref(), Some("b"), "Shift+Down moves focus");
    assert_eq!(picks(&keyed).len(), 1, "and extends the selection with one interactionSelect");
    let toggled = key(&mut m.ui, " ", EventModifiers { ctrl: true, ..EventModifiers::default() });
    assert_eq!(picks(&toggled), vec![("invertive".to_string(), r#"[{"granularity":"piece","id":"b"}]"#.to_string())], "Ctrl+Space toggles the focused row");
}

fn glyph_rects(draw: &DrawList) -> Vec<[f32; 4]> {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - semio_ui_draw_contract::KIND_GLYPH).abs() < 0.01).map(|instance| instance.uv_rect).collect()
}

fn icon_atlas() -> IconAtlas {
    IconAtlas::from_packed(
        4,
        1,
        vec![255; 16],
        vec![("chevron-down".into(), [0.0, 0.0, 0.25, 1.0]), ("chevron-right".into(), [0.25, 0.0, 0.25, 1.0]), ("folder".into(), [0.5, 0.0, 0.25, 1.0]), ("chevron-left".into(), [0.75, 0.0, 0.25, 1.0])],
    )
}

fn painted(node: &UiNode, icons: Option<&IconAtlas>) -> DrawList {
    let mut m = mount(node);
    let mut draw = DrawList::default();
    drive_frame(&mut m.ui, &mut m.atlas, icons, &mut draw);
    draw
}

fn icon_instances(draw: &DrawList, uv: [f32; 4]) -> Vec<[f32; 4]> {
    draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.uv_rect == uv).map(|instance| instance.rect).collect()
}

#[test]
fn a_section_header_paints_its_fold_chevron_and_an_uppercase_semibold_label() {
    let icons = icon_atlas();
    let child = || vec![UiTreeItemNode::base("child", Label::data("x"))];
    let open = painted(&tree_node(child(), "Layers", true, false), Some(&icons));
    let closed = painted(&tree_node(child(), "Layers", false, false), Some(&icons));
    let size = semio_ui_wgpu_paint::TREE_SECTION_CHEVRON_SIZE;
    let down = icon_instances(&open, [0.0, 0.0, 0.25, 1.0]);
    assert!(down.iter().any(|rect| (rect[2] - size).abs() < 0.01), "an open section header paints the chevron-down at React's size-small: {down:?}");
    let right = icon_instances(&closed, [0.25, 0.0, 0.25, 1.0]);
    assert!(right.iter().any(|rect| (rect[2] - size).abs() < 0.01), "a closed section header paints the chevron-right: {right:?}");
    assert!(!icon_instances(&closed, [0.0, 0.0, 0.25, 1.0]).iter().any(|rect| (rect[2] - size).abs() < 0.01), "and not the open one");
    assert!(!icon_instances(&painted(&tree_node(Vec::new(), "Layers", true, false), Some(&icons)), [0.0, 0.0, 0.25, 1.0]).iter().any(|rect| (rect[2] - size).abs() < 0.01), "an empty section has nothing to fold, so no chevron");

    let lower = painted(&tree_node(Vec::new(), "ab", true, false), None);
    let upper = painted(&tree_node(Vec::new(), "AB", true, false), None);
    assert_eq!(glyph_rects(&lower).len(), 4, "two letters, each struck twice: semibold is a double strike");
    assert_eq!(glyph_rects(&lower), glyph_rects(&upper), "the label paints uppercase whatever case it was authored in");
}

#[test]
fn a_row_paints_the_stem_and_elbow_guides_that_hang_it_from_its_parent_or_section() {
    let mut parent = UiTreeItemNode::base("parent", Label::data("Parent"));
    parent.default_open = Some(true);
    parent.items = Some(vec![UiTreeItemNode::base("first", Label::data("First")), UiTreeItemNode::base("last", Label::data("Last"))]);
    let nested = painted(&tree_node(vec![parent], "S", true, false), None);
    let flat = painted(&tree_node(vec![UiTreeItemNode::base("only", Label::data("Only"))], "S", true, false), None);
    let solids = |draw: &DrawList| -> Vec<[f32; 4]> { draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - semio_ui_draw_contract::KIND_SOLID).abs() < 0.01).map(|instance| instance.rect).collect() };
    let hair = Theme::default().stroke_hairline;
    let leaf_elbow = semio_ui_wgpu_paint::TREE_INDENT_PER_LEVEL;
    let branch_elbow = semio_ui_wgpu_paint::TREE_INDENT_PER_LEVEL - semio_ui_wgpu_paint::TREE_GUIDE_INSET;
    let elbows = |draw: &DrawList, width: f32| solids(draw).into_iter().filter(|rect| (rect[3] - hair).abs() < 0.001 && (rect[2] - width).abs() < 0.01).count();
    let stems = |draw: &DrawList| solids(draw).into_iter().filter(|rect| (rect[2] - hair).abs() < 0.001 && rect[3] > hair).count();
    assert_eq!(elbows(&nested, leaf_elbow), 2, "each of the two leaf children draws one elbow from the parent's stem to its own guide column");
    assert_eq!(elbows(&nested, branch_elbow), 1, "the open parent draws its elbow from the section stem to its fold chevron");
    assert!(stems(&nested) >= 4, "the section stem, the parent's branch stem and one parent guide per child");
    assert_eq!(elbows(&flat, leaf_elbow), 1, "a top-level row hangs from the section's stem: rows sit one level under their header");
}

fn catalogue_tree(payload: &str) -> UiNode {
    let mut source = UiTreeItemNode::base("source", Label::data("source"));
    source.draggable = Some(true);
    source.drag_data = Some([(semio_ui_wgpu_engine::TREE_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())].into_iter().collect());
    let mut target = UiTreeItemNode::base("target", Label::data("target"));
    target.action = Some(activation("target"));
    tree_node(vec![source, target], "Section", true, false)
}

fn drag_source_onto_target(m: &mut Mounted) -> Vec<UiCommand> {
    let mut draw = DrawList::default();
    drive_frame(&mut m.ui, &mut m.atlas, None, &mut draw);
    let centre = |m: &Mounted, id: &str| {
        let rect = m.ui.window_hit_targets("fixture").iter().find(|hit| hit.control_id == id).unwrap_or_else(|| panic!("hit {id}")).rect;
        (rect.x + rect.w * 0.5, rect.y + rect.h * 0.5)
    };
    let (from, to) = (centre(m, "tree.label.source"), centre(m, "tree.label.target"));
    let modifiers = EventModifiers::default();
    let mut commands = m.ui.dispatch_event("fixture", UiEvent::PointerDown { x: from.0, y: from.1, button: PointerButton::Primary, modifiers });
    commands.extend(m.ui.dispatch_event("fixture", UiEvent::PointerMove { x: from.0, y: from.1 + 5.0, modifiers }));
    commands.extend(m.ui.dispatch_event("fixture", UiEvent::PointerMove { x: to.0, y: to.1, modifiers }));
    commands.extend(m.ui.dispatch_event("fixture", UiEvent::PointerUp { x: to.0, y: to.1, button: PointerButton::Primary, modifiers }));
    commands
}

#[test]
fn a_catalogue_drop_on_an_activatable_row_of_a_tree_without_a_drop_binding_clicks_the_row_like_react() {
    let mut m = mount(&catalogue_tree("{\"id\":\"a\"}"));
    let commands = drag_source_onto_target(&mut m);
    assert_eq!(activated(&commands), vec!["target".to_string()], "the row's own activation fires once, as the synthetic onClick of catalogueTreeDragController.handleDrop");
    assert!(!commands.iter().any(|command| matches!(command, UiCommand::DropCommitted { .. } | UiCommand::DropCancelled { .. })), "a row click is not a committed drop");
    assert!(m.ui.windows.get("fixture").expect("window").router.focus.tree_anchor.is_empty(), "the fallback click records no range anchor: React's synthetic onClick never reaches handleSelectItem");
}

#[test]
fn a_catalogue_payload_that_is_not_json_emits_nothing_like_react_whose_json_parse_throws() {
    let mut m = mount(&catalogue_tree("{not json"));
    let commands = drag_source_onto_target(&mut m);
    assert!(activated(&commands).is_empty(), "an undecodable payload clicks nothing");
}
