//! 🐢️ The `UiDirtyScope` selection + union law, driven by the language-agnostic fixture
//! `🧫️fixtures/🐢️ui-dirty-scope/🔣️.json` that `🎠️kernel/🟦️.ts`'s TypeScript twin drives too.
//!
//! The defect this pins: a shell may READ the field and a shell may THROW IT AWAY, and both boot,
//! paint and pass every other gate. The wgpu shell's `refresh_ui` re-rendered every window and every
//! panel leaf on every settle — 116 of 137 renders per converging edit answered `patched=0`, and the
//! flow window was re-minted eight times at ~525 000 intake phases each, ≈1.7 s of a 5.4 s
//! `flush-deferred` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
//! `📓️wgpu-edit-convergence-perf-2026-09-14.md` §7).

use super::*;

#[derive(serde::Deserialize)]
struct SurfacesFixture {
    #[serde(rename = "windowBodies")]
    window_bodies: Vec<String>,
    #[serde(rename = "panelBodies")]
    panel_bodies: Vec<String>,
}

#[derive(serde::Deserialize)]
struct FlagsFixture {
    utilities: bool,
    tools: bool,
    engagements: bool,
    measures: bool,
    labels: bool,
}

#[derive(serde::Deserialize)]
struct SelectionFixture {
    id: String,
    scope: UiDirtyScope,
    #[serde(rename = "windowBodies")]
    window_bodies: Vec<String>,
    #[serde(rename = "panelBodies")]
    panel_bodies: Vec<String>,
    flags: FlagsFixture,
    catalogue: bool,
    #[serde(rename = "asksForNothing")]
    asks_for_nothing: bool,
}

#[derive(serde::Deserialize)]
struct UnionFixture {
    id: String,
    first: UiDirtyScope,
    second: UiDirtyScope,
    union: UiDirtyScope,
}

#[derive(serde::Deserialize)]
struct ScopeFixture {
    surfaces: SurfacesFixture,
    selections: Vec<SelectionFixture>,
    unions: Vec<UnionFixture>,
    laws: Vec<String>,
}

fn fixture() -> ScopeFixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/🐢️ui-dirty-scope/🔣️.json")).expect("ui-dirty-scope fixture JSON")
}

const FLAGS: [(UiDirtySection, fn(&FlagsFixture) -> bool); 5] = [
    (UiDirtySection::Utilities, |flags| flags.utilities),
    (UiDirtySection::Tools, |flags| flags.tools),
    (UiDirtySection::Engagements, |flags| flags.engagements),
    (UiDirtySection::Measures, |flags| flags.measures),
    (UiDirtySection::Labels, |flags| flags.labels),
];

/// ⚖️ Every declared selection, section by section, against the whole surface census — so "renders
/// exactly that surface" is asserted as an equality over the census, never as a spot check on the
/// one body the case happens to name.
#[test]
fn a_scope_selects_exactly_the_surfaces_the_fixture_names() {
    let fixture = fixture();
    for case in &fixture.selections {
        let windows: Vec<String> = fixture.surfaces.window_bodies.iter().filter(|body| case.scope.wants_window_body(body)).cloned().collect();
        assert_eq!(windows, case.window_bodies, "window bodies for {}", case.id);
        let panels: Vec<String> = fixture.surfaces.panel_bodies.iter().filter(|body| case.scope.wants_panel_body(body)).cloned().collect();
        assert_eq!(panels, case.panel_bodies, "panel bodies for {}", case.id);
        for (section, expected) in FLAGS {
            assert_eq!(case.scope.wants_section(section), expected(&case.flags), "{section:?} for {}", case.id);
        }
        assert_eq!(case.scope.wants_catalogue(), case.catalogue, "catalogue for {}", case.id);
        assert_eq!(case.scope.asks_for_nothing(), case.asks_for_nothing, "asks-for-nothing for {}", case.id);
    }
}

/// ⚖️ A coalesced pass covers the UNION of everything asked for while it ran — body-key order
/// included, so the Rust and TypeScript requests cannot drift into different section orders.
#[test]
fn two_scopes_union_exactly_as_the_fixture_declares() {
    let fixture = fixture();
    for case in &fixture.unions {
        assert_eq!(case.first.clone().merged_with(case.second.clone()), case.union, "union for {}", case.id);
    }
}

/// ⚖️ A missing `uiScope` is `full`, never `none` — the one default that keeps an unmodified program
/// rendering everything instead of silently rendering nothing.
#[test]
fn an_absent_scope_deserializes_to_full() {
    #[derive(serde::Deserialize)]
    struct Carrier {
        #[serde(default)]
        ui_scope: UiDirtyScope,
    }
    let parsed: Carrier = serde_json::from_str("{}").expect("carrier without a ui scope");
    assert_eq!(parsed.ui_scope, UiDirtyScope::Full);
    assert!(!parsed.ui_scope.asks_for_nothing());
}

/// 🩸️ The shell this oracle was written against — one that renders EVERY surface on every settle —
/// must fail it. Without this the suite would stay green against the exact defect it exists to
/// forbid: 116 of 137 renders per converging edit answering `patched=0`.
#[test]
fn a_shell_that_renders_every_surface_fails_this_oracle() {
    let fixture = fixture();
    let renders_everything = fixture.selections.iter().all(|case| case.window_bodies == fixture.surfaces.window_bodies && case.panel_bodies == fixture.surfaces.panel_bodies);
    assert!(!renders_everything, "the oracle must contain at least one scope that selects FEWER surfaces than the census");
    let discriminating = fixture.selections.iter().filter(|case| case.window_bodies != fixture.surfaces.window_bodies).count();
    assert!(discriminating >= 3, "only {discriminating} scope(s) select less than the whole census");
}

/// 📜️ The fixture's own law list is the declaration both renderers answer to; an empty one would
/// make every assertion above vacuous.
#[test]
fn the_fixture_declares_its_laws() {
    let fixture = fixture();
    assert_eq!(fixture.laws.len(), 5);
    assert!(fixture.laws.iter().any(|law| law == "a-settle-that-dirties-one-window-renders-exactly-that-surface"));
    assert!(fixture.laws.iter().any(|law| law == "a-scope-less-settle-still-renders-every-surface"));
    assert!(fixture.laws.iter().any(|law| law == "a-none-scope-opens-no-refresh-pass-at-all"));
}
