//! 📂️ OS command `os.open-artifact` — opens an artifact in its resolved viewer/editor surface
//! (empty `plugin_id`/`app_id` on the wire `AppCommand::OpenArtifact` means "ask the
//! `OpeningResolver`"). See contract-freeze.md §3 of
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id, dispatched as `AppCommand::OpenArtifact` over the app channel.
pub const ID: &str = "os.open-artifact";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Open Artifact";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Artefakt öffnen";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
