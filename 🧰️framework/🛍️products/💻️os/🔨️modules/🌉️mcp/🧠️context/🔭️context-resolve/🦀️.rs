//! 🔭️ Context resolve of the shell-channel crate: The live-shell artifact channel and context resolution.

use crate::shell_channel::ChannelKind;
use semio_framework_os_mcp_catalog::catalog::Catalog;
use semio_framework_os_mcp_foundation::reports::ContextSummary;

//#region 🔖️TokenBudget

pub const DEFAULT_MAX_TOKENS: u32 = 4096;

pub const HARD_MAX_TOKENS: u32 = 32768;

/// 🔢️ `ceil(bytes/4)` — the crate-wide token estimate every resource projection budgets against.
pub fn estimate_tokens(bytes: usize) -> u32 {
    ((bytes + 3) / 4) as u32
}

/// ✂️ Result of `truncate_to_budget` — `omitted` carries a JSON pointer per dropped element so a
/// caller can request the rest via a follow-up, narrower read.
#[derive(Clone, Debug, PartialEq)]
pub struct Truncated {
    pub value: serde_json::Value,
    pub omitted: Vec<String>,
    pub token_estimate: u32,
}

/// ✂️ Breadth-first truncation over the top-level `"entries"` array (the shape every projection in
/// this module produces) — drops elements from the END until the remaining JSON fits `max_tokens`
/// (clamped to `[1, HARD_MAX_TOKENS]`), recording each dropped index as `/entries/<index>`. A value
/// with no `"entries"` array, or one that already fits, passes through with `omitted: []`.
pub fn truncate_to_budget(value: serde_json::Value, max_tokens: u32) -> Truncated {
    let max_tokens = max_tokens.clamp(1, HARD_MAX_TOKENS);
    let byte_budget = max_tokens as usize * 4;
    let full_bytes = serde_json::to_vec(&value).unwrap_or_default().len();
    if full_bytes <= byte_budget {
        return Truncated { value: value.clone(), omitted: Vec::new(), token_estimate: estimate_tokens(full_bytes) };
    }
    let mut working = value;
    let mut omitted = Vec::new();
    let mut empty = working.clone();
    if let Some(entries) = empty.get_mut("entries").and_then(serde_json::Value::as_array_mut) {
        entries.clear();
    }
    let empty_bytes = serde_json::to_vec(&empty).unwrap_or_default().len();
    if let Some(object) = working.as_object_mut() {
        if let Some(serde_json::Value::Array(entries)) = object.get_mut("entries") {
            let original_len = entries.len();
            let mut current_bytes = empty_bytes;
            let mut retained = 0;
            for entry in entries.iter() {
                let entry_bytes = serde_json::to_vec(entry).unwrap_or_default().len();
                let separator_bytes = usize::from(retained > 0);
                if current_bytes.saturating_add(separator_bytes).saturating_add(entry_bytes) > byte_budget {
                    break;
                }
                current_bytes += separator_bytes + entry_bytes;
                retained += 1;
            }
            entries.truncate(retained);
            omitted.extend((retained..original_len).map(|index| format!("/entries/{index}")));
        }
    }
    let final_bytes = serde_json::to_vec(&working).unwrap_or_default().len();
    Truncated { value: working, omitted, token_estimate: estimate_tokens(final_bytes) }
}

//#endregion 🔖️TokenBudget

//#region 🔖️ContextResolve

/// 🪪️ Mints a fresh, opaque session id — blake3-mixed `(principal, wall-clock ms, call counter)`, the
/// same "no new dependency, `blake3` already in-tree" precedent `🎫️handles/🦀️.rs`'s own
/// `mint_id` set (`📓️terra-P1b-report.md` §2.2). A real `AgentSession` (handle-table-backed,
/// refreshed on every resolve) is P6/P7 work — this is `context.resolve`'s minimal, real-today
/// contract: a stable-looking id, not a persisted session record.
pub fn mint_session_id(principal: &str, counter: u64) -> String {
    let now_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis()).unwrap_or(0);
    format!("sess_{}", framework_hash::hash_bytes(format!("{principal}:{now_ms}:{counter}").as_bytes()))
}

/// 🪪️ `context.resolve` — the token-cheap summary every session opens with. `channel` is stamped
/// [`crate::shell_channel::ChannelKind::Headless`] here and re-stamped by
/// `registry_override_context_resolve` on a gateway that actually holds a
/// [`crate::shell_channel::SessionChannelBinding`]: this function has no bridge to ask, and a
/// summary must never claim a shell route it cannot prove.
pub fn resolve_context(catalog: &Catalog, session_id: String, principal: &str, scopes: Vec<String>, active_artifact_id: Option<String>, locale: &str) -> ContextSummary {
    ContextSummary {
        session_id,
        principal: principal.to_string(),
        scopes,
        active_artifact_id,
        catalog_hash: catalog.hash.clone(),
        locale: locale.to_string(),
        channel: ChannelKind::Headless.label().to_string(),
    }
}

//#endregion 🔖️ContextResolve
