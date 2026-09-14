//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🗂️ Repository codebase: the bundle, folder and file walk over a repository root, the technology
//! and bundle detection that gives every path its owning bundle, and the artifact id and uri
//! builders every other module addresses artifacts with.
//!
//! Behaviour twin of `github.com/usalu/semio/repo/codebase`. The Go original keeps its root
//! directory and its technology, folder-kind and gitignore caches in package-level globals; this
//! crate binds them to an explicit [`Codebase`] handle instead, so a walk can be pointed at any
//! root without a process-wide mutation.
//!
//! Everything above this module in the dependency DAG — statutes, tickets, goals, contributors —
//! contributes its own aggregates; this crate never reaches up for them. Breaches enter as already
//! decoded [`Breach`] records through [`CodebaseContext::with_breachs`].
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/🧬️schema/🔣️.json

//#endregion 🧲️Header

use semio_framework_repo_identity::{emoji_text, flat, normalize_path};
use semio_framework_repo_languages::{language_for_path, parse_definitions, parse_sections};
use semio_framework_repo_model::{
    bundle_kind_emoji, definition_kind_emoji, derive_technology_kind, file_kind_emoji, folder_kind_emoji, technology_kind_emoji, Breach, Bundle, BundleKind, BundleMetricsInternal, CbTreeNode, CbTreeNodeKind, Codebase as CodebaseSnapshot, CodebaseBreach,
    CodebaseBundle, CodebaseDefinition, CodebaseFile, CodebaseFolder, CodebaseSection, Definition, BreachFile, BreachFolder, BreachPriority, DefinitionMetricsInternal, FileBreachRef, FileMetricsInternal,
    FileRange, FolderKind, FolderMetricsInternal, Package, RangePosition, Section, SectionMetricsInternal, Technology, TechnologyKind,
};
use semio_framework_repo_workspace::glob_match;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

//#region 🛤️Paths

/// 🧹️ Reexported so a walk never reaches into `🪪️identity` for the `filepath` twins it keys on.
pub use semio_framework_repo_identity::{base_of, dir_of, ext_of};

//#endregion 🛤️Paths

//#region 🙈️Ignore

/// 🩶️ Reports whether any segment of a path marks generated output.
pub fn is_generated_folder(path: &str) -> bool {
    let normalized = normalize_path(path);
    for part in normalized.split('/') {
        if matches!(part, "generated" | "dist" | "build" | "node_modules" | "__pycache__" | ".next" | "coverage") {
            return true;
        }
    }
    for generated in ["js/vscode/generated", "js/compose/generated"] {
        if normalized == generated || normalized.starts_with(&format!("{generated}/")) {
            return true;
        }
    }
    false
}

//#endregion 🙈️Ignore

//#region 🏷️Kinds

/// 🏷️ Normalises a raw bundle name to the label the codebase aggregates key on.
pub fn normalize_bundle_label(name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }
    if name.starts_with('@') {
        return name.to_string();
    }
    match name {
        "vscode" => "repo/vscode".to_string(),
        "repo" => "repo/go".to_string(),
        other => format!("compose/{other}"),
    }
}

//#endregion 🏷️Kinds



//#region 🗂️Codebase

/// 🗂️ A repository root plus the caches the Go twin keeps in package-level globals.
pub struct Codebase {
    root_dir: PathBuf,
    technologies: Mutex<Option<Vec<Technology>>>,
    folder_kinds: Mutex<HashMap<String, FolderKind>>,
    gitignore: Mutex<Option<Option<semio_framework_repo_workspace::GitIgnore>>>,
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
    fn read_text(&self, relative: &str) -> Option<String> {
        std::fs::read_to_string(self.root_dir.join(relative)).ok()
    }

    /// 🔍️ Reports whether a path below the repository root exists.
    fn exists(&self, relative: &str) -> bool {
        self.root_dir.join(relative).exists()
    }

    //#region 🏗️Technologies

    /// 🏗️ Reports whether a root-level directory is a technology directory.
    fn is_technology_dir(&self, name: &str) -> bool {
        let Some(content) = self.read_text(&format!("{name}/README.md")) else { return false };
        if !content.starts_with("---") {
            return false;
        }
        if name.starts_with('.') {
            return false;
        }
        !matches!(name, "node_modules" | "target" | "temp")
    }

    /// 🧾️ Decodes the leading `---` front matter block of a document.
    fn front_matter(&self, relative: &str) -> Option<serde_json::Value> {
        let content = self.read_text(relative)?;
        if !content.starts_with("---") {
            return None;
        }
        let end = content[3..].find("---")?;
        if end == 0 {
            return None;
        }
        semio_framework_repo_yaml::unmarshal(&content[3..3 + end]).ok()
    }

    /// 📦️ Infers a bundle kind from a bundle's own `package.json` or `project.json`.
    pub fn derive_bundle_kind(&self, root: &str) -> BundleKind {
        for config in ["package.json", "project.json"] {
            let Some(content) = self.read_text(&format!("{root}/{config}")) else { continue };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else { continue };
            let Some(raw) = value.get("bundleKind").and_then(serde_json::Value::as_str) else { continue };
            if let Ok(kind) = serde_json::from_value::<BundleKind>(serde_json::Value::String(raw.to_lowercase())) {
                return kind;
            }
        }
        BundleKind::Library
    }

    /// 📦️ Reads a bundle's declared packages from its `package.json`.
    fn load_packages(&self, root: &str) -> Vec<Package> {
        let Some(content) = self.read_text(&format!("{root}/package.json")) else { return Vec::new() };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else { return Vec::new() };
        let name = value.get("name").and_then(serde_json::Value::as_str).unwrap_or_default();
        if name.is_empty() {
            return Vec::new();
        }
        vec![Package {
            name: name.to_string(),
            version: value.get("version").and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
            path: root.to_string(),
            kind: "npm".to_string(),
        }]
    }

    /// 🧱️ Reads the `sourceRoot` and `tags` a bundle's project or package manifest declares.
    fn apply_manifest(&self, bundle: &mut Bundle) {
        let mut config = format!("{}/project.json", bundle.root);
        if !self.exists(&config) {
            config = format!("{}/package.json", bundle.root);
        }
        let Some(content) = self.read_text(&config) else { return };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else { return };
        bundle.source_root = value.get("sourceRoot").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        bundle.tags = value
            .get("tags")
            .and_then(serde_json::Value::as_array)
            .map(|items| items.iter().filter_map(serde_json::Value::as_str).map(str::to_string).collect())
            .unwrap_or_default();
    }

    /// 🎨️ Reads a bundle's declared emoji from its `AGENTS.md` front matter.
    fn bundle_emoji(&self, root: &str) -> String {
        self.front_matter(&format!("{root}/AGENTS.md"))
            .and_then(|value| value.get("bundle").and_then(|bundle| bundle.get("emoji")).and_then(serde_json::Value::as_str).map(str::to_string))
            .unwrap_or_default()
    }

    /// 🏗️ Walks the repository root and returns every technology with its bundles.
    pub fn technologies(&self) -> Vec<Technology> {
        let mut cache = self.technologies.lock().expect("technology cache");
        if let Some(cached) = cache.as_ref() {
            return cached.clone();
        }
        let loaded = self.load_technologies();
        *cache = Some(loaded.clone());
        loaded
    }

    /// 📦️ Every bundle of every technology, in technology order.
    pub fn bundles(&self) -> Vec<Bundle> {
        self.technologies().into_iter().flat_map(|technology| technology.bundles.unwrap_or_default()).collect()
    }

    /// 🏗️ The uncached technology walk.
    fn load_technologies(&self) -> Vec<Technology> {
        let mut technologies = Vec::new();
        for name in read_dir_names(&self.root_dir, true) {
            if !self.is_technology_dir(&name) {
                continue;
            }
            let raw_name = name.strip_prefix('@').unwrap_or(&name).to_string();
            let mut technology_name = raw_name.clone();
            let mut technology_kind = derive_technology_kind(&raw_name);
            if let Some(matter) = self.front_matter(&format!("{name}/README.md")) {
                if let Some(declared) = matter.get("name").and_then(serde_json::Value::as_str) {
                    if !declared.is_empty() {
                        technology_name = declared.to_string();
                    }
                }
                if let Some(declared) = matter.get("kind").and_then(serde_json::Value::as_str) {
                    match declared.to_lowercase().as_str() {
                        "user" => technology_kind = TechnologyKind::User,
                        "infrastructure" => technology_kind = TechnologyKind::Infrastructure,
                        "research" => technology_kind = TechnologyKind::Research,
                        _ => {}
                    }
                }
            }
            let emoji = self
                .front_matter(&format!("{name}/AGENTS.md"))
                .and_then(|matter| matter.get("emoji").and_then(serde_json::Value::as_str).map(str::to_string))
                .unwrap_or_default();
            let mut bundles = Vec::new();
            for sub in read_dir_names(&self.root_dir.join(&name), true) {
                if sub.starts_with('.') || sub == "node_modules" {
                    continue;
                }
                if sub == "sites" {
                    for site in read_dir_names(&self.root_dir.join(&name).join(&sub), true) {
                        if site.starts_with('.') || site == "node_modules" {
                            continue;
                        }
                        let root = format!("{name}/{sub}/{site}");
                        let mut bundle = Bundle {
                            name: format!("{technology_name}/{site}"),
                            root: root.clone(),
                            source_root: String::new(),
                            technology_name: technology_name.clone(),
                            tags: Vec::new(),
                            kind: BundleKind::Site,
                            emoji: self.bundle_emoji(&root),
                            packages: self.load_packages(&root),
                        };
                        self.apply_manifest(&mut bundle);
                        bundles.push(bundle);
                    }
                    continue;
                }
                let root = format!("{name}/{sub}");
                let mut bundle = Bundle {
                    name: format!("{technology_name}/{sub}"),
                    root: root.clone(),
                    source_root: String::new(),
                    technology_name: technology_name.clone(),
                    tags: Vec::new(),
                    kind: self.derive_bundle_kind(&root),
                    emoji: self.bundle_emoji(&root),
                    packages: self.load_packages(&root),
                };
                self.apply_manifest(&mut bundle);
                bundles.push(bundle);
            }
            technologies.push(Technology { name: technology_name, root: name, kind: technology_kind, emoji, bundles: Some(bundles) });
        }
        technologies
    }

    /// 🏗️ The emoji a technology code resolves to, falling back to its derived kind.
    fn resolve_technology_emoji(&self, technology_code: &str) -> String {
        for technology in self.technologies() {
            if technology.name == technology_code {
                if !technology.emoji.is_empty() {
                    return technology.emoji;
                }
                break;
            }
        }
        derive_technology_kind(technology_code).as_str().to_string()
    }

    /// 🏗️ The emoji id of a technology.
    pub fn technology_id(&self, technology: &Technology) -> String {
        format!("{}{}", technology_kind_emoji(technology), flat(&technology.name))
    }

    /// 📦️ The emoji id of a bundle.
    pub fn bundle_id(&self, bundle: &Bundle) -> String {
        let (technology_code, bundle_code) = split_bundle_name(&bundle.name);
        let technology_emoji = self.resolve_technology_emoji(&technology_code);
        format!("{}{}{}{}", emoji_text(&technology_emoji), flat(&technology_code), bundle_kind_emoji(bundle), flat(&bundle_code))
    }

    /// 📦️ The bundle owning a path — the deepest matching bundle root.
    pub fn bundle_by_path(&self, path: &str) -> Option<Bundle> {
        let normalized = normalize_path(path);
        let mut best: Option<Bundle> = None;
        let mut matched = 0usize;
        for bundle in self.bundles() {
            let root = normalize_path(&bundle.root);
            if (normalized.starts_with(&format!("{root}/")) || normalized == root) && root.len() > matched {
                matched = root.len();
                best = Some(bundle);
            }
        }
        best
    }

    //#endregion 🏗️Technologies

    //#region 🙈️Ignore

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

    /// 🐙️ The non-comment patterns of the root `.gitignore`, in file order.
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

    /// 🚫️ Reports whether a path is structurally outside every codebase projection.
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

    /// 🐙️ Reports whether a path is ignored by the repository's own ignore rules.
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

    //#endregion 🙈️Ignore

    //#region 🚶️Walk

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
    fn walk(&self, dir: &Path, ignore_patterns: &[String], allowed: &BTreeSet<String>, results: &mut Vec<String>) {
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

    //#endregion 🚶️Walk

    //#region 🪪️Artifact Ids

    /// 📁️ Infers whether a folder is required by a toolchain or purely organisational.
    pub fn derive_folder_kind(&self, path: &str) -> FolderKind {
        if base_of(path).starts_with('.') {
            return FolderKind::Required;
        }
        if let Some(cached) = self.folder_kinds.lock().expect("folder kind cache").get(path) {
            return *cached;
        }
        let mut kind = FolderKind::Organization;
        for indicator in ["package.json", "pyproject.toml", "go.mod", "Cargo.toml"] {
            if self.exists(&format!("{path}/{indicator}")) {
                kind = FolderKind::Required;
                break;
            }
        }
        if kind == FolderKind::Organization {
            for name in read_dir_names(&self.root_dir.join(path), false) {
                if self.root_dir.join(path).join(&name).is_dir() {
                    continue;
                }
                let extension = ext_of(&name);
                if extension == ".csproj" || extension == ".sln" {
                    kind = FolderKind::Required;
                    break;
                }
            }
        }
        self.folder_kinds.lock().expect("folder kind cache").insert(path.to_string(), kind);
        kind
    }

    /// 🛤️ Builds the id of the folder chain leading to a directory.
    fn resolve_parent_id_from_path(&self, dir_path: &str) -> String {
        let normalized = normalize_path(dir_path);
        if normalized == "." || normalized.is_empty() {
            return self.bundle_by_path(".").map(|bundle| self.bundle_id(&bundle)).unwrap_or_default();
        }
        if let Some(bundle) = self.bundle_by_path(&normalized) {
            let bundle_root = normalize_path(&bundle.root);
            if normalized == bundle_root {
                return self.bundle_id(&bundle);
            }
            if let Some(relative) = normalized.strip_prefix(&format!("{bundle_root}/")) {
                let mut parent_id = self.bundle_id(&bundle);
                let mut current = bundle_root;
                for part in relative.split('/') {
                    current = format!("{current}/{part}");
                    parent_id = format!("{parent_id}{}{}", folder_kind_emoji(self.derive_folder_kind(&current)), flat(part));
                }
                return parent_id;
            }
        }
        let mut parent_id = String::new();
        let mut current = String::new();
        for part in normalized.split('/') {
            current = if current.is_empty() { part.to_string() } else { format!("{current}/{part}") };
            parent_id = format!("{parent_id}{}{}", folder_kind_emoji(self.derive_folder_kind(&current)), flat(part));
        }
        parent_id
    }

    /// 📁️ Builds the emoji id of a folder.
    pub fn build_folder_id(&self, path: &str) -> String {
        let normalized = normalize_path(path);
        if normalized == "." || normalized.is_empty() {
            return self.bundle_by_path(".").map(|bundle| self.bundle_id(&bundle)).unwrap_or_default();
        }
        let dir = dir_of(&normalized);
        let name = base_of(&normalized);
        let bundle = self.bundle_by_path(&normalized);
        let parent_id = match &bundle {
            Some(bundle) => {
                let bundle_root = normalize_path(&bundle.root);
                if normalized == bundle_root {
                    return self.bundle_id(bundle);
                }
                if dir == bundle_root {
                    self.bundle_id(bundle)
                } else {
                    self.resolve_parent_id_from_path(&dir)
                }
            }
            None if dir != "." && !dir.is_empty() => self.resolve_parent_id_from_path(&dir),
            None => self.bundle_by_path(".").map(|bundle| self.bundle_id(&bundle)).unwrap_or_default(),
        };
        let kind = self.derive_folder_kind(&normalized);
        if kind == FolderKind::Root {
            if let Some(bundle) = &bundle {
                return self.bundle_id(bundle);
            }
        }
        format!("{parent_id}{}{}", folder_kind_emoji(kind), flat(&name))
    }

    /// 📄️ Builds the emoji id of a file.
    pub fn build_file_id(&self, path: &str) -> String {
        let normalized = normalize_path(path);
        let name = base_of(&normalized);
        let kind = derive_file_kind(&name);
        let dir = dir_of(&normalized);
        let bundle = self.bundle_by_path(&normalized);
        let parent_id = if dir != "." && !dir.is_empty() {
            match &bundle {
                Some(bundle) if normalize_path(&bundle.root) == normalize_path(&dir) => self.bundle_id(bundle),
                _ => self.resolve_parent_id_from_path(&dir),
            }
        } else {
            bundle.as_ref().map(|bundle| self.bundle_id(bundle)).unwrap_or_default()
        };
        let extension = ext_of(&name);
        let stem = name.strip_suffix(&extension).unwrap_or(&name);
        format!("{parent_id}{}{}", file_kind_emoji(kind), flat(stem))
    }

    /// 🔗️ The `repo://file/…` uri of a path.
    pub fn file_uri(&self, path: &str) -> String {
        let id = self.build_file_id(path);
        if id.is_empty() {
            return String::new();
        }
        format!("repo://file/{id}")
    }

    /// 🔗️ The uri a folder is addressed by. The Go twin routes folders through the same builder.
    pub fn folder_uri(&self, path: &str) -> String {
        self.file_uri(&normalize_path(path))
    }

    //#endregion 🪪️Artifact Ids
}

//#endregion 🗂️Codebase

//#region 🪪️Section Ids

/// 🔖️ Reexported: the section, definition and file-kind derivations live in `📐️model`, next to the
/// `GetArtifactID` grammar they feed, exactly as the Go twin has them.
pub use semio_framework_repo_model::{build_definition_id, build_section_id, derive_file_kind, is_test_function_name};

//#endregion 🪪️Section Ids

//#region 🧭️Scope

/// 🧭️ What subtree of the repository a walk covers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Scope {
    /// 🏠️ The whole repository.
    Repo,
    /// 🏗️ One bundle named by its full bundle name.
    Technology(String),
    /// 📁️ One folder prefix.
    Folder(String),
    /// 📄️ One single file.
    File(String),
}

//#endregion 🧭️Scope

//#region 🔧️Helpers

/// 📂️ The entries of a directory in Go's lexical `os.ReadDir` order.
fn read_dir_names(dir: &Path, directories_only: bool) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut names: Vec<String> = entries
        .flatten()
        .filter(|entry| !directories_only || entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    names
}

/// 📦️ Splits a bundle name into its technology and bundle code.
fn split_bundle_name(name: &str) -> (String, String) {
    match name.split_once('/') {
        Some((technology, bundle)) => (technology.to_string(), bundle.to_string()),
        None => (name.to_string(), name.to_string()),
    }
}

/// 🙈️ Reports whether one ignore pattern excludes a path.
pub fn matches_ignore_pattern(path: &str, is_dir: bool, pattern: &str) -> bool {
    let pattern = normalize_path(pattern.trim());
    if pattern.is_empty() {
        return false;
    }
    let path = normalize_path(path.trim());
    if path.is_empty() {
        return false;
    }
    let mut candidates = vec![path.clone()];
    if is_dir && !path.ends_with('/') {
        candidates.push(format!("{path}/"));
    }
    let mut patterns = vec![pattern.clone()];
    if let Some(base) = pattern.strip_suffix("/**") {
        if !base.is_empty() {
            patterns.push(base.to_string());
            patterns.push(format!("{base}/"));
        }
    }
    for candidate_pattern in &patterns {
        if candidate_pattern.is_empty() {
            continue;
        }
        for candidate_path in &candidates {
            if glob_match(candidate_pattern, candidate_path).unwrap_or(false) {
                return true;
            }
        }
    }
    false
}

/// 🔢️ The total number of sections including every nested child.
pub fn count_sections(sections: &[Section]) -> i64 {
    sections.iter().fold(sections.len() as i64, |total, section| total + count_sections(&section.children))
}

/// 🧲️ The file path half of a breach scope.
pub fn extract_file_path(scope: &str) -> String {
    scope.split('#').next().unwrap_or_default().split('§').next().unwrap_or_default().to_string()
}

/// 🔖️ The section path a definition's line range falls inside.
pub fn find_section_for_definition(sections: &[Section], start_line: i64, end_line: i64, prefix: &str) -> String {
    for section in sections {
        if start_line >= section.start_line && end_line <= section.end_line {
            let path = if prefix.is_empty() { section.name.clone() } else { format!("{prefix}#{}", section.name) };
            if !section.children.is_empty() {
                let child = find_section_for_definition(&section.children, start_line, end_line, &path);
                if !child.is_empty() {
                    return child;
                }
            }
            return path;
        }
    }
    prefix.to_string()
}

/// 📖️ Every definition of one file with every addressable field filled: the section that encloses
/// it, its kind emoji and the file it belongs to.
///
/// [`semio_framework_repo_languages::parse_definitions`] answers names, kinds and line ranges only;
/// which section a definition sits in is a property of the whole file, so it is resolved once here
/// and every caller — the `definition list` tool, the GraphQL aggregate, the export snapshot — reads
/// the same record.
pub fn file_definitions(content: &str, file_path: &str) -> Vec<Definition> {
    let parsed = parse_definitions(content, file_path);
    if parsed.is_empty() {
        return Vec::new();
    }
    let sections = parse_sections(content, file_path);
    parsed
        .into_iter()
        .map(|mut definition| {
            definition.file_path = file_path.to_string();
            definition.section_path = find_section_for_definition(&sections, definition.start_line, definition.end_line, "");
            definition.emoji = definition_kind_emoji(definition.kind);
            definition
        })
        .collect()
}

/// 🪨️ The identity one definition of a file is addressed by, built from the file, the section that
/// encloses it, its kind and its name.
pub fn definition_id(file_id: &str, definition: &Definition) -> String {
    build_definition_id(file_id, &semio_framework_repo_languages::normalize_section_path(&definition.section_path), &definition.name, definition.kind)
}

/// 🤝️ Parses a copyright header line into the contributor name and email it carries.
pub fn parse_contributor_identity(line: &str) -> Option<(String, String)> {
    let bytes: Vec<char> = line.chars().collect();
    for start in 0..bytes.len().saturating_sub(3) {
        if !bytes[start..start + 4].iter().all(char::is_ascii_digit) {
            continue;
        }
        if start > 0 && bytes[start - 1].is_ascii_digit() {
            continue;
        }
        let mut cursor = start + 4;
        if bytes.get(cursor).is_some_and(|value| value.is_ascii_digit()) {
            continue;
        }
        if !bytes.get(cursor).is_some_and(|value| value.is_whitespace()) {
            continue;
        }
        while bytes.get(cursor).is_some_and(|value| value.is_whitespace()) {
            cursor += 1;
        }
        let rest: String = bytes[cursor..].iter().collect();
        let Some(open) = rest.find('<') else { continue };
        let Some(close) = rest[open + 1..].find('>') else { continue };
        let email = rest[open + 1..open + 1 + close].trim().to_string();
        if email.is_empty() {
            continue;
        }
        let name = rest[..open].trim().to_string();
        if name.is_empty() {
            continue;
        }
        return Some((name, email));
    }
    None
}

//#endregion 🔧️Helpers

//#region 📚️Context

/// 💿️ Everything one codebase projection is built from.
pub struct CodebaseContext<'a> {
    /// 🗂️ The codebase the projection walks.
    pub codebase: &'a Codebase,
    /// 📦️ The bundles the projection groups by.
    pub bundles: Vec<Bundle>,
    /// 📄️ The repository-relative source files the projection covers.
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

    /// ⚠️ Adopts breaches the statutes module produced for these files.
    pub fn with_breachs(mut self, breachs: Vec<Breach>) -> Self {
        self.breachs = breachs;
        self
    }

    /// 📦️ The label of the bundle owning a file, or the repository fallback.
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
    fn read(&self, file: &str) -> Option<String> {
        std::fs::read_to_string(self.codebase.root_dir().join(file)).ok()
    }

    //#endregion 📚️Context

    //#region 🧱️Aggregates

    /// 🧱️ Projects every bundle with its folder, file, line, section and definition counts.
    pub fn build_bundles(&self) -> Vec<CodebaseBundle> {
        let mut file_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut line_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut section_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut definition_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut folder_sets: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut contributor_sets: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut breach_counts: BTreeMap<String, i64> = BTreeMap::new();

        for bundle in &self.bundles {
            let name = normalize_bundle_label(&bundle.name);
            folder_sets.entry(name.clone()).or_default();
            contributor_sets.entry(name).or_default();
        }
        folder_sets.entry("repo/repo".to_string()).or_default();
        contributor_sets.entry("repo/repo".to_string()).or_default();

        for file in &self.files {
            if file == "README.md" || file == "AGENTS.md" {
                continue;
            }
            let bundle_name = self.bundle_for_file(file);
            if bundle_name.is_empty() {
                continue;
            }
            *file_counts.entry(bundle_name.clone()).or_default() += 1;
            let folder = normalize_path(&dir_of(file));
            if folder != "." {
                folder_sets.entry(bundle_name.clone()).or_default().insert(folder);
            }
            let Some(content) = self.read(file) else { continue };
            *line_counts.entry(bundle_name.clone()).or_default() += content.matches('\n').count() as i64 + 1;
            let sections = parse_sections(&content, file);
            *section_counts.entry(bundle_name.clone()).or_default() += count_sections(&sections);
            if language_for_path(file).is_some_and(|language| language.supports_definitions()) {
                *definition_counts.entry(bundle_name.clone()).or_default() += parse_definitions(&content, file).len() as i64;
            }
            if let Some(header) = find_section(&sections, "Header") {
                let start = header.start_index.max(0) as usize;
                let end = (header.end_index.max(0) as usize).min(content.len());
                if start <= end && content.is_char_boundary(start) && content.is_char_boundary(end) {
                    for line in content[start..end].split('\n') {
                        if let Some((_, email)) = parse_contributor_identity(line) {
                            contributor_sets.entry(bundle_name.clone()).or_default().insert(email);
                        }
                    }
                }
            }
        }

        for breach in &self.breachs {
            let bundle_name = self.bundle_for_file(&breach.scope);
            if !bundle_name.is_empty() {
                *breach_counts.entry(bundle_name).or_default() += 1;
            }
        }

        folder_sets
            .keys()
            .map(|name| {
                let bundle_root = self.bundles.iter().find(|bundle| &normalize_bundle_label(&bundle.name) == name).map(|bundle| bundle.root.clone()).unwrap_or_default();
                CodebaseBundle {
                    id: name.clone(),
                    folder: bundle_root.clone(),
                    uri: self.codebase.file_uri(&bundle_root),
                    contributors: contributor_sets.get(name).map(|set| set.iter().cloned().collect()).unwrap_or_default(),
                    tickets: Vec::new(),
                    metrics: Some(BundleMetricsInternal {
                        folders: folder_sets.get(name).map(BTreeSet::len).unwrap_or_default() as i64,
                        files: file_counts.get(name).copied().unwrap_or_default(),
                        sections: section_counts.get(name).copied().unwrap_or_default(),
                        definitions: definition_counts.get(name).copied().unwrap_or_default(),
                        lines: line_counts.get(name).copied().unwrap_or_default(),
                        breachs: breach_counts.get(name).copied().unwrap_or_default(),
                    }),
                }
            })
            .collect()
    }

    /// 📦️ Projects every folder that holds at least one considered file.
    pub fn build_folders(&self) -> Vec<CodebaseFolder> {
        let mut folder_set: BTreeSet<String> = BTreeSet::new();
        let mut file_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut line_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut breach_counts: BTreeMap<String, i64> = BTreeMap::new();

        for file in &self.files {
            let folder = normalize_path(&dir_of(file));
            if folder == "." {
                continue;
            }
            folder_set.insert(folder.clone());
            *file_counts.entry(folder.clone()).or_default() += 1;
            if let Some(content) = self.read(file) {
                *line_counts.entry(folder).or_default() += content.matches('\n').count() as i64 + 1;
            }
        }
        for breach in &self.breachs {
            let file_path = extract_file_path(&breach.scope);
            if file_path.is_empty() {
                continue;
            }
            let folder = normalize_path(&dir_of(&file_path));
            if folder != "." {
                *breach_counts.entry(folder).or_default() += 1;
            }
        }

        let mut result: Vec<CodebaseFolder> = folder_set
            .iter()
            .map(|folder| CodebaseFolder {
                id: self.codebase.build_folder_id(folder),
                path: folder.clone(),
                uri: self.codebase.file_uri(folder),
                name: base_of(folder),
                parent_id: match dir_of(folder) {
                    parent if parent != "." && !parent.is_empty() => Some(self.codebase.build_folder_id(&parent)),
                    _ => None,
                },
                bundle_id: self.codebase.bundle_by_path(folder).map(|bundle| self.codebase.bundle_id(&bundle)),
                metrics: Some(FolderMetricsInternal {
                    files: file_counts.get(folder).copied().unwrap_or_default(),
                    lines: line_counts.get(folder).copied().unwrap_or_default(),
                    breachs: breach_counts.get(folder).copied().unwrap_or_default(),
                }),
            })
            .collect();
        result.sort_by(|left, right| left.path.cmp(&right.path));
        result
    }

    /// 🔸️ Projects every considered file with its metrics and breach references.
    pub fn build_files(&self) -> Vec<CodebaseFile> {
        let mut breachs_by_file: BTreeMap<String, Vec<&Breach>> = BTreeMap::new();
        for breach in &self.breachs {
            let file_path = extract_file_path(&breach.scope);
            if !file_path.is_empty() {
                breachs_by_file.entry(file_path).or_default().push(breach);
            }
        }
        let mut result: Vec<CodebaseFile> = self
            .files
            .iter()
            .map(|file| {
                let metrics = self.read(file).map(|content| {
                    let sections = parse_sections(&content, file);
                    let definitions = if language_for_path(file).is_some_and(|language| language.supports_definitions()) { parse_definitions(&content, file).len() as i64 } else { 0 };
                    FileMetricsInternal { sections: count_sections(&sections), definitions, lines: content.split('\n').count() as i64 }
                });
                CodebaseFile {
                    id: self.codebase.build_file_id(file),
                    path: file.clone(),
                    uri: self.codebase.file_uri(file),
                    folder_id: match dir_of(file) {
                        parent if parent != "." && !parent.is_empty() => Some(self.codebase.build_folder_id(&parent)),
                        _ => None,
                    },
                    bundle_id: self.codebase.bundle_by_path(file).map(|bundle| self.codebase.bundle_id(&bundle)),
                    metrics,
                    breachs: breachs_by_file
                        .get(file)
                        .map(|breachs| {
                            breachs
                                .iter()
                                .map(|breach| FileBreachRef {
                                    kind: breach.kind.clone(),
                                    priority: breach.lint_priority.unwrap_or(BreachPriority::Low),
                                    autofixable: breach.lint_autofixable.unwrap_or_default(),
                                    solution: breach.solution.clone(),
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                }
            })
            .collect();
        result.sort_by(|left, right| left.path.cmp(&right.path));
        result
    }

    /// 🔺️ Projects every section of every considered file, depth first.
    pub fn build_sections(&self) -> Vec<CodebaseSection> {
        let mut result = Vec::new();
        for file in &self.files {
            let Some(content) = self.read(file) else { continue };
            let sections = parse_sections(&content, file);
            let file_id = self.codebase.build_file_id(file);
            self.add_sections(&mut result, file, &file_id, &content, &sections, "");
        }
        result.sort_by(|left, right| left.path.cmp(&right.path));
        result
    }

    /// ➕️ The recursive half of [`CodebaseContext::build_sections`].
    fn add_sections(&self, result: &mut Vec<CodebaseSection>, file: &str, file_id: &str, content: &str, sections: &[Section], parent_path: &str) {
        for section in sections {
            let section_path = if parent_path.is_empty() { section.name.clone() } else { format!("{parent_path}#{}", section.name) };
            let start = section.start_index.max(0) as usize;
            let end = (section.end_index.max(0) as usize).min(content.len());
            let section_content = if start < content.len() && start <= end && content.is_char_boundary(start) && content.is_char_boundary(end) { &content[start..end] } else { "" };
            let definitions = if language_for_path(file).is_some_and(|language| language.supports_definitions()) { parse_definitions(section_content, file).len() as i64 } else { 0 };
            result.push(CodebaseSection {
                id: format!("{file_id}#{section_path}"),
                path: format!("{file}#{section_path}"),
                uri: format!("{}#{section_path}", self.codebase.file_uri(file)),
                metrics: Some(SectionMetricsInternal { definitions, lines: section.end_line - section.start_line + 1, breachs: 0 }),
            });
            self.add_sections(result, file, file_id, content, &section.children, &section_path);
        }
    }

    /// ▶️ Projects every definition of every considered file.
    pub fn build_definitions(&self) -> Vec<CodebaseDefinition> {
        let mut result = Vec::new();
        for file in &self.files {
            let Some(content) = self.read(file) else { continue };
            if !language_for_path(file).is_some_and(|language| language.supports_definitions()) {
                continue;
            }
            let definitions = parse_definitions(&content, file);
            let sections = parse_sections(&content, file);
            let file_id = self.codebase.build_file_id(file);
            for definition in &definitions {
                let section_path = find_section_for_definition(&sections, definition.start_line, definition.end_line, "");
                let id = if section_path.is_empty() { format!("{file_id}§{}", definition.name) } else { format!("{file_id}#{section_path}§{}", definition.name) };
                result.push(CodebaseDefinition {
                    id: id.clone(),
                    path: id,
                    uri: format!("{}§{}", self.codebase.file_uri(file), definition.name),
                    metrics: Some(DefinitionMetricsInternal { definitions: 0, lines: definition.end_line - definition.start_line + 1, breachs: 0 }),
                });
            }
        }
        result.sort_by(|left, right| left.path.cmp(&right.path));
        result
    }

    /// ⬜️ Projects every breach with the folder and file it points at.
    pub fn build_breachs(&self) -> Vec<CodebaseBreach> {
        self.breachs
            .iter()
            .enumerate()
            .map(|(index, breach)| {
                let file_path = extract_file_path(&breach.scope);
                let bundle_name = self.bundle_for_file(&file_path);
                let mut folders = Vec::new();
                let mut files = Vec::new();
                if !file_path.is_empty() {
                    let folder = normalize_path(&dir_of(&file_path));
                    if folder != "." {
                        let folder_id = if bundle_name.is_empty() { folder.clone() } else { format!("{bundle_name}/{folder}") };
                        folders.push(BreachFolder { id: folder_id, path: folder.clone(), uri: self.codebase.folder_uri(&folder) });
                    }
                    let file_id = if bundle_name.is_empty() { file_path.clone() } else { format!("{bundle_name}/{}", base_of(&file_path)) };
                    files.push(BreachFile {
                        id: file_id,
                        path: file_path.clone(),
                        uri: self.codebase.file_uri(&file_path),
                        range: Some(FileRange { start: RangePosition { line: breach.line, column: breach.column }, end: RangePosition { line: breach.line, column: breach.column } }),
                    });
                }
                CodebaseBreach {
                    id: format!("{}#|{bundle_name}|{file_path}#{index}", breach.kind),
                    folders,
                    files,
                    kind: breach.kind.clone(),
                    priority: breach.lint_priority.unwrap_or(BreachPriority::Low),
                    autofixable: breach.lint_autofixable.unwrap_or_default(),
                    reason: breach.reason.clone(),
                    solution: breach.solution.clone(),
                }
            })
            .collect()
    }

    /// 🌳️ Projects the containment tree of bundles, folders and files.
    pub fn build_tree(&self, bundles: &[CodebaseBundle], files: &[CodebaseFile]) -> BTreeMap<String, CbTreeNode> {
        let mut root = CbTreeNode { kind: CbTreeNodeKind::Repo, children: BTreeMap::new() };
        for bundle in bundles {
            root.children.insert(bundle.id.clone(), CbTreeNode { kind: CbTreeNodeKind::Bundle, children: BTreeMap::new() });
        }
        let mut folder_paths: BTreeSet<String> = BTreeSet::new();
        for file in files {
            let bundle_name = self.bundle_for_file(&file.path);
            let folder = normalize_path(&dir_of(&file.path));
            if folder == "." {
                let parent = if root.children.contains_key(&bundle_name) { root.children.get_mut(&bundle_name).expect("bundle node") } else { &mut root };
                parent.children.insert(file.id.clone(), CbTreeNode { kind: CbTreeNodeKind::File, children: BTreeMap::new() });
                continue;
            }
            let parts: Vec<&str> = folder.split('/').collect();
            for index in 0..parts.len() {
                let folder_path = parts[..index + 1].join("/");
                if folder_paths.contains(&folder_path) {
                    continue;
                }
                folder_paths.insert(folder_path.clone());
                let branch = if root.children.contains_key(&bundle_name) { root.children.get_mut(&bundle_name).expect("bundle node") } else { &mut root };
                let node = descend(branch, &parts[..index]);
                node.children.insert(parts[index].to_string(), CbTreeNode { kind: CbTreeNodeKind::Folder, children: BTreeMap::new() });
            }
            let branch = if root.children.contains_key(&bundle_name) { root.children.get_mut(&bundle_name).expect("bundle node") } else { &mut root };
            let node = descend(branch, &parts);
            node.children.insert(file.id.clone(), CbTreeNode { kind: CbTreeNodeKind::File, children: BTreeMap::new() });
        }
        BTreeMap::from([("compose".to_string(), root)])
    }

    /// 🟥️ Assembles the whole codebase snapshot from this context.
    pub fn build(&self) -> CodebaseSnapshot {
        let bundles = self.build_bundles();
        let files = self.build_files();
        let tree = self.build_tree(&bundles, &files);
        CodebaseSnapshot {
            bundles: Some(bundles),
            folders: Some(self.build_folders()),
            files: Some(files),
            sections: Some(self.build_sections()),
            definitions: Some(self.build_definitions()),
            contributors: None,
            tickets: None,
            policies: None,
            breachs: Some(self.build_breachs()),
            tree: Some(tree),
        }
    }

    //#endregion 🧱️Aggregates
}

/// 🌳️ Walks a chain of folder segments, creating nothing.
fn descend<'node>(node: &'node mut CbTreeNode, parts: &[&str]) -> &'node mut CbTreeNode {
    let mut current = node;
    for part in parts {
        if !current.children.contains_key(*part) {
            current.children.insert((*part).to_string(), CbTreeNode { kind: CbTreeNodeKind::Folder, children: BTreeMap::new() });
        }
        current = current.children.get_mut(*part).expect("folder node");
    }
    current
}

/// 🔖️ The first section with a given name, searched depth first.
fn find_section<'a>(sections: &'a [Section], name: &str) -> Option<&'a Section> {
    for section in sections {
        if section.name == name {
            return Some(section);
        }
        if let Some(found) = find_section(&section.children, name) {
            return Some(found);
        }
    }
    None
}

//#endregion 📚️Context

//#region 🔁️Reexports

pub use semio_framework_repo_model::{Bundle as ModelBundle, DefinitionKind as ModelDefinitionKind, FileKind as ModelFileKind, FolderKind as ModelFolderKind};

/// 🏷️ Reexported so a caller never has to reach into `model` for the derivation this crate keys on.
pub use semio_framework_repo_model::derive_definition_kind as definition_kind_of;

//#endregion 🔁️Reexports
