//! 🏷️ The kind and status enumerations, the kind derivations, their emojis and the section and definition ids.

use super::*;

//#region 🏷️Enumerations

/// 📖️ What a definition is, derived from the raw keyword a language plugin reports.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum DefinitionKind {
    #[serde(rename = "implementation")]
    #[default]
    Implementation,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "constant")]
    Constant,
    #[serde(rename = "test")]
    Test,
}

impl DefinitionKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            DefinitionKind::Implementation => "implementation",
            DefinitionKind::Interface => "interface",
            DefinitionKind::Constant => "constant",
            DefinitionKind::Test => "test",
        }
    }
}

impl fmt::Display for DefinitionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 📝️ Infers the definition kind from a language's raw declaration keyword. Anything
/// unrecognised is an implementation, so the function is total.
pub fn derive_definition_kind(raw_kind: &str) -> DefinitionKind {
    match go_to_lower(raw_kind).as_str() {
        "interface" | "type" | "trait" | "abstract" | "extend type" | "extend interface"
        | "extend enum" | "extend union" | "extend input" | "input" | "union" | "scalar"
        | "delegate" | "record" => DefinitionKind::Interface,
        "const" | "constant" | "enum" | "var" | "let" | "static" => DefinitionKind::Constant,
        "test" => DefinitionKind::Test,
        _ => DefinitionKind::Implementation,
    }
}

/// 🎫️ Lifecycle state of a ticket.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TicketStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

impl TicketStatus {
    /// 📡️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            TicketStatus::Open => "open",
            TicketStatus::Closed => "closed",
        }
    }
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🎯️ Lifecycle state of a goal.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GoalStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

impl GoalStatus {
    /// 🌐️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            GoalStatus::Open => "open",
            GoalStatus::Closed => "closed",
        }
    }
}

impl fmt::Display for GoalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🔸️ How urgently a breach must be dealt with.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum BreachPriority {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    #[default]
    Low,
}

impl BreachPriority {
    /// 🔣️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            BreachPriority::High => "high",
            BreachPriority::Medium => "medium",
            BreachPriority::Low => "low",
        }
    }
}

impl fmt::Display for BreachPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🛠️ Which family a technology belongs to. The wire value is the technology emoji itself.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TechnologyKind {
    #[serde(rename = "👤️")]
    User,
    #[serde(rename = "🧰️")]
    Infrastructure,
    #[serde(rename = "🔬️")]
    Research,
    #[serde(rename = "🌱️")]
    Mono,
}

impl TechnologyKind {
    /// 🧾️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            TechnologyKind::User => "👤️",
            TechnologyKind::Infrastructure => "🧰️",
            TechnologyKind::Research => "🔬️",
            TechnologyKind::Mono => "🌱️",
        }
    }
}

impl fmt::Display for TechnologyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 💡️ Infers the technology kind from a technology name.
pub fn derive_technology_kind(name: &str) -> TechnologyKind {
    match name {
        "compose" => TechnologyKind::User,
        "repo" => TechnologyKind::Infrastructure,
        "coda" => TechnologyKind::Research,
        _ => match name.strip_prefix('@') {
            Some(rest) => derive_technology_kind(rest),
            None => TechnologyKind::User,
        },
    }
}

/// 📦️ What a bundle produces.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum BundleKind {
    #[serde(rename = "library")]
    #[default]
    Library,
    #[serde(rename = "schema")]
    Schema,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "ui")]
    Ui,
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "assets")]
    Assets,
    #[serde(rename = "repo")]
    Repo,
}

impl BundleKind {
    /// 🔗️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            BundleKind::Library => "library",
            BundleKind::Schema => "schema",
            BundleKind::Binary => "binary",
            BundleKind::Ui => "ui",
            BundleKind::Site => "site",
            BundleKind::Assets => "assets",
            BundleKind::Repo => "repo",
        }
    }
}

impl fmt::Display for BundleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 📁️ Whether a folder is required by a toolchain or purely organisational.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FolderKind {
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "root")]
    Root,
}

impl FolderKind {
    /// 🪢️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            FolderKind::Organization => "organization",
            FolderKind::Required => "required",
            FolderKind::Root => "root",
        }
    }
}

impl fmt::Display for FolderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 📄️ What role a file plays. The Go twin keeps these as bare string constants.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileKind {
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "config")]
    Config,
    #[serde(rename = "lab")]
    Lab,
    #[serde(rename = "docs")]
    Docs,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "template")]
    Template,
    #[serde(rename = "license")]
    License,
}

impl FileKind {
    /// 🧬️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            FileKind::Code => "code",
            FileKind::Script => "script",
            FileKind::Config => "config",
            FileKind::Lab => "lab",
            FileKind::Docs => "docs",
            FileKind::Resource => "resource",
            FileKind::Template => "template",
            FileKind::License => "license",
        }
    }
}

impl fmt::Display for FileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// ♻️ How an entity changed between two checkpoints.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemanticChangeType {
    #[serde(rename = "added")]
    Added,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "renamed")]
    Renamed,
}

impl SemanticChangeType {
    /// 🏺️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            SemanticChangeType::Added => "added",
            SemanticChangeType::Deleted => "deleted",
            SemanticChangeType::Modified => "modified",
            SemanticChangeType::Renamed => "renamed",
        }
    }
}

impl fmt::Display for SemanticChangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

//#endregion 🏷️Enumerations

//#region 🏷️File Kinds

/// 🗂️ Infers the role a file plays from its name alone.
pub fn derive_file_kind(name: &str) -> FileKind {
    let ext = ext_of(name).to_lowercase();
    let name_lower = name.to_lowercase();
    let name_no_ext = name_lower.strip_suffix(&ext).unwrap_or(&name_lower).to_string();

    if name_lower.contains("license") || name_lower.contains("licence") {
        return FileKind::License;
    }
    for suffix in [".test", ".spec", ".tests", ".requirements", "_test", "_tests", "_spec", "_benchmark", ".benchmark", ".stories", ".story"] {
        if name_no_ext.ends_with(suffix) {
            return FileKind::Lab;
        }
    }
    if name_lower.starts_with("test_") || name_lower.starts_with("test.") || name_lower == "conftest.py" {
        return FileKind::Lab;
    }
    for suffix in [".config", ".conf", ".rc", ".cfg"] {
        if name_no_ext.ends_with(suffix) {
            return FileKind::Config;
        }
    }
    const SCRIPT: &[&str] = &[".sh", ".bash", ".zsh", ".fish", ".bat", ".cmd", ".ps1", ".psm1"];
    const CONFIG: &[&str] = &[
        ".json", ".yaml", ".yml", ".toml", ".xml", ".ini", ".conf", ".config", ".env", ".properties", ".cfg", ".hcl", ".tf", ".tfvars", ".gitignore", ".gitattributes", ".gitmodules",
        ".dockerignore", ".editorconfig", ".eslintignore", ".prettierignore", ".npmignore", ".browserslistrc",
    ];
    const DOCS: &[&str] = &[".md", ".txt", ".rst", ".adoc", ".asciidoc", ".org", ".rdoc"];
    const TEMPLATE: &[&str] = &[
        ".tpl", ".tmpl", ".gotmpl", ".mustache", ".hbs", ".handlebars", ".ejs", ".njk", ".nunjucks", ".jinja", ".jinja2", ".j2", ".liquid", ".pug", ".jade", ".slim", ".haml",
    ];
    const RESOURCE: &[&str] = &[
        ".png", ".jpg", ".jpeg", ".gif", ".svg", ".ico", ".webp", ".bmp", ".tiff", ".tif", ".ttf", ".woff", ".woff2", ".eot", ".otf", ".mp3", ".mp4", ".wav", ".ogg", ".webm", ".avi",
        ".mov", ".pdf", ".zip", ".tar", ".gz", ".bz2", ".xz", ".7z", ".rar", ".bin", ".dat", ".db", ".sqlite", ".sqlite3", ".wasm", ".map",
    ];
    const CODE: &[&str] = &[
        ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".mts", ".cts", ".py", ".pyi", ".pyw", ".cs", ".csx", ".go", ".c", ".cpp", ".cc", ".cxx", ".h", ".hpp", ".hxx", ".rs", ".java",
        ".kt", ".kts", ".scala", ".swift", ".m", ".mm", ".rb", ".erb", ".php", ".lua", ".r", ".jl", ".ex", ".exs", ".hs", ".lhs", ".ml", ".mli", ".clj", ".cljs", ".cljc", ".dart",
        ".v", ".sv", ".vhd", ".vhdl", ".zig", ".nim", ".cr", ".d", ".f", ".f90", ".f95", ".f03", ".pl", ".pm", ".css", ".scss", ".less", ".sass", ".styl", ".html", ".htm", ".vue",
        ".svelte", ".astro", ".sql", ".graphql", ".gql", ".proto", ".thrift", ".avsc", ".g4",
    ];
    if SCRIPT.contains(&ext.as_str()) {
        return FileKind::Script;
    }
    if CONFIG.contains(&ext.as_str()) {
        return FileKind::Config;
    }
    if DOCS.contains(&ext.as_str()) {
        return FileKind::Docs;
    }
    if TEMPLATE.contains(&ext.as_str()) {
        return FileKind::Template;
    }
    if RESOURCE.contains(&ext.as_str()) {
        return FileKind::Resource;
    }
    if CODE.contains(&ext.as_str()) {
        return FileKind::Code;
    }
    const NAMED_CONFIG: &[&str] = &[
        "makefile", "justfile", "rakefile", "gemfile", "vagrantfile", "procfile", "brewfile", "cmakelists.txt", ".babelrc", ".eslintrc", ".prettierrc", ".stylelintrc", ".nycrc",
        ".npmrc", ".nvmrc", ".yarnrc", ".ruby-version", ".python-version", ".node-version", ".tool-versions", ".cursorrules", ".clinerules", "nx.json", "tsconfig.json", "tslint.json",
        "composer.json", "cargo.lock", "package-lock.json", "yarn.lock", "pnpm-lock.yaml", "go.sum", "uv.lock",
    ];
    if name_lower.starts_with("dockerfile") || NAMED_CONFIG.contains(&name_lower.as_str()) {
        return FileKind::Config;
    }
    FileKind::Resource
}

//#endregion 🏷️File Kinds

//#region 🔣️Kind Emojis

/// 📂️ The emoji a folder kind renders as.
pub fn folder_kind_emoji(kind: FolderKind) -> String {
    match kind {
        FolderKind::Organization => emoji_text(entity_emoji("folder-organization")),
        FolderKind::Root => emoji_text(entity_emoji("folder-root")),
        FolderKind::Required => emoji_text(entity_emoji("folder-required")),
    }
}

/// 🖼️ The emoji a file kind renders as.
pub fn file_kind_emoji(kind: FileKind) -> String {
    emoji_text(entity_emoji(match kind {
        FileKind::Code => "file-code",
        FileKind::Lab => "file-lab",
        FileKind::Script => "file-script",
        FileKind::Docs => "file-docs",
        FileKind::Config => "file-config",
        FileKind::Resource => "file-resource",
        FileKind::Template => "file-template",
        FileKind::License => "file-license",
    }))
}

/// 🖌️ The emoji a definition kind renders as.
pub fn definition_kind_emoji(kind: DefinitionKind) -> String {
    emoji_text(entity_emoji(match kind {
        DefinitionKind::Interface => "definition-interface",
        DefinitionKind::Constant => "definition-constant",
        DefinitionKind::Test => "definition-test",
        DefinitionKind::Implementation => "definition-impl",
    }))
}

/// 🎁️ The emoji a bundle kind renders as, empty for a kind with no vocabulary entry.
pub fn bundle_kind_emoji(bundle: &Bundle) -> String {
    if !bundle.emoji.is_empty() {
        return emoji_text(&bundle.emoji);
    }
    emoji_text(entity_emoji(match bundle.kind {
        BundleKind::Schema => "bundle-schema",
        BundleKind::Binary => "bundle-binary",
        BundleKind::Ui => "bundle-ui",
        BundleKind::Site => "bundle-site",
        BundleKind::Assets => "bundle-assets",
        BundleKind::Library => "bundle-library",
        BundleKind::Repo => "bundle-repo",
    }))
}

/// 🏗️ The emoji a technology renders as, falling back to its kind.
pub fn technology_kind_emoji(technology: &Technology) -> String {
    if !technology.emoji.is_empty() {
        return emoji_text(&technology.emoji);
    }
    emoji_text(match technology.kind {
        TechnologyKind::Infrastructure => entity_emoji("technology-infrastructure"),
        TechnologyKind::Research => entity_emoji("technology-research"),
        TechnologyKind::Mono => entity_emoji("technology-mono"),
        TechnologyKind::User => entity_emoji("technology-user"),
    })
}

//#endregion 🔣️Kind Emojis

//#region 🪪️Section Ids

/// 🔖️ Builds the emoji id of a section inside a file.
pub fn build_section_id(file_id: &str, section_path: &[String]) -> String {
    let segments: Vec<&String> = section_path.iter().filter(|segment| !segment.is_empty()).collect();
    if segments.is_empty() {
        return file_id.to_string();
    }
    let mut result = file_id.to_string();
    for segment in segments {
        let (emoji, name) = extract_entity_emoji(segment);
        let emoji = if emoji.is_empty() { entity_emoji("section").to_string() } else { emoji };
        result = format!("{result}{}{}", emoji_text(&emoji), flat(&name));
    }
    result
}

/// 🧪️ Reports whether a bare declaration name denotes a test by convention.
pub fn is_test_function_name(name: &str) -> bool {
    for prefix in ["Test", "Benchmark", "Fuzz"] {
        if let Some(rest) = name.strip_prefix(prefix) {
            if rest.chars().next().is_some_and(|value| value.is_ascii_uppercase()) {
                return true;
            }
        }
    }
    name.starts_with("test_")
}

/// 🔨️ Builds the emoji id of a definition inside a section.
pub fn build_definition_id(file_id: &str, section_path: &[String], name: &str, kind: DefinitionKind) -> String {
    let mut effective = kind;
    if kind == DefinitionKind::Implementation && file_id.contains(&emoji_text(entity_emoji("file-lab"))) && is_test_function_name(name) {
        effective = DefinitionKind::Test;
    }
    format!("{}{}{}", build_section_id(file_id, section_path), definition_kind_emoji(effective), flat(name))
}

//#endregion 🪪️Section Ids
