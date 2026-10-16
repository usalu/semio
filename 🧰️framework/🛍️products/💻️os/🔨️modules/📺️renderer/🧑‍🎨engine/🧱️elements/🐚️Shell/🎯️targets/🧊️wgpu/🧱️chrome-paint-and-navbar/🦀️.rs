//! 🧱️ Chrome paint steps, navbar/footer layout, utilities and window pane chrome.
//!
//! Immediate-mode chrome text and icons, group painting, bands, navbar/footer layouts, the sync pill, hub
//! projections, utility rendering, keybinding formatting, navbar controls, World3d status, the safe area and the
//! window pane chrome.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-retained-chrome-text-laws/🦀️.rs"]
mod retained_chrome_text_laws;

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs"]
mod shell_chrome_parity_tests;

#[cfg(test)]
fn chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    push_chrome_group_border(draw, rect, theme);
}

#[cfg(test)]
fn measure_chrome_group_item(atlas: &mut FontAtlas, theme: &Theme, item: &ChromeGroupItem<'_>) -> f32 {
    let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
    let text_w = item.label.map(|label| atlas.measure_text(label, theme.font_size_small).0).unwrap_or(0.0);
    theme.padding_standard * 2.0 + icon_w + text_w
}

#[cfg(test)]
fn render_chrome_group(draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, rect: Rect, items: &[ChromeGroupItem<'_>], register_hits: bool) {
    if items.is_empty() {
        return;
    }
    let hair = theme.stroke_hairline;
    let inner_y = rect.y + hair;
    let inner_h = (rect.h - hair * 2.0).max(0.0);
    let mut x = rect.x;
    for (index, item) in items.iter().enumerate() {
        let item_w = measure_chrome_group_item(atlas, theme, item);
        let item_rect = Rect::new(x, inner_y, item_w, inner_h);
        let hovered = !item.disabled && item_rect.contains(input.pointer_x, input.pointer_y);
        let bg = if item.disabled { theme.overlay_shadow } else { chrome_item_bg(theme, item.active, hovered) };
        if bg.a > 0.0 {
            draw.push_solid([item_rect.x, item_rect.y, item_rect.w, item_rect.h], bg);
        }
        let text_color = if item.disabled { theme.text_muted } else { chrome_item_text(theme, item.active, hovered) };
        let mut content_x = item_rect.x + theme.padding_standard;
        if let Some(icon_id) = item.icon_id {
            chrome_icon(draw, icons, icon_id, content_x, item_rect.y + (item_rect.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, text_color);
            content_x += CHROME_ICON_TINY + theme.gap_standard;
        }
        #[cfg(test)]
        {
            if let Some(label) = item.label {
                chrome_text(draw, atlas, input, theme, label, content_x, item_rect.y + (item_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, text_color);
            }
        }
        if register_hits && !item.disabled {
            note_chrome_group_item(item);
            input.register_hit(HitTarget { rect: item_rect, event: None, control_id: Some(item.control_id.into()), kind: item.kind.clone(), drag_axis: None, drag_data: None });
        }
        x += item_w;
        if index + 1 < items.len() {
            draw.push_solid([x, inner_y, hair, inner_h], theme.border_normal);
        }
    }
    chrome_group_border(draw, rect, theme);
}

#[cfg(test)]
fn partition_utilities_by_category(utilities: &[UtilityNode]) -> [Vec<UtilityNode>; 4] {
    let mut buckets: [Vec<UtilityNode>; 4] = [vec![], vec![], vec![], vec![]];
    for utility in utilities {
        let idx = match utility.category() {
            ui_wgpu::wgpu::UtilityCategory::Selection => 0,
            ui_wgpu::wgpu::UtilityCategory::Utilities => 1,
            ui_wgpu::wgpu::UtilityCategory::History => 2,
            ui_wgpu::wgpu::UtilityCategory::Sync => 3,
        };
        buckets[idx].push(utility.clone());
    }
    buckets
}



/// 🧭️ Item 5 (ribbon nesting): a collapsed group still highlights when the active picker
/// segment lives somewhere inside it (recursively, through further nested `Collection`s) —
/// `active-path` tracking, ported from the React ribbon's recursive reconciliation.
///
/// 🧭️ Item 5: previously flattened one level (`.filter(|child| !matches!(child,
/// UtilityNode::Collection { .. }))` dropped nested `Collection`s outright) — recursing
/// on the full, unfiltered `children` supports arbitrary nesting depth; each nested
/// `Collection` gets its own `collection_expanded` entry (already a flat `id`-keyed map,
/// so no path-keying change needed) and paints its own active-path highlight in turn.
#[cfg(test)]
pub(crate) fn render_utility_nodes(
    chrome: &mut ShellChromeBuildState,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    mut x: f32,
    btn_y: f32,
    btn_h: f32,
    utilities: &[UtilityNode],
    collection_expanded: &HashMap<String, bool>,
) -> f32 {
    for utility in utilities {
        match utility {
            UtilityNode::Separator { .. } => {
                x = render_utility_section_divider(draw, theme, x, btn_y, btn_h);
            }
            UtilityNode::Button { id, icon_id, label, text, title, disabled, on_press, .. } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = utility_node_label(label, text, title, id);
                let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}button.{id}");
                let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(label_text), active: false, disabled: false, kind: HitKind::Button };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                chrome.register_element_rect(id.clone(), rect);
                input.register_hit(HitTarget { rect, event: Some(on_press.clone()), control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
                x += item_w + theme.gap_standard * 0.5;
            }
            UtilityNode::Toggle { id, icon_id, label, text, title, pressed, disabled, on_change, .. } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = utility_node_label(label, text, title, id);
                let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}toggle.{id}");
                let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(label_text), active: pressed.unwrap_or(false), disabled: false, kind: HitKind::Toggle };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                chrome.register_element_rect(id.clone(), rect);
                input.register_hit(HitTarget { rect, event: Some(on_change.clone()), control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
                x += item_w + theme.gap_standard * 0.5;
            }
            UtilityNode::Collection { id, icon_id, label, text, title, disabled, children, .. } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let expanded = collection_expanded.get(id).copied().unwrap_or(false);
                let on_active_path = expanded || utility_subtree_has_active_path(children);
                let label_text = utility_node_label(label, text, title, id);
                let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}collection.{id}");
                let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(label_text), active: on_active_path, disabled: false, kind: HitKind::Button };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                input.register_hit(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
                x += item_w + theme.gap_standard * 0.5;
                if expanded {
                    x = render_utility_nodes(chrome, draw, atlas, icons, input, theme, x, btn_y, btn_h, children, collection_expanded);
                }
            }
        }
    }
    x
}

//#region ActionPanelAndUtilities

/// 📇️ The first window kind whose resolved actions include `action_id` — the window the palette/keybinding
/// redirect focuses to open an arg-carrying action's form (Architecture Decision 8, P3/P4).
// 🎯️ Exercised today only by `🛰️Dock`'s own `action_host_window_id_finds_scoping_window` test — the
// palette/keybinding redirect call site itself hasn't landed yet, hence the matching test-only cfg.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn action_host_window_id(app: &AppDefinition, action_id: &str) -> Option<String> {
    app.window_kinds.iter().find(|kind| semio_framework::resolve_window_actions(app, kind).iter().any(|action| action.id == action_id)).map(|kind| kind.id.clone())
}

/// ⌨️ Whether a key event is one of the hardcoded shell chords (palette/find/panels/nav/surface-role/
/// mode-cycle) that must win over app-declared keybindings (P4 — "reserved shell chords still win").
///
/// The `mod+alt+e`/`mod+alt+v` and `mod+alt+←`/`mod+alt+→` rows are the wgpu twin of React's
/// `SHELL_KEYBINDINGS` entries for `playground.navbar.roles.{editor,viewer}` and
/// `ui.shell.mode.{next,previous}` — see `🧱️elements/🏛️ShellHost/🟦️.tsx` and
/// `🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json`'s `keybindings` rows, which pin both spellings.
#[cfg(test)]
pub(crate) fn is_reserved_shell_chord(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> bool {
    is_reserved_shell_chord_in(&shell_shortcut_table(&HashMap::new()), action, modifiers)
}

//#region 🔀️ChromeParity

/// 👁️✏️🔁️ **Dedup note**: the surface-role (`mod+alt+e`/`mod+alt+v`) and mode-cycle
/// (`mod+alt+→`/`mod+alt+←`) chords used to be hand-rolled predicates run AHEAD of the shortcut
/// table, which made both rows unremappable by construction (an override could add a chord but never
/// retire the hardcoded one) and matched a chord held with Shift that React's own
/// `keyboardEventMatchesChord` refuses. They are now ordinary [`SHELL_SHORTCUT_ROWS`] rows resolved
/// by [`shell_shortcut_for_in`] like every other verb.

//#endregion 🔀️ChromeParity

//#region 🪟️WindowPaneChrome

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs"]
mod navbar_footer_parity_tests;
//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities
