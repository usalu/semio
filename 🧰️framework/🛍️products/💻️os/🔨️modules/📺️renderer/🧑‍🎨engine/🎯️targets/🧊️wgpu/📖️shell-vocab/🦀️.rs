//! 📖️ Shell chrome strings, language axes, labels and refusal vocabulary.
//!
//! 🗣️ Holds the EN/DE chrome string table of the wgpu shell as a pure lookup (`shell_chrome_string`), carved out of the renderer monolith by RENDERER-WP4 as the first physical Shell crate: no state, no I/O, no first-party dependency.

#[path = "🏠️host-conflict-and-plugin-vocab/🦀️.rs"]
mod host_conflict_and_plugin;
use host_conflict_and_plugin::shell_chrome_host_conflict_and_plugin;

#[path = "📄️document-and-settings-vocab/🦀️.rs"]
mod document_and_settings;
use document_and_settings::shell_chrome_document_and_settings;

#[path = "🛍️marketplace-chat-and-panel-vocab/🦀️.rs"]
mod marketplace_chat_and_panel;
use marketplace_chat_and_panel::shell_chrome_marketplace_chat_and_panel;

#[path = "🔎️window-sync-and-search-vocab/🦀️.rs"]
mod window_sync_and_search;
use window_sync_and_search::shell_chrome_window_sync_and_search;

#[path = "✅️check-in-and-palette-vocab/🦀️.rs"]
mod check_in_and_palette;
use check_in_and_palette::shell_chrome_check_in_and_palette;

//#region 🗣️ChromeI18n
/// 🗣️ EN/DE lookup for a curated subset of previously-hardcoded chrome strings, byte-identical to
/// `ui/js/react/index.tsx`'s `uiChromeTranslationBundles.{en,de}.translation.ui.*` "normal" labels
/// (`:2898-3975`) at the dotted paths named below (e.g. `"display.tab.windows"` ==
/// `ui.display.tab.windows`). Unknown keys fall back to the key itself rather than inventing text.
///
/// 🖥️ React's `ui.display.*` block (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`), verbatim on both sides.
///
/// ⚔️ React's `ui.conflict.*` block.
///
/// 🔌️ React's `ui.plugins.status.*` / `ui.plugins.action.*` blocks.
///
/// 🎨️ wgpu-only additions (not verified against the external `elements/ui` i18next resource
/// bundle React's `shellLabel`/`uiI18n.t` ultimately reads — that package isn't vendored in this
/// repo tree, so these are reasonable EN/DE pairs in the same terse register as the rest of this
/// curated subset, not a byte-identical trace like the entries above copied from `index.tsx:2898-3975`).
///
/// 🎨️ The theme EDITOR's own vocabulary (packet W15d) — React reads these from
/// `ui.settings.theme.*`/`ui.mutation.policy.*` in the external i18next bundle this repo does
/// not vendor, so these are EN/DE pairs in the same terse register as the wgpu-only additions
/// above, not a byte-identical trace.
///
/// 💬️ The `os.agent.chat.*` bundle React registers in `🔗️AgentBridge/🟦️.tsx`, en first then de,
/// string-for-string — this renderer resolves chrome copy from this table instead of i18next.
///
/// 🚦️ `#s-sync-status`'s vocabulary, term for term with `syncPillText`'s own `FrozenLabel`s
/// (`🛠️ShellHelpers/🟦️.tsx`). `sync.recovering` has no React twin — see `ShellSyncPill`.
pub fn shell_chrome_string(key: &'static str, is_de: bool) -> &'static str {
    shell_chrome_host_conflict_and_plugin(key, is_de)
        .or_else(|| shell_chrome_document_and_settings(key, is_de))
        .or_else(|| shell_chrome_marketplace_chat_and_panel(key, is_de))
        .or_else(|| shell_chrome_window_sync_and_search(key, is_de))
        .or_else(|| shell_chrome_check_in_and_palette(key, is_de))
        .unwrap_or(key)
}
//#endregion 🗣️ChromeI18n

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
