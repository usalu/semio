//! 🧹 OS command `os.clear-default-app` — clears a previously pinned viewer or editor default for
//! one `(artifact_kind, standard, subset, role)` coordinate, dispatched as
//! `AppCommand::ClearDefaultApp`; the `OpeningResolver` then falls back to the owner surface, then
//! the first `AppRouter` entry. See contract-freeze.md §3 of
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id.
pub const ID: &str = "os.clear-default-app";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Clear Default App";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Standard-App zurücksetzen";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
