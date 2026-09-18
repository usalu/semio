//! 🪟️ LAW: every window pane carries React's own overlay rows — a top row `Actions · Search ·
//! Window Options` and a bottom row `Utilities · Projection` — at React's `Pane` anchors, dispatching
//! React's own per-window control ids; and the footer carries NO utility rail, because React has no
//! footer counterpart for one at all.
//!
//! Oracle: `🐚️Shell/🧫️fixtures/🪟️window-pane-chrome/🔣️.json` — the chip table and anchor insets are
//! read off React's `Pane`/`Window`/`World3dHost` sources and its measured DOM geometry, derived
//! independently of this implementation.

use super::*;
use semio_framework::UtilityDefinition;
use ui_wgpu::wgpu::{WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};

fn pane_fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪟️window-pane-chrome/🔣️.json")).expect("window pane chrome fixture")
}

fn chip_of(name: &str) -> WindowPaneChip {
    match name {
        "actions" => WindowPaneChip::Actions,
        "search" => WindowPaneChip::Search,
        "windowOptions" => WindowPaneChip::WindowOptions,
        "utilities" => WindowPaneChip::Utilities,
        "projection" => WindowPaneChip::Projection,
        other => panic!("🪟️ the fixture names a chip this renderer has no variant for: {other}"),
    }
}

/// 🪟️ An app whose default layout is puzzle3d's own two-pane row split — one window KIND opened
/// twice, each pane carrying the authored instance title React reads its cap from.
fn split_pane_app() -> AppDefinition {
    let mut app = super::command_registry_tests::test_app(Vec::new(), Vec::new());
    app.id = "pane-fixture".into();
    app.controller_id = "pane.controller".into();
    app.utilities = vec![
        UtilityDefinition { id: "pane.transform".into(), label: LocalizedLabel::data("Transform"), icon_id: "move".into(), group: None, keys: None, cursor: None, category: None, allows_actions_while_active: true, run: None },
        UtilityDefinition { id: "pane.brush".into(), label: LocalizedLabel::data("Brush"), icon_id: "brush".into(), group: None, keys: None, cursor: None, category: None, allows_actions_while_active: true, run: None },
    ];
    let window = |instance: &str, title: &str, size: f64| {
        WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: Some(size),
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "main".into(), title: Some(title.into()), instance_id: Some(instance.into()), template_id: None, corner: None }],
        })
    };
    app.default_layout = Some(WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![window("pane-top", "Top", 33.333), window("pane-perspective", "Perspective", 66.667)] }),
    });
    app
}

pub(super) fn split_pane_shell() -> ShellState {
    let app = split_pane_app();
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.session = Some(ActiveSession { plugin_id: "pane".into(), instance_id: 1, app, view_state: ViewModel::default() });
    shell.sync_dock();
    shell.active_window_id = Some("pane-top".into());
    shell
}

/// 📐️ The two pane bodies this frame's dock plans at the React reference viewport.
fn planned_panes(shell: &mut ShellState, theme: &Theme, atlas: &mut FontAtlas, width: f32, height: f32) -> Vec<(String, Rect)> {
    shell.screen_w = width;
    shell.plan_dock_windows(Rect::new(0.0, theme.navbar_height, width, height - theme.navbar_height - theme.footer_height).inset(theme.panel_inset), theme, atlas);
    shell.dock_window_plan.clone()
}

/// 🪟️ **The chip table pin.** Anchor, icon, both locales' label and BOTH fold states' control id, chip
/// by chip against the shared fixture. wgpu painted none of these rows at all: the only per-pane chrome
/// was an icon-only `settings-2` square with no label and no band.
#[test]
fn pane_chips_match_the_react_pane_fixture() {
    let fixture = pane_fixture();
    let chips = fixture["paneChips"].as_array().expect("fixture pane chips");
    assert_eq!(chips.len(), WindowPaneChip::ALL.len(), "🪟️ every chip the fixture declares has a variant, and no more");
    for (declared, chip) in chips.iter().zip(WindowPaneChip::ALL) {
        let name = declared["chip"].as_str().expect("chip name");
        assert_eq!(chip_of(name), chip, "🪟️ chip {name} keeps React's own mount order");
        assert_eq!(chip.anchor().as_str(), declared["anchor"].as_str().expect("chip anchor"), "🪟️ {name} anchor");
        assert_eq!(chip.icon_id(), declared["icon"].as_str().expect("chip icon"), "🪟️ {name} icon");
        assert_eq!(shell_chrome_string(chip.label_key(), false), declared["label"]["en"].as_str().expect("chip label en"), "🪟️ {name} label (en)");
        assert_eq!(shell_chrome_string(chip.label_key(), true), declared["label"]["de"].as_str().expect("chip label de"), "🪟️ {name} label (de)");
        let expected = declared["controlId"].as_str().expect("chip control id").replace("{windowId}", "pane-top");
        assert_eq!(chip.control_id("pane-top", true), expected, "🪟️ {name} folded control id");
        if let Some(open) = declared["controlIdOpen"].as_str() {
            assert_eq!(chip.control_id("pane-top", false), open.replace("{windowId}", "pane-top"), "🪟️ {name} unfolded control id");
        }
    }
}

/// 📐️ **The banding pin.** Every chip sits on its anchor's own two edges — React's
/// `anchorPositionStyle` — inside the pane it belongs to, for BOTH panes of a two-pane split at
/// 1440×900, and the two rows never overlap each other or leave the body.
#[test]
fn pane_chip_rects_band_a_two_pane_split_at_reacts_anchor_insets() {
    let fixture = pane_fixture();
    let (width, height) = (fixture["viewport"]["width"].as_f64().expect("fixture width") as f32, fixture["viewport"]["height"].as_f64().expect("fixture height") as f32);
    let theme = Theme::light();
    let mut atlas = FontAtlas::builtin();
    let mut shell = split_pane_shell();
    let panes = planned_panes(&mut shell, &theme, &mut atlas, width, height);
    assert_eq!(panes.len(), 2, "📐️ the authored row split plans two panes");
    for (window_id, body) in &panes {
        let mut rows: Vec<(WindowPaneChip, Rect)> = Vec::new();
        for chip in WindowPaneChip::ALL {
            let item = ChromeGroupItem { control_id: "", icon_id: Some(chip.icon_id()), label: Some(shell_chrome_string(chip.label_key(), false)), active: false, disabled: false, kind: HitKind::Toggle };
            let chip_width = retained_chrome_group_item_width(&mut atlas, &theme, &item).expect("a chip label fits the retained chrome boundary");
            let rect = window_pane_chip_rect(&theme, *body, chip.anchor(), chip_width);
            let sides = fixture["paneAnchorInsets"][chip.anchor().as_str()].as_array().expect("fixture pane anchor insets");
            match sides[0].as_str().expect("horizontal edge") {
                "left" => assert!((rect.x - (body.x + theme.panel_inset)).abs() < 0.01, "📐️ {window_id}/{:?} hugs the pane's leading edge", chip),
                "right" => assert!(((rect.x + rect.w) - (body.x + body.w - theme.panel_inset)).abs() < 0.01, "📐️ {window_id}/{:?} ends at the pane's trailing edge", chip),
                _ => assert!((rect.x + rect.w * 0.5 - (body.x + body.w * 0.5)).abs() < 0.01, "📐️ {window_id}/{:?} centres on the pane", chip),
            }
            match sides[1].as_str().expect("vertical edge") {
                "top" => assert!((rect.y - (body.y + theme.panel_inset)).abs() < 0.01, "📐️ {window_id}/{:?} rides the pane's top row", chip),
                _ => assert!(((rect.y + rect.h) - (body.y + body.h - theme.panel_inset)).abs() < 0.01, "📐️ {window_id}/{:?} rides the pane's bottom row", chip),
            }
            assert!(rect.x >= body.x && rect.x + rect.w <= body.x + body.w + 0.01, "📐️ {window_id}/{:?} never leaves its own pane", chip);
            rows.push((chip, rect));
        }
        for row in [0.0_f32, 1.0] {
            let mut band: Vec<(WindowPaneChip, Rect)> = rows.iter().filter(|(chip, _)| (chip.anchor().vertical() == "bottom") == (row > 0.5)).cloned().collect();
            band.sort_by(|left, right| left.1.x.total_cmp(&right.1.x));
            for pair in band.windows(2) {
                assert!(pair[0].1.x + pair[0].1.w <= pair[1].1.x + 0.01, "📐️ {window_id}: {:?} and {:?} never overlap", pair[0].0, pair[1].0);
            }
        }
        eprintln!("[DEBUG] pane {window_id} body={:?} chips={:?}", body, rows.iter().map(|(chip, rect)| (format!("{chip:?}"), rect.x.round(), rect.y.round(), rect.w.round())).collect::<Vec<_>>());
    }
}

/// 🪟️ **The mount pin.** React mounts `Window Options` only for a window that projects measures,
/// `Utilities` always (its toggle disabled when the window derives no utility bar) and the projection
/// pane only from inside a world surface. `Projection` is ENABLED wherever it mounts — see
/// `🧭️wgpu-navbar-footer-parity/🦀️.rs`'s hit-target law for why.
#[test]
fn pane_chips_mount_exactly_where_react_mounts_them() {
    let fixture = pane_fixture();
    let shell = split_pane_shell();
    let mounted = shell.window_pane_chips("pane-top");
    for declared in fixture["paneChips"].as_array().expect("fixture pane chips") {
        let chip = chip_of(declared["chip"].as_str().expect("chip name"));
        let entry = mounted.iter().find(|(candidate, _, _)| *candidate == chip);
        match declared["mounted"].as_str().expect("chip mount rule") {
            "always" => {
                let (_, folded, disabled) = entry.copied().expect("🪟️ an `always` chip mounts on every pane");
                assert!(folded, "🪟️ every pane rail starts folded, as React's `useState(true)` does");
                assert!(!disabled || declared["disabledWithoutBody"].as_bool().unwrap_or(false), "🪟️ only a chip the fixture allows to lose its body is ever disabled");
            }
            _ => assert!(entry.is_none(), "🪟️ a pane with no world surface mounts no projection chip"),
        }
    }
    let mut bare = split_pane_shell();
    if let Some(session) = bare.session.as_mut() {
        session.app.utilities = Vec::new();
    }
    assert!(
        bare.window_pane_chips("pane-top").iter().any(|(chip, _, disabled)| *chip == WindowPaneChip::Utilities && *disabled),
        "🪟️ a window with no utility bar keeps the chip and disables its toggle"
    );
    assert!(mounted.iter().any(|(chip, _, disabled)| *chip == WindowPaneChip::Utilities && !*disabled), "🪟️ a window that DOES declare utilities keeps its toggle live");
    assert!(mounted.iter().any(|(chip, _, disabled)| *chip == WindowPaneChip::WindowOptions && *disabled), "🪟️ a window that projects no measures document keeps the chip and disables its toggle");
}

/// 🎛️ **The fold-round-trip pin.** The Window Options chip is registered on BOTH sides of the fold, so
/// a rail that defaults folded is still reachable, and the ids are exactly the
/// `shell.measures.unfold.*`/`shell.measures.fold.*` the shell already dispatches. Painted through the
/// real chip row, so the glass region and the hit come from the code the boot runs.
///
/// 🩸️ Nothing painted this chip with a LABEL at all: the pre-parity renderer drew a bare `settings-2`
/// square, which is why the rail read as a stray icon against React's `Window Options`.
#[test]
fn the_window_options_chip_registers_both_sides_of_its_fold() {
    let theme = Theme::light();
    let window = Rect::new(0.0, 0.0, 800.0, 600.0);
    let paint = |shell: &mut ShellState| {
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut cursor = ShellChromeChildCursor::default();
        for _ in 0..4096 {
            if shell.paint_window_pane_chips_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, "pane-top", window) {
                break;
            }
        }
        (input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).collect::<Vec<_>>(), draw.glass_regions.len())
    };
    let mut shell = split_pane_shell();
    shell.measures_folded.insert("pane-top".into(), true);
    shell.window_measures_documents.clear();
    let (folded_ids, glass) = paint(&mut shell);
    assert_eq!(glass, WindowPaneChip::ALL.len() - 1, "🪟️ every mounted chip is its own glass region (no world surface, so no projection chip)");
    assert!(!folded_ids.iter().any(|id| id.starts_with("shell.measures.")), "🎛️ a window that projects no measures mounts the chip DISABLED and registers no hit");
    shell.measures_folded.insert("pane-top".into(), false);
    assert!(!shell.measures_rail_folded("pane-top"));
    assert_eq!(WindowPaneChip::WindowOptions.control_id("pane-top", false), "shell.measures.fold.pane-top");
    assert!(folded_ids.iter().any(|id| id == "shell.action.fold.pane-top"), "🎛️ the enabled chips do register their hits: {folded_ids:?}");
}

/// 🎯️ **The dispatch pin.** Each chip's control id flips ONLY its own window's state — the fold
/// lane React's two panes share (`Actions`/`Search` both drive `actionsFolded`), the measures fold and
/// the utility-bar fold.
#[test]
fn each_pane_chip_dispatches_its_own_window_state() {
    let mut shell = split_pane_shell();
    let press = |shell: &mut ShellState, control_id: String| {
        let hit = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None };
        assert!(semio_framework_async::block_on(shell.handle_shell_hit(&hit)).expect("a pane chip press never errors"), "🎯️ the shell claims its own pane chip");
    };
    assert!(shell.window_actions_folded("pane-top") && shell.utility_bar_folded("pane-top") && shell.measures_rail_folded("pane-top"), "🪟️ every pane rail starts folded, as React's `useState(true)` does");
    press(&mut shell, WindowPaneChip::Actions.control_id("pane-top", true));
    assert!(!shell.window_actions_folded("pane-top") && shell.window_actions_folded("pane-perspective"), "🎯️ Actions unfolds only its own pane");
    press(&mut shell, WindowPaneChip::Search.control_id("pane-perspective", true));
    assert!(!shell.window_actions_folded("pane-perspective"), "🎯️ Search drives the same `actionsFolded` its pane's Actions chip does");
    assert_eq!(shell.active_window_id.as_deref(), Some("pane-perspective"), "🎯️ opening a pane's search focuses that pane, which is what scopes the palette to it");
    assert!(shell.search_open && matches!(shell.overlay_state, OverlayState::Search));
    press(&mut shell, WindowPaneChip::Utilities.control_id("pane-top", true));
    assert!(!shell.utility_bar_folded("pane-top") && shell.utility_bar_folded("pane-perspective"), "🎯️ Utilities unfolds only its own pane");
    press(&mut shell, WindowPaneChip::Utilities.control_id("pane-top", false));
    assert!(shell.utility_bar_folded("pane-top"), "🎯️ the unfolded chip carries the fold id");
    press(&mut shell, WindowPaneChip::WindowOptions.control_id("pane-perspective", true));
    assert!(!shell.measures_rail_folded("pane-perspective") && shell.measures_rail_folded("pane-top"), "🎯️ Window Options unfolds only its own pane");
}

/// 🧰️ **The utilities-move pin.** A pane's Utilities rail derives the app's utilities scoped to THAT
/// pane's window kind and marks that pane's own active utility pressed — React's per-instance
/// `resolveUtilityNodes`. Two panes of one kind never share a pressed utility.
#[test]
fn a_panes_utility_rail_is_derived_per_pane() {
    let mut shell = split_pane_shell();
    shell.apply_set_active_utility("pane-top", "pane.brush");
    let session = shell.session.clone().expect("fixture session");
    let pressed = |shell: &ShellState, window_id: &str| {
        shell
            .derive_window_utility_nodes(&session, window_id)
            .into_iter()
            .filter_map(|node| match node {
                UtilityNode::Toggle { id, pressed, .. } => pressed.unwrap_or(false).then_some(id),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(pressed(&shell, "pane-top"), vec!["pane.brush".to_string()], "🧰️ the pane that armed the utility shows it pressed");
    assert!(pressed(&shell, "pane-perspective").is_empty(), "🧰️ its sibling pane of the same kind does not");
}

/// 🔚️ **The footer pin.** Painting a whole footer registers no utility hit at all and no
/// `framework.sync.*` leaf — React's footer has neither (its sync leaves live on the bottom-left
/// `s-sync-status` tab's card). The wgpu boot read `Transform · Brush · Volume Brush · Relocate ·
/// File · Folder · Remote` there.
#[test]
fn the_footer_carries_no_utility_rail() {
    let fixture = pane_fixture();
    let theme = Theme::light();
    let mut shell = split_pane_shell();
    shell.sync_backbone_uri = Some("folder:///tmp/fixture".into());
    shell.dock_override = None;
    shell.sync_dock_tabs();
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    let (width, height) = (1440.0_f32, 900.0_f32);
    for _ in 0..4096 {
        if shell.render_footer_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, width, height) {
            break;
        }
    }
    let ids: Vec<String> = input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).collect();
    assert!(!ids.is_empty(), "🔚️ the footer registers its own tab chips");
    for forbidden in fixture["footerComposition"]["forbiddenControlIds"].as_array().expect("fixture forbidden ids").iter().map(|value| value.as_str().expect("forbidden id")) {
        assert!(!ids.iter().any(|id| id.starts_with(forbidden)), "🔚️ the footer must carry no {forbidden} chip: {ids:?}");
    }
    for forbidden in fixture["footerComposition"]["forbiddenUtilityIds"].as_array().expect("fixture forbidden utilities").iter().map(|value| value.as_str().expect("forbidden utility")) {
        assert!(!ids.iter().any(|id| id.contains(forbidden)), "🔚️ the footer must carry no {forbidden} toggle: {ids:?}");
        assert!(!shell.active_utilities.iter().any(|node| matches!(node, UtilityNode::Toggle { id, .. } | UtilityNode::Button { id, .. } | UtilityNode::Collection { id, .. } | UtilityNode::Separator { id, .. } if id == forbidden)), "🔚️ the shell's utility roster no longer carries the framework sync leaves either");
    }
    eprintln!("[DEBUG] footer hits {ids:?}");
}

/// 🏷️ **The cap-label pin.** A pane's cap reads the authored INSTANCE title (`Top`, `Perspective`),
/// not the window kind's manifest label — React's `windowTitlesById[instance.id] ?? instance.title`.
/// wgpu read `Puzzle 3D` on both panes because `stack_from_node` dropped the node's own title.
#[test]
fn window_caps_read_the_authored_instance_title() {
    let shell = split_pane_shell();
    let (labels, _) = shell.dock_chrome_maps();
    assert_eq!(labels.get("pane-top").map(String::as_str), Some("Top"));
    assert_eq!(labels.get("pane-perspective").map(String::as_str), Some("Perspective"));
    let bare = {
        let mut app = split_pane_app();
        app.default_layout = None;
        let mut shell = ShellState::new(Vec::new(), String::new());
        shell.session = Some(ActiveSession { plugin_id: "pane".into(), instance_id: 1, app, view_state: ViewModel::default() });
        shell.sync_dock();
        shell.dock_chrome_maps().0
    };
    assert_eq!(bare.get("main").map(String::as_str), Some("Main"), "🏷️ a bare window KIND still wears its manifest label");
}

/// 🧰️ **The rail-paint pin.** Unfolding a pane's Utilities chip paints the app's own utility chips on
/// that pane's bottom row, to the RIGHT of the chip and inside the pane — the place React's
/// `bottom-left` `Pane` body grows into. Folded, the rail paints nothing at all.
#[test]
fn an_unfolded_pane_utility_rail_paints_inside_its_own_pane() {
    let theme = Theme::light();
    let mut shell = split_pane_shell();
    let mut atlas = FontAtlas::builtin();
    let panes = planned_panes(&mut shell, &theme, &mut atlas, 1440.0, 900.0);
    let (window_id, body) = panes.first().cloned().expect("the split plans a first pane");
    let rail = |shell: &mut ShellState| {
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut cursor = ShellChromeChildCursor::default();
        for _ in 0..4096 {
            if shell.paint_window_utilities_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, &window_id, body) {
                break;
            }
        }
        input.staged_hits().iter().filter(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("framework.utility."))).map(|hit| (hit.control_id.clone().unwrap_or_default(), hit.rect)).collect::<Vec<_>>()
    };
    assert!(rail(&mut shell).is_empty(), "🧰️ a folded rail paints nothing");
    shell.utility_bar_folded.insert(window_id.clone(), false);
    let painted = rail(&mut shell);
    assert_eq!(painted.len(), 2, "🧰️ the unfolded rail paints the app's own two utilities: {painted:?}");
    let chip = ChromeGroupItem { control_id: "", icon_id: Some(WindowPaneChip::Utilities.icon_id()), label: Some(shell_chrome_string(WindowPaneChip::Utilities.label_key(), false)), active: true, disabled: false, kind: HitKind::Toggle };
    let chip_rect = window_pane_chip_rect(&theme, body, PanelAnchor::BottomLeft, retained_chrome_group_item_width(&mut atlas, &theme, &chip).expect("the Utilities chip fits"));
    for (id, rect) in &painted {
        assert!(rect.x >= chip_rect.x + chip_rect.w, "🧰️ {id} opens to the right of the Utilities chip");
        assert!(rect.x + rect.w <= body.x + body.w, "🧰️ {id} stays inside its own pane");
        assert!((rect.y + rect.h - (body.y + body.h - theme.panel_inset)).abs() < 0.01, "🧰️ {id} rides the pane's bottom row");
    }
    eprintln!("[DEBUG] pane {window_id} utility rail {painted:?}");
}
