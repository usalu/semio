//! 👁️ OS command `os.set-default-viewer` — pins the currently-open `AppRef` as the default
//! viewer for its `(artifact_kind, standard, subset)` coordinate, dispatched as
//! `AppCommand::SetDefaultApp` with `role = 0` (`AppRole::Viewer`). See contract-freeze.md §3 of
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id.
pub const ID: &str = "os.set-default-viewer";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Set Default Viewer";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Standard-Betrachter festlegen";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_default_viewer_id_and_labels_are_frozen() {
        assert_eq!(ID, "os.set-default-viewer");
        assert_eq!(LABEL_EN, "Set Default Viewer");
        assert_eq!(LABEL_DE, "Standard-Betrachter festlegen");
    }
}
//#endregion 🧪️Tests
