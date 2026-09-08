//! 📇️ OS command `os.directory.create-space` — creates a new hub space, dispatched through the
//! shell's directory command funnel (`Effect::ReplayShellCommand` → `DirectoryClient.command`
//! → `POST /directory/commands` → the resulting `DirectoryEvent`s arrive back over `/directory/socket/v1`;
//! never an optimistic local mutation). See contract-freeze.md §C6 of
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/`.

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
