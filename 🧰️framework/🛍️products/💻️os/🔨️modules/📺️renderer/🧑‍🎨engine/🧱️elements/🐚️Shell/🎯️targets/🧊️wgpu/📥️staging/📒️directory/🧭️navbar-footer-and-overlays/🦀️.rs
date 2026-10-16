//! 📒️ `shell-directory` staging of `navbar-footer-and-overlays`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl DirectoryState {
    /// 👥️🖼️ The board presence this frame paints, in paint order: every cursor (viewport, dot, chip),
    /// then every mark chip, stacked down the board's right edge by entity row like React's canvas marks.
    pub(super) fn board_presence_paint_items(&self, theme: &Theme, surfaces: &SurfaceState) -> Vec<ShellBoardPresencePaint> {
        let mut items = Vec::new();
        for (bounds, overlays) in self.board_peer_overlays(surfaces) {
            items.extend(overlays.cursors.into_iter().map(|peer| ShellBoardPresencePaint::Cursor { bounds, peer }));
            items.extend(overlays.marks.into_iter().map(|mark| {
                let width = board_presence_chip_width(theme, &mark.chip);
                let rect = Rect::new(bounds.x + bounds.w - 8.0 - width, bounds.y + 8.0 + mark.row as f32 * 18.0, width, theme.font_size_small + 4.0);
                ShellBoardPresencePaint::Mark { bounds, rect, mark }
            }));
        }
        items
    }

    /// 🛂️ Resolves this frame's administration sheet from the retained operation alone — packet
    /// W15e. `None` means no operation is live, which is exactly when React mounts nothing.
    ///
    /// ⚖️ Every row comes from [`shell_space_administration_controls`], the SAME builder the native
    /// shell already renders from and the one whose `enabled` flags are derived solely from the hub's
    /// own capability answer. The chrome adds no authority of its own.
    ///
    /// 🧾️ The status row is the sheet's own status LINE, not a roster row — React paints it as the
    /// `role="status"` paragraph above the rosters, so it is lifted out of the control list here.
    ///
    /// 📋️ The invite-link control is deliberately WITHHELD by this packet, on both targets.
    /// Acknowledging the one-shot capability ERASES it in the same turn, so a control that
    /// acknowledges without delivering destroys the invitation outright — and neither target
    /// can deliver it yet: the frame Worker owns no `navigator.clipboard` (`web_sys::window()`
    /// is `None` there, the defect the file and preference doors exist for), and the native
    /// clipboard is an `InteractiveJob` (`ui_host::ClipboardIoJob`) with no shell-side runner.
    /// Offering a control that silently loses an invitation is strictly worse than not
    /// offering it; the clipboard door is named as a hand-off in this packet's report.
    pub(crate) fn space_administration_plan(&self, prefs: &PrefsState) -> Option<crate::space_administration::SpaceAdministrationPlan> {
        use crate::space_administration as pane;
        let operation = self.space_administration.as_ref()?;
        let german = prefs.locale_id == "de";
        let locale = prefs.active_locale();
        let controls = shell_space_administration_controls(operation, german);
        let notice = match operation.page() {
            Some(DirectorySpaceAdministrationPageV1::Public { .. }) => Some(pane::space_administration_public_notice(locale).to_string()),
            Some(DirectorySpaceAdministrationPageV1::Member { .. }) => Some(pane::space_administration_spectator_notice(locale).to_string()),
            _ => None,
        };
        let mut rows = Vec::new();
        let mut status = shell_space_administration_status(operation.phase(), german).to_string();
        for control in controls {
            if control.control_id == "os.space-administration.status" {
                status = control.label.to_string();
                continue;
            }
            if control.control_id == "os.space-administration.invite.copy" {
                continue;
            }
            rows.push(pane::SpaceAdministrationRow { control_id: control.control_id, enabled: control.enabled, label: control.label.to_string(), value: control.value });
        }
        Some(pane::SpaceAdministrationPlan { notice, rows, space_id: operation.space_id().to_string(), status, title: shell_space_administration_label("title", german).to_string() })
    }
}

//#endregion ShellChrome

