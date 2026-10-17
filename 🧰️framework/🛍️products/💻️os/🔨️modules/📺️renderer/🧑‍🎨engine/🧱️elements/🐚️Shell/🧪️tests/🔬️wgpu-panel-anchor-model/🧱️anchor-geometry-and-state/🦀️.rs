/// 🧱️ Anchor geometry and state: anchor insets, banding and chrome-hosted style, the open-anchor snapshot, census, column projections and path reconciliation.

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
///
/// 📐️ The chrome-hosted root remains in the navbar/footer. The floating child panel grows into
/// the overhang but omits that already-owned root row.
#[test]
fn anchor_rect_bands_only_the_open_anchors_of_a_column() {
    let theme = Theme::default();
    let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
    let mut shell = fixture_dock_shell();
    shell.dock.toggle_anchor(PanelAnchor::TopLeft);
    let alone = ChromeViewCtx::of(&mut shell).anchor_rect(PanelAnchor::TopLeft, body, &theme);
    assert_eq!(alone.h, body.h + (theme.navbar_height + theme.control_height) * 0.5 - theme.control_height);
    shell.dock.toggle_anchor(PanelAnchor::BottomLeft);
    let shared = ChromeViewCtx::of(&mut shell).anchor_rect(PanelAnchor::TopLeft, body, &theme);
    assert!(shared.h < alone.h * 0.6, "🧭️ opening the column's other anchor must shrink this one's band");
    assert_eq!(shell.dock.open_anchors(), vec![PanelAnchor::TopLeft, PanelAnchor::BottomLeft]);
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
    shell.dock.toggle_anchor(PanelAnchor::TopLeft);
    let rect = ChromeViewCtx::of(&mut shell).anchor_rect(PanelAnchor::TopLeft, body, &theme);
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
    shell.dock.toggle_anchor(PanelAnchor::BottomLeft);
    let bottom = ChromeViewCtx::of(&mut shell).anchor_rect(PanelAnchor::BottomLeft, body, &theme);
    assert!((bottom.y + bottom.h - (body.y + body.h + overhang - theme.control_height)).abs() < 0.01, "🧭️ a bottom child panel ends at its footer-owned root row: {bottom:?}");
    let mut middle_shell = fixture_dock_shell();
    middle_shell.dock.toggle_anchor(PanelAnchor::LeftMiddle);
    let middle = ChromeViewCtx::of(&mut middle_shell).anchor_rect(PanelAnchor::LeftMiddle, body, &theme);
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
    shell.dock.toggle_anchor(PanelAnchor::TopLeft);
    let empty_plan = ChromeViewCtx::of(&mut shell).anchor_rect(PanelAnchor::TopLeft, body, &theme);
    shell.dock.dock_window_plan = vec![("puzzle3d-main-top".to_string(), Rect::new(3.0, body.y + 32.0, 530.0, 800.0))];
    let planned = ChromeViewCtx::of(&mut shell).anchor_rect(PanelAnchor::TopLeft, body, &theme);
    assert_eq!(empty_plan.y, planned.y, "🧭️ the dock's tab bar no longer pushes the panel down");
    assert_eq!(empty_plan.h, planned.h);
}
//#endregion 🧭️AnchorGeometry

//#region 🧭️AnchorState
#[test]
fn panel_anchor_snapshot_reports_the_open_anchor_and_its_active_leaf() {
    let mut shell = fixture_dock_shell();
    shell.dock.toggle_anchor_tab(PanelAnchor::TopLeft, "fixture.workbench");
    let top_left = shell.dock.panel_anchor_snapshot(PanelAnchor::TopLeft);
    assert!(top_left.visible);
    assert_eq!(top_left.size, DEFAULT_PANEL_WIDTH_PX);
    assert_eq!(top_left.active_tab.as_deref(), Some("fixture.workbench"));
    assert_eq!(shell.dock.panel_anchor_snapshot(PanelAnchor::BottomLeft), PanelAnchorSnapshot::default(), "a folded anchor reports nothing");
}

/// 🪟️ **The surface-census law.** What `dumpChrome` publishes beside the hit registry: this frame's
/// dock panes at WINDOW level and every open anchor's active tab at PANEL level, under React's own
/// `panelTabElementId`. A probe comparing two shells reads this instead of the engine's window list,
/// which cannot tell a docked panel from a pane here because a panel IS a window instance to this
/// renderer (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md`).
#[test]
fn the_chrome_surface_census_separates_panes_from_docked_panels() {
    let mut shell = fixture_dock_shell();
    shell.dock.dock_window_plan = vec![("puzzle3d-main-top".to_string(), Rect::new(0.0, 54.0, 600.0, 900.0)), ("puzzle3d-main-perspective".to_string(), Rect::new(600.0, 54.0, 600.0, 900.0))];
    assert_eq!(
        shell.input.chrome_surface_census(&shell.chrome, &shell.dock),
        vec![("puzzle3d-main-top".to_string(), "window", "puzzle3d-main-top".to_string()), ("puzzle3d-main-perspective".to_string(), "window", "puzzle3d-main-perspective".to_string())],
        "🪟️ a shell with no open anchor publishes its panes and nothing else"
    );
    shell.dock.toggle_anchor_tab(PanelAnchor::TopLeft, "fixture.workbench");
    let census = shell.input.chrome_surface_census(&shell.chrome, &shell.dock);
    assert_eq!(
        census.iter().filter(|(_, level, _)| *level == "panel").map(|(id, _, element)| (id.as_str(), element.as_str())).collect::<Vec<_>>(),
        vec![("fixture.workbench", "framework.panelTab.fixture.workbench")],
        "🪟️ the open anchor's ACTIVE TAB is the panel, named the way React names it"
    );
    assert_eq!(census.iter().filter(|(_, level, _)| *level == "window").count(), 2, "🪟️ and opening a panel adds no window");
}

/// 🧭️ Every anchor opens INDEPENDENTLY — the whole point of the packet. The former model could only
/// ever have one left and one right panel open at a time.
#[test]
fn all_eight_anchors_open_independently() {
    let mut shell = fixture_dock_shell();
    for anchor in PanelAnchor::ALL {
        *shell.dock.dock_tabs.tabs_mut(anchor) = vec![DockTabNode::leaf(format!("leaf.{}", anchor.as_str()), "Leaf", "circle-dot", 0)];
    }
    for anchor in PanelAnchor::ALL {
        shell.dock.toggle_anchor(anchor);
    }
    assert_eq!(shell.dock.open_anchors().len(), 8);
    shell.dock.toggle_anchor(PanelAnchor::TopLeft);
    assert!(!shell.dock.anchor_open(PanelAnchor::TopLeft));
    assert_eq!(shell.dock.open_anchors().len(), 7, "folding one anchor leaves the other seven alone");
}

/// 🧭️ An anchor carrying no tab never opens — no empty glass box over the canvas.
#[test]
fn an_empty_anchor_never_opens() {
    let mut shell = fixture_dock_shell();
    assert!(shell.dock.dock_tabs.tabs(PanelAnchor::LeftMiddle).is_empty());
    shell.dock.anchor_state_mut(PanelAnchor::LeftMiddle).visible = true;
    assert!(!shell.dock.anchor_open(PanelAnchor::LeftMiddle));
    assert!(!shell.dock.open_anchors().contains(&PanelAnchor::LeftMiddle));
}

/// 🧭️ `left_panel_open`/`right_panel_open` survive only as COLUMN projections of their two corner
/// anchors — the contract the overlay safe area and the `⌘️B` chords still read.
#[test]
fn column_projections_follow_their_corner_anchors() {
    let mut shell = fixture_dock_shell();
    assert!(!shell.dock.left_panel_open() && !shell.dock.right_panel_open());
    shell.dock.toggle_anchor(PanelAnchor::BottomLeft);
    assert!(shell.dock.left_panel_open(), "the left column is open while EITHER of its corners is");
    assert!(!shell.dock.right_panel_open());
    shell.dock.toggle_column(true);
    assert!(!shell.dock.left_panel_open(), "the column chord folds every anchor in that column");
    shell.dock.toggle_column(true);
    assert!(shell.dock.anchor_open(PanelAnchor::TopLeft), "reopening a folded column reopens its top corner");
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
