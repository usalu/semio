/// 🛠️ Actions, utilities and tab actions: the actions and utilities app, the scoping window, the utility option partition, the painted tab controls, the maximize toggle, tab hits and label truncation.

/// 🧰️ Builds a two-window app: window `main` scopes `utility.a`, window `aux` scopes nothing; `utility.b`
/// is an orphan (no window references it). Actions: `zeroArg` (no args) + `withArgs` (required text +
/// defaulted toggle) scoped to `main`.
///
/// Scope utility.a + both actions to `main`; leave utility.b an orphan referenced by no window.
fn actions_utilities_app() -> AppDefinition {
    let mut app = sample_app(&["main", "aux"], None);
    app.controller_id = "ctrl".into();
    app.utilities = vec![UtilityDefinition::new("utility.a", LocalizedLabel::data("Utility A"), "circle"), UtilityDefinition { allows_actions_while_active: true, ..UtilityDefinition::new("utility.b", LocalizedLabel::data("Utility B"), "square") }];
    let actions = vec![
        ActionDefinition::bounded_catalog("zeroArg", LocalizedLabel::data("Zero Arg"), ActionKind::View),
        ActionDefinition {
            args: vec![ActionArgDef::text("name", LocalizedLabel::data("Name")).required(), ActionArgDef { default: Some(semio_framework_value::ToValue::to_value(&true)), ..ActionArgDef::toggle("flag", LocalizedLabel::data("Flag")) }],
            keys: Some("mod+e".into()),
            ..ActionDefinition::bounded_catalog("withArgs", LocalizedLabel::data("With Args"), ActionKind::View)
        },
    ];
    for kind in app.window_kinds.iter_mut() {
        if kind.id == "main" {
            kind.utilities = vec![UtilityRef::new("utility.a")];
            kind.actions = actions.clone();
        }
    }
    app
}

/// 📇️ The first window kind whose resolved actions include `action_id` — the window the palette/keybinding
/// redirect focuses to open an arg-carrying action's form (Architecture Decision 8, P3/P4).
fn action_host_window_id(app: &AppDefinition, action_id: &str) -> Option<String> {
    app.window_kinds.iter().find(|kind| semio_framework::resolve_window_actions(app, kind).iter().any(|action| action.id == action_id)).map(|kind| kind.id.clone())
}

#[test]
fn action_host_window_id_finds_scoping_window() {
    let app = actions_utilities_app();
    assert_eq!(action_host_window_id(&app, "withArgs").as_deref(), Some("main"));
}

/// 🎯️ The Utility Options rail (`render_utility_options_rail`) resolves its content through
/// `partition_window_measures`: a tagged group surfaces only for its matching active utility, and is
/// absent from BOTH buckets otherwise — untagged groups always stay in the general Measures rail.
#[test]
fn utility_options_partition_gates_tagged_group_by_active_utility() {
    use semio_framework_ui_layout::layout::{partition_window_measures, ActionDescriptor, WindowMeasure};
    let measures = vec![
        WindowMeasure::Group {
            id: "brush-params".into(),
            label: "Brush".into(),
            default_open: Some(true),
            active_utility_id: Some("utility.a".into()),
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: vec![WindowMeasure::Toggle {
                id: "brush-size".into(),
                icon_id: "paintbrush".into(),
                label: Some("Size".into()),
                pressed: false,
                text: None,
                on_change: ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None },
            }],
        },
        WindowMeasure::Group { id: "grid".into(), label: "Grid".into(), default_open: Some(true), active_utility_id: None, value: None, min: None, max: None, step: None, ready: None, loading: None, waiting: None, on_change: None, children: vec![] },
    ];
    let semio_framework_ui_layout::layout::WindowMeasurePartition { general, utility_options } = partition_window_measures(&measures, Some("utility.a")).expect("bounded fixture measure partition");
    assert_eq!(utility_options.len(), 1, "matching utility surfaces the tagged group in utility options");
    assert!(matches!(utility_options.get(0).copied(), Some(WindowMeasure::Toggle { id, .. }) if id == "brush-size"));
    assert_eq!(general.len(), 1, "untagged group stays in the general measures rail");
    assert!(matches!(general.get(0).copied(), Some(WindowMeasure::Group { id, .. }) if id == "grid"));
    let semio_framework_ui_layout::layout::WindowMeasurePartition { general: general_other, utility_options: utility_options_other } = partition_window_measures(&measures, Some("utility.b")).expect("bounded fixture measure partition");
    assert!(utility_options_other.is_empty(), "wrong active utility drops the tagged group");
    assert_eq!(general_other.len(), 1, "untagged group unaffected by active utility");
    let semio_framework_ui_layout::layout::WindowMeasurePartition { general: general_none, utility_options: utility_options_none } = partition_window_measures(&measures, None).expect("bounded fixture measure partition");
    assert!(utility_options_none.is_empty(), "no active utility drops the tagged group");
    assert_eq!(general_none.len(), 1);
}

fn dock_with(root: DockNode, active: &str) -> DockState {
    let mut dock = DockState::default();
    dock.root = root;
    dock.sync_active_window(active);
    dock
}

fn painted_tab_control_ids(dock: &DockState, labels: &HashMap<String, String>) -> Vec<String> {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let icon_ids = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: labels, window_icon_ids: &icon_ids, control_names: None };
    dock.paint_chrome(&mut ctx, Rect::new(0.0, 0.0, 600.0, 400.0), false);
    input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).filter(|id| id.starts_with("dock.tab.")).collect()
}

/// 📑️ React renders Focus/Unfocus only when `!mobile && canMaximize`, then Close unconditionally —
/// `🎨️Canvas/🟦️.tsx:1072-1100`. There is no third ("new window") action anywhere in that tab bar.
#[test]
fn dock_tab_actions_match_react_mode_dock_tab_bar() {
    assert_eq!(dock_tab_actions(true, false), vec![("focus", "maximize-2"), ("close", "x"), ("drag", "grip-vertical")]);
    assert_eq!(dock_tab_actions(true, true), vec![("focus", "minimize-2"), ("close", "x"), ("drag", "grip-vertical")], "a maximized stack shows Unfocus");
    assert_eq!(dock_tab_actions(false, false), vec![("close", "x"), ("drag", "grip-vertical")], "single window or mobile keeps Close plus the grip");
    assert!(!dock_tab_actions(true, false).iter().any(|(action, _)| *action == "new"), "the dead new-window chip is gone");
}

/// 📑️ `canMaximize = modeCollectWindowIds(layout).length > 1` (`🎨️Canvas/🟦️.tsx:1800`) and
/// `showMaximize = !mobile && canMaximize` (`:1158`).
#[test]
fn show_maximize_follows_window_count_and_mobile() {
    let single = dock_with(stack_tabs(&["a"], "a"), "a");
    assert!(!single.can_maximize());
    assert!(!single.show_maximize());
    let mut many = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    assert!(many.can_maximize());
    assert!(many.show_maximize());
    many.mobile = true;
    assert!(many.can_maximize(), "the canvas still holds two windows");
    assert!(!many.show_maximize(), "mobile hides Focus/Unfocus — a mobile window always fills the canvas");
}

/// 📑️ Exactly the React action set reaches the input layer: `.close` always, `.focus` only when the
/// canvas can maximize, and never a `.new` control id (no shell dispatch arm ever answered one).
#[test]
fn dock_tab_hits_register_only_reacts_two_actions() {
    let labels = HashMap::from([("a".to_string(), "A".to_string()), ("b".to_string(), "B".to_string())]);
    let two = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    let ids = painted_tab_control_ids(&two, &labels);
    assert!(ids.iter().any(|id| id == "dock.tab..a.close"));
    assert!(ids.iter().any(|id| id == "dock.tab..a.focus"));
    assert!(ids.iter().any(|id| id == "dock.tab..a.drag"), "React's DragHandle is the ONLY drag origin, so it must be a target");
    assert!(!ids.iter().any(|id| id.ends_with(".new")), "no dead new-window control id is ever registered");
    let one = dock_with(stack_tabs(&["a"], "a"), "a");
    let ids = painted_tab_control_ids(&one, &labels);
    assert!(ids.iter().any(|id| id == "dock.tab..a.close"), "close stays reachable for a lone window");
    assert!(ids.iter().any(|id| id == "dock.tab..a.drag"), "so does the grip");
    assert!(!ids.iter().any(|id| id.ends_with(".focus")), "React hides Focus when the canvas holds one window");
}

/// 📑️ ⚖️ LAW: a tab publishes its SELECT target before its action chips, and its chips left to right
/// (`focus`, `close`, `drag`) — React's own DOM order (`🎨️Canvas/🟦️.tsx:1034-1100`: the
/// `<button role="tab">` label, then `mode-dock-tab-focus`, `mode-dock-tab-close`, then the
/// `DragHandle`).
///
/// 🩸️ The chips were published FIRST, so the first `dock.tab.…` row of a stack was its destructive
/// `close` chip. The parity probe's `window-reopen` step resolves "the dock tab" by scanning the
/// published rows in order: on React it landed on the `mode-dock-tabbar` container and did nothing,
/// here it landed on `dock.tab..puzzle3d-main-perspective.close` and closed the LAST world pane, after
/// which every window-owned chord was a hinted no-op against an empty dock
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `🗑️generated/w12c-parity-run-19/steps.json` step 18).
#[test]
fn a_tabs_select_target_is_published_before_its_destructive_chips() {
    let labels = HashMap::from([("a".to_string(), "A".to_string()), ("b".to_string(), "B".to_string())]);
    let two = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    let ids = painted_tab_control_ids(&two, &labels);
    let first = ids.first().map(String::as_str);
    assert_eq!(first, Some("dock.tab..a"), "📑️ the first dock tab row a stack publishes is a SELECT target, never a close chip: {ids:?}");
    for window_id in ["a", "b"] {
        let position = |suffix: &str| ids.iter().position(|id| *id == format!("dock.tab..{window_id}{suffix}")).unwrap_or_else(|| panic!("📑️ {window_id}{suffix} is registered: {ids:?}"));
        let select = position("");
        assert!(select < position(".focus"), "📑️ {window_id}: select precedes Focus");
        assert!(position(".focus") < position(".close"), "📑️ {window_id}: Focus precedes Close, left to right");
        assert!(position(".close") < position(".drag"), "📑️ {window_id}: Close precedes the grip");
    }
    assert!(!ids.iter().take_while(|id| !id.ends_with("..a")).any(|id| id.ends_with(".close")), "📑️ no close chip is published before the tab it belongs to");
}

/// 🈳️ ⚖️ LAW: a stack holding NO tabs still answers a DROP body — React's `WindowChrome` keeps
/// rendering for an empty stack, which is how a dragged window is dropped back into an emptied dock —
/// but it owns no window and no silhouette, because React's `activeDescriptor` is `undefined` there
/// and its `stackBody` renders nothing (`🎨️Canvas/🟦️.tsx:1176`/`:1212`).
///
/// 🩸️ The silhouette was keyed by the stack's `active` tab unconditionally, so closing the last window
/// minted a `""`-keyed window: the engine-surface census reported `windows=["", "tool.fill", …]`, the
/// chrome census published a `window:` surface, and the body registered a full-bounds
/// `HitKind::ScrollRegion` with an EMPTY control id over the whole canvas
/// (`📓️w12c-chords-and-camera-live.md` §4.3).
#[test]
fn an_emptied_stack_keeps_its_drop_body_and_owns_no_window() {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let labels = HashMap::from([("a".to_string(), "A".to_string())]);
    let mut dock = dock_with(stack_tabs(&["a"], "a"), "a");
    assert!(dock.close_window("a"), "🪟️ the last window closes");
    let (bodies, silhouettes) = dock.stack_body_rects_with_silhouettes(Rect::new(0.0, 0.0, 600.0, 400.0), &theme, &labels, &mut atlas);
    assert_eq!(bodies.len(), 1, "🈳️ the emptied root stack is still a drop body");
    assert_eq!(bodies[0].2, "", "🈳️ …and names no window");
    assert!(silhouettes.is_empty(), "🈳️ an empty stack owns no silhouette — React draws no window there");
    assert!(dock.collect_window_ids().is_empty(), "🈳️ and the dock holds no window id at all");
}
