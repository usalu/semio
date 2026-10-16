//! 🎛️ `shell-prefs` component structs: `PrefsState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 🎛️ `PrefsState`: appearance, locale, terminology, driver and theme drafts, merge policy and open conflicts.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `prefs`.
pub struct PrefsState {
    pub appearance_id: String,
    pub locale_id: String,
    pub terminology_id: String,
    pub driver_id: String,
    /// 🚗️ Session-local unsaved driver document. Its seven axes immediately replace the live
    /// resolved chrome while the owned preference record remains untouched until Save.
    pub driver_draft: Option<OsUiDriver>,
    /// 🚗️ React's `driverSaveLabel`, cleared after a successful save.
    pub driver_save_label: String,
    /// 🎨️ The unsaved theme document this session is editing — React's `uiThemeDraft`
    /// (`SET_UI_THEME_DRAFT`). `Some` is exactly React's `themeDirty`.
    pub theme_draft: Option<ThemeDocument>,
    /// 🎨️ The theme `Save` box — React's `themeSaveLabel`, session-local exactly as its
    /// `useState` is.
    pub theme_save_label: String,
    /// ⚖️ This session's merge policy — React's `shellState.merge.mergePolicy`, the value the
    /// `framework.settings.mergePolicy` selector writes. `"LaissezFaire" | "Normal" | "Vigilant"`,
    /// React's own option values.
    pub merge_policy: String,
    /// 📤️ Whether a theme IMPORT picker is parked for the gesture lane — a picker is gated on
    /// user activation, so it rides the same narrow door `pending_file_opens` does.
    pub pending_theme_import: bool,
    /// ⚔️ This session's open first-class conflicts — React's `selectOpenConflicts(shellState)`
    /// over `🐚️Shell`'s `merge.conflicts`. Seeded once per session from
    /// `ProgramBridgeEntry::read_conflicts` and replaced by every `resolve_conflict` reply, which is
    /// React's own `AppCommand::ReadConflicts` seed plus reply-carried roster.
    pub open_conflicts: Vec<ShellConflictRow>,
    /// ⚔️ The conflict row whose diff preview is expanded — React's `selectedConflictId`.
    pub selected_conflict_id: Option<String>,
    /// ⚔️ Whether this session already asked the guest for its conflict roster, so the seed
    /// costs one exchange per session rather than one per frame.
    pub conflicts_seeded: bool,
}
