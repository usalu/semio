//! 📇️ OS command `os.directory.share-link` — mints a redeemable invite link for a hub space.
//! Client-side sugar: the directory schema (contract §C1) has no `"share-link"` command kind, so the
//! shell's directory funnel maps this id onto `DirectoryCommand::CreateInvite { spaceId, role,
//! ttlSecs }` before it reaches `DirectoryClient.command` → `POST /directory/commands`; the
//! resulting `DirectoryEvent`s (and the invite token itself, riding the command result) arrive back
//! over `/directory/socket/v1`, never an optimistic local mutation. See contract-freeze.md §C6 of
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS/`.

//#region 🔖️Command
/// 🪪️ Canonical OS command id.
pub const ID: &str = "os.directory.share-link";

/// 🗣️ English label — declared first, no default language.
pub const LABEL_EN: &str = "Share Link";
/// 🗣️ German label.
pub const LABEL_DE: &str = "Link teilen";
//#endregion 🔖️Command

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
