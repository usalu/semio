/// 📌️ Anchor ids, resize and default dock: anchor group and id mapping, panel width and resize hits, and the shared default dock fixture.

#[test]
fn panel_anchor_from_group_matches_panel_group_anchor_corners() {
    assert_eq!(PanelAnchor::from_group(PanelGroup::Workbench), PanelAnchor::TopLeft);
    assert_eq!(PanelAnchor::from_group(PanelGroup::Details), PanelAnchor::TopRight);
    assert_eq!(PanelAnchor::from_group(PanelGroup::Display), PanelAnchor::BottomLeft);
    assert_eq!(PanelAnchor::from_group(PanelGroup::Settings), PanelAnchor::BottomRight);
}

#[test]
fn panel_anchor_as_str_matches_react_panel_anchor_ids() {
    let expected = [
        (PanelAnchor::TopLeft, "top-left"),
        (PanelAnchor::TopMiddle, "top-middle"),
        (PanelAnchor::TopRight, "top-right"),
        (PanelAnchor::RightMiddle, "right-middle"),
        (PanelAnchor::BottomRight, "bottom-right"),
        (PanelAnchor::BottomMiddle, "bottom-middle"),
        (PanelAnchor::BottomLeft, "bottom-left"),
        (PanelAnchor::LeftMiddle, "left-middle"),
    ];
    for (index, (anchor, id)) in expected.into_iter().enumerate() {
        assert_eq!(anchor.as_str(), id);
        assert_eq!(PanelAnchor::from_str(id), Some(anchor));
        assert_eq!(anchor.index(), index, "`index()` must agree with `ALL`'s own order");
        assert_eq!(PanelAnchor::ALL[index], anchor);
    }
    assert_eq!(PanelAnchor::ALL.len(), 8);
}

/// ↔ Keeps native panel initialization aligned with the React shell's shared 300px default.
#[test]
fn panel_default_width_is_uniform_and_wider_than_the_former_document_panel() {
    assert_eq!(DEFAULT_PANEL_WIDTH_PX, 300.0);
    assert!(DEFAULT_PANEL_WIDTH_PX > 280.0);
}

/// ↔️ The real panel walk publishes exactly React's resize edges and one compact-spacing hit rail.
#[test]
fn panel_resize_hits_match_reacts_edge_count_and_single_spacing_width() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/↔️panel-resize/🔣️.json")).expect("panel resize fixture");
    let theme = Theme::default();
    assert_eq!(fixture["widthUiSpacing"], serde_json::json!(1));
    for case in fixture["cases"].as_array().expect("resize cases") {
        let anchor_id = case["anchor"].as_str().expect("anchor");
        let anchor = PanelAnchor::from_str(anchor_id).expect("known fixture anchor");
        let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        *shell.dock.dock_tabs.tabs_mut(anchor) = vec![DockTabNode::leaf("fixture.resize", "Resize", "panel-right", 0)];
        shell.dock.anchor_state_mut(anchor).path = vec!["fixture.resize".into()];
        shell.dock.anchor_state_mut(anchor).visible = true;
        let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
        let panel = ChromeViewCtx::of(&mut shell).anchor_rect(anchor, body, &theme);
        let mut draw = DrawList::default();
        draw.set_screen_height(800.0);
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
        let mut cursor = ShellChromeChildCursor::default();
        assert!((0..32).any(|_| caller.turn(|cx|ChromeViewCtx::of(&mut shell).render_panel_step(&mut cursor, anchor, &mut draw, None, &mut atlas, &icons, &mut input, &theme, body, &mut resources, cx))), "{anchor_id} panel walk completes");
        input.publish_hits();
        let actual: Vec<&HitTarget<ActionDescriptor>> = input.hits().iter().filter(|hit| hit.kind == HitKind::PanelResize).collect();
        let expected = case["handles"].as_array().expect("handles");
        assert_eq!(actual.len(), expected.len(), "{anchor_id} resize handle count");
        for handle in expected {
            let edge = handle["edge"].as_str().expect("edge");
            let suffix = handle["nativeSuffix"].as_str().expect("native suffix");
            let expected_factor = handle["deltaFactor"].as_i64().expect("delta factor");
            let id = format!("panel.resize.{anchor_id}.{suffix}");
            let hit = actual.iter().find(|hit| hit.control_id.as_deref() == Some(id.as_str())).unwrap_or_else(|| panic!("↔️ {anchor_id} must publish {id}: {actual:?}"));
            let actual_factor: i64 = (if anchor.horizontal() == "middle" { 2 } else { 1 }) * if suffix == "inner" { -1 } else { 1 };
            assert_eq!(actual_factor, expected_factor, "{anchor_id}/{edge} keeps React's drag sign and middle multiplier");
            assert!((hit.rect.w - theme.panel_inset).abs() < 0.01, "{anchor_id}/{edge} uses one compact spacing unit");
            let expected_x = if edge == "left" { panel.x } else { panel.x + panel.w - theme.panel_inset };
            assert!((hit.rect.x - expected_x).abs() < 0.01, "{anchor_id}/{edge} sits on React's same physical edge: {:?}", hit.rect);
            assert_eq!(hit.rect.y, panel.y);
            assert_eq!(hit.rect.h, panel.h);
        }
        caller.retire_world(&mut resources);
    }
    caller.finish();
}

//#region 🧭️DefaultDockFixture
/// 🧭️ The shared default-dock fixture both renderers assert against — the id-only skeleton React's
/// `defaultDock` (`🏛️ShellHost/🟦️.tsx`'s 🧭️DockAssembly) and this shell's `default_dock` must agree on.
fn default_dock_fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧭️default-dock/🔣️.json")).expect("default dock fixture")
}

fn fixture_group(group: &str) -> PanelGroup {
    match group {
        "workbench" => PanelGroup::Workbench,
        "details" => PanelGroup::Details,
        "display" => PanelGroup::Display,
        _ => PanelGroup::Settings,
    }
}

/// 🧭️ A shell whose session declares exactly the fixture's panel tabs, plus the framework History tab
/// `AppBuilder::try_build_definition` injects into every app, and one attached sync backbone.
fn fixture_dock_shell() -> ShellState {
    let fixture = default_dock_fixture();
    let mut app = test_app(Vec::new(), Vec::new());
    app.id = "fixture-app".into();
    app.controller_id = "fixture.controller".into();
    app.panel_tabs = fixture["app"]["panelTabs"]
        .as_array()
        .expect("fixture panel tabs")
        .iter()
        .map(|tab| PanelTabDefinition {
            kind: PanelTabKind::App(tab["id"].as_str().expect("tab id").to_string()),
            label: LocalizedLabel::data(tab["label"].as_str().expect("tab label")),
            group: fixture_group(tab["group"].as_str().expect("tab group")),
            body_key: Some(tab["id"].as_str().expect("tab id").replace('.', "/")),
            children: vec![],
        })
        .collect();
    app.panel_tabs.push(PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_HISTORY_ID.into()), label: LocalizedLabel::data("History"), group: PanelGroup::Settings, body_key: Some("framework/history".into()), children: vec![] });
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.docs.session = Some(ActiveSession { plugin_id: "fixture".into(), instance_id: 1, app, view_state: ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
    shell.sync.sync_backbone_uri = Some("folder:///tmp/fixture".into());
    SessionCtx::of(&mut shell).sync_dock_tabs();
    shell
}

/// 🧭️ **The parity pin.** `default_dock` must produce, anchor by anchor and id by id, exactly the
/// arrangement the shared fixture declares — the same one React's `defaultDock` builds from the same
/// sources. A renderer that silently drops an anchor (which is precisely what the former hardcoded
/// left/right fold did to six of eight) fails here rather than in a screenshot.
#[test]
fn default_dock_matches_the_shared_react_fixture() {
    let fixture = default_dock_fixture();
    let shell = fixture_dock_shell();
    let skeleton = dock_skeleton_of(&SessionCtx::of(&mut shell).default_dock());
    let expected: DockSkeleton = serde_json::from_value(fixture["expected"].clone()).expect("fixture skeleton");
    assert_eq!(skeleton.version, 3);
    for anchor in PanelAnchor::ALL {
        let empty = Vec::new();
        let actual_ids: Vec<&str> = skeleton.anchors.get(anchor.as_str()).unwrap_or(&empty).iter().map(|tab| tab.id.as_str()).collect();
        let expected_ids: Vec<&str> = expected.anchors.get(anchor.as_str()).unwrap_or(&empty).iter().map(|tab| tab.id.as_str()).collect();
        assert_eq!(actual_ids, expected_ids, "🧭️ anchor {} must carry the fixture's tabs in the fixture's order", anchor.as_str());
    }
    assert_eq!(skeleton, expected, "🧭️ the whole skeleton, branch children included, must match the fixture");
}

/// 🧭️ The command palette is a REAL `bottom-middle` anchor now, not a Settings tab — the packet's
/// headline gap. Its branch id and category leaf ids are React's.
#[test]
fn default_dock_puts_the_command_branch_on_bottom_middle() {
    let shell = fixture_dock_shell();
    let dock = SessionCtx::of(&mut shell).default_dock();
    let branch = dock.tabs(PanelAnchor::BottomMiddle).first().expect("bottom-middle carries the Command branch");
    assert_eq!(branch.id, FRAMEWORK_CATEGORY_COMMAND_ID);
    assert!(branch.is_branch(), "the Command branch nests its category leaves");
    assert!(branch.children.iter().all(|child| child.id.starts_with(FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX)));
    assert_eq!(dock.locate(FRAMEWORK_CATEGORY_COMMAND_ID).map(|(anchor, _)| anchor), Some(PanelAnchor::BottomMiddle));
}
//#endregion 🧭️DefaultDockFixture
