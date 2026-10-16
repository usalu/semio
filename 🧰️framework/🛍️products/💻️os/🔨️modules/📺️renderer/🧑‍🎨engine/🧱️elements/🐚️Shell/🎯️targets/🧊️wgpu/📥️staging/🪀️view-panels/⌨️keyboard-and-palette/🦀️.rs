//! 🪀️ `shell-view-panels` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl<'ctx> PanelsViewCtx<'ctx> {
    pub fn render_tree_drag_overlay_node(&self, overlay: &mut DrawList, input: &InputState<ActionDescriptor>, theme: &Theme) {
        let Some(drag) = &self.input.tree_drag else {
            return;
        };
        overlay.push_solid([drag.x - 60.0, drag.y - 12.0, 120.0, 24.0], theme.selected.with_alpha(0.85));
        if let Some(hit) = input.hit_at(drag.x, drag.y) {
            if let Some(target_id) = hit.control_id.as_deref().and_then(tree_hit_item_id) {
                let _ = target_id;
                match drag.drop_position {
                    TreeDropPosition::Before => overlay.push_solid([hit.rect.x, hit.rect.y, hit.rect.w, 2.0], theme.accent),
                    TreeDropPosition::After => overlay.push_solid([hit.rect.x, hit.rect.y + hit.rect.h - 2.0, hit.rect.w, 2.0], theme.accent),
                    TreeDropPosition::Inside => overlay.push_rounded([hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h], theme.accent.with_alpha(0.15), theme.border_radius),
                }
            }
        }
    }
}

//#endregion ShellInput

