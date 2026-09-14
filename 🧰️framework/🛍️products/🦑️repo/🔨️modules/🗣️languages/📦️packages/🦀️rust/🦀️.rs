//! 🗣️ Repo language table: section parsing, definition parsing, scope identifiers and file headers.
//!
//! Every parsing rule comes from `🧬️schema/🔣️languages.json`, embedded here and read at init by the
//! Go twin, so the two implementations cannot drift. Patterns are the declarative token language of
//! `🧬️schema/🔣️.json`, matched by the hand-rolled backtracking matcher below — this crate depends on
//! no regular-expression engine.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

// #region 🔖️IdentityPending

// 🚚️ `is_emoji_rune` / `extract_entity_emoji` belong to `🔨️modules/🪪️identity` per the taxonomy plan.
// They live here until that crate exists; re-export from identity and delete this region then.

/// 🎭️ Reports whether a scalar is an emoji base character, a joiner or a presentation selector.
pub fn is_emoji_rune(r: char) -> bool {
    let c = r as u32;
    const RANGES: &[(u32, u32)] = &[
        (0x1F600, 0x1F64F),
        (0x1F300, 0x1F5FF),
        (0x1F680, 0x1F6FF),
        (0x1F700, 0x1F77F),
        (0x1F780, 0x1F7FF),
        (0x1F800, 0x1F8FF),
        (0x1F900, 0x1F9FF),
        (0x1FA00, 0x1FA6F),
        (0x1FA70, 0x1FAFF),
        (0x2600, 0x26FF),
        (0x2700, 0x27BF),
        (0x2300, 0x23FF),
        (0x2B50, 0x2B55),
        (0x200D, 0x200D),
        (0xFE00, 0xFE0F),
        (0x2196, 0x2199),
        (0x21A9, 0x21AA),
        (0x231A, 0x231B),
        (0x25AA, 0x25AB),
        (0x25B6, 0x25C0),
        (0x25FB, 0x25FE),
        (0x2614, 0x2615),
        (0x2648, 0x2653),
        (0x267F, 0x267F),
        (0x2693, 0x2693),
        (0x26A1, 0x26A1),
        (0x26AA, 0x26AB),
        (0x26BD, 0x26BE),
        (0x26C4, 0x26C5),
        (0x26CE, 0x26CF),
        (0x26D4, 0x26D4),
        (0x26EA, 0x26EA),
        (0x26F2, 0x26F3),
        (0x26F5, 0x26F5),
        (0x26FA, 0x26FA),
        (0x26FD, 0x26FD),
    ];
    const SINGLES: &[u32] = &[0x2139, 0x2194, 0x2195, 0x203C, 0x2049, 0x20E3, 0x00A9, 0x00AE, 0x2122];
    RANGES.iter().any(|&(lo, hi)| c >= lo && c <= hi) || SINGLES.contains(&c)
}

/// 🧲️ Splits a leading emoji grapheme (with joiners, selectors and skin tones) from the rest.
pub fn extract_entity_emoji(s: &str) -> (String, String) {
    let runes: Vec<char> = s.chars().collect();
    if runes.is_empty() || !is_emoji_rune(runes[0]) {
        return (String::new(), s.to_string());
    }
    let mut i = 1usize;
    while i < runes.len() {
        let r = runes[i] as u32;
        if r == 0xFE0F || r == 0xFE0E || r == 0x20E3 {
            i += 1;
        } else if r == 0x200D {
            i += 1;
            if i < runes.len() {
                i += 1;
                while i < runes.len() && matches!(runes[i] as u32, 0xFE0F | 0xFE0E) {
                    i += 1;
                }
            }
        } else if (0x1F3FB..=0x1F3FF).contains(&r) {
            i += 1;
        } else {
            break;
        }
    }
    (runes[..i].iter().collect(), runes[i..].iter().collect())
}

// #endregion 🔖️IdentityPending

// #region 🔖️Records

// 📐️ `Section`, `Definition` and `DefinitionKind` are owned by `🔨️modules/📐️model` and re-exported
// here so a consumer of this crate never has to name the model crate to hold a parse result.
pub use semio_framework_repo_model::{Definition, DefinitionKind, Section};

/// 📐️ A definition before it is given a canonical kind and file identity. The raw keyword survives
/// here — `Definition` only carries the canonical kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionRange {
    pub name: String,
    pub kind: String,
    pub start: i64,
    pub end: i64,
    pub excerpt: String,
}

// #endregion 🔖️Records

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

/// 🗂️ The whole language table.
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

const TABLE_JSON: &str = include_str!("../../🧬️schema/🔣️languages.json");

static TABLE: OnceLock<LanguageTable> = OnceLock::new();

/// 🗂️ The parsed language table. Panics only when the embedded table is malformed, which the
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

fn path_extension(path: &str) -> String {
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

// #region 🔖️Matcher

const fn is_ws_byte(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C)
}

const fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// 🪪️ The captures a pattern match produced, in match order.
pub type Captures = Vec<(String, String)>;

enum Cont<'a> {
    Nil,
    Cons(&'a [Token], &'a Cont<'a>),
}

fn eq_at(line: &str, pos: usize, lit: &str, ci: bool) -> bool {
    let bytes = line.as_bytes();
    let lb = lit.as_bytes();
    if pos + lb.len() > bytes.len() {
        return false;
    }
    if ci {
        bytes[pos..pos + lb.len()].eq_ignore_ascii_case(lb)
    } else {
        &bytes[pos..pos + lb.len()] == lb
    }
}

fn run_len(line: &str, pos: usize, pred: impl Fn(u8) -> bool) -> usize {
    line.as_bytes()[pos..].iter().take_while(|b| pred(**b)).count()
}

fn scoped_ends(line: &str, pos: usize, sep: &str) -> Vec<usize> {
    let first = ident_len(line, pos);
    if first == 0 {
        return Vec::new();
    }
    let mut ends = vec![pos + first];
    let mut cursor = pos + first;
    loop {
        if !eq_at(line, cursor, sep, false) {
            break;
        }
        let next = cursor + sep.len();
        let n = ident_len(line, next);
        if n == 0 {
            break;
        }
        cursor = next + n;
        ends.push(cursor);
    }
    ends.reverse();
    ends
}

fn ident_len(line: &str, pos: usize) -> usize {
    let b = line.as_bytes();
    if pos >= b.len() || !(b[pos].is_ascii_alphabetic() || b[pos] == b'_') {
        return 0;
    }
    1 + run_len(line, pos + 1, is_word_byte)
}

fn upper_ident_len(line: &str, pos: usize, extra: &[u8]) -> usize {
    let b = line.as_bytes();
    if pos >= b.len() || !b[pos].is_ascii_uppercase() {
        return 0;
    }
    1 + run_len(line, pos + 1, |c| is_word_byte(c) || extra.contains(&c))
}

fn char_count_until(line: &str, pos: usize, stop: &str) -> usize {
    line[pos..].chars().take_while(|c| !stop.contains(*c)).count()
}

fn nth_char_offset(line: &str, pos: usize, n: usize) -> usize {
    line[pos..].char_indices().nth(n).map_or(line.len(), |(i, _)| pos + i)
}

/// 🔚️ The lazy `(.+?)\s*$` capture: the trailing-trimmed rest, or one character when all-blank.
fn rest_trim_capture(content: &str) -> Option<String> {
    if content.is_empty() {
        return None;
    }
    let trimmed = content.trim_end_matches(|c: char| is_ws_byte(c as u8) && c.is_ascii());
    if trimmed.is_empty() {
        content.chars().next().map(|c| c.to_string())
    } else {
        Some(trimmed.to_string())
    }
}

/// 🔢️ The end offsets a single token can reach from `pos`, in Go's preference order.
fn token_ends(tok: &Token, line: &str, pos: usize, ci: bool) -> Vec<usize> {
    let bytes = line.as_bytes();
    match tok {
        Token::Ws => {
            let n = run_len(line, pos, is_ws_byte);
            (0..=n).rev().map(|k| pos + k).collect()
        }
        Token::Ws1 => {
            let n = run_len(line, pos, is_ws_byte);
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::WsChar => {
            if pos < bytes.len() && is_ws_byte(bytes[pos]) {
                vec![pos + 1]
            } else {
                Vec::new()
            }
        }
        Token::End => {
            if pos == bytes.len() {
                vec![pos]
            } else {
                Vec::new()
            }
        }
        Token::WordEnd => {
            if pos >= bytes.len() || !is_word_byte(bytes[pos]) {
                vec![pos]
            } else {
                Vec::new()
            }
        }
        Token::LineStart => {
            if pos == 0 {
                vec![pos]
            } else {
                Vec::new()
            }
        }
        Token::Lit { v } => {
            if eq_at(line, pos, v, ci) {
                vec![pos + v.len()]
            } else {
                Vec::new()
            }
        }
        Token::AnyOf { v } => v.iter().filter(|s| eq_at(line, pos, s, ci)).map(|s| pos + s.len()).collect(),
        Token::CharIn { v } => line[pos..].chars().next().filter(|c| v.contains(*c)).map(|c| vec![pos + c.len_utf8()]).unwrap_or_default(),
        Token::Ident => {
            let n = ident_len(line, pos);
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::IdentUpper => {
            let n = upper_ident_len(line, pos, &[]);
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::TypeIdentUpper => {
            let n = upper_ident_len(line, pos, b"<>");
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::Word => {
            let n = run_len(line, pos, is_word_byte);
            (1..=n).rev().map(|k| pos + k).collect()
        }
        Token::IdentScoped { sep } => scoped_ends(line, pos, sep),
        Token::Until { stop, min } => {
            let n = char_count_until(line, pos, stop);
            if n < *min {
                return Vec::new();
            }
            (*min..=n).rev().map(|k| nth_char_offset(line, pos, k)).collect()
        }
        Token::RestTrim => {
            if pos < bytes.len() {
                vec![bytes.len()]
            } else {
                Vec::new()
            }
        }
        Token::RestTrimNonSpace => {
            if pos < bytes.len() && !is_ws_byte(bytes[pos]) {
                vec![bytes.len()]
            } else {
                Vec::new()
            }
        }
        Token::Opt { .. } | Token::Alt { .. } | Token::Rep { .. } | Token::Cap { .. } | Token::RepChar { .. } => Vec::new(),
    }
}

fn capture_text(tok: &Token, line: &str, pos: usize, end: usize) -> String {
    match tok {
        Token::RestTrim | Token::RestTrimNonSpace => rest_trim_capture(&line[pos..]).unwrap_or_default(),
        _ => line[pos..end].to_string(),
    }
}

fn run_tokens(toks: &[Token], k: &Cont<'_>, line: &str, pos: usize, ci: bool, caps: &mut Captures) -> Option<usize> {
    let Some((head, tail)) = toks.split_first() else {
        return match k {
            Cont::Nil => Some(pos),
            Cont::Cons(next, rest) => run_tokens(next, rest, line, pos, ci, caps),
        };
    };
    let saved = caps.len();
    match head {
        Token::Opt { of } => {
            let k2 = Cont::Cons(tail, k);
            if let Some(e) = run_tokens(of, &k2, line, pos, ci, caps) {
                return Some(e);
            }
            caps.truncate(saved);
            run_tokens(tail, k, line, pos, ci, caps)
        }
        Token::Alt { of } => {
            let k2 = Cont::Cons(tail, k);
            for branch in of {
                if let Some(e) = run_tokens(branch, &k2, line, pos, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
        Token::Rep { of, min } => {
            let mut stops = vec![pos];
            let mut cursor = pos;
            while let Some(next) = run_tokens(of, &Cont::Nil, line, cursor, ci, caps) {
                caps.truncate(saved);
                if next == cursor {
                    break;
                }
                cursor = next;
                stops.push(cursor);
            }
            caps.truncate(saved);
            for (count, stop) in stops.iter().enumerate().rev() {
                if count < *min {
                    break;
                }
                if let Some(e) = run_tokens(tail, k, line, *stop, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
        Token::Cap { name, of } => {
            for end in token_ends(of, line, pos, ci) {
                caps.push((name.clone(), capture_text(of, line, pos, end)));
                if let Some(e) = run_tokens(tail, k, line, end, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
        Token::RepChar { v, min, max, name } => {
            let c = v.chars().next()?;
            let run = line[pos..].chars().take_while(|x| *x == c).count();
            let hi = run.min(*max);
            for n in (*min..=hi).rev() {
                let end = pos + n * c.len_utf8();
                caps.push((name.clone(), line[pos..end].to_string()));
                if let Some(e) = run_tokens(tail, k, line, end, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
        simple => {
            for end in token_ends(simple, line, pos, ci) {
                if let Some(e) = run_tokens(tail, k, line, end, ci, caps) {
                    return Some(e);
                }
                caps.truncate(saved);
            }
            None
        }
    }
}

/// 🎯️ Matches a pattern anchored at the start of `line`, returning the end offset and captures.
pub fn pattern_match(p: &Pattern, line: &str) -> Option<(usize, Captures)> {
    let mut caps = Captures::new();
    run_tokens(&p.tokens, &Cont::Nil, line, 0, p.ci, &mut caps).map(|end| (end, caps))
}

/// 🔍️ Whether a pattern matches anywhere in `line`.
pub fn pattern_find(p: &Pattern, line: &str) -> bool {
    (0..=line.len()).any(|i| line.is_char_boundary(i) && pattern_match_at(p, line, i).is_some())
}

fn pattern_match_at(p: &Pattern, line: &str, pos: usize) -> Option<usize> {
    let mut caps = Captures::new();
    run_tokens(&p.tokens, &Cont::Nil, line, pos, p.ci, &mut caps)
}

fn capture(caps: &Captures, name: &str) -> Option<String> {
    caps.iter().find(|(k, _)| k == name).map(|(_, v)| v.clone())
}

// #endregion 🔖️Matcher

// #region 🔖️Sections

/// 🧱️ A zeroed section, since the model crate does not derive Default.
fn empty_section() -> Section {
    Section {
        id: String::new(),
        name: String::new(),
        path: String::new(),
        file_path: String::new(),
        emoji: String::new(),
        start_line: 0,
        end_line: 0,
        start_index: 0,
        end_index: 0,
        children: Vec::new(),
        definitions: Vec::new(),
    }
}

fn split_lines(content: &str) -> Vec<&str> {
    content.split('\n').collect()
}

fn parse_marker_sections(lang: &Language, content: &str) -> Vec<Section> {
    let Some(start_pat) = lang.section_start.as_ref() else {
        return Vec::new();
    };
    let lines = split_lines(content);
    let mut arena: Vec<Section> = Vec::new();
    let mut children: Vec<Vec<usize>> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut char_index = 0usize;
    for (i, line) in lines.iter().enumerate() {
        let line_start = char_index;
        let line_num = (i + 1) as i64;
        if let Some((_, caps)) = pattern_match(start_pat, line) {
            let raw = capture(&caps, "name").unwrap_or_default();
            let raw = raw.trim().to_string();
            let (emoji, rest) = extract_entity_emoji(&raw);
            let mut name = rest.trim().to_string();
            if name.is_empty() {
                name = raw.clone();
            }
            arena.push(Section {
                name,
                emoji,
                start_line: line_num,
                end_line: lines.len() as i64,
                start_index: line_start as i64,
                end_index: content.len() as i64,
                ..empty_section()
            });
            children.push(Vec::new());
            let idx = arena.len() - 1;
            match stack.last() {
                Some(&parent) => children[parent].push(idx),
                None => roots.push(idx),
            }
            stack.push(idx);
        } else if lang.section_end.as_ref().is_some_and(|p| pattern_match(p, line).is_some()) {
            if let Some(idx) = stack.pop() {
                arena[idx].end_line = line_num;
                arena[idx].end_index = (char_index + line.len()) as i64;
            }
        }
        char_index += line.len() + 1;
    }
    fn build(idx: usize, arena: &[Section], children: &[Vec<usize>]) -> Section {
        let mut s = arena[idx].clone();
        s.children = children[idx].iter().map(|c| build(*c, arena, children)).collect();
        s
    }
    roots.iter().map(|r| build(*r, &arena, &children)).collect()
}

/// 📰️ Markdown sections from ATX headings, with front-matter lines added to every line number.
pub fn parse_markdown_sections(content: &str) -> Vec<Section> {
    let lang = language_by_name("markdown").expect("markdown is registered");
    let head_pat = lang.section_start.as_ref().expect("markdown declares a heading pattern");
    let lines = split_lines(content);
    let mut frontmatter_lines = 0i64;
    if let Some(rest) = content.strip_prefix("---") {
        if let Some(end_index) = rest.find("---") {
            frontmatter_lines = content[..end_index + 6].matches('\n').count() as i64;
        }
    }
    let mut arena: Vec<Section> = Vec::new();
    let mut children: Vec<Vec<usize>> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut char_index = 0usize;
    for (i, line) in lines.iter().enumerate() {
        let line_start = char_index;
        if let Some((_, caps)) = pattern_match(head_pat, line) {
            let level = capture(&caps, "level").unwrap_or_default().len();
            let name = capture(&caps, "name").unwrap_or_default().trim().to_string();
            while stack.last().is_some_and(|(l, _)| *l >= level) {
                let (_, idx) = stack.pop().expect("stack is not empty");
                arena[idx].end_line = frontmatter_lines + i as i64;
                arena[idx].end_index = line_start as i64 - 1;
            }
            arena.push(Section {
                name,
                start_line: frontmatter_lines + i as i64 + 1,
                end_line: -1,
                start_index: line_start as i64,
                end_index: -1,
                ..empty_section()
            });
            children.push(Vec::new());
            let idx = arena.len() - 1;
            match stack.last() {
                Some(&(_, parent)) => children[parent].push(idx),
                None => roots.push(idx),
            }
            stack.push((level, idx));
        }
        char_index += line.len() + 1;
    }
    while let Some((_, idx)) = stack.pop() {
        arena[idx].end_line = frontmatter_lines + lines.len() as i64;
        arena[idx].end_index = content.len() as i64;
    }
    fn build(idx: usize, arena: &[Section], children: &[Vec<usize>]) -> Section {
        let mut s = arena[idx].clone();
        s.children = children[idx].iter().map(|c| build(*c, arena, children)).collect();
        s
    }
    roots.iter().map(|r| build(*r, &arena, &children)).collect()
}

/// 🔣️ JSON sections: one section per object key, spanning its key through its value.
///
/// The Go original keeps `*Section` pointers into slices it keeps appending to, so every key but the
/// last in an object is written through a stale backing array and comes back with `endLine = -1`.
/// This implementation gives every key its real range; the defect is recorded for the Go split.
pub fn parse_json_sections(content: &str) -> Vec<Section> {
    struct Frame {
        kind: u8,
        section: Option<usize>,
        path: String,
        expect_key: bool,
        location: Option<usize>,
    }
    let bytes = content.as_bytes();
    let mut arena: Vec<Section> = Vec::new();
    let mut children: Vec<Vec<usize>> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    let mut stack: Vec<Frame> = Vec::new();
    let mut line = 1i64;
    let mut in_string = false;
    let mut escape = false;
    let mut string_start = 0usize;
    let mut buf = String::new();
    let mut pending_key: Option<(String, usize, i64)> = None;
    let mut awaiting: Option<usize> = None;
    let mut awaiting_value_start: Option<usize> = None;
    let mut i = 0usize;
    while i < bytes.len() {
        let ch = bytes[i];
        if ch == b'\n' {
            line += 1;
        }
        if in_string {
            if escape {
                escape = false;
                buf.push(ch as char);
            }
            if ch == b'\\' {
                escape = true;
                buf.push(ch as char);
                i += 1;
                continue;
            }
            if ch == b'"' {
                in_string = false;
                let value = std::mem::take(&mut buf);
                let expecting = stack.last().is_some_and(|f| f.kind == b'{' && f.expect_key);
                if expecting && awaiting.is_none() {
                    pending_key = Some((value, string_start, line));
                    if let Some(f) = stack.last_mut() {
                        f.expect_key = false;
                    }
                } else if let Some(idx) = awaiting {
                    if awaiting_value_start == Some(string_start) {
                        arena[idx].end_line = line;
                        arena[idx].end_index = i as i64 + 1;
                        awaiting = None;
                        awaiting_value_start = None;
                    }
                }
                i += 1;
                continue;
            }
            buf.push_str(&content[i..i + 1]);
            i += 1;
            continue;
        }
        if ch == b'"' {
            if awaiting.is_some() {
                awaiting_value_start = Some(i);
            }
            in_string = true;
            string_start = i;
            i += 1;
            continue;
        }
        if ch == b':' && pending_key.is_some() && stack.last().is_some_and(|f| f.kind == b'{') {
            let (key, key_start, key_line) = pending_key.take().expect("pending key present");
            let top = stack.last().expect("object frame present");
            let path = if top.path.is_empty() { key.clone() } else { format!("{}/{}", top.path, key) };
            arena.push(Section {
                name: key,
                path: path.clone(),
                start_line: key_line,
                end_line: -1,
                start_index: key_start as i64,
                end_index: -1,
                ..empty_section()
            });
            children.push(Vec::new());
            let idx = arena.len() - 1;
            match top.section {
                Some(parent) => children[parent].push(idx),
                None => roots.push(idx),
            }
            awaiting = Some(idx);
            awaiting_value_start = None;
            i += 1;
            continue;
        }
        if let Some(idx) = awaiting {
            if ch == b'{' || ch == b'[' {
                let path = arena[idx].path.clone();
                stack.push(Frame { kind: ch, section: Some(idx), path, expect_key: ch == b'{', location: Some(idx) });
                awaiting = None;
                awaiting_value_start = None;
                i += 1;
                continue;
            }
            if ch == b'-' || ch.is_ascii_digit() || ch == b't' || ch == b'f' || ch == b'n' {
                let mut end = i;
                while end < bytes.len() {
                    let c = bytes[end];
                    if c == b'\n' {
                        line += 1;
                    }
                    if matches!(c, b',' | b'}' | b']' | b' ' | b'\t' | b'\r' | b'\n') {
                        break;
                    }
                    end += 1;
                }
                arena[idx].end_line = line;
                arena[idx].end_index = end as i64;
                awaiting = None;
                awaiting_value_start = None;
                i = end;
                continue;
            }
        }
        if ch == b'{' || ch == b'[' {
            stack.push(Frame { kind: ch, section: None, path: String::new(), expect_key: ch == b'{', location: None });
            i += 1;
            continue;
        }
        if ch == b'}' || ch == b']' {
            if let Some(top) = stack.pop() {
                if let Some(idx) = top.location {
                    arena[idx].end_line = line;
                    arena[idx].end_index = i as i64 + 1;
                }
                if let Some(f) = stack.last_mut() {
                    if f.kind == b'{' {
                        f.expect_key = true;
                    }
                }
            }
            i += 1;
            continue;
        }
        if ch == b',' {
            if let Some(f) = stack.last_mut() {
                if f.kind == b'{' {
                    f.expect_key = true;
                }
            }
            i += 1;
            continue;
        }
        i += 1;
    }
    for s in arena.iter_mut() {
        if s.end_index == -1 {
            s.end_line = line;
            s.end_index = content.len() as i64;
        }
    }
    fn build(idx: usize, arena: &[Section], children: &[Vec<usize>]) -> Section {
        let mut s = arena[idx].clone();
        s.children = children[idx].iter().map(|c| build(*c, arena, children)).collect();
        s
    }
    roots.iter().map(|r| build(*r, &arena, &children)).collect()
}

/// 📑️ Sections of `content` as read by the language registered under `language_name`.
pub fn parse_code_sections(content: &str, language_name: &str) -> Vec<Section> {
    let Some(lang) = language_by_name(language_name) else {
        return Vec::new();
    };
    if !lang.supports_sections() {
        return Vec::new();
    }
    parse_sections_with(lang, content)
}

/// 📩️ Sections of `content` as read by the language claiming `file_path`.
pub fn parse_sections(content: &str, file_path: &str) -> Vec<Section> {
    match language_for_path(file_path) {
        Some(lang) => parse_sections_with(lang, content),
        None => Vec::new(),
    }
}

/// 🧭️ Sections of `content` under an explicit language.
pub fn parse_sections_with(lang: &Language, content: &str) -> Vec<Section> {
    match lang.section_engine {
        SectionEngine::Markers => parse_marker_sections(lang, content),
        SectionEngine::MarkdownHeadings => parse_markdown_sections(content),
        SectionEngine::JsonKeys => parse_json_sections(content),
        SectionEngine::None => Vec::new(),
    }
}

/// 🛤️ Splits a section path on `#` and `/`, dropping empty segments.
pub fn normalize_section_path(section_path: &str) -> Vec<String> {
    section_path
        .replace('#', "/")
        .split('/')
        .filter(|p| !p.is_empty())
        .map(str::to_string)
        .collect()
}

/// 🎯️ Attaches every definition to the deepest section whose line range contains it.
pub fn hydrate_sections_with_definitions(sections: &[Section], definitions: &[Definition]) -> Vec<Section> {
    sections
        .iter()
        .map(|section| {
            let mut out = section.clone();
            let subset: Vec<Definition> = definitions
                .iter()
                .filter(|d| d.start_line >= out.start_line && d.end_line <= out.end_line)
                .cloned()
                .collect();
            out.children = hydrate_sections_with_definitions(&out.children, &subset);
            out.definitions = subset
                .iter()
                .filter(|d| !out.children.iter().any(|c| d.start_line >= c.start_line && d.end_line <= c.end_line))
                .cloned()
                .collect();
            out
        })
        .collect()
}

// #endregion 🔖️Sections

// #region 🔖️Definitions

/// 🗺️ The canonical kind of a raw definition keyword.
pub fn derive_definition_kind(raw_kind: &str) -> DefinitionKind {
    match table().definition_kind_map.get(&raw_kind.to_lowercase()).map(String::as_str) {
        Some("interface") => DefinitionKind::Interface,
        Some("constant") => DefinitionKind::Constant,
        Some("test") => DefinitionKind::Test,
        _ => DefinitionKind::Implementation,
    }
}

fn extract_definition_keyword(full_match: &str, name: &str) -> String {
    let kw = &table().definition_keywords;
    let lower = full_match.to_lowercase();
    for multi in &kw.multi_word {
        if lower.contains(&multi.to_lowercase()) {
            return multi.clone();
        }
    }
    let lower_name = name.to_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();
    let mut prev = String::new();
    for word in &words {
        let clean = word.trim_end_matches(['(', '<', '{', '[']);
        if clean == lower_name {
            break;
        }
        prev = clean.to_string();
    }
    if !prev.is_empty() && kw.keywords.contains(&prev) {
        return prev;
    }
    for word in &words {
        let clean = word.trim_end_matches(['(', '<', '{', '[']);
        if clean == lower_name || kw.modifiers.iter().any(|m| m == clean) {
            continue;
        }
        if kw.keywords.iter().any(|k| k == clean) {
            return clean.to_string();
        }
    }
    kw.fallback.clone()
}

fn refine_definition_kind(raw_kind: &str, line: &str) -> String {
    let r = &table().refinements;
    let lower = raw_kind.to_lowercase();
    if r.promotes.contains(&lower)
        && (pattern_find(&r.arrow_function, line) || pattern_find(&r.function_expression, line) || pattern_find(&r.class_expression, line))
    {
        return r.promoted_to.clone();
    }
    raw_kind.to_string()
}

fn indent_of(line: &str) -> usize {
    line.len() - line.trim_start_matches([' ', '\t']).len()
}

fn parse_definition_ranges_default(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange> {
    let Some(pat) = lang.definition.as_ref() else {
        return Vec::new();
    };
    let indent_scoping = lang.definition_engine == Some(DefinitionEngine::Indent);
    let mut starts: Vec<(String, String, usize)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if let Some((end, caps)) = pattern_match(pat, line) {
            let name = capture(&caps, "name").unwrap_or_default();
            if !name.is_empty() {
                let kind = extract_definition_keyword(&line[..end], &name);
                starts.push((name, kind, i + 1));
            }
        }
    }
    let mut out = Vec::new();
    for i in 0..starts.len() {
        let start = starts[i].2;
        let mut end = start;
        if indent_scoping {
            let start_indent = indent_of(lines[start - 1]);
            // 📏️The index is the answer here, not the element: `end` is a line NUMBER.
            #[allow(clippy::needless_range_loop)]
            for line_index in start..lines.len() {
                let line = lines[line_index].trim_end_matches('\r');
                if line.trim().is_empty() {
                    continue;
                }
                if indent_of(line) <= start_indent {
                    end = line_index;
                    break;
                }
                end = line_index + 1;
            }
        } else {
            let mut brace_depth = 0i64;
            let mut saw_open = false;
            let next_def_start = if i + 1 < starts.len() { starts[i + 1].2 - 1 } else { lines.len() };
            let mut line_index = start - 1;
            while line_index < lines.len() {
                if !saw_open && line_index >= next_def_start {
                    break;
                }
                let mut broke = false;
                for ch in lines[line_index].chars() {
                    if ch == '{' {
                        brace_depth += 1;
                        saw_open = true;
                    } else if ch == '}' {
                        if brace_depth > 0 {
                            brace_depth -= 1;
                        }
                        if saw_open && brace_depth == 0 {
                            end = line_index + 1;
                            broke = true;
                            break;
                        }
                    }
                }
                if broke {
                    break;
                }
                if saw_open && brace_depth == 0 && end > start {
                    break;
                }
                line_index += 1;
            }
            if !saw_open && i + 1 < starts.len() {
                end = starts[i + 1].2 - 1;
            }
        }
        let end = end.max(start);
        out.push(DefinitionRange {
            name: starts[i].0.clone(),
            kind: refine_definition_kind(&starts[i].1, lines[start - 1]),
            start: start as i64,
            end: end as i64,
            excerpt: starts[i].0.clone(),
        });
    }
    out
}

fn parse_definition_ranges_ruby(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange> {
    let Some(pat) = lang.definition.as_ref() else {
        return Vec::new();
    };
    let end_pat = lang.aux_patterns.get("endLine").expect("ruby declares endLine");
    let block_pat = lang.aux_patterns.get("blockStart").expect("ruby declares blockStart");
    let mut def_stack: Vec<(String, i64, i64)> = Vec::new();
    let mut out: Vec<DefinitionRange> = Vec::new();
    let mut depth = 0i64;
    for (i, line) in lines.iter().enumerate() {
        let line_num = (i + 1) as i64;
        let trimmed = line.trim();
        match pattern_match(pat, line) {
            Some((_, caps)) => {
                let name = capture(&caps, "name").unwrap_or_default();
                if !name.is_empty() {
                    def_stack.push((name, line_num, depth));
                    depth += 1;
                }
            }
            None => {
                if pattern_find(block_pat, line) && !line.contains(" do ") {
                    depth += 1;
                }
            }
        }
        if pattern_match(end_pat, trimmed).is_some() {
            if depth > 0 {
                depth -= 1;
            }
            while def_stack.last().is_some_and(|(_, _, d)| *d == depth) {
                let (name, start, _) = def_stack.pop().expect("stack is not empty");
                out.push(DefinitionRange { name: name.clone(), kind: String::new(), start, end: line_num, excerpt: name });
            }
        }
    }
    for (name, start, _) in def_stack {
        out.push(DefinitionRange { name: name.clone(), kind: String::new(), start, end: lines.len() as i64, excerpt: name });
    }
    out.sort_by_key(|d| d.start);
    out
}

/// ▶️ The raw definition ranges a language finds in `lines`.
pub fn parse_definition_ranges(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange> {
    match lang.definition_engine {
        Some(DefinitionEngine::RubyEnd) => parse_definition_ranges_ruby(lang, lines),
        _ => parse_definition_ranges_default(lang, lines),
    }
}

/// 🧷️ The extra single-line definitions a language contributes outside its definition pattern.
pub fn extra_orphan_definitions(lang: &Language, lines: &[&str]) -> Vec<DefinitionRange> {
    let mut defs = Vec::new();
    match lang.orphan_definitions {
        OrphanDefinitions::None => {}
        OrphanDefinitions::GoPackageAndImports => {
            for (i, line) in lines.iter().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("package ") {
                    defs.push(DefinitionRange {
                        name: format!("package-{}", i + 1),
                        kind: String::new(),
                        start: i as i64 + 1,
                        end: i as i64 + 1,
                        excerpt: trimmed.to_string(),
                    });
                    break;
                }
            }
            let mut i = 0usize;
            while i < lines.len() {
                let trimmed = lines[i].trim();
                if trimmed.starts_with("import ") {
                    let start = i + 1;
                    let mut end = start;
                    if trimmed.starts_with("import (") {
                        // 📏️The index is the answer here, and it also advances the outer cursor.
                        #[allow(clippy::needless_range_loop)]
                        for j in i + 1..lines.len() {
                            if lines[j].trim() == ")" {
                                end = j + 1;
                                i = j;
                                break;
                            }
                        }
                    }
                    defs.push(DefinitionRange {
                        name: format!("import-{start}"),
                        kind: String::new(),
                        start: start as i64,
                        end: end as i64,
                        excerpt: lines[start - 1].trim().to_string(),
                    });
                }
                i += 1;
            }
        }
        OrphanDefinitions::RustModDeclarations => {
            for (i, line) in lines.iter().enumerate() {
                let trimmed = line.trim();
                let rest = trimmed.strip_prefix("pub ").unwrap_or(trimmed);
                if let Some(tail) = rest.strip_prefix("mod ") {
                    let name = tail.trim_end_matches(';').trim();
                    if !name.is_empty() && tail.trim_end().ends_with(';') && ident_len(name, 0) == name.len() {
                        defs.push(DefinitionRange {
                            name: format!("mod-{}-{}", name, i + 1),
                            kind: String::new(),
                            start: i as i64 + 1,
                            end: i as i64 + 1,
                            excerpt: trimmed.to_string(),
                        });
                    }
                }
            }
        }
        OrphanDefinitions::RubyModules => {
            for (i, line) in lines.iter().enumerate() {
                let after = line.trim_start();
                if let Some(tail) = after.strip_prefix("module ") {
                    let ends = scoped_ends(tail, 0, "::");
                    if let Some(&end) = ends.first() {
                        defs.push(DefinitionRange {
                            name: format!("module-{}-{}", &tail[..end], i + 1),
                            kind: String::new(),
                            start: i as i64 + 1,
                            end: i as i64 + 1,
                            excerpt: line.trim().to_string(),
                        });
                    }
                }
            }
        }
    }
    defs
}

/// 📖️ The definitions of `content` under the language claiming `file_path`.
pub fn parse_definitions(content: &str, file_path: &str) -> Vec<Definition> {
    let Some(lang) = language_for_path(file_path) else {
        return Vec::new();
    };
    if !lang.supports_definitions() {
        return Vec::new();
    }
    let lines = split_lines(content);
    parse_definition_ranges(lang, &lines)
        .into_iter()
        .map(|r| Definition {
            id: String::new(),
            name: r.name,
            kind: derive_definition_kind(&r.kind),
            file_path: file_path.to_string(),
            section_path: String::new(),
            emoji: String::new(),
            start_line: r.start,
            end_line: r.end,
            start_index: 0,
            end_index: 0,
        })
        .collect()
}

// #endregion 🔖️Definitions

// #region 🔖️Scopes

/// 📍️ One addressable scope inside a file.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ScopeEntry {
    pub kind: String,
    pub id: String,
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(rename = "sectionPath", skip_serializing_if = "String::is_empty")]
    pub section_path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub definition: String,
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[serde(rename = "endLine")]
    pub end_line: i64,
}

/// 📑️ A section found by the scope builder: flat, dot-joined path, no emoji split.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSection {
    pub name: String,
    pub path: String,
    pub start_line: i64,
    pub end_line: i64,
}

/// 🔭️ The deterministic identifier of a scope.
pub fn build_scope_id(kind: &str, file_path: &str, section_path: &str, definition: &str) -> String {
    match kind {
        "file" => format!("file:{file_path}"),
        "section" => format!("section:{file_path}#{section_path}"),
        _ if !section_path.is_empty() => format!("def:{file_path}#{section_path}::{definition}"),
        _ => format!("def:{file_path}#{definition}"),
    }
}

/// 💬️ A region marker on `line`: its name and whether it closes a region.
pub fn parse_region_marker(line: &str) -> Option<(String, bool)> {
    let m = &table().scope_region_marker;
    let mut trimmed = line.trim().to_string();
    for prefix in &m.strip_prefixes {
        if let Some(rest) = trimmed.strip_prefix(prefix.as_str()) {
            trimmed = rest.to_string();
        }
    }
    for suffix in &m.strip_suffixes {
        if let Some(rest) = trimmed.strip_suffix(suffix.as_str()) {
            trimmed = rest.to_string();
        }
    }
    let trimmed = trimmed.trim();
    for (keyword, is_end) in [(&m.start_keyword, false), (&m.end_keyword, true)] {
        if let Some(rest) = trimmed.strip_prefix(keyword.as_str()) {
            let (emoji, name) = extract_entity_emoji(rest.trim());
            if !emoji.is_empty() {
                return Some((name, is_end));
            }
        }
    }
    None
}

/// 🔬️ A markdown heading on `line`: its level and title.
pub fn parse_markdown_heading(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return None;
    }
    let level = trimmed.chars().take_while(|c| *c == '#').count();
    if level == 0 || level > 6 {
        return None;
    }
    let name = trimmed[level..].trim();
    if name.is_empty() {
        return None;
    }
    Some((level, name.to_string()))
}

/// 🌳️ Sections of `lines` for the scope builder: region markers plus markdown headings.
pub fn parse_sections_from_lines(lines: &[&str], ext: &str) -> Vec<ParsedSection> {
    struct Frame {
        name: String,
        start_line: i64,
        level: usize,
        path: String,
    }
    let mut sections = Vec::new();
    let mut stack: Vec<Frame> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let line_number = (index + 1) as i64;
        if let Some((name, is_end)) = parse_region_marker(line) {
            if is_end {
                if let Some(frame) = stack.pop() {
                    sections.push(ParsedSection { name: frame.name, path: frame.path, start_line: frame.start_line, end_line: line_number - 1 });
                }
            } else {
                let path = match stack.last() {
                    Some(top) => format!("{}.{}", top.path, name),
                    None => name.clone(),
                };
                stack.push(Frame { name, start_line: line_number, level: 0, path });
            }
            continue;
        }
        if ext == ".md" || ext == ".mdx" {
            if let Some((level, title)) = parse_markdown_heading(line) {
                while stack.last().is_some_and(|f| f.level >= level) {
                    let frame = stack.pop().expect("stack is not empty");
                    sections.push(ParsedSection { name: frame.name, path: frame.path, start_line: frame.start_line, end_line: line_number - 1 });
                }
                let path = match stack.last() {
                    Some(top) => format!("{}.{}", top.path, title),
                    None => title.clone(),
                };
                stack.push(Frame { name: title, start_line: line_number, level, path });
            }
        }
    }
    for frame in stack {
        sections.push(ParsedSection { name: frame.name, path: frame.path, start_line: frame.start_line, end_line: lines.len() as i64 });
    }
    sections
}

/// 🧩️ Single-line definitions of `lines` for the scope builder.
pub fn parse_definitions_from_lines(lines: &[&str], patterns: &[Pattern]) -> Vec<(String, i64)> {
    let mut defs = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        for pattern in patterns {
            if let Some((_, caps)) = pattern_match(pattern, line) {
                if let Some(name) = capture(&caps, "name") {
                    defs.push((name, (index + 1) as i64));
                    break;
                }
            }
        }
    }
    defs
}

/// 🏗️ Every addressable scope of a file: the file itself, its sections and its definitions.
pub fn build_scopes_for_file(path: &str, content: &str) -> Vec<ScopeEntry> {
    let lines = split_lines(content);
    let ext = path_extension(path).to_lowercase();
    let mut entries = vec![ScopeEntry {
        kind: "file".to_string(),
        id: build_scope_id("file", path, "", ""),
        file_path: path.to_string(),
        section_path: String::new(),
        definition: String::new(),
        start_line: 1,
        end_line: lines.len() as i64,
    }];
    let sections = parse_sections_from_lines(&lines, &ext);
    for s in &sections {
        entries.push(ScopeEntry {
            kind: "section".to_string(),
            id: build_scope_id("section", path, &s.path, ""),
            file_path: path.to_string(),
            section_path: s.path.clone(),
            definition: String::new(),
            start_line: s.start_line,
            end_line: s.end_line,
        });
    }
    let mut section_by_line: BTreeMap<i64, String> = BTreeMap::new();
    for s in &sections {
        for line in s.start_line..=s.end_line {
            section_by_line.insert(line, s.path.clone());
        }
    }
    let empty = Vec::new();
    let patterns = table().scope_definition_patterns.get(&ext).unwrap_or(&empty);
    for (name, start_line) in parse_definitions_from_lines(&lines, patterns) {
        let sp = section_by_line.get(&start_line).cloned().unwrap_or_default();
        entries.push(ScopeEntry {
            kind: "definition".to_string(),
            id: build_scope_id("definition", path, &sp, &name),
            file_path: path.to_string(),
            section_path: sp,
            definition: name,
            start_line,
            end_line: start_line,
        });
    }
    entries
}

// #endregion 🔖️Scopes

// #region 🔖️Headers

/// 🧾️ The decomposed fields of a file header region.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize)]
pub struct Header {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "fileUri")]
    pub file_uri: String,
    pub summary: String,
    pub contributors: String,
    pub license: String,
    pub requirements: String,
}

/// 🐍️ The Rust module identifier a section name is wrapped in.
pub fn section_name_to_mod_name(name: &str) -> String {
    let runes: Vec<char> = name.chars().collect();
    let mut buf = String::new();
    for (i, r) in runes.iter().enumerate() {
        if r.is_ascii_uppercase() {
            if i > 0 {
                let prev = runes[i - 1];
                if prev.is_ascii_lowercase() || prev.is_ascii_digit() {
                    buf.push('_');
                }
            }
            buf.push(r.to_ascii_lowercase());
        } else if r.is_ascii_lowercase() || r.is_ascii_digit() {
            buf.push(*r);
        } else if !buf.is_empty() && !buf.ends_with('_') {
            buf.push('_');
        }
    }
    let result = buf.trim_matches('_').to_string();
    if result.is_empty() {
        "section".to_string()
    } else {
        result
    }
}

fn apply_format(lang: &Language, format: &str, name: &str) -> String {
    if format.is_empty() {
        return String::new();
    }
    let with_name = format.replace("{name}", name);
    match lang.section_naming {
        SectionNaming::ModName => with_name.replace("{modName}", &section_name_to_mod_name(name)),
        SectionNaming::Verbatim => with_name,
    }
}

/// 🔤️ The opening marker of a named section.
pub fn format_section_start(lang: &Language, name: &str) -> String {
    apply_format(lang, &lang.section_start_format, name)
}

/// ⏹️ The closing marker of a named section.
pub fn format_section_end(lang: &Language, name: &str) -> String {
    apply_format(lang, &lang.section_end_format, name)
}

/// ⬛️ The empty section skeleton of a named section.
pub fn format_section_both(lang: &Language, name: &str) -> String {
    apply_format(lang, &lang.section_both_format, name)
}

/// ⬜️ The header region of a file, or the empty string for a language without headers.
pub fn format_header(lang: &Language, header: &Header) -> String {
    if !lang.supports_headers {
        return String::new();
    }
    let cp = &lang.comment_prefix;
    let mut b = String::new();
    b.push_str(&format_section_start(lang, "Header"));
    b.push('\n');
    b.push_str(&format!("{} [{}]({})\n", cp, header.file_id, header.file_uri));
    for line in header.contributors.split('\n') {
        if !line.trim().is_empty() {
            b.push_str(&format!("{cp} {line}\n"));
        }
    }
    b.push('\n');
    for line in header.license.split('\n') {
        if line.is_empty() {
            b.push_str(&format!("{cp}\n"));
        } else {
            b.push_str(&format!("{cp} {line}\n"));
        }
    }
    b.push('\n');
    for block in [&header.summary, &header.requirements] {
        if block.is_empty() {
            continue;
        }
        for line in block.split('\n') {
            if line.is_empty() {
                b.push_str(&format!("{cp}\n"));
            } else {
                b.push_str(&format!("{cp} {line}\n"));
            }
        }
        b.push('\n');
    }
    b.push_str(&format_section_end(lang, "Header"));
    b.push('\n');
    b
}

/// 🧾️ Reads a header region back into its fields.
///
/// A formatted header carries no marker separating the summary block from the requirements block, so
/// the first optional block is read as the summary and the second as the requirements. The property
/// that holds for every input is idempotence: formatting a parsed header reproduces the text exactly.
pub fn parse_header(lang: &Language, content: &str) -> Option<Header> {
    if !lang.supports_headers {
        return None;
    }
    let cp = &lang.comment_prefix;
    let start = format_section_start(lang, "Header");
    let end = format_section_end(lang, "Header");
    let lines: Vec<&str> = content.split('\n').collect();
    let start_at = lines.iter().position(|l| l.trim_end() == start.trim_end())?;
    let end_at = lines.iter().skip(start_at + 1).position(|l| l.trim_end() == end.trim_end())? + start_at + 1;
    let body = &lines[start_at + 1..end_at];
    let mut blocks: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for raw in body {
        if raw.trim().is_empty() {
            if !current.is_empty() {
                blocks.push(std::mem::take(&mut current));
            }
            continue;
        }
        let stripped = raw.strip_prefix(cp.as_str())?;
        current.push(stripped.strip_prefix(' ').unwrap_or(stripped).to_string());
    }
    if !current.is_empty() {
        blocks.push(current);
    }
    let identity = blocks.first()?.first()?.clone();
    let open = identity.find('[')?;
    let close = identity.find("](")?;
    let uri_end = identity.rfind(')')?;
    let mut header = Header {
        file_id: identity[open + 1..close].to_string(),
        file_uri: identity[close + 2..uri_end].to_string(),
        contributors: blocks[0][1..].join("\n"),
        ..Header::default()
    };
    if let Some(license) = blocks.get(1) {
        header.license = license.join("\n");
    }
    if let Some(summary) = blocks.get(2) {
        header.summary = summary.join("\n");
    }
    if let Some(requirements) = blocks.get(3) {
        header.requirements = requirements.join("\n");
    }
    Some(header)
}

// #endregion 🔖️Headers

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_loads_every_registered_language() {
        let t = table();
        assert_eq!(t.schema_version, 1);
        assert_eq!(t.registry.len(), 12);
        for name in &t.registry {
            assert!(language_by_name(name).is_some(), "{name} is missing from the table");
        }
        assert!(language_for_extension_unregistered(".json").is_some());
    }

    /// 🔗️ The table's kind map and the model crate's hand-written classifier are two statements of the
    /// same rule; a divergence here means one of them was edited alone.
    #[test]
    fn definition_kind_map_agrees_with_the_model_crate() {
        for raw in table().definition_kind_map.keys() {
            assert_eq!(derive_definition_kind(raw), semio_framework_repo_model::derive_definition_kind(raw), "{raw}");
        }
        for raw in ["func", "fn", "class", "struct", "impl", "definition", ""] {
            assert_eq!(derive_definition_kind(raw), semio_framework_repo_model::derive_definition_kind(raw), "{raw}");
        }
    }

    #[test]
    fn extension_lookup_follows_registry_order() {
        assert_eq!(language_for_path("a/b/🐹️.go").map(|l| l.name.as_str()), Some("go"));
        assert_eq!(language_for_path("🟦️.TSX").map(|l| l.name.as_str()), Some("typescript"));
        assert!(language_for_path("x.unknown").is_none());
    }

    #[test]
    fn marker_sections_nest_and_close() {
        let src = "// #region 🔖️Outer\nlet a = 1;\n// #region 🔖️Inner\nlet b = 2;\n// #endregion 🔖️Inner\n// #endregion 🔖️Outer\n";
        let sections = parse_sections(src, "x.ts");
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].name, "Outer");
        assert_eq!(sections[0].emoji, "🔖️");
        assert_eq!((sections[0].start_line, sections[0].end_line), (1, 6));
        assert_eq!(sections[0].children.len(), 1);
        assert_eq!(sections[0].children[0].name, "Inner");
        assert_eq!((sections[0].children[0].start_line, sections[0].children[0].end_line), (3, 5));
    }

    #[test]
    fn unclosed_region_runs_to_the_end_of_the_file() {
        let src = "// #region 🔖️Open\nlet a = 1;\n";
        let sections = parse_sections(src, "x.ts");
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].end_line, 3);
        assert_eq!(sections[0].end_index, src.len() as i64);
    }

    #[test]
    fn stray_endregion_is_ignored() {
        let src = "// #endregion 🔖️Nothing\nlet a = 1;\n";
        assert!(parse_sections(src, "x.ts").is_empty());
    }

    #[test]
    fn markdown_sections_nest_by_level() {
        let src = "# Title\ntext\n## Child\nmore\n# Second\n";
        let sections = parse_markdown_sections(src);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].name, "Title");
        assert_eq!(sections[0].children.len(), 1);
        assert_eq!(sections[0].children[0].name, "Child");
        assert_eq!(sections[1].name, "Second");
    }

    #[test]
    fn json_sections_span_key_through_value() {
        let src = "{\n  \"a\": 1,\n  \"b\": { \"c\": \"x\" }\n}\n";
        let sections = parse_json_sections(src);
        assert_eq!(sections.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["a", "b"]);
        assert_eq!(sections[1].children.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["c"]);
        assert!(sections.iter().all(|s| s.end_line > 0));
    }

    #[test]
    fn typescript_definitions_carry_kinds() {
        let src = "export function alpha() {\n  return 1;\n}\nexport const beta = () => 2;\nexport interface Gamma {\n  a: number;\n}\n";
        let ranges = parse_definition_ranges(language_by_name("typescript").expect("registered"), &split_lines(src));
        let defs = parse_definitions(src, "x.ts");
        let seen: Vec<(&str, &str, i64, i64)> = ranges.iter().map(|r| (r.name.as_str(), r.kind.as_str(), r.start, r.end)).collect();
        assert_eq!(seen[0], ("alpha", "function", 1, 3));
        assert_eq!(seen[1].0, "beta");
        assert_eq!(seen[1].1, "function");
        assert_eq!(seen[2], ("Gamma", "interface", 5, 7));
        assert_eq!(defs[2].kind, DefinitionKind::Interface);
    }

    #[test]
    fn go_method_receivers_do_not_become_the_name() {
        let src = "func (l *BaseLanguage) Name() string {\n\treturn l.name\n}\n";
        let defs = parse_definitions(src, "x.go");
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].name, "Name");
        assert_eq!(parse_definition_ranges(language_by_name("go").expect("registered"), &split_lines(src))[0].kind, "func");
    }

    #[test]
    fn python_definitions_close_on_dedent() {
        let src = "def outer():\n    x = 1\n    return x\n\ndef other():\n    pass\n";
        let defs = parse_definitions(src, "x.py");
        assert_eq!(defs[0].name, "outer");
        assert_eq!((defs[0].start_line, defs[0].end_line), (1, 4));
        assert_eq!(defs[1].name, "other");
    }

    #[test]
    fn sql_definitions_use_the_multi_word_keyword() {
        let src = "CREATE TABLE IF NOT EXISTS public.items (\n  id INT\n);\n";
        let defs = parse_definitions(src, "x.sql");
        assert_eq!(defs[0].name, "public.items");
        assert_eq!(parse_definition_ranges(language_by_name("sql").expect("registered"), &split_lines(src))[0].kind, "CREATE TABLE");
    }

    #[test]
    fn graphql_extend_type_is_an_interface() {
        let src = "extend type Query {\n  a: Int\n}\n";
        let defs = parse_definitions(src, "x.graphql");
        assert_eq!(defs[0].name, "Query");
        assert_eq!(defs[0].kind, DefinitionKind::Interface);
    }

    #[test]
    fn ruby_definitions_close_on_end() {
        let src = "class Alpha\ndef beta\n  1\nend\nend\n";
        let defs = parse_definitions(src, "x.rb");
        assert_eq!(defs.iter().map(|d| (d.name.as_str(), d.start_line, d.end_line)).collect::<Vec<_>>(), [("Alpha", 1, 5), ("beta", 2, 4)]);
    }

    /// 🪤️ The Ruby definition pattern is anchored with no leading whitespace, exactly as the Go
    /// original is, so an indented `def` is invisible to it. Pinned so a silent "fix" cannot slip in.
    #[test]
    fn ruby_ignores_indented_definitions() {
        let src = "class Alpha\n  def beta\n    1\n  end\nend\n";
        let defs = parse_definitions(src, "x.rb");
        assert_eq!(defs.iter().map(|d| (d.name.as_str(), d.start_line, d.end_line)).collect::<Vec<_>>(), [("Alpha", 1, 4)]);
    }

    #[test]
    fn scope_ids_follow_the_grammar() {
        assert_eq!(build_scope_id("file", "a/b.ts", "", ""), "file:a/b.ts");
        assert_eq!(build_scope_id("section", "a/b.ts", "Outer.Inner", ""), "section:a/b.ts#Outer.Inner");
        assert_eq!(build_scope_id("definition", "a/b.ts", "Outer", "alpha"), "def:a/b.ts#Outer::alpha");
        assert_eq!(build_scope_id("definition", "a/b.ts", "", "alpha"), "def:a/b.ts#alpha");
    }

    #[test]
    fn scopes_for_a_file_cover_sections_and_definitions() {
        let src = "// #region 🔖️Outer\nexport function alpha() {}\n// #endregion 🔖️Outer\n";
        let scopes = build_scopes_for_file("a/b.ts", src);
        let ids: Vec<&str> = scopes.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(ids, ["file:a/b.ts", "section:a/b.ts#Outer", "def:a/b.ts#Outer::alpha"]);
    }

    #[test]
    fn hydration_puts_definitions_on_the_deepest_section() {
        let src = "// #region 🔖️Outer\n// #region 🔖️Inner\nexport function alpha() {}\n// #endregion 🔖️Inner\nexport function beta() {}\n// #endregion 🔖️Outer\n";
        let sections = parse_sections(src, "x.ts");
        let defs = parse_definitions(src, "x.ts");
        let hydrated = hydrate_sections_with_definitions(&sections, &defs);
        assert_eq!(hydrated[0].definitions.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(), ["beta"]);
        assert_eq!(hydrated[0].children[0].definitions.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(), ["alpha"]);
    }

    #[test]
    fn normalize_section_path_splits_on_both_separators() {
        assert_eq!(normalize_section_path("a/b#c//d"), ["a", "b", "c", "d"]);
        assert_eq!(normalize_section_path(""), Vec::<String>::new());
    }

    #[test]
    fn header_formatting_round_trips_through_parsing() {
        let header = Header {
            file_id: "💻️test/file.ts".to_string(),
            file_uri: "repo://file/💻️test".to_string(),
            summary: "A test file".to_string(),
            contributors: "2025 Test User <test@test.com>".to_string(),
            license: "AGPL license text here".to_string(),
            requirements: "Some requirements".to_string(),
        };
        for name in ["typescript", "go", "python", "csharp", "rust", "ruby", "shell", "sql", "graphql"] {
            let lang = language_by_name(name).expect("registered");
            let text = format_header(lang, &header);
            assert!(text.contains("[💻️test/file.ts](repo://file/💻️test)"), "{name}");
            let parsed = parse_header(lang, &text).expect("header parses");
            assert_eq!(format_header(lang, &parsed), text, "{name}");
            assert_eq!(parsed, header, "{name}");
        }
        for name in ["markdown", "toml", "yaml"] {
            let lang = language_by_name(name).expect("registered");
            assert_eq!(format_header(lang, &header), "");
        }
    }

    #[test]
    fn rust_sections_wrap_a_module() {
        let lang = language_by_name("rust").expect("registered");
        assert_eq!(format_section_start(lang, "Header"), "mod header { // 🔖️Header");
        assert_eq!(format_section_end(lang, "Header"), "} // 🔖️Header");
        assert_eq!(section_name_to_mod_name("Missing Hook Functions"), "missing_hook_functions");
        assert_eq!(section_name_to_mod_name("🔖️"), "section");
    }

    #[test]
    fn case_insensitive_markers_are_accepted() {
        let src = "// #REGION 🔖️Loud\n// #EndRegion 🔖️Loud\n";
        let sections = parse_sections(src, "x.ts");
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].name, "Loud");
    }
}

// #endregion 🔖️Tests
