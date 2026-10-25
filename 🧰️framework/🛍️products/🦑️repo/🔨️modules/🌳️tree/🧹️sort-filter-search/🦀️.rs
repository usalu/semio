//! 🧹️ Sorting, filtering and search: deterministic child order, kind filtering and the in-memory fuzzy search.
//!
//! Holds the `🔀️Sorting`, `🧹️Filtering`, `🔎️Search` regions of the crate root; the root re-exports its items.

use super::*;

//#region 🔀️Sorting

/// 📶️ Sorts every child list: folders before everything else, then by label, recursively.
pub fn sort_tree_children(node: &mut TreeNode) {
    if let Some(children) = node.children.as_mut() {
        children.sort_by(|left, right| {
            if left.kind != right.kind {
                let left_folder = left.kind == TreeNodeKind::Folder;
                let right_folder = right.kind == TreeNodeKind::Folder;
                if left_folder != right_folder {
                    return if left_folder { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
                }
            }
            left.label.cmp(&right.label)
        });
        for child in children.iter_mut() {
            sort_tree_children(child);
        }
    }
}

//#endregion 🔀️Sorting

//#region 🧹️Filtering

/// 🔍️ Filters a monorepo tree, keeping its structure and collapsing hidden entity levels.
pub fn filter_monorepo_tree(root: &TreeNode, filter: Option<&TreeFilter>) -> TreeNode {
    let Some(filter) = filter else { return root.clone() };
    let Some(mut filtered) = filter_node(root, filter) else {
        let mut empty = tree_node(TreeNodeKind::Category, "", ".", "");
        empty.children = Some(Vec::new());
        return empty;
    };
    if filter.has_only_kinds() || filter.exclude_kinds.as_ref().is_some_and(|kinds| !kinds.is_empty()) {
        collapse_filtered_kinds(&mut filtered, filter);
    }
    filtered
}

/// 🌳️ Keeps a node when it survives every criterion, otherwise keeps only its surviving subtree.
pub fn filter_node(node: &TreeNode, filter: &TreeFilter) -> Option<TreeNode> {
    if node.kind != TreeNodeKind::Category {
        if !filter.is_kind_visible(node.kind) {
            let kept: Vec<TreeNode> = children_of(node).iter().filter_map(|child| filter_node(child, filter)).collect();
            if kept.is_empty() {
                return None;
            }
            let mut copy = node.clone();
            copy.children = Some(kept);
            return Some(copy);
        }
        if !filter.matches_sub_kind(node.kind, &node.sub_kind) {
            return None;
        }
        if node.year > 0 && !filter.matches_date(node.year, node.month, node.day) {
            return None;
        }
        if !node.status.is_empty() && !filter.matches_status(&node.status) {
            return None;
        }
        if !node.contributor.is_empty() && !filter.matches_contributor(&node.contributor) {
            return None;
        }
    }
    let kept: Vec<TreeNode> = children_of(node).iter().filter_map(|child| filter_node(child, filter)).collect();
    if node.kind == TreeNodeKind::Category && kept.is_empty() {
        return None;
    }
    let mut copy = node.clone();
    copy.children = if kept.is_empty() { None } else { Some(kept) };
    Some(copy)
}

/// 🏷️ Lifts the children of every hidden entity node into its parent, depth first.
pub fn collapse_filtered_kinds(node: &mut TreeNode, filter: &TreeFilter) {
    let Some(children) = node.children.take() else { return };
    let mut result: Vec<TreeNode> = Vec::new();
    for mut child in children {
        collapse_filtered_kinds(&mut child, filter);
        if child.kind != TreeNodeKind::Category && !filter.is_kind_visible(child.kind) {
            result.extend(child.children.take().unwrap_or_default());
        } else {
            result.push(child);
        }
    }
    node.children = Some(result);
}

//#endregion 🧹️Filtering

//#region 🔎️Search

/// 🔤️ Flattens a node's data map into the searchable text the Go twin builds.
pub fn serialize_tree_node_data(data: Option<&BTreeMap<String, Value>>) -> String {
    let Some(data) = data else { return String::new() };
    if data.is_empty() {
        return String::new();
    }
    let parts: Vec<String> = data
        .values()
        .map(|value| match value {
            Value::String(inner) => inner.clone(),
            Value::Array(items) => items
                .iter()
                .map(|item| match item {
                    Value::String(inner) => inner.clone(),
                    other => other.to_string(),
                })
                .collect::<Vec<String>>()
                .join(" "),
            Value::Bool(inner) => inner.to_string(),
            Value::Number(inner) => inner.to_string(),
            Value::Null => "null".to_string(),
            other => other.to_string(),
        })
        .collect();
    parts.join(" ")
}

/// 📄️ Builds the search document of one node, including its non-category children.
pub fn build_search_document_text(node: &TreeNode) -> String {
    let mut parts = vec![
        node.label.clone(),
        node.id.clone(),
        node.sub_kind.clone(),
        node.description.clone(),
        node.uri.clone(),
        node.status.clone(),
        node.contributor.clone(),
        node.kind.as_str().to_string(),
        serialize_tree_node_data(node.data.as_ref()),
    ];
    for child in children_of(node) {
        if child.kind == TreeNodeKind::Category {
            continue;
        }
        parts.push(child.label.clone());
        parts.push(child.id.clone());
        parts.push(child.description.clone());
        parts.push(serialize_tree_node_data(child.data.as_ref()));
    }
    parts.join(" ")
}

/// 🔶️ The Levenshtein distance between two byte strings.
pub fn levenshtein(left: &str, right: &str) -> usize {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.is_empty() {
        return right.len();
    }
    if right.is_empty() {
        return left.len();
    }
    let mut previous: Vec<usize> = (0..=right.len()).collect();
    let mut current: Vec<usize> = vec![0; right.len() + 1];
    for row in 1..=left.len() {
        current[0] = row;
        for column in 1..=right.len() {
            let cost = usize::from(left[row - 1] != right[column - 1]);
            current[column] = (current[column - 1] + 1).min(previous[column] + 1).min(previous[column - 1] + cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[right.len()]
}

/// 🔹️ Whether a text contains a term literally or within the tolerated edit distance.
pub fn fuzzy_contains(text: &str, term: &str) -> bool {
    if text.contains(term) {
        return true;
    }
    let max_distance = if term.len() > 5 { 2 } else { 1 };
    for word in text.split_whitespace() {
        if levenshtein(word, term) <= max_distance {
            return true;
        }
        if word.len() >= term.len() {
            let mut start = 0;
            while start <= word.len() - term.len() + max_distance && start < word.len() {
                let end = (start + term.len() + max_distance).min(word.len());
                if word.is_char_boundary(start) && word.is_char_boundary(end) && levenshtein(&word[start..end], term) <= max_distance {
                    return true;
                }
                start += 1;
            }
        }
    }
    false
}

/// 🔎️ Prunes a tree to the nodes whose search document matches every query term.
pub fn search_tree_in_memory(root: &TreeNode, query: &str) -> TreeNode {
    let terms: Vec<String> = query.to_lowercase().split_whitespace().map(str::to_string).collect();
    if terms.is_empty() {
        return root.clone();
    }
    let mut matched: BTreeSet<String> = BTreeSet::new();
    collect_matches(root, &terms, &mut matched);
    if matched.is_empty() {
        let mut empty = tree_node(TreeNodeKind::Category, "", ".", "");
        empty.children = Some(Vec::new());
        return empty;
    }
    match prune_unmatched(root, &matched, false) {
        Some(pruned) => pruned,
        None => {
            let mut empty = tree_node(TreeNodeKind::Category, "", ".", "");
            empty.children = Some(Vec::new());
            empty
        }
    }
}

/// 🎯️ Records the document ids of every matching non-category node.
fn collect_matches(node: &TreeNode, terms: &[String], matched: &mut BTreeSet<String>) {
    if node.kind != TreeNodeKind::Category {
        let document = build_search_document_text(node).to_lowercase();
        if terms.iter().all(|term| fuzzy_contains(&document, term)) {
            matched.insert(document_id(node));
        }
    }
    for child in children_of(node) {
        collect_matches(child, terms, matched);
    }
}

/// 🆔️ The id a node is matched under: its id, or its label when it has none.
fn document_id(node: &TreeNode) -> String {
    if node.id.is_empty() {
        node.label.clone()
    } else {
        node.id.clone()
    }
}

/// ✂️ Keeps every matched node, its ancestors and, once matched, its whole subtree.
fn prune_unmatched(node: &TreeNode, matched: &BTreeSet<String>, ancestor_matched: bool) -> Option<TreeNode> {
    let self_matched = matched.contains(&document_id(node));
    let keep_descendants = ancestor_matched || self_matched;
    let kept: Vec<TreeNode> = children_of(node).iter().filter_map(|child| prune_unmatched(child, matched, keep_descendants)).collect();
    if keep_descendants || !kept.is_empty() || node.kind == TreeNodeKind::Category {
        if node.kind == TreeNodeKind::Category && kept.is_empty() && !self_matched && !ancestor_matched {
            return None;
        }
        let mut copy = node.clone();
        copy.children = if kept.is_empty() { None } else { Some(kept) };
        return Some(copy);
    }
    None
}

//#endregion 🔎️Search
