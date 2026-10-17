//! 🚶️ Walk of the codebase: the lexical subtree walk, the extension glob and the scope to file resolution. See [`super`].

use super::*;

//#region 🗂️Codebase
//#region 🚶️Walk
impl Codebase {
    /// 🚶️ Walks a subtree in Go's lexical `WalkDir` order and returns every relative path whose
    /// extension is allowed and which no ignore pattern excludes.
    pub fn glob_by_extension(&self, pattern_base: &str, extensions: &[&str], ignore_patterns: &[String], respect_gitignore: bool) -> Vec<String> {
        let base = if pattern_base == "**/*" { String::new() } else { pattern_base.trim_end_matches("/**/*").to_string() };
        let abs_base = if base.is_empty() { self.root_dir.clone() } else { self.root_dir.join(&base) };
        if !abs_base.is_dir() {
            return Vec::new();
        }
        let mut all_ignore: Vec<String> = ignore_patterns.to_vec();
        if respect_gitignore {
            all_ignore.extend(self.gitignore_patterns());
        }
        let allowed: BTreeSet<String> = extensions.iter().map(|value| value.to_lowercase()).collect();
        let mut results = Vec::new();
        self.walk(&abs_base, &all_ignore, &allowed, &mut results);
        results
    }

    /// 🚶️ The recursive half of [`Codebase::glob_by_extension`].
    pub(super) fn walk(&self, dir: &Path, ignore_patterns: &[String], allowed: &BTreeSet<String>, results: &mut Vec<String>) {
        for name in read_dir_names(dir, false) {
            let path = dir.join(&name);
            let is_dir = path.is_dir();
            if is_dir && name.starts_with('.') {
                continue;
            }
            let Ok(relative) = path.strip_prefix(&self.root_dir) else { continue };
            let relative = normalize_path(&relative.to_string_lossy());
            let relative = relative.strip_prefix("./").unwrap_or(&relative).to_string();
            if relative.is_empty() || relative == "." {
                continue;
            }
            if ignore_patterns.iter().any(|pattern| matches_ignore_pattern(&relative, is_dir, pattern)) {
                continue;
            }
            if is_dir {
                self.walk(&path, ignore_patterns, allowed, results);
                continue;
            }
            let extension = ext_of(&relative).trim_start_matches('.').to_lowercase();
            if allowed.contains(&extension) {
                results.push(relative);
            }
        }
    }

    /// 🚶️ Resolves a scope to the source files it covers.
    pub fn scope_to_files(&self, scope: &Scope) -> Vec<String> {
        const EXTENSIONS: &[&str] = &["ts", "tsx", "py", "cs", "go", "rs"];
        let ignore_patterns = vec!["**/node_modules/**".to_string(), "**/.venv/**".to_string()];
        let files = match scope {
            Scope::Repo => self.glob_by_extension("**/*", EXTENSIONS, &ignore_patterns, true),
            Scope::Technology(name) => self
                .bundles()
                .into_iter()
                .find(|bundle| &bundle.name == name)
                .map(|bundle| self.glob_by_extension(&format!("{}/**/*", bundle.root), EXTENSIONS, &ignore_patterns, true))
                .unwrap_or_default(),
            Scope::Folder(path) if !path.is_empty() => self.glob_by_extension(&format!("{path}**/*"), EXTENSIONS, &ignore_patterns, true),
            Scope::Folder(_) => Vec::new(),
            Scope::File(path) => {
                if path.is_empty() {
                    Vec::new()
                } else {
                    vec![path.clone()]
                }
            }
        };
        self.filter_considered_files(&files)
    }
}
//#endregion 🚶️Walk
//#endregion 🗂️Codebase
