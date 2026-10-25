//! 🚫️ Ignore rules of the codebase: the compiled gitignore, the repo exclusion and the filtering of considered files. See [`super`].

use super::*;

//#region 🗂️Codebase
//#region 🙈️Ignore
impl Codebase {
    /// 🐙️ The compiled root `.gitignore`, decoded once.
    fn compiled_gitignore(&self) -> Option<semio_framework_repo_workspace::GitIgnore> {
        let mut cache = self.gitignore.lock().expect("gitignore cache");
        if let Some(cached) = cache.as_ref() {
            return cached.clone();
        }
        let compiled = semio_framework_repo_workspace::GitIgnore::compile_file(&self.root_dir.join(".gitignore")).ok();
        *cache = Some(compiled.clone());
        compiled
    }

    /// 📝️ The non-comment patterns of the root `.gitignore`, in file order.
    pub fn gitignore_patterns(&self) -> Vec<String> {
        let Some(content) = self.read_text(".gitignore") else { return Vec::new() };
        content.split('\n').map(str::trim).filter(|line| !line.is_empty() && !line.starts_with('#')).map(str::to_string).collect()
    }

    /// 🛤️ Normalises any path to a repository-relative slash path.
    pub fn normalize_repo_path(&self, path: &str) -> String {
        let normalized = normalize_path(path);
        let root = normalize_path(&self.root_dir.to_string_lossy());
        let relative = if let Some(stripped) = normalized.strip_prefix(&format!("{root}/")) { stripped.to_string() } else { normalized };
        relative.strip_prefix("./").unwrap_or(&relative).to_string()
    }

    /// 🦑️ Reports whether a path is structurally outside every codebase projection.
    pub fn is_repo_excluded_path(&self, path: &str) -> bool {
        let normalized = self.normalize_repo_path(path);
        if normalized.is_empty() {
            return false;
        }
        for root in [".🧬semio", "node_modules", ".git"] {
            if normalized == root || normalized.starts_with(&format!("{root}/")) {
                return true;
            }
        }
        for infix in ["/node_modules/", "/.git/"] {
            if normalized.contains(infix) {
                return true;
            }
        }
        if normalized == "assets/repo" || normalized.starts_with("assets/repo/") || normalized.contains("/asset/repo/") {
            return true;
        }
        for segment in ["/dist/", "/build/", "/target/", "/__pycache__/", "/.next/", "/coverage/"] {
            if normalized.contains(segment) {
                return true;
            }
        }
        if base_of(&normalized).ends_with(".Designer.cs") {
            return true;
        }
        normalized.contains("/codegen/")
    }

    /// 🙈️ Reports whether a path is ignored by the repository's own ignore rules.
    pub fn is_gitignored(&self, path: &str) -> bool {
        if base_of(path) == "LICENSE.md" {
            return true;
        }
        let relative = self.normalize_repo_path(path);
        if relative.is_empty() {
            return false;
        }
        self.compiled_gitignore().is_some_and(|ignore| ignore.matches_path(&relative))
    }

    /// 🧹️ Drops every structurally excluded and ignored path, preserving order.
    pub fn filter_considered_files(&self, files: &[String]) -> Vec<String> {
        files.iter().filter(|file| !self.is_repo_excluded_path(file) && !self.is_gitignored(file)).cloned().collect()
    }
}
//#endregion 🙈️Ignore
//#endregion 🗂️Codebase
