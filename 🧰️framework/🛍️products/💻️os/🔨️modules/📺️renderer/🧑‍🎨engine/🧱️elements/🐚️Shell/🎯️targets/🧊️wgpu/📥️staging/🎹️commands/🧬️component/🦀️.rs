//! 🎹️ `shell-commands` component structs: `CommandState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 🎹️ `CommandState`: search and find palette state and staged command arguments.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `commands`.
pub struct CommandState {
    pub search_open: bool,
    pub find_open: bool,
    pub search_query: String,
    pub search_selected: usize,
    pub find_query: String,
    pub find_items: ShellFindItems,
    pub find_selected: usize,
    /// 📝️ Staged action argument values keyed `"{window_id}:{action_id}"` — edits buffer here
    /// and never dispatch until Execute (Architecture Decision 8, P2).
    pub staged_action_args: HashMap<String, serde_json::Map<String, Value>>,
    /// 🎛️ The command whose staged form the Commands panel should show expanded — the twin of
    /// React's `SET_COMMAND_EXPANDED`, set when an arg-carrying command is picked from the palette
    /// (`command-form:` in `activate_search_item`). Keyed by `command_address_stable_key`.
    pub expanded_command_id: Option<String>,
    /// 📝️ Staged command argument values keyed by `command_address_stable_key` — the twin of
    /// React's `STAGE_COMMAND_ARG`/`RESET_COMMAND_ARGS` buffer, which Execute reads and nothing else
    /// dispatches from.
    pub staged_command_args: HashMap<String, serde_json::Map<String, Value>>,
    /// 🔎️ Per-window possibles-chevron state — React's `Search` `possiblesExpanded`
    /// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10771`), so absent = collapsed.
    pub search_possibles_open: HashMap<String, bool>,
}
