//! 🗂️ Language table: the embedded language table and its lookups.
//!
//! Holds the `🔖️Table` region of the crate root; the root re-exports its items.

use super::*;

// #region 🔖️Table

/// 🔤️ One token of the declarative pattern language.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "t", rename_all = "camelCase")]
pub enum Token {
    Ws,
    Ws1,
    WsChar,
    End,
    WordEnd,
    LineStart,
    Ident,
    IdentUpper,
    TypeIdentUpper,
    Word,
    RestTrim,
    RestTrimNonSpace,
    Lit { v: String },
    AnyOf { v: Vec<String> },
    CharIn { v: String },
    IdentScoped { sep: String },
    Until { stop: String, #[serde(default)] min: usize },
    Opt { of: Vec<Token> },
    Alt { of: Vec<Vec<Token>> },
    Rep { of: Vec<Token>, #[serde(default)] min: usize },
    Cap { name: String, of: Box<Token> },
    RepChar { v: String, min: usize, max: usize, name: String },
}

/// 🧵️ An anchored line pattern.
#[derive(Debug, Clone, Deserialize)]
pub struct Pattern {
    #[serde(default)]
    pub ci: bool,
    pub tokens: Vec<Token>,
}

/// 🧭️ Which section reader a language uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SectionEngine {
    Markers,
    MarkdownHeadings,
    JsonKeys,
    None,
}

/// 📏️ How a matched definition line is given an end line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DefinitionEngine {
    Braces,
    Indent,
    RubyEnd,
}

/// 🧷️ Extra single-line definitions a language contributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrphanDefinitions {
    None,
    GoPackageAndImports,
    RustModDeclarations,
    RubyModules,
}

/// 🏷️ How a section name becomes the identifier a formatter writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SectionNaming {
    Verbatim,
    ModName,
}

/// 🗣️ One language of the table.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub name: String,
    pub emoji: String,
    pub extensions: Vec<String>,
    pub section_engine: SectionEngine,
    pub section_start: Option<Pattern>,
    pub section_end: Option<Pattern>,
    pub policy_section_start: Option<Pattern>,
    pub policy_section_end: Option<Pattern>,
    pub definition: Option<Pattern>,
    #[serde(default)]
    pub definition_engine: Option<DefinitionEngine>,
    #[serde(default = "orphan_none")]
    pub orphan_definitions: OrphanDefinitions,
    #[serde(default)]
    pub aux_patterns: BTreeMap<String, Pattern>,
    #[serde(default)]
    pub comment_prefix: String,
    #[serde(default)]
    pub block_comment_start: String,
    #[serde(default)]
    pub block_comment_end: String,
    #[serde(default)]
    pub section_start_format: String,
    #[serde(default)]
    pub section_end_format: String,
    #[serde(default)]
    pub section_both_format: String,
    pub supports_headers: bool,
    pub uses_indent_scoping: bool,
    pub supports_definitions_override: Option<bool>,
    pub supports_comments_override: Option<bool>,
    #[serde(default)]
    pub skip_directives: Vec<String>,
    #[serde(default)]
    pub string_features: Vec<String>,
    #[serde(default = "naming_verbatim")]
    pub section_naming: SectionNaming,
}

fn orphan_none() -> OrphanDefinitions {
    OrphanDefinitions::None
}

fn naming_verbatim() -> SectionNaming {
    SectionNaming::Verbatim
}

/// 🔑️ The vocabulary that turns a matched definition line into a raw kind.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionKeywords {
    pub modifiers: Vec<String>,
    pub keywords: Vec<String>,
    pub multi_word: Vec<String>,
    pub fallback: String,
}

/// ✨️ The callable-initialiser promotions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Refinements {
    pub arrow_function: Pattern,
    pub function_expression: Pattern,
    pub class_expression: Pattern,
    pub promotes: Vec<String>,
    pub promoted_to: String,
}

/// 💬️ The comment-prefix-agnostic region marker reader used by the scope builder.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeRegionMarker {
    pub strip_prefixes: Vec<String>,
    pub strip_suffixes: Vec<String>,
    pub start_keyword: String,
    pub end_keyword: String,
}

/// 🏓️ The whole language table.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageTable {
    pub schema_version: u32,
    pub registry: Vec<String>,
    pub definition_keywords: DefinitionKeywords,
    pub definition_kind_map: BTreeMap<String, String>,
    pub refinements: Refinements,
    pub scope_region_marker: ScopeRegionMarker,
    pub scope_definition_patterns: BTreeMap<String, Vec<Pattern>>,
    pub languages: Vec<Language>,
}

const TABLE_JSON: &str = include_str!("../🧬️schema/🔣️languages.json");

static TABLE: OnceLock<LanguageTable> = OnceLock::new();

/// 📊️ The parsed language table. Panics only when the embedded table is malformed, which the
/// crate's own contract test rules out.
pub fn table() -> &'static LanguageTable {
    TABLE.get_or_init(|| serde_json::from_str(TABLE_JSON).expect("embedded language table is valid"))
}

/// 📖️ The language registered under `name`.
pub fn language_by_name(name: &str) -> Option<&'static Language> {
    let t = table();
    t.registry
        .iter()
        .find(|n| n.as_str() == name)
        .and_then(|n| t.languages.iter().find(|l| &l.name == n))
}

/// 🏪️ The language claiming a path's extension, honouring registry order.
pub fn language_for_path(path: &str) -> Option<&'static Language> {
    let ext = path_extension(path).to_lowercase();
    if ext.is_empty() {
        return None;
    }
    let t = table();
    for name in &t.registry {
        if let Some(lang) = t.languages.iter().find(|l| &l.name == name) {
            if lang.extensions.contains(&ext) {
                return Some(lang);
            }
        }
    }
    None
}

/// 🧩️ The language claiming an extension, including entries outside the registry.
pub fn language_for_extension_unregistered(ext: &str) -> Option<&'static Language> {
    let ext = ext.to_lowercase();
    table().languages.iter().find(|l| l.extensions.contains(&ext))
}

pub(super) fn path_extension(path: &str) -> String {
    let base = path.rsplit(['/', '\\']).next().unwrap_or(path);
    match base.rfind('.') {
        Some(i) if i > 0 || base.len() > 1 => base[i..].to_string(),
        _ => String::new(),
    }
}

impl Language {
    /// 📑️ Whether the language produces sections.
    pub fn supports_sections(&self) -> bool {
        match self.section_engine {
            SectionEngine::None => false,
            SectionEngine::Markers => self.section_start.is_some(),
            _ => true,
        }
    }

    /// 🔸️ Whether the language produces definitions.
    pub fn supports_definitions(&self) -> bool {
        self.supports_definitions_override.unwrap_or_else(|| self.definition.is_some())
    }

    /// 🔺️ Whether the language has line comments.
    pub fn supports_comments(&self) -> bool {
        self.supports_comments_override.unwrap_or(!self.comment_prefix.is_empty())
    }

    /// 🎯️ Whether a lowercased extension belongs to this language.
    pub fn matches_extension(&self, ext: &str) -> bool {
        let ext = ext.to_lowercase();
        self.extensions.contains(&ext)
    }

    /// 🟨️ The built-in skip directives plus the language's own.
    pub fn skip_directives_all(&self) -> Vec<String> {
        let mut out = vec!["TODO".to_string(), "compose-ignore-".to_string()];
        out.extend(self.skip_directives.iter().cloned());
        out
    }
}

// #endregion 🔖️Table
