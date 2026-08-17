//! 📇️ OS command `os.directory.create-space` — creates a new hub space, dispatched through the
//! shell's directory command funnel (`HostEffect::ReplayShellCommand` → `DirectoryClient.command`
//! → `POST /directory/commands` → the resulting `DirectoryEvent`s arrive back over `/directory/ws`;
//! never an optimistic local mutation). See contract-freeze.md §C6 of
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id.
pub const ID: &str = "os.directory.create-space";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Create Space";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Space erstellen";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_create_space_id_and_labels_are_frozen() {
        assert_eq!(ID, "os.directory.create-space");
        assert_eq!(LABEL_EN, "Create Space");
        assert_eq!(LABEL_DE, "Space erstellen");
    }
}
//#endregion 🧪️Tests
