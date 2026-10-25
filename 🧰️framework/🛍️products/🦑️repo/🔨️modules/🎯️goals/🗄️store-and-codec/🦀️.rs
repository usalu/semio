//! 🏪️ Store and codec of the goals: the goal document codec, the goal store port and its memory and filesystem stores. See [`super`].

use super::*;

//#region 📄️DocumentCodec
/// 📥️ Decodes a `🎯️goal.json` document, deriving the identifier and parent from the path the
/// document was found at, exactly as the reader does — the stored `parent` member is the compose
/// form and is authoritative only for the document, never for the tree.
pub fn decode_goal(id: &str, document: &str) -> Result<Goal, GoalError> {
    let mut goal: Goal = serde_json::from_str(document).map_err(|error| GoalError::Document(error.to_string()))?;
    goal.id = id.to_string();
    goal.parent = parent_goal_id(id);
    Ok(goal)
}

/// 📤️ Encodes a goal back into its document: two-space indentation, members in declaration order,
/// a trailing newline, and the `parent` member rewritten from the filesystem path into the compose
/// identifier the stored document carries.
pub fn encode_goal(goal: &Goal) -> Result<String, GoalError> {
    let mut stored = goal.clone();
    if !stored.parent.is_empty() && !stored.parent.contains(&identity::emoji_text(identity::entity("goal"))) {
        stored.parent = goal_path_to_compose_id(&stored.parent);
    }
    let text = serde_json::to_string_pretty(&stored).map_err(|error| GoalError::Document(error.to_string()))?;
    Ok(format!("{text}\n"))
}
//#endregion 📄️DocumentCodec

//#region 🗄️Store
/// 🗄️ Where goal documents live. One entry per `GOAL/SUBGOAL` identifier.
pub trait GoalStore {
    /// 📖️ The document at an identifier.
    fn read(&self, id: &str) -> Result<String, GoalError>;
    /// 💾️ Replaces or creates the document at an identifier.
    fn write(&self, id: &str, document: &str) -> Result<(), GoalError>;
    /// 🔍️ Whether a document exists at an identifier.
    fn exists(&self, id: &str) -> bool;
    /// 🚚️ Moves a goal and everything below it to a new identifier.
    fn rename(&self, from: &str, to: &str) -> Result<(), GoalError>;
    /// 🗑️ Removes a goal and everything below it.
    fn remove(&self, id: &str) -> Result<(), GoalError>;
    /// 📋️ Every identifier that carries a document, in ascending order.
    fn ids(&self) -> Vec<String>;
}

/// 🧠️ The in-memory store the language-agnostic tests run the lifecycle against.
#[derive(Debug, Default)]
pub struct MemoryGoalStore {
    documents: RefCell<BTreeMap<String, String>>,
}

impl MemoryGoalStore {
    /// 🆕️ An empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🌱️ A store seeded with `(identifier, document)` pairs.
    pub fn seeded<I: IntoIterator<Item = (String, String)>>(entries: I) -> Self {
        let store = Self::new();
        for (id, document) in entries {
            store.documents.borrow_mut().insert(id, document);
        }
        store
    }

    /// 📸️ Every stored `(identifier, document)` pair, in identifier order.
    pub fn snapshot(&self) -> Vec<(String, String)> {
        self.documents.borrow().iter().map(|(id, document)| (id.clone(), document.clone())).collect()
    }
}

impl GoalStore for MemoryGoalStore {
    fn read(&self, id: &str) -> Result<String, GoalError> {
        self.documents.borrow().get(id).cloned().ok_or_else(|| GoalError::NotFound(id.to_string()))
    }

    fn write(&self, id: &str, document: &str) -> Result<(), GoalError> {
        self.documents.borrow_mut().insert(id.to_string(), document.to_string());
        Ok(())
    }

    fn exists(&self, id: &str) -> bool {
        self.documents.borrow().contains_key(id)
    }

    fn rename(&self, from: &str, to: &str) -> Result<(), GoalError> {
        let mut documents = self.documents.borrow_mut();
        let moved: Vec<String> = documents.keys().filter(|key| key.as_str() == from || key.starts_with(&format!("{from}/"))).cloned().collect();
        if moved.is_empty() {
            return Err(GoalError::NotFound(from.to_string()));
        }
        for key in moved {
            let document = documents.remove(&key).unwrap_or_default();
            documents.insert(format!("{to}{}", &key[from.len()..]), document);
        }
        Ok(())
    }

    fn remove(&self, id: &str) -> Result<(), GoalError> {
        let mut documents = self.documents.borrow_mut();
        let removed: Vec<String> = documents.keys().filter(|key| key.as_str() == id || key.starts_with(&format!("{id}/"))).cloned().collect();
        for key in removed {
            documents.remove(&key);
        }
        Ok(())
    }

    fn ids(&self) -> Vec<String> {
        self.documents.borrow().keys().cloned().collect()
    }
}

/// 💽️ The store over a real `.🧬semio/🦑️repo/🎯️goals` tree.
#[derive(Debug, Clone)]
pub struct FsGoalStore {
    pub(super) root: PathBuf,
}

impl FsGoalStore {
    /// 📂️ The store of the goals directory of a repository root.
    pub fn new(repo_root: &Path) -> Self {
        Self { root: semio_framework_repo_workspace::goals_dir_for_root(repo_root) }
    }

    /// 📁️ The directory of one goal.
    pub fn directory(&self, id: &str) -> PathBuf {
        id.split('/').fold(self.root.clone(), |path, segment| path.join(segment))
    }

    fn document_path(&self, id: &str) -> PathBuf {
        self.directory(id).join("🎯️goal.json")
    }
}

impl GoalStore for FsGoalStore {
    fn read(&self, id: &str) -> Result<String, GoalError> {
        std::fs::read_to_string(self.document_path(id)).map_err(|_| GoalError::NotFound(id.to_string()))
    }

    fn write(&self, id: &str, document: &str) -> Result<(), GoalError> {
        let path = self.document_path(id);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| GoalError::Port(error.to_string()))?;
        }
        std::fs::write(path, document).map_err(|error| GoalError::Port(error.to_string()))
    }

    fn exists(&self, id: &str) -> bool {
        self.document_path(id).exists()
    }

    fn rename(&self, from: &str, to: &str) -> Result<(), GoalError> {
        let target = self.directory(to);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|error| GoalError::Port(error.to_string()))?;
        }
        std::fs::rename(self.directory(from), target).map_err(|error| GoalError::Port(error.to_string()))
    }

    fn remove(&self, id: &str) -> Result<(), GoalError> {
        std::fs::remove_dir_all(self.directory(id)).map_err(|error| GoalError::Port(error.to_string()))
    }

    fn ids(&self) -> Vec<String> {
        let mut found = Vec::new();
        collect_goal_ids(&self.root, "", &mut found);
        found.sort();
        found
    }
}

fn collect_goal_ids(directory: &Path, prefix: &str, found: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(directory) else { return };
    for entry in entries.flatten() {
        if !entry.path().is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let id = if prefix.is_empty() { name } else { format!("{prefix}/{name}") };
        if entry.path().join("🎯️goal.json").exists() {
            found.push(id.clone());
        }
        collect_goal_ids(&entry.path(), &id, found);
    }
}
//#endregion 🗄️Store
