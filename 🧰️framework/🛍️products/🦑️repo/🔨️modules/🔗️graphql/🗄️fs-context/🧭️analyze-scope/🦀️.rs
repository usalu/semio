//! 🧭️ Analyze scope of the production context: the scope kinds an analyze request names and their parsing. See [`super`].

use super::*;

//#region 🗄️FsRepoContext
/// 🔍️ One analyze scope, parsed the way the reference `ParseScope` reads a raw scope string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnalyzeScope {
    Repo,
    Definition(String),
    Section(String),
    File(String),
    Technology(String),
    Folder(String),
}

impl AnalyzeScope {
    /// 🔤️ Parses a raw scope: `compose` and blank are the repository, `§` a definition, `#` a section.
    pub fn parse(raw: &str) -> AnalyzeScope {
        const CODE_EXTENSIONS: [&str; 13] = [".ts", ".tsx", ".js", ".jsx", ".py", ".cs", ".go", ".json", ".md", ".yaml", ".yml", ".sql", ".graphql"];
        if raw.is_empty() || raw == "compose" {
            return AnalyzeScope::Repo;
        }
        if let Some((file, _)) = raw.split_once('§') {
            return AnalyzeScope::Definition(file.to_string());
        }
        if let Some((file, _)) = raw.split_once('#') {
            return AnalyzeScope::Section(file.to_string());
        }
        let extension = semio_framework_repo_identity::ext_of(raw).to_lowercase();
        if CODE_EXTENSIONS.contains(&extension.as_str()) {
            return AnalyzeScope::File(raw.to_string());
        }
        if raw.starts_with("compose/") {
            return AnalyzeScope::Technology(raw.to_string());
        }
        AnalyzeScope::Folder(raw.to_string())
    }

    /// 🎯️ Whether a breach belongs to this scope, twin of `BreachMatchesScope`.
    pub fn matches(&self, breach: &Breach) -> bool {
        let scope = breach.scope.replace('\\', "/");
        let prefix = |base: &str| {
            let base = base.replace('\\', "/");
            let base = base.trim_end_matches('/');
            base.is_empty() || scope == base || scope.starts_with(&format!("{base}/"))
        };
        match self {
            AnalyzeScope::Repo => true,
            AnalyzeScope::File(path) => {
                let path = path.replace('\\', "/");
                scope == path || scope.starts_with(&format!("{path}#")) || scope.starts_with(&format!("{path}::"))
            }
            AnalyzeScope::Folder(path) | AnalyzeScope::Technology(path) => prefix(path),
            AnalyzeScope::Section(path) | AnalyzeScope::Definition(path) => scope.starts_with(&path.replace('\\', "/")),
        }
    }
}
//#endregion 🗄️FsRepoContext
