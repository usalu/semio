//! 🔧️ Refactor operations of the production context: the move, integrate, extract and rename refactors, the cached breaches and the autofix. See [`super`].

use super::*;

//#region 🗄️FsRepoContext
impl FsRepoContext {
    /// 📑️ Renames a section of one file and reports the plan lines.
    pub fn move_section(&self, file: &str, old_name: &str, new_name: &str) -> Result<Vec<String>, ContextError> {
        let workspace = self.workspace(&[file], &[]);
        self.apply(semio_framework_repo_move::plan_section_move(&workspace, file, old_name, new_name))
    }

    /// 🧩️ Integrates a whole file into a named section of another file.
    pub fn integrate_file(&self, source: &str, target_section: &str, target_file: &str, target_parent: &str) -> Result<Vec<String>, ContextError> {
        let workspace = self.workspace(&[source, target_file], &[]);
        self.apply(semio_framework_repo_move::plan_integrate(&workspace, source, target_section, target_file, target_parent))
    }

    /// 🧲️ Extracts one section of a file into a file of its own.
    pub fn extract_section(&self, source_file: &str, source_section: &str, target_file: &str) -> Result<Vec<String>, ContextError> {
        let workspace = self.workspace(&[source_file, target_file], &[]);
        self.apply(semio_framework_repo_move::plan_extract(&workspace, source_file, source_section, target_file))
    }

    /// 🔤️ Rewrites every casing of one token across the contents and names below a scope.
    pub fn rename_token(&self, old_token: &str, new_token: &str, scope: &str) -> Result<Vec<String>, ContextError> {
        let workspace = self.workspace(&[], &[if scope.is_empty() { "." } else { scope }]);
        self.apply(semio_framework_repo_move::plan_rename(&workspace, old_token, new_token, scope))
    }

    /// 📂️ Every breach the repository's breach cache records.
    pub fn cached_breachs(&self) -> Result<Vec<Breach>, ContextError> {
        let directory = semio_framework_repo_workspace::repo_meta_path_for_root(&self.root, "⚡️cache/breaches");
        let Ok(entries) = std::fs::read_dir(&directory) else { return Ok(Vec::new()) };
        let mut names: Vec<std::path::PathBuf> = entries.flatten().map(|entry| entry.path()).filter(|path| path.is_file() && path.extension().is_some_and(|extension| extension == "json")).collect();
        names.sort();
        let documents = names.iter().map(|path| std::fs::read_to_string(path).map_err(|error| ContextError::new(format!("{}: {error}", path.display())))).collect::<Result<Vec<String>, ContextError>>()?;
        semio_framework_repo_statutes::breachs_from_cache(&documents).map_err(ContextError::new)
    }

    /// 🩹️ Applies every autofix to the files of one scope and reports how many breaches it cleared.
    pub fn autofix(&self, scope: Option<&str>) -> Result<FixResult, ContextError> {
        let scope = AnalyzeScope::parse(scope.unwrap_or("compose"));
        let files = self.codebase.scope_to_files(&match &scope {
            AnalyzeScope::Repo => CodebaseScope::Repo,
            AnalyzeScope::Technology(name) => CodebaseScope::Technology(name.clone()),
            AnalyzeScope::Folder(path) => CodebaseScope::Folder(if path.ends_with('/') { path.clone() } else { format!("{path}/") }),
            AnalyzeScope::File(path) | AnalyzeScope::Section(path) | AnalyzeScope::Definition(path) => CodebaseScope::File(path.clone()),
        });
        let sources: Vec<SourceFile> = files.iter().filter_map(|path| self.read(path).map(|content| SourceFile { path: path.clone(), content })).collect();
        let before = semio_framework_repo_statutes::filter_ignored(semio_framework_repo_statutes::analyze(&SourceSet::new(sources.clone())), &SourceSet::new(sources.clone()));
        let mut fixed_sources = Vec::with_capacity(sources.len());
        for source in &sources {
            let fixed = semio_framework_repo_statutes::autofix(source);
            if fixed.content != source.content {
                std::fs::write(self.root.join(&source.path), &fixed.content).map_err(|error| ContextError::new(format!("{}: {error}", source.path)))?;
            }
            fixed_sources.push(SourceFile { path: source.path.clone(), content: fixed.content });
        }
        let set = SourceSet::new(fixed_sources);
        let after = semio_framework_repo_statutes::filter_ignored(semio_framework_repo_statutes::analyze(&set), &set);
        Ok(FixResult { fixed: before.len().saturating_sub(after.len()) as i64, remaining: after.len() as i64, breachs: Some(after) })
    }
}
//#endregion 🗄️FsRepoContext
