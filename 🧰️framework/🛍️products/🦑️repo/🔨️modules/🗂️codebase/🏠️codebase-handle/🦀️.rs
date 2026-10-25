//! 🧭️ Codebase handle: the codebase bound to one repository root with its construction, root accessors and cache invalidation. See [`super`].

use super::*;

//#region 🗂️Codebase
/// 🗂️ A repository root plus the caches the Go twin keeps in package-level globals.
pub struct Codebase {
    pub(super) root_dir: PathBuf,
    pub(super) technologies: Mutex<Option<Vec<Technology>>>,
    pub(super) folder_kinds: Mutex<HashMap<String, FolderKind>>,
    pub(super) gitignore: Mutex<Option<Option<semio_framework_repo_workspace::GitIgnore>>>,
}

impl Codebase {
    /// 🆕️ Binds a codebase to one repository root.
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self { root_dir: root_dir.into(), technologies: Mutex::new(None), folder_kinds: Mutex::new(HashMap::new()), gitignore: Mutex::new(None) }
    }

    /// 🏠️ The repository root this codebase walks.
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }

    /// 🔗️ The `file://` uri of the repository root.
    pub fn root_uri(&self) -> String {
        format!("file://{}", normalize_path(&self.root_dir.to_string_lossy()))
    }

    /// ♻️ Drops the cached technology tree so the next walk re-reads the filesystem.
    pub fn invalidate_technology_cache(&self) {
        *self.technologies.lock().expect("technology cache") = None;
        self.folder_kinds.lock().expect("folder kind cache").clear();
    }

    /// 📖️ Reads a file below the repository root as text.
    pub(super) fn read_text(&self, relative: &str) -> Option<String> {
        std::fs::read_to_string(self.root_dir.join(relative)).ok()
    }

    /// 🔍️ Reports whether a path below the repository root exists.
    pub(super) fn exists(&self, relative: &str) -> bool {
        self.root_dir.join(relative).exists()
    }
}
//#endregion 🗂️Codebase
