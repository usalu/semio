//! ⏳️ `shell-async` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
/// 🖱️ Edge-detects "pointer just went down this frame", computed once per `render_chrome` call —
/// `InputState::pointer_down` is level-triggered (true every frame the button is held), so the
/// chrome-owned click handling below (dialogs, tour controls, the tour trigger) that lives outside the
/// `ActionDescriptor`/`ShellActions` dispatch pipeline needs its own edge tracking to avoid re-firing on
/// every held frame. Reads afterward (`chrome_clicked_this_frame`) are pure.
/// ⏱️ Self-contained wall-clock reader for the tooltip hover-delay timer — deliberately not sharing the
/// file's other `*_now_ms` helper (its exact module/scope kept shifting under concurrent edits from
/// `w3-prefs-i18n-themes` while this region was being written), same cfg-gated `js_sys::Date::now()` /
/// `SystemTime` split as the rest of this file already uses elsewhere.
#[cfg(target_arch = "wasm32")]
pub fn chrome_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn chrome_now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_secs_f64() * 1000.0).unwrap_or(0.0)
}

//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
/// 🧭️ Item 5's "active-path tracking": true if `nodes` (recursively, through nested `Collection`s)
/// contains a pressed `Toggle` — used so a *collapsed* ribbon `Collection` still highlights when the
/// user's current selection lives inside it, mirroring `ui/js/react/index.tsx`'s recursive picker
/// active-path reconciliation. Called from `render_utility_nodes` (an off-limits `ShellInput`-
/// adjacent function this wave — see the report's coordination note).
#[cfg(any(test, feature = "testkit"))]
pub fn utility_subtree_has_active_path(nodes: &[UtilityNode]) -> bool {
    nodes.iter().any(|node| match node {
        UtilityNode::Toggle { pressed, .. } => pressed.unwrap_or(false),
        UtilityNode::Collection { children, .. } => utility_subtree_has_active_path(children),
        _ => false,
    })
}

/// 🎓️ Raised-cosine breathing thickness for the introduced-element pulse ring — mirrors the
/// `data-introduced` CSS keyframes (`ui/styling/js/🎨️ui.css`: hairline → focus → hairline over 1.6s,
/// ease-in-out), which are exactly a raised cosine.
#[cfg(any(test, feature = "testkit"))]
pub fn introduced_pulse_thickness(now_ms: f64, hairline: f32, focus: f32) -> f32 {
    let phase = (now_ms.rem_euclid(INTRODUCED_PULSE_PERIOD_MS) / INTRODUCED_PULSE_PERIOD_MS) as f32;
    hairline + (focus - hairline) * 0.5 * (1.0 - (phase * std::f32::consts::TAU).cos())
}

/// 👻️ Pure accept-decision for the ghost-text click affordance — factored out of
/// `render_engagement_input` so it's unit-testable without a `GpuContext` fixture (that function
/// unconditionally needs a real wgpu device, per its own `_gpu` parameter).
#[cfg(any(test, feature = "testkit"))]
pub fn engagement_ghost_accept_on_click(ghost_rect: Rect, pointer_x: f32, pointer_y: f32, clicked_this_frame: bool, query: &str, suffix: &str) -> Option<String> {
    if clicked_this_frame && ghost_rect.contains(pointer_x, pointer_y) {
        Some(format!("{query}{suffix}"))
    } else {
        None
    }
}

/// ⌨️ Ports `engagementInlineCompletion`/`engagementCompletionSuffix` (`ui/js/react/index.tsx`) to this
/// renderer: the first `possible` (in the order the host already gave them — wgpu's engagement rail has
/// no ranked-match dropdown to reorder by) whose label case-insensitively prefix-matches `query`, sliced
/// on a char boundary (never a byte index) so a multi-byte label can't panic.
#[cfg(any(test, feature = "testkit"))]
pub fn engagement_completion_suffix(query: &str, possibles: Option<&[ui_wgpu::wgpu::WindowEngagementPossible]>) -> String {
    let query = query.trim();
    if query.is_empty() {
        return String::new();
    }
    let Some(possibles) = possibles else {
        return String::new();
    };
    let query_lower = query.to_lowercase();
    for possible in possibles {
        let label_lower = possible.label.to_lowercase();
        if !label_lower.starts_with(&query_lower) {
            continue;
        }
        let query_chars = query.chars().count();
        let split_at = possible.label.char_indices().nth(query_chars).map(|(byte_index, _)| byte_index).unwrap_or(possible.label.len());
        let suffix = &possible.label[split_at..];
        if !suffix.is_empty() {
            return suffix.to_string();
        }
    }
    String::new()
}

#[cfg(any(test, feature = "testkit"))]
pub const INTRODUCED_PULSE_PERIOD_MS: f64 = 1600.0;
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome
