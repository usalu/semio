//! 🗄️ Workspace: what a workspace path holds and the in-memory workspace every operation reads.

use std::collections::BTreeMap;

//#region 🗄️Workspace

/// 📦️ What a workspace path holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry {
    /// 📂️ A directory with no content of its own.
    Directory,
    /// 📄️ A text file and its content.
    File(String),
}

/// 🐘️ A checkout held entirely in memory: relative slash-separated paths to entries. Every planner
/// reads one and no planner writes one; a plan applied through [`Executor`] is the only mutation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Workspace {
    pub(super) entries: BTreeMap<String, Entry>,
}

impl Workspace {
    /// 🆕️ An empty workspace.
    pub fn new() -> Self {
        Self { entries: BTreeMap::new() }
    }

    /// ➕️ Adds a file and every directory leading to it.
    pub fn with_file(mut self, path: &str, content: &str) -> Self {
        self.insert_file(path, content);
        self
    }

    /// 📁️ Adds a directory and every directory leading to it.
    pub fn with_directory(mut self, path: &str) -> Self {
        self.insert_directory(path);
        self
    }

    /// 🧱️ Puts a file at `path`, materialising its ancestors.
    pub fn insert_file(&mut self, path: &str, content: &str) {
        let key = clean_relative(path);
        if key.is_empty() {
            return;
        }
        self.insert_ancestors(&key);
        self.entries.insert(key, Entry::File(content.to_string()));
    }

    /// 📇️ Puts a directory at `path`, materialising its ancestors.
    pub fn insert_directory(&mut self, path: &str) {
        let key = clean_relative(path);
        if key.is_empty() {
            return;
        }
        self.insert_ancestors(&key);
        self.entries.entry(key).or_insert(Entry::Directory);
    }

    pub(super) fn insert_ancestors(&mut self, key: &str) {
        let mut prefix = String::new();
        let segments: Vec<&str> = key.split('/').collect();
        for segment in &segments[..segments.len().saturating_sub(1)] {
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(segment);
            self.entries.entry(prefix.clone()).or_insert(Entry::Directory);
        }
    }

    /// 🔍️ The entry at `path`.
    pub fn entry(&self, path: &str) -> Option<&Entry> {
        self.entries.get(&clean_relative(path))
    }

    /// 📖️ The content of the file at `path`.
    pub fn file(&self, path: &str) -> Option<&str> {
        match self.entries.get(&clean_relative(path)) {
            Some(Entry::File(content)) => Some(content.as_str()),
            _ => None,
        }
    }

    /// ✅️ Whether anything sits at `path`.
    pub fn exists(&self, path: &str) -> bool {
        self.entries.contains_key(&clean_relative(path))
    }

    /// 🛤️ Whether a directory sits at `path`.
    pub fn is_directory(&self, path: &str) -> bool {
        matches!(self.entries.get(&clean_relative(path)), Some(Entry::Directory))
    }

    /// 🗂️ Every path, in sorted order.
    pub fn paths(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }

    /// 🔃️ Every path and entry, in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Entry)> {
        self.entries.iter()
    }

    /// 🧾️ Every file path mapped to its content, in sorted order.
    pub fn files(&self) -> BTreeMap<String, String> {
        self.entries
            .iter()
            .filter_map(|(path, entry)| match entry {
                Entry::File(content) => Some((path.clone(), content.clone())),
                Entry::Directory => None,
            })
            .collect()
    }

    /// 🌳️ Every path inside `root`, `root` itself excluded.
    pub fn descendants(&self, root: &str) -> Vec<String> {
        let prefix = format!("{}/", clean_relative(root));
        self.entries.keys().filter(|p| p.starts_with(&prefix)).cloned().collect()
    }
}

/// 🧹️ A path reduced to the slash-separated relative form every planner speaks.
pub fn clean_relative(path: &str) -> String {
    let unified = path.replace('\\', "/");
    let mut segments: Vec<&str> = Vec::new();
    for segment in unified.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            other => segments.push(other),
        }
    }
    segments.join("/")
}

//#endregion 🗄️Workspace
