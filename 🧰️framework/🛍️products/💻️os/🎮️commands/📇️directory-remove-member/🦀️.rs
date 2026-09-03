//! 📇️ OS command `os.directory.remove-member` — removes a member from a hub space (the owner's own
//! membership can never be removed, contract §C1's decider laws), dispatched through the shell's
//! directory command funnel (`Effect::ReplayShellCommand` → `DirectoryClient.command` →
//! `POST /directory/commands`; the resulting `DirectoryEvent`s arrive back over `/directory/socket/v1`,
//! never an optimistic local mutation). See contract-freeze.md §C6 of
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id.
pub const ID: &str = "os.directory.remove-member";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Remove Member";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Mitglied entfernen";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_remove_member_id_and_labels_are_frozen() {
        assert_eq!(ID, "os.directory.remove-member");
        assert_eq!(LABEL_EN, "Remove Member");
        assert_eq!(LABEL_DE, "Mitglied entfernen");
    }
}
//#endregion 🧪️Tests
