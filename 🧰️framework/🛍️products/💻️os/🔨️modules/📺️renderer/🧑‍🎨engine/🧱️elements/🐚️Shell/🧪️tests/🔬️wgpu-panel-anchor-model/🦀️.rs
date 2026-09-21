
use super::*;
// 🩹️ `PanelTabKind` stopped being reachable through the shell module's own `use` list, and the
// `📌️panel-state` fixture moved from `🧑‍🎨engine/🧫️fixtures/` into this element's own `🧫️fixtures/`
// (panel-tab lane, 2026-09-13) — both are re-pointed here so the crate's test target builds again.
use semio_framework::PanelTabKind;

/// 🧪️ `ShellState::new` calls `load_persisted_dock`, which — on native — reads whatever the
/// machine running the test happens to have in its `semio.os.config` document. Every assertion
/// below explicitly sets the state it exercises afterward, so the outcome never depends on that.
fn fresh_state() -> ShellState {
    ShellState::new(Vec::new(), String::new())
}

fn host_test_apps() -> (AppDefinition, AppDefinition) {
    let mut home = super::command_registry_tests::test_app(Vec::new(), Vec::new());
    home.id = "home".into();
    home.controller_id = "space.home".into();
    home.panel_tabs = vec![PanelTabDefinition { kind: PanelTabKind::App("home-library".into()), label: LocalizedLabel::data("Home Library"), group: PanelGroup::Display, body_key: Some("home.library".into()), children: vec![] }];
    let mut studio = super::command_registry_tests::test_app(Vec::new(), Vec::new());
    studio.id = "studio".into();
    studio.controller_id = "space.studio".into();
    studio.panel_tabs = vec![
        PanelTabDefinition { kind: PanelTabKind::App("s-play-catalogue".into()), label: LocalizedLabel::data("Catalogue"), group: PanelGroup::Workbench, body_key: Some("s.play.catalogue".into()), children: vec![] },
        PanelTabDefinition { kind: PanelTabKind::App("s-play-inspector".into()), label: LocalizedLabel::data("Inspector"), group: PanelGroup::Details, body_key: Some("s.play.inspector".into()), children: vec![] },
        PanelTabDefinition {
            kind: PanelTabKind::App("studio-settings".into()),
            label: LocalizedLabel::data("Studio Settings"),
            group: PanelGroup::Settings,
            body_key: None,
            children: vec![PanelTabDefinition { kind: PanelTabKind::App("studio-settings-leaf".into()), label: LocalizedLabel::data("Settings"), group: PanelGroup::Settings, body_key: Some("studio.settings".into()), children: vec![] }],
        },
    ];
    (home, studio)
}

pub(super) fn host_test_shell() -> ShellState {
    let (home, studio) = host_test_apps();
    let manifest = semio_framework::PluginManifest {
        plugin_id: "space".into(),
        label: "Space".into(),
        version: "1".into(),
        apps: vec![home, studio.clone()],
        examples: vec![],
        capabilities: vec![],
        topic_contributions: vec![],
        commands: vec![],
        artifact_kinds: vec![],
        dependencies: vec![],
        contributions: vec![],
    };
    let bridge = ProgramBridgeEntry::from_wasm("space".into(), None, std::path::PathBuf::from("missing-host-panel-guest.wasm"), manifest).expect("nonrunnable host bridge");
    let mut shell = ShellState::new(vec![bridge], "space".into());
    let base = SpacePanelState {
        active_panel_tab: "s-play-catalogue".into(),
        spawned_apps: vec![
            SpawnedAppEntry { id: "spawned-model".into(), plugin_id: "model".into(), instance_id: 41, app_id: "model@1/any#editor".into(), label: "Model".into(), breadcrumb: vec!["Workspace".into(), "Model".into()] },
            SpawnedAppEntry { id: "spawned-note".into(), plugin_id: "note".into(), instance_id: 42, app_id: "note@1/any#editor".into(), label: "Note".into(), breadcrumb: vec!["Workspace".into(), "Note".into()] },
        ],
        active_spawned_id: Some("spawned-model".into()),
    };
    let mut view_state = ViewModel::default();
    view_state.active_mode_id = Some(studio.default_mode_id.clone());
    view_state.active_window_kind_id = Some(studio.window_kinds.first().id.clone());
    view_state.panel_json = Some(ShellState::panel_json(&base).expect("strict base panel"));
    shell.session = Some(ActiveSession { plugin_id: "space".into(), instance_id: 77, app: studio, view_state });
    shell
}

#[test]
fn host_panel_json_codec_matches_the_neutral_fixture_and_rejects_invalid_carriage() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/📌️panel-state/🔣️.json")).expect("panel fixture");
    let base: SpacePanelState = serde_json::from_value(fixture["base"].clone()).expect("fixture base");
    let encoded = ShellState::panel_json(&base).expect("strict panel JSON");
    assert!(encoded.starts_with('{'));
    let mut view = ViewModel::default();
    view.panel_json = Some(encoded.clone());
    assert_eq!(ShellState::panel_state_from_view(&view).expect("strict decode"), Some(base.clone()));
    view.panel_json = Some(serde_json::json!({ "activePanelTab": "s-play-catalogue", "spawnedApps": [], "programs": [] }).to_string());
    assert_eq!(ShellState::panel_state_from_view(&view).unwrap_err(), "host-panel.invalid-json");
    let mut duplicate = base.clone();
    duplicate.spawned_apps.push(base.spawned_apps[0].clone());
    assert_eq!(ShellState::panel_json(&duplicate).unwrap_err(), "host-panel.duplicate-spawned-identity");
    let mut missing_focus = base;
    missing_focus.active_spawned_id = Some("spawned-missing".into());
    assert_eq!(ShellState::panel_json(&missing_focus).unwrap_err(), "host-panel.invalid-active-spawned");
}

#[test]
fn host_panel_action_is_claimed_before_guest_and_preserves_the_session_roster_and_home_projection() {
    let mut shell = host_test_shell();
    let before = ShellState::panel_state_from_view(&shell.session.as_ref().unwrap().view_state).unwrap().unwrap();
    semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "s-play-inspector" }) })).expect("configured leaf is host-owned");
    let after = ShellState::panel_state_from_view(&shell.session.as_ref().unwrap().view_state).unwrap().unwrap();
    assert_eq!(after.active_panel_tab, "s-play-inspector");
    assert_eq!(after.spawned_apps, before.spawned_apps);
    assert_eq!(after.active_spawned_id, before.active_spawned_id);
    assert!(shell.anchor_open(PanelAnchor::TopRight), "a Details-group leaf reveals the top-right anchor");
    assert_eq!(shell.anchor_state(PanelAnchor::TopRight).active_tab(), Some("s-play-inspector"));

    let accepted_json = shell.session.as_ref().unwrap().view_state.panel_json.clone();
    let container_error = semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "studio-settings" }) })).unwrap_err();
    assert_eq!(container_error, "host-panel.tab-is-not-configured-leaf");
    assert_eq!(shell.session.as_ref().unwrap().view_state.panel_json, accepted_json);
    let stale_error = semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "missing" }) })).unwrap_err();
    assert_eq!(stale_error, "host-panel.tab-is-not-configured-leaf");
    let missing_error = semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: None })).unwrap_err();
    assert_eq!(missing_error, "host-panel.invalid-tab-id");

    let guest_error = semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "foreign.controller".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "s-play-inspector" }) })).expect_err("unclaimed controller reaches the missing guest fixture");
    assert!(!guest_error.starts_with("host-panel."), "unclaimed route must expose the actual missing guest failure: {guest_error}");

    let (home, _) = host_test_apps();
    let home_panel = SpacePanelState { active_panel_tab: "s-play-catalogue".into(), spawned_apps: after.spawned_apps.clone(), active_spawned_id: after.active_spawned_id.clone() };
    let mut home_view = ViewModel::default();
    home_view.active_mode_id = Some(home.default_mode_id.clone());
    home_view.active_window_kind_id = Some(home.window_kinds.first().id.clone());
    home_view.panel_json = Some(ShellState::panel_json(&home_panel).unwrap());
    shell.directory_home = Some(DirectoryHomeProjection::new("space".into(), 88, home.clone(), home_view.clone()).expect("directory home projection"));
    shell.session = Some(ActiveSession { plugin_id: "space".into(), instance_id: 88, app: home, view_state: home_view });
    semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.home".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "home-library" }) })).expect("active host session owns its configured leaf");
    let restored = shell.directory_home.as_ref().unwrap().active_session();
    let restored_panel = ShellState::panel_state_from_view(&restored.view_state).unwrap().unwrap();
    assert_eq!(restored_panel.active_panel_tab, "home-library");
    assert_eq!(restored_panel.spawned_apps, after.spawned_apps);
    assert_eq!(restored_panel.active_spawned_id, after.active_spawned_id);
    assert!(shell.anchor_open(PanelAnchor::BottomLeft), "a Display-group leaf reveals the bottom-left anchor");
    assert_eq!(shell.anchor_state(PanelAnchor::BottomLeft).active_tab(), Some("home-library"));
    eprintln!("[DEBUG] native host panel action stayed in session ownership, rejected invalid claimed routes before the missing guest, and restored DirectoryHomeProjection state");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn host_panel_view_context_replaces_stale_session_identity_for_every_native_call() {
    let mut shell = host_test_shell();
    let session = shell.session.as_mut().expect("session");
    session.view_state.session_identity = Some(ViewSessionIdentity { user_id: "stale".into(), display_name: "Stale".into() });
    let session = shell.session.as_ref().expect("session").clone();
    assert_eq!(shell.live_view_state(&session).session_identity, None, "an unauthenticated host must remove a persisted stale identity");
    shell.identity = Some(Identity { user_id: "user-a".into(), email: "a@example.test".into(), display_name: "Ada".into(), hub_base_url: "https://hub.example".into(), issued_at_ms: 1 });
    assert_eq!(shell.live_view_state(&session).session_identity, Some(ViewSessionIdentity { user_id: "user-a".into(), display_name: "Ada".into() }));
    shell.identity = Some(Identity { user_id: "user-b".into(), email: "b@example.test".into(), display_name: "Berta".into(), hub_base_url: "https://hub.example".into(), issued_at_ms: 2 });
    assert_eq!(shell.live_view_state(&session).session_identity, Some(ViewSessionIdentity { user_id: "user-b".into(), display_name: "Berta".into() }));
}

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
    let fixture: Value = serde_json::from_str(include_str!("../../🧪️fixtures/↔️panel-resize/🔣️.json")).expect("panel resize fixture");
    let theme = Theme::default();
    assert_eq!(fixture["widthUiSpacing"], serde_json::json!(1));
    for case in fixture["cases"].as_array().expect("resize cases") {
        let anchor_id = case["anchor"].as_str().expect("anchor");
        let anchor = PanelAnchor::from_str(anchor_id).expect("known fixture anchor");
        let mut shell = ShellState::new(Vec::new(), String::new());
        *shell.dock_tabs.tabs_mut(anchor) = vec![DockTabNode::leaf("fixture.resize", "Resize", "panel-right", 0)];
        shell.anchor_state_mut(anchor).path = vec!["fixture.resize".into()];
        shell.anchor_state_mut(anchor).visible = true;
        let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
        let panel = shell.anchor_rect(anchor, body, &theme);
        let mut draw = DrawList::default();
        draw.set_screen_height(800.0);
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
        let mut cursor = ShellChromeChildCursor::default();
        assert!((0..32).any(|_| shell.render_panel_step(&mut cursor, anchor, &mut draw, None, &mut atlas, &icons, &mut input, &theme, body, &mut resources)), "{anchor_id} panel walk completes");
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
    }
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
    let mut app = super::command_registry_tests::test_app(Vec::new(), Vec::new());
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
    app.panel_tabs.push(PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_HISTORY_ID.into()),
        label: LocalizedLabel::data("History"),
        group: PanelGroup::Settings,
        body_key: Some("framework/history".into()),
        children: vec![],
    });
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.session = Some(ActiveSession { plugin_id: "fixture".into(), instance_id: 1, app, view_state: ViewModel::default() });
    shell.sync_backbone_uri = Some("folder:///tmp/fixture".into());
    shell.sync_dock_tabs();
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
    let skeleton = dock_skeleton_of(&shell.default_dock());
    let expected: DockSkeleton = serde_json::from_value(fixture["expected"].clone()).expect("fixture skeleton");
    assert_eq!(skeleton.version, 3);
    for anchor in PanelAnchor::ALL {
        let empty = Vec::new();
        let actual_ids: Vec<&str> = skeleton.anchors.get(anchor.as_str()).unwrap_or(&empty).iter().map(|tab| tab.id.as_str()).collect();
        let expected_ids: Vec<&str> = expected.anchors.get(anchor.as_str()).unwrap_or(&empty).iter().map(|tab| tab.id.as_str()).collect();
        assert_eq!(actual_ids, expected_ids, "🧭️ anchor {} must carry the fixture's tabs in the fixture's order", anchor.as_str());
    }
    assert_eq!(skeleton, expected, "🧭️ the whole skeleton, branch children included, must match the fixture");
    eprintln!("[DEBUG] wgpu default dock matched the shared React default-dock fixture across all 8 anchors");
}

/// 🧭️ The command palette is a REAL `bottom-middle` anchor now, not a Settings tab — the packet's
/// headline gap. Its branch id and category leaf ids are React's.
#[test]
fn default_dock_puts_the_command_branch_on_bottom_middle() {
    let shell = fixture_dock_shell();
    let dock = shell.default_dock();
    let branch = dock.tabs(PanelAnchor::BottomMiddle).first().expect("bottom-middle carries the Command branch");
    assert_eq!(branch.id, FRAMEWORK_CATEGORY_COMMAND_ID);
    assert!(branch.is_branch(), "the Command branch nests its category leaves");
    assert!(branch.children.iter().all(|child| child.id.starts_with(FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX)));
    assert_eq!(dock.locate(FRAMEWORK_CATEGORY_COMMAND_ID).map(|(anchor, _)| anchor), Some(PanelAnchor::BottomMiddle));
}
//#endregion 🧭️DefaultDockFixture

//#region 🧭️AnchorGeometry
/// 🧭️ `anchorPositionStyle`'s insets: a left column starts one inset in from the body's left edge, a
/// right column ends one inset in from its right, and a middle column centres — mirrored on the vertical
/// axis by the column banding. Checked against the same `anchorInsets` table the shared fixture carries.
#[test]
fn anchor_rects_sit_at_the_react_anchor_insets() {
    let fixture = default_dock_fixture();
    let theme = Theme::default();
    let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
    let inset = theme.panel_inset;
    for anchor in PanelAnchor::ALL {
        let rect = anchor_panel_rect(anchor, DEFAULT_PANEL_WIDTH_PX, None, 1, 0, body, &theme);
        let sides = fixture["anchorInsets"][anchor.as_str()].as_array().expect("fixture insets");
        assert_eq!(sides[0].as_str(), Some(anchor.horizontal()), "🧭️ {}'s column must match the fixture", anchor.as_str());
        assert_eq!(sides[1].as_str(), Some(anchor.vertical()), "🧭️ {}'s row must match the fixture", anchor.as_str());
        match anchor.horizontal() {
            "left" => assert_eq!(rect.x, body.x + inset),
            "right" => assert_eq!(rect.x + rect.w, body.x + body.w - inset),
            _ => assert!((rect.x + rect.w * 0.5 - (body.x + body.w * 0.5)).abs() < 0.01, "🧭️ a middle column centres on the body"),
        }
        assert_eq!(rect.y, body.y + inset, "🧭️ a column's only open anchor spans the whole band");
        assert_eq!(rect.h, body.h - inset * 2.0);
        assert_eq!(rect.w, DEFAULT_PANEL_WIDTH_PX);
    }
}

/// 🧭️ Two open anchors in one column band between themselves instead of painting over each other —
/// the geometric consequence of there being eight independent anchors rather than two slots.
#[test]
fn two_open_anchors_in_one_column_split_its_band() {
    let theme = Theme::default();
    let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
    let top = anchor_panel_rect(PanelAnchor::TopLeft, DEFAULT_PANEL_WIDTH_PX, None, 2, 0, body, &theme);
    let bottom = anchor_panel_rect(PanelAnchor::BottomLeft, DEFAULT_PANEL_WIDTH_PX, None, 2, 1, body, &theme);
    assert_eq!(top.x, bottom.x, "both sit in the same column");
    assert!(top.y + top.h <= bottom.y + 0.01, "🧭️ the top anchor's band ends before the bottom anchor's begins");
    assert!((bottom.y + bottom.h - (body.y + body.h - theme.panel_inset)).abs() < 0.01, "🧭️ the bottom anchor ends one inset above the body's bottom edge");
}

/// 🧭️ The live per-anchor geometry: only OPEN anchors claim a band, so opening the second anchor of a
/// column halves the first rather than leaving it full height and overlapped.
#[test]
fn anchor_rect_bands_only_the_open_anchors_of_a_column() {
    let theme = Theme::default();
    let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
    let mut shell = fixture_dock_shell();
    shell.toggle_anchor(PanelAnchor::TopLeft);
    let alone = shell.anchor_rect(PanelAnchor::TopLeft, body, &theme);
    // 📐️ The chrome-hosted root remains in the navbar/footer. The floating child panel grows into
    // the overhang but omits that already-owned root row.
    assert_eq!(alone.h, body.h + (theme.navbar_height + theme.control_height) * 0.5 - theme.control_height);
    shell.toggle_anchor(PanelAnchor::BottomLeft);
    let shared = shell.anchor_rect(PanelAnchor::TopLeft, body, &theme);
    assert!(shared.h < alone.h * 0.6, "🧭️ opening the column's other anchor must shrink this one's band");
    assert_eq!(shell.open_anchors(), vec![PanelAnchor::TopLeft, PanelAnchor::BottomLeft]);
}

/// 🧭️ React's `chromeHostedOpenPanelPositionStyle`: an OPEN top anchor is pulled
/// `-(size-large + size-medium)/2` out of the mode body and into the navbar band. The root row stays
/// in chrome, so the floating child panel starts one root-row height after that edge. Measured on
/// React's live DOM at 1600×1000 (`🗑️generated/w14d-cap-occlusion.json`, ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY): mode body `y=29`, open Catalogue panel `y=3 h=168`, dock cap
/// row `y=32..61`, focus/close chips `[48,35,22,22]`/`[73,35,22,22]` — under the panel's body.
#[test]
fn an_open_top_panel_is_pulled_into_the_navbar_band_like_reacts_chrome_hosted_style() {
    let theme = Theme::default();
    let overhang = (theme.navbar_height + theme.control_height) * 0.5;
    let body = Rect::new(0.0, theme.navbar_height, 1600.0, 1000.0 - theme.navbar_height - theme.footer_height);
    let mut shell = fixture_dock_shell();
    shell.toggle_anchor(PanelAnchor::TopLeft);
    let rect = shell.anchor_rect(PanelAnchor::TopLeft, body, &theme);
    assert!((rect.y - (body.y - overhang + theme.control_height)).abs() < 0.01, "🧭️ a top child panel starts after its chrome-owned root row: {rect:?}");
    assert!(rect.y < theme.navbar_height, "🧭️ …which is inside the navbar band React parks the folded toggle in");
    assert!((rect.h - (body.h + overhang - theme.control_height)).abs() < 0.01, "🧭️ and its band omits the chrome-owned root-row height");
}

/// 🧭️ The same rule mirrored on the bottom row: a bottom child panel ends where the footer-owned
/// root row starts, and a middle anchor — which React gives no bonded vertical edge — still centres
/// on the plain mode body.
#[test]
fn a_bottom_anchor_overhangs_into_the_footer_and_a_middle_anchor_keeps_the_body() {
    let theme = Theme::default();
    let overhang = (theme.navbar_height + theme.control_height) * 0.5;
    let body = Rect::new(0.0, theme.navbar_height, 1600.0, 1000.0 - theme.navbar_height - theme.footer_height);
    let mut shell = fixture_dock_shell();
    shell.toggle_anchor(PanelAnchor::BottomLeft);
    let bottom = shell.anchor_rect(PanelAnchor::BottomLeft, body, &theme);
    assert!((bottom.y + bottom.h - (body.y + body.h + overhang - theme.control_height)).abs() < 0.01, "🧭️ a bottom child panel ends at its footer-owned root row: {bottom:?}");
    let mut middle_shell = fixture_dock_shell();
    middle_shell.toggle_anchor(PanelAnchor::LeftMiddle);
    let middle = middle_shell.anchor_rect(PanelAnchor::LeftMiddle, body, &theme);
    assert!((middle.y + middle.h * 0.5 - (body.y + body.h * 0.5)).abs() < 0.01, "🧭️ a middle anchor centres on the body itself: {middle:?}");
}

/// 🩸️ The regression this replaces: the panel band no longer depends on `dock_window_plan`, so an
/// open panel occupies the SAME box whether the dock carries two stacks, one, or none — the geometry
/// React has, where the dock's caps live under the panel instead of beside it. Two dock states, one
/// rect.
#[test]
fn the_panel_band_no_longer_moves_with_the_dock_window_plan() {
    let theme = Theme::default();
    let body = Rect::new(0.0, theme.navbar_height, 1600.0, 900.0);
    let mut shell = fixture_dock_shell();
    shell.toggle_anchor(PanelAnchor::TopLeft);
    let empty_plan = shell.anchor_rect(PanelAnchor::TopLeft, body, &theme);
    shell.dock_window_plan = vec![("puzzle3d-main-top".to_string(), Rect::new(3.0, body.y + 32.0, 530.0, 800.0))];
    let planned = shell.anchor_rect(PanelAnchor::TopLeft, body, &theme);
    assert_eq!(empty_plan.y, planned.y, "🧭️ the dock's tab bar no longer pushes the panel down");
    assert_eq!(empty_plan.h, planned.h);
}
//#endregion 🧭️AnchorGeometry

//#region 🧭️AnchorState
#[test]
fn panel_anchor_snapshot_reports_the_open_anchor_and_its_active_leaf() {
    let mut shell = fixture_dock_shell();
    shell.toggle_anchor_tab(PanelAnchor::TopLeft, "fixture.workbench");
    let top_left = shell.panel_anchor_snapshot(PanelAnchor::TopLeft);
    assert!(top_left.visible);
    assert_eq!(top_left.size, DEFAULT_PANEL_WIDTH_PX);
    assert_eq!(top_left.active_tab.as_deref(), Some("fixture.workbench"));
    assert_eq!(shell.panel_anchor_snapshot(PanelAnchor::BottomLeft), PanelAnchorSnapshot::default(), "a folded anchor reports nothing");
}

/// 🪟️ **The surface-census law.** What `dumpChrome` publishes beside the hit registry: this frame's
/// dock panes at WINDOW level and every open anchor's active tab at PANEL level, under React's own
/// `panelTabElementId`. A probe comparing two shells reads this instead of the engine's window list,
/// which cannot tell a docked panel from a pane here because a panel IS a window instance to this
/// renderer (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md`).
#[test]
fn the_chrome_surface_census_separates_panes_from_docked_panels() {
    let mut shell = fixture_dock_shell();
    shell.dock_window_plan = vec![("puzzle3d-main-top".to_string(), Rect::new(0.0, 54.0, 600.0, 900.0)), ("puzzle3d-main-perspective".to_string(), Rect::new(600.0, 54.0, 600.0, 900.0))];
    assert_eq!(
        shell.chrome_surface_census(),
        vec![("puzzle3d-main-top".to_string(), "window", "puzzle3d-main-top".to_string()), ("puzzle3d-main-perspective".to_string(), "window", "puzzle3d-main-perspective".to_string())],
        "🪟️ a shell with no open anchor publishes its panes and nothing else"
    );
    shell.toggle_anchor_tab(PanelAnchor::TopLeft, "fixture.workbench");
    let census = shell.chrome_surface_census();
    assert_eq!(census.iter().filter(|(_, level, _)| *level == "panel").map(|(id, _, element)| (id.as_str(), element.as_str())).collect::<Vec<_>>(), vec![("fixture.workbench", "framework.panelTab.fixture.workbench")], "🪟️ the open anchor's ACTIVE TAB is the panel, named the way React names it");
    assert_eq!(census.iter().filter(|(_, level, _)| *level == "window").count(), 2, "🪟️ and opening a panel adds no window");
}

/// 🧭️ Every anchor opens INDEPENDENTLY — the whole point of the packet. The former model could only
/// ever have one left and one right panel open at a time.
#[test]
fn all_eight_anchors_open_independently() {
    let mut shell = fixture_dock_shell();
    for anchor in PanelAnchor::ALL {
        *shell.dock_tabs.tabs_mut(anchor) = vec![DockTabNode::leaf(format!("leaf.{}", anchor.as_str()), "Leaf", "circle-dot", 0)];
    }
    for anchor in PanelAnchor::ALL {
        shell.toggle_anchor(anchor);
    }
    assert_eq!(shell.open_anchors().len(), 8);
    shell.toggle_anchor(PanelAnchor::TopLeft);
    assert!(!shell.anchor_open(PanelAnchor::TopLeft));
    assert_eq!(shell.open_anchors().len(), 7, "folding one anchor leaves the other seven alone");
}

/// 🧭️ An anchor carrying no tab never opens — no empty glass box over the canvas.
#[test]
fn an_empty_anchor_never_opens() {
    let mut shell = fixture_dock_shell();
    assert!(shell.dock_tabs.tabs(PanelAnchor::LeftMiddle).is_empty());
    shell.anchor_state_mut(PanelAnchor::LeftMiddle).visible = true;
    assert!(!shell.anchor_open(PanelAnchor::LeftMiddle));
    assert!(!shell.open_anchors().contains(&PanelAnchor::LeftMiddle));
}

/// 🧭️ `left_panel_open`/`right_panel_open` survive only as COLUMN projections of their two corner
/// anchors — the contract the overlay safe area and the `⌘️B` chords still read.
#[test]
fn column_projections_follow_their_corner_anchors() {
    let mut shell = fixture_dock_shell();
    assert!(!shell.left_panel_open() && !shell.right_panel_open());
    shell.toggle_anchor(PanelAnchor::BottomLeft);
    assert!(shell.left_panel_open(), "the left column is open while EITHER of its corners is");
    assert!(!shell.right_panel_open());
    shell.toggle_column(true);
    assert!(!shell.left_panel_open(), "the column chord folds every anchor in that column");
    shell.toggle_column(true);
    assert!(shell.anchor_open(PanelAnchor::TopLeft), "reopening a folded column reopens its top corner");
}

#[test]
fn reconcile_path_drills_a_branch_to_its_first_leaf_and_drops_vanished_tabs() {
    let mut dock = ShellDock::default();
    *dock.tabs_mut(PanelAnchor::BottomMiddle) = vec![DockTabNode::branch("branch", "Branch", "wrench", 0, vec![DockTabNode::leaf("leaf-a", "A", "circle-dot", 0), DockTabNode::leaf("leaf-b", "B", "circle-dot", 1)])];
    assert_eq!(dock.reconcile_path(PanelAnchor::BottomMiddle, &["branch".to_string()]), vec!["branch".to_string(), "leaf-a".to_string()]);
    assert_eq!(dock.reconcile_path(PanelAnchor::BottomMiddle, &["branch".to_string(), "leaf-b".to_string()]), vec!["branch".to_string(), "leaf-b".to_string()]);
    assert_eq!(dock.reconcile_path(PanelAnchor::BottomMiddle, &["gone".to_string()]), dock.default_path(PanelAnchor::BottomMiddle));
}
//#endregion 🧭️AnchorState

//#region 🗄️DockOverrideRoundTrip
/// 🗄️ `apply_dock_skeleton` REARRANGES the default dock, never reconstructs it: a tab moved to another
/// anchor lands there, and a default tab the skeleton never mentions is appended at its default home.
#[test]
fn apply_dock_skeleton_moves_mentioned_tabs_and_appends_unmentioned_defaults() {
    let shell = fixture_dock_shell();
    let default_dock = shell.default_dock();
    let mut skeleton = dock_skeleton_of(&default_dock);
    skeleton.anchors.insert("top-left".to_string(), Vec::new());
    skeleton.anchors.insert("left-middle".to_string(), vec![DockTabSkeleton { id: "fixture.workbench".into(), children: None, trees: Some(vec![dock_leaf_tree_unit_id("fixture.workbench")]) }]);
    let rearranged = apply_dock_skeleton(&default_dock, Some(&skeleton));
    assert_eq!(rearranged.locate("fixture.workbench").map(|(anchor, _)| anchor), Some(PanelAnchor::LeftMiddle));
    assert!(rearranged.tabs(PanelAnchor::TopLeft).is_empty());
    assert_eq!(
        rearranged.tabs(PanelAnchor::TopRight).iter().map(|tab| tab.id.as_str()).collect::<Vec<_>>(),
        default_dock.tabs(PanelAnchor::TopRight).iter().map(|tab| tab.id.as_str()).collect::<Vec<_>>(),
        "an untouched anchor keeps its default row"
    );
}

/// 🗄️ A skeleton that mentions nothing is the identity, and an id the default no longer declares is
/// dropped rather than resurrected.
#[test]
fn apply_dock_skeleton_is_identity_for_none_and_drops_unknown_ids() {
    let shell = fixture_dock_shell();
    let default_dock = shell.default_dock();
    assert_eq!(apply_dock_skeleton(&default_dock, None), default_dock);
    let mut stale = DockSkeleton::default();
    stale.anchors.insert("top-middle".to_string(), vec![DockTabSkeleton { id: "tab.that.no.longer.exists".into(), children: None, trees: None }]);
    let applied = apply_dock_skeleton(&default_dock, Some(&stale));
    assert!(applied.tabs(PanelAnchor::TopMiddle).is_empty());
    assert_eq!(applied.tabs(PanelAnchor::TopLeft).len(), default_dock.tabs(PanelAnchor::TopLeft).len(), "nothing mentioned it, so the default row is re-seeded");
}

/// 🎯️ A tab dragged to another anchor is a pure `move_tab_in_dock` transform, and dropping a subtree
/// into itself is refused.
#[test]
fn move_tab_in_dock_reanchors_a_tab_and_refuses_self_drops() {
    let shell = fixture_dock_shell();
    let mut dock = shell.default_dock();
    let target = DockTabMoveTarget { anchor: PanelAnchor::RightMiddle, parent_path: Vec::new(), index: 0 };
    assert!(move_tab_in_dock(&mut dock, "fixture.workbench", &target));
    assert_eq!(dock.locate("fixture.workbench").map(|(anchor, _)| anchor), Some(PanelAnchor::RightMiddle));
    assert!(dock.tabs(PanelAnchor::TopLeft).is_empty());
    let into_itself = DockTabMoveTarget { anchor: PanelAnchor::BottomMiddle, parent_path: vec![FRAMEWORK_CATEGORY_COMMAND_ID.to_string()], index: 0 };
    let before = dock.clone();
    assert!(!move_tab_in_dock(&mut dock, FRAMEWORK_CATEGORY_COMMAND_ID, &into_itself));
    assert_eq!(dock, before, "a refused move leaves the dock byte-identical");
}

/// 🗄️ The override ROUND TRIP through the exact `semio.os.config` document React's `DockLayoutStore`
/// owns. It runs over an in-memory document rather than the process-global store on purpose: every
/// `prefs_set` is a read-modify-write of that ONE document, so a parallel test writing an unrelated
/// `preferences` key clobbers whatever dock layer was written a microsecond earlier. The layer rules —
/// per-app precedence, `None` removal, the version gate — are what this pins, and they are the whole
/// of what a renderer switch depends on.
///
/// 🗄️ A per-app layer wins over the shared `os` one; removing it falls back to `os`; removing both
/// leaves nothing — `DockLayoutStore::getSnapshot`/`save`/`saveOs` exactly.
///
/// 🧭️ And moving the tab back to its computed default drops the override entirely, so nothing persists.
#[test]
fn dock_override_round_trips_through_the_os_shell_config_document() {
    let mut shell = fixture_dock_shell();
    let target = DockTabMoveTarget { anchor: PanelAnchor::TopMiddle, parent_path: Vec::new(), index: 0 };
    assert!(shell.move_dock_tab("fixture.details", &target));
    let moved = shell.dock_override.clone().expect("a rearranged dock stops matching its default");

    let mut config = serde_json::json!({ "version": 1, "preferences": {} });
    write_os_shell_config_layer_in(&mut config, "dockLayouts", Some("fixture-app"), Some(&moved));
    assert_eq!(config["dockLayouts"]["apps"]["fixture-app"]["version"], serde_json::json!(3));
    assert_eq!(config["dockLayouts"]["apps"]["fixture-app"]["anchors"]["top-middle"][0]["id"], serde_json::json!("fixture.details"));

    let loaded: DockSkeleton = read_os_shell_config_layer(&config, "dockLayouts", Some("fixture-app")).expect("the per-app layer reads back");
    assert!(dock_skeletons_equal(Some(&loaded), Some(&moved)));
    let reloaded = apply_dock_skeleton(&shell.default_dock(), Some(&loaded));
    assert_eq!(reloaded.locate("fixture.details").map(|(anchor, _)| anchor), Some(PanelAnchor::TopMiddle));
    assert_eq!(dock_skeleton_of(&reloaded), dock_skeleton_of(&shell.dock_tabs));

    let default_skeleton = dock_skeleton_of(&shell.default_dock());
    write_os_shell_config_layer_in(&mut config, "dockLayouts", None, Some(&default_skeleton));
    let still_app: DockSkeleton = read_os_shell_config_layer(&config, "dockLayouts", Some("fixture-app")).expect("per-app layer still wins");
    assert!(dock_skeletons_equal(Some(&still_app), Some(&moved)));
    write_os_shell_config_layer_in::<DockSkeleton>(&mut config, "dockLayouts", Some("fixture-app"), None);
    let os_layer: DockSkeleton = read_os_shell_config_layer(&config, "dockLayouts", Some("fixture-app")).expect("falls back to the os layer");
    assert!(dock_skeletons_equal(Some(&os_layer), Some(&default_skeleton)));
    write_os_shell_config_layer_in::<DockSkeleton>(&mut config, "dockLayouts", None, None);
    assert!(read_os_shell_config_layer::<DockSkeleton>(&config, "dockLayouts", Some("fixture-app")).is_none());

    let back = DockTabMoveTarget { anchor: PanelAnchor::TopRight, parent_path: Vec::new(), index: 0 };
    assert!(shell.move_dock_tab("fixture.details", &back));
    assert!(shell.dock_override.is_none(), "a dock back at its computed default persists no override");
    eprintln!("[DEBUG] dock override round-tripped through semio.os.config's dockLayouts layers and cleared at the default");
}

/// 🗄️ The per-anchor chrome round trip: visibility, size and active path survive a reload, and an
/// anchor absent from the persisted document reloads folded.
#[test]
fn dock_ui_state_round_trips_visibility_size_and_path() {
    let mut shell = fixture_dock_shell();
    shell.toggle_anchor_tab(PanelAnchor::BottomRight, "fixture.settings");
    shell.anchor_state_mut(PanelAnchor::BottomRight).size = 412.0;
    let snapshot = shell.dock_ui_snapshot();
    assert_eq!(snapshot.version, 3);
    assert_eq!(snapshot.anchors.get("bottom-right").and_then(|entry| entry.size), Some(412.0));
    assert!(snapshot.anchors.get("top-left").is_none(), "a default anchor stores nothing");

    let mut config = serde_json::json!({ "version": 1, "preferences": {} });
    write_os_shell_config_layer_in(&mut config, "dockUi", Some("fixture-app"), Some(&snapshot));
    let loaded: DockUiState = read_os_shell_config_layer(&config, "dockUi", Some("fixture-app")).expect("dockUi layer reads back");
    assert_eq!(loaded, snapshot);

    let mut target = fixture_dock_shell();
    target.apply_dock_ui(&loaded);
    assert!(target.anchor_open(PanelAnchor::BottomRight));
    assert_eq!(target.anchor_state(PanelAnchor::BottomRight).size, 412.0);
    assert_eq!(target.anchor_state(PanelAnchor::BottomRight).active_tab(), Some("fixture.settings"));
    assert!(!target.anchor_open(PanelAnchor::TopLeft));
}

/// 🗄️ `persist_dock_ui_if_changed`'s dirty-check: the first call after a real change persists, and a
/// second call with nothing changed must not write again — the render-loop hook that replaces patching
/// every `ui.panelToggle.*` call site. Asserted against the owned present-state snapshot rather than the
/// shared store, for the same read-modify-write reason the override test above documents.
///
/// Nothing changed since: a wrongly-unconditional write would be observable as a new snapshot object
/// replacing this deliberately-wrong marker.
#[test]
fn persist_dock_ui_if_changed_is_idempotent_when_nothing_changed() {
    let mut shell = fixture_dock_shell();
    shell.chrome_present.last_persisted_dock_ui = Some(shell.dock_ui_snapshot());
    shell.toggle_anchor(PanelAnchor::TopLeft);
    shell.persist_dock_ui_if_changed();
    let persisted = shell.chrome_present.last_persisted_dock_ui.clone().expect("first call must persist");
    assert!(persisted.anchors.get("top-left").and_then(|entry| entry.visible).unwrap_or(false));

    shell.chrome_present.last_persisted_dock_ui = Some(persisted.clone());
    shell.persist_dock_ui_if_changed();
    assert_eq!(shell.chrome_present.last_persisted_dock_ui.as_ref(), Some(&persisted), "unchanged state must not re-persist");
    assert_eq!(shell.chrome_present.last_persisted_dock_skeleton, Some(shell.dock_override.clone()));
}
//#endregion 🗄️DockOverrideRoundTrip

/// 🎨️ `build_settings_theme_ui`'s reachability contract: a select node listing the built-in themes
/// plus any saved custom ones, a reset button always present, and a delete button gated strictly on
/// the active theme id being a `"custom."`-prefixed one (mirrors React's `host.themeId.startsWith(
/// "custom.")` gate on the same button, `ui/js/react/index.tsx:9489`).
#[test]
fn build_settings_theme_ui_lists_builtins_and_gates_delete_on_custom_theme() {
    let mut state = fresh_state();
    state.chrome_build.preferences.theme_id = "semio".to_string();
    let builtin = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &state.build_settings_theme_ui()).expect("built-in theme editor projection");
    assert!(builtin.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.select")), "the canonical tree publishes the theme picker");
    assert!(builtin.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.reset")), "the canonical tree publishes Reset");
    assert!(!builtin.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.delete")), "a built-in theme publishes no Delete control");
    drop(builtin);
    while !ui_contract::close_ui_value_page_one() {}

    state.chrome_build.preferences.theme_id = "custom.wp-audit-test".to_string();
    let custom = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &state.build_settings_theme_ui()).expect("custom theme editor projection");
    assert!(custom.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.reset")), "a custom theme retains Reset");
    assert!(custom.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.delete")), "a custom theme publishes Delete");
    drop(custom);
    while !ui_contract::close_ui_value_page_one() {}
}

//#region ⚙️SettingsBranchAndShellOwnedLeaves
/// ⚙️ The bottom-right Settings BRANCH: one toggle carrying the app's own Settings-group tabs first and
/// then the five framework leaves, exactly React's
/// `integrateAppSettingsPanelTabsIntoFrameworkBranch(frameworkSettingsTab, settingsBottomRightTabs)`
/// child order — plus the Marketplace leaf and the framework-owned History leaf beside it, never
/// nested inside it.
#[test]
fn the_settings_branch_nests_the_five_framework_leaves_after_the_apps_own() {
    let fixture = default_dock_fixture();
    let shell = fixture_dock_shell();
    let dock = shell.default_dock();
    let roots: Vec<&str> = dock.tabs(PanelAnchor::BottomRight).iter().map(|tab| tab.id.as_str()).collect();
    assert_eq!(roots, vec![FRAMEWORK_SETTINGS_PANEL_ID, FRAMEWORK_MARKETPLACE_TAB_ID, FRAMEWORK_TASK_MANAGER_PANEL_ID, FRAMEWORK_PANEL_TAB_HISTORY_ID], "⚙️ bottom-right carries Settings, Marketplace, Task Manager, then History");
    let branch = dock.tabs(PanelAnchor::BottomRight).first().expect("the Settings branch");
    assert!(branch.is_branch(), "⚙️ Settings is a branch, not a leaf");
    let children: Vec<&str> = branch.children.iter().map(|child| child.id.as_str()).collect();
    assert_eq!(children.first(), Some(&"fixture.settings"), "⚙️ the app's own Settings tabs come first (document-first child order)");
    let declared: Vec<&str> = fixture["settingsBranch"]["children"].as_array().expect("fixture settings children").iter().map(|id| id.as_str().expect("child id")).collect();
    assert_eq!(&children[1..], declared.as_slice(), "⚙️ then React's five framework leaves, in React's order");
    for (index, child) in branch.children.iter().enumerate() {
        assert_eq!(child.order, index as i32, "⚙️ children are re-ordered past the app's, React's own `+ appSettingsTabs.length`");
    }
}

/// 🧾️ Every SHELL-OWNED leaf the dock declares has a body, and every body the shell publishes is a
/// leaf the dock declares — the assembler's own closure property. A leaf that is dockable but paints
/// nothing (which is exactly what the Settings branch's children were before this packet) fails here.
#[test]
fn every_shell_owned_leaf_projects_into_retained_records() {
    let shell = fixture_dock_shell();
    let dock = shell.default_dock();
    for tab_id in shell.shell_owned_panel_leaves() {
        if tab_id == crate::hub_connection::FRAMEWORK_HUB_PANEL_ID {
            assert!(dock.locate(&tab_id).is_none(), "🧾️ Hub is the route-owned overlay React opens from its footer/command, never a dock substitute");
        } else {
            assert!(dock.locate(&tab_id).is_some(), "🧾️ {tab_id} publishes a body but no anchor declares it");
        }
        let node = match tab_id.as_str() {
            FRAMEWORK_SETTINGS_GENERAL_TAB_ID => shell.build_settings_general_ui(),
            FRAMEWORK_SETTINGS_THEME_TAB_ID => shell.build_settings_theme_ui(),
            FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID => shell.build_settings_keybindings_ui(),
            FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID => shell.build_settings_default_apps_ui(),
            FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID => shell.build_settings_conflicts_ui(),
            FRAMEWORK_MARKETPLACE_TAB_ID => shell.build_marketplace_ui(),
            FRAMEWORK_SYNC_PANEL_TAB_ID => shell.build_sync_attach_ui(),
            FRAMEWORK_TASK_MANAGER_PANEL_ID => shell.build_task_manager_ui(),
            FRAMEWORK_CHAT_PANEL_ID => shell.build_agent_chat_ui(),
            other => shell.build_command_category_ui(other.trim_start_matches(FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX)),
        };
        let records = panel_ui_records(&tab_id, &node).unwrap_or_else(|error| panic!("🧾️ {tab_id} must project: {error}"));
        assert!(!records.is_empty(), "🧾️ {tab_id} projected an empty document");
        assert_eq!(records[0].id, ui_contract::UiNodeId(1), "🧾️ the root record is always id 1 (the published assembly identity's root)");
        for (index, record) in records.iter().enumerate() {
            assert_eq!(record.id, ui_contract::UiNodeId(index as u64 + 1), "🧾️ records are parent-first and densely numbered");
        }
    }
    eprintln!("[DEBUG] wgpu shell-owned panel leaves all project into retained records");
}

/// 🧾️ The assembler refuses a node it cannot project instead of silently dropping it, so a builder
/// that grows a `Tree`/`Image`/`ComponentScene` fails at publication rather than painting a hole.
#[test]
fn the_panel_assembler_refuses_an_unprojectable_node() {
    let node = UiNode::Image(ui_wgpu::wgpu::UiImageNode { id: "x".into(), src: "y".into(), alt: None, presence: UiPresence::default(), menu: None });
    let error = panel_ui_records("framework.settings.general", &node).expect_err("an image is not projectable by this assembler");
    assert!(error.contains("image"), "the refusal names the variant: {error}");
}
//#endregion ⚙️SettingsBranchAndShellOwnedLeaves

//#region ⌨️RemappableKeybindings
/// ⌨️ An override REPLACES a row's chord: the new chord resolves to that row's verb and is reserved,
/// and the default it replaced stops being the shell's — React's `composeControlKeybindings` (the
/// override wins unconditionally) plus `reservedShellChordsV1` over the same resolved table.
#[test]
fn a_keybinding_override_remaps_a_shell_chord_and_frees_its_default() {
    let mut overrides = HashMap::new();
    overrides.insert("ui.search.toggle".to_string(), "ctrl+j".to_string());
    let table = shell_shortcut_table(&overrides);
    let accelerator = PointerModifiers { ctrl: true, ..PointerModifiers::default() };
    let remapped = ui_wgpu::wgpu::KeyAction::Char("j".into());
    let default = ui_wgpu::wgpu::KeyAction::Char("p".into());
    assert_eq!(shell_shortcut_for_in(&table, &remapped, &accelerator), Some(ShellShortcut::ToggleSearch), "⌨️ the override's chord opens the palette");
    assert!(is_reserved_shell_chord_in(&table, &remapped, &accelerator), "⌨️ and is reserved at its new spelling");
    assert_eq!(shell_shortcut_for_in(&table, &default, &accelerator), None, "⌨️ `mod+p` is no longer the shell's");
    assert!(!is_reserved_shell_chord_in(&table, &default, &accelerator), "⌨️ so an app may claim it");
    assert_eq!(shell_shortcut_for(&default, &accelerator), Some(ShellShortcut::ToggleSearch), "⌨️ the DEFAULT table is untouched");
}

/// ⌨️ An override carrying no parsable chord is dropped rather than blanking the row — React's
/// `parseUiKeybindingOverrides` gate.
#[test]
fn a_blank_keybinding_override_keeps_the_declared_default() {
    let mut overrides = HashMap::new();
    overrides.insert("ui.find.toggle".to_string(), "  ,  ".to_string());
    let table = shell_shortcut_table(&overrides);
    let row = table.iter().find(|(id, _)| id == "ui.find.toggle").expect("the find row");
    assert_eq!(row.1, "mod+f", "⌨️ a chordless override never retires the default");
}

/// ⌨️ The conflict marker is React's exactly: last writer of a first chord owns it, so the EARLIER row
/// of a colliding pair is the flagged one and the later is clean.
#[test]
fn keybinding_rows_flag_the_earlier_owner_of_a_shared_chord() {
    let mut overrides = HashMap::new();
    overrides.insert("ui.find.toggle".to_string(), "mod+p".to_string());
    let rows = shell_keybinding_rows(&overrides);
    let conflicted: Vec<&str> = rows.iter().filter(|row| row.conflicted).map(|row| row.control_id.as_str()).collect();
    assert_eq!(conflicted.len(), 1, "⌨️ exactly one of a colliding pair is marked, never both: {conflicted:?}");
    assert!(rows.iter().any(|row| row.control_id == "ui.find.toggle" && row.overridden), "⌨️ the remapped row reports itself overridden");
    assert!(rows.windows(2).all(|pair| pair[0].control_id <= pair[1].control_id), "⌨️ rows are control-id sorted, React's own `localeCompare` order");
}

/// ⌨️ An armed capture turns the next key event into that row's override and consumes it — the chord
/// spelling is React's `chordFromKeyboardEvent` order (`ctrl, meta, alt, shift`, then the key).
#[test]
fn an_armed_capture_writes_the_next_chord_as_an_override() {
    let mut shell = fresh_state();
    shell.keybinding_capture_control_id = Some("ui.nav.up".into());
    let modifiers = PointerModifiers { ctrl: true, alt: true, shift: true, ..PointerModifiers::default() };
    assert!(shell.capture_keybinding_chord(&ui_wgpu::wgpu::KeyAction::ArrowUp, &modifiers), "⌨️ the capture consumes the event");
    assert_eq!(shell.chrome_build.preferences.keybinding_overrides.get("ui.nav.up").map(String::as_str), Some("ctrl+alt+shift+arrowup"));
    assert_eq!(shell.keybinding_capture_control_id, None, "⌨️ one gesture, one write, then disarmed");
    shell.keybinding_capture_control_id = Some("ui.nav.back".into());
    assert!(shell.capture_keybinding_chord(&ui_wgpu::wgpu::KeyAction::Escape, &modifiers), "⌨️ Escape cancels and still consumes");
    assert!(!shell.chrome_build.preferences.keybinding_overrides.contains_key("ui.nav.back"), "⌨️ a cancelled capture writes nothing");
}

/// ⌨️ Every shortcut row the Keybindings leaf paints is addressable and every chord it prints is the
/// one in force — the rows W1c's shortcut list and this leaf must agree on.
#[test]
fn the_keybindings_leaf_paints_one_row_per_shortcut() {
    let mut shell = fresh_state();
    shell.chrome_build.preferences.keybinding_overrides.insert("ui.search.toggle".into(), "ctrl+j".into());
    let UiNode::Stack(panel) = shell.build_settings_keybindings_ui() else { panic!("expected a stack root") };
    let UiNode::Section(section) = panel.children.first().expect("the keybindings section") else { panic!("expected a section") };
    assert_eq!(section.children.len(), SHELL_SHORTCUT_ROWS.len(), "⌨️ one row per declared shortcut");
    let ids: Vec<String> = section.children.iter().filter_map(|node| if let UiNode::Stack(row) = node { row.id.clone() } else { None }).collect();
    assert!(ids.contains(&"framework.settings.keybindings.ui.search.toggle".to_string()));
    let overridden_row = section.children.iter().find_map(|node| if let UiNode::Stack(row) = node { (row.id.as_deref() == Some("framework.settings.keybindings.ui.search.toggle")).then_some(row) } else { None }).expect("the remapped row");
    assert_eq!(overridden_row.children.iter().filter(|node| matches!(node, UiNode::Button(_))).count(), 2, "⌨️ a remapped row offers capture AND reset");
}
//#endregion ⌨️RemappableKeybindings

//#region 📐️ContentHug
/// 📐️ A measured panel takes exactly its content's height from its own bonded edge, and never more
/// than React's `calc(100% - spacing*2)` clamp — the divergence `📓️w1h`'s gap 1 recorded.
#[test]
fn a_panel_hugs_its_measured_content_up_to_the_react_clamp() {
    let theme = Theme::default();
    let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
    let inset = theme.panel_inset;
    let band = body.h - inset * 2.0;
    let top = anchor_panel_rect(PanelAnchor::TopLeft, DEFAULT_PANEL_WIDTH_PX, Some(180.0), 1, 0, body, &theme);
    assert_eq!(top.h, 180.0, "📐️ a top anchor takes its content's height");
    assert_eq!(top.y, body.y + inset, "📐️ and grows DOWN from its own inset");
    let bottom = anchor_panel_rect(PanelAnchor::BottomLeft, DEFAULT_PANEL_WIDTH_PX, Some(180.0), 1, 0, body, &theme);
    assert_eq!(bottom.h, 180.0);
    assert_eq!(bottom.y + bottom.h, body.y + inset + band, "📐️ a bottom anchor grows UP from the body's bottom inset");
    let middle = anchor_panel_rect(PanelAnchor::LeftMiddle, DEFAULT_PANEL_WIDTH_PX, Some(180.0), 1, 0, body, &theme);
    assert!((middle.y + middle.h * 0.5 - (body.y + inset + band * 0.5)).abs() < 0.01, "📐️ a middle anchor centres on its band");
    let oversized = anchor_panel_rect(PanelAnchor::TopLeft, DEFAULT_PANEL_WIDTH_PX, Some(5000.0), 1, 0, body, &theme);
    assert_eq!(oversized.h, band, "📐️ the clamp is React's own maxHeight, never the content");
    let unmeasured = anchor_panel_rect(PanelAnchor::TopLeft, DEFAULT_PANEL_WIDTH_PX, None, 1, 0, body, &theme);
    assert_eq!(unmeasured.h, band, "📐️ an unmeasured surface keeps its full band rather than collapsing");
}

/// 📐️ Two hugging anchors in one column still cannot overlap: each is clamped to its own share.
#[test]
fn two_hugging_anchors_in_one_column_never_overlap() {
    let theme = Theme::default();
    let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
    let top = anchor_panel_rect(PanelAnchor::TopLeft, DEFAULT_PANEL_WIDTH_PX, Some(5000.0), 2, 0, body, &theme);
    let bottom = anchor_panel_rect(PanelAnchor::BottomLeft, DEFAULT_PANEL_WIDTH_PX, Some(5000.0), 2, 1, body, &theme);
    assert!(top.y + top.h <= bottom.y + 0.01, "📐️ {top:?} must not reach into {bottom:?}");
}
//#endregion 📐️ContentHug

//#region 📱️MobileFlattening
/// 📱️ Below React's `UI_MOBILE_MEDIA_QUERY` breakpoint the eight anchors flatten into ONE panel whose
/// tab list is every anchor's root tabs in `ANCHORS` order — React's `mobilePanelTabs`, which flattens
/// `defaultDock` and appends the synthetic app leaf only when the navbar has clusters with no room.
#[test]
fn the_mobile_panel_flattens_every_anchor_in_anchor_order() {
    let fixture = default_dock_fixture();
    let mut shell = fixture_dock_shell();
    shell.screen_w = 1280.0;
    assert!(!shell.mobile_panel_active(), "📱️ a desktop viewport keeps the eight anchors");
    shell.screen_w = crate::dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX;
    assert!(shell.mobile_panel_active(), "📱️ the breakpoint itself is mobile (React's `max-width: 767px` is inclusive)");
    let ids: Vec<String> = shell.mobile_panel_tabs().into_iter().map(|tab| tab.id).collect();
    let expected: Vec<String> = fixture["mobilePanelTabs"]["ids"].as_array().expect("fixture mobile ids").iter().map(|id| id.as_str().expect("id").to_string()).collect();
    assert_eq!(ids, expected, "📱️ the flattened order is the fixture's");
    eprintln!("[DEBUG] wgpu mobile panel flattened {} tabs across all 8 anchors", ids.len());
}

/// 📱️ The mobile panel's remembered path reconciles onto the flattened list and drills a branch to its
/// first leaf, so selecting the Settings branch on mobile lands on a real body.
#[test]
fn the_mobile_panel_path_drills_a_branch_to_its_first_leaf() {
    let shell = fixture_dock_shell();
    let tabs = shell.mobile_panel_tabs();
    let path = mobile_panel_reconcile_path(&tabs, &[FRAMEWORK_SETTINGS_PANEL_ID.to_string()]);
    assert_eq!(path.first().map(String::as_str), Some(FRAMEWORK_SETTINGS_PANEL_ID));
    assert_eq!(path.last().map(String::as_str), Some("fixture.settings"), "📱️ drilled to the branch's first leaf");
    let vanished = mobile_panel_reconcile_path(&tabs, &["gone".to_string()]);
    assert_eq!(vanished.first().map(String::as_str), tabs.first().map(|tab| tab.id.as_str()), "📱️ a vanished path falls back to the first tab");
}
//#endregion 📱️MobileFlattening

//#region 🎛️StagedCommandForm
/// 🎛️ `expanded_command_id`'s READER — the gap `📓️w1c` recorded. The expanded command's staged form is
/// a section of its own carrying one control per argument plus Execute and Reset, and that command is
/// filtered out of the list section below it (React's `buildCommandCategoryTree`).
#[test]
fn the_command_dock_opens_the_expanded_commands_staged_form() {
    let mut shell = fresh_state();
    let entry = shell.resolved_commands().into_iter().find(|entry| entry.definition.in_palette && !entry.definition.args.is_empty()).expect("an arg-carrying os command");
    let key = command_address_stable_key(&entry.address);
    let category = entry.definition.category.clone();
    shell.expanded_command_id = Some(key.clone());
    let UiNode::Stack(panel) = shell.build_command_category_ui(&category) else { panic!("expected a stack root") };
    let UiNode::Section(form) = panel.children.first().expect("the form section") else { panic!("the expanded form is the FIRST section, so a bottom anchor paints it above the list") };
    assert_eq!(form.id, format!("command.category.{category}.form"));
    let buttons: Vec<&str> = form.children.iter().filter_map(|node| if let UiNode::Button(button) = node { button.id.as_deref() } else { None }).collect();
    assert_eq!(buttons.len(), 2, "🎛️ Execute and Reset, React's own two section actions");
    assert!(buttons[0].ends_with("-execute") && buttons[1].ends_with("-reset"));
    let arg_rows = form.children.iter().filter(|node| matches!(node, UiNode::Field(_))).count();
    assert_eq!(arg_rows, entry.definition.args.len(), "🎛️ one control per declared argument");
    let list = panel.children.iter().find_map(|node| if let UiNode::Section(section) = node { (section.id == "command.category.list").then_some(section) } else { None });
    let listed: Vec<String> = list.map(|section| section.children.iter().filter_map(|node| if let UiNode::Select(select) = node { Some(select.id.clone()) } else { None }).collect()).unwrap_or_default();
    assert!(!listed.contains(&format!("shell.commands.{}", entry.definition.id)), "🎛️ the expanded command is not also listed");
}

/// 🎛️ An Execute is DISABLED while a required argument is unstaged — React's `missing.length > 0` gate.
#[test]
fn execute_is_disabled_until_every_required_argument_is_staged() {
    let mut shell = fresh_state();
    let entry = shell.resolved_commands().into_iter().find(|entry| entry.definition.in_palette && entry.definition.args.iter().any(|arg| arg.required)).map(|entry| (command_address_stable_key(&entry.address), entry.definition.category.clone(), entry.definition.args.iter().find(|arg| arg.required).expect("a required arg").id.clone()));
    let Some((key, category, arg_id)) = entry else {
        eprintln!("[DEBUG] no os command declares a required argument — gate exercised by the assembler only");
        return;
    };
    shell.expanded_command_id = Some(key.clone());
    let disabled = |shell: &ShellState| {
        let UiNode::Stack(panel) = shell.build_command_category_ui(&category) else { panic!("stack root") };
        let UiNode::Section(form) = panel.children.first().expect("form") else { panic!("form section") };
        form.children.iter().any(|node| matches!(node, UiNode::Button(button) if button.id.as_deref().is_some_and(|id| id.ends_with("-execute")) && matches!(button.presence.state, ui_wgpu::wgpu::component::ui::UiState::Disabled)))
    };
    assert!(disabled(&shell), "🎛️ unstaged required argument blocks Execute");
    shell.staged_command_args.entry(key).or_default().insert(arg_id, serde_json::Value::String("x".into()));
    assert!(!disabled(&shell), "🎛️ staging it enables Execute");
}
//#endregion 🎛️StagedCommandForm

//#region 💬️AgentChatTranscript
/// 🌉️ Opens the bridge the way a transport does — a `Welcome` frame — so the panel is in its
/// connected state and the composer is live.
fn welcomed_shell() -> ShellState {
    let mut shell = fresh_state();
    let welcome = crate::agent_bridge::GatewayToShell::Welcome { bridge_version: crate::agent_bridge::BRIDGE_VERSION, connection: "connection-1".into(), principal: "agent:local".into() };
    shell.apply_agent_bridge_frame(&welcome.encode()).expect("a Welcome frame decodes");
    shell
}

/// 💬️ Every text line the chat panel's own subtree paints, in order — the feed's readable content.
fn chat_panel_lines(node: &UiNode, out: &mut Vec<String>) {
    match node {
        UiNode::Text(text) => out.push(text.value.as_str().to_string()),
        UiNode::Stack(stack) => stack.children.iter().for_each(|child| chat_panel_lines(child, out)),
        _ => {}
    }
}

/// 🆔️ The ids of the conversation rows inside the panel's own FEED — React's
/// `<ol id="framework.chat.feed">`, which the wgpu body wraps its rows in so the transcript is one
/// addressable, labelled thing rather than rows loose beside the composer.
fn chat_entry_ids(root: &UiNode) -> Vec<String> {
    let UiNode::Stack(panel) = root else { panic!("💬️ the chat panel root is a stack") };
    let feed = panel
        .children
        .iter()
        .find_map(|child| match child {
            UiNode::Stack(stack) if stack.id.as_deref() == Some(FRAMEWORK_CHAT_FEED_ID) => Some(stack),
            _ => None,
        })
        .expect("💬️ the chat panel publishes its feed");
    feed.children.iter().filter_map(|child| if let UiNode::Stack(stack) = child { stack.id.clone() } else { None }).collect()
}

fn chat_button_action(root: &UiNode, id: &str) -> Option<ActionDescriptor> {
    match root {
        UiNode::Button(button) if button.id.as_deref() == Some(id) => Some(button.action.clone()),
        UiNode::Stack(stack) => stack.children.iter().find_map(|child| chat_button_action(child, id)),
        _ => None,
    }
}

/// 💬️ LAW: a bridge `AgentToolCall` frame — the one the gateway emits from its own `tools/call`
/// dispatch — becomes a conversation row in the wgpu panel model, and the matching `AgentToolResult`
/// folds its summary into that SAME row rather than opening a second one. This is the parity the wgpu
/// renderer owed React's `💬️AgentChatPanel` feed; nothing here is generated locally.
#[test]
fn a_bridge_tool_call_frame_becomes_a_conversation_row_in_the_chat_panel() {
    let mut shell = welcomed_shell();
    assert!(chat_entry_ids(&shell.build_agent_chat_ui()).is_empty(), "💬️ no bridge activity, no rows");

    let call = crate::agent_bridge::GatewayToShell::AgentToolCall { invocation_id: "inv_7".into(), tool_name: "artifact_open".into(), arguments: "{\"artifactId\":\"doc-1\"}".into() };
    shell.apply_agent_bridge_frame(&call.encode()).expect("an AgentToolCall frame decodes");
    let running = shell.build_agent_chat_ui();
    assert_eq!(chat_entry_ids(&running), vec!["framework.chat.entry.toolCall.inv_7".to_string()], "💬️ the tool call is one addressable row");
    let mut lines = Vec::new();
    chat_panel_lines(&running, &mut lines);
    assert!(lines.contains(&"Tool call".to_string()), "💬️ the translated role noun leads the row: {lines:?}");
    assert!(lines.contains(&"Running…".to_string()), "💬️ a call with no result yet is running: {lines:?}");
    assert!(lines.contains(&"artifact_open".to_string()), "💬️ the real tool name: {lines:?}");
    assert!(lines.contains(&"{\"artifactId\":\"doc-1\"}".to_string()), "💬️ and its real arguments: {lines:?}");
    assert!(panel_ui_records(FRAMEWORK_CHAT_PANEL_ID, &running).is_ok(), "💬️ the conversation projects as publishable panel records");

    let result = crate::agent_bridge::GatewayToShell::AgentToolResult { invocation_id: "inv_7".into(), tool_name: "artifact_open".into(), ok: true, summary: "opened doc-1".into() };
    shell.apply_agent_bridge_frame(&result.encode()).expect("an AgentToolResult frame decodes");
    let settled = shell.build_agent_chat_ui();
    assert_eq!(chat_entry_ids(&settled), vec!["framework.chat.entry.toolCall.inv_7".to_string()], "💬️ one row per invocation, never two");
    let mut settled_lines = Vec::new();
    chat_panel_lines(&settled, &mut settled_lines);
    assert!(settled_lines.contains(&"Done".to_string()), "💬️ the state chip settles: {settled_lines:?}");
    assert!(settled_lines.contains(&"Result: opened doc-1".to_string()), "💬️ the summary folds into the call's own row: {settled_lines:?}");
}

/// 🛑️ The actual retained button carries the gateway's invocation id into Shell dispatch, sends one
/// cancel frame, retires itself while cancellation is pending, and trusts only the result to settle.
#[test]
fn the_chat_cancel_control_drives_the_shared_invocation_lifecycle_end_to_end() {
    let fixture: Value = serde_json::from_str(include_str!("../../../🔗️AgentBridge/🧫️fixtures/🛑️cancellation/🔣️.json")).expect("cancellation fixture");
    let invocation_id = fixture["invocation"]["id"].as_str().expect("invocation id");
    let tool_name = fixture["invocation"]["toolName"].as_str().expect("tool name");
    let arguments = fixture["invocation"]["arguments"].as_str().expect("arguments");
    let mut shell = welcomed_shell();
    shell.apply_agent_bridge_frame(&crate::agent_bridge::GatewayToShell::AgentToolCall { invocation_id: invocation_id.into(), tool_name: tool_name.into(), arguments: arguments.into() }.encode()).expect("tool call frame");
    let control_id = format!("framework.chat.cancel.{invocation_id}");
    let action = chat_button_action(&shell.build_agent_chat_ui(), &control_id).expect("a running call publishes its cancel control");
    assert_eq!(action.action, "cancelToolCall");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("invocationId")).and_then(DslValue::as_str), Some(invocation_id));
    semio_framework_async::block_on(shell.dispatch_action(action)).expect("framework owns cancellation");
    let frames = shell.take_agent_bridge_outbox();
    assert_eq!(frames.len(), 1);
    assert_eq!(crate::agent_bridge::ShellToGateway::decode(&frames[0]).expect("cancel frame"), crate::agent_bridge::ShellToGateway::AgentCancel { invocation_id: invocation_id.into() });
    let cancelling = shell.build_agent_chat_ui();
    assert!(chat_button_action(&cancelling, &control_id).is_none(), "a cancelling row cannot send a duplicate control action");
    let mut cancelling_lines = Vec::new();
    chat_panel_lines(&cancelling, &mut cancelling_lines);
    assert!(cancelling_lines.contains(&"Cancelling…".to_string()), "optimistic state is visible: {cancelling_lines:?}");

    shell.apply_agent_bridge_frame(
        &crate::agent_bridge::GatewayToShell::AgentToolResult {
            invocation_id: invocation_id.into(),
            tool_name: tool_name.into(),
            ok: fixture["terminalResult"]["ok"].as_bool().expect("terminal ok"),
            summary: fixture["terminalResult"]["summary"].as_str().expect("terminal summary").into(),
        }
        .encode(),
    )
    .expect("terminal result frame");
    let settled = shell.build_agent_chat_ui();
    assert!(chat_button_action(&settled, &control_id).is_none());
    let mut settled_lines = Vec::new();
    chat_panel_lines(&settled, &mut settled_lines);
    assert!(settled_lines.contains(&"Failed".to_string()) && settled_lines.iter().any(|line| line.contains("cancelled by user")), "the gateway result settles the row: {settled_lines:?}");

    let mut offline = welcomed_shell();
    offline.apply_agent_bridge_frame(&crate::agent_bridge::GatewayToShell::AgentToolCall { invocation_id: invocation_id.into(), tool_name: tool_name.into(), arguments: arguments.into() }.encode()).expect("tool call frame");
    offline.chrome_build.agent.note_socket_closed();
    let offline_action = chat_button_action(&offline.build_agent_chat_ui(), &control_id).expect("React leaves a running row's control mounted while its sender can refuse");
    semio_framework_async::block_on(offline.dispatch_action(offline_action)).expect("framework owns cancellation");
    assert!(offline.take_agent_bridge_outbox().is_empty(), "a disconnected cancel changes and sends nothing");
    assert!(matches!(&offline.chrome_build.agent.conversation[0], crate::agent_bridge::AgentConversationEntry::ToolCall { state: crate::agent_bridge::AgentToolCallState::Running, .. }));
}

/// 💬️ The composer: a turn typed here leaves as a real `ShellToGateway::AgentMessage` and is echoed
/// into the same feed, a blank draft sends nothing, and with no socket open the draft is KEPT rather
/// than silently swallowed — React's `sendAgentMessage` contract, returning `false`.
#[test]
fn the_chat_composer_sends_one_agent_message_per_turn_and_keeps_an_unsendable_draft() {
    let mut offline = fresh_state();
    offline.agent_chat_draft = "hello".into();
    assert!(!offline.send_agent_chat_draft(), "💬️ no socket, nothing sent");
    assert_eq!(offline.agent_chat_draft, "hello", "💬️ and the human's text is not discarded");

    let mut shell = welcomed_shell();
    let _ = shell.take_agent_bridge_outbox();
    assert!(!shell.send_agent_chat_draft(), "💬️ a blank draft sends nothing");
    shell.agent_chat_draft = "  move it  ".into();
    assert!(shell.send_agent_chat_draft(), "💬️ a connected shell sends");
    assert!(shell.agent_chat_draft.is_empty(), "💬️ the draft clears on send");
    let outbox = shell.take_agent_bridge_outbox();
    assert_eq!(outbox.len(), 1, "💬️ exactly one frame per turn");
    match crate::agent_bridge::ShellToGateway::decode(&outbox[0]).expect("the outbox frame decodes") {
        crate::agent_bridge::ShellToGateway::AgentMessage { text, .. } => assert_eq!(text, "move it", "💬️ trimmed, React's own `trimmed`"),
        other => panic!("💬️ expected an AgentMessage, got {other:?}"),
    }
    let panel = shell.build_agent_chat_ui();
    assert!(chat_entry_ids(&panel).iter().any(|id| id.starts_with("framework.chat.entry.userMessage.")), "💬️ the turn is echoed into the feed");
    let mut lines = Vec::new();
    chat_panel_lines(&panel, &mut lines);
    assert!(lines.contains(&"You".to_string()) && lines.contains(&"move it".to_string()), "💬️ under the translated role noun: {lines:?}");
    let UiNode::Stack(root) = &panel else { panic!("💬️ the chat panel root is a stack") };
    assert!(root.children.iter().any(|node| matches!(node, UiNode::Input(input) if input.id == "framework.chat.draft")), "💬️ the draft box is part of the published body");
    assert!(root.children.iter().any(|node| matches!(node, UiNode::Button(button) if button.id.as_deref() == Some("framework.chat.send"))), "💬️ so is Send");
    assert!(panel_ui_records(FRAMEWORK_CHAT_PANEL_ID, &panel).is_ok(), "💬️ and the whole body projects");
}
//#endregion 💬️AgentChatTranscript

//#region 🌳️TreeUnits
/// 🌳️ Every persisted LEAF carries its single `<leafId>.tree` unit and every BRANCH carries none —
/// React's `panelTabNodeToSkeleton`, so a skeleton this renderer writes round-trips through React's
/// own unit resolution instead of dropping every leaf's units.
#[test]
fn a_persisted_skeleton_carries_each_leafs_tree_unit() {
    let shell = fixture_dock_shell();
    let skeleton = dock_skeleton_of(&shell.default_dock());
    fn walk(entries: &[DockTabSkeleton]) {
        for entry in entries {
            match entry.children.as_deref() {
                Some(children) => {
                    assert_eq!(entry.trees, None, "🌳️ a branch carries no tree units: {}", entry.id);
                    walk(children);
                }
                None => assert_eq!(entry.trees.as_deref(), Some([dock_leaf_tree_unit_id(&entry.id)].as_slice()), "🌳️ leaf {} carries its own single unit", entry.id),
            }
        }
    }
    for anchor in PanelAnchor::ALL {
        walk(skeleton.anchors.get(anchor.as_str()).map(Vec::as_slice).unwrap_or_default());
    }
}
//#endregion 🌳️TreeUnits

//#region 🧭️ChromeBandFixture
/// 🖥️ The Display branch is FRAMEWORK chrome: its children are the framework's own two display
/// leaves (React's `createFrameworkDisplayPanelTabs`), and an app's display-group tabs are its
/// SIBLINGS. Before this, wgpu built the branch out of the app's tabs and dropped the whole "Display"
/// chip for an app that declares none — which is exactly what the first real wgpu boot showed.
#[test]
fn default_dock_display_branch_carries_the_framework_display_leaves() {
    let fixture = default_dock_fixture();
    let shell = fixture_dock_shell();
    let dock = shell.default_dock();
    let bottom_left = dock.tabs(PanelAnchor::BottomLeft);
    let branch = bottom_left.first().expect("bottom-left opens with the framework Display branch");
    assert_eq!(branch.id, fixture["displayBranch"]["id"].as_str().expect("fixture display branch id"));
    assert!(branch.is_branch(), "the Display branch nests the framework display leaves");
    let expected: Vec<&str> = fixture["displayBranch"]["children"].as_array().expect("fixture display children").iter().map(|child| child.as_str().expect("child id")).collect();
    assert_eq!(branch.children.iter().map(|child| child.id.as_str()).collect::<Vec<_>>(), expected);
    assert!(bottom_left.iter().any(|tab| tab.id == "fixture.display"), "🧭️ an app's display-group tab is a SIBLING of the branch, not one of its children");
    assert!(!branch.children.iter().any(|child| child.id == "fixture.display"));
    let empty_app_shell = ShellState::new(Vec::new(), String::new());
    assert!(empty_app_shell.default_dock().tabs(PanelAnchor::BottomLeft).is_empty(), "🧭️ no session means no dock at all");
}

/// 🧭️ A dock whose chrome bands are exactly what the shared fixture declares.
fn banded_dock_shell() -> ShellState {
    let mut shell = fixture_dock_shell();
    shell.dock_override = None;
    shell.sync_dock_tabs();
    shell
}

/// 📑️ **The navbar band pin.** Top-left's folded tabs read at the navbar's LEADING edge in declared
/// order, and the trailing row is enumerated right-to-left so top-middle/top-right read left-to-right
/// with the last tab nearest `Fullscreen` — React's `navbarItems` order. The pre-parity renderer
/// packed all three navbar anchors onto the trailing side in reverse, which is why the boot showed
/// `Catalogue · Artifact · Chat · Tool runs · Inspection` all on the right.
#[test]
fn navbar_bands_match_the_react_chrome_band_fixture() {
    let fixture = default_dock_fixture();
    let shell = banded_dock_shell();
    let bands = &fixture["chromeBands"]["navbar"];
    let leading: Vec<&str> = bands["leading"].as_array().expect("navbar leading band").iter().map(|anchor| anchor.as_str().expect("anchor id")).collect();
    let trailing: Vec<&str> = bands["trailing"].as_array().expect("navbar trailing band").iter().map(|anchor| anchor.as_str().expect("anchor id")).collect();
    let expected_leading: Vec<(String, String)> = leading
        .iter()
        .flat_map(|anchor| {
            let anchor = PanelAnchor::from_str(anchor).expect("fixture anchor");
            shell.dock_tabs.tabs(anchor).iter().map(move |node| (anchor.as_str().to_string(), node.id.clone())).collect::<Vec<_>>()
        })
        .collect();
    for (index, (anchor, id)) in expected_leading.iter().enumerate() {
        let (actual_anchor, actual) = shell.navbar_leading_tab_row_item(index).expect("the leading band carries every top-left tab");
        assert_eq!((actual_anchor.as_str(), actual.id.as_str()), (anchor.as_str(), id.as_str()), "📑️ leading navbar slot {index}");
    }
    assert!(shell.navbar_leading_tab_row_item(expected_leading.len()).is_none(), "📑️ the leading band ends with top-left's last tab");
    let mut expected_trailing: Vec<(String, String)> = trailing
        .iter()
        .flat_map(|anchor| {
            let anchor = PanelAnchor::from_str(anchor).expect("fixture anchor");
            shell.dock_tabs.tabs(anchor).iter().map(move |node| (anchor.as_str().to_string(), node.id.clone())).collect::<Vec<_>>()
        })
        .collect();
    expected_trailing.reverse();
    for (index, (anchor, id)) in expected_trailing.iter().enumerate() {
        let (actual_anchor, actual) = shell.navbar_trailing_tab_row_item(index).expect("the trailing band carries every top-middle/top-right tab");
        assert_eq!((actual_anchor.as_str(), actual.id.as_str()), (anchor.as_str(), id.as_str()), "📑️ trailing navbar paint slot {index}");
    }
    assert!(shell.navbar_trailing_tab_row_item(expected_trailing.len()).is_none());
    assert!(!expected_leading.is_empty() && !expected_trailing.is_empty(), "🧭️ the fixture app populates both navbar bands");
    eprintln!("[DEBUG] navbar leading={expected_leading:?} trailing(paint order)={expected_trailing:?}");
}

/// 📑️ Footer bands follow React's measured placement: edge bands hug their edge and the centered
/// group clamps to the free span left by panel tabs, presence, and the Hub indicator.
#[test]
fn footer_bands_match_the_react_chrome_band_fixture() {
    let fixture = default_dock_fixture();
    let shell = banded_dock_shell();
    let theme = Theme::light();
    let mut atlas = FontAtlas::builtin();
    let (width, btn_h) = (1440.0_f32, theme.control_height);
    let lead_x = theme.padding_standard;
    let mut rows = Vec::new();
    for index in 0..64 {
        let Some((anchor, node, rect)) = shell.footer_tab_row_rect(&mut atlas, &theme, index, width, 0.0, btn_h) else { break };
        rows.push((anchor, node.id.clone(), rect));
    }
    assert!(!rows.is_empty(), "the fixture dock populates the footer");
    let band = |name: &str| -> Vec<(PanelAnchor, String, Rect)> {
        let anchor = PanelAnchor::from_str(name).expect("fixture anchor");
        rows.iter().filter(|(candidate, _, _)| *candidate == anchor).cloned().collect()
    };
    for name in fixture["chromeBands"]["footer"]["leading"].as_array().expect("footer leading band").iter().map(|value| value.as_str().expect("anchor id")) {
        let rows = band(name);
        assert_eq!(rows.first().map(|(_, _, rect)| rect.x), Some(lead_x), "📑️ {name} opens at the footer's leading edge");
    }
    for name in fixture["chromeBands"]["footer"]["centre"].as_array().expect("footer centre band").iter().map(|value| value.as_str().expect("anchor id")) {
        let rows = band(name);
        let (Some(first), Some(last)) = (rows.first(), rows.last()) else { continue };
        let span = (last.2.x + last.2.w) - first.2.x;
        let layout = shell.footer_chrome_layout(&mut atlas, &theme, width, 0.0, btn_h);
        let ideal = (width - span) * 0.5;
        let expected = ideal.clamp(layout.center.free.left, (layout.center.free.right - span).max(layout.center.free.left));
        assert!((first.2.x - expected).abs() < 0.5, "📑️ {name} uses the free footer band: first={first:?}, last={last:?}, span={span}, expected={expected}");
        assert!(first.2.x >= layout.center.free.left && last.2.x + last.2.w <= layout.center.free.right + 0.01);
    }
    for name in fixture["chromeBands"]["footer"]["trailing"].as_array().expect("footer trailing band").iter().map(|value| value.as_str().expect("anchor id")) {
        let rows = band(name);
        let Some(last) = rows.last() else { continue };
        assert!(((last.2.x + last.2.w) - (width - theme.padding_standard)).abs() < 0.5, "📑️ {name} ends at the footer's trailing edge");
    }
    for window in rows.windows(2) {
        let (left, right) = (&window[0], &window[1]);
        assert!(left.2.x + left.2.w <= right.2.x + 0.01, "📑️ footer chips never overlap: {} then {}", left.1, right.1);
    }
    eprintln!("[DEBUG] footer bands {:?}", rows.iter().map(|(anchor, id, rect)| (anchor.as_str(), id.as_str(), rect.x, rect.w)).collect::<Vec<_>>());
}
//#endregion 🧭️ChromeBandFixture
