//! 🗃️ OS command `os.open-artifact-with` — opens the "Open with…" chooser over every registered
//! viewer/editor `AppRef` for the artifact's dialect (owner surface first, then `AppRouter`'s
//! deterministic plugin-id/app-id order). See contract-freeze.md §3 of
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id.
pub const ID: &str = "os.open-artifact-with";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Open With…";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Öffnen mit…";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_artifact_with_id_and_labels_are_frozen() {
        assert_eq!(ID, "os.open-artifact-with");
        assert_eq!(LABEL_EN, "Open With…");
        assert_eq!(LABEL_DE, "Öffnen mit…");
    }
}
//#endregion 🧪️Tests
