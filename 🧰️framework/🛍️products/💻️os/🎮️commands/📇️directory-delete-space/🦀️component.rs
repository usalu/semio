//! 📇️ OS command `os.directory.delete-space` — permanently deletes a hub space (owner or admin
//! only, contract §C2), dispatched through the shell's directory command funnel
//! (`HostEffect::ReplayShellCommand` → `DirectoryClient.command` → `POST /directory/commands`; the
//! resulting `DirectoryEvent`s arrive back over `/directory/ws`, never an optimistic local
//! mutation). See contract-freeze.md §C6 of
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id.
pub const ID: &str = "os.directory.delete-space";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Delete Space";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Space löschen";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_delete_space_id_and_labels_are_frozen() {
        assert_eq!(ID, "os.directory.delete-space");
        assert_eq!(LABEL_EN, "Delete Space");
        assert_eq!(LABEL_DE, "Space löschen");
    }
}
//#endregion 🧪️Tests
