//! 📚️ Codebase context: everything one codebase projection is built from, with its construction and loading. See [`super`].

use super::*;

//#region 📚️Context
/// 💿️ Everything one codebase projection is built from.
pub struct CodebaseContext<'a> {
    /// 🗂️ The codebase the projection walks.
    pub codebase: &'a Codebase,
    /// 👥️ The bundles the projection groups by.
    pub bundles: Vec<Bundle>,
    /// 📔️ The repository-relative source files the projection covers.
    pub files: Vec<String>,
    /// ⚠️ The breaches contributed by the statutes module.
    pub breachs: Vec<Breach>,
}

impl<'a> CodebaseContext<'a> {
    /// 🆕️ Opens an empty context over one codebase.
    pub fn new(codebase: &'a Codebase) -> Self {
        Self { codebase, bundles: Vec::new(), files: Vec::new(), breachs: Vec::new() }
    }

    /// 📦️ Loads every bundle of the codebase.
    pub fn load_bundles(&mut self) {
        self.bundles = self.codebase.bundles();
    }

    /// 📄️ Loads every considered source file of the whole repository.
    pub fn load_files(&mut self) {
        self.files = self.codebase.scope_to_files(&Scope::Repo);
    }

    /// 🚨️ Adopts breaches the statutes module produced for these files.
    pub fn with_breachs(mut self, breachs: Vec<Breach>) -> Self {
        self.breachs = breachs;
        self
    }

    /// 🔖️ The label of the bundle owning a file, or the repository fallback.
    pub fn bundle_for_file(&self, file_path: &str) -> String {
        self.bundle_info(file_path).map_or_else(|| "repo/repo".to_string(), |(name, _)| name)
    }

    /// 🏪️ The label and root of the deepest bundle owning a path.
    pub fn bundle_info(&self, path: &str) -> Option<(String, String)> {
        let normalized = normalize_path(path);
        let mut matched_bundle = String::new();
        let mut matched_root = String::new();
        let mut matched_len = 0usize;
        for bundle in &self.bundles {
            let root = normalize_path(&bundle.root);
            if (normalized.starts_with(&format!("{root}/")) || normalized == root) && root.len() > matched_len {
                matched_bundle = bundle.name.clone();
                matched_root = root;
                matched_len = matched_root.len();
            }
        }
        if matched_len > 0 || !matched_bundle.is_empty() {
            return Some((normalize_bundle_label(&matched_bundle), matched_root));
        }
        None
    }

    /// 📖️ Reads a repository-relative file as text.
    pub(super) fn read(&self, file: &str) -> Option<String> {
        std::fs::read_to_string(self.codebase.root_dir().join(file)).ok()
    }
}
//#endregion 📚️Context
