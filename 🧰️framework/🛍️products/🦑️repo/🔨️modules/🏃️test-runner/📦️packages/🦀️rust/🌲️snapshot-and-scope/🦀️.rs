//! 🌲️ Snapshot and scope: the in-memory filesystem snapshot and the resolution of a scope id to a test scope.
//!
//! Holds the `🌲️Snapshot`, `🔭️Scope` regions of the crate root; the root re-exports its items.

use super::*;

//#region 🌲️Snapshot
/// 🌲️ Whether a snapshot entry is a file or a directory.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    /// 📄️ A regular file.
    File,
    /// 📁️ A directory.
    Directory,
}

/// 📄️ One path in a planning snapshot, with contents only where a planning rule reads them.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SnapshotEntry {
    /// 🛤️ Repository-relative or absolute path, always forward-slashed.
    pub path: String,
    /// 🌲️ File or directory.
    pub kind: EntryKind,
    /// 📝️ Text contents, present only for files a planning rule inspects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
}

/// 📦️ The two bundle fields planning needs, projected from `📐️model`'s richer `Bundle`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SnapshotBundle {
    /// 🏷️ `technology/bundle` name.
    pub name: String,
    /// 🛤️ Repository-relative bundle root.
    pub root: String,
}

impl From<&semio_framework_repo_model::Bundle> for SnapshotBundle {
    fn from(bundle: &semio_framework_repo_model::Bundle) -> Self {
        Self { name: bundle.name.clone(), root: bundle.root.clone() }
    }
}

/// 🌍️ Everything planning is allowed to know about the world.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct FilesystemSnapshot {
    /// 🏠️ Repository root every relative path is joined onto.
    #[serde(default)]
    pub root: String,
    /// 🌲️ Every path the plan may probe.
    #[serde(default)]
    pub entries: Vec<SnapshotEntry>,
    /// 📦️ The bundle table Go reads through `LoadBundles`.
    #[serde(default)]
    pub bundles: Vec<SnapshotBundle>,
    /// 🐍️ Whether `uv` is on `PATH` — Go `uvExists`.
    #[serde(rename = "uvAvailable", default)]
    pub uv_available: bool,
}

impl FilesystemSnapshot {
    /// 🧭️ Absolute form of a repository-relative path.
    pub fn absolute(&self, path: &str) -> String {
        if is_absolute_path(path) {
            clean_path(path)
        } else {
            join_path(&self.root, path)
        }
    }

    /// ✅️ Go `FileExists`: true only for a present regular file.
    pub fn file_exists(&self, path: &str) -> bool {
        let wanted = clean_path(path);
        self.entries.iter().any(|entry| entry.kind == EntryKind::File && clean_path(&entry.path) == wanted)
    }

    /// 📁️ Whether a directory is present.
    pub fn directory_exists(&self, path: &str) -> bool {
        let wanted = clean_path(path);
        self.entries.iter().any(|entry| entry.kind == EntryKind::Directory && clean_path(&entry.path) == wanted)
    }

    /// 📝️ Contents of a recorded file.
    pub fn read_text(&self, path: &str) -> Option<&str> {
        let wanted = clean_path(path);
        self.entries
            .iter()
            .find(|entry| entry.kind == EntryKind::File && clean_path(&entry.path) == wanted)
            .and_then(|entry| entry.contents.as_deref())
    }

    /// 🔎️ Go `filepath.Glob(dir + "/" + pattern)`: non-recursive, direct children only.
    pub fn glob_children(&self, directory: &str, pattern: &str) -> Vec<String> {
        let directory = clean_path(directory);
        let mut matched: Vec<String> = self
            .entries
            .iter()
            .filter(|entry| entry.kind == EntryKind::File)
            .map(|entry| clean_path(&entry.path))
            .filter(|path| dir_name(path) == directory)
            .filter(|path| semio_framework_repo_workspace::glob_match(pattern, &base_name(path)).unwrap_or(false))
            .collect();
        matched.sort();
        matched
    }

    /// 🚶️ Every recorded file at or below `root`, in recorded order — Go `filepath.Walk` order after sorting.
    pub fn walk_files(&self, root: &str, recursive: bool) -> Vec<String> {
        let root = clean_path(root);
        let prefix = format!("{}/", root.trim_end_matches('/'));
        let mut found: Vec<String> = self
            .entries
            .iter()
            .filter(|entry| entry.kind == EntryKind::File)
            .map(|entry| clean_path(&entry.path))
            .filter(|path| *path == root || path.starts_with(&prefix))
            .filter(|path| recursive || dir_name(path) == root)
            .collect();
        found.sort();
        found
    }

    /// 📦️ Go `findBundleByName`: exact name first, then flattened equality.
    pub fn bundle_by_name(&self, name: &str) -> Option<&SnapshotBundle> {
        self.bundles.iter().find(|bundle| bundle.name == name || flat(&bundle.name) == flat(name))
    }

    /// 📦️ Go `GetBundleByPath`: the longest bundle root that prefixes the path.
    pub fn bundle_by_path(&self, path: &str) -> Option<&SnapshotBundle> {
        let path = clean_path(&normalize_separators(path));
        self.bundles
            .iter()
            .filter(|bundle| {
                let root = clean_path(&bundle.root);
                path == root || path.starts_with(&format!("{}/", root.trim_end_matches('/')))
            })
            .max_by_key(|bundle| clean_path(&bundle.root).len())
    }
}

/// 🗣️ Go `detectBundleLanguage`: manifest probing in a fixed, order-sensitive sequence.
pub fn detect_bundle_language(snapshot: &FilesystemSnapshot, bundle_root: &str) -> String {
    let absolute = snapshot.absolute(bundle_root);
    if snapshot.file_exists(&join_path(&absolute, "go.mod")) {
        return "go".to_string();
    }
    if snapshot.file_exists(&join_path(&absolute, "Cargo.toml")) {
        return "rust".to_string();
    }
    if !snapshot.glob_children(&absolute, "*.csproj").is_empty() {
        return "csharp".to_string();
    }
    if !snapshot.glob_children(&absolute, "*.sln").is_empty() {
        return "csharp".to_string();
    }
    if snapshot.file_exists(&join_path(&absolute, "package.json")) {
        return "typescript".to_string();
    }
    if snapshot.file_exists(&join_path(&absolute, "pyproject.toml")) || snapshot.file_exists(&join_path(&absolute, "requirements.txt")) {
        return "python".to_string();
    }
    String::new()
}
//#endregion 🌲️Snapshot

//#region 🔭️Scope
/// 🔭️ How wide a test run reaches — Go `testScopeKind`.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ScopeKind {
    /// 🌍️ Every bundle with a detectable language.
    #[default]
    All,
    /// 🧰️ Every bundle of one technology.
    Technology,
    /// ⌨️ One bundle.
    Bundle,
    /// 🥼️ One file.
    File,
    /// 🔖️ One section of one file.
    Section,
    /// 🧪️ One test function.
    Definition,
}

/// 🧪️ A resolved scope — Go `testScope`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct TestScope {
    /// 🔭️ Granularity.
    pub kind: ScopeKindField,
    /// 📦️ Bundle root, or the technology name for a technology scope.
    #[serde(default)]
    pub bundle_root: String,
    /// 🥼️ File path.
    #[serde(default)]
    pub file_path: String,
    /// 🔖️ Section name.
    #[serde(default)]
    pub section: String,
    /// 🧪️ Flat test name.
    #[serde(default)]
    pub test_name: String,
    /// 🗣️ Language.
    #[serde(default)]
    pub language: String,
}

/// 🔭️ `ScopeKind` with `All` as the serde default, so a fixture may omit it.
pub type ScopeKindField = ScopeKind;

impl TestScope {
    /// 🌍️ The scope an empty or unrecognised selector resolves to.
    pub fn all() -> Self {
        Self { kind: ScopeKind::All, ..Self::default() }
    }
}

/// 🔗️ Go `resolveTestScopes`: no selector means one `all` scope.
pub fn resolve_test_scopes(ids: &[String], snapshot: &FilesystemSnapshot, identity: &dyn EntityIdentity, languages: &dyn LanguageTable) -> Vec<TestScope> {
    if ids.is_empty() {
        return vec![TestScope::all()];
    }
    ids.iter().map(|raw| resolve_test_scope(raw, snapshot, identity, languages)).collect()
}

/// 🔷️ Go `resolveTestScope`: variation-selector stripping, `repo://` parsing, `p/` and `f/` routes.
pub fn resolve_test_scope(raw: &str, snapshot: &FilesystemSnapshot, identity: &dyn EntityIdentity, languages: &dyn LanguageTable) -> TestScope {
    let normalized = raw.replace(['\u{FE0E}', '\u{FE0F}'], "").trim().to_string();
    let uri = if normalized.starts_with("repo://") { normalized } else { identity.id_to_uri(&normalized) };
    if uri.is_empty() {
        return TestScope::all();
    }
    let path = uri.strip_prefix("repo://").unwrap_or(&uri).to_string();

    if let Some(rest) = path.strip_prefix("p/") {
        let parts = split_n(rest, '/', 3);
        if parts.len() == 2 {
            return TestScope { kind: ScopeKind::Technology, bundle_root: identity.path_from_uri_path(&parts[1]), ..TestScope::default() };
        }
        if parts.len() >= 3 {
            if let Some(bundle_rest) = parts[2].strip_prefix("b/") {
                let bundle_parts = split_n(bundle_rest, '/', 3);
                if bundle_parts.len() >= 2 {
                    let technology = identity.path_from_uri_path(&parts[1]);
                    let bundle = identity.path_from_uri_path(&bundle_parts[1]);
                    let bundle_name = format!("{}/{}", technology.trim_start_matches('@'), bundle);
                    let bundle_root = snapshot.bundle_by_name(&bundle_name).map_or(bundle_name, |found| found.root.clone());
                    if bundle_parts.len() == 2 {
                        let language = detect_bundle_language(snapshot, &bundle_root);
                        return TestScope { kind: ScopeKind::Bundle, bundle_root, language, ..TestScope::default() };
                    }
                    return resolve_test_scope_from_bundle_sub_path(&bundle_root, &bundle_parts[2], snapshot, identity, languages);
                }
            }
        }
    }

    if let Some(rest) = path.strip_prefix("f/") {
        let parts = split_n(rest, '/', 2);
        let file_path = identity.path_from_uri_path(&parts[0]);
        let language = languages.language_of(&file_path).unwrap_or_default();
        if parts.len() == 1 {
            return TestScope { kind: ScopeKind::File, file_path, language, ..TestScope::default() };
        }
        return resolve_test_scope_from_file_sub_path(&file_path, &language, &parts[1], identity);
    }

    TestScope::all()
}

/// 📦️ Go `resolveTestScopeFromBundleSubPath`: `f`/`s`/`d`/`fd` markers narrow the scope in order.
pub fn resolve_test_scope_from_bundle_sub_path(
    bundle_root: &str,
    sub_path: &str,
    snapshot: &FilesystemSnapshot,
    identity: &dyn EntityIdentity,
    languages: &dyn LanguageTable,
) -> TestScope {
    let mut language = detect_bundle_language(snapshot, bundle_root);
    let parts: Vec<&str> = sub_path.split('/').collect();
    let mut file_path = String::new();
    let mut section = String::new();
    let mut test_name = String::new();
    let mut kind = ScopeKind::Bundle;
    let mut index = 0;
    while index < parts.len() {
        if parts[index] == "f" && index + 1 < parts.len() {
            file_path = join_path(bundle_root, &identity.path_from_uri_path(parts[index + 1]));
            kind = ScopeKind::File;
            index += 1;
        } else if parts[index] == "s" && index + 1 < parts.len() {
            section = identity.path_from_uri_path(parts[index + 1]);
            kind = ScopeKind::Section;
            index += 1;
        } else if parts[index] == "d" && index + 2 < parts.len() {
            test_name = identity.path_from_uri_path(parts[index + 2]);
            kind = ScopeKind::Definition;
            index += 2;
        } else if parts[index] == "fd" && index + 2 < parts.len() {
            index += 2;
        }
        index += 1;
    }
    if let Some(found) = languages.language_of(&file_path) {
        language = found;
    }
    TestScope { kind, bundle_root: bundle_root.to_string(), file_path, section, test_name, language }
}

/// 📄️ Go `resolveTestScopeFromFileSubPath`.
pub fn resolve_test_scope_from_file_sub_path(file_path: &str, language: &str, sub_path: &str, identity: &dyn EntityIdentity) -> TestScope {
    let parts: Vec<&str> = sub_path.split('/').collect();
    let mut section = String::new();
    let mut test_name = String::new();
    let mut kind = ScopeKind::File;
    let mut index = 0;
    while index < parts.len() {
        if parts[index] == "s" && index + 1 < parts.len() {
            section = identity.path_from_uri_path(parts[index + 1]);
            kind = ScopeKind::Section;
            index += 1;
        } else if parts[index] == "d" && index + 2 < parts.len() {
            test_name = identity.path_from_uri_path(parts[index + 2]);
            kind = ScopeKind::Definition;
            index += 2;
        }
        index += 1;
    }
    TestScope { kind, bundle_root: String::new(), file_path: file_path.to_string(), section, test_name, language: language.to_string() }
}

/// ✂️ Go `strings.SplitN`: at most `n` pieces, the last one keeping every remaining separator.
fn split_n(value: &str, separator: char, n: usize) -> Vec<String> {
    value.splitn(n, separator).map(str::to_string).collect()
}
//#endregion 🔭️Scope
