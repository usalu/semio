//! 🌳️ The command tree: the command registry projected as launcher rows and as the wizard's steps —
//! the verb, the owner taxonomy and the command. A leaf names a registry command and the parameters
//! the row pins; nothing here resolves or starts anything, the registry's `Launch` is the only start input.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🔣️.json

use crate::registry::{Entry, Kind, Registry};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// #region 🔖️Types
/// 🧷️ What activating a launcher row starts: a registry command and the parameters the row pins.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Leaf {
    pub id: String,
    pub parameters: Vec<(String, String)>,
}

/// 🔎️ One line of the flat launcher: its searchable label and the invocation it starts.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LauncherRow {
    pub label: String,
    pub leaf: Leaf,
}

/// 🌿️ One node of the command tree.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommandNode {
    pub key: String,
    pub label: String,
    pub children: Vec<CommandNode>,
    pub leaf: Option<Leaf>,
}
// #endregion 🔖️Types

// #region 🔖️Discover
/// 🧭️ Discovers the registry of the workspace at `root` and projects it as the wizard tree.
pub fn discover(root: &Path) -> CommandNode { discover_cancellable(root, &std::sync::atomic::AtomicBool::new(false)) }

/// ⏳️ Discovers current commands with cooperative cancellation.
pub fn discover_cancellable(root: &Path, cancelled: &std::sync::atomic::AtomicBool) -> CommandNode { tree(&crate::inventory::discover(root, cancelled), true) }
// #endregion 🔖️Discover

// #region 🔖️Projection
/// 🍂️ The rows of one registry command with the step each adds below the command's own: a playground
/// offers one row per renderer, every other command one row.
fn rows_of(registry: &Registry, entry: &Entry) -> Vec<(Option<String>, Leaf)> {
    if entry.kind != Kind::Playground { return vec![(None, Leaf { id: entry.id.clone(), parameters: Vec::new() })]; }
    let renderers = registry.parameters(entry).into_iter().find(|parameter| parameter.id == "renderer").map(|parameter| parameter.values).unwrap_or_default();
    renderers.into_iter().map(|(renderer, _)| (Some(renderer.clone()), Leaf { id: entry.id.clone(), parameters: vec![("renderer".into(), renderer)] })).collect()
}

/// 🔭️ The flat launcher of the default searchable commands: `verb / owner / … / command`.
pub fn launcher(registry: &Registry) -> Vec<LauncherRow> {
    registry.entries().iter().filter(|entry| entry.listed).flat_map(|entry| rows_of(registry, entry).into_iter().map(move |(step, leaf)| LauncherRow { label: step.map_or_else(|| entry.label.clone(), |step| format!("{} / {step}", entry.label)), leaf })).collect()
}

#[derive(Default)]
struct TrieNode { label: String, children: BTreeMap<String, TrieNode>, leaf: Option<Leaf> }

impl TrieNode {
    fn insert(&mut self, path: &mut dyn Iterator<Item = (String, String)>, leaf: Leaf) {
        match path.next() {
            None => self.leaf = Some(leaf),
            Some((key, label)) => self.children.entry(key).or_insert_with(|| TrieNode { label, ..Default::default() }).insert(path, leaf),
        }
    }

    fn into_command_node(self, key: &str, depth: usize) -> CommandNode {
        let mut children: Vec<CommandNode> = self.children.into_iter().map(|(key, node)| node.into_command_node(&key, depth + 1)).collect();
        if depth == 0 { children.sort_by(|left, right| crate::registry::verb_rank(&left.key).cmp(&crate::registry::verb_rank(&right.key)).then_with(|| left.label.cmp(&right.label))); } else { children.sort_by(|left, right| left.label.cmp(&right.label)); }
        CommandNode { key: key.to_string(), label: if self.label.is_empty() { key.to_string() } else { self.label }, children, leaf: self.leaf }
    }
}

/// 🌲️ The registry as the wizard tree: the verb in launcher order at the root, then each command's
/// place in the owner taxonomy, sorted alphabetically. `unlisted` adds the commands of closed tickets.
pub fn tree(registry: &Registry, unlisted: bool) -> CommandNode {
    let mut trie = TrieNode { label: "semio".into(), ..Default::default() };
    for entry in registry.entries().iter().filter(|entry| unlisted || entry.listed) {
        for (step, leaf) in rows_of(registry, entry) {
            let path = std::iter::once((entry.verb.clone(), entry.verb.clone())).chain(entry.path.iter().map(|part| (part.key.clone(), part.label.clone()))).chain(step.map(|step| (step.clone(), step)));
            trie.insert(&mut path.into_iter(), leaf);
        }
    }
    trie.into_command_node("root", 0)
}
// #endregion 🔖️Projection

// #region 🔖️Document
/// 🔣️ The tree as the JSON document `🧬️schema/🔣️.json` describes.
pub fn tree_json(registry: &Registry, node: &CommandNode) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert("key".to_string(), serde_json::Value::String(node.key.clone()));
    object.insert("label".to_string(), serde_json::Value::String(node.label.clone()));
    if let Some(leaf) = &node.leaf { object.insert("leaf".to_string(), leaf_json(registry, leaf)); }
    if !node.children.is_empty() { object.insert("children".to_string(), serde_json::Value::Array(node.children.iter().map(|child| tree_json(registry, child)).collect())); }
    serde_json::Value::Object(object)
}

/// 🧾️ The discovered tree of a repository root as the pretty JSON document `semio command-tree
/// --dump-tree` prints, so a client never has to name the encoder crate itself.
pub fn tree_json_text(root: &Path) -> String {
    let registry = crate::inventory::discover(root, &std::sync::atomic::AtomicBool::new(false));
    serde_json::to_string_pretty(&tree_json(&registry, &tree(&registry, true))).unwrap_or_else(|error| format!("{{\"error\":\"{error}\"}}"))
}

fn leaf_json(registry: &Registry, leaf: &Leaf) -> serde_json::Value {
    match registry.find(&leaf.id).and_then(Entry::repo_action) {
        Some(action) => serde_json::json!({ "kind": "repo", "action": crate::repo_domain::action_key(action), "goArgv": action.go_argv() }),
        None => serde_json::json!({ "kind": "command", "id": leaf.id, "parameters": leaf.parameters }),
    }
}
// #endregion 🔖️Document

// #region 🔖️Command
/// 🖨️ Presents the discovered command tree without entering the interactive dashboard.
pub fn run(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {
    let root = parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from);
    if parsed.has_flag("dump-tree") {
        println!("{}", tree_json_text(&root));
        return 0;
    }
    for line in outline(&discover(&root), 0) {
        println!("{line}");
    }
    0
}

fn outline(node: &CommandNode, depth: usize) -> Vec<String> {
    let mut lines = vec![format!("{}{}", "  ".repeat(depth), node.label)];
    for child in &node.children {
        lines.extend(outline(child, depth + 1));
    }
    lines
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
