use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

// #region 🔖️Types
/// ▶️ One runnable shell invocation built by the wizard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    pub cmd: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
}

/// 🌳️ One node in the runtime-discovered command tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandNode {
    pub key: String,
    pub label: String,
    pub children: Vec<CommandNode>,
    pub spec: Option<CommandSpec>,
}
// #endregion 🔖️Types

// #region 🔖️Discover
const PROJECT_MANIFESTS: &[&str] = &["📋️project.json", "project.json"];
const WALK_SKIP_DIRS: &[&str] = &["node_modules", "target", ".git", "dist", "build", "generated", "cache"];
const TAXONOMY_SKIP_KEYS: &[&str] = &["packages", "modules", "products", "plugins", "artifacts", "standards", "subsets", "extensions", "targets"];

const VERB_ORDER: &[&str] = &["dev", "build", "test", "verify", "gate", "lint", "format", "generate", "publish"];

/// 🧭️ Walks the repo at `root` and builds the wizard command tree.
pub fn discover(root: &Path) -> CommandNode {
    let mut trie = TrieNode::default();
    collect_project_targets(root, root, &mut trie);
    inject_playground_dev(root, &mut trie);
    let mut root_node = trie.into_command_node("root", "semio");
    sort_tree(&mut root_node, 0);
    root_node
}
// #endregion 🔖️Discover

// #region 🔖️Trie
#[derive(Default)]
struct TrieNode {
    label: String,
    children: BTreeMap<String, TrieNode>,
    spec: Option<CommandSpec>,
}

impl TrieNode {
    fn insert_path(&mut self, path: &[Segment], spec: CommandSpec) {
        if path.is_empty() {
            self.spec = Some(spec);
            return;
        }
        let head = &path[0];
        let child = self.children.entry(head.key.clone()).or_insert_with(|| TrieNode { label: head.label.clone(), ..Default::default() });
        child.insert_path(&path[1..], spec);
    }

    fn into_command_node(self, key: &str, label: &str) -> CommandNode {
        CommandNode {
            key: key.to_string(),
            label: label.to_string(),
            children: self
                .children
                .into_iter()
                .map(|(k, n)| {
                    let child_label = if n.label.is_empty() { k.clone() } else { n.label.clone() };
                    n.into_command_node(&k, &child_label)
                })
                .collect(),
            spec: self.spec,
        }
    }
}

#[derive(Clone)]
struct Segment {
    key: String,
    label: String,
}
// #endregion 🔖️Trie

// #region 🔖️Walk
fn collect_project_targets(root: &Path, dir: &Path, trie: &mut TrieNode) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if should_skip_walk_dir(name) {
                continue;
            }
            collect_project_targets(root, &path, trie);
            continue;
        }
        if !PROJECT_MANIFESTS.iter().any(|m| path.file_name().and_then(|s| s.to_str()) == Some(*m)) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        let project_name = json.get("name").and_then(|v| v.as_str()).unwrap_or("workspace");
        let targets = json.get("targets").and_then(|v| v.as_object());
        if targets.is_none() {
            continue;
        }
        let manifest_dir = path.parent().unwrap_or(dir);
        let segments = taxonomy_segments(root, manifest_dir);
        for (target, _) in targets.unwrap() {
            let spec = CommandSpec { cmd: "bun".into(), args: vec!["nx".into(), "run".into(), format!("{project_name}:{target}")], cwd: root.to_path_buf(), env: Vec::new() };
            let mut path_segments = vec![Segment { key: target.clone(), label: target.clone() }];
            path_segments.extend(segments.clone());
            trie.insert_path(&path_segments, spec);
        }
    }
}

fn should_skip_walk_dir(name: &str) -> bool {
    let key = segment_key(name);
    WALK_SKIP_DIRS.iter().any(|s| key == *s) || name.starts_with('.') && name != ".semio"
}

fn should_skip_taxonomy_segment(name: &str) -> bool {
    let key = segment_key(name);
    TAXONOMY_SKIP_KEYS.iter().any(|s| key == *s)
}

fn taxonomy_segments(root: &Path, manifest_dir: &Path) -> Vec<Segment> {
    let rel = manifest_dir.strip_prefix(root).unwrap_or(manifest_dir);
    rel.components().filter_map(|c| c.as_os_str().to_str()).filter(|c| !c.is_empty()).filter(|c| !should_skip_taxonomy_segment(c)).map(|c| Segment { key: segment_key(c), label: c.to_string() }).filter(|s| !s.key.is_empty()).collect()
}

fn segment_key(component: &str) -> String {
    let s = component.trim();
    let start = s.find(|c: char| c.is_ascii_alphanumeric()).unwrap_or(s.len());
    s[start..].to_ascii_lowercase()
}

fn inject_playground_dev(root: &Path, trie: &mut TrieNode) {
    let catalog = crate::catalog::load_playground_catalog(root);
    for row in catalog {
        for renderer in ["react", "wgpu-wasm", "wgpu-native"] {
            let env = crate::env_contract::build_dev_env(&row.variant, Some(&row), &crate::env_contract::DevOptions { renderer: renderer.into(), ..Default::default() });
            let spec = CommandSpec { cmd: "bun".into(), args: vec!["nx".into(), "run".into(), "@semio-tech/framework-os-dev:dev".into()], cwd: root.to_path_buf(), env };
            let path = vec![
                Segment { key: "dev".into(), label: "dev".into() },
                Segment { key: segment_key(&row.plugin_id), label: row.plugin_id.clone() },
                Segment { key: segment_key(&row.variant), label: row.variant.clone() },
                Segment { key: renderer.to_string(), label: renderer.into() },
            ];
            trie.insert_path(&path, spec);
        }
    }
}

fn verb_rank(key: &str) -> usize {
    VERB_ORDER.iter().position(|v| *v == key).unwrap_or(VERB_ORDER.len() + 1)
}

fn sort_tree(node: &mut CommandNode, depth: usize) {
    if depth == 0 {
        node.children.sort_by(|a, b| verb_rank(&a.key).cmp(&verb_rank(&b.key)).then_with(|| a.label.cmp(&b.label)));
    } else {
        node.children.sort_by(|a, b| a.label.cmp(&b.label));
    }
    for child in &mut node.children {
        sort_tree(child, depth + 1);
    }
}
// #endregion 🔖️Walk

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn segment_key_strips_emoji_prefix() {
        assert_eq!(segment_key("🌊️flow"), "flow");
        assert_eq!(segment_key("📦️packages"), "packages");
    }

    #[test]
    fn discover_builds_verb_first_level() {
        let tmp = std::env::temp_dir().join(format!("semio-discover-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let pkg = tmp.join("✏️s/🔌️plugins/demo/📦️packages/🦀️rust");
        fs::create_dir_all(&pkg).unwrap();
        fs::write(pkg.join("📋️project.json"), r#"{"name":"@semio-tech/demo","targets":{"test":{"executor":"nx:run-commands"}}}"#).unwrap();
        let tree = discover(&tmp);
        let verbs: Vec<&str> = tree.children.iter().map(|c| c.key.as_str()).collect();
        assert!(verbs.contains(&"test"));
    }
}
// #endregion 🔖️Tests
