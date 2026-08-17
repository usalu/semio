//! 📇️ OS command `os.directory.set-visibility` — flips a hub space between `private`/`public`,
//! dispatched through the shell's directory command funnel (`Effect::ReplayShellCommand` →
//! `DirectoryClient.command` → `POST /directory/commands`; the resulting `DirectoryEvent`s arrive
//! back over `/directory/ws`, never an optimistic local mutation). See contract-freeze.md §C6 of
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id.
pub const ID: &str = "os.directory.set-visibility";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Set Visibility";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Sichtbarkeit festlegen";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_set_visibility_id_and_labels_are_frozen() {
        assert_eq!(ID, "os.directory.set-visibility");
        assert_eq!(LABEL_EN, "Set Visibility");
        assert_eq!(LABEL_DE, "Sichtbarkeit festlegen");
    }
}
//#endregion 🧪️Tests
