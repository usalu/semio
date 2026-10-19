//! 🪀️ `shell-view-panels` staging of `icon-export`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::{present_media_export_bytes_cancellable, shell_chrome_string, ShellDetached, ShellState};
use semio_framework_os_renderer_wgpu_icon_host::icon_export::asset::IconExportAssetRequest;
use semio_framework_os_renderer_wgpu_icon_host::icon_export::{IconExportFault, IconExportFormat, IconExportPreparedScene, IconExportScenePreparation, IconExportSceneRejected, IconGpuPngExport, IconGpuPngRejected, IconSvgExport, IconSvgRejected};
use semio_framework::kernel::IconRenderExportItem;

impl<'ctx> PanelsViewCtx<'ctx> {
    pub fn render_icon_export_step(&mut self, cursor: &mut super::ShellChromeChildCursor, overlay: &mut semio_framework_ui_draw_contract::DrawList, atlas: &mut semio_framework_ui_text::FontAtlas, input: &mut semio_framework_ui_wgpu_input::InputState<semio_framework_ui_declaration::layout::ActionDescriptor>, theme: &semio_framework_ui_theme::Theme, width: f32) -> bool {
        let Some(batch) = self.opening.icon_export.as_ref() else { return true };
        let is_de = self.prefs.locale_id == "de";
        let action_label = shell_chrome_string(if batch.running() { "icon.export.cancel" } else { "common.close" }, is_de);
        let message = batch.message(is_de);
        let severity = if batch.phase == Phase::Failed { semio_framework::Severity::Error } else { semio_framework::Severity::Info };
        let (border, fill, text) = super::transient_notice_tone(severity, theme);
        let mut band = super::document_opening_rect(&message, action_label, width, theme);
        band.y += band.h + super::PLUGIN_INSTALL_TOP_GAP;
        let action = super::plugin_install_action_rect(band, action_label, theme);
        let hair = theme.stroke_hairline;
        match cursor.scalar {
            0 => overlay.push_rounded([band.x, band.y, band.w, band.h], fill, theme.border_radius),
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [band.x, band.y, band.w, hair],
                    2 => [band.x, band.y + band.h - hair, band.w, hair],
                    3 => [band.x, band.y, hair, band.h],
                    _ => [band.x + band.w - hair, band.y, hair, band.h],
                };
                overlay.push_solid(edge, border);
            }
            5 | 6 => {
                let (value, x, max_width) = if cursor.scalar == 5 {
                    (message.as_str(), band.x + theme.padding_standard, (action.x - band.x - theme.padding_standard * 2.0).max(1.0))
                } else { (action_label, action.x, action.w.max(1.0)) };
                let baseline = band.y + (band.h + theme.font_size_small) * 0.5 - 1.0;
                match super::chrome_text_complete_step(overlay, atlas, value, x, baseline, max_width, theme.font_size_small, text, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => cursor.glyph.reset(),
                }
            }
            7 => {
                super::note_chrome_control_name(CONTROL_ID, Some(action_label));
                input.register_hit(semio_framework_ui_wgpu_input::HitTarget { rect: action, event: None, control_id: Some(CONTROL_ID.into()), kind: semio_framework_ui_wgpu_input::HitKind::Button, drag_axis: None, drag_data: None });
            }
            8 => {
                if self.chrome.chrome_build.clicked_this_frame && action.contains(input.pointer_x, input.pointer_y) { self.opening.cancel_icon_export(); }
            }
            _ => return true,
        }
        cursor.scalar += 1;
        false
    }
}

