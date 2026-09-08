//! ✏️ OS command `os.set-default-editor` — pins the currently-open `AppRef` as the default
//! editor for its `(artifact_kind, standard, subset)` coordinate, dispatched as
//! `AppCommand::SetDefaultApp` with `role = 1` (`AppRole::Editor`). See contract-freeze.md §3 of
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id.
pub const ID: &str = "os.set-default-editor";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Set Default Editor";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Standard-Editor festlegen";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
