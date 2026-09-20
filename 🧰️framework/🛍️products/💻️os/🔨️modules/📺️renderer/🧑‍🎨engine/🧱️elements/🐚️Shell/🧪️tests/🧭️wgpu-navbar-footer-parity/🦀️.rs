//! 🧭️ LAWS: packet W6a's five chrome-parity rules — every painted chip is hit-testable, the navbar
//! centre reads React's humanised title (and its role badge, and its role chips' chords), the footer
//! reads React's composition in React's order, and the app's introduction survives the manifest this
//! renderer loads.
//!
//! Oracles: `🐚️Shell/🧫️fixtures/🪟️window-pane-chrome/🔣️.json` (extended with the footer order measured
//! off React's DOM in `📓️w3b-chip-text-and-footer-dock.md` §3), React's own
//! `🛠️ShellHelpers/🟦️.tsx`/`🏛️ShellHost/🟦️.tsx` sources, and the puzzle plugin's authored descriptor.

use super::*;

/// 🌳️ `…/🧑‍🎨engine` — the same derivation `🌓️appearance-tour-and-footer-pills/🦀️.rs` uses.
fn engine_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..").canonicalize().expect("engine root")
}

fn repo_root() -> std::path::PathBuf {
    engine_root().join("../../../../../..").canonicalize().expect("repo root")
}

fn chrome_geometry_fixture() -> Value {
    let path = repo_root().join("🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔝️navbar-centered-band/🔣️.json");
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))).expect("navbar centered-band fixture")
}

/// 🌍️ A pane that hosts a world surface, so the Projection chip mounts at all.
fn world_pane_shell() -> ShellState {
    let mut shell = super::window_pane_chrome_tests::split_pane_shell();
    let _ = shell.world3d_states.try_insert("pane-top".into(), World3dState::new("pane-top".into(), "pane.controller".into()));
    shell
}

fn paint_pane_chips(shell: &mut ShellState, theme: &Theme, window: Rect) -> Vec<String> {
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut cursor = ShellChromeChildCursor::default();
    for _ in 0..8192 {
        if shell.paint_window_pane_chips_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, theme, "pane-top", window) {
            break;
        }
    }
    // 🎯️ A chip's hit row is DEFERRED to `pane_overlay_hits` and registered after the panels
    // (`ShellChromeFramePhase::PaneOverlayHits`), so the pane's own overlay outranks a panel floating
    // over it — the ledger this reads is the one that walk drains.
    let _ = input.staged_hits();
    shell.pane_overlay_hits.iter().filter_map(|hit| hit.control_id.clone()).collect()
}

//#region 🎯️HitTargetLaw

/// 🎯️ **The hit-target law.** EVERY chip a pane paints is registered in the hit ledger — the ledger
/// the shell publishes to `dumpChrome` and the one `hit_at` walks — unless the fixture itself says
/// that chip may lose its body and be disabled.
///
/// 🩸️ This is `📓️audit-visual-parity-puzzle3d.md` §3's P0: `Projection` painted a bordered pill at
/// the pane's bottom-right with the same fill and border every sibling chip has, and appeared in NONE
/// of the 43 rows a live `dumpChrome` returned, because `window_pane_chips` mounted it permanently
/// disabled and `paint_window_pane_chips_step` registers no hit for a disabled chip. React never
/// disables it: `WorldOrbitProjectionSwitchPane` passes no `toggleDisabled` and is handed a spec
/// SYNTHESISED from the camera when the camera carries none.
#[test]
fn every_painted_pane_chip_registers_a_hit_target() {
    let theme = Theme::light();
    let window = Rect::new(0.0, 0.0, 800.0, 600.0);
    let mut shell = world_pane_shell();
    shell.window_measures_documents.clear();
    let mounted = shell.window_pane_chips("pane-top");
    assert!(mounted.iter().any(|(chip, _, _)| *chip == WindowPaneChip::Projection), "🎯️ a world pane mounts the Projection chip");
    let ids = paint_pane_chips(&mut shell, &theme, window);
    for (chip, folded, disabled) in &mounted {
        let control_id = chip.control_id("pane-top", *folded);
        if *disabled {
            assert!(!ids.contains(&control_id), "🎯️ a DISABLED chip registers nothing: {control_id}");
            continue;
        }
        assert!(ids.contains(&control_id), "🎯️ {chip:?} is painted, so it must be hit-testable — registered ids: {ids:?}");
    }
    assert!(ids.contains(&"framework.worldOrbit.projection.paneTop.pane.fold".to_string()), "🎯️ the Projection chip wears React's own pane id, `world3dProjectionPaneElementId` + `…pane.fold`: {ids:?}");
}

/// 🔀️ **The projection-fold law.** Pressing the chip flips THAT pane's own fold and nothing else —
/// React's `onFoldToggle={() => setFolded((value) => !value)}` on a pane-local `useState`, with ONE
/// control id on both sides of the fold (`Pane`'s `chromeToggleId = toggleId ?? childElementId(id,
/// "pane", "fold")`). Unfolded, the pane paints React's whole template taxonomy, and choosing a row
/// moves the chip's own icon, which is where React shows the live spec too.
#[test]
fn the_projection_chip_folds_its_own_pane_and_switches_its_template() {
    let theme = Theme::light();
    let window = Rect::new(0.0, 0.0, 800.0, 600.0);
    let mut shell = world_pane_shell();
    let press = |shell: &mut ShellState, control_id: String| {
        let hit = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None };
        assert!(semio_framework_async::block_on(shell.handle_shell_hit(&hit)).expect("a projection press never errors"), "🔀️ the shell claims its own projection chip");
    };
    assert!(shell.projection_pane_folded("pane-top"), "🔀️ React's pane starts folded");
    assert_eq!(WindowPaneChip::Projection.control_id("pane-top", true), WindowPaneChip::Projection.control_id("pane-top", false), "🔀️ React derives ONE toggle id, never a fold-direction pair");

    let rows = |shell: &mut ShellState| {
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut cursor = ShellChromeChildCursor::default();
        for _ in 0..8192 {
            if shell.paint_window_projection_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, "pane-top", window) {
                break;
            }
        }
        input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).collect::<Vec<_>>()
    };
    assert!(rows(&mut shell).is_empty(), "🔀️ a folded projection pane paints no body");

    press(&mut shell, WindowPaneChip::Projection.control_id("pane-top", true));
    assert!(!shell.projection_pane_folded("pane-top"), "🔀️ the press unfolds this pane");
    assert!(shell.projection_pane_folded("pane-perspective"), "🔀️ and leaves its sibling alone");
    let painted = rows(&mut shell);
    assert_eq!(painted.len(), WORLD_PROJECTION_TEMPLATES.len(), "🔀️ the unfolded pane paints React's whole taxonomy: {painted:?}");
    for (row, template) in painted.iter().zip(WORLD_PROJECTION_TEMPLATES) {
        assert_eq!(row, &format!("shell.projection.template.pane-top::{}", template.id), "🔀️ the rows keep React's depth-first template order");
    }

    assert_eq!(shell.world_projection_template_id("pane-top"), WORLD_PROJECTION_DEFAULT_TEMPLATE_ID, "🔀️ a pane with no selection reads React's own `worldProjectionDefaults(\"threePoint\")` fallback");
    assert_eq!(shell.window_pane_chip_icon_id(WindowPaneChip::Projection, "pane-top"), "projection-three-point");
    assert_eq!(shell.world3d_states.get("pane-top").map(infinite_world::world::world3d_camera_projection), Some(ui_wgpu::wgpu::CameraProjection3d::Perspective), "🔀️ the pane opens on its delivered family");
    press(&mut shell, "shell.projection.template.pane-top::orthographic".into());
    assert_eq!(shell.world_projection_template_id("pane-top"), "orthographic");
    // 🔀️ The switch is not a label: it moves the pane's CAMERA onto the other family and queues the
    // settle that publishes the new pose, exactly as React's remount does.
    assert_eq!(shell.world3d_states.get("pane-top").map(infinite_world::world::world3d_camera_projection), Some(ui_wgpu::wgpu::CameraProjection3d::Orthographic), "🔀️ pressing a Parallel row makes the pane's camera parallel");
    assert!(shell.world3d_states.get("pane-top").is_some_and(infinite_world::world::world3d_pending_camera_settle), "🔀️ and queues the camera settle that dispatches it");
    assert_eq!(shell.window_pane_chip_icon_id(WindowPaneChip::Projection, "pane-top"), "projection-orthographic", "🔀️ the chip wears the selected template's icon, as React's `worldProjectionSpecIconId(spec)` pane icon does");
    assert_eq!(shell.world_projection_template_id("pane-perspective"), WORLD_PROJECTION_DEFAULT_TEMPLATE_ID, "🔀️ the selection is per pane");

    press(&mut shell, WindowPaneChip::Projection.control_id("pane-top", false));
    assert!(shell.projection_pane_folded("pane-top"), "🔀️ the same id folds it again");
}

/// 🛑️ **The overlay-press law.** A press on a pane's OWN overlay chip belongs to the shell, never to
/// the engine surface the chip is painted over — React gets this for free because a `Pane` chip is a
/// DOM button in a layer above the `<canvas>` and the canvas never sees the press at all.
///
/// 🩸️ `pointer_press_belongs_to_shell_chrome` matched on chrome KINDS alone, and a pane chip is
/// `HitKind::Toggle`. So `🧊️renderer/🦀️.rs`'s press path handed every chip inside a world pane's
/// rect to `enqueue_world3d_event` and returned before `handle_shell_hit` ran: `Actions`, `Search`,
/// `Window Options`, `Utilities` and `Projection` all painted, all hit-tested, and all dispatched
/// NOTHING. Measured live at 1440×900 (`📓️w9b-projection-pane-framing-grid-materials.md` §1): one
/// `wgpu-shell pointer button` line for the whole click, `down=false`, and a hit ledger frozen at 43
/// rows through ten retries.
#[test]
fn a_pane_chip_press_over_an_engine_surface_belongs_to_the_shell() {
    let mut shell = world_pane_shell();
    shell.window_measures_documents.clear();
    let chrome = |control_id: &str, kind: HitKind| {
        let hit = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some(control_id.to_string()), kind, drag_axis: None, drag_data: None };
        ShellState::pointer_press_belongs_to_shell_chrome(Some(&hit))
    };
    for (chip, folded, disabled) in shell.window_pane_chips("pane-top") {
        if disabled {
            continue;
        }
        let control_id = chip.control_id("pane-top", folded);
        assert!(chrome(&control_id, HitKind::Toggle), "🛑️ {chip:?}'s press is the shell's: {control_id}");
    }
    assert!(chrome("shell.projection.template.pane-top::orthographic", HitKind::DropdownItem), "🛑️ and so is a row of the body it opens");
    assert!(chrome("ui.introduction.skip", HitKind::Button), "🛑️ the tour veil still owns every pointer while it blocks");

    assert!(!chrome("pane-top", HitKind::World3d), "🛑️ the surface's OWN region is never chrome");
    assert!(!chrome("pane-top", HitKind::ScrollRegion), "🛑️ nor its scroll region");
    assert!(!chrome("pane-top.vfs.chevron.root", HitKind::Generic), "🛑️ nor a target the SURFACE minted under its own id — every scene keys its targets by `surface_id` (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`)");
    assert!(!ShellState::pointer_press_belongs_to_shell_chrome(None), "🛑️ empty canvas is nobody's press");
}

/// ⏱️ **The one-frame-body law.** An unfolded projection pane publishes its WHOLE taxonomy inside one
/// bounded run of the chrome walk — every row a hit, every row on the one measured column, and the
/// run under [`WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES`].
///
/// 🩸️ The body measured `world_projection_column_width` — a walk of all 15 labels — once per ROW,
/// so one worker turn paid 225 `FontAtlas::measure_text` calls to answer the same number 15 times,
/// and a pane too short for the taxonomy abandoned the body on its FIRST row instead of clamping to
/// its own inset, which paints nothing at all (`📓️w8b` §7.2's live "no row within 14 s").
#[test]
fn an_unfolded_projection_pane_publishes_its_rows_within_one_frame_budget() {
    let theme = Theme::light();
    let mut atlas = FontAtlas::builtin();
    let column = world_projection_column_width(&mut atlas, &theme);
    let body = |shell: &mut ShellState, window: Rect| {
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut cursor = ShellChromeChildCursor::default();
        let mut opportunities = 0_usize;
        while opportunities < WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES {
            opportunities += 1;
            if shell.paint_window_projection_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, "pane-top", window) {
                break;
            }
        }
        (opportunities, input.staged_hits().to_vec())
    };

    let mut shell = world_pane_shell();
    shell.projection_pane_folded.insert("pane-top".into(), false);
    let (opportunities, rows) = body(&mut shell, Rect::new(0.0, 0.0, 800.0, 600.0));
    assert!(opportunities < WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES, "⏱️ the body terminates well inside its own budget, it does not exhaust it: {opportunities}");
    assert_eq!(rows.len(), WORLD_PROJECTION_TEMPLATES.len(), "⏱️ and publishes every row of React's taxonomy");
    for (row, template) in rows.iter().zip(WORLD_PROJECTION_TEMPLATES) {
        let indent = f32::from(template.depth) * theme.tree_indent_per_level;
        let expected = 800.0 - theme.panel_inset - column + indent;
        assert!((row.rect.x - expected).abs() < 0.5, "⏱️ every row hangs off ONE measured column trailing edge, indented per level: {} at {} wanted {expected}", template.id, row.rect.x);
    }
    let heights: Vec<f32> = rows.iter().map(|row| row.rect.y).collect();
    assert!(heights.windows(2).all(|pair| pair[1] > pair[0]), "⏱️ and reads top-down in React's declared order: {heights:?}");

    // 🧭️ A pane too short for 15 rows clamps to its own inset and still paints every row it can —
    // it never abandons the body, which is what left the live pane blank.
    let mut short = world_pane_shell();
    short.projection_pane_folded.insert("pane-top".into(), false);
    let (_, clamped) = body(&mut short, Rect::new(0.0, 0.0, 800.0, theme.control_height * 6.0));
    assert!(!clamped.is_empty(), "🧭️ a short pane still paints the rows that fit");
    assert!(clamped.len() < WORLD_PROJECTION_TEMPLATES.len(), "🧭️ and only the rows that fit");
    for row in &clamped {
        assert!(row.rect.y >= theme.panel_inset - 0.5, "🧭️ every painted row stays inside the pane: {:?}", row.rect.y);
    }
}
//#endregion 🎯️HitTargetLaw

//#region 🗺️NavbarTitle

/// 🗺️ **The title-formatter law.** The navbar centre reads the app's own breadcrumb joined by
/// `" · "` — React's `appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology))` — never the
/// artifact-dialect id.
///
/// 🩸️ wgpu painted `session.app.id`, i.e. `s.puzzle.puzzle3d@1/*#editor`, where React paints
/// `semio · puzzle · 3d` (`📓️audit-visual-parity-puzzle3d.md` §2).
#[test]
fn the_navbar_title_is_the_apps_breadcrumb_not_its_dialect_id() {
    let mut app = super::command_registry_tests::test_app(Vec::new(), Vec::new());
    app.id = "s.puzzle.puzzle3d@1/*#editor".into();
    app.breadcrumb = vec!["semio".into(), "puzzle".into(), "3d".into()];
    app.terminology_breadcrumbs = std::collections::HashMap::from([("reuse".to_string(), vec!["Entwerfen mit Bestand".to_string(), "Aggregator".to_string()])]);
    assert_eq!(shell_navbar_title(&app, "native"), "semio · puzzle · 3d");
    assert_eq!(shell_navbar_title(&app, "reuse"), "Entwerfen mit Bestand · Aggregator", "🗺️ a terminology override replaces the WHOLE breadcrumb, as React's `terminologyBreadcrumbs` does");
    assert!(!shell_navbar_title(&app, "native").contains(&app.id), "🗺️ the raw dialect id never reaches the navbar");

    let mut bare = app.clone();
    bare.breadcrumb = Vec::new();
    bare.terminology_breadcrumbs = std::collections::HashMap::new();
    assert_eq!(shell_navbar_title(&bare, "native"), "", "🗺️ an app that declares no breadcrumb renders a nameless title rather than faulting — React's own `appBreadcrumb(undefined)` contract");
}

/// 🧭️ **The navbar-walk law.** The whole centre cluster paints to completion in one bounded walk —
/// logo, title, role badge, example picker, modes, roles, fullscreen and both tab bands — leaving no
/// retained fault behind, and the leading band still opens at the navbar's own leading edge with the
/// cluster after it. Three new phases were spliced into this cursor (the title's `UiText`, the role
/// badge, and the re-entry into the example picker), so the law that matters is that the machine still
/// terminates and still registers the chips either side of the cluster.
#[test]
fn the_navbar_cluster_walks_to_completion_and_keeps_its_bands() {
    let theme = Theme::light();
    let mut shell = world_pane_shell();
    shell.screen_w = 1440.0;
    shell.sync_dock_tabs();
    let resumed_label = "Resumed Geometry Tab";
    shell.dock_tabs.tabs_mut(PanelAnchor::TopLeft).push(DockTabNode::leaf("fixture.leading.first", "First", "box", 0));
    shell.dock_tabs.tabs_mut(PanelAnchor::TopLeft).push(DockTabNode::leaf("fixture.leading.second", "Second", "box", 0));
    shell.dock_tabs.tabs_mut(PanelAnchor::TopLeft).push(DockTabNode::leaf("fixture.leading.resumed", resumed_label, "box", 0));
    shell.dock_tabs.tabs_mut(PanelAnchor::TopMiddle).push(DockTabNode::leaf("fixture.center", "Center", "box", 0));
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    let mut walked = 0;
    for _ in 0..16_384 {
        walked += 1;
        if shell.render_navbar_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, 1440.0) {
            break;
        }
    }
    assert!(walked < 16_384, "🧭️ the navbar walk terminates");
    assert_eq!(shell.error, None, "🧭️ and leaves no retained fault: {:?}", shell.error);
    let mut hits: Vec<(String, Rect)> = input.staged_hits().iter().filter_map(|hit| hit.control_id.clone().map(|id| (id, hit.rect))).collect();
    hits.sort_by(|left, right| left.1.x.total_cmp(&right.1.x));
    let ids: Vec<&str> = hits.iter().map(|(id, _)| id.as_str()).collect();
    eprintln!("[DEBUG] navbar x-order {:?}", hits.iter().map(|(id, rect)| (id, rect.x.round())).collect::<Vec<_>>());
    let fullscreen = hits.iter().position(|(id, _)| id == "ui.fullscreen.toggle").expect("🧭️ the navbar carries the fullscreen chip");
    let leading: Vec<usize> = hits.iter().enumerate().filter(|(_, (id, _))| shell.panel_tab_anchor(id) == Some(PanelAnchor::TopLeft)).map(|(index, _)| index).collect();
    assert!(leading.len() >= 3, "🧭️ three top-left tabs exercise the retained advancing cursor: {ids:?}");
    assert!(leading.iter().all(|index| *index < fullscreen), "🧭️ and stay left of the trailing band: {ids:?}");
    assert!((hits[leading[0]].1.x - theme.padding_standard).abs() < 0.01, "🧭️ the leading band still opens at the navbar's own padding, with the logo cluster after it");
    for pair in leading.windows(2) {
        let prior = hits[pair[0]].1;
        let next = hits[pair[1]].1;
        assert!((next.x - prior.x - prior.w).abs() < 0.01, "🧭️ an advancing cursor never replays a preceding tab width: {prior:?} → {next:?}");
    }
    let resumed = hits.iter().find(|(id, _)| id == "fixture.leading.resumed").expect("🧭️ resumed fixture tab");
    let resumed_item = ChromeGroupItem { control_id: "", icon_id: Some("box"), label: Some(resumed_label), active: false, disabled: false, kind: HitKind::Toggle };
    assert!((resumed.1.w - retained_panel_chrome_item_width(&mut atlas, &theme, &resumed_item).expect("measured panel tab")).abs() < 0.01, "🧭️ the physical chip includes React's trailing grip extent");
    let dock_fixture: Value = serde_json::from_str(&std::fs::read_to_string(repo_root().join("🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📐️dock-axis-geometry/🔣️.json")).expect("shared physical React dock oracle")).expect("dock geometry fixture");
    let leading_icon = dock_fixture["panelChrome"]["leadingIconPixels"].as_f64().expect("leading icon pixels") as f32;
    let grip = dock_fixture["panelChrome"]["dragHandlePixels"].as_f64().expect("drag handle pixels") as f32;
    let measured_width = theme.padding_standard * 2.0 + leading_icon + atlas.measure_text(resumed_label, theme.font_size_small).0 + theme.gap_standard * 2.0 + grip;
    assert!((resumed.1.w - measured_width).abs() < 0.01, "🧭️ the rendered hit width uses React's independently measured leading icon and drag handle sizes");
    assert!(walked > resumed_label.chars().count(), "🧭️ the multi-glyph fixture required retained cursor resumptions");
    let layout = shell.navbar_chrome_layout(&mut atlas, &theme, 1440.0);
    let middle = hits.iter().filter(|(id, _)| shell.panel_tab_anchor(id) == Some(PanelAnchor::TopMiddle)).collect::<Vec<_>>();
    assert!(!middle.is_empty(), "🧭️ TopMiddle belongs to the centered cluster");
    assert!(middle.iter().all(|(_, rect)| rect.x >= layout.center.centered.left - 0.01 && rect.x + rect.w <= layout.center.centered.right + 0.01), "🧭️ TopMiddle stays inside the centered occupied band: {middle:?} vs {:?}", layout.center.centered);
    let centered_hits = hits.iter().filter(|(id, rect)| shell.panel_tab_anchor(id) == Some(PanelAnchor::TopMiddle) || id.starts_with("playground.navbar.")).map(|(_, rect)| *rect).collect::<Vec<_>>();
    let edge_hits = hits.iter().filter(|(id, _)| shell.panel_tab_anchor(id).is_some_and(|anchor| matches!(anchor, PanelAnchor::TopLeft | PanelAnchor::TopRight)) || id == "ui.fullscreen.toggle").map(|(_, rect)| *rect).collect::<Vec<_>>();
    for center in centered_hits {
        for edge in &edge_hits {
            assert!(center.x + center.w <= edge.x + 0.01 || edge.x + edge.w <= center.x + 0.01, "🧭️ centered and edge physical hit boxes never overlap: {center:?} / {edge:?}");
        }
    }
}

/// 👁️✏️ **The role-badge law.** The title is followed by React's read-only role chip, in both
/// locales, and the badge is NOT the role switch: React gives it no id and no handler.
#[test]
fn the_navbar_role_badge_reads_reacts_frozen_pair() {
    assert_eq!(shell_surface_role_chip_text(semio_framework::manifest::AppRole::Editor, false), "Editor");
    assert_eq!(shell_surface_role_chip_text(semio_framework::manifest::AppRole::Editor, true), "Editor");
    assert_eq!(shell_surface_role_chip_text(semio_framework::manifest::AppRole::Viewer, false), "Viewer");
    assert_eq!(shell_surface_role_chip_text(semio_framework::manifest::AppRole::Viewer, true), "Betrachter");
}

/// ⌨️ **The hotkey-badge law.** Each `playground.navbar.roles.*` chip carries its chord inline, the
/// way React's `ControlHotkeyBadge` paints one inside the button for the default `hotkeys: "inline"`
/// driver — the `Editor ⌘️⌥️E` / `Viewer ⌘️⌥️V` the React DOM read shows and the wgpu boot did not.
#[test]
fn the_navbar_role_chips_carry_their_inline_hotkey_badge() {
    let table = shell_shortcut_table(&HashMap::new());
    for role in ["editor", "viewer"] {
        let control_id = format!("playground.navbar.roles.{role}");
        let badge = shell_control_hotkey_badge(&table, &control_id).unwrap_or_else(|| panic!("⌨️ {control_id} is one of the two per-button `SHELL_KEYBINDINGS` rows"));
        assert!(!badge.is_empty(), "⌨️ {control_id} resolves a chord");
        assert_eq!(badge, format_keybinding_shortcut(&table.iter().find(|(id, _)| id == &control_id).expect("the row").1), "⌨️ the badge is the SAME formatter the palette and the settings tree use");
    }
    assert!(shell_control_hotkey_badge(&table, "playground.navbar.fixture").is_none(), "⌨️ a control with no chord carries no badge, exactly as React's `useControlHotkey` answers `undefined`");

    let overridden = shell_shortcut_table(&HashMap::from([("playground.navbar.roles.viewer".to_string(), "mod+shift+9".to_string())]));
    assert_eq!(shell_control_hotkey_badge(&overridden, "playground.navbar.roles.viewer"), Some(format_keybinding_shortcut("mod+shift+9")), "⌨️ the badge follows a remap, never the frozen default");
}
//#endregion 🗺️NavbarTitle

//#region 🔚️FooterOrder

/// 🔚️ **The footer-order law.** A whole painted footer reads React's composition left to right:
/// `Display · Remote: detached | Tool · Command | No one else is here · Settings · Marketplace ·
/// History` — the leading tab band then the sync pill, the centred band, then the presence pill
/// immediately before the trailing band — and it carries no `Check In` chip at all.
///
/// 🩸️ The wgpu boot read `Remote: detached · No one else is here · Check In · Display · …`: all three
/// pills were painted AHEAD of the bands off one running cursor, and `#s-checkin` is a row of React's
/// History PANEL, never footer chrome (`📓️audit-visual-parity-puzzle3d.md` §7 item 5).
#[test]
fn the_footer_statuses_and_tabs_follow_reacts_collision_free_sequence() {
    let fixture = chrome_geometry_fixture();
    let theme = Theme::light();
    let mut shell = world_pane_shell();
    shell.screen_w = 1440.0;
    shell.sync_dock_tabs();
    let mut atlas = FontAtlas::builtin();
    let (width, height) = (1440.0_f32, 900.0_f32);
    let btn_y = height - theme.footer_height + (theme.footer_height - theme.control_height) * 0.5;
    let layout = shell.footer_chrome_layout(&mut atlas, &theme, width, btn_y, theme.control_height);
    let presence = layout.presence.expect("desktop footer keeps presence");
    assert!(presence.x + presence.w + theme.gap_standard <= layout.hub.x + 0.01);
    assert!(layout.hub.x + layout.hub.w + theme.gap_standard <= layout.bottom_right_left + 0.01);
    assert!(layout.center.centered.left >= layout.center.free.left && layout.center.centered.right <= layout.center.free.right + 0.01);

    let mut draw = DrawList::default();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    for _ in 0..8192 {
        if shell.render_footer_step(&mut cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, width, height) {
            break;
        }
    }
    let hits = input.staged_hits();
    assert_eq!(hits.iter().filter(|hit| hit.control_id.as_deref() == Some("s-sync-status")).count(), 1, "the sync item is React's bottom-left tab, never a duplicate status pill");
    assert!(!hits.iter().any(|hit| hit.control_id.as_deref() == Some("s-presence-peers") || hit.control_id.as_deref() == Some("s-hub-connection")), "ambient status children do not invent button actions");
    assert_eq!(hits.iter().filter(|hit| hit.control_id.as_deref() == Some("framework.hub.signIn")).count(), 1, "a signed-out hub is the footer's separate targetable workspace opener");
    for hit in hits {
        if shell.panel_tab_anchor(hit.control_id.as_deref().unwrap_or_default()) == Some(PanelAnchor::BottomMiddle) {
            assert!(hit.rect.x >= layout.center.centered.left - 0.01 && hit.rect.x + hit.rect.w <= layout.center.centered.right + 0.01);
        }
    }

    let order = fixture["contract"]["footerTrailingOrder"].as_array().expect("footer order").iter().map(|value| value.as_str().expect("footer order entry")).collect::<Vec<_>>();
    assert_eq!(order, vec!["presence", "hubConnection", "bottomRight"]);

    shell.screen_w = crate::dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX;
    let mobile = shell.footer_chrome_layout(&mut atlas, &theme, shell.screen_w, btn_y, theme.control_height);
    assert!(mobile.presence.is_none(), "mobile omits presence");
    assert!(mobile.hub.w > 0.0 && mobile.hub.x + mobile.hub.w <= shell.screen_w - theme.padding_standard + 0.01, "mobile retains the hub badge inside the physical band");
}

#[test]
fn the_shared_band_fixture_and_custom_cap_metrics_hold_in_rust() {
    let fixture = chrome_geometry_fixture();
    for row in fixture["bands"].as_array().expect("bands") {
        let width = row["width"].as_f64().expect("width") as f32;
        let spans = row["occupied"].as_array().expect("occupied").iter().map(|span| ShellChromeBandSpan { left: span["left"].as_f64().expect("left") as f32, right: span["right"].as_f64().expect("right") as f32 }).collect::<Vec<_>>();
        let actual = shell_chrome_free_band(width, &spans);
        assert_eq!([actual.left, actual.right], [row["band"]["left"].as_f64().expect("expected left") as f32, row["band"]["right"].as_f64().expect("expected right") as f32]);
    }
    for row in fixture["placements"].as_array().expect("placements") {
        let width = row["width"].as_f64().expect("width") as f32;
        let band = ShellChromeBandSpan { left: row["band"]["left"].as_f64().expect("left") as f32, right: row["band"]["right"].as_f64().expect("right") as f32 };
        let actual = shell_chrome_centered_band(width, &[ShellChromeBandSpan { left: 0.0, right: band.left }, ShellChromeBandSpan { left: band.right, right: width }], row["contentWidth"].as_f64().expect("content width") as f32);
        assert_eq!(actual.centered.left, row["left"].as_f64().expect("expected left") as f32);
    }
    for row in fixture["dockCapMetrics"].as_array().expect("dock cap metrics") {
        let spacing = row["spacing"].as_f64().expect("spacing") as f32;
        let control = spacing * row["controlHeightUiSpacing"].as_f64().expect("control factor") as f32;
        let padding = spacing * row["paddingUiSpacing"].as_f64().expect("padding factor") as f32;
        let navbar = spacing * row["navbarHeightUiSpacing"].as_f64().expect("navbar factor") as f32;
        let cap = control + padding * 2.0;
        assert!((cap - row["capDepth"].as_f64().expect("cap depth") as f32).abs() < 0.001);
        if row["name"].as_str().is_some_and(|name| name.contains("custom navbar")) {
            assert!((cap - navbar).abs() > 0.001, "custom navbar height cannot define dock cap depth");
        }
    }
}

//#endregion 🔚️FooterOrder

//#region 🎓️Introduction

/// 🎓️ **The introduction-survives-the-manifest law.** The puzzle plugin AUTHORS an introduction for
/// its 3d editor, and the `AppDefinition` this renderer deserialises keeps it — which is the one input
/// `auto_start_introduction` gates on (`session.app.introduction.is_some()`). A schema drift that
/// silently dropped the field would leave the wgpu shell with a tour that can never arm while React's
/// still does, and nothing else in this crate would notice.
///
/// 🔍️ The live counter-evidence this law was written against: a `dumpChrome` of the running wgpu boot
/// DID return `shell.tour.skip` / `shell.tour.next` rows (`📓️audit-visual-parity-puzzle3d.md` §1), so
/// `has_introduction` was already true on this target — this pins that it stays true.
#[test]
fn the_puzzle3d_app_carries_the_introduction_the_tour_arms_on() {
    let path = repo_root().join("✏️s/🔌️plugins/🧩️puzzle/🔣️.json");
    let descriptor: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))).expect("the puzzle descriptor is JSON");
    let apps = descriptor["manifest"]["apps"].as_array().expect("the puzzle descriptor declares apps");
    let editor = apps.iter().find(|app| app["id"].as_str() == Some("s.puzzle.puzzle3d@1/*#editor")).expect("🎓️ the puzzle descriptor declares the puzzle3d editor");
    let authored = editor.get("introduction").cloned().unwrap_or(Value::Null);
    assert!(!authored.is_null(), "🎓️ the puzzle3d editor authors an introduction");
    let introduction: semio_framework::IntroductionDefinition = serde_json::from_value(authored).expect("🎓️ it deserialises through the very `IntroductionDefinition` `AppDefinition::introduction` carries, so a schema drift that dropped the field fails here");
    assert!(!introduction.steps.is_empty(), "🎓️ the tour has steps to walk");

    let mut app = super::command_registry_tests::test_app(Vec::new(), Vec::new());
    app.id = editor["id"].as_str().expect("the authored id").to_string();
    app.breadcrumb = editor["breadcrumb"].as_array().expect("the authored breadcrumb").iter().map(|part| part.as_str().expect("breadcrumb part").to_string()).collect();
    app.introduction = Some(introduction);
    assert_eq!(app.breadcrumb, vec!["semio".to_string(), "puzzle".to_string(), "3d".to_string()], "🎓️ and the navbar title law's own oracle is the authored breadcrumb");
    assert_eq!(shell_navbar_title(&app, "native"), "semio · puzzle · 3d");

    assert!(should_auto_start_introduction(&app.id, app.introduction.is_some(), false, false, false), "🎓️ an unseen tour on a tutorial-free shell arms");
    assert!(!should_auto_start_introduction(&app.id, app.introduction.is_some(), false, true, false), "🎓️ an answered one never re-arms");
    assert!(!should_auto_start_introduction(&app.id, false, false, false, false), "🎓️ an app that authors none arms nothing");
}
//#endregion 🎓️Introduction
