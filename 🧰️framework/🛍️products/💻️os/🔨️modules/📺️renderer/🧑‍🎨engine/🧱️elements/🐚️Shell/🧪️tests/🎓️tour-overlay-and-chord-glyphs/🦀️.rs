//! 🎓️ LAWS: packet W7a's three standing rules — the introduction overlay's PAINT (a veil, a measured
//! card, its copy and its four controls all present in the built frame), the control-id vocabulary the
//! parity probe resolves against React's own, and the chord-glyph formatter's platform table.
//!
//! Oracles: React's `UIIntroduction` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`), its control-keybinding rows
//! (`🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`), its `panelTabElementId`/`windowElementId`
//! (`🖥️platform/🟦️.ts`), and the shared fixture `🐚️Shell/🧫️fixtures/⌨️keybinding-glyphs/🔣️.json`,
//! which React's own suite reads too.

use super::*;

/// 🌳️ `…/🧑‍🎨engine` — the same derivation the two sibling law files use.
fn engine_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..").canonicalize().expect("engine root")
}

/// 🌳️ `🧰️framework`, where React's own `🔨️modules/🖱️ui` and `🔨️modules/🖥️platform` live.
fn framework_root() -> std::path::PathBuf {
    engine_root().join("../../../../..").canonicalize().expect("framework root")
}

fn armed_tour_shell() -> ShellState {
    let mut shell = super::appearance_tour_and_footer_pill_tests::tour_shell(Some(super::appearance_tour_and_footer_pill_tests::tour_introduction()));
    shell.screen_w = 1440.0;
    shell.screen_h = 900.0;
    shell.auto_start_introduction("tour-app", false);
    shell
}

struct PaintedTour {
    quads: Vec<[f32; 4]>,
    glass: Vec<[f32; 4]>,
    veil: Vec<ui_wgpu::wgpu::draw_types::GlassRegion>,
    foreground: Vec<[f32; 4]>,
    background: Vec<[f32; 4]>,
    hits: Vec<(String, Rect)>,
    steps: usize,
}

/// 🔬️ Drives `render_chrome_tour_step` to completion and reports what the OVERLAY draw list and the
/// hit ledger actually received — the introspection oracle this law is written against, because the
/// defect it pins is "hit rows exist, nothing is painted".
fn paint_tour(shell: &mut ShellState, theme: &Theme) -> PaintedTour {
    let mut overlay = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    let mut steps = 0usize;
    let complete = (0..8192).any(|_| {
        steps += 1;
        shell.render_chrome_tour_step(&mut cursor, &mut overlay, &mut atlas, &icons, &mut input, theme, 1440.0, 900.0)
    });
    assert!(complete, "🎓️ the tour walk terminates");
    assert_eq!(shell.error, None, "🎓️ and leaves no retained fault");
    let card_region = overlay.layers.iter().find_map(|layer| layer.foreground_of);
    let quads_of = |foreground: bool| -> Vec<[f32; 4]> { overlay.layers.iter().filter(|layer| layer.foreground_of.is_some() == foreground).flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect() };
    PaintedTour {
        quads: overlay.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect(),
        glass: overlay.glass_regions.iter().enumerate().filter(|(index, _)| card_region == Some(*index)).map(|(_, region)| region.rect).collect(),
        veil: overlay.glass_regions.iter().enumerate().filter(|(index, _)| card_region != Some(*index)).map(|(_, region)| *region).collect(),
        foreground: quads_of(true),
        background: quads_of(false),
        hits: input.staged_hits().iter().filter_map(|hit| hit.control_id.clone().map(|id| (id, hit.rect))).collect(),
        steps,
    }
}

//#region 🎨️OverlayPaintLaw

/// 🧪️ **The overlay paint law.** An armed tour puts a full-viewport veil, a glass card, its title, its
/// body and its four controls into the built frame — not just into the hit ledger.
///
/// 🩸️ The defect: a live wgpu boot answered `dumpChrome` with `shell.tour.skip`/`shell.tour.next` rows
/// at generation 191 while the canvas showed no tour at all
/// (`📓️w7a-tour-overlay-and-chord-glyphs.md` §1). Paint and hit registration are two different lanes,
/// so a law that only reads the ledger cannot tell them apart — this one reads the DrawList.
#[test]
fn an_armed_tour_paints_a_veil_a_card_and_its_controls() {
    let theme = Theme::light();
    let mut shell = armed_tour_shell();
    assert_eq!(shell.chrome_build.tour_state.as_ref().map(|tour| tour.step_index), Some(0), "🎓️ a fresh profile arms step 1");
    let painted = paint_tour(&mut shell, &theme);

    assert!(painted.veil.iter().any(|region| region.rect[2] >= 1439.0 && region.rect[3] >= 899.0), "🎓️ a screen-style step veils the WHOLE viewport: {:?}", painted.veil.len());
    assert_eq!(painted.glass.len(), 1, "🎓️ and paints exactly one glass card");
    let card = painted.glass[0];
    assert!(card[2] >= 240.0 && card[2] <= 384.0, "🎓️ the card is measured to its copy inside React's `max-w-sm`: {card:?}");
    assert!(card[3] > 0.0);
    let inside = painted.quads.iter().filter(|rect| rect[0] >= card[0] && rect[0] <= card[0] + card[2] && rect[1] >= card[1] && rect[1] <= card[1] + card[3]).count();
    assert!(inside >= 8, "🎓️ the card carries its own glyphs and chips, not an empty sheet: {inside}");

    let ids: Vec<&str> = painted.hits.iter().map(|(id, _)| id.as_str()).collect();
    assert!(ids.contains(&UI_INTRODUCTION_VEIL_CONTROL_ID), "🎓️ the veil is a real hit target: {ids:?}");
    assert!(ids.contains(&UI_INTRODUCTION_SKIP_CONTROL_ID), "🎓️ Skip sits in the cap row: {ids:?}");
    assert!(ids.contains(&UI_INTRODUCTION_NEXT_CONTROL_ID), "🎓️ a step with no interactions advances by button: {ids:?}");
    assert!(!ids.contains(&UI_INTRODUCTION_BACK_CONTROL_ID), "🎓️ step 1 has nothing to go back to: {ids:?}");

    let veil_index = ids.iter().position(|id| *id == UI_INTRODUCTION_VEIL_CONTROL_ID).expect("the veil registers");
    let skip_index = ids.iter().position(|id| *id == UI_INTRODUCTION_SKIP_CONTROL_ID).expect("Skip registers");
    assert!(veil_index < skip_index, "🎓️ the veil registers FIRST so the card's controls win `hit_at`'s reverse-order resolution: {ids:?}");
    assert!(painted.steps < 8192);
}

/// 🧪️ **The spotlight law.** A step that names an `introduce` target cuts that target out of the veil
/// and anchors the card beside it — React elevates the element above the scrim and places the box
/// through `resolveIntroductionPlacement`.
///
/// 🩸️ Both halves were production-dead: `introduction_veil_bands`/`punch_introduction_cutout` and
/// `resolve_introduction_placement` all carried `#[cfg(test)]`, and the painter drew ONE opaque
/// full-screen quad with a fixed 320×168 box centred on the viewport
/// (`📓️w4a-boot-appearance-and-tour.md` hand-off 3).
#[test]
fn an_introduced_target_is_cut_out_of_the_veil_and_the_card_moves_beside_it() {
    let theme = Theme::light();
    let mut shell = armed_tour_shell();
    let screen = paint_tour(&mut shell, &theme).glass[0];

    let anchor = Rect::new(100.0, 400.0, 120.0, 40.0);
    shell.chrome_build.register_element_rect("transform", anchor);
    let mut introduction = super::appearance_tour_and_footer_pill_tests::tour_introduction();
    introduction.steps[0].introduce = Some("transform".into());
    introduction.steps[0].placement = semio_framework::IntroductionPlacement::Right;
    if let Some(session) = shell.session.as_mut() {
        session.app.introduction = Some(introduction);
    }
    let painted = paint_tour(&mut shell, &theme);

    assert!(!painted.veil.iter().any(|region| region.rect[2] >= 1439.0 && region.rect[3] >= 899.0), "🎯️ the veil is no longer one full-screen band — it is banded around the cutout");
    assert!(painted.veil.iter().any(|region| (region.rect[0] + region.rect[2] - anchor.x).abs() < 0.01), "🎯️ a band ends exactly at the target's left edge: {:?}", painted.veil.iter().map(|region| region.rect).take(8).collect::<Vec<_>>());
    let (cx, cy) = (anchor.x + anchor.w * 0.5, anchor.y + anchor.h * 0.5);
    let reaches = |rect: &[f32; 4]| rect[0] <= cx && rect[0] + rect[2] >= cx && rect[1] <= cy && rect[1] + rect[3] >= cy;
    assert!(!painted.veil.iter().any(|region| reaches(&region.rect)), "🎯️ no veil band reaches the cutout: {:?}", painted.veil.iter().map(|region| region.rect).collect::<Vec<_>>());
    let over_target: Vec<&[f32; 4]> = painted.quads.iter().filter(|rect| reaches(rect)).collect();
    assert_eq!(over_target.len(), 1, "🎯️ exactly one quad reaches the target — its own pulsing ring, never a veil band: {over_target:?}");
    assert_eq!(*over_target[0], [anchor.x, anchor.y, anchor.w, anchor.h], "🎯️ and that quad IS the target's box, React's `data-introduced` border");
    let card = painted.glass[0];
    assert!(card[0] > anchor.x + anchor.w, "🎯️ `placement: right` puts the card to the target's right: {card:?}");
    assert!((card[0] - screen[0]).abs() > 1.0, "🎯️ an anchored step no longer sits where a centred one does");
}

/// 🧪️ **The pointer-containment law.** While the veil blocks, the shell claims the press — so nothing
/// under the card ever sees it. React's scrim is `pointer-events-auto` with its own `onPointerDown`.
///
/// 🩸️ The defect: clicking the tour's Skip on 6213 ALSO dispatched `interactionHover` +
/// `interactionSelect` into the 3D scene beneath it, because `pointer_press_belongs_to_shell_chrome`
/// claimed only four `HitKind`s and the tour's controls are `Button`
/// (`📓️w5b-interaction-parity-probe.md` §4.3).
#[test]
fn the_veil_owns_every_pointer_it_covers() {
    let theme = Theme::light();
    let mut shell = armed_tour_shell();
    let painted = paint_tour(&mut shell, &theme);
    for (id, rect) in &painted.hits {
        let hit = HitTarget { rect: *rect, event: None, control_id: Some(id.clone()), kind: HitKind::Button, drag_axis: None, drag_data: None };
        assert!(ShellState::pointer_press_belongs_to_shell_chrome(Some(&hit)), "🧯️ {id} is the shell's press, never the surface's");
    }
    let stray = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some("tree.label.seed-left-001".into()), kind: HitKind::Button, drag_axis: None, drag_data: None };
    assert!(!ShellState::pointer_press_belongs_to_shell_chrome(Some(&stray)), "🧯️ and an ordinary chrome Button still leaves the surface's press alone");

    let press = |shell: &mut ShellState, control_id: &str| {
        let hit = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some(control_id.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None };
        assert!(semio_framework_async::block_on(shell.handle_shell_hit(&hit)).expect("a tour press never errors"), "🧯️ the shell claims {control_id}");
    };
    press(&mut shell, UI_INTRODUCTION_NEXT_CONTROL_ID);
    assert_eq!(shell.chrome_build.tour_state.as_ref().map(|tour| tour.step_index), Some(1), "🎓️ Next advances");
    press(&mut shell, UI_INTRODUCTION_BACK_CONTROL_ID);
    assert_eq!(shell.chrome_build.tour_state.as_ref().map(|tour| tour.step_index), Some(0), "🎓️ Back returns");
    press(&mut shell, UI_INTRODUCTION_VEIL_CONTROL_ID);
    assert!(shell.chrome_build.tour_state.is_none(), "🧯️ a press on the veil ends the tour, as React's `onPointerDown={{skip}}` does");
    assert!(shell.chrome_build.introduction_was_seen("tour-app"), "🎓️ and persists the answer through the same door Skip uses");
    assert_eq!(shell.chrome_build.introduction_seen_writes, vec!["tour-app".to_string()], "🎓️ queued for W5a's storage door, not written twice");
}

/// 🧪️ **The checklist law.** A step that declares interactions shows React's checklist instead of a
/// Next button, and a completed interaction flips its own bullet.
#[test]
fn an_interaction_step_paints_a_checklist_and_no_next_button() {
    let theme = Theme::light();
    let mut shell = armed_tour_shell();
    let mut introduction = super::appearance_tour_and_footer_pill_tests::tour_introduction();
    introduction.steps[0].interactions = vec![
        semio_framework::IntroductionInteraction { label: "Zoom".into(), on: semio_framework::IntroductionInteractionKind::Action("zoomIn".into()), celebrate: None },
        semio_framework::IntroductionInteraction { label: "Pan".into(), on: semio_framework::IntroductionInteractionKind::Action("panLeft".into()), celebrate: None },
    ];
    if let Some(session) = shell.session.as_mut() {
        session.app.introduction = Some(introduction);
    }
    let painted = paint_tour(&mut shell, &theme);
    let ids: Vec<&str> = painted.hits.iter().map(|(id, _)| id.as_str()).collect();
    assert!(!ids.contains(&UI_INTRODUCTION_NEXT_CONTROL_ID), "🎓️ advance-by-doing hides Next, exactly as `advanceByButton` does: {ids:?}");
    assert!(ids.contains(&UI_INTRODUCTION_SKIP_CONTROL_ID), "🎓️ Skip is always offered: {ids:?}");
    let card = painted.glass[0];
    assert!(card[3] > 100.0, "🎓️ the card grew to hold its checklist: {card:?}");
}

//#endregion 🎨️OverlayPaintLaw

//#region 🆔️ControlIdParityLaw

/// 🧪️ **The control-id parity law.** Every id this shell publishes for the introduction and for a
/// panel tab is the id React publishes for the same control, read out of React's own sources — so a
/// parity probe resolves ONE vocabulary and needs no alias table.
///
/// 🩸️ The drift: `shell.tour.{skip,next,back}` against React's `ui.introduction.*`, and
/// `shell.panel.tab.<anchor>.<tabId>` against React's bare `<tabId>`
/// (`📓️w5b-interaction-parity-probe.md` §4.1–4.2).
#[test]
fn the_published_control_ids_are_reacts_own() {
    let keybindings = std::fs::read_to_string(framework_root().join("🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx")).expect("React's control-keybinding rows");
    for (id, chord) in [(UI_INTRODUCTION_SKIP_CONTROL_ID, "escape"), (UI_INTRODUCTION_NEXT_CONTROL_ID, "enter,arrowright"), (UI_INTRODUCTION_BACK_CONTROL_ID, "arrowleft")] {
        assert!(keybindings.contains(&format!("\"{id}\": \"{chord}\"")), "🆔️ React binds {id} to {chord}");
    }
    let platform = std::fs::read_to_string(framework_root().join("🔨️modules/🖥️platform/🟦️.ts")).expect("React's element-id helpers");
    assert!(platform.contains("return `framework.panelTab.${tabId}`;"), "🆔️ React's panel-tab element id appends the dotted tab id verbatim");
    assert_eq!(panel_tab_introduction_element_id("framework.panel.catalogue"), "framework.panelTab.framework.panel.catalogue");

    let shell = super::appearance_tour_and_footer_pill_tests::tour_shell(None);
    assert_eq!(shell.panel_tab_anchor("nothing.like.a.tab"), None, "🆔️ a non-tab id is not a panel tab");

    let source = std::fs::read_to_string(engine_root().join("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")).expect("the wgpu shell source");
    for dead in ["\"shell.tour.skip\"", "\"shell.tour.next\"", "\"shell.tour.back\"", "fn dock_tab_control("] {
        assert!(!source.contains(dead), "🆔️ {dead} is gone — no alias, no second vocabulary");
    }
    assert!(source.contains("format!(\"shell.panel.tab.mobile.{}\", node.id)"), "🆔️ the mobile panel keeps its own id shape, which React's mobile row also derives separately");
}

/// 🧪️ **The panel-journal law.** Opening and closing a panel journals the same two shell commands
/// React journals — `shell.panelToggle` on a visibility flip and `shell.panelTab` on a path move.
///
/// 🩸️ The wgpu shell journalled NOTHING for a panel press: the surface came and went in
/// `dumpStructure` with no action crossing `dispatch_action`
/// (`📓️w5b-interaction-parity-probe.md` §4.4).
#[test]
fn a_panel_toggle_journals_reacts_own_shell_commands() {
    assert_eq!(shell_chrome_string("shellCommand.panelToggle", false), "Toggle Panel");
    assert_eq!(shell_chrome_string("shellCommand.panelToggle", true), "Panel umschalten");
    assert_eq!(shell_chrome_string("shellCommand.panelTab", false), "Switch Panel Tab");
    assert_eq!(shell_chrome_string("shellCommand.panelTab", true), "Panel-Tab wechseln");

    let host = std::fs::read_to_string(engine_root().join("🧱️elements/🏛️ShellHost/🟦️.tsx")).expect("React's shell host");
    assert!(host.contains("noteShellCommand(\"shell.panelToggle\", shellLabel(\"ui.shellCommand.panelToggle\"), { anchor, visible: value })"), "🕒️ React journals a visibility flip");
    assert!(host.contains("noteShellCommand(\"shell.panelTab\", shellLabel(\"ui.shellCommand.panelTab\"), { anchor, tabId })"), "🕒️ and a path move");

    let source = std::fs::read_to_string(engine_root().join("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")).expect("the wgpu shell source");
    let journal = source.split("async fn journal_panel_selection").nth(1).expect("the panel journal exists");
    let journal = &journal[..journal.find("async fn select_panel_tab").unwrap_or(journal.len())];
    assert!(journal.contains("\"shell.panelToggle\"") && journal.contains("\"shell.panelTab\""), "🕒️ and so does this shell");
    assert!(source.contains("self.journal_panel_selection(anchor, tab_id, was_visible, was_path).await?"), "🕒️ every panel selection runs through it");
}

//#endregion 🆔️ControlIdParityLaw

//#region ⌨️ChordGlyphLaw

#[derive(serde::Deserialize)]
struct GlyphFixture {
    platforms: GlyphPlatforms,
    rows: Vec<GlyphRow>,
}

#[derive(serde::Deserialize)]
struct GlyphPlatforms {
    apple: Vec<String>,
    other: Vec<String>,
}

#[derive(serde::Deserialize)]
struct GlyphRow {
    keys: String,
    apple: String,
    other: String,
}

/// 🧪️ **The glyph-formatter law.** Both platform columns of the shared fixture, through this shell's
/// own formatter, plus the platform predicate that chooses between them.
///
/// 🩸️ The defect: `format_keybinding_shortcut` resolved Apple-ness with `cfg!(target_os = "macos")`,
/// a COMPILE-time fact that is false in every wasm build — so a macOS browser painted
/// `Ctrl+Alt+E`/`Ctrl+Alt+V` on the navbar role chips where React painted `⌘️⌥️E`/`⌘️⌥️V`
/// (`📓️audit-visual-parity-puzzle3d.md` §7 item 7).
#[test]
fn the_chord_formatter_answers_the_shared_platform_fixture() {
    let fixture: GlyphFixture = serde_json::from_str(include_str!("../../🧫️fixtures/⌨️keybinding-glyphs/🔣️.json")).expect("the shared glyph fixture");
    for platform in &fixture.platforms.apple {
        assert!(crate::keybinding_platform_uses_meta(platform), "⌨️ {platform} is an Apple platform");
    }
    for platform in &fixture.platforms.other {
        assert!(!crate::keybinding_platform_uses_meta(platform), "⌨️ {platform} is not");
    }
    for row in &fixture.rows {
        crate::set_host_platform_uses_meta(true);
        assert_eq!(format_keybinding_shortcut(&row.keys), row.apple, "⌨️ apple: {}", row.keys);
        crate::set_host_platform_uses_meta(false);
        assert_eq!(format_keybinding_shortcut(&row.keys), row.other, "⌨️ other: {}", row.keys);
    }
    crate::set_host_platform_uses_meta(cfg!(target_os = "macos"));
}

/// 🧪️ **The platform-door law.** The formatter reads a PUBLISHED platform, the browser hook exists,
/// and every browser door makes the read and forwards it. Source-reading for the same reason
/// `🧭️boot-axis-parity/🦀️.rs` is: `cfg(target_arch = "wasm32")` never compiles into a native test.
#[test]
fn the_platform_door_is_wired_end_to_end() {
    let renderer = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")).expect("the renderer source");
    assert!(renderer.contains("js_name = semioWgpuSetHostPlatform"), "⌨️ the browser hook is exported");
    assert!(renderer.contains("const { std::cell::Cell::new(cfg!(target_os = \"macos\")) }"), "⌨️ and a native process answers from its own OS with no door call at all");

    let shell = std::fs::read_to_string(engine_root().join("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")).expect("the wgpu shell source");
    let formatter = shell.split("fn format_keybinding_shortcut").nth(1).expect("the formatter exists");
    let formatter = &formatter[..formatter.find("\n}\n").unwrap_or(formatter.len())];
    assert!(formatter.contains("crate::host_platform_uses_meta()"), "⌨️ the formatter reads the published platform");
    assert!(!formatter.contains("cfg!(target_os"), "⌨️ and never a compile-time one");

    let descriptor = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts")).expect("the boot descriptor module");
    assert!(descriptor.contains("export function resolveWgpuHostPlatform"), "⌨️ the page-realm read lives beside the appearance one");
    assert!(descriptor.contains("userAgentData"), "⌨️ and prefers `userAgentData.platform`, as React's own predicate does");

    let worker = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts")).expect("the frame worker");
    assert!(worker.contains("loaded.semioWgpuSetHostPlatform?.(message.platform)"), "⌨️ the frame Worker applies the boot value");
    let page = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts")).expect("the page boot");
    assert!(page.contains("platform: hostPlatform()"), "⌨️ the page makes the read");
    let embed = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts")).expect("the embeddable door");
    assert!(embed.contains("semioWgpuSetHostPlatform?.(resolveWgpuHostPlatform(window))"), "⌨️ and so does the embeddable door, which runs on the page itself");
}

//#endregion ⌨️ChordGlyphLaw

//#region 🫧CrispnessLaw

/// 🧪️ **The foreground law: a tour armed means its card's glyphs are encoded in the FOREGROUND
/// pass.** Every quad the card carries — title, Skip/Next/Back chips, the counter, the body, the
/// checklist bullets and the spotlight ring — lives on a layer opened by `begin_glass_content`, which
/// is what makes the prepared ladder re-encode it into the composite AFTER the glass pass instead of
/// into the scene that pass samples.
///
/// 🩸️ The defect: the boot screenshot `🗑️generated/w7b-fix-1/run-1/shot-30s.png` shows the card and
/// its copy BLURRED while the navbar and footer chips stay crisp — the exact inverse of React
/// (`🗑️generated/react-6313/final.png`, where the backdrop blurs and `Skip` / `1 / 5` / `Next` stay
/// sharp). The card pushed its glass region and then painted its own content straight into the scene,
/// so the region blurred away the very copy it carries. `📓️w3c` §3 fixed the identical defect for the
/// window cap; the tour was never given the split.
#[test]
fn an_armed_tours_card_is_encoded_in_the_foreground_pass() {
    let theme = Theme::light();
    let mut shell = armed_tour_shell();
    let painted = paint_tour(&mut shell, &theme);
    let card = painted.glass[0];
    let inside = |rect: &[f32; 4]| rect[0] >= card[0] - 0.5 && rect[0] <= card[0] + card[2] + 0.5 && rect[1] >= card[1] - 0.5 && rect[1] <= card[1] + card[3] + 0.5;

    assert!(painted.foreground.len() >= 8, "🫧 the card's own quads are foreground content: {}", painted.foreground.len());
    assert!(painted.foreground.iter().all(inside), "🫧 and every one of them sits on the card: {:?}", painted.foreground.iter().find(|rect| !inside(rect)));
    assert!(!painted.background.iter().any(inside), "🫧 nothing the card carries is left in the scene the glass pass samples: {:?}", painted.background.iter().find(|rect| inside(rect)));
}

/// 🧪️ **The veil law.** The scrim is React's whole `ui-veil` utility, not just its fill: the blur
/// radius is `--veil-blur` and the tint is the dialog level's own surface at `--veil-alpha`.
///
/// 🩸️ `Theme::veil_blur_px()` had no production consumer at all, so the wgpu boot left the entire
/// scene razor-sharp behind an opaque sheet while React blurred the page (`📓️w4a` hand-off 2,
/// `📓️w7a` hand-off 2).
#[test]
fn the_veil_carries_reacts_own_blur_and_tint() {
    let theme = Theme::light();
    let mut shell = armed_tour_shell();
    let painted = paint_tour(&mut shell, &theme);
    assert!(!painted.veil.is_empty(), "🌫️ an armed tour veils the viewport");
    let expected = theme.veil_glass(Level::Dialog);
    for region in &painted.veil {
        assert_eq!(region.blur_px, Theme::veil_blur_px(), "🌫️ every band blurs at `--veil-blur`");
        assert_eq!(region.alpha, expected.alpha, "🌫️ and tints at `--veil-alpha`");
        assert_eq!(region.tint, expected.tint, "🌫️ with the dialog level's own surface, never a theme-agnostic black");
        assert_eq!(region.saturate, expected.saturate, "🌫️ and React's `saturate(var(--glass-saturate))`");
    }
}

/// 🧪️ **The chord-glyph raster law.** Every glyph `format_keybinding_shortcut` can emit, on BOTH
/// platform columns of the shared fixture, rasterises to a non-empty bitmap through the atlas a real
/// host boots with.
///
/// 🩸️ The defect: the navbar role chips painted `Editor □□□□E` / `Viewer □□□□V` — four `.notdef`
/// boxes for `⌘️⌥️`. No face under `🖼️assets/🔤️fonts` carries U+2318/U+2325 (nor the arrows, nor
/// `⇧⌃⎋↵⌫⌦`), React falls back to the host's system UI font, and this atlas scans none by design.
/// `📝️text`'s `SYMBOL_FACE` now owns those outlines; this law is what keeps the two tables in step —
/// a new row in the fixture that names a glyph the face lacks fails HERE, not in a screenshot.
#[test]
fn every_chord_glyph_the_formatter_emits_rasterises() {
    let fixture: GlyphFixture = serde_json::from_str(include_str!("../../🧫️fixtures/⌨️keybinding-glyphs/🔣️.json")).expect("the shared glyph fixture");
    let mut atlas = FontAtlas::shaped_default();
    let mut ignorable = 0usize;
    for row in &fixture.rows {
        for spelling in [&row.apple, &row.other] {
            for glyph in spelling.chars() {
                let (x, y, w, h, advance) = {
                    let entry = atlas.ensure_glyph(glyph, 11.2);
                    (entry.atlas_x, entry.atlas_y, entry.width, entry.height, entry.advance)
                };
                if w == 0 && h == 0 {
                    assert_eq!(advance, 0.0, "⌨️ U+{:04X} paints nothing, so it must move the pen by nothing", glyph as u32);
                    ignorable += 1;
                    continue;
                }
                let page = atlas.width;
                let inked = (0..h).any(|row| (0..w).any(|col| atlas.pixels[((y + row) * page + x + col) as usize] != 0));
                assert!(inked, "⌨️ `{spelling}` rasterised U+{:04X} as a blank box", glyph as u32);
            }
        }
    }
    assert!(ignorable > 0, "⌨️ the fixture spells its symbols with U+FE0F, which must resolve to the zero-width glyph");
}

/// 🧪️ **The same split for the confirm dialog.** It is the tour's twin — a `ui-veil` scrim over the
/// shell and a glass sheet carrying its own title, body and two buttons — and it carried the same
/// defect, so it is pinned by the same law rather than left to be rediscovered from a screenshot.
#[test]
fn a_confirm_dialogs_sheet_is_encoded_in_the_foreground_pass_over_a_blurred_scrim() {
    let theme = Theme::light();
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.chrome_build.open_dialog(ChromeDialogRequest {
        id: "confirm-1".into(),
        title: "Delete?".into(),
        body: "Sure?".into(),
        confirm_label: "Delete".into(),
        confirm_action: ActionDescriptor { controller_id: "test".into(), action: "delete".into(), args: None },
        cancel_label: "Cancel".into(),
    });
    let mut overlay = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    let complete = (0..8192).any(|_| shell.render_chrome_dialog_step(&mut cursor, &mut overlay, &mut atlas, &mut input, &theme, 1440.0, 900.0));
    assert!(complete, "🗨️ the dialog walk terminates");

    let sheet = overlay.layers.iter().find_map(|layer| layer.foreground_of).expect("🫧 the dialog opens a glass-content layer");
    let scrim: Vec<&ui_wgpu::wgpu::draw_types::GlassRegion> = overlay.glass_regions.iter().enumerate().filter(|(index, _)| *index != sheet).map(|(_, region)| region).collect();
    assert_eq!(scrim.len(), 1, "🌫️ exactly one scrim");
    assert_eq!(scrim[0].blur_px, Theme::veil_blur_px(), "🌫️ which blurs at `--veil-blur` like React's `ui-veil`");
    assert_eq!(scrim[0].rect, [0.0, 0.0, 1440.0, 900.0], "🌫️ over the whole viewport");

    let card = overlay.glass_regions[sheet].rect;
    let inside = |rect: &[f32; 4]| rect[0] >= card[0] - 0.5 && rect[0] <= card[0] + card[2] + 0.5 && rect[1] >= card[1] - 0.5 && rect[1] <= card[1] + card[3] + 0.5;
    let foreground: Vec<[f32; 4]> = overlay.layers.iter().filter(|layer| layer.foreground_of.is_some()).flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect();
    let background: Vec<[f32; 4]> = overlay.layers.iter().filter(|layer| layer.foreground_of.is_none()).flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect();
    assert!(foreground.len() >= 4, "🫧 the sheet carries its own buttons and copy: {}", foreground.len());
    assert!(foreground.iter().all(inside), "🫧 all of it on the sheet");
    assert!(!background.iter().any(inside), "🫧 and none of it left in the scene the glass pass samples");
}

/// 🔬️ The measured card of the armed step, straight off the production measurer.
fn tour_layout(shell: &mut ShellState, theme: &Theme, atlas: &mut FontAtlas) -> ChromeTourLayout {
    let step = shell.chrome_tour_active_step().expect("🎓️ a step is armed");
    let step_index = shell.chrome_build.tour_state.as_ref().map(|tour| tour.step_index).expect("🎓️ the tour carries its index");
    let step_count = shell.session.as_ref().and_then(|session| session.app.introduction.as_ref()).map(|introduction| introduction.steps.len()).expect("🎓️ the app declares its steps");
    chrome_tour_layout(shell, atlas, theme, &step, step_index, step_count, 1440.0, 900.0)
}

/// 🧪️ **The step-counter law.** The footer chip reads `1 / 2` — the step AND the total, with nothing
/// clipped off its tail.
///
/// 🩸️ The defect: the live card read `1 /` (`🗑️generated/tour-paint-1/t025.png`) against React's
/// `1 / 5` (`🗑️generated/react-6313/final.png`). The chip was priced at
/// `measure_text(counter) + padding_standard * 2`, while the painter insets its text by
/// `padding_standard * 2` on BOTH sides and clips the run to what is left — a box exactly
/// `padding_standard * 2` narrower than its own label, so the total always fell off the end. Chrome
/// text flows `RetainedTextFlow::Clip`, which drops an overflowing glyph silently rather than
/// wrapping, so nothing in the walk reports it.
#[test]
fn the_footer_counter_paints_its_whole_step_of_the_total() {
    let theme = Theme::light();
    let mut shell = armed_tour_shell();
    let mut atlas = FontAtlas::builtin();
    let layout = tour_layout(&mut shell, &theme, &mut atlas);
    assert_eq!(layout.counter_text, "1 / 2", "🎓️ React's `{{stepIndex + 1}} / {{steps.length}}`");
    let label_w = atlas.measure_text(&layout.counter_text, theme.font_size_small).0;
    let text_box = layout.counter.w - theme.padding_standard * 4.0;
    assert!(text_box >= label_w, "🎓️ the chip holds its whole label: box {text_box} for a {label_w} label");

    let painted = paint_tour(&mut shell, &theme);
    let counter = layout.counter;
    // 🔠️ Only the label: the chip's four hairline strokes all start on its own edge, so insetting by
    // one `--ui-spacing` leaves exactly the glyph quads. The vertical band is generous because a
    // bitmap-fallback cell is taller than the `--text-xs` box it sits in.
    let glyphs = painted.foreground.iter().filter(|rect| rect[0] >= counter.x + theme.padding_standard && rect[0] < counter.x + counter.w - theme.padding_standard && rect[1] > counter.y - counter.h && rect[1] < counter.y + counter.h).count();
    assert_eq!(glyphs, layout.counter_text.chars().count(), "🎓️ every scalar of `{}` is painted, none clipped off the end", layout.counter_text);
}

/// 🧪️ **The chip law.** React composes each footer control as `ButtonGroupItem` — inline label, then
/// the `ControlHotkeyBadge` chord, then the icon — so the card reads `Next ↵ ›`, and composes Skip as
/// `WindowChrome`'s `close` control, which leads with its `CloseIcon`, so it reads `✕ Skip`. The cap
/// row also carries the `DragHandle` grip between the title and the chip's edge.
///
/// 🩸️ The wgpu card painted a bare `Next` and a bare `Skip` with no icon, no chord and no grip
/// (`🗑️generated/tour-paint-1/t025.png`).
#[test]
fn the_cards_chips_carry_reacts_label_chord_icon_order() {
    let theme = Theme::light();
    let mut shell = armed_tour_shell();
    let mut atlas = FontAtlas::builtin();
    let layout = tour_layout(&mut shell, &theme, &mut atlas);

    let next = layout.next.as_ref().expect("🎓️ a step with no interactions advances by button");
    assert_eq!(next.icon_id, "chevron-right", "🎓️ React's `icon={{isLast ? \"check\" : \"chevron-right\"}}`");
    assert!(!next.leading_icon, "🎓️ `ButtonGroupItem` renders the icon LAST");
    assert_eq!(next.hotkey.as_deref(), Some(format_keybinding_shortcut("enter,arrowright").as_str()), "🎓️ the chord badge is the first chord of `ui.introduction.next`");
    assert_eq!(layout.skip.icon_id, "x", "🎓️ `WindowChrome`'s close control wears `CloseIcon`");
    assert!(layout.skip.leading_icon, "🎓️ and leads with it");
    assert_eq!(layout.skip.hotkey, None, "🎓️ the close control is not a `Button`, so it wears no inline badge");

    let icon_x = chrome_tour_chip_icon_x(next, &theme);
    let (label_x, _, label_w) = chrome_tour_chip_label_box(next, &mut atlas, &theme);
    let (_, (hotkey_x, _, hotkey_w)) = chrome_tour_chip_hotkey_box(next, &mut atlas, &theme).expect("🎓️ Next carries a chord");
    assert!(label_x + label_w <= hotkey_x, "🎓️ label before chord");
    assert!(hotkey_x + hotkey_w <= icon_x + 0.5, "🎓️ chord before icon");
    assert!(icon_x + CHROME_ICON_TINY <= next.rect.x + next.rect.w, "🎓️ and the icon inside the chip");

    assert!(layout.grip.w > 0.0 && layout.grip.x + layout.grip.w <= layout.title_chip.x + layout.title_chip.w, "🎓️ the drag grip sits at the title chip's trailing edge");
    assert!(layout.title_rect.x + layout.title_rect.w <= layout.grip.x, "🎓️ after the title, never over it");

    shell.chrome_build.tour_state.as_mut().expect("🎓️ the tour is armed").step_index = 1;
    let last = tour_layout(&mut shell, &theme, &mut atlas);
    assert_eq!(last.counter_text, "2 / 2");
    assert_eq!(last.next.as_ref().expect("🎓️ the last step still advances by button").icon_id, "check", "🎓️ the terminal step is Done, with React's own check icon");
    let back = last.back.as_ref().expect("🎓️ step 2 offers Back");
    assert_eq!(back.icon_id, "chevron-left");
    assert_eq!(back.hotkey.as_deref(), Some(format_keybinding_shortcut("arrowleft").as_str()));
}

/// 🧪️ **The overlay-sheet law.** EVERY overlay sheet that pushes a glass region encodes its own
/// glyphs in the FOREGROUND pass. A sheet that paints into the scene under its own region blurs its
/// labels away the instant it opens — the defect `📓️w8a` fixed twice (tour card, confirm dialog) and
/// listed six more times as hand-off 2: the navbar/search/find dropdown, the World3d status pill, the
/// retained context menu, the tooltip, the agent-approvals modal and the immediate-mode menu level.
///
/// The source scan is what keeps a NEW sheet honest: a bare `push_glass` that is not a veil (a veil
/// is a backdrop with no content of its own) and does not open a content layer fails here rather than
/// in a screenshot nobody takes until the sheet is open.
#[test]
fn every_overlay_sheet_with_a_glass_region_encodes_its_glyphs_in_the_foreground_pass() {
    let source = std::fs::read_to_string(engine_root().join("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")).expect("the wgpu shell source");
    let mut bare = Vec::new();
    for (index, line) in source.lines().enumerate() {
        if !line.contains("push_glass(") || line.contains("fn ") {
            continue;
        }
        let opens = line.contains("open_chrome_overlay_glass_content") || source.lines().skip(index).take(8).any(|near| near.contains("begin_glass_content"));
        if !line.contains("veil_glass(") && !opens {
            bare.push(format!("{}: {}", index + 1, line.trim()));
        }
    }
    assert!(bare.is_empty(), "🫧 every non-veil glass region opens a content layer, these do not: {bare:#?}");

    let theme = Theme::light();
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.context_menu = Some(ContextMenuState { x: 100.0, y: 100.0, items: vec![ContextMenuItem { id: "menu.one".into(), label: "Duplicate".into(), icon: Some("copy".into()), ..Default::default() }], ..Default::default() });
    let mut overlay = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut cursor = ShellChromeChildCursor::default();
    let complete = (0..8192).any(|_| shell.render_context_menu_step(&mut cursor, &mut overlay, &mut atlas, &icons, &mut input, &theme, 1440.0, 900.0));
    assert!(complete, "🖱️ the context-menu walk terminates");
    let sheet = overlay.layers.iter().find_map(|layer| layer.foreground_of).expect("🫧 the context menu opens a glass-content layer");
    let menu = overlay.glass_regions[sheet].rect;
    let inside = |rect: &[f32; 4]| rect[0] >= menu[0] - 0.5 && rect[0] <= menu[0] + menu[2] + 0.5 && rect[1] >= menu[1] - 0.5 && rect[1] <= menu[1] + menu[3] + 0.5;
    let foreground: Vec<[f32; 4]> = overlay.layers.iter().filter(|layer| layer.foreground_of.is_some()).flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect();
    let background: Vec<[f32; 4]> = overlay.layers.iter().filter(|layer| layer.foreground_of.is_none()).flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect();
    assert!(foreground.len() >= 2, "🫧 the menu's row and its label are foreground content: {}", foreground.len());
    assert!(!background.iter().any(inside), "🫧 and nothing it carries is left in the scene the glass pass samples");
    assert!(cursor.depth == 0, "🫧 the walk closed its own layer on the way out");
}

//#endregion 🫧CrispnessLaw
