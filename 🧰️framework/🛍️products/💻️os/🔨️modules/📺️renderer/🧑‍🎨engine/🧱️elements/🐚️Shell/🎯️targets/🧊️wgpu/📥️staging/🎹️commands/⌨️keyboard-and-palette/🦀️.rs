//! 🎹️ `shell-commands` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl CommandState {
    pub(crate) fn dismiss_overlays(&mut self, x: f32, y: f32, input: &mut InputState<ActionDescriptor>, input_state: &mut ShellInputState) -> bool {
        let hit = input.hit_at(x, y);
        let on_palette = hit.and_then(|hit| hit.control_id.as_deref()).is_some_and(|id| [ShellPaletteKind::Search, ShellPaletteKind::Find].into_iter().any(|kind| kind.owns_control_id(id)));
        if matches!(input_state.overlay_state, OverlayState::Search | OverlayState::Find) && !on_palette {
            self.close_command_palette(input, input_state);
            return true;
        }
        let on_overlay = on_palette || hit.is_some_and(|h| matches!(h.kind, HitKind::ContextMenu | HitKind::DropdownItem | HitKind::NavbarItem | HitKind::Select | HitKind::Input));
        if input_state.open_selects.values().any(|open| *open) && !on_overlay {
            input_state.close_open_selects();
            return true;
        }
        if input_state.context_menu.is_some() && !on_overlay {
            input_state.context_menu = None;
            return true;
        }
        if input_state.overlay_state != OverlayState::None && !on_overlay {
            self.close_command_palette(input, input_state);
            return true;
        }
        false
    }

    /// 🔍️ Find-in-content rows — React's `UIFind` ranks its own `UIFindItem[]` through the very same
    /// `rankFuzzyItems` call (same fields/weights/threshold/limit), so this is not a second algorithm.
    pub(crate) fn filtered_find_items(&self) -> Vec<ShellFindItem> {
        rank_fuzzy_items(
            self.find_items.iter().cloned().collect(),
            &self.find_query,
            &[(|item: &ShellFindItem| Some(item.label.as_str()), 2.0), (|item: &ShellFindItem| item.description.as_deref(), 1.0), (|item: &ShellFindItem| item.category.as_deref(), 0.5)],
            FUZZY_SEARCH_THRESHOLD,
            FUZZY_SEARCH_LIMIT,
        )
    }

    pub(crate) fn palette_query(&self, kind: ShellPaletteKind) -> &str {
        match kind {
            ShellPaletteKind::Search => &self.search_query,
            ShellPaletteKind::Find => &self.find_query,
        }
    }

    pub(crate) fn set_palette_query(&mut self, kind: ShellPaletteKind, value: String, input: &mut InputState<ActionDescriptor>, input_state: &mut ShellInputState) {
        match kind {
            ShellPaletteKind::Search => {
                self.search_query = value.clone();
                self.search_selected = 0;
            }
            ShellPaletteKind::Find => {
                self.find_query = value.clone();
                self.find_selected = 0;
            }
        }
        input.focus_input_owned(kind.input_id().to_string(), value);
        input_state.accessibility_focused_control_id = Some(kind.input_id().to_string());
    }

    pub(crate) fn close_command_palette(&mut self, input: &mut InputState<ActionDescriptor>, input_state: &mut ShellInputState) {
        if input_state.pending_chrome_release.as_deref().is_some_and(|id| [ShellPaletteKind::Search, ShellPaletteKind::Find].into_iter().any(|kind| kind.owns_control_id(id))) {
            input_state.pending_chrome_release = None;
        }
        input_state.overlay_state = OverlayState::None;
        self.search_open = false;
        self.find_open = false;
        input.blur_input();
        input_state.accessibility_focused_control_id = None;
    }
}

//#endregion ShellInput

