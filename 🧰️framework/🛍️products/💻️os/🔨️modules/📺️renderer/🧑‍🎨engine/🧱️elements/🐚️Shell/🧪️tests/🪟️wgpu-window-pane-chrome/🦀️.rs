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
    app.default_layout = Some(WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![window("pane-top", "Top", 33.333), window("pane-perspective", "Perspective", 66.667)] }) });
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

/// 🗣️ The split-pane fixture with ONE engagement payload on `pane-top` — a status line (React's
/// `engagement` prop) and a typed line (React's `search` prop), which is the minimum that mounts the
/// two top panes at all.
pub(super) fn engaged_pane_shell() -> ShellState {
    let mut shell = split_pane_shell();
    shell.window_engagements.insert(
        "pane-top".into(),
        WindowEngagement {
            session_active: None,
            options: None,
            input: Some(ui_wgpu::wgpu::WindowEngagementInput { id: Some("pane-engagement".into()), value: Some(String::new()), placeholder: None, disabled: None, on_change: None, on_submit: None, on_repeat_last: None, on_abort: None }),
            control: None,
            controls: None,
            status: Some(vec![ui_wgpu::wgpu::WindowEngagementStatus { id: "pane.status".into(), text: "Ready".into() }]),
            possible_engagements: None,
        },
    );
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
        let expected = declared["controlId"].as_str().expect("chip control id").replace("{windowSegment}", "paneTop");
        assert_eq!(chip.control_id("pane-top", true), expected, "🪟️ {name} folded control id");
        if let Some(open) = declared["controlIdOpen"].as_str() {
            assert_eq!(chip.control_id("pane-top", false), open.replace("{windowSegment}", "paneTop"), "🪟️ {name} unfolded control id");
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
    let engaged = engaged_pane_shell();
    let engaged_chips = engaged.window_pane_chips("pane-top");
    for declared in fixture["paneChips"].as_array().expect("fixture pane chips") {
        let chip = chip_of(declared["chip"].as_str().expect("chip name"));
        let entry = mounted.iter().find(|(candidate, _, _)| *candidate == chip);
        match declared["mounted"].as_str().expect("chip mount rule") {
            "always" => {
                let (_, folded, disabled) = entry.copied().expect("🪟️ an `always` chip mounts on every pane");
                assert!(folded, "🪟️ every pane rail starts folded, as React's `useState(true)` does");
                assert!(!disabled || declared["disabledWithoutBody"].as_bool().unwrap_or(false), "🪟️ only a chip the fixture allows to lose its body is ever disabled");
            }
            // 🎬️ React mounts these two `Pane`s only when their PROP is defined, and both props are
            // `undefined` for an empty payload — an unmounted pane is not a disabled one, so the bare
            // fixture (no engagement, no panel-eligible action) must carry neither chip, and the same
            // pane with a payload must carry both, enabled.
            rule @ ("withEngagementOrActionPane" | "withSearchSpec") => {
                assert!(entry.is_none(), "🎬️ a pane with no engagement and no panel-eligible action mounts no {rule} chip");
                let (_, folded, disabled) = engaged_chips.iter().find(|(candidate, _, _)| *candidate == chip).copied().unwrap_or_else(|| panic!("🎬️ a pane whose engagement carries a status line AND a typed line mounts the {rule} chip"));
                assert!(folded && !disabled, "🎬️ and it mounts folded and pressable, never React's disabled-toggle case");
            }
            _ => assert!(entry.is_none(), "🪟️ a pane with no world surface mounts no projection chip"),
        }
    }
    // 🧰️ The Actions chip's OTHER half of React's `engagement || actionPane`: a kind that declares a
    // panel-eligible action mounts it with no engagement payload at all, and the Search chip — which
    // reads only the engagement — still does not.
    let action_only = super::window_actions_search_pane_tests::actions_shell();
    let action_chips = action_only.window_pane_chips("pane-top");
    assert!(action_chips.iter().any(|(chip, _, _)| *chip == WindowPaneChip::Actions), "🧰️ `actionPane` alone mounts the Actions pane");
    assert!(!action_chips.iter().any(|(chip, _, _)| *chip == WindowPaneChip::Search), "🔎️ and it never mounts the Search pane, which React drives from the engagement's own spec");
    let mut bare = split_pane_shell();
    if let Some(session) = bare.session.as_mut() {
        session.app.utilities = Vec::new();
    }
    assert!(bare.window_pane_chips("pane-top").iter().any(|(chip, _, disabled)| *chip == WindowPaneChip::Utilities && *disabled), "🪟️ a window with no utility bar keeps the chip and disables its toggle");
    assert!(mounted.iter().any(|(chip, _, disabled)| *chip == WindowPaneChip::Utilities && !*disabled), "🪟️ a window that DOES declare utilities keeps its toggle live");
    assert!(mounted.iter().any(|(chip, _, disabled)| *chip == WindowPaneChip::WindowOptions && *disabled), "🪟️ a window that projects no measures document keeps the chip and disables its toggle");
}

/// 🎛️ **The fold-round-trip pin.** The Window Options chip is registered on BOTH sides of the fold, so
/// a rail that defaults folded is still reachable, and the ids are exactly the
/// `framework.window.<segment>.measures.{unfold,fold}` React mints. Painted through the
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
        // 🎯️ Deferred to `pane_overlay_hits` — see `ShellChromeFramePhase::PaneOverlayHits`.
        (shell.pane_overlay_hits.iter().filter_map(|hit| hit.control_id.clone()).collect::<Vec<_>>(), draw.glass_regions.len())
    };
    let mut shell = engaged_pane_shell();
    shell.measures_folded.insert("pane-top".into(), true);
    shell.window_measures_documents.clear();
    let (folded_ids, glass) = paint(&mut shell);
    assert_eq!(glass, shell.window_pane_chips("pane-top").len(), "🪟️ every MOUNTED chip is its own glass region (no world surface, so no projection chip)");
    assert!(!folded_ids.iter().any(|id| id.starts_with("framework.window.paneTop.measures.")), "🎛️ a window that projects no measures mounts the chip DISABLED and registers no hit");
    shell.measures_folded.insert("pane-top".into(), false);
    assert!(!shell.measures_rail_folded("pane-top"));
    assert_eq!(WindowPaneChip::WindowOptions.control_id("pane-top", false), "framework.window.paneTop.measures.fold");
    assert!(folded_ids.iter().any(|id| id == "framework.window.paneTop.engagement.toggle"), "🎛️ the enabled chips do register their hits: {folded_ids:?}");
}

/// 🎯️ **The dispatch pin.** Each chip's control id flips ONLY its own window's state — the fold
/// lane React gives each pane, the measures fold and the utility-bar fold.
#[test]
fn each_pane_chip_dispatches_its_own_window_state() {
    let mut shell = split_pane_shell();
    let press = |shell: &mut ShellState, control_id: String| {
        let hit = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None };
        assert!(semio_framework_async::block_on(shell.handle_shell_hit(&hit)).expect("a pane chip press never errors"), "🎯️ the shell claims its own pane chip");
    };
    assert!(shell.window_actions_folded("pane-top") && shell.window_search_folded("pane-top") && shell.utility_bar_folded("pane-top") && shell.measures_rail_folded("pane-top"), "🪟️ every pane rail starts folded, as React's `useState(true)` does");
    press(&mut shell, WindowPaneChip::Actions.control_id("pane-top", true));
    assert!(!shell.window_actions_folded("pane-top") && shell.window_actions_folded("pane-perspective"), "🎯️ Actions unfolds only its own pane");
    press(&mut shell, WindowPaneChip::Search.control_id("pane-perspective", true));
    assert!(!shell.window_search_folded("pane-perspective") && shell.window_actions_folded("pane-perspective"), "🎯️ Search unfolds only its own pane");
    assert_eq!(shell.active_window_id.as_deref(), Some("pane-perspective"), "🎯️ pressing any chip activates its own window, as React's `onPointerDownCapture` → `onActivate` does");
    assert!(!shell.search_open && matches!(shell.overlay_state, OverlayState::None), "🎯️ and it opens the WINDOW's search pane, never the shell's centred `⌘K` palette");
    press(&mut shell, WindowPaneChip::Utilities.control_id("pane-top", true));
    assert!(!shell.utility_bar_folded("pane-top") && shell.utility_bar_folded("pane-perspective"), "🎯️ Utilities unfolds only its own pane");
    press(&mut shell, WindowPaneChip::Utilities.control_id("pane-top", false));
    assert!(shell.utility_bar_folded("pane-top"), "🎯️ the unfolded chip carries the fold id");
    press(&mut shell, WindowPaneChip::WindowOptions.control_id("pane-perspective", true));
    assert!(!shell.measures_rail_folded("pane-perspective") && shell.measures_rail_folded("pane-top"), "🎯️ Window Options unfolds only its own pane");
}

/// 🪟️ **The stacking law.** A pane chip's hit row is NOT registered where it is painted: it is held in
/// `pane_overlay_hits` and flushed in its own chrome phase, which runs after the window BODIES and
/// BEFORE the docked panels. `InputState::hit_at` resolves in reverse registration order, so that one
/// order is React's whole three-level stack: `--z-window: 10` under `--z-pane: 20` under
/// `--z-panel: 30` (`🖱️ui/🎨️styling/🖌️ui/🎨️.css:834-836`). A chip therefore takes the pointer from
/// the world surface it is painted over, and a docked panel floating over the chip takes it back —
/// React says so where it mounts them: "window pane toggles stay on their authored anchors behind
/// anchored chrome panels; panels paint above `z-window` and occlude overlap without shifting pane
/// chrome" (`🪟️Window/🟦️.tsx:200`).
///
/// 🩸️ The rows used to be flushed AFTER the panels, which made a covered chip outrank the panel over
/// it. Measured on the React reference with the Catalogue panel open over the top pane, pressing
/// `framework.window.puzzle3dMainTop.{engagement,search}.toggle` journalled the catalogue row's own
/// `addObjectKind` and left every fold closed (`🗑️generated/parity-run-12/steps.json` steps 9-10) —
/// the press never reaches the chip there, and on this renderer it did
/// (`📓️w12d-pane-chip-ids-and-projection-toggle.md` §1).
#[test]
fn a_pane_chips_hit_row_is_flushed_after_its_window_body_and_before_the_panels_that_occlude_it() {
    let theme = Theme::light();
    let window = Rect::new(0.0, 0.0, 800.0, 600.0);
    let mut shell = engaged_pane_shell();
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
    assert!(input.staged_hits().is_empty(), "🪟️ painting a chip registers NOTHING where it paints");
    let deferred: Vec<String> = shell.pane_overlay_hits.iter().filter_map(|hit| hit.control_id.clone()).collect();
    assert!(deferred.iter().any(|id| id == &WindowPaneChip::Actions.control_id("pane-top", true)), "🪟️ the Actions chip's row is held for its own registration phase: {deferred:?}");
    assert!(shell.pane_overlay_hits.iter().all(|hit| hit.kind == HitKind::Toggle && hit.rect.w > 0.0), "🪟️ and it is a complete row, not a placeholder");

    // 🎯️ The window's own body row is already registered when the chips are flushed, so a chip
    // outranks the surface it floats on.
    let body = HitTarget { rect: window, event: None, control_id: Some("pane-top".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None };
    input.register_hit(body);
    let mut frame = ShellChromeFrameCursor { phase: ShellChromeFramePhase::PaneOverlayHits, setup: 0, child: ShellChromeChildCursor::default(), ..ShellChromeFrameCursor::default() };
    let mut overlay = DrawList::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    for _ in 0..4096 {
        shell.render_chrome_step(&mut frame, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, &mut world_resources);
        if !matches!(frame.phase, ShellChromeFramePhase::PaneOverlayHits) {
            break;
        }
    }
    assert!(matches!(frame.phase, ShellChromeFramePhase::Panels), "🪟️ the chips' rows are flushed BEFORE the panels that occlude them, never after");
    assert!(shell.pane_overlay_hits.is_empty(), "🪟️ and the whole held row is drained in that one phase");
    let registered: Vec<String> = input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).collect();
    let chip_index = registered.iter().position(|id| id == &WindowPaneChip::Actions.control_id("pane-top", true)).expect("🪟️ the Actions chip registered its row");
    let body_index = registered.iter().position(|id| id == "pane-top").expect("🪟️ the window body registered its own row");
    assert!(body_index < chip_index, "🪟️ the body is registered first, so `hit_at`'s reverse scan answers the chip over it: {registered:?}");
    let chip_rect = input.staged_hits()[chip_index].rect;
    let centre = (chip_rect.x + chip_rect.w * 0.5, chip_rect.y + chip_rect.h * 0.5);
    let frame_rows: Vec<HitTarget<ActionDescriptor>> = input.staged_hits().to_vec();
    input.publish_hits();
    assert_eq!(input.hit_at(centre.0, centre.1).and_then(|hit| hit.control_id.clone()), Some(WindowPaneChip::Actions.control_id("pane-top", true)), "🪟️ and a point on the chip answers the chip, not the body under it");
    // 📑️ The docked panels are walked AFTER this phase, so the very same point answers the PANEL —
    // React's `z-panel: 30` over `z-pane: 20`.
    for row in frame_rows {
        input.register_hit(row);
    }
    input.register_hit(HitTarget { rect: window, event: None, control_id: Some("framework.panelTab.framework.panel.catalogue".into()), kind: HitKind::PanelTab, drag_axis: None, drag_data: None });
    input.publish_hits();
    assert_eq!(input.hit_at(centre.0, centre.1).and_then(|hit| hit.control_id.clone()), Some("framework.panelTab.framework.panel.catalogue".to_string()), "📑️ a panel floating over a chip takes the pointer from it, exactly as React's stacking does");
}

/// 🆔️ **The id round-trip law.** Every chip's published id decodes back to the very window instance and
/// chip it was minted for, and only for a window this shell actually carries. React resolves its chips
/// through closures; a canvas renderer has nothing but the id, so the id has to be BOTH React's exact
/// spelling and losslessly resolvable — `elementIdSegment` is not invertible, so the segment is matched
/// against the live instances instead (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w9c-behaviour-parity-run-2.md` steps 9–12).
#[test]
fn every_pane_chip_id_decodes_back_to_its_own_window_and_chip() {
    let shell = split_pane_shell();
    for chip in WindowPaneChip::ALL {
        for folded in [true, false] {
            let id = chip.control_id("pane-top", folded);
            assert!(id.starts_with("framework."), "🆔️ {chip:?} publishes React's namespace, never a shell-private one: {id}");
            assert_eq!(shell.window_pane_chip_target(&id), Some(("pane-top".to_string(), chip)), "🆔️ {id} decodes to its own window and chip");
            assert_eq!(shell.window_pane_chip_target(&chip.control_id("pane-perspective", folded)), Some(("pane-perspective".to_string(), chip)), "🆔️ and the sibling pane is never confused with it");
        }
    }
    assert_eq!(shell.window_pane_chip_target("framework.window.paneGone.engagement.toggle"), None, "🆔️ a window this shell no longer carries resolves to nothing");
    assert_eq!(shell.window_pane_chip_target("framework.window.paneTop.windowControls.close"), None, "🆔️ and a window control is not a pane chip");
    assert_eq!(shell.window_pane_chip_target("framework.window.paneTop.pane.fold"), None, "🆔️ the projection fold is only a projection fold under its own parent id");
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
        assert!(
            !shell.active_utilities.iter().any(|node| matches!(node, UtilityNode::Toggle { id, .. } | UtilityNode::Button { id, .. } | UtilityNode::Collection { id, .. } | UtilityNode::Separator { id, .. } if id == forbidden)),
            "🔚️ the shell's utility roster no longer carries the framework sync leaves either"
        );
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

/// 🎛️ **The fold-independence law.** One chip press flips ONE fold. React gives every `Pane` its own
/// `useState(true)` and couples exactly two of them — `Actions` and `Search` share `actionsFolded`
/// (`setEngagementBarFolded`, `🪟️Window/🟦️.tsx:373`/`:388`) — so no chip closes another pane's rail,
/// and no chip opens or closes a SURFACE.
///
/// 🩸️ The packet that opened this lane read run 14's React column as "the projection toggle closes
/// three surfaces". It does not: React resolves no control for the journey's `projection.toggle` key
/// at all (it spells that chip `framework.worldOrbit.projection.<segment>.pane.fold`), and the three
/// surfaces that vanished in that one run were the React host remounting mid-journey — ten earlier
/// runs record the same step as `absent` with an empty delta
/// (`🗑️generated/parity-run-{1..12}/steps.json`, `📓️w12d-pane-chip-ids-and-projection-toggle.md` §5).
#[test]
fn one_pane_chip_press_flips_one_fold_and_moves_no_surface() {
    let fixture = pane_fixture();
    let shared: Vec<WindowPaneChip> = fixture["paneFolds"]["sharedFold"].as_array().expect("fixture shared fold").iter().map(|name| chip_of(name.as_str().expect("chip name"))).collect();
    let independent: Vec<WindowPaneChip> = fixture["paneFolds"]["independentFolds"].as_array().expect("fixture independent folds").iter().map(|name| chip_of(name.as_str().expect("chip name"))).collect();
    assert!(fixture["paneFolds"]["surfacesMovedByAnyChip"].as_array().expect("fixture surface moves").is_empty(), "🎛️ the fixture states that no chip moves a surface");
    let folds = |shell: &ShellState, window_id: &str| (shell.window_actions_folded(window_id), shell.window_search_folded(window_id), shell.measures_rail_folded(window_id), shell.utility_bar_folded(window_id), shell.projection_pane_folded(window_id));
    let press = |shell: &mut ShellState, chip: WindowPaneChip, window_id: &str| {
        let control_id = chip.control_id(
            window_id,
            match chip {
                WindowPaneChip::Actions => shell.window_actions_folded(window_id),
                WindowPaneChip::Search => shell.window_search_folded(window_id),
                WindowPaneChip::WindowOptions => shell.measures_rail_folded(window_id),
                WindowPaneChip::Utilities => shell.utility_bar_folded(window_id),
                WindowPaneChip::Projection => shell.projection_pane_folded(window_id),
            },
        );
        let hit = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None };
        assert!(semio_framework_async::block_on(shell.handle_shell_hit(&hit)).expect("a pane chip press never errors"), "🎯️ the shell claims {chip:?}");
    };
    for chip in WindowPaneChip::ALL {
        let mut shell = split_pane_shell();
        let surfaces_before = shell.chrome_surface_census();
        let before = folds(&shell, "pane-top");
        press(&mut shell, chip, "pane-top");
        let after = folds(&shell, "pane-top");
        let moved = [after.0 != before.0, after.1 != before.1, after.2 != before.2, after.3 != before.3, after.4 != before.4];
        let expected = [chip == WindowPaneChip::Actions, chip == WindowPaneChip::Search, chip == WindowPaneChip::WindowOptions, chip == WindowPaneChip::Utilities, chip == WindowPaneChip::Projection];
        assert_eq!(moved, expected, "🎛️ {chip:?} flips its OWN fold and no other: {before:?} -> {after:?}");
        assert_eq!(folds(&shell, "pane-perspective"), (true, true, true, true, true), "🎛️ and never the sibling pane's");
        assert_eq!(shell.chrome_surface_census(), surfaces_before, "🎛️ {chip:?} opens and closes no surface");
        assert!(!shell.search_open && matches!(shell.overlay_state, OverlayState::None), "🎛️ and raises no shell overlay");
    }
    assert!(shared.is_empty(), "🎛️ the mounted React panes share no fold");
    for chip in &independent {
        assert!(!shared.contains(chip), "🎛️ a fold is shared or independent, never both");
    }
}

/// 🧰️ **The rail-census law.** Unfolding the `Utilities` chip publishes exactly what React's unfolded
/// `Pane` names: the chip flipping to its `…utilityBar.fold` id, and ONE row per enabled utility leaf
/// carrying that leaf's own framework element id. Folding it again removes precisely those rows.
///
/// ⚖️ React's DOM census counts 14 further `data-slot` scaffolding nodes on the same gesture
/// (`ribbon-zone`, `inline-label`, …). They name no control and are not published here — the same
/// reading `📓️w9c-behaviour-parity-run-2.md` §3 established for the panels.
#[test]
fn unfolding_the_utilities_chip_publishes_reacts_own_rail_census() {
    let fixture = pane_fixture();
    let census = &fixture["utilityRailCensus"];
    assert!(census["leafElementIdIsUtilityId"].as_bool().expect("fixture leaf id rule"), "🧰️ React names a leaf by its utility id");
    let theme = Theme::light();
    let mut shell = split_pane_shell();
    let mut atlas = FontAtlas::builtin();
    let panes = planned_panes(&mut shell, &theme, &mut atlas, 1440.0, 900.0);
    let (window_id, body) = panes.first().cloned().expect("the split plans a first pane");
    let published = |shell: &mut ShellState| {
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut chips = ShellChromeChildCursor::default();
        for _ in 0..4096 {
            if shell.paint_window_pane_chips_step(&mut chips, &mut draw, &mut atlas, &icons, &mut input, &theme, &window_id, body) {
                break;
            }
        }
        let mut rail = ShellChromeChildCursor::default();
        for _ in 0..4096 {
            if shell.paint_window_utilities_step(&mut rail, &mut draw, &mut atlas, &icons, &mut input, &theme, &window_id, body) {
                break;
            }
        }
        let mut ids: Vec<String> = shell.pane_overlay_hits.drain(..).chain(input.staged_hits().iter().cloned()).filter_map(|hit| hit.control_id.clone()).collect();
        ids.sort();
        ids
    };
    let folded = published(&mut shell);
    shell.utility_bar_folded.insert(window_id.clone(), false);
    let unfolded = published(&mut shell);
    let added: Vec<&String> = unfolded.iter().filter(|id| !folded.contains(id)).collect();
    let removed: Vec<&String> = folded.iter().filter(|id| !unfolded.contains(id)).collect();
    let session = shell.session.clone().expect("fixture session");
    let leaves: Vec<String> = shell
        .derive_window_utility_nodes(&session, &window_id)
        .into_iter()
        .filter_map(|node| match node {
            UtilityNode::Toggle { id, .. } | UtilityNode::Button { id, .. } => Some(id),
            _ => None,
        })
        .collect();
    assert!(!leaves.is_empty(), "🧰️ the fixture app declares utilities to publish");
    let mut expected_added: Vec<String> = census["addedControlIds"].as_array().expect("fixture added ids").iter().map(|id| id.as_str().expect("added id").replace("{windowSegment}", &semio_framework::element_id_segment(&window_id))).collect();
    expected_added.extend(leaves.iter().map(|id| format!("{WINDOW_UTILITY_RAIL_PARENT}toggle.{id}")));
    expected_added.sort();
    let mut added_owned: Vec<String> = added.into_iter().cloned().collect();
    added_owned.sort();
    assert_eq!(added_owned, expected_added, "🧰️ unfolding publishes the fold chip and one row per utility leaf, and nothing else");
    let expected_removed: Vec<String> = census["removedControlIds"].as_array().expect("fixture removed ids").iter().map(|id| id.as_str().expect("removed id").replace("{windowSegment}", &semio_framework::element_id_segment(&window_id))).collect();
    assert_eq!(removed.into_iter().cloned().collect::<Vec<String>>(), expected_removed, "🧰️ and the unfold chip is the only row it takes away");
    for leaf in &leaves {
        assert!(expected_added.iter().any(|id| id.ends_with(&format!(".{leaf}"))), "🧰️ every published row is tailed by React's own element id for that utility: {leaf}");
    }
}

/// 🧰️ **The rail-routing law.** A utility row is painted INSIDE a world pane's rect, so — like the
/// pane chips before it — a press on one only reaches the shell if the ownership predicate claims its
/// id. It carries the `framework.utility.` routing namespace for exactly that reason; without it every
/// press on `Transform`/`Brush` was handed to `enqueue_world3d_event` and the utility never armed
/// (`📓️w12d-pane-chip-ids-and-projection-toggle.md` §3).
#[test]
fn a_pane_utility_row_is_claimed_by_the_shell_and_not_by_the_world_under_it() {
    let rect = Rect::new(10.0, 10.0, 80.0, 24.0);
    for (control_id, kind) in
        [(format!("{WINDOW_UTILITY_RAIL_PARENT}toggle.pane.brush"), HitKind::Toggle), (format!("{WINDOW_UTILITY_RAIL_PARENT}button.pane.undo"), HitKind::Button), (format!("{WINDOW_UTILITY_RAIL_PARENT}collection.group"), HitKind::Button)]
    {
        let hit = HitTarget { rect, event: None, control_id: Some(control_id.clone()), kind, drag_axis: None, drag_data: None };
        assert!(ShellState::pointer_press_belongs_to_shell_chrome(Some(&hit)), "🧰️ {control_id} is chrome, not surface");
    }
    let world = HitTarget { rect, event: None, control_id: Some("pane-top".into()), kind: HitKind::World3d, drag_axis: None, drag_data: None };
    assert!(!ShellState::pointer_press_belongs_to_shell_chrome(Some(&world)), "🌍️ and the surface under the rail still owns its own presses");
}

/// 🪪️ **The window-control law.** Neither renderer publishes a window control GROUP in this shell: the
/// dock hands its windows no `onClose`/`onMaximize`/`onOpenInNewWindow` (focus and close live on the
/// dock TAB), and React mints the group only when it carries a control — so
/// `framework.window.<segment>.windowControls` names nothing on either side.
///
/// 🩸️ React used to mint it unconditionally from `showControls`, publishing an empty 2px-wide box in
/// every DOM census (`🗑️generated/w11a-parity-run-14/steps.json` step `pane-chip-windowcontrols`,
/// `rect: [524, 65, 2, 16]`) that a canvas renderer could only ever report as `absent`. The prop is
/// gone (`🪟️Window/🟦️.tsx:270`).
#[test]
fn no_window_publishes_an_empty_window_control_group() {
    let fixture = pane_fixture();
    let controls = &fixture["windowControls"];
    assert!(!controls["publishedInShell"].as_bool().expect("fixture window-control rule"), "🪪️ the fixture states the OS shell mints no window control group");
    let group = controls["controlId"].as_str().expect("fixture window-control id").replace("{windowSegment}", "paneTop");
    let theme = Theme::light();
    let window = Rect::new(0.0, 0.0, 800.0, 600.0);
    let mut shell = split_pane_shell();
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    for _ in 0..4096 {
        if shell.paint_window_pane_chips_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, "pane-top", window) {
            break;
        }
    }
    let ids: Vec<String> = shell.pane_overlay_hits.iter().filter_map(|hit| hit.control_id.clone()).chain(input.staged_hits().iter().filter_map(|hit| hit.control_id.clone())).collect();
    assert!(!ids.iter().any(|id| id.contains(".windowControls")), "🪪️ a pane publishes no window control row: {ids:?}");
    assert_eq!(shell.window_pane_chip_target(&group), None, "🪪️ and the group id resolves to no pane chip either");
    for item in controls["items"].as_array().expect("fixture window-control items") {
        let id = format!("{group}.{}", item.as_str().expect("window control item"));
        assert_eq!(shell.window_pane_chip_target(&id), None, "🪪️ {id} is a window control, never a pane chip");
    }
}
