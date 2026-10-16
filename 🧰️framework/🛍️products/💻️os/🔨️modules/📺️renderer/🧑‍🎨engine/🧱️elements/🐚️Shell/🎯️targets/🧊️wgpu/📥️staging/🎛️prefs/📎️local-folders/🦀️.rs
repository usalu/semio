//! 🎛️ `shell-prefs` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::local_folders::*;
use semio_framework_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};

//#region 🔖️Reconnect
/// 🏷️ The name a person knows a folder by: its last path segment — React's `localFolderNameV1`.
pub(crate) fn local_folder_name(path: &str) -> &str {
    path.split(['/', '\\']).filter(|segment| !segment.is_empty()).last().unwrap_or(path)
}

/// 📁️ A binding's folder path.
pub(crate) fn local_folder_path(binding: &LocalFolderBinding) -> &str {
    match &binding.folder {
        LocalFolderRef::Path { path } => path,
    }
}

pub(crate) const FOLDER_RECONNECT_CONTROL_ID: &str = "s-folder-reconnect";

pub(crate) const FOLDER_FORGET_CONTROL_ID: &str = "s-folder-forget";

/// 📐️ The band's widest extent on one row; wider, it wraps compact.
const FOLDER_BAND_MAX_WIDTH: f32 = 720.0;

/// 📐️ One laid-out reconnect band: the strip, the message's lines and the two buttons.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FolderReconnectBandPlan {
    pub band: Rect,
    pub lines: Vec<time_travel::ChromeBandLine>,
    pub buttons: [(&'static str, &'static str, String, Rect); 2],
}

/// 📐️ Lays the band out bottom-centre through the time-travel band's [`time_travel::chrome_band_layout`], stacked above
/// the time-travel band when one shows (React stacks the folder offer first), else as the subfooter's last row under the footer: one row with
/// the buttons right-aligned after the message, or on a phone-width viewport the message wrapped at its words with the
/// buttons below it.
pub(crate) fn folder_reconnect_band_plan(message: String, reconnect: String, forget: String, below: Option<Rect>, width: f32, height: f32, theme: &Theme) -> FolderReconnectBandPlan {
    let floor = below.map_or(height, |rect| rect.y);
    let layout = time_travel::chrome_band_layout(&message, " ", &[reconnect.as_str(), forget.as_str()], floor, width, FOLDER_BAND_MAX_WIDTH, theme);
    let (reconnect_rect, forget_rect) = (layout.buttons[0], layout.buttons[1]);
    FolderReconnectBandPlan { band: layout.band, lines: layout.lines, buttons: [(FOLDER_RECONNECT_CONTROL_ID, FOLDER_RECONNECT_ACTION, reconnect, reconnect_rect), (FOLDER_FORGET_CONTROL_ID, FOLDER_FORGET_ACTION, forget, forget_rect)] }
}

//#endregion 🔖️Reconnect
impl PrefsState {
    /// 📐️ This frame's band, localized.
    pub(crate) fn folder_reconnect_band_plan_for(&self, offer: &LocalFolderBinding, theme: &Theme, chrome: &ChromeState, history: &HistoryState) -> FolderReconnectBandPlan {
        let locale = self.active_locale();
        let below = self.history_reprojection_band_plan_for(theme, chrome, history).map(|(_, plan)| plan.band).or_else(|| history.history_time_travel.as_ref().map(|status| self.time_travel_band_plan_for(status, theme, chrome).band));
        let message = local_folder_text(LocalFolderText::Message, local_folder_name(local_folder_path(offer)), locale);
        folder_reconnect_band_plan(message, local_folder_text(LocalFolderText::Attach, "", locale), local_folder_text(LocalFolderText::Forget, "", locale), below, chrome.screen_w, chrome.screen_h, theme)
    }
}

