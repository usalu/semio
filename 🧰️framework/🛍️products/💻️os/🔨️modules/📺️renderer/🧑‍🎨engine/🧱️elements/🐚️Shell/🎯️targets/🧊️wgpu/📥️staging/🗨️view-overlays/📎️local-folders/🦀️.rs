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
                            let event = ActionDescriptor { controller_id: "framework.sync".into(), action: action.into(), args: semio_renderer_wgpu_host_base::action_args_json!({ "documentId": offer.document_id.clone() }) };
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

    /// 🗂️ The stacked rect the listing band sits above: the folder offer's band when one shows, else the history bands.
    fn local_files_below(&self, theme: &Theme) -> Option<Rect> {
        if let Some(offer) = self.docs.folder_reconnect_band_offer(&self.history, &self.sync) {
            return Some(self.prefs.folder_reconnect_band_plan_for(&offer, theme, &self.chrome, &self.history).band);
        }
        self.prefs.history_reprojection_band_plan_for(theme, &self.chrome, &self.history).map(|(_, plan)| plan.band).or_else(|| self.history.history_time_travel.as_ref().map(|status| self.prefs.time_travel_band_plan_for(status, theme, &self.chrome).band))
    }

    /// 🗂️ Whether the listing shows: React mounts `LocalFilesV1` only in host mode (`hostMode`, a plugin filter with a host config) and only while a kept document names a surface.
    fn local_files_listed(&self) -> bool {
        self.plugins.host_config().is_some() && !self.history.local_files.is_empty()
    }

    /// 🗂️ Paints React's "Files on this device" listing, one scalar, glyph run or hit per opportunity like the reconnect band: fill, edges, the title, then per document button its fill, name and hit. A button dispatches `openKeptFile` on the shell's sync controller, so a click, the keyboard ring and an assistive technology reopen the same document; none registers under a modal dialog.
    pub fn render_local_files_band_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> bool {
        if !self.local_files_listed() {
            return true;
        }
        let below = self.local_files_below(theme);
        let scroll = self.input.scroll_offsets.get(LOCAL_FILES_SCROLL_ID).copied().unwrap_or_default();
        let Some(plan) = self.prefs.local_files_band_plan_for(&self.history.local_files, below, scroll, theme, &self.chrome) else { return true };
        let band = plan.band;
        let opening = self.opening.document_opening.as_ref().is_some_and(semio_renderer_wgpu_shell_doc_open::ShellDocumentOpening::running);
        let hair = theme.stroke_hairline;
        let baseline = |rect: Rect| rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0;
        let buttons_from = 5 + plan.lines.len();
        match cursor.scalar {
            0 => {
                overlay.push_rounded([band.x, band.y, band.w, band.h], theme.level_bg[Level::Menu.index()], theme.border_radius);
                if let Some(thumb) = plan.thumb {
                    overlay.push_solid([thumb.x, thumb.y, thumb.w, thumb.h], theme.border_normal);
                    self.input.scroll_offsets.insert(LOCAL_FILES_SCROLL_ID.into(), scroll.clamp(0.0, plan.max_first_row as f32 * plan.row_pitch));
                    input.register_hit(HitTarget { rect: band, event: None, control_id: Some(LOCAL_FILES_SCROLL_ID.into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
                }
            }
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
                        self.chrome.error = Some("Shell local files band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            scalar => {
                let Some((control_id, document_id, label, rect)) = plan.buttons.get((scalar - buttons_from) / 3).cloned() else { return true };
                match (scalar - buttons_from) % 3 {
                    0 => overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius),
                    1 => match chrome_text_complete_step(overlay, atlas, &label, rect.x + theme.padding_standard, baseline(rect), (rect.w - theme.padding_standard).max(1.0), theme.font_size_small, theme.text, &mut cursor.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => {
                            self.chrome.error = Some("Shell local files band button text exceeded the retained glyph boundary".to_string());
                            cursor.glyph.reset();
                        }
                    },
                    _ => {
                        if !opening && !self.chrome.chrome_build.dialog_open() {
                            note_chrome_control_name(control_id.as_str(), Some(label.as_str()));
                            let event = ActionDescriptor { controller_id: "framework.sync".into(), action: LOCAL_FILE_OPEN_ACTION.into(), args: semio_renderer_wgpu_host_base::action_args_json!({ "documentId": document_id }) };
                            input.register_hit(HitTarget { rect, event: Some(event), control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
                        }
                    }
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// 🔊️ The listing's accessible section, named by its title like React's `<section aria-label>`; one `current` document is not tracked here.
    pub fn local_files_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        if !self.local_files_listed() {
            return None;
        }
        Some(chrome_status_accessibility_node(node_id, LOCAL_FILES_STATUS_ID, local_files_title(self.prefs.active_locale()).to_string()))
    }
}
