//! ⌨️ The WAI-ARIA tree keyboard and selection rule — the Rust twin of `🌳️Tree/⌨️roving/🟦️.ts` (`treeNavigateV1`,
//! `applyRovingTabindexV1`) and of `🌳️Tree/🎚️selection/🟦️.ts` (`getTreeNextSelectionState`). One rule, two hosts: both read the
//! neutral vectors `🧪️tests/🧫️fixtures/⌨️roving/🔣️.json`. Pure: no layout, no arena, no DOM — a host lists its visible rows, this
//! answers which intents a key or a click means, and a host performs them.
//! @see 🎫️ 26/09/17 WGPU-RENDERER-REACT-PARITY · 📓️opus55-fix-tree.md

/// 🌳️ One visible tree row as the keyboard pattern sees it: a stable id, its one-based level, its fold state (`None` for a leaf) and the text typeahead matches.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNavRow {
    pub id: String,
    pub level: usize,
    pub expanded: Option<bool>,
    pub label: String,
}

/// ⌨️ The modifier keys held during a key or a click.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TreeNavModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub alt: bool,
}

impl TreeNavModifiers {
    fn additive(self) -> bool {
        self.ctrl || self.meta
    }

    fn none(self) -> bool {
        !(self.shift || self.ctrl || self.meta || self.alt)
    }
}

/// 🔀️ How a pick combines with the selection — React's `interactionMergeFromModifiers`: Shift is a range, Ctrl/Cmd inverts the row, a bare pick replaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeMerge {
    Replace,
    Range,
    Invertive,
}

impl TreeMerge {
    /// 🔀️ The merge a pointer press with `modifiers` means (Shift wins over Ctrl/Cmd).
    pub fn from_modifiers(modifiers: TreeNavModifiers) -> Self {
        if modifiers.shift {
            Self::Range
        } else if modifiers.additive() {
            Self::Invertive
        } else {
            Self::Replace
        }
    }
}

/// 🎯️ What a key or a click asks the host to do, in order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeIntent {
    /// 🔦️ Move focus (and the roving tab stop) to the row.
    Focus(usize),
    /// 🖱️ The row's click: folds or unfolds an expandable row, then selects or activates it with `merge` — what `element.click()` does in React.
    Click { index: usize, merge: TreeMerge },
    /// ☑️ Select the row with `merge` without folding it.
    Pick { index: usize, merge: TreeMerge },
    /// 🖱️🖱️ The row's double-click activation.
    DoubleClick(usize),
}

/// ⌨️ What `key` does at the row `current` in the WAI-ARIA tree pattern, or `None` when the tree leaves the key alone.
///
/// Up, Down, Home and End move focus; Right unfolds a collapsed row or enters an expanded one; Left folds an expanded row or climbs
/// to its parent; Enter and Space click the row. With `multiple`, Shift with a move key extends the range from the anchor, Shift+Space
/// selects the range to the row and Ctrl/Cmd+Space toggles the row in the selection.
pub fn tree_key(key: &str, modifiers: TreeNavModifiers, rows: &[TreeNavRow], current: usize, multiple: bool) -> Option<Vec<TreeIntent>> {
    if rows.is_empty() || current >= rows.len() || modifiers.alt {
        return None;
    }
    let row = &rows[current];
    let last = rows.len() - 1;
    let moved = match key {
        "ArrowDown" => (current < last).then_some(current + 1),
        "ArrowUp" => current.checked_sub(1),
        "Home" => Some(0),
        "End" => Some(last),
        _ => None,
    };
    if let Some(index) = moved {
        let extends = multiple && modifiers.shift && !modifiers.additive();
        return Some(if extends { vec![TreeIntent::Focus(index), TreeIntent::Pick { index, merge: TreeMerge::Range }] } else { vec![TreeIntent::Focus(index)] });
    }
    if matches!(key, "ArrowDown" | "ArrowUp") {
        return None;
    }
    match key {
        " " | "Space" if multiple && modifiers.additive() && !modifiers.shift => Some(vec![TreeIntent::Pick { index: current, merge: TreeMerge::Invertive }]),
        " " | "Space" if multiple && modifiers.shift && !modifiers.additive() => Some(vec![TreeIntent::Pick { index: current, merge: TreeMerge::Range }]),
        " " | "Space" | "Enter" | "NumpadEnter" if modifiers.none() => Some(vec![TreeIntent::Click { index: current, merge: TreeMerge::Replace }]),
        "ArrowRight" if modifiers.none() => match row.expanded {
            Some(false) => Some(vec![TreeIntent::Click { index: current, merge: TreeMerge::Replace }]),
            Some(true) if rows.get(current + 1).is_some_and(|next| next.level > row.level) => Some(vec![TreeIntent::Focus(current + 1)]),
            _ => None,
        },
        "ArrowLeft" if modifiers.none() => {
            if row.expanded == Some(true) {
                return Some(vec![TreeIntent::Click { index: current, merge: TreeMerge::Replace }]);
            }
            rows[..current].iter().rposition(|candidate| candidate.level < row.level).map(|parent| vec![TreeIntent::Focus(parent)])
        }
        _ => None,
    }
}

/// 🖱️ What a primary press on row `index` means: the first press focuses and clicks it with the merge its modifiers name, a repeated press
/// (`click_count > 1`) yields to the row's double-click activation — React's `event.detail > 1` rule.
pub fn tree_pointer(modifiers: TreeNavModifiers, index: usize, click_count: u32) -> Vec<TreeIntent> {
    if click_count > 1 {
        return vec![TreeIntent::DoubleClick(index)];
    }
    vec![TreeIntent::Focus(index), TreeIntent::Click { index, merge: TreeMerge::from_modifiers(modifiers) }]
}

/// 🔤️ The row a typed `query` jumps to: the first row after `current` (wrapping) whose label starts with it, case-insensitively; a query
/// of one repeated character cycles through the rows that start with that character.
pub fn tree_typeahead(rows: &[TreeNavRow], current: usize, query: &str) -> Option<usize> {
    let lowered = query.to_lowercase();
    let first = lowered.chars().next()?;
    let cycling = lowered.chars().all(|character| character == first);
    let needle = if cycling { first.to_string() } else { lowered };
    let skip = if cycling { 1 } else { 0 };
    (0..rows.len()).map(|offset| (current + skip + offset) % rows.len()).find(|&index| rows[index].label.to_lowercase().starts_with(&needle))
}

/// 🎯️ The row that is the tree's one tab stop: the remembered focus when it is still visible, else the first selected row, else the first row.
pub fn tree_roving_stop(rows: &[TreeNavRow], remembered: Option<&str>, selected: &[String]) -> Option<usize> {
    remembered
        .and_then(|id| rows.iter().position(|row| row.id == id))
        .or_else(|| rows.iter().position(|row| selected.contains(&row.id)))
        .or_else(|| (!rows.is_empty()).then_some(0))
}

/// ☑️ A selection as the interaction machine keeps it: the chosen row ids in the order chosen, and the anchor a range extends from.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TreeSelection {
    pub ids: Vec<String>,
    pub anchor: Option<String>,
}

/// ☑️ The selection after picking `target` with `merge` — `getTreeNextSelectionState`. `ordered` is EVERY item in tree order, folded rows
/// included (`getTreeItemOrderedIds`), not just the visible ones. A single-select tree always replaces; a range replaces the selection with
/// the rows between the anchor (else the last selected, else the row focus came from, else the target) and the target and keeps the
/// anchor; a replace or an inversion moves the anchor to the target.
pub fn tree_selection(ordered: &[TreeNavRow], current: &TreeSelection, focus_before: Option<&str>, target: &str, merge: TreeMerge, multiple: bool) -> TreeSelection {
    let own = || TreeSelection { ids: vec![target.to_owned()], anchor: Some(target.to_owned()) };
    if !multiple {
        return own();
    }
    match merge {
        TreeMerge::Replace => own(),
        TreeMerge::Invertive => {
            let present = current.ids.iter().any(|id| id == target);
            let mut ids: Vec<String> = current.ids.iter().filter(|id| *id != target).cloned().collect();
            if !present {
                ids.push(target.to_owned());
            }
            TreeSelection { ids, anchor: Some(target.to_owned()) }
        }
        TreeMerge::Range => {
            let anchor = current.anchor.clone().or_else(|| current.ids.last().cloned()).or_else(|| focus_before.map(str::to_owned)).unwrap_or_else(|| target.to_owned());
            match (ordered.iter().position(|row| row.id == anchor), ordered.iter().position(|row| row.id == target)) {
                (Some(from), Some(to)) => TreeSelection { ids: ordered[from.min(to)..=from.max(to)].iter().map(|row| row.id.clone()).collect(), anchor: Some(anchor) },
                _ => own(),
            }
        }
    }
}

#[cfg(test)]
#[path = "../🧪️tests/⌨️roving/🦀️.rs"]
mod tests;
