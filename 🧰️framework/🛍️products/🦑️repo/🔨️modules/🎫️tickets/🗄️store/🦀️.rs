//! 🏪️ The ticket store port with its in-memory and file implementations.

use super::*;

//#region 🗄️Store

/// 🧱️ What a node in the store is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeKind {
    File,
    Directory,
    Symlink,
}

/// 📏️ What the store knows about one node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub kind: NodeKind,
    pub size: u64,
    pub mode: u32,
}

/// 📁️ One child of a directory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DirectoryEntry {
    pub name: String,
    pub kind: NodeKind,
}

/// 🗄️ Every filesystem touch the ticket domain performs.
///
/// Paths are forward-slash strings; an implementation converts to the platform spelling itself.
pub trait TicketStore {
    /// 🔍️ What is at a path, without following a symlink.
    fn metadata(&self, path: &str) -> Option<NodeMetadata>;
    /// 📖️ Reads a document as text.
    fn read(&self, path: &str) -> TicketResult<String>;
    /// ✍️ Writes a document, creating parent directories.
    fn write(&self, path: &str, content: &str) -> TicketResult<()>;
    /// ✨️ Creates an empty file, failing when anything already occupies the path.
    fn create_exclusive(&self, path: &str, mode: u32) -> TicketResult<()>;
    /// 🗂️ Creates a directory and every missing ancestor.
    fn create_directory(&self, path: &str) -> TicketResult<()>;
    /// 🗑️ Removes one file.
    fn remove_file(&self, path: &str) -> TicketResult<()>;
    /// ➖️ Removes one directory, which must be empty.
    fn remove_directory(&self, path: &str) -> TicketResult<()>;
    /// 🧨️ Removes a whole subtree.
    fn remove_tree(&self, path: &str) -> TicketResult<()>;
    /// 🚚️ Moves a node.
    fn rename(&self, from: &str, to: &str) -> TicketResult<()>;
    /// 📇️ Lists the children of a directory, sorted by name.
    fn entries(&self, path: &str) -> TicketResult<Vec<DirectoryEntry>>;

    /// ✔️ Whether anything is at a path.
    fn exists(&self, path: &str) -> bool {
        self.metadata(path).is_some()
    }

    /// 📂️ Whether a directory is at a path.
    fn is_directory(&self, path: &str) -> bool {
        matches!(self.metadata(path), Some(NodeMetadata { kind: NodeKind::Directory, .. }))
    }

    /// 📄️ Whether a regular file is at a path.
    fn is_file(&self, path: &str) -> bool {
        matches!(self.metadata(path), Some(NodeMetadata { kind: NodeKind::File, .. }))
    }
}

/// 🧠️ One node of the in-memory store.
#[derive(Debug, Clone)]
struct MemoryNode {
    pub(super) kind: NodeKind,
    pub(super) content: String,
    pub(super) mode: u32,
}

/// 🐘️ A whole ticket tree in memory, with failure injection, so a test never touches `.🧬semio`.
#[derive(Debug, Default)]
pub struct MemoryTicketStore {
    nodes: RefCell<BTreeMap<String, MemoryNode>>,
    write_failures: RefCell<BTreeMap<String, String>>,
}

impl MemoryTicketStore {
    /// 🆕️ An empty store.
    pub fn new() -> MemoryTicketStore {
        MemoryTicketStore::default()
    }

    /// 💣️ Makes the next and every following write to a path fail with a message.
    pub fn fail_write(&self, path: &str, message: &str) {
        self.write_failures.borrow_mut().insert(normalize_separators(path), message.to_string());
    }

    /// 🧹️ Lifts an injected failure.
    pub fn clear_failure(&self, path: &str) {
        self.write_failures.borrow_mut().remove(&normalize_separators(path));
    }

    /// 📸️ Every path in the store with its content, for a projection.
    pub fn snapshot(&self) -> BTreeMap<String, String> {
        self.nodes
            .borrow()
            .iter()
            .filter(|(_, node)| node.kind == NodeKind::File)
            .map(|(path, node)| (path.clone(), node.content.clone()))
            .collect()
    }

    /// 🛤️ Every path in the store, files and directories alike.
    pub fn paths(&self) -> Vec<String> {
        self.nodes.borrow().keys().cloned().collect()
    }

    /// 🌱️ Seeds one file, creating its ancestors.
    pub fn seed_file(&self, path: &str, content: &str) {
        let path = normalize_separators(path);
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::File, content: content.to_string(), mode: 0o644 });
    }

    /// 🎲️ Seeds one file of a size, without materialising its bytes beyond the recorded length.
    pub fn seed_sized_file(&self, path: &str, size: u64) {
        let path = normalize_separators(path);
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path.clone(), MemoryNode { kind: NodeKind::File, content: String::new(), mode: 0o644 });
        if let Some(node) = self.nodes.borrow_mut().get_mut(&path) {
            node.content = "\0".repeat(usize::try_from(size).unwrap_or(usize::MAX));
        }
    }

    /// 🪴️ Seeds one directory.
    pub fn seed_directory(&self, path: &str) {
        let path = normalize_separators(path);
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::Directory, content: String::new(), mode: 0o755 });
    }

    /// 🔗️ Seeds a symlink, which every transaction guard must refuse.
    pub fn seed_symlink(&self, path: &str) {
        let path = normalize_separators(path);
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::Symlink, content: String::new(), mode: 0o777 });
    }

    fn ensure_ancestors(&self, path: &str) {
        let mut current = dir_name(path);
        let mut pending: Vec<String> = Vec::new();
        while current != "." && current != "/" && !current.is_empty() {
            if self.nodes.borrow().contains_key(&current) {
                break;
            }
            pending.push(current.clone());
            let parent = dir_name(&current);
            if parent == current {
                break;
            }
            current = parent;
        }
        for directory in pending.into_iter().rev() {
            self.nodes.borrow_mut().insert(directory, MemoryNode { kind: NodeKind::Directory, content: String::new(), mode: 0o755 });
        }
    }

    fn injected(&self, path: &str) -> Option<String> {
        self.write_failures.borrow().get(path).cloned()
    }
}

impl TicketStore for MemoryTicketStore {
    fn metadata(&self, path: &str) -> Option<NodeMetadata> {
        let path = normalize_separators(path);
        self.nodes.borrow().get(&path).map(|node| NodeMetadata { kind: node.kind, size: node.content.len() as u64, mode: node.mode })
    }

    fn read(&self, path: &str) -> TicketResult<String> {
        let path = normalize_separators(path);
        match self.nodes.borrow().get(&path) {
            Some(node) if node.kind == NodeKind::File => Ok(node.content.clone()),
            Some(_) => Err(TicketError::store(format!("{path} is not a regular file"))),
            None => Err(TicketError::not_found(format!("{path} does not exist"))),
        }
    }

    fn write(&self, path: &str, content: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        if let Some(message) = self.injected(&path) {
            return Err(TicketError::store(message));
        }
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::File, content: content.to_string(), mode: 0o644 });
        Ok(())
    }

    fn create_exclusive(&self, path: &str, mode: u32) -> TicketResult<()> {
        let path = normalize_separators(path);
        if let Some(message) = self.injected(&path) {
            return Err(TicketError::store(message));
        }
        if self.nodes.borrow().contains_key(&path) {
            return Err(TicketError::conflict(format!("{path} already exists")));
        }
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::File, content: String::new(), mode });
        Ok(())
    }

    fn create_directory(&self, path: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        if let Some(message) = self.injected(&path) {
            return Err(TicketError::store(message));
        }
        if let Some(existing) = self.nodes.borrow().get(&path) {
            return if existing.kind == NodeKind::Directory { Ok(()) } else { Err(TicketError::conflict(format!("{path} is not a directory"))) };
        }
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::Directory, content: String::new(), mode: 0o755 });
        Ok(())
    }

    fn remove_file(&self, path: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        match self.nodes.borrow_mut().remove(&path) {
            Some(_) => Ok(()),
            None => Err(TicketError::not_found(format!("{path} does not exist"))),
        }
    }

    fn remove_directory(&self, path: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        let prefix = format!("{path}/");
        if self.nodes.borrow().keys().any(|candidate| candidate.starts_with(&prefix)) {
            return Err(TicketError::conflict(format!("{path} is not empty")));
        }
        match self.nodes.borrow_mut().remove(&path) {
            Some(_) => Ok(()),
            None => Err(TicketError::not_found(format!("{path} does not exist"))),
        }
    }

    fn remove_tree(&self, path: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        let prefix = format!("{path}/");
        let doomed: Vec<String> = self.nodes.borrow().keys().filter(|candidate| **candidate == path || candidate.starts_with(&prefix)).cloned().collect();
        let mut nodes = self.nodes.borrow_mut();
        for candidate in doomed {
            nodes.remove(&candidate);
        }
        Ok(())
    }

    fn rename(&self, from: &str, to: &str) -> TicketResult<()> {
        let from = normalize_separators(from);
        let to = normalize_separators(to);
        if let Some(message) = self.injected(&to) {
            return Err(TicketError::store(message));
        }
        if !self.nodes.borrow().contains_key(&from) {
            return Err(TicketError::not_found(format!("{from} does not exist")));
        }
        if self.nodes.borrow().contains_key(&to) {
            return Err(TicketError::conflict(format!("{to} already exists")));
        }
        let prefix = format!("{from}/");
        let moved: Vec<String> = self.nodes.borrow().keys().filter(|candidate| **candidate == from || candidate.starts_with(&prefix)).cloned().collect();
        self.ensure_ancestors(&to);
        let mut nodes = self.nodes.borrow_mut();
        for candidate in moved {
            let Some(node) = nodes.remove(&candidate) else { continue };
            let target = if candidate == from { to.clone() } else { format!("{to}/{}", &candidate[prefix.len()..]) };
            nodes.insert(target, node);
        }
        Ok(())
    }

    fn entries(&self, path: &str) -> TicketResult<Vec<DirectoryEntry>> {
        let path = normalize_separators(path);
        if !self.is_directory(&path) {
            return Err(TicketError::not_found(format!("{path} is not a directory")));
        }
        let prefix = format!("{path}/");
        let mut listed: BTreeSet<DirectoryEntry> = BTreeSet::new();
        for (candidate, node) in self.nodes.borrow().iter() {
            let Some(rest) = candidate.strip_prefix(&prefix) else { continue };
            if rest.contains('/') {
                continue;
            }
            listed.insert(DirectoryEntry { name: rest.to_string(), kind: node.kind });
        }
        Ok(listed.into_iter().collect())
    }
}

/// 💽️ The real filesystem.
#[derive(Debug, Clone, Default)]
pub struct FileTicketStore;

impl FileTicketStore {
    /// 🏠️ The store rooted at the machine's filesystem.
    pub fn new() -> FileTicketStore {
        FileTicketStore
    }
}

impl TicketStore for FileTicketStore {
    fn metadata(&self, path: &str) -> Option<NodeMetadata> {
        let metadata = std::fs::symlink_metadata(path).ok()?;
        let kind = if metadata.file_type().is_symlink() {
            NodeKind::Symlink
        } else if metadata.is_dir() {
            NodeKind::Directory
        } else {
            NodeKind::File
        };
        Some(NodeMetadata { kind, size: metadata.len(), mode: file_mode(&metadata) })
    }

    fn read(&self, path: &str) -> TicketResult<String> {
        std::fs::read_to_string(path).map_err(|error| TicketError::store(format!("read {path}: {error}")))
    }

    fn write(&self, path: &str, content: &str) -> TicketResult<()> {
        let parent = dir_name(path);
        if parent != "." && parent != "/" {
            std::fs::create_dir_all(&parent).map_err(|error| TicketError::store(format!("create {parent}: {error}")))?;
        }
        std::fs::write(path, content).map_err(|error| TicketError::store(format!("write {path}: {error}")))
    }

    fn create_exclusive(&self, path: &str, _mode: u32) -> TicketResult<()> {
        std::fs::OpenOptions::new().write(true).create_new(true).open(path).map(|_| ()).map_err(|error| TicketError::store(format!("create {path}: {error}")))
    }

    fn create_directory(&self, path: &str) -> TicketResult<()> {
        std::fs::create_dir_all(path).map_err(|error| TicketError::store(format!("create {path}: {error}")))
    }

    fn remove_file(&self, path: &str) -> TicketResult<()> {
        std::fs::remove_file(path).map_err(|error| TicketError::store(format!("remove {path}: {error}")))
    }

    fn remove_directory(&self, path: &str) -> TicketResult<()> {
        std::fs::remove_dir(path).map_err(|error| TicketError::store(format!("remove {path}: {error}")))
    }

    fn remove_tree(&self, path: &str) -> TicketResult<()> {
        match std::fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(TicketError::store(format!("remove {path}: {error}"))),
        }
    }

    fn rename(&self, from: &str, to: &str) -> TicketResult<()> {
        std::fs::rename(from, to).map_err(|error| TicketError::store(format!("rename {from} -> {to}: {error}")))
    }

    fn entries(&self, path: &str) -> TicketResult<Vec<DirectoryEntry>> {
        let reader = std::fs::read_dir(path).map_err(|error| TicketError::store(format!("read directory {path}: {error}")))?;
        let mut listed: Vec<DirectoryEntry> = Vec::new();
        for entry in reader {
            let entry = entry.map_err(|error| TicketError::store(format!("read directory {path}: {error}")))?;
            let file_type = entry.file_type().map_err(|error| TicketError::store(format!("read directory {path}: {error}")))?;
            let kind = if file_type.is_symlink() {
                NodeKind::Symlink
            } else if file_type.is_dir() {
                NodeKind::Directory
            } else {
                NodeKind::File
            };
            listed.push(DirectoryEntry { name: entry.file_name().to_string_lossy().to_string(), kind });
        }
        listed.sort();
        Ok(listed)
    }
}

#[cfg(unix)]
fn file_mode(metadata: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::MetadataExt;
    metadata.mode()
}

#[cfg(not(unix))]
fn file_mode(metadata: &std::fs::Metadata) -> u32 {
    if metadata.permissions().readonly() {
        0o444
    } else {
        0o644
    }
}

//#endregion 🗄️Store
