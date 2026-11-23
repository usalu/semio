//! 🎛️ `shell-prefs` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use semio_renderer_wgpu_shell_history::LocalFileRow;
use semio_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};

//#region 🔖️Reconnect
/// 🏷️ The name a person knows a folder by: its last path segment — React's `localFolderNameV1`.
pub fn local_folder_name(path: &str) -> &str {
    path.split(['/', '\\']).filter(|segment| !segment.is_empty()).last().unwrap_or(path)
}

/// 📁️ A binding's folder path.
pub fn local_folder_path(binding: &LocalFolderBinding) -> &str {
    match &binding.folder {
        LocalFolderRef::Path { path } => path,
    }
}

pub const FOLDER_RECONNECT_CONTROL_ID: &str = "s-folder-reconnect";

pub const FOLDER_FORGET_CONTROL_ID: &str = "s-folder-forget";

/// 📐️ The band's widest extent on one row; wider, it wraps compact.
fn folder_band_max_width(width: f32) -> f32 {
    width * semio_renderer_wgpu_shell_chrome_state::chrome_parity_constants().sizes.folder_band_max_width_fraction
}

/// 📐️ One laid-out reconnect band: the strip, the message's lines and the two buttons.
#[derive(Clone, Debug, PartialEq)]
pub struct FolderReconnectBandPlan {
    pub band: Rect,
    pub lines: Vec<ChromeBandLine>,
    pub buttons: [(&'static str, &'static str, String, Rect); 2],
}

/// 📐️ Lays the band out bottom-centre through the time-travel band's [`chrome_band_layout`], stacked above
/// the time-travel band when one shows (React stacks the folder offer first), else as the subfooter's last row under the footer: one row with
/// the buttons right-aligned after the message, or on a phone-width viewport the message wrapped at its words with the
/// buttons below it.
pub fn folder_reconnect_band_plan(message: String, reconnect: String, forget: String, below: Option<Rect>, width: f32, height: f32, theme: &Theme) -> FolderReconnectBandPlan {
    let floor = below.map_or(height, |rect| rect.y);
    let layout = chrome_band_layout(&message, " ", &[reconnect.as_str(), forget.as_str()], floor, width, folder_band_max_width(width), theme);
    let (reconnect_rect, forget_rect) = (layout.buttons[0], layout.buttons[1]);
    FolderReconnectBandPlan { band: layout.band, lines: layout.lines, buttons: [(FOLDER_RECONNECT_CONTROL_ID, FOLDER_RECONNECT_ACTION, reconnect, reconnect_rect), (FOLDER_FORGET_CONTROL_ID, FOLDER_FORGET_ACTION, forget, forget_rect)] }
}

//#endregion 🔖️Reconnect
impl PrefsState {
    /// 📐️ This frame's band, localized.
    pub fn folder_reconnect_band_plan_for(&self, offer: &LocalFolderBinding, theme: &Theme, chrome: &ChromeState, history: &HistoryState) -> FolderReconnectBandPlan {
        let locale = self.active_locale();
        let below = self.history_reprojection_band_plan_for(theme, chrome, history).map(|(_, plan)| plan.band).or_else(|| history.history_time_travel.as_ref().map(|status| self.time_travel_band_plan_for(status, theme, chrome).band));
        let message = local_folder_text(LocalFolderText::Message, local_folder_name(local_folder_path(offer)), locale);
        folder_reconnect_band_plan(message, local_folder_text(LocalFolderText::Attach, "", locale), local_folder_text(LocalFolderText::Forget, "", locale), below, chrome.screen_w, chrome.screen_h, theme)
    }
}

//#region 🔖️Reconnect
pub const FOLDER_FORGET_ACTION: &str = "forgetFolder";

/// 🎬️ The band's two verbs, dispatched on the shell's own sync controller.
pub const FOLDER_RECONNECT_ACTION: &str = "reconnectFolder";

/// 🗣️ One line of the band (or of a reconnect refusal) in `locale`; `folder` fills the message.
pub fn local_folder_text(text: LocalFolderText, folder: &str, locale: Locale) -> String {
    match (text, locale) {
        (LocalFolderText::Label, Locale::En) => "Folder of this document".into(),
        (LocalFolderText::Label, Locale::De) => "Ordner dieses Dokuments".into(),
        (LocalFolderText::Message, Locale::En) => format!("This document was attached to the folder “{folder}”."),
        (LocalFolderText::Message, Locale::De) => format!("Dieses Dokument war mit dem Ordner „{folder}“ verbunden."),
        (LocalFolderText::Attach, Locale::En) => "Reconnect folder".into(),
        (LocalFolderText::Attach, Locale::De) => "Ordner wieder verbinden".into(),
        (LocalFolderText::Forget, Locale::En) => "Forget folder".into(),
        (LocalFolderText::Forget, Locale::De) => "Ordner vergessen".into(),
        (LocalFolderText::Unidentified, Locale::En) => "This program has no document to attach".into(),
        (LocalFolderText::Unidentified, Locale::De) => "Dieses Programm hat kein Dokument zum Verbinden".into(),
    }
}

/// 🗣️ The band's copy — React's `ui.sync.reconnect.{label,message,attach,forget}` at the normal terminology.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalFolderText {
    Label,
    Message,
    Attach,
    Forget,
    Unidentified,
}
//#endregion 🔖️Reconnect

//#region 🔖️LocalFiles
/// 🆔️ The listing's own nodes: the band's status id and the prefix of each document button's control id (`<prefix><documentId>`).
pub const LOCAL_FILES_STATUS_ID: &str = "s-local-files";

pub const LOCAL_FILE_CONTROL_PREFIX: &str = "s-local-file.";

/// 🎬️ The verb a document button dispatches on the shell's sync controller: reopen that kept document (React's `openKeptFile`).
pub const LOCAL_FILE_OPEN_ACTION: &str = "openKeptFile";

/// 📜️ The listing's scroll region: when the documents do not fit above the footer, the wheel moves the first listed row, like React's section growing past the viewport.
pub const LOCAL_FILES_SCROLL_ID: &str = "s-local-files.scroll";

/// 🗣️ The listing's title (React's `LocalFilesV1` `title` label).
pub fn local_files_title(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Files on this device",
        Locale::De => "Dateien auf diesem Gerät",
    }
}

/// 📐️ One laid-out listing band: the strip, its title line, the visible window of the list (one button per kept document: control id, document id, name, box), the pixel pitch of one row, the largest first row and the scroll thumb when rows are hidden.
#[derive(Clone, Debug, PartialEq)]
pub struct LocalFilesBandPlan {
    pub band: Rect,
    pub lines: Vec<ChromeBandLine>,
    pub buttons: Vec<(String, String, String, Rect)>,
    pub row_pitch: f32,
    pub max_first_row: usize,
    pub thumb: Option<Rect>,
}

impl PrefsState {
    /// 📐️ This frame's listing band, stacked bottom-centre above `below` (the folder offer, else the history bands): React's `<ul>`, one button per kept document, all of them. Rows the space above `below` cannot hold scroll by `scroll` pixels (whole rows); `None` while the device keeps no document with a surface.
    pub fn local_files_band_plan_for(&self, rows: &[LocalFileRow], below: Option<Rect>, scroll: f32, theme: &Theme, chrome: &ChromeState) -> Option<LocalFilesBandPlan> {
        if rows.is_empty() {
            return None;
        }
        let (pad, gap, small) = (theme.padding_standard, theme.gap_standard, theme.font_size_small);
        let glyph_w = |text: &str| text.chars().count() as f32 * small * 0.6;
        let title = local_files_title(self.active_locale());
        let row_h = theme.size_large();
        let pitch = row_h + gap * 0.5;
        let title_h = small * 1.6;
        let head_h = pad + title_h + gap * 0.5;
        let floor = below.map_or(chrome.screen_h, |rect| rect.y);
        let widest = rows.iter().map(|row| glyph_w(row.name.as_str()) + pad * 2.0).fold(glyph_w(title), f32::max);
        let width = (widest + pad * 2.0).min((chrome.screen_w * 0.9).min(folder_band_max_width(chrome.screen_w))).max(1.0);
        let fit = (((floor - theme.navbar_height - pad - head_h) / pitch).floor().max(1.0)) as usize;
        let visible = fit.min(rows.len());
        let max_first_row = rows.len() - visible;
        let first = ((scroll.max(0.0) / pitch).floor() as usize).min(max_first_row);
        let band_h = head_h + visible as f32 * pitch;
        let band = Rect::new(((chrome.screen_w - width) * 0.5).max(0.0), (floor - band_h).max(0.0), width, band_h);
        let inner = (width - pad * 2.0).max(1.0);
        let top = band.y + pad * 0.5 + title_h + gap * 0.5;
        let lines = vec![ChromeBandLine { text: title.to_string(), rect: Rect::new(band.x + pad, band.y + pad * 0.5, inner, title_h) }];
        let buttons = rows.iter().skip(first).take(visible).enumerate().map(|(index, row)| (format!("{LOCAL_FILE_CONTROL_PREFIX}{}", row.document_id), row.document_id.clone(), row.name.clone(), Rect::new(band.x + pad, top + index as f32 * pitch, inner, row_h))).collect();
        let thumb = (visible < rows.len()).then(|| {
            let track = visible as f32 * pitch;
            let share = visible as f32 / rows.len() as f32;
            Rect::new(band.x + band.w - pad * 0.5 - theme.stroke_hairline * 2.0, top + track * first as f32 / rows.len() as f32, theme.stroke_hairline * 2.0, track * share)
        });
        Some(LocalFilesBandPlan { band, lines, buttons, row_pitch: pitch, max_first_row, thumb })
    }
}
//#endregion 🔖️LocalFiles
