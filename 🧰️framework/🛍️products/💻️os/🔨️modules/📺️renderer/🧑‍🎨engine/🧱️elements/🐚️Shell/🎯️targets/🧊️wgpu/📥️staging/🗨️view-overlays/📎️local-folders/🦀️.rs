//! 🗨️ `shell-view-overlays` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🔖️Reconnect
/// 🆔️ The band's own nodes — React's ids (`#s-folder-reconnect-message`, `#s-folder-reconnect`, `#s-folder-forget`).
pub const FOLDER_RECONNECT_STATUS_ID: &str = "s-folder-reconnect-message";

//#endregion 🔖️Reconnect
impl<'ctx> OverlaysViewCtx<'ctx> {
    /// 📎️ Paints the reconnect band, one scalar, glyph run or hit per opportunity: fill, edges, each line of the message,
    /// then per button its fill, caption and hit. Each button dispatches its verb on the shell's sync controller, so a
    /// click, the keyboard ring and an assistive technology reconnect or forget the same way; none registers under a modal
    /// dialog.
    pub fn render_folder_reconnect_band_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> bool {
        let Some(offer) = self.docs.folder_reconnect_band_offer(&self.history, &self.sync) else { return true };
        let plan = self.prefs.folder_reconnect_band_plan_for(&offer, theme, &self.chrome, &self.history);
        let band = plan.band;
        let hair = theme.stroke_hairline;
        let baseline = |rect: Rect| rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0;
        let buttons_from = 5 + plan.lines.len();
        match cursor.scalar {
            0 => overlay.push_rounded([band.x, band.y, band.w, band.h], theme.level_bg[Level::Menu.index()], theme.border_radius),
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [band.x, band.y, band.w, hair],
                    2 => [band.x, band.y + band.h - hair, band.w, hair],
                    3 => [band.x, band.y, hair, band.h],
                    _ => [band.x + band.w - hair, band.y, hair, band.h],
                };
                overlay.push_solid(edge, theme.border_normal);
            }
            scalar if scalar < buttons_from => {
                let line = &plan.lines[scalar - 5];
                match chrome_text_complete_step(overlay, atlas, &line.text, line.rect.x, baseline(line.rect), line.rect.w, theme.font_size_small, theme.text, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.chrome.error = Some("Shell folder reconnect band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            scalar => {
                let Some((control_id, action, label, rect)) = plan.buttons.get((scalar - buttons_from) / 3).cloned() else { return true };
                match (scalar - buttons_from) % 3 {
                    0 => overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius),
                    1 => match chrome_text_complete_step(overlay, atlas, &label, rect.x + theme.padding_standard, baseline(rect), (rect.w - theme.padding_standard).max(1.0), theme.font_size_small, theme.text, &mut cursor.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => {
                            self.chrome.error = Some("Shell folder reconnect band button text exceeded the retained glyph boundary".to_string());
                            cursor.glyph.reset();
                        }
                    },
                    _ => {
                        if !self.chrome.chrome_build.dialog_open() {
                            note_chrome_control_name(control_id, Some(label.as_str()));
                            let event = ActionDescriptor { controller_id: "framework.sync".into(), action: action.into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "documentId": offer.document_id.clone() }) };
                            input.register_hit(HitTarget { rect, event: Some(event), control_id: Some(control_id.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                        }
                    }
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// 🔊️ The band's live node — React's polite `role=status` region: named by the message it paints and described by
    /// what it is about ("Folder of this document").
    pub fn folder_reconnect_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let offer = self.docs.folder_reconnect_band_offer(&self.history, &self.sync)?;
        let locale = self.prefs.active_locale();
        let mut node = chrome_status_accessibility_node(node_id, FOLDER_RECONNECT_STATUS_ID, local_folder_text(LocalFolderText::Message, local_folder_name(local_folder_path(&offer)), locale));
        node.description = Some(local_folder_text(LocalFolderText::Label, "", locale));
        Some(node)
    }
}

