//! 🌅️ One-time utility admission preserves addressed arms and explicit clears.

use super::UtilityRef;
use std::collections::HashMap;

/// 🧰️ The authored roster and current window authority used at first admission.
pub struct InitialWindowUtilityInput<'a> {
    pub window_id: &'a str,
    pub utility_ids: &'a [UtilityRef],
    pub initial_utility_id: Option<&'a UtilityRef>,
    pub active_utility_by_window_id: &'a HashMap<String, Option<String>>,
    pub active_tool_id: Option<&'a str>,
}

/// 🪛️ An authority write, or the already resolved value which must be preserved.
#[derive(Debug, PartialEq, Eq)]
pub struct InitialWindowUtilityResolution<'a> {
    pub write: bool,
    pub utility_id: Option<&'a str>,
}

/// 🧭️ Resolves the neutral contract without inferring an arm from roster order.
pub fn resolve_initial_window_utility(input: InitialWindowUtilityInput<'_>) -> Result<InitialWindowUtilityResolution<'_>, &'static str> {
    if input.initial_utility_id.is_some_and(|id| !input.utility_ids.contains(id)) {
        return Err("initial utility must be accepted by its window kind");
    }
    if let Some(current) = input.active_utility_by_window_id.get(input.window_id) {
        return Ok(InitialWindowUtilityResolution { write: false, utility_id: current.as_deref() });
    }
    Ok(InitialWindowUtilityResolution { write: true, utility_id: input.initial_utility_id.filter(|_| input.active_tool_id.is_none()).map(UtilityRef::as_str) })
}
