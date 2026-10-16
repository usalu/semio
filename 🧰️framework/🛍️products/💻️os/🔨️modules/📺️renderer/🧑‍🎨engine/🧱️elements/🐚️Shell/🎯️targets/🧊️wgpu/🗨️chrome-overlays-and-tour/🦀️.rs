//! 🗨️ Chrome overlays: notices, bands, agent approvals, dialog requests and the tour.
//!
//! Tooltips, transient notices, plugin-install and document-open bands, dispatch fault notices, agent approval
//! paint ops, chrome dialog requests and helpers, the chrome tour state and element rects.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
// 🍿️ w3-overlays-chrome-polish (WP15+WP16): tooltips, a generic modal dialog, and the app introduction
// tour — layered on the existing immediate-mode chrome through `ShellChromeBuildState`, while
// `OverlayState` remains dedicated to the pre-existing menu/search overlays.
// Placement math reuses `ui_wgpu`'s w1d-events-overlay manager types (`OverlayKind`, `OverlayPlacement`,
// `resolve_overlay_placement`) even though the manager's own `EventRouter`/`open_overlay` stay
// `pub(crate)` to `ui_wgpu` (an `engine::Ui`/retained-`UiTree` implementation detail) and out of reach
// for this non-tree-based immediate-mode chrome renderer — confirmed unreachable the same way
// `report-w2-text-editor.md` found for its own local-fallback popup.

//#region 🧯️TransientNoticeAndAgentOverlays
// 🧯️ WGPU-RENDERER-REACT-PARITY packet W1j. React's `showTransientNotice` (`🏛️ShellHost/🟦️.tsx`) is
// the ONE channel every non-fatal shell fault reaches the user through — viewer-read-only,
// mutation-rejected, render-error, keybinding-unowned, a guest `Effect::Notify`, a degraded remote
// merge. This shell had no equivalent at all: such faults either set the persistent bottom-left
// `self.error` line (which never clears) or vanished. The three rules below are React's, exactly:
// ONE notice at a time (a new one replaces rather than queues), auto-dismiss after 4 s, and an
// explicit close control.



/// 📜️ The suffix of a retained panel's whole-content scroll region id (`framework.panel.history.scroll`).
pub(crate) const RETAINED_PANEL_SCROLL_SUFFIX: &str = ".scroll";

/// 🛟️ The native input draft disposition of an exact typed history refusal.
pub(crate) fn dispatch_refusal_discards_input_draft(error: &str) -> bool {
    time_travel::history_refusal_discards_input_draft(error)
}

//#endregion 🧯️TransientNoticeAndAgentOverlays

//#region 🗨️ChromeDialog

/// 🔢️ A number field's text: at its precision when it has one, else twelve significant digits.
pub(super) fn chrome_dialog_number_text(value: f64, precision: Option<u16>) -> String {
    precision.map_or_else(|| ui_contract::format_ui_number(value), |precision| ui_contract::format_ui_number_fixed(value, precision))
}

/// 🧭️ A vector component's name — React's staged vector row names them `x y z w`, then by index.
pub(super) fn chrome_dialog_axis_name(axis: usize) -> String {
    ["x", "y", "z", "w"].get(axis).map_or_else(|| axis.to_string(), |name| (*name).to_string())
}

//#endregion 🗨️ChromeDialog

/// 🧭️ Item 5's "active-path tracking": true if `nodes` (recursively, through nested `Collection`s)
/// contains a pressed `Toggle` — used so a *collapsed* ribbon `Collection` still highlights when the
/// user's current selection lives inside it, mirroring `ui/js/react/index.tsx`'s recursive picker
/// active-path reconciliation. Called from `render_utility_nodes` (an off-limits `ShellInput`-
/// adjacent function this wave — see the report's coordination note).
#[cfg(test)]
pub(crate) fn utility_subtree_has_active_path(nodes: &[UtilityNode]) -> bool {
    nodes.iter().any(|node| match node {
        UtilityNode::Toggle { pressed, .. } => pressed.unwrap_or(false),
        UtilityNode::Collection { children, .. } => utility_subtree_has_active_path(children),
        _ => false,
    })
}

#[cfg(test)]
pub(crate) const INTRODUCED_PULSE_PERIOD_MS: f64 = 1600.0;

/// 🎓️ Raised-cosine breathing thickness for the introduced-element pulse ring — mirrors the
/// `data-introduced` CSS keyframes (`ui/styling/js/🎨️ui.css`: hairline → focus → hairline over 1.6s,
/// ease-in-out), which are exactly a raised cosine.
#[cfg(test)]
pub(crate) fn introduced_pulse_thickness(now_ms: f64, hairline: f32, focus: f32) -> f32 {
    let phase = (now_ms.rem_euclid(INTRODUCED_PULSE_PERIOD_MS) / INTRODUCED_PULSE_PERIOD_MS) as f32;
    hairline + (focus - hairline) * 0.5 * (1.0 - (phase * std::f32::consts::TAU).cos())
}

/// ⌨️ Ports `engagementInlineCompletion`/`engagementCompletionSuffix` (`ui/js/react/index.tsx`) to this
/// renderer: the first `possible` (in the order the host already gave them — wgpu's engagement rail has
/// no ranked-match dropdown to reorder by) whose label case-insensitively prefix-matches `query`, sliced
/// on a char boundary (never a byte index) so a multi-byte label can't panic.
#[cfg(test)]
pub(crate) fn engagement_completion_suffix(query: &str, possibles: Option<&[ui_wgpu::wgpu::WindowEngagementPossible]>) -> String {
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

/// 👻️ Pure accept-decision for the ghost-text click affordance — factored out of
/// `render_engagement_input` so it's unit-testable without a `GpuContext` fixture (that function
/// unconditionally needs a real wgpu device, per its own `_gpu` parameter).
#[cfg(test)]
pub(crate) fn engagement_ghost_accept_on_click(ghost_rect: Rect, pointer_x: f32, pointer_y: f32, clicked_this_frame: bool, query: &str, suffix: &str) -> Option<String> {
    if clicked_this_frame && ghost_rect.contains(pointer_x, pointer_y) {
        Some(format!("{query}{suffix}"))
    } else {
        None
    }
}
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome
