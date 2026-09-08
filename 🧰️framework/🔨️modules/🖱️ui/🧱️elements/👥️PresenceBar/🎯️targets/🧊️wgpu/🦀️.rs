//! 👥 wgpu twin of the `PresenceBar` element (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS
//! lane 2-F). Unlike the `widgets`-mod chrome renderers this file sits beside (`Button`, `KeyValue`,
//! `Ring`, …), `PresenceBar` never held shell-chrome inline code to extract — it builds a plain
//! `UiNode` tree (`Stack` of `Stack`-wrapped `Text` avatars) through the same declarative builders the
//! generic `🟦️Interpreter` element (`os/renderer/engine/elements/🟦️Interpreter`) already walks for every
//! plugin surface, so it needs only the light `wgpu` feature (declarative types), never `wgpu-engine`.
//! Each peer is its own `UiStackNode` carrying `id: "peer:{actor}"`; the Interpreter's
//! `uiNodePathSegment` (`type[index]#id`) turns that into a `data-ui-path` segment on the React side —
//! the wgpu↔React parity join this ticket's contract freeze §C0 asks for — with no new `UiNode`
//! variant and no edit to the shared `component.rs`/`🟦️Interpreter` files (out of this lane's lease).
//! Wired as a CRATE-ROOT sibling module of `crate::wgpu::widgets`, `#[path]`-mounted right before
//! `pub mod widgets` in `🦀️.rs`, mirroring how `button`/`key_value`/`ring` are mounted there.
//!
//! React twin: `🟦️.tsx` in this same folder — the `🔖️Palette` region's `presence_color` below
//! is byte-for-byte mirrored by its `presenceColor` (contract freeze §C7.5's hub-assigned-index formula)
//! so both shells tint a given peer identically. Replaces the deleted FNV-hash `presence_hue_for_actor`.
//!
//! `data-ui-path` note (requested by the W0 scout): grepping `os/renderer/engine/elements/Table` and
//! the React `📊️Table` element turned up no `data-ui-path` on table rows — only `data-row-id`. The
//! scout's suspicion was correct; `data-ui-path` genuinely exists only on the generic `Interpreter`'s
//! own rendered nodes (`os/renderer/engine/elements/🟦️Interpreter/🟦️.tsx`), which is the
//! mechanism this file's `UiStackNode` ids plug into.

use crate::wgpu::{Label, Locale, LocalizedLabel, Terminology, UiNode, UiPresence, UiStackNode, UiTextNode};

/// 🧮️ Default visible-avatar cap before the "+N" overflow node takes over — mirrors the React twin's
/// `PRESENCE_BAR_DEFAULT_MAX`.
pub const PRESENCE_BAR_DEFAULT_MAX: usize = 5;

/// 🎭️ A peer's editing/viewing stance on the shared `(space, document, surface)` — mirrors the hub's
/// `SpaceRole` vocabulary (contract freeze §C1) and the React twin's `PresenceRole`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceRole {
    Author,
    Spectator,
}

/// 👤 One peer currently attached to the same `(space, document, surface)` presence scope — mirrors
/// the React twin's `PresencePeer` prop shape.
#[derive(Clone, Debug, PartialEq)]
pub struct PresencePeerRow {
    pub actor: String,
    pub user_id: Option<String>,
    pub label: String,
    pub role: Option<PresenceRole>,
    pub connected_at_ms: Option<i64>,
    /// 🎨️ Hub-assigned session-color palette index (contract freeze §C7.5) — `None` for a folder-only
    /// peer with no hub connection, which renders as index 0.
    pub color: Option<u8>,
}

//#region 🔖️Palette
/// 🎨️ Resolved HSL triple for one presence palette entry — `h` in degrees `[0, 360)`, `s`/`l` in
/// `[0.0, 1.0]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PresenceHsl {
    pub h: u16,
    pub s: f64,
    pub l: f64,
}

/// 🌓️ Selects which of `ui_styling::presence::LIGHT`/`DARK` [`presence_color`] resolves against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceAppearance {
    Light,
    Dark,
}

/// 🎨️ Deterministic per-session palette color for a hub-assigned index (contract freeze §C7.5):
/// `index % 12` selects one of the 12 base hues (`ui_styling::presence::HUES`); `index / 12` (`k`)
/// desaturates by `0.25` once the roster wraps past two full cycles and alternates lightness by
/// `±0.14` every other cycle (lighter in `Light`, darker in `Dark`). Byte-identical to the TS twin
/// `presenceColor` in `🟦️.tsx`.
// 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
pub fn presence_color(index: u8, appearance: PresenceAppearance) -> PresenceHsl {
    use ui_styling::presence;
    let base = (index % 12) as usize;
    let k = index / 12;
    let h = presence::HUES[base];
    let (base_s, base_l) = match appearance {
        PresenceAppearance::Light => presence::LIGHT,
        PresenceAppearance::Dark => presence::DARK,
    };
    let s = base_s - if k >= 2 { 0.25 } else { 0.0 };
    let l_shift = if k % 2 == 1 { 0.14 } else { 0.0 };
    let l = match appearance {
        PresenceAppearance::Light => base_l + l_shift,
        PresenceAppearance::Dark => base_l - l_shift,
    };
    PresenceHsl { h, s, l }
}

/// 🎨️ CSS custom-property reference for a peer's base-cycle palette index (`index % 12`) — only
/// meaningful when `index / 12 == 0`; callers past the first cycle render [`presence_color`]'s HSL
/// inline instead (contract freeze §C7.5).
// 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
pub fn presence_css_var(index: u8) -> String {
    format!("var(--presence-{})", index % 12)
}
//#endregion 🔖️Palette

fn presence_stack(id: String, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "horizontal".into(), gap: Some("tight".into()), padding: None, id: Some(id), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None, children })
}

fn presence_text(value: String) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

/// 👥️ Builds the `PresenceBar` roster as a `UiNode` tree: an empty-state text node when `peers` is
/// empty, else one `id: "peer:{actor}"`-carrying `UiStackNode` per visible peer (capped at `max`,
/// defaulting to [`PRESENCE_BAR_DEFAULT_MAX`]) plus a trailing overflow node past the cap. `id` becomes
/// the root `UiStackNode`'s own id (the shells pass `s-presence-peers`, contract freeze §C0).
/// `locale` resolves this element's own framework-owned copy (empty state, overflow suffix, role
/// words) — terminology-invariant, so [`Terminology::ALL`]'s first entry is used to resolve it.
pub fn build_presence_bar(id: impl Into<String>, peers: &[PresencePeerRow], max: Option<usize>) -> UiNode {
    build_presence_bar_localized(id, peers, max, Locale::default())
}

/// 🌐️ [`build_presence_bar`] with an explicit [`Locale`] — the host resolves the active locale itself
/// (native shells read it from the same source as every other framework-owned label); this is the
/// entry point that actually localizes.
pub fn build_presence_bar_localized(id: impl Into<String>, peers: &[PresencePeerRow], max: Option<usize>, locale: Locale) -> UiNode {
    let id = id.into();
    let terminology = Terminology::ALL[0];
    if peers.is_empty() {
        let empty_text = LocalizedLabel::native("No one else is here", "Niemand sonst ist hier").resolve(terminology, locale).to_string();
        return presence_stack(id, vec![presence_text(empty_text)]);
    }

    let max = max.unwrap_or(PRESENCE_BAR_DEFAULT_MAX);
    let visible_count = peers.len().min(max);
    let visible = &peers[..visible_count];
    let overflow_count = peers.len() - visible_count;

    let mut children: Vec<UiNode> = Vec::with_capacity(visible_count);
    for peer in visible {
        let text = presence_text(peer.label.clone());
        children.push(presence_stack(format!("peer:{}", peer.actor), vec![text]));
    }

    if overflow_count > 0 {
        let more_word = LocalizedLabel::native("more", "weitere").resolve(terminology, locale).to_string();
        children.push(presence_stack("peer:overflow".into(), vec![presence_text(format!("+{overflow_count} {more_word}"))]));
    }

    presence_stack(id, children)
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
