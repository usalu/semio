//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 📜️ Repository statutes: the closed catalog of statutes and policies, the breach analysis that
//! detects violations of them, the ignore directives that suppress a breach, the autofix
//! transformations, and the compressed breach-cache envelope.
//! Behaviour twin of `github.com/usalu/semio/repo/statutes`; the vocabulary comes from
//! `🧬️schema/🔣️statutes.json`, the one table the twin is meant to read too.

//#endregion 🧲️Header

use semio_framework_repo_languages as languages;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// 🚪️ The model types the statute surface speaks in, re-exported so a client never depends on
/// `📐️model` directly to read a catalog entry or a breach.
pub use semio_framework_repo_model::{Breach, BreachPriority, Policy, Statute, StatuteMeta, Territory};

//#region 📚️Catalog

/// 📖️ The catalog document both implementations read.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    pub statutes: Vec<StatuteMeta>,
    pub policies: Vec<Policy>,
}

const CATALOG_SOURCE: &str = include_str!("../../🧬️schema/🔣️statutes.json");

static CATALOG: OnceLock<Catalog> = OnceLock::new();
static META_INDEX: OnceLock<BTreeMap<String, usize>> = OnceLock::new();

/// 📚️ The parsed statute catalog.
pub fn catalog() -> &'static Catalog {
    CATALOG.get_or_init(|| serde_json::from_str(CATALOG_SOURCE).expect("🔣️statutes.json is a valid catalog"))
}

fn meta_index() -> &'static BTreeMap<String, usize> {
    META_INDEX.get_or_init(|| catalog().statutes.iter().enumerate().map(|(i, s)| (s.kind.0.clone(), i)).collect())
}

/// 📜️ Every statute of the catalog, in declaration order.
pub fn statutes() -> &'static [StatuteMeta] {
    &catalog().statutes
}

/// 🏛️ Every policy of the catalog, in declaration order.
pub fn policies() -> &'static [Policy] {
    &catalog().policies
}

/// 🔍️ The policy with the given id.
pub fn find_policy(id: &str) -> Option<&'static Policy> {
    catalog().policies.iter().find(|p| p.id == id)
}

/// ℹ️ The declared metadata of a statute, or the unknown-breach fallback.
pub fn statute_info(kind: &Statute) -> StatuteMeta {
    match meta_index().get(&kind.0) {
        Some(index) => catalog().statutes[*index].clone(),
        None => StatuteMeta {
            kind: kind.clone(),
            policy_id: String::new(),
            priority: BreachPriority::Low,
            reason: "Unknown breach".to_string(),
            solution: "Fix the breach".to_string(),
            autofixable: false,
        },
    }
}

/// 🔧️ Whether the statute supports automatic fixing.
pub fn is_autofixable(kind: &Statute) -> bool {
    statute_info(kind).autofixable
}

/// 🪜️ Every statute a territory and its nested territories declare, depth first.
pub fn territory_kinds(territory: &Territory) -> Vec<Statute> {
    territory.all_kinds()
}

/// 🪜️ Every statute a policy declares through its territories, depth first.
pub fn policy_kinds(policy: &Policy) -> Vec<Statute> {
    policy.groups.iter().flatten().flat_map(|g| g.all_kinds()).collect()
}

//#endregion 📚️Catalog

//#region 🙈️Ignore Directives

const IGNORE_PREFIX: &str = "// compose-ignore-";
const IGNORE_WINDOW: i64 = 100;

/// 🙈️ The statute prefixes each `// compose-ignore-` line suppresses, keyed by one-based line.
pub fn parse_ignore_directives(content: &str) -> BTreeMap<i64, Vec<String>> {
    let mut result: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    for (index, line) in content.split('\n').enumerate() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(IGNORE_PREFIX) {
            for part in rest.split(',') {
                let pattern = part.trim();
                if !pattern.is_empty() {
                    result.entry(index as i64 + 1).or_default().push(pattern.to_string());
                }
            }
        }
    }
    result
}

/// 🚫️ Whether a breach on `breach_line` is suppressed by a directive of the file.
pub fn is_ignored(directives: &BTreeMap<i64, Vec<String>>, breach_line: i64, kind: &Statute) -> bool {
    directives.iter().any(|(ignore_line, patterns)| {
        breach_line > *ignore_line
            && breach_line <= ignore_line + IGNORE_WINDOW
            && patterns.iter().any(|pattern| kind.0.starts_with(pattern))
    })
}

/// 📄️ The file part of a breach scope, dropping the section and definition suffixes.
pub fn extract_file_from_scope(scope: &str) -> String {
    let mut value = scope;
    if let Some(index) = value.find('#') {
        value = &value[..index];
    }
    if let Some(index) = value.find("::") {
        value = &value[..index];
    }
    value.to_string()
}

/// 🧹️ The breaches that survive the ignore directives of the files they point at.
pub fn filter_ignored(breachs: Vec<Breach>, sources: &SourceSet) -> Vec<Breach> {
    breachs
        .into_iter()
        .filter(|breach| {
            let file = extract_file_from_scope(&breach.scope);
            match sources.directives(&file) {
                Some(directives) => !is_ignored(directives, breach.line, &breach.kind),
                None => true,
            }
        })
        .collect()
}

//#endregion 🙈️Ignore Directives

//#region 🔶️Breach

/// 🆔️ The canonical identifier of a breach at a scope and position.
pub fn build_breach_id(scope: &str, line: i64, col: i64) -> String {
    if line > 0 && col > 0 {
        format!("repo/breach/{scope}#{line}:{col}")
    } else if line > 0 {
        format!("repo/breach/{scope}#{line}")
    } else {
        format!("repo/breach/{scope}")
    }
}

/// 🔶️ A breach of a statute at a scope and position.
pub fn create_breach(summary: &str, kind: Statute, scope: &str, line: i64, col: i64, excerpt: &str) -> Breach {
    Breach {
        id: build_breach_id(scope, line, col),
        summary: summary.to_string(),
        kind,
        scope: scope.to_string(),
        line,
        column: col,
        excerpt: excerpt.to_string(),
        lint_priority: None,
        lint_autofixable: None,
        reason: String::new(),
        solution: String::new(),
    }
}

//#endregion 🔶️Breach

//#region 📐️Specification Text

const SPEC_KEYWORDS: [&str; 7] = ["MUST", "SHOULD", "SHALL", "MAY", "REQUIRED", "RECOMMENDED", "OPTIONAL"];

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn word_at(bytes: &[u8], position: usize, word: &str) -> bool {
    let end = position + word.len();
    if end > bytes.len() || &bytes[position..end] != word.as_bytes() {
        return false;
    }
    if position > 0 && is_word_byte(bytes[position - 1]) {
        return false;
    }
    end == bytes.len() || !is_word_byte(bytes[end])
}

/// 📐️ Whether the text carries a specification keyword as a whole word.
pub fn is_spec_text(text: &str) -> bool {
    let bytes = text.as_bytes();
    (0..bytes.len()).any(|position| SPEC_KEYWORDS.iter().any(|word| word_at(bytes, position, word)))
}

fn backtick_pair(bytes: &[u8]) -> bool {
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'`' {
            let mut scan = index + 1;
            while scan < bytes.len() && bytes[scan] != b'`' {
                scan += 1;
            }
            if scan < bytes.len() && scan > index + 1 {
                return true;
            }
            index = scan;
        }
        index += 1;
    }
    false
}

fn qualified_call_at(bytes: &[u8], position: usize) -> Option<usize> {
    if !(bytes[position].is_ascii_alphabetic() || bytes[position] == b'_') {
        return None;
    }
    let mut scan = position + 1;
    while scan < bytes.len() && is_word_byte(bytes[scan]) {
        scan += 1;
    }
    while scan > position {
        if bytes.get(scan) == Some(&b'.') {
            let mut tail = scan + 1;
            while tail < bytes.len() && is_word_byte(bytes[tail]) {
                tail += 1;
            }
            if tail > scan + 1 && bytes.get(tail) == Some(&b'(') {
                return Some(tail + 1);
            }
        }
        scan -= 1;
    }
    None
}

fn bare_call_at(bytes: &[u8], position: usize) -> Option<usize> {
    if !bytes[position].is_ascii_uppercase() {
        return None;
    }
    let mut scan = position + 1;
    while scan < bytes.len() && is_word_byte(bytes[scan]) {
        scan += 1;
    }
    if scan > position + 1 && bytes.get(scan) == Some(&b'(') {
        return Some(scan + 1);
    }
    None
}

/// 🧪️ Whether a specification line leaks implementation syntax, and why.
pub fn has_implementation_syntax(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    if backtick_pair(bytes) {
        return Some("backtick-wrapped code".to_string());
    }
    for position in 0..bytes.len() {
        let end = qualified_call_at(bytes, position).or_else(|| bare_call_at(bytes, position));
        if let Some(end) = end {
            let matched = &text[position..end];
            if matched != "MUST(" && matched != "SHOULD(" && matched != "SHALL(" && matched != "MAY(" {
                return Some(format!("function/method call: {matched}"));
            }
        }
    }
    None
}

fn contributor_line(text: &str) -> bool {
    let bytes = text.as_bytes();
    for start in 0..bytes.len() {
        if start + 4 > bytes.len() || !bytes[start..start + 4].iter().all(u8::is_ascii_digit) {
            continue;
        }
        let mut cursor = start + 4;
        let space_start = cursor;
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor == space_start {
            continue;
        }
        let filler_start = cursor;
        while cursor < bytes.len() && (is_word_byte(bytes[cursor]) || bytes[cursor].is_ascii_whitespace()) {
            cursor += 1;
        }
        let mut back = cursor;
        while back > filler_start {
            if bytes.get(back) == Some(&b'<') {
                let mut tail = back + 1;
                while tail < bytes.len() && (is_word_byte(bytes[tail]) || matches!(bytes[tail], b'.' | b'@' | b'-')) {
                    tail += 1;
                }
                if tail > back + 1 && bytes.get(tail) == Some(&b'>') {
                    return true;
                }
            }
            back -= 1;
        }
    }
    false
}

//#endregion 📐️Specification Text

//#region 🗂️Sources

/// 📄️ One analysed file: its repository-relative path and its full text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
}

/// 🗂️ The analysed files with their parsed sections and ignore directives.
pub struct SourceSet {
    files: Vec<SourceFile>,
    directives: BTreeMap<String, BTreeMap<i64, Vec<String>>>,
}

impl SourceSet {
    /// 🆕️ Parses the ignore directives of every file once.
    pub fn new(files: Vec<SourceFile>) -> Self {
        let directives = files.iter().map(|f| (f.path.clone(), parse_ignore_directives(&f.content))).collect();
        Self { files, directives }
    }

    /// 📄️ The analysed files.
    pub fn files(&self) -> &[SourceFile] {
        &self.files
    }

    /// 🙈️ The ignore directives of one file.
    pub fn directives(&self, path: &str) -> Option<&BTreeMap<i64, Vec<String>>> {
        self.directives.get(path)
    }
}

fn is_test_or_benchmark_file(file: &str) -> bool {
    let normalized = file.replace('\\', "/").to_lowercase();
    let name = normalized.rsplit('/').next().unwrap_or(&normalized).to_string();
    let extension = match name.rfind('.') {
        Some(index) if index > 0 => name[index..].to_string(),
        _ => String::new(),
    };
    let stem = name.strip_suffix(&extension).unwrap_or(&name).to_string();
    let lab_suffixes = [".test", ".spec", ".tests", ".requirements", "_test", "_tests", "_spec", "_benchmark", ".benchmark", ".stories", ".story"];
    if lab_suffixes.iter().any(|s| stem.ends_with(s)) || name.starts_with("test_") || name.starts_with("test.") || name == "conftest.py" {
        return true;
    }
    if normalized.contains("/tests/") || normalized.contains(".tests/") || normalized.contains("/test/") || normalized.contains(".test/") || normalized.contains("/benchmark/") || normalized.contains(".benchmark/") {
        return true;
    }
    name.ends_with("_test.go")
        || name.ends_with(".test.ts")
        || name.ends_with(".test.tsx")
        || name.ends_with(".test.js")
        || name.ends_with(".test.jsx")
        || name.ends_with(".spec.ts")
        || name.ends_with(".spec.tsx")
        || name.ends_with(".spec.js")
        || name.ends_with(".spec.jsx")
        || name.starts_with("test_")
        || name.contains("benchmark")
}

fn policy_section_start_match(lang: &languages::Language, line: &str) -> Option<String> {
    let pattern = lang.policy_section_start.as_ref()?;
    let (_, captures) = languages::pattern_match(pattern, line)?;
    Some(capture_name(&captures))
}

fn policy_section_end_match(lang: &languages::Language, line: &str) -> Option<String> {
    let pattern = lang.policy_section_end.as_ref()?;
    let (_, captures) = languages::pattern_match(pattern, line)?;
    Some(capture_name(&captures))
}

fn capture_name(captures: &languages::Captures) -> String {
    let raw = captures.iter().find(|(name, _)| name == "name").map(|(_, value)| value.trim().to_string()).unwrap_or_default();
    let (_, rest) = languages::extract_entity_emoji(&raw);
    rest.trim().to_string()
}

/// 🧿 The words a TypeScript definition head may carry before `function` or `class`.
const DEFINITION_MODIFIER_WORDS: [&str; 4] = ["async ", "abstract ", "declare ", "default "];

/// ✂️ Strips leading modifier words as whole space-separated prefixes, never as a character cutset.
fn strip_definition_modifier_words(line: &str) -> &str {
    let mut rest = line;
    loop {
        let Some(stripped) = DEFINITION_MODIFIER_WORDS.iter().find_map(|word| rest.strip_prefix(word)) else { return rest };
        rest = stripped;
    }
}

fn requires_definition_requirements(line: &str, lang_name: &str) -> bool {
    let trimmed = line.trim();
    match lang_name {
        "typescript" => {
            let no_export = trimmed.strip_prefix("export ").unwrap_or(trimmed);
            let no_modifier = strip_definition_modifier_words(no_export);
            no_modifier.starts_with("function ") || no_modifier.starts_with("class ")
        }
        "go" => trimmed.starts_with("func "),
        "python" => trimmed.starts_with("def ") || trimmed.starts_with("class ") || trimmed.starts_with("async def "),
        "csharp" => !trimmed.contains(" interface ") && !trimmed.contains(" enum "),
        "rust" => {
            let mut no_prefix = trimmed.strip_prefix("pub ").unwrap_or(trimmed);
            if no_prefix.starts_with('(') {
                if let Some(index) = no_prefix.find(") ") {
                    no_prefix = no_prefix[index + 2..].trim_start();
                }
            }
            no_prefix.starts_with("fn ") || no_prefix.starts_with("struct ") || no_prefix.starts_with("impl ") || no_prefix.starts_with("trait ")
        }
        _ => true,
    }
}

//#endregion 🗂️Sources

//#region 🔍️Analyze

const AGPL_MARKERS: [&str; 3] = ["GNU Affero General Public License", "AGPL", "https://www.gnu.org/licenses/"];

/// 🔍️ Every breach the ported policies detect over the given sources, ignore directives applied.
pub fn analyze(sources: &SourceSet) -> Vec<Breach> {
    let mut breachs = Vec::new();
    breachs.extend(header_policy(sources));
    breachs.extend(section_policy(sources));
    breachs.extend(requirements_policy(sources));
    breachs
}

fn find_header(sections: &[languages::Section]) -> Option<&languages::Section> {
    sections.iter().find(|s| s.name.to_lowercase() == "header")
}

/// 🧲️ The header-region breaches of every source: missing region, contributors, license and summary.
pub fn header_policy(sources: &SourceSet) -> Vec<Breach> {
    let mut breachs = Vec::new();
    for source in sources.files() {
        let file = source.path.as_str();
        let content = source.content.as_str();
        if content.is_empty() {
            continue;
        }
        let Some(language) = languages::language_for_path(file) else { continue };
        if !language.supports_headers {
            continue;
        }
        let sections = languages::parse_sections(content, file);
        let Some(header) = find_header(&sections) else {
            breachs.push(create_breach(&format!("Missing header region in {file}"), Statute::from("code/file/missing-header-region"), file, 0, 0, ""));
            continue;
        };
        let header_content = &content[header.start_index as usize..header.end_index as usize];
        let header_lines: Vec<&str> = header_content.split('\n').collect();
        let has_contributors = header_lines.iter().any(|line| contributor_line(line));
        if !has_contributors {
            breachs.push(create_breach(
                &format!("Missing contributors in header of {file}"),
                Statute::from("code/file/missing-contributors"),
                &format!("{file}#Header"),
                header.start_line,
                0,
                "",
            ));
        }
        let has_license = AGPL_MARKERS.iter().any(|marker| header_content.contains(marker));
        if !has_license {
            breachs.push(create_breach(
                &format!("Missing license in header of {file}"),
                Statute::from("code/file/missing-license"),
                &format!("{file}#Header"),
                header.start_line,
                0,
                "",
            ));
        } else {
            let mut wrong = ["MIT", "Apache", "BSD"].iter().any(|name| header_content.contains(name));
            if header_content.contains("GPL") && !header_content.contains("AGPL") && !header_content.contains("LGPL") {
                wrong = true;
            }
            if wrong {
                breachs.push(create_breach(
                    &format!("Wrong license in header of {file}"),
                    Statute::from("code/file/wrong-license"),
                    &format!("{file}#Header"),
                    header.start_line,
                    0,
                    "",
                ));
            }
        }
        let prefix = language.comment_prefix.as_str();
        let mut has_summary = false;
        let mut seen_license = false;
        let mut license_end = false;
        for line in &header_lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                if seen_license && !license_end {
                    license_end = true;
                }
                continue;
            }
            if trimmed.starts_with(&format!("{prefix} #region")) || trimmed.starts_with(&format!("{prefix} #endregion")) {
                continue;
            }
            let Some(rest) = trimmed.strip_prefix(prefix) else { continue };
            let comment_text = rest.trim();
            if comment_text.is_empty() {
                if seen_license && !license_end {
                    license_end = true;
                }
                continue;
            }
            if contributor_line(line) {
                continue;
            }
            let is_license_line = AGPL_MARKERS.iter().any(|marker| line.contains(marker));
            if is_license_line {
                seen_license = true;
                continue;
            }
            let license_body = ["without", "program", "License", "http", "version", "terms", "WARRANTY", "PURPOSE", "General Public", "redistribute", "published", "Foundation", "received"];
            if !license_end && seen_license && license_body.iter().any(|needle| comment_text.contains(needle)) {
                continue;
            }
            if is_spec_text(comment_text) {
                continue;
            }
            if comment_text.starts_with("TODO:") {
                continue;
            }
            has_summary = true;
            break;
        }
        if !has_summary && !is_test_or_benchmark_file(file) {
            breachs.push(create_breach(
                &format!("Missing summary in header of {file}"),
                Statute::from("code/file/missing-summary"),
                &format!("{file}#Header"),
                header.start_line,
                0,
                "",
            ));
        }
    }
    filter_ignored(breachs, sources)
}

struct SectionFrame {
    name: String,
    line: i64,
}

/// 🧩️ The section-structure breaches of every source: markers, emptiness, summaries and orphans.
pub fn section_policy(sources: &SourceSet) -> Vec<Breach> {
    let mut breachs = Vec::new();
    for source in sources.files() {
        let file = source.path.as_str();
        let content = source.content.as_str();
        if content.is_empty() {
            continue;
        }
        let Some(language) = languages::language_for_path(file) else { continue };
        if !language.supports_sections() {
            continue;
        }
        let lines: Vec<&str> = content.split('\n').collect();
        let mut stack: Vec<SectionFrame> = Vec::new();
        for (index, raw) in lines.iter().enumerate() {
            let line_number = index as i64 + 1;
            let line = raw.strip_suffix('\r').unwrap_or(raw);
            if let Some(name) = policy_section_start_match(language, line) {
                if name.is_empty() {
                    breachs.push(create_breach(
                        &format!("Missing section name at {file}:{line_number}"),
                        Statute::from("code/section/missing-start-name"),
                        file,
                        line_number,
                        0,
                        line.trim(),
                    ));
                }
                stack.push(SectionFrame { name, line: line_number });
                continue;
            }
            if let Some(end_name) = policy_section_end_match(language, line) {
                if let Some(open) = stack.pop() {
                    if !open.name.is_empty() {
                        if end_name.is_empty() {
                            breachs.push(create_breach(
                                &format!("Missing end section name at {file}:{line_number}"),
                                Statute::from("code/section/missing-end-name"),
                                file,
                                line_number,
                                0,
                                line.trim(),
                            ));
                        } else if end_name != open.name {
                            breachs.push(create_breach(
                                &format!("Section name mismatch at {file}:{line_number}"),
                                Statute::from("code/section/name-mismatch"),
                                file,
                                line_number,
                                0,
                                &format!("Start: \"{}\" at line {}, End: \"{}\"", open.name, open.line, end_name),
                            ));
                        }
                    }
                }
            }
        }
        let sections = languages::parse_sections(content, file);
        let prefix = language.comment_prefix.as_str();
        let mut section_breachs = Vec::new();
        for section in &sections {
            check_section(section, "", file, content, &lines, language, prefix, &mut section_breachs);
        }
        breachs.append(&mut section_breachs);
        breachs.extend(orphan_breachs(file, &lines, &sections, language, prefix));
        breachs.extend(definition_breachs(file, content, &lines, language, prefix));
    }
    filter_ignored(breachs, sources)
}

#[allow(clippy::too_many_arguments)]
fn check_section(
    section: &languages::Section,
    parent_path: &str,
    file: &str,
    content: &str,
    lines: &[&str],
    language: &languages::Language,
    prefix: &str,
    breachs: &mut Vec<Breach>,
) {
    let section_path = if parent_path.is_empty() { section.name.clone() } else { format!("{parent_path}#{}", section.name) };
    let section_content = &content[section.start_index as usize..section.end_index as usize];
    let section_lines: Vec<&str> = section_content.split('\n').collect();
    let mut non_empty = 0;
    if section_lines.len() >= 2 {
        for line in &section_lines[1..section_lines.len() - 1] {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with('#') {
                non_empty += 1;
            }
        }
    }
    let is_exempt = section.name == "Header";
    if non_empty == 0 && section.children.is_empty() && !is_exempt {
        breachs.push(create_breach(
            &format!("Empty section \"{}\" in {file}", section.name),
            Statute::from("code/section/empty"),
            &format!("{file}#{section_path}"),
            section.start_line,
            0,
            "",
        ));
    }
    if (section.start_line as usize) < lines.len() && lines[section.start_line as usize].trim().is_empty() {
        breachs.push(create_breach(
            &format!("Blank line after region start marker in section \"{}\" in {file}:{}", section.name, section.start_line + 1),
            Statute::from("code/section/wrong-format/newline-after-region"),
            &format!("{file}#{section_path}"),
            section.start_line + 1,
            0,
            "",
        ));
    }
    if !is_exempt && !section.name.is_empty() && !is_test_or_benchmark_file(file) {
        let mut has_summary = false;
        for line in section_lines.iter().take(section_lines.len().saturating_sub(1)).skip(1) {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if policy_section_start_match(language, trimmed).is_some() || policy_section_end_match(language, trimmed).is_some() {
                break;
            }
            let Some(rest) = trimmed.strip_prefix(prefix) else { break };
            if rest.trim().is_empty() {
                continue;
            }
            has_summary = true;
        }
        if !has_summary {
            breachs.push(create_breach(
                &format!("Section \"{}\" is missing a summary comment in {file}", section.name),
                Statute::from("code/section/missing-summary"),
                &format!("{file}#{}", section.name),
                section.start_line,
                0,
                "",
            ));
        }
    }
    for child in &section.children {
        check_section(child, &section_path, file, content, lines, language, prefix, breachs);
    }
}

fn mark_covered(section: &languages::Section, covered: &mut [bool]) {
    let start = section.start_line.max(1);
    let mut end = section.end_line.max(start);
    if end > covered.len() as i64 {
        end = covered.len() as i64;
    }
    for line in start..=end {
        if line >= 1 {
            covered[line as usize - 1] = true;
        }
    }
    for child in &section.children {
        mark_covered(child, covered);
    }
}

struct OrphanRange {
    start: i64,
    end: i64,
    first_line: String,
    is_comment_block: bool,
}

fn orphan_breachs(file: &str, lines: &[&str], sections: &[languages::Section], language: &languages::Language, prefix: &str) -> Vec<Breach> {
    let mut breachs = Vec::new();
    if is_test_or_benchmark_file(file) {
        return breachs;
    }
    let mut covered = vec![false; lines.len()];
    for section in sections {
        mark_covered(section, &mut covered);
    }
    let mut orphan_lines = vec![false; lines.len()];
    for (index, raw) in lines.iter().enumerate() {
        if covered[index] {
            continue;
        }
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if line.trim().is_empty() {
            continue;
        }
        if index == 0 && line.trim().starts_with("#!") {
            continue;
        }
        if policy_section_start_match(language, line).is_some() || policy_section_end_match(language, line).is_some() {
            continue;
        }
        orphan_lines[index] = true;
    }
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut in_orphan = false;
    let mut start_line = 0i64;
    for (index, flag) in orphan_lines.iter().enumerate() {
        if *flag {
            if !in_orphan {
                in_orphan = true;
                start_line = index as i64 + 1;
            }
        } else if in_orphan {
            ranges.push((start_line, index as i64));
            in_orphan = false;
        }
    }
    if in_orphan {
        ranges.push((start_line, lines.len() as i64));
    }
    let mut def_ranges: Vec<(String, i64, i64)> = Vec::new();
    let mut excerpts: BTreeMap<String, String> = BTreeMap::new();
    if language.supports_definitions() {
        for def in languages::parse_definition_ranges(language, lines) {
            def_ranges.push((def.name.clone(), def.start, def.end));
            excerpts.insert(def.name, def.excerpt);
        }
    }
    for def in languages::extra_orphan_definitions(language, lines) {
        def_ranges.push((def.name.clone(), def.start, def.end));
        excerpts.insert(def.name, def.excerpt);
    }
    let mut infos = Vec::new();
    for (start, end) in ranges {
        let mut first_line = String::new();
        let mut is_comment_block = true;
        for line_index in start..=end {
            let raw = lines[line_index as usize - 1];
            let line = raw.strip_suffix('\r').unwrap_or(raw);
            if line.trim().is_empty() {
                continue;
            }
            if first_line.is_empty() {
                first_line = line.trim().to_string();
            }
            if !line.trim().starts_with(prefix) {
                is_comment_block = false;
            }
        }
        if is_comment_block {
            let name = format!("comment-block-{start}");
            def_ranges.push((name.clone(), start, end));
            excerpts.insert(name, first_line.clone());
        }
        infos.push(OrphanRange { start, end, first_line, is_comment_block });
    }
    let mut reported: BTreeMap<String, bool> = BTreeMap::new();
    for info in &infos {
        let _ = info.is_comment_block;
        let mut matched = false;
        for (name, start, end) in &def_ranges {
            if info.start <= *end && info.end >= *start {
                if !reported.contains_key(name) {
                    reported.insert(name.clone(), true);
                    let excerpt = excerpts.get(name).filter(|v| !v.is_empty()).cloned().unwrap_or_else(|| name.clone());
                    breachs.push(create_breach(
                        &format!("Orphan definition outside sections at {file}:{start}"),
                        Statute::from("code/section/orphan-definition"),
                        &format!("{file}::{name}"),
                        *start,
                        0,
                        &excerpt,
                    ));
                }
                matched = true;
            }
        }
        if matched {
            continue;
        }
        let name = format!("orphan-block-{}", info.start);
        breachs.push(create_breach(
            &format!("Orphan definition outside sections at {file}:{}", info.start),
            Statute::from("code/section/orphan-definition"),
            &format!("{file}::{name}"),
            info.start,
            0,
            &info.first_line,
        ));
    }
    breachs
}

struct DocState {
    has_summary: bool,
    has_requirements: bool,
    is_native: bool,
}

fn scan_jsdoc(lines: &[&str], prev_index: i64) -> Option<DocState> {
    let prev = lines[prev_index as usize].trim();
    if !(prev.ends_with("**/") || prev.ends_with("*/")) {
        return None;
    }
    let mut state = DocState { has_summary: false, has_requirements: false, is_native: true };
    let mut scan = prev_index;
    while scan >= 0 {
        let line = lines[scan as usize].trim();
        let is_open = line.starts_with("/**");
        let mut text = line.to_string();
        if is_open {
            text = text.trim_start_matches("/**").to_string();
        } else if let Some(rest) = text.strip_prefix("* ") {
            text = rest.to_string();
        } else if text == "*" || text == "*/" || text == "**/" {
            scan -= 1;
            continue;
        }
        let mut text = text.trim().to_string();
        if text.ends_with("**/") || text.ends_with("*/") {
            text = text.trim_end_matches("**/").trim_end_matches("*/").trim().to_string();
        }
        if text.is_empty() {
            if is_open {
                break;
            }
            scan -= 1;
            continue;
        }
        if let Some(rest) = text.strip_prefix("* ") {
            text = rest.trim().to_string();
        }
        if is_spec_text(&text) {
            state.has_requirements = true;
        } else {
            state.has_summary = true;
        }
        if is_open {
            break;
        }
        scan -= 1;
    }
    Some(state)
}

fn scan_triple_slash(lines: &[&str], prev_index: i64) -> Option<DocState> {
    if !lines[prev_index as usize].trim().starts_with("///") {
        return None;
    }
    let mut state = DocState { has_summary: false, has_requirements: false, is_native: true };
    let mut scan = prev_index;
    while scan >= 0 {
        let line = lines[scan as usize].trim();
        let Some(rest) = line.strip_prefix("///") else { break };
        let mut text = rest.trim().to_string();
        if text.is_empty() {
            scan -= 1;
            continue;
        }
        text = text.trim_start_matches("<summary>").trim_end_matches("</summary>").trim().to_string();
        if text == "<remarks>" || text == "</remarks>" || text.is_empty() {
            scan -= 1;
            continue;
        }
        if is_spec_text(&text) {
            state.has_requirements = true;
        } else {
            state.has_summary = true;
        }
        scan -= 1;
    }
    Some(state)
}

fn scan_leading_comments(lines: &[&str], start: i64, prefix: &str, state: &mut DocState) {
    let mut index = start - 2;
    while index >= 0 {
        let line = lines[index as usize].trim();
        if line.is_empty() {
            break;
        }
        let Some(rest) = line.strip_prefix(prefix) else { break };
        let text = rest.trim();
        if text.is_empty() {
            index -= 1;
            continue;
        }
        if is_spec_text(text) {
            state.has_requirements = true;
        } else {
            state.has_summary = true;
        }
        index -= 1;
    }
}

fn definition_breachs(file: &str, content: &str, lines: &[&str], language: &languages::Language, prefix: &str) -> Vec<Breach> {
    let mut breachs = Vec::new();
    if is_test_or_benchmark_file(file) || !language.supports_definitions() {
        return breachs;
    }
    let _ = content;
    let lang_name = language.name.as_str();
    for def in languages::parse_definition_ranges(language, lines) {
        let def_line = if def.start >= 1 && (def.start as usize) <= lines.len() { lines[def.start as usize - 1] } else { "" };
        let prev_index = def.start - 2;
        let mut state = DocState { has_summary: false, has_requirements: false, is_native: false };
        if prev_index >= 0 {
            let scanned = match lang_name {
                "typescript" => scan_jsdoc(lines, prev_index),
                "csharp" | "rust" => scan_triple_slash(lines, prev_index),
                _ => None,
            };
            if let Some(scanned) = scanned {
                state = scanned;
            }
        }
        if !state.is_native && lang_name == "python" {
            scan_python_docstring(lines, def.start, &mut state);
            if state.is_native {
                let mut index = def.start - 2;
                while index >= 0 {
                    let line = lines[index as usize].trim();
                    if line.is_empty() {
                        break;
                    }
                    let Some(rest) = line.strip_prefix(prefix) else { break };
                    if !rest.trim().is_empty() {
                        state.is_native = false;
                        break;
                    }
                    index -= 1;
                }
            }
        }
        if !state.is_native {
            if lang_name == "go" {
                state.is_native = true;
            }
            scan_leading_comments(lines, def.start, prefix, &mut state);
        }
        if !state.is_native && (state.has_summary || state.has_requirements) {
            breachs.push(create_breach(
                &format!("Definition \"{}\" is not using native docstring format in {file}:{}", def.name, def.start),
                Statute::from("code/definition/wrong-format/not-native-docstring"),
                &format!("{file}::{}", def.name),
                def.start,
                0,
                &def.name,
            ));
        }
        if !state.has_summary {
            breachs.push(create_breach(
                &format!("Definition \"{}\" is missing a summary comment in {file}:{}", def.name, def.start),
                Statute::from("code/definition/missing-summary"),
                &format!("{file}::{}", def.name),
                def.start,
                0,
                &def.name,
            ));
        }
        if !state.has_requirements && requires_definition_requirements(def_line, lang_name) {
            breachs.push(create_breach(
                &format!("Definition \"{}\" is missing spec comments in {file}:{}", def.name, def.start),
                Statute::from("code/definition/missing-requirements"),
                &format!("{file}::{}", def.name),
                def.start,
                0,
                &def.name,
            ));
        }
    }
    breachs
}

fn scan_python_docstring(lines: &[&str], start: i64, state: &mut DocState) {
    let raw = lines[start as usize - 1];
    let mut depth = 0i64;
    for ch in raw.chars() {
        if ch == '(' {
            depth += 1;
        }
        if ch == ')' {
            depth -= 1;
        }
    }
    let mut body_start = start;
    if depth > 0 {
        let mut scan = start;
        while scan < lines.len() as i64 && scan < start + 15 {
            for ch in lines[scan as usize].chars() {
                if ch == '(' {
                    depth += 1;
                }
                if ch == ')' {
                    depth -= 1;
                }
            }
            if depth <= 0 {
                body_start = scan + 1;
                break;
            }
            scan += 1;
        }
    }
    let mut body = body_start;
    while body < lines.len() as i64 && body < body_start + 5 {
        let trimmed = lines[body as usize].trim();
        if trimmed.is_empty() {
            body += 1;
            continue;
        }
        if trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
            state.is_native = true;
            let quote = if trimmed.starts_with("'''") { "'''" } else { "\"\"\"" };
            let after_open = trimmed.strip_prefix(quote).unwrap_or(trimmed);
            if let Some(close) = after_open.find(quote) {
                let text = after_open[..close].trim();
                if !text.is_empty() {
                    if is_spec_text(text) {
                        state.has_requirements = true;
                    } else {
                        state.has_summary = true;
                    }
                }
            } else {
                let first = after_open.trim();
                if !first.is_empty() {
                    if is_spec_text(first) {
                        state.has_requirements = true;
                    } else {
                        state.has_summary = true;
                    }
                }
                let mut scan = body + 1;
                while scan < lines.len() as i64 {
                    let line = lines[scan as usize].trim();
                    if line == quote {
                        break;
                    }
                    if line.ends_with(quote) {
                        let text = line.trim_end_matches(quote).trim();
                        if !text.is_empty() {
                            if is_spec_text(text) {
                                state.has_requirements = true;
                            } else {
                                state.has_summary = true;
                            }
                        }
                        break;
                    }
                    if !line.is_empty() {
                        if is_spec_text(line) {
                            state.has_requirements = true;
                        } else {
                            state.has_summary = true;
                        }
                    }
                    scan += 1;
                }
            }
        }
        break;
    }
}

/// 📐️ The implementation-syntax breaches of every specification comment of every source.
pub fn requirements_policy(sources: &SourceSet) -> Vec<Breach> {
    let mut breachs = Vec::new();
    for source in sources.files() {
        let file = source.path.as_str();
        let content = source.content.as_str();
        if content.is_empty() {
            continue;
        }
        let Some(language) = languages::language_for_path(file) else { continue };
        if !language.supports_headers {
            continue;
        }
        let lines: Vec<&str> = content.split('\n').collect();
        let sections = languages::parse_sections(content, file);
        let prefix = language.comment_prefix.as_str();
        if let Some(header) = find_header(&sections) {
            let requirements = header.children.iter().find(|c| c.name.to_lowercase() == "requirements");
            match requirements {
                Some(child) => {
                    let mut index = child.start_line + 1;
                    while index < child.end_line && index <= lines.len() as i64 {
                        let line = lines[index as usize - 1].trim();
                        if line.is_empty() {
                            index += 1;
                            continue;
                        }
                        let text = line.strip_prefix(prefix).unwrap_or(line).trim();
                        if text.is_empty() {
                            index += 1;
                            continue;
                        }
                        if let Some(reason) = has_implementation_syntax(text) {
                            breachs.push(create_breach(
                                &format!("Spec contains implementation syntax in {file}:{index} ({reason})"),
                                Statute::from("code/requirements/implementation-syntax"),
                                &format!("{file}#Header/Requirements"),
                                index,
                                0,
                                text,
                            ));
                        }
                        index += 1;
                    }
                }
                None => {
                    let mut index = header.start_line + 1;
                    while index < header.end_line && index <= lines.len() as i64 {
                        let line = lines[index as usize - 1].trim();
                        if line.is_empty() {
                            index += 1;
                            continue;
                        }
                        if let Some(rest) = line.strip_prefix(prefix) {
                            let text = rest.trim();
                            if !text.is_empty() && is_spec_text(text) {
                                if let Some(reason) = has_implementation_syntax(text) {
                                    breachs.push(create_breach(
                                        &format!("Spec contains implementation syntax in {file}:{index} ({reason})"),
                                        Statute::from("code/requirements/implementation-syntax"),
                                        &format!("{file}#Header"),
                                        index,
                                        0,
                                        text,
                                    ));
                                }
                            }
                        }
                        index += 1;
                    }
                }
            }
        }
        for section in &sections {
            check_section_requirements(section, file, &lines, prefix, &mut breachs);
        }
    }
    filter_ignored(breachs, sources)
}

fn check_section_requirements(section: &languages::Section, file: &str, lines: &[&str], prefix: &str, breachs: &mut Vec<Breach>) {
    if section.name.to_lowercase() == "header" {
        return;
    }
    let mut index = section.start_line + 1;
    while index < section.end_line && index <= lines.len() as i64 {
        let line = lines[index as usize - 1].trim();
        if line.is_empty() {
            index += 1;
            continue;
        }
        let Some(rest) = line.strip_prefix(prefix) else { break };
        let text = rest.trim();
        if !is_spec_text(text) {
            break;
        }
        if let Some(reason) = has_implementation_syntax(text) {
            breachs.push(create_breach(
                &format!("Spec contains implementation syntax in {file}:{index} ({reason})"),
                Statute::from("code/requirements/implementation-syntax"),
                &format!("{file}#{}", section.name),
                index,
                0,
                text,
            ));
        }
        index += 1;
    }
    for child in &section.children {
        check_section_requirements(child, file, lines, prefix, breachs);
    }
}

//#endregion 🔍️Analyze

//#region 🩹️Autofix

/// 🩹️ The outcome of applying every autofix transformation to one file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FixedFile {
    pub path: String,
    pub content: String,
    pub fixed: Vec<Statute>,
}

/// 🩹️ Applies every autofixable transformation to one source and reports the statutes it repaired.
pub fn autofix(source: &SourceFile) -> FixedFile {
    let mut content = source.content.clone();
    let mut fixed = Vec::new();
    if let Some(next) = fix_newline_after_region(&source.path, &content) {
        if next != content {
            content = next;
            fixed.push(Statute::from("code/section/wrong-format/newline-after-region"));
        }
    }
    FixedFile { path: source.path.clone(), content, fixed }
}

fn fix_newline_after_region(path: &str, content: &str) -> Option<String> {
    let language = languages::language_for_path(path)?;
    if !language.supports_sections() {
        return None;
    }
    let lines: Vec<&str> = content.split('\n').collect();
    let mut drop = vec![false; lines.len()];
    for (index, raw) in lines.iter().enumerate() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if policy_section_start_match(language, line).is_none() {
            continue;
        }
        let mut next = index + 1;
        while next < lines.len() && lines[next].trim().is_empty() {
            drop[next] = true;
            next += 1;
        }
    }
    let kept: Vec<&str> = lines.iter().enumerate().filter(|(index, _)| !drop[*index]).map(|(_, line)| *line).collect();
    Some(kept.join("\n"))
}

//#endregion 🩹️Autofix

//#region 🔏️Digest

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
    0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// 🔏️ The SHA-256 digest of the input.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut state: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut message = data.to_vec();
    let bit_length = (data.len() as u64) * 8;
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_length.to_be_bytes());
    for chunk in message.chunks(64) {
        let mut w = [0u32; 64];
        for (index, word) in chunk.chunks(4).enumerate() {
            w[index] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7) ^ w[index - 15].rotate_right(18) ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17) ^ w[index - 2].rotate_right(19) ^ (w[index - 2] >> 10);
            w[index] = w[index - 16].wrapping_add(s0).wrapping_add(w[index - 7]).wrapping_add(s1);
        }
        let mut h = state;
        for index in 0..64 {
            let s1 = h[4].rotate_right(6) ^ h[4].rotate_right(11) ^ h[4].rotate_right(25);
            let ch = (h[4] & h[5]) ^ ((!h[4]) & h[6]);
            let temp1 = h[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(SHA256_K[index]).wrapping_add(w[index]);
            let s0 = h[0].rotate_right(2) ^ h[0].rotate_right(13) ^ h[0].rotate_right(22);
            let maj = (h[0] & h[1]) ^ (h[0] & h[2]) ^ (h[1] & h[2]);
            let temp2 = s0.wrapping_add(maj);
            h[7] = h[6];
            h[6] = h[5];
            h[5] = h[4];
            h[4] = h[3].wrapping_add(temp1);
            h[3] = h[2];
            h[2] = h[1];
            h[1] = h[0];
            h[0] = temp1.wrapping_add(temp2);
        }
        for index in 0..8 {
            state[index] = state[index].wrapping_add(h[index]);
        }
    }
    let mut digest = [0u8; 32];
    for (index, word) in state.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

/// 🔏️ The lowercase hexadecimal SHA-256 digest of the input.
pub fn sha256_hex(data: &[u8]) -> String {
    sha256(data).iter().map(|byte| format!("{byte:02x}")).collect()
}

//#endregion 🔏️Digest

//#region 🗜️Deflate

fn crc32(data: &[u8]) -> u32 {
    static TABLE: OnceLock<[u32; 256]> = OnceLock::new();
    let table = TABLE.get_or_init(|| {
        let mut table = [0u32; 256];
        for (index, slot) in table.iter_mut().enumerate() {
            let mut value = index as u32;
            for _ in 0..8 {
                value = if value & 1 == 1 { 0xEDB8_8320 ^ (value >> 1) } else { value >> 1 };
            }
            *slot = value;
        }
        table
    });
    let mut crc = 0xFFFF_FFFFu32;
    for byte in data {
        crc = table[((crc ^ u32::from(*byte)) & 0xFF) as usize] ^ (crc >> 8);
    }
    crc ^ 0xFFFF_FFFF
}

/// 🗜️ The gzip member wrapping the input, written as stored DEFLATE blocks.
pub fn gzip_encode(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x1f, 0x8b, 0x08, 0x00, 0, 0, 0, 0, 0x04, 0xff];
    if data.is_empty() {
        out.extend_from_slice(&[0x01, 0x00, 0x00, 0xff, 0xff]);
    } else {
        let chunks: Vec<&[u8]> = data.chunks(65535).collect();
        for (index, chunk) in chunks.iter().enumerate() {
            let final_block = u8::from(index + 1 == chunks.len());
            out.push(final_block);
            let length = chunk.len() as u16;
            out.extend_from_slice(&length.to_le_bytes());
            out.extend_from_slice(&(!length).to_le_bytes());
            out.extend_from_slice(chunk);
        }
    }
    out.extend_from_slice(&crc32(data).to_le_bytes());
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out
}

struct BitReader<'a> {
    data: &'a [u8],
    position: usize,
    bit: u32,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8], position: usize) -> Self {
        Self { data, position, bit: 0 }
    }

    fn read_bit(&mut self) -> Result<u32, String> {
        let byte = *self.data.get(self.position).ok_or_else(|| "truncated deflate stream".to_string())?;
        let value = (u32::from(byte) >> self.bit) & 1;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.position += 1;
        }
        Ok(value)
    }

    fn read_bits(&mut self, count: u32) -> Result<u32, String> {
        let mut value = 0u32;
        for index in 0..count {
            value |= self.read_bit()? << index;
        }
        Ok(value)
    }

    fn align(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.position += 1;
        }
    }
}

struct Huffman {
    counts: Vec<u16>,
    symbols: Vec<u16>,
}

impl Huffman {
    fn new(lengths: &[u8]) -> Self {
        let max = lengths.iter().copied().max().unwrap_or(0) as usize;
        let mut counts = vec![0u16; max + 1];
        for length in lengths {
            counts[*length as usize] += 1;
        }
        if !counts.is_empty() {
            counts[0] = 0;
        }
        let mut offsets = vec![0u16; max + 2];
        for length in 1..=max {
            offsets[length + 1] = offsets[length] + counts[length];
        }
        let mut symbols = vec![0u16; lengths.len()];
        for (symbol, length) in lengths.iter().enumerate() {
            if *length != 0 {
                symbols[offsets[*length as usize] as usize] = symbol as u16;
                offsets[*length as usize] += 1;
            }
        }
        Self { counts, symbols }
    }

    fn decode(&self, reader: &mut BitReader<'_>) -> Result<u16, String> {
        let mut code = 0i32;
        let mut first = 0i32;
        let mut index = 0i32;
        for length in 1..self.counts.len() {
            code |= reader.read_bit()? as i32;
            let count = i32::from(self.counts[length]);
            if code - first < count {
                return Ok(self.symbols[(index + (code - first)) as usize]);
            }
            index += count;
            first = (first + count) << 1;
            code <<= 1;
        }
        Err("invalid huffman code".to_string())
    }
}

const LENGTH_BASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LENGTH_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE: [u16; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DIST_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
const CODE_LENGTH_ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

fn fixed_trees() -> (Huffman, Huffman) {
    let mut literal_lengths = vec![8u8; 288];
    for length in literal_lengths.iter_mut().take(256).skip(144) {
        *length = 9;
    }
    for length in literal_lengths.iter_mut().take(280).skip(256) {
        *length = 7;
    }
    (Huffman::new(&literal_lengths), Huffman::new(&[5u8; 30]))
}

fn inflate(data: &[u8], start: usize) -> Result<Vec<u8>, String> {
    let mut reader = BitReader::new(data, start);
    let mut out: Vec<u8> = Vec::new();
    loop {
        let final_block = reader.read_bit()?;
        let kind = reader.read_bits(2)?;
        match kind {
            0 => {
                reader.align();
                if reader.position + 4 > data.len() {
                    return Err("truncated stored block".to_string());
                }
                let length = u16::from_le_bytes([data[reader.position], data[reader.position + 1]]) as usize;
                reader.position += 4;
                if reader.position + length > data.len() {
                    return Err("truncated stored payload".to_string());
                }
                out.extend_from_slice(&data[reader.position..reader.position + length]);
                reader.position += length;
            }
            1 | 2 => {
                let (literals, distances) = if kind == 1 {
                    fixed_trees()
                } else {
                    let hlit = reader.read_bits(5)? as usize + 257;
                    let hdist = reader.read_bits(5)? as usize + 1;
                    let hclen = reader.read_bits(4)? as usize + 4;
                    let mut code_lengths = [0u8; 19];
                    for index in 0..hclen {
                        code_lengths[CODE_LENGTH_ORDER[index]] = reader.read_bits(3)? as u8;
                    }
                    let code_tree = Huffman::new(&code_lengths);
                    let mut lengths = vec![0u8; hlit + hdist];
                    let mut index = 0usize;
                    while index < lengths.len() {
                        let symbol = code_tree.decode(&mut reader)?;
                        match symbol {
                            0..=15 => {
                                lengths[index] = symbol as u8;
                                index += 1;
                            }
                            16 => {
                                let previous = if index == 0 { return Err("repeat with no previous length".to_string()) } else { lengths[index - 1] };
                                let repeat = 3 + reader.read_bits(2)? as usize;
                                for _ in 0..repeat {
                                    if index >= lengths.len() {
                                        return Err("length repeat overflow".to_string());
                                    }
                                    lengths[index] = previous;
                                    index += 1;
                                }
                            }
                            17 => {
                                let repeat = 3 + reader.read_bits(3)? as usize;
                                index = (index + repeat).min(lengths.len());
                            }
                            18 => {
                                let repeat = 11 + reader.read_bits(7)? as usize;
                                index = (index + repeat).min(lengths.len());
                            }
                            _ => return Err("invalid code length symbol".to_string()),
                        }
                    }
                    (Huffman::new(&lengths[..hlit]), Huffman::new(&lengths[hlit..]))
                };
                loop {
                    let symbol = literals.decode(&mut reader)?;
                    if symbol == 256 {
                        break;
                    }
                    if symbol < 256 {
                        out.push(symbol as u8);
                        continue;
                    }
                    let index = symbol as usize - 257;
                    if index >= LENGTH_BASE.len() {
                        return Err("invalid length symbol".to_string());
                    }
                    let length = LENGTH_BASE[index] as usize + reader.read_bits(u32::from(LENGTH_EXTRA[index]))? as usize;
                    let dist_symbol = distances.decode(&mut reader)? as usize;
                    if dist_symbol >= DIST_BASE.len() {
                        return Err("invalid distance symbol".to_string());
                    }
                    let distance = DIST_BASE[dist_symbol] as usize + reader.read_bits(u32::from(DIST_EXTRA[dist_symbol]))? as usize;
                    if distance > out.len() {
                        return Err("distance beyond window".to_string());
                    }
                    let from = out.len() - distance;
                    for offset in 0..length {
                        let byte = out[from + offset];
                        out.push(byte);
                    }
                }
            }
            _ => return Err("reserved deflate block type".to_string()),
        }
        if final_block == 1 {
            return Ok(out);
        }
    }
}

/// 🗜️ The payload of a gzip member, verifying its CRC-32 and length trailer.
pub fn gzip_decode(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 18 || data[0] != 0x1f || data[1] != 0x8b {
        return Err("not a gzip member".to_string());
    }
    if data[2] != 8 {
        return Err("unsupported gzip compression method".to_string());
    }
    let flags = data[3];
    let mut cursor = 10usize;
    if flags & 0x04 != 0 {
        let extra = u16::from_le_bytes([data[cursor], data[cursor + 1]]) as usize;
        cursor += 2 + extra;
    }
    if flags & 0x08 != 0 {
        while cursor < data.len() && data[cursor] != 0 {
            cursor += 1;
        }
        cursor += 1;
    }
    if flags & 0x10 != 0 {
        while cursor < data.len() && data[cursor] != 0 {
            cursor += 1;
        }
        cursor += 1;
    }
    if flags & 0x02 != 0 {
        cursor += 2;
    }
    let payload = inflate(data, cursor)?;
    let trailer = &data[data.len() - 8..];
    let expected_crc = u32::from_le_bytes([trailer[0], trailer[1], trailer[2], trailer[3]]);
    let expected_length = u32::from_le_bytes([trailer[4], trailer[5], trailer[6], trailer[7]]);
    if crc32(&payload) != expected_crc {
        return Err("gzip crc mismatch".to_string());
    }
    if payload.len() as u32 != expected_length {
        return Err("gzip length mismatch".to_string());
    }
    Ok(payload)
}

//#endregion 🗜️Deflate

//#region 🧷️Breach Cache

/// 🧷️ The on-disk JSON shape of one breach-cache document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachCacheEnvelope {
    #[serde(rename = "entityId")]
    pub entity_id: String,
    pub script: String,
    pub breachs: Vec<Breach>,
}

/// 🔐️ The digest that keys one breach-cache document, over its canonical JSON encoding.
pub fn breach_cache_digest(envelope: &BreachCacheEnvelope) -> Result<String, String> {
    let encoded = serde_json::to_vec(envelope).map_err(|error| error.to_string())?;
    Ok(sha256_hex(&encoded))
}

/// 📄️ The canonical JSON encoding of one breach-cache document.
pub fn encode_breach_cache_json(envelope: &BreachCacheEnvelope) -> Result<String, String> {
    serde_json::to_string(envelope).map_err(|error| error.to_string())
}

/// 📄️ The breach-cache document one canonical JSON encoding carries.
pub fn parse_breach_cache(document: &str) -> Result<BreachCacheEnvelope, String> {
    serde_json::from_str(document).map_err(|error| error.to_string())
}

/// 🗜️ The compressed breach-cache document: gzip over its canonical JSON encoding.
pub fn encode_breach_cache(envelope: &BreachCacheEnvelope) -> Result<Vec<u8>, String> {
    let encoded = serde_json::to_vec(envelope).map_err(|error| error.to_string())?;
    Ok(gzip_encode(&encoded))
}

/// 📂️ The breach-cache document a compressed member carries.
pub fn decode_breach_cache(data: &[u8]) -> Result<BreachCacheEnvelope, String> {
    let payload = gzip_decode(data)?;
    serde_json::from_slice(&payload).map_err(|error| error.to_string())
}

/// 📂️ Every breach of a set of plain-JSON breach-cache documents, in document order.
pub fn breachs_from_cache(documents: &[String]) -> Result<Vec<Breach>, String> {
    let mut all = Vec::new();
    for document in documents {
        let envelope: BreachCacheEnvelope = serde_json::from_str(document).map_err(|error| error.to_string())?;
        all.extend(envelope.breachs);
    }
    Ok(all)
}

//#endregion 🧷️Breach Cache
