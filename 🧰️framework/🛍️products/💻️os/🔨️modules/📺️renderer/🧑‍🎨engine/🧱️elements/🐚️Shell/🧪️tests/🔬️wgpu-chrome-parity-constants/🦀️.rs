//! 📐️ Laws for the geometry both chromes read from `🖱️ui/🧬️schema/📐️chrome-parity-constants`: panel drop feedback, toast sizing and the status stacks.

use semio_renderer_wgpu_shell_chrome_state::chrome_parity_constants;
use semio_renderer_wgpu_shell_view_chrome::{ShellIdentityResolution, ShellRouteAdmission, StatusAction, StatusCell, StatusOp, StatusStackKind, shell_identity_resolution, shell_route_admission, status_stack_layout, status_stack_ops, transient_notice_layout, wrap_status_text};
use semio_ui_theme::Theme;
use semio_ui_theme::geometry::Rect;

fn session_lane_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🏛️ShellHost/🧪️tests/🧫️fixtures/🧭️session-lane/🔣️.json")).expect("session lane fixture")
}

fn identity_of(name: &str) -> ShellIdentityResolution {
    match name {
        "local-only" => ShellIdentityResolution::LocalOnly,
        "signed-out" => ShellIdentityResolution::SignedOut,
        "pending" => ShellIdentityResolution::Pending,
        "signed-in" => ShellIdentityResolution::SignedIn,
        "hub-unavailable" => ShellIdentityResolution::HubUnavailable,
        other => panic!("unknown identity {other}"),
    }
}

/// ⚖️ LAW: the insert bar is the shared schema's width, and a drop nests into a branch over the schema's 30 to 70 percent band.
#[test]
fn the_panel_drop_constants_are_the_shared_schemas() {
    let drop = &chrome_parity_constants().panel_drop;
    assert_eq!(drop.insert_bar_width_px, semio_renderer_wgpu_shell_view_paint::TAB_INSERT_PREVIEW_WIDTH_PX);
    assert_eq!((drop.nest_min_fraction, drop.nest_max_fraction, drop.nest_fill_alpha, drop.empty_zone_min_width_rem), (0.3, 0.7, 0.2, 10.0));
}

/// ⚖️ LAW: an empty anchor's drop zone is `min-w-[10rem]` by one control, inside its one-row panel box where the root tab row would sit.
#[test]
fn an_empty_anchors_drop_zone_sits_inside_its_anchor_row() {
    use semio_renderer_wgpu_shell_windows::PanelAnchor;
    let theme = Theme::dark();
    let panel = Rect::new(100.0, 50.0, 300.0, theme.control_height);
    let zone = |anchor| semio_renderer_wgpu_shell_dock::empty_anchor_drop_zone_rect(anchor, panel, &theme);
    assert_eq!(zone(PanelAnchor::TopLeft), Rect::new(100.0 + theme.gap_standard, 50.0, 160.0, theme.control_height));
    let right = zone(PanelAnchor::TopRight);
    assert_eq!(right.x + right.w, 100.0 + 300.0 - theme.gap_standard);
    assert_eq!(zone(PanelAnchor::BottomLeft).y, 50.0);
    for anchor in PanelAnchor::ALL {
        let zone = zone(anchor);
        assert!(zone.x >= panel.x && zone.x + zone.w <= panel.x + panel.w && zone.y >= panel.y && zone.y + zone.h <= panel.y + panel.h);
    }
}

/// ⚖️ LAW: the toast no longer caps at 520 px: it shrink-wraps to the half of the workbench `left-1/2` leaves it and wraps its copy past that.
#[test]
fn the_toast_is_as_wide_as_react_leaves_it_and_wraps_past_that() {
    let theme = Theme::dark();
    let notice = |message: String| semio_renderer_wgpu_shell_chrome_state::ShellTransientNotice { message, severity: semio_framework::Severity::Info, code: None, shown_at_ms: 0.0 };
    let long = transient_notice_layout(&notice("word ".repeat(200)), "Close", 1600.0, &theme);
    assert!((long.rect.w - 800.0).abs() < 0.01, "half of a 1600 px workbench, not 520 px: {}", long.rect.w);
    assert!(long.lines.len() > 1 && long.rect.h > theme.font_size_small * 1.6 + theme.padding_standard * 2.0);
    let short = transient_notice_layout(&notice("saved".into()), "Close", 1600.0, &theme);
    assert!(short.rect.w < 200.0 && short.lines.len() == 1, "a short notice shrink-wraps");
    assert_eq!(wrap_status_text("a b\nc", 80), vec!["a b".to_string(), "c".to_string()]);
}

fn cell(text: &str, action: Option<&str>) -> StatusCell {
    StatusCell { text: text.to_string(), progress: None, action: action.map(|label| StatusAction { control_id: format!("shell.status.test.{label}"), label: label.to_string(), event: None }) }
}

/// ⚖️ LAW: the stacks sit where React's classes put them and stay inside the widths those classes allow.
#[test]
fn the_status_stacks_sit_and_size_like_react_notice_stacks() {
    let theme = Theme::dark();
    let viewport = (1600.0, 900.0);
    let long = "word ".repeat(120);
    let bootstrap = status_stack_layout(StatusStackKind::Bootstrap, &[cell(&long, None)], viewport, &theme).expect("bootstrap");
    let target = status_stack_layout(StatusStackKind::ExecutionTarget, &[cell("one", None)], viewport, &theme).expect("execution target");
    let artifact = status_stack_layout(StatusStackKind::ArtifactCreation, &[cell(&long, Some("Cancel")), cell("two", None)], viewport, &theme).expect("artifact creation");
    let route = status_stack_layout(StatusStackKind::RouteAdmission, &[cell(&long, None), cell("", Some("Sign in"))], viewport, &theme).expect("route admission");
    assert!(bootstrap.frame.w <= 800.0 + 0.01 && ((bootstrap.frame.x + bootstrap.frame.w * 0.5) - 800.0).abs() < 0.01, "left-1/2 -translate-x-1/2, at most half the workbench");
    assert_eq!(bootstrap.frame.y, theme.navbar_height + semio_renderer_wgpu_shell_view_paint::TRANSIENT_NOTICE_TOP_GAP);
    assert_eq!(target.frame.y, bootstrap.frame.y + theme.padding_standard * 2.0, "mt-double");
    assert_eq!(artifact.frame.x, theme.padding_standard * 2.0);
    assert!(artifact.frame.w <= 448.0 + 0.01, "max-w-[28rem]");
    assert_eq!(artifact.cells.len(), 2);
    assert!(artifact.cells[1].rect.y > artifact.cells[0].rect.y + artifact.cells[0].rect.h, "one hairline between cells, none around them");
    let session = status_stack_layout(StatusStackKind::SessionAuthority, &[cell(&long, Some("Cancel"))], viewport, &theme).expect("session authority");
    assert_eq!(session.frame.y, bootstrap.frame.y, "top-workbench");
    assert!((session.frame.x + session.frame.w - (viewport.0 - theme.padding_standard * 2.0)).abs() < 0.01, "right-double");
    assert!(session.frame.w <= 448.0 + 0.01, "max-w-[28rem]");
    assert_eq!(StatusStackKind::ALL.len(), 5, "five notice stacks");
    assert!(route.frame.w <= 1440.0 + 0.01, "max-w-[90vw]");
    assert_eq!(route.frame.y + route.frame.h, 900.0 - theme.footer_height);
    assert!(route.cells[1].rect.x >= route.cells[0].rect.x + route.cells[0].rect.w, "a horizontal row");
    let ops = status_stack_ops(&artifact, &theme);
    assert!(ops.iter().any(|op| matches!(op, StatusOp::Hit { action, .. } if action.label == "Cancel")));
    assert!(status_stack_layout(StatusStackKind::Bootstrap, &[], viewport, &theme).is_none());
}

/// ⚖️ LAW: identity resolution and route admission answer every row of the fixture React's `🧭️session-lane` laws read, and the vocabulary is its labels.
#[test]
fn route_admission_answers_the_session_lane_fixture() {
    let fixture = session_lane_fixture();
    for row in fixture["identities"].as_array().expect("identities") {
        let state = &row["state"];
        let resolved = shell_identity_resolution(state["hubConfigured"].as_bool().unwrap(), state["sessionHeld"].as_bool().unwrap(), state["refused"].as_bool().unwrap(), state["confirmed"].as_bool().unwrap(), state["offline"].as_bool().unwrap());
        assert_eq!(resolved, identity_of(row["expected"].as_str().unwrap()), "{state}");
    }
    for row in fixture["admissions"].as_array().expect("admissions") {
        let expected = match row["expected"].as_str().unwrap() {
            "apply" => ShellRouteAdmission::Apply,
            "apply-offline" => ShellRouteAdmission::ApplyOffline,
            "await-identity" => ShellRouteAdmission::AwaitIdentity,
            "await-sign-in" => ShellRouteAdmission::AwaitSignIn,
            other => panic!("unknown admission {other}"),
        };
        assert_eq!(shell_route_admission(row["route"].as_str().unwrap(), identity_of(row["identity"].as_str().unwrap())), expected, "{row}");
    }
    for label in fixture["admissionLabels"].as_array().expect("labels") {
        let is_de = label["locale"] == "de";
        let key = match label["admission"].as_str().unwrap() {
            "await-identity" => "status.routeAdmission.awaitIdentity",
            "await-sign-in" => "status.routeAdmission.awaitSignIn",
            _ => "status.routeAdmission.applyOffline",
        };
        assert_eq!(semio_renderer_wgpu_shell_vocab::shell_chrome_string(key, is_de), label["text"].as_str().unwrap());
        if let Some(action) = label["action"].as_str() {
            assert_eq!(semio_renderer_wgpu_shell_vocab::shell_chrome_string("status.routeAdmission.signIn", is_de), action);
        }
    }
}

/// ⚖️ LAW: the pane, palette and band extents are the shared schema's, not literals.
#[test]
fn the_pane_palette_and_band_extents_are_the_shared_schemas() {
    let sizes = &chrome_parity_constants().sizes;
    assert_eq!((sizes.panel_default_px, sizes.window_pane_default_px, sizes.palette_dialog_max_width_rem), (300.0, 300.0, 32.0));
    assert_eq!((sizes.time_travel_band_max_width_fraction, sizes.folder_band_max_width_fraction), (0.9, 0.9));
}

/// ⚖️ LAW: a cell with a progress meter lays out a track and paints its filled fraction between the copy and the action.
#[test]
fn a_status_cell_with_progress_paints_its_meter() {
    let theme = Theme::dark();
    let mut meter = cell("Restoring", Some("Cancel"));
    meter.progress = Some(0.25);
    let layout = status_stack_layout(StatusStackKind::Bootstrap, &[meter], (1200.0, 800.0), &theme).expect("laid out");
    let (track, fraction) = layout.cells[0].bar.expect("meter");
    assert_eq!(fraction, 0.25);
    let action = layout.cells[0].action.as_ref().expect("action").0;
    assert!(track.y + track.h <= action.y);
    let fills = status_stack_ops(&layout, &theme).into_iter().filter(|op| matches!(op, StatusOp::Fill(rect, _) if rect.y == track.y && rect.h == track.h)).count();
    assert_eq!(fills, 2);
}

/// ⚖️ LAW: the selected-and-focused underline is React's 2 px at `0.2em` (`text-decoration-thickness`, `text-underline-offset`).
#[test]
fn the_focus_underline_is_reacts_geometry() {
    let focus = &chrome_parity_constants().focus;
    assert_eq!((focus.underline_thickness_px, focus.underline_offset_em), (2.0, 0.2));
}

/// ⚖️ LAW: the host closes at once unless the shell reported edits it cannot vouch for; a held close is taken once, its approval once, and dropped files arrive in drop order.
#[test]
fn the_host_window_lifecycle_holds_a_blocked_close_and_queues_dropped_files() {
    use semio_renderer_wgpu_shell_window_actions as host;
    host::publish_host_unsaved(false);
    assert!(host::request_host_close(), "nothing to lose closes at once");
    host::publish_host_unsaved(true);
    assert!(!host::request_host_close(), "blocked closes are held");
    assert!(host::take_host_close_request() && !host::take_host_close_request());
    host::approve_host_close();
    assert!(host::take_host_close_approval() && !host::take_host_close_approval());
    host::note_host_file_drop("a.semio".into());
    host::note_host_file_drop("b.semio".into());
    assert_eq!(host::take_host_file_drops(), vec![std::path::PathBuf::from("a.semio"), std::path::PathBuf::from("b.semio")]);
    assert!(host::take_host_file_drops().is_empty());
}

/// 🛬️ LAW: the browser page's dropped files arrive in drop order with the page's tokens, wake the shell's drain, and an unreadable list drops nothing.
#[test]
fn the_browser_page_hands_dropped_files_over_by_token() {
    use semio_renderer_wgpu_shell_window_actions as host;
    host::note_host_file_drop_json("not json");
    assert!(host::take_host_picked_files().is_empty());
    host::note_host_file_drop_json(r#"[{"token":3,"name":"a.pdf","size":10,"mime":"application/pdf"},{"token":4,"name":"b.pdf","size":0}]"#);
    assert!(host::host_window_lifecycle_pending());
    assert_eq!(host::take_host_picked_files(), vec![(3, "a.pdf".to_string(), 10, "application/pdf".to_string()), (4, "b.pdf".to_string(), 0, String::new())]);
    assert!(host::take_host_picked_files().is_empty() && !host::host_window_lifecycle_pending());
}
