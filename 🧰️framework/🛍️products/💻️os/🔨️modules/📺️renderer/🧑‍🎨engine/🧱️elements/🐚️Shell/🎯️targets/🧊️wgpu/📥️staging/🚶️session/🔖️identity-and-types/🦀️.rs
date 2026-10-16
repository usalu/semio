//! 🚶️ `shell-session` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

/// 🌳️ Byte-identical to React's `FRAMEWORK_CATEGORY_DISPLAY_ID` (`🛠️ShellHelpers/🟦️.tsx:224`) — the
/// bottom-left anchor's Display branch, the one anchor whose app tabs nest under a category root.
pub(crate) const FRAMEWORK_CATEGORY_DISPLAY_ID: &str = "framework.category.display";

/// 🎛️ Byte-identical to React's `FRAMEWORK_CATEGORY_COMMAND_ID` — the bottom-middle Command branch.
pub(crate) const FRAMEWORK_CATEGORY_COMMAND_ID: &str = "framework.category.command";

/// 🛠️ Byte-identical to React's `FRAMEWORK_CATEGORY_TOOL_ID` — the bottom-middle Tool branch, ordered
/// left of Command. wgpu has no mode-level tool leaves to nest under it yet (`📓️w1h-…` remaining gaps).
pub(crate) const FRAMEWORK_CATEGORY_TOOL_ID: &str = "framework.category.tool";

/// 🎛️ One command category's leaf id under the Command branch — React's `command.category.<id>`.
pub(crate) const FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX: &str = "command.category.";

/// ⚙️ Byte-identical to React's `FRAMEWORK_SETTINGS_PANEL_ID`/`FRAMEWORK_SETTINGS_BRANCH_ID`
/// (`📌️ChromePanels/🟦️.tsx`, `🛠️ShellHelpers/🟦️.tsx`) — the bottom-right Settings BRANCH whose children
/// are the five framework settings leaves, with the app's own Settings-group tabs nested ahead of them.
pub(crate) const FRAMEWORK_SETTINGS_PANEL_ID: &str = "framework.settings";

/// 🛍️ The Marketplace's windowed roster section — React's `framework.marketplace.source.<sourceId>` for this renderer's one source.
pub(crate) const MARKETPLACE_LOCAL_SECTION_ID: &str = "framework.marketplace.source.local";

/// 🛍️ The node ledger one Marketplace window may materialise — the body-wide tree window budget less the windowed section itself.
pub(crate) const MARKETPLACE_WINDOW_NODE_BUDGET: usize = ui_contract::TREE_WINDOW_BODY_NODE_BUDGET - 1;

/// 🛠️ One mode tool's leaf id under the Tool branch — React's `tool.<id>` (`🛠️ShellHelpers/🟦️.tsx`'s
/// `buildToolTabs`/`toolIdFromPanelTabId`). The leaf tab IS the activation control.
pub(crate) const FRAMEWORK_TOOL_PANEL_TAB_PREFIX: &str = "tool.";

/// 📱️ Byte-identical to React's synthetic mobile "App" tab (`🏛️ShellHost/🟦️.tsx`'s `mobilePanelTabs`).
pub(crate) const FRAMEWORK_MOBILE_APP_TAB_ID: &str = "framework.mobile.app";

//#region 🔖️IdentityPure
/// 📇️ The space index a program names through its `os.space-index` topic contribution.
pub(crate) fn program_space_index(program: &ProgramBridgeEntry) -> Option<semio_framework::manifest::SpaceIndexContribution> {
    semio_framework::manifest::SpaceIndexContribution::from_topic_contributions(&program.manifest.topic_contributions)
}

/// 🪐️ Whether `session` is the app that serves the space index its own program names.
pub(crate) fn session_is_space_index(session: &ActiveSession, plugins: &[ProgramBridgeEntry]) -> bool {
    plugins.iter().find(|program| program.plugin_id == session.plugin_id).and_then(program_space_index).is_some_and(|index| index.dialect == session.app.dialect)
}

/// 🛟️ How long a rebootstrapping document waits before asking the hub for its pair again after a transient refusal.
pub(crate) const SYNC_RESEED_RETRY_MS: f64 = 1_000.0;

//#endregion 🔖️IdentityPure
//#region 🔖️CheckInPure
/// 🧾️ ticket §C5 — folds one `HistoryPatch` into the running per-session entry map, the wgpu twin of
/// the React shell's `applyHistoryPatch` reducer (`ShellHost/🟦️.tsx`): `replace=true` (a
/// fresh `ReadHistory` snapshot on session/document mount) clears the map first; otherwise a patch no
/// newer than the tracked cursor is a stale/duplicate reply and is ignored outright. Returns whether
/// the fold actually changed anything, so callers only re-derive the uncommitted count when it could
/// have moved.
///
/// 🔑️ Rows fold under [`semio_framework::kernel::HistoryEntry::key`] — `edit:<editId>`, else `seq:<seq>` — the
/// identity the kernel wire names (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), so a row re-emitted for the
/// same edit replaces its predecessor instead of doubling it.
pub(crate) fn fold_history_patch(entries: &mut BTreeMap<String, semio_framework::kernel::HistoryEntry>, cursor: &mut u64, patch: &semio_framework::kernel::HistoryPatch, replace: bool) -> bool {
    if !replace && patch.cursor <= *cursor {
        return false;
    }
    if replace {
        entries.clear();
    }
    for entry in &patch.upserts {
        entries.insert(entry.key(), entry.clone());
    }
    *cursor = patch.cursor;
    true
}

//#endregion 🔖️CheckInPure
//#region ShellTypes
#[derive(Clone, Debug)]
pub(crate) enum ShellPalettePlanEntry {
    Input { rect: Rect, query: String, placeholder: String },
    Group { rect: Rect, label: String },
    Row { rect: Rect, index: usize, item_id: String, label: String, description: Option<String>, selected: bool },
    Empty { rect: Rect, label: String },
    Close { rect: Rect, label: String },
}

#[derive(Clone, Debug)]
pub(crate) struct ShellPalettePlan {
    pub(crate) kind: ShellPaletteKind,
    pub(crate) dialog: Rect,
    pub(crate) list: Rect,
    pub(crate) entries: Vec<ShellPalettePlanEntry>,
}

const SHELL_PALETTE_RESULT_LIMIT: usize = 20;

pub(crate) const SHELL_PALETTE_PLAN_ENTRY_CAPACITY: usize = 2 + SHELL_PALETTE_RESULT_LIMIT * 2;

pub(crate) const SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX: f32 = 512.0;

pub(crate) const SHELL_PALETTE_LIST_MAX_UI_SPACING: f32 = 93.75;

pub(crate) const SHELL_PALETTE_TINY_UI_SPACING: f32 = 3.0;

pub(crate) const SHELL_PALETTE_MEDIUM_UI_SPACING: f32 = 7.0;

//#endregion ShellTypes

