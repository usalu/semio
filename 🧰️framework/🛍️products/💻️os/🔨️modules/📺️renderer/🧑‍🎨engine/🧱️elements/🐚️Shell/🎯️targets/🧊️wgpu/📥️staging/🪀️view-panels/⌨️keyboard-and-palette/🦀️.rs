//! 🪀️ `shell-view-panels` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use semio_ui_wgpu_input::{TREE_DRAG_GHOST_ICON_KEY, TREE_DRAG_GHOST_LABEL_KEY};
use semio_renderer_wgpu_ui_engine::retained_ui_engine::{drag_ghost_position, retained_tree_drop_places, retained_tree_drop_preview, tree_drop_band, DropPlaceState, DropRect, TreeBand};

//#region ShellInput
impl<'ctx> PanelsViewCtx<'ctx> {
    /// 🫳️ Paints the tree drag: every row the drag can be dropped on is tinted `DROP_TINT_AVAILABLE_PERCENT`, the band of the row under the pointer `DROP_TINT_ACTIVE_PERCENT`, and the copy travels with the pointer as menu glass with the chip tint, all rectangles inside their frames.
    pub fn render_tree_drag_overlay_node(&self, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) {
        let tint = |state: DropPlaceState| theme.accent.with_alpha(theme.accent.a * state.tint_alpha());
        if !self.input.pointer_input_is_modal() {
            let retained = input.hit_at(input.pointer_x, input.pointer_y).and_then(|hit| self.input.retained_hit_window(hit));
            if let Some((window_id, body)) = retained {
                for [x, y, w, h] in retained_tree_drop_places(&window_id) {
                    overlay.push_solid([body.x + x, body.y + y, w, h], tint(DropPlaceState::Available));
                }
                if let Some([x, y, w, h]) = retained_tree_drop_preview(&window_id) {
                    overlay.push_solid([body.x + x, body.y + y, w, h], tint(DropPlaceState::Active));
                }
            }
        }
        let Some(drag) = &self.input.tree_drag else {
            return;
        };
        if let Some(hit) = input.hit_at(drag.x, drag.y) {
            if hit.control_id.as_deref().and_then(tree_hit_item_id).is_some() {
                let row = DropRect::from_frame([hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h]);
                let band = match drag.drop_position {
                    TreeDropPosition::Before => TreeBand::Before,
                    TreeDropPosition::After => TreeBand::After,
                    TreeDropPosition::Inside => TreeBand::Inside,
                };
                overlay.push_solid(row.frame(), tint(DropPlaceState::Available));
                overlay.push_solid(tree_drop_band(row, band).frame(), tint(DropPlaceState::Active));
            }
        }
        let [x, y] = drag_ghost_position(drag.x, drag.y);
        let label = drag.drag_data.get(TREE_DRAG_GHOST_LABEL_KEY).map(String::as_str).unwrap_or_default();
        let icon = drag.drag_data.get(TREE_DRAG_GHOST_ICON_KEY).map(String::as_str).filter(|icon| icons.icon_uv(icon).is_some());
        let (height, pad, icon_size) = (theme.chip_row_height(), theme.padding_standard, theme.font_size_small);
        let icon_span = if icon.is_some() { icon_size + theme.gap_standard } else { 0.0 };
        let width = (atlas.measure_text(label, theme.font_size_small).0 + icon_span + pad * 2.0).clamp(64.0, 240.0);
        let ghost = overlay.push_glass([x, y, width, height], 0.0, theme.drag_ghost_glass());
        overlay.begin_glass_content(ghost);
        if let Some(icon) = icon {
            chrome_icon(overlay, icons, icon, x + pad, y + (height - icon_size) * 0.5, icon_size, theme.text);
        }
        chrome_text(overlay, atlas, input, theme, label, x + pad + icon_span, y + (height + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
        overlay.end_glass_content();
    }
}

//#endregion ShellInput

