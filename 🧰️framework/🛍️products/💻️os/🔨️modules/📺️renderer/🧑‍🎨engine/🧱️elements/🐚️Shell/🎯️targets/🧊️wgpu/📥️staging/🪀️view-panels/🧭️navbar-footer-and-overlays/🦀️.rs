//! 🪀️ `shell-view-panels` staging of `navbar-footer-and-overlays`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl<'ctx> PanelsViewCtx<'ctx> {
    pub fn render_footer_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        let y = height - theme.footer_height - self.as_session().subfooter_height(theme);
        let btn_h = theme.control_height;
        let btn_y = y + (theme.footer_height - btn_h) * 0.5;
        match cursor.phase {
            0 => {
                draw.push_solid([0.0, y, width, theme.footer_height], theme.navbar);
                cursor.phase = 1;
            }
            1 => {
                let hovered = Rect::new(0.0, y, width, theme.footer_height).contains(input.pointer_x, input.pointer_y);
                draw.push_solid([0.0, y, width, theme.stroke_hairline], if hovered { theme.border_emphasized } else { theme.border_normal });
                cursor.item = 0;
                cursor.phase = 2;
            }
            2 => {
                let Some((anchor, node, band_rect)) = self.footer_tab_row_rect(atlas, theme, cursor.item, width, btn_y, btn_h) else {
                    cursor.item = 0;
                    cursor.rect = None;
                    cursor.phase = 3;
                    return false;
                };
                let control_id = node.id.clone();
                let item = ChromeGroupItem {
                    control_id: control_id.as_str(),
                    icon_id: Some(node.icon_id.as_str()),
                    label: Some(node.label.as_str()),
                    active: self.dock.anchor_open(anchor) && self.dock.anchor_state(anchor).path.first() == Some(&node.id),
                    disabled: false,
                    kind: HitKind::Toggle,
                };
                if cursor.rect.is_none() {
                    cursor.rect = Some(band_rect);
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_panel_chrome_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell footer panel tab exceeded the retained glyph boundary".to_string()),
                }
                self.chrome.chrome_build.register_element_rect(panel_tab_introduction_element_id(&control_id), rect);
                cursor.rect = None;
                cursor.item += 1;
                return false;
            }
            3 => {
                if self.chrome.mobile_panel_active() {
                    cursor.phase = 4;
                    return false;
                }
                let layout = self.footer_chrome_layout(atlas, theme, width, btn_y, btn_h);
                let Some(rect) = layout.presence else {
                    cursor.phase = 4;
                    return false;
                };
                let presence = self.footer_presence_rows();
                let label = semio_framework_ui_wgpu_light::presence_bar::presence_bar_chip_text(&presence, None, self.prefs.active_locale());
                match render_footer_status_step(cursor, draw, atlas, icons, input, theme, rect, "s-presence-peers", None, &label, false) {
                    Ok(false) => return false,
                    Ok(true) => cursor.phase = 4,
                    Err(()) => {
                        self.chrome.error = Some("Shell presence footer exceeded the retained chrome boundary".to_string());
                        cursor.phase = 4;
                    }
                }
            }
            4 => {
                let layout = self.footer_chrome_layout(atlas, theme, width, btn_y, btn_h);
                let hub = self.sync.hub_connection_state(&self.directory);
                let label = self.sync.hub_footer_label(&self.prefs, &self.directory);
                match render_footer_status_step(cursor, draw, atlas, icons, input, theme, layout.hub, "s-hub-connection", Some(hub.icon_id()), &label, false) {
                    Ok(false) => return false,
                    Ok(true) => cursor.phase = 5,
                    Err(()) => {
                        self.chrome.error = Some("Shell hub footer exceeded the retained chrome boundary".to_string());
                        cursor.phase = 5;
                    }
                }
            }
            5 => {
                let layout = self.footer_chrome_layout(atlas, theme, width, btn_y, btn_h);
                let Some(rect) = layout.sign_in else {
                    cursor.phase = 6;
                    return false;
                };
                let label = shell_chrome_string("hub.signIn", self.prefs.locale_id == "de");
                match render_footer_status_step(cursor, draw, atlas, icons, input, theme, rect, "framework.hub.signIn", Some("user"), label, true) {
                    Ok(false) => return false,
                    Ok(true) => cursor.phase = 6,
                    Err(()) => {
                        self.chrome.error = Some("Shell hub sign-in footer exceeded the retained chrome boundary".to_string());
                        cursor.phase = 6;
                    }
                }
            }
            6 => return true,
            _ => return false,
        }
        false
    }
}

//#endregion ShellChrome

