//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🏠️ Repository workspace: monorepo root discovery, the `.🧬semio` layout, `📋️config.toml`
//! settings, and the owned glob and ignore primitives every other repo crate matches paths with.
//! Behaviour twin of `github.com/usalu/semio/repo/workspace`.

//#endregion 🧲️Header

use std::fs;
use std::path::{Path, PathBuf};

//#region 🖍️Pattern

/// 🧩️ One compiled element of a glob pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    /// 🔤️ A literal character that must match exactly.
    Literal(char),
    /// ❓️ Any single character except the path separator.
    AnyOne,
    /// ⭐️ Any run of characters except the path separator.
    Star,
    /// 🌌️ Any run of characters, separators included.
    DoubleStar,
    /// 🛣️ An optional run of characters that ends with a separator.
    DoubleStarSlash,
    /// 🔢️ A character class, negated or not.
    Class { negated: bool, items: Vec<ClassItem> },
}

/// 🔢️ One member of a character class: a single character or an inclusive range.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ClassItem {
    /// 🔤️ A single character.
    One(char),
    /// ↔️ An inclusive character range.
    Range(char, char),
}

/// 🃏️ A compiled glob pattern: one token stream per brace-free alternative it expands to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    alternatives: Vec<Vec<Token>>,
}

/// 🪧️ Bounds brace alternation so a pathological pattern cannot explode; a pattern that would
/// expand past it keeps its braces literal instead.
const MAX_BRACE_EXPANSIONS: usize = 1024;

impl Pattern {
    /// 🛠️ Compiles a slash-separated glob pattern. Brace alternation (`{a,b}`, nested and possibly
    /// empty) stands for the set of brace-free patterns it expands to and matches when any of them
    /// matches. An unpaired brace, and a group without a top-level comma, are literal. Escaping a
    /// brace with a backslash is not a portable spelling: separator normalisation rewrites
    /// backslashes to slashes before the pattern is read.
    pub fn compile(pattern: &str) -> Result<Self, String> {
        let alternatives = expand_braces(pattern).iter().map(|expanded| compile_tokens(expanded)).collect::<Result<Vec<Vec<Token>>, String>>()?;
        Ok(Pattern { alternatives })
    }

    /// 🔍️ Reports whether the whole name is covered by any alternative of the pattern.
    pub fn matches(&self, name: &str) -> bool {
        let text: Vec<char> = to_slash(name).chars().collect();
        self.alternatives.iter().any(|tokens| matches_from(tokens, 0, &text, 0))
    }

    /// 🔤️ The longest literal run every alternative of this pattern requires, or `None` when one
    /// alternative has none.
    ///
    /// A path that matches an alternative necessarily contains that alternative's literal runs, so a
    /// path containing none of the returned literals cannot match the pattern at all. That makes the
    /// answer a sound, exact prefilter — and a cheap one, which is what a 625-rule ignore file
    /// evaluated against half a million historical paths needs.
    pub fn required_literals(&self) -> Option<Vec<String>> {
        let mut literals = Vec::with_capacity(self.alternatives.len());
        for alternative in &self.alternatives {
            let mut longest = String::new();
            let mut current = String::new();
            for token in alternative {
                match token {
                    Token::Literal(value) => current.push(*value),
                    _ => {
                        if current.len() > longest.len() {
                            longest = std::mem::take(&mut current);
                        }
                        current.clear();
                    }
                }
            }
            if current.len() > longest.len() {
                longest = current;
            }
            if longest.is_empty() {
                return None;
            }
            literals.push(longest);
        }
        Some(literals)
    }
}

/// 🪄️ Rewrites brace alternation into the brace-free patterns it stands for.
fn expand_braces(pattern: &str) -> Vec<String> {
    if !pattern.contains('{') {
        return vec![pattern.to_string()];
    }
    let mut expanded: Vec<String> = Vec::new();
    let mut queue: Vec<String> = vec![pattern.to_string()];
    let mut head = 0usize;
    while head < queue.len() {
        let current: Vec<char> = queue[head].chars().collect();
        head += 1;
        match brace_group(&current) {
            None => expanded.push(current.iter().collect()),
            Some((start, end)) => {
                let prefix: String = current[..start].iter().collect();
                let suffix: String = current[end + 1..].iter().collect();
                for alternative in brace_alternatives(&current[start + 1..end]) {
                    queue.push(format!("{prefix}{alternative}{suffix}"));
                }
                if expanded.len() + (queue.len() - head) > MAX_BRACE_EXPANSIONS {
                    return vec![pattern.to_string()];
                }
            }
        }
    }
    expanded
}

/// 🔎️ Returns the char bounds of the first closed brace group carrying a top-level comma.
fn brace_group(chars: &[char]) -> Option<(usize, usize)> {
    let mut index = 0usize;
    while index < chars.len() {
        match chars[index] {
            '\\' => index += 1,
            '{' => {
                if let (Some(end), true) = brace_end(chars, index) {
                    return Some((index, end));
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

/// 🧮️ Scans from an opening brace to its partner, reporting where it closed and whether the body it
/// encloses carries a comma at its own nesting level.
fn brace_end(chars: &[char], start: usize) -> (Option<usize>, bool) {
    let mut depth = 0i32;
    let mut class = false;
    let mut comma = false;
    let mut cursor = start;
    while cursor < chars.len() {
        match chars[cursor] {
            '\\' => cursor += 1,
            '[' => class = true,
            ']' => class = false,
            '{' if !class => depth += 1,
            '}' if !class => {
                depth -= 1;
                if depth == 0 {
                    return (Some(cursor), comma);
                }
            }
            ',' if !class && depth == 1 => comma = true,
            _ => {}
        }
        cursor += 1;
    }
    (None, false)
}

/// ✂️ Splits a brace body on the commas that belong to it rather than to a nested group.
fn brace_alternatives(body: &[char]) -> Vec<String> {
    let mut alternatives = Vec::new();
    let mut depth = 0i32;
    let mut class = false;
    let mut start = 0usize;
    let mut cursor = 0usize;
    while cursor < body.len() {
        match body[cursor] {
            '\\' => cursor += 1,
            '[' => class = true,
            ']' => class = false,
            '{' if !class => depth += 1,
            '}' if !class && depth > 0 => depth -= 1,
            ',' if !class && depth == 0 => {
                alternatives.push(body[start..cursor].iter().collect());
                start = cursor + 1;
            }
            _ => {}
        }
        cursor += 1;
    }
    alternatives.push(body[start..].iter().collect());
    alternatives
}

/// 🛠️ Compiles one brace-free glob pattern into its token stream.
fn compile_tokens(pattern: &str) -> Result<Vec<Token>, String> {
    let normalized = to_slash(pattern);
    let chars: Vec<char> = normalized.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0usize;
    while index < chars.len() {
        match chars[index] {
            '*' => {
                if index + 1 < chars.len() && chars[index + 1] == '*' {
                    index += 1;
                    if index + 1 < chars.len() && chars[index + 1] == '/' {
                        index += 1;
                        tokens.push(Token::DoubleStarSlash);
                    } else {
                        tokens.push(Token::DoubleStar);
                    }
                } else {
                    tokens.push(Token::Star);
                }
            }
            '?' => tokens.push(Token::AnyOne),
            '[' => {
                let end = chars[index + 1..]
                    .iter()
                    .position(|value| *value == ']')
                    .ok_or_else(|| format!("invalid glob {normalized:?}: unclosed character class"))?
                    + index
                    + 1;
                let body: String = chars[index + 1..end].iter().collect();
                tokens.push(compile_class(&body));
                index = end;
            }
            '\\' => {
                if index + 1 >= chars.len() {
                    return Err(format!("invalid glob {normalized:?}: trailing escape"));
                }
                index += 1;
                tokens.push(Token::Literal(chars[index]));
            }
            other => tokens.push(Token::Literal(other)),
        }
        index += 1;
    }
    Ok(tokens)
}

/// 🔢️ Compiles the raw body of a character class, reproducing the Go quoting rules exactly:
/// a `!`-negated class quotes its members so every one of them is literal, while a plain class
/// is handed to the regular-expression engine untouched and therefore keeps `a-z` ranges.
fn compile_class(body: &str) -> Token {
    if let Some(rest) = body.strip_prefix('!') {
        return Token::Class { negated: true, items: rest.chars().map(ClassItem::One).collect() };
    }
    let (negated, rest) = match body.strip_prefix('^') {
        Some(stripped) => (true, stripped),
        None => (false, body),
    };
    let chars: Vec<char> = rest.chars().collect();
    let mut items = Vec::new();
    let mut index = 0usize;
    while index < chars.len() {
        if index + 2 < chars.len() && chars[index + 1] == '-' {
            items.push(ClassItem::Range(chars[index], chars[index + 2]));
            index += 3;
        } else {
            items.push(ClassItem::One(chars[index]));
            index += 1;
        }
    }
    Token::Class { negated, items }
}

/// 🧮️ Backtracks the compiled token stream against the remaining text.
fn matches_from(tokens: &[Token], token_index: usize, text: &[char], text_index: usize) -> bool {
    if token_index == tokens.len() {
        return text_index == text.len();
    }
    match &tokens[token_index] {
        Token::Literal(expected) => {
            text_index < text.len()
                && text[text_index] == *expected
                && matches_from(tokens, token_index + 1, text, text_index + 1)
        }
        Token::AnyOne => {
            text_index < text.len()
                && text[text_index] != '/'
                && matches_from(tokens, token_index + 1, text, text_index + 1)
        }
        Token::Class { negated, items } => {
            text_index < text.len()
                && class_contains(items, text[text_index]) != *negated
                && matches_from(tokens, token_index + 1, text, text_index + 1)
        }
        Token::Star => {
            let mut cursor = text_index;
            loop {
                if matches_from(tokens, token_index + 1, text, cursor) {
                    return true;
                }
                if cursor >= text.len() || text[cursor] == '/' {
                    return false;
                }
                cursor += 1;
            }
        }
        Token::DoubleStar => {
            let mut cursor = text_index;
            loop {
                if matches_from(tokens, token_index + 1, text, cursor) {
                    return true;
                }
                if cursor >= text.len() {
                    return false;
                }
                cursor += 1;
            }
        }
        Token::DoubleStarSlash => {
            if matches_from(tokens, token_index + 1, text, text_index) {
                return true;
            }
            let mut cursor = text_index;
            while cursor < text.len() {
                if text[cursor] == '/' && matches_from(tokens, token_index + 1, text, cursor + 1) {
                    return true;
                }
                cursor += 1;
            }
            false
        }
    }
}

/// 🔎️ Reports whether a character is a member of a compiled class.
fn class_contains(items: &[ClassItem], value: char) -> bool {
    items.iter().any(|item| match item {
        ClassItem::One(one) => *one == value,
        ClassItem::Range(low, high) => *low <= value && value <= *high,
    })
}

/// 🃏️ Reports whether name satisfies a slash-separated glob pattern.
pub fn glob_match(pattern: &str, name: &str) -> Result<bool, String> {
    Ok(Pattern::compile(pattern)?.matches(name))
}

/// ➗️ Rewrites backslashes to forward slashes.
fn to_slash(value: &str) -> String {
    value.replace('\\', "/")
}

//#endregion 🖍️Pattern

//#region 🗂️Traversal

/// 🗺️ Walks the static prefix of the pattern and returns every matching path, sorted.
pub fn filepath_glob(pattern: &str) -> Result<Vec<String>, String> {
    filepath_glob_with(pattern, &mut |_| {})
}

/// 🚦️ Walks the static prefix of the pattern, reporting the number of visited entries.
pub fn filepath_glob_with(pattern: &str, progress: &mut dyn FnMut(usize)) -> Result<Vec<String>, String> {
    let root = traversal_root(pattern);
    if !Path::new(&root).exists() {
        return Ok(Vec::new());
    }
    let compiled = Pattern::compile(pattern)?;
    let mut matches = Vec::new();
    let mut visited = 0usize;
    let mut stack = vec![PathBuf::from(&root)];
    while let Some(current) = stack.pop() {
        visited += 1;
        progress(visited);
        let display = current.to_string_lossy().to_string();
        if compiled.matches(&display) {
            matches.push(display);
        }
        if current.is_dir() {
            let entries = fs::read_dir(&current).map_err(|error| error.to_string())?;
            for entry in entries {
                stack.push(entry.map_err(|error| error.to_string())?.path());
            }
        }
    }
    matches.sort();
    Ok(matches)
}

/// 🌱️ Returns the longest wildcard-free directory prefix of a pattern.
pub fn traversal_root(pattern: &str) -> String {
    let normalized = pattern.replace('\\', "/");
    let absolute = normalized.starts_with('/');
    let volume = windows_volume(&normalized);
    let body = &normalized[volume.len()..];
    let mut root = String::from(volume);
    if absolute || (!volume.is_empty() && body.starts_with('/')) {
        root.push('/');
    }
    let mut collected: Vec<&str> = Vec::new();
    for part in body.trim_start_matches('/').split('/') {
        if part.contains('*') || part.contains('?') || part.contains('[') {
            break;
        }
        collected.push(part);
    }
    root.push_str(&collected.join("/"));
    if root.is_empty() {
        return ".".to_string();
    }
    let path = Path::new(&root);
    if path.is_file() {
        return path.parent().map(|parent| parent.to_string_lossy().to_string()).unwrap_or(root);
    }
    root
}

/// 🪟️ Returns the leading `C:` style volume of a path, or the empty string.
fn windows_volume(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
        &value[..2]
    } else {
        ""
    }
}

//#endregion 🗂️Traversal

//#region 🙈️Ignore

/// 📏️ One compiled ignore line together with its negation flag.
#[derive(Debug, Clone)]
struct Rule {
    pattern: Pattern,
    negated: bool,
    /// 🔎️ The literal runs a path must carry to have any chance of matching, empty when
    /// the pattern admits no such prefilter.
    literals: Vec<String>,
}

/// 🙈️ An ordered ignore rule set in which the last matching rule decides.
#[derive(Debug, Clone, Default)]
pub struct GitIgnore {
    rules: Vec<Rule>,
}

impl GitIgnore {
    /// 📝️ Parses ignore lines already held in memory into an ordered rule set.
    pub fn compile_lines<'a, I: IntoIterator<Item = &'a str>>(lines: I) -> Self {
        let mut result = GitIgnore::default();
        for line in lines {
            result.add_line(line);
        }
        result
    }

    /// 📄️ Parses an ignore file into an ordered rule set.
    pub fn compile_file(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
        Ok(GitIgnore::compile_lines(text.lines()))
    }

    /// ➕️ Normalises one ignore line and appends it unless it is blank or a comment.
    fn add_line(&mut self, raw: &str) {
        let mut line = raw.trim().to_string();
        if line.is_empty() || line.starts_with('#') {
            return;
        }
        let negated = line.starts_with('!');
        if negated {
            line = line[1..].to_string();
        }
        line = to_slash(line.strip_prefix('/').unwrap_or(&line));
        if line.ends_with('/') {
            line.push_str("**");
        }
        if !line.contains('/') {
            line = format!("**/{line}");
        }
        if let Ok(pattern) = Pattern::compile(&line) {
            let literals = pattern.required_literals().unwrap_or_default();
            self.rules.push(Rule { pattern, negated, literals });
        }
    }

    /// 🔍️ Reports whether the last matching rule ignores the path.
    pub fn matches_path(&self, path: &str) -> bool {
        let normalized = to_slash(path);
        let normalized = normalized.strip_prefix("./").unwrap_or(&normalized);
        let mut matched = false;
        for rule in &self.rules {
            if !rule.literals.is_empty() && !rule.literals.iter().any(|literal| normalized.contains(literal.as_str())) {
                continue;
            }
            if rule.pattern.matches(normalized) {
                matched = !rule.negated;
            }
        }
        matched
    }
}

//#endregion 🙈️Ignore

//#region 🧭️Layout

/// 🧬️ The workspace-local semio directory inside a monorepo root.
pub const SEMIO_DIR_NAME: &str = ".🧬semio";

/// 🦑️ The repo product directory inside the semio directory.
pub const REPO_DIR_NAME: &str = "🦑️repo";

/// 🎫️ The ticket collection directory inside the repo meta directory.
pub const TICKETS_DIR_NAME: &str = "🎫️tickets";

/// 🎯️ The goal collection directory inside the repo meta directory.
pub const GOALS_DIR_NAME: &str = "🎯️goals";

/// 🧑️ The contributor collection directory inside the repo meta directory.
pub const DEVS_DIR_NAME: &str = "🧑️‍💻️devs";

/// 📁️ The codebase file index inside the repo meta directory.
pub const FILES_INDEX_NAME: &str = "📁️files.json";

/// ⚙️ The repo settings file inside the repo meta directory.
pub const CONFIG_FILE_NAME: &str = "📋️config.toml";

/// 🧬️ Returns the workspace-local semio root under an explicit monorepo root.
pub fn semio_dir_for_root(repo_root: &Path) -> PathBuf {
    repo_root.join(SEMIO_DIR_NAME)
}

/// 📦️ Returns the repo meta dir for an explicit monorepo root.
pub fn repo_meta_dir_for_root(repo_root: &Path) -> PathBuf {
    repo_root.join(SEMIO_DIR_NAME).join(REPO_DIR_NAME)
}

/// 🛤️ Joins a relative path onto the repo meta dir of an explicit monorepo root.
pub fn repo_meta_path_for_root(repo_root: &Path, path: &str) -> PathBuf {
    repo_meta_dir_for_root(repo_root).join(path)
}

/// 🎫️ Returns the ticket collection directory of an explicit monorepo root.
pub fn tickets_dir_for_root(repo_root: &Path) -> PathBuf {
    repo_meta_path_for_root(repo_root, TICKETS_DIR_NAME)
}

/// 🎯️ Returns the goal collection directory of an explicit monorepo root.
pub fn goals_dir_for_root(repo_root: &Path) -> PathBuf {
    repo_meta_path_for_root(repo_root, GOALS_DIR_NAME)
}

/// 🧑️ Returns the contributor collection directory of an explicit monorepo root.
pub fn devs_dir_for_root(repo_root: &Path) -> PathBuf {
    repo_meta_path_for_root(repo_root, DEVS_DIR_NAME)
}

/// 📁️ Returns the codebase file index path of an explicit monorepo root.
pub fn files_index_for_root(repo_root: &Path) -> PathBuf {
    repo_meta_path_for_root(repo_root, FILES_INDEX_NAME)
}

//#endregion 🧭️Layout

//#region 🧭️Discovery

/// 🧭️ Walks upwards from `start_dir` and returns the first monorepo root recognised, in the
/// order legacy CLI entry point, git checkout, Go module.
pub fn find_repo_root(start_dir: &Path) -> PathBuf {
    let dir = match fs::canonicalize(start_dir) {
        Ok(resolved) => strip_verbatim(&resolved),
        Err(_) => start_dir.to_path_buf(),
    };
    if let Some(found) = ascend(&dir, |candidate| candidate.join("repo").join("cli").join("main.go").is_file()) {
        return found;
    }
    if let Some(found) = ascend(&dir, |candidate| candidate.join(".git").exists()) {
        return found;
    }
    if let Some(found) = ascend(&dir, |candidate| candidate.join("go.mod").exists()) {
        return found;
    }
    dir
}

/// ⬆️ Walks from `dir` to the filesystem root and returns the first accepted directory.
fn ascend(dir: &Path, matcher: impl Fn(&Path) -> bool) -> Option<PathBuf> {
    let mut current = dir.to_path_buf();
    loop {
        if matcher(&current) {
            return Some(current);
        }
        match current.parent() {
            Some(parent) if parent != current => current = parent.to_path_buf(),
            _ => return None,
        }
    }
}

/// 🪟️ Drops the Windows verbatim prefix that canonicalisation adds.
fn strip_verbatim(path: &Path) -> PathBuf {
    let text = path.to_string_lossy().to_string();
    PathBuf::from(text.strip_prefix(r"\\?\").unwrap_or(&text).to_string())
}

//#endregion 🧭️Discovery

//#region ⚙️RepoConfig

/// 📋️ Hook logging switches and detail for `📋️config.toml` `[logging]`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LoggingConfig {
    /// 🎬️ Whether whole-session logging is recorded.
    pub session: bool,
    /// 🔧️ Whether individual operations are recorded.
    pub operations: bool,
    /// 🗺️ Whether plan transitions are recorded.
    pub plan: bool,
    /// 🔬️ How much of each record is kept: `minimal`, `standard` or `full`.
    pub detail: String,
}

/// ⚙️ Repo-wide settings loaded from `.🧬semio/🦑️repo/📋️config.toml`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RepoConfig {
    /// 📋️ The `[logging]` section.
    pub logging: LoggingConfig,
}

impl Default for RepoConfig {
    fn default() -> Self {
        RepoConfig {
            logging: LoggingConfig {
                session: false,
                operations: true,
                plan: true,
                detail: "standard".to_string(),
            },
        }
    }
}

impl LoggingConfig {
    /// 📤️ Reports whether hook logs carry the response payload at this detail level.
    pub fn include_response(&self) -> bool {
        self.detail.trim().to_lowercase() != "minimal"
    }

    /// 🔬️ Reports whether hook logs carry the native payload at this detail level.
    pub fn include_native(&self) -> bool {
        self.detail.trim().to_lowercase() == "full"
    }
}

/// ✅️ Reads the affirmative spellings a config value may use.
fn parse_repo_config_bool(value: &str) -> bool {
    matches!(value.trim().to_lowercase().as_str(), "true" | "yes" | "1" | "on")
}

/// ✂️ Strips one matching pair of surrounding quotes.
fn unquote_repo_config_value(value: &str) -> String {
    let trimmed = value.trim();
    let bytes = trimmed.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }
    trimmed.to_string()
}

/// 📥️ Reads `.🧬semio/🦑️repo/📋️config.toml` under `repo_root`; a missing file yields defaults.
pub fn load_repo_config(repo_root: &Path) -> RepoConfig {
    if repo_root.as_os_str().is_empty() {
        return RepoConfig::default();
    }
    match fs::read_to_string(repo_meta_path_for_root(repo_root, CONFIG_FILE_NAME)) {
        Ok(document) => parse_repo_config(&document),
        Err(_) => RepoConfig::default(),
    }
}

/// 📜️ Applies the recognised `[logging]` keys of a config document onto the defaults.
pub fn parse_repo_config(document: &str) -> RepoConfig {
    let mut config = RepoConfig::default();
    let mut section = String::new();
    for line in document.split('\n') {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            section = trimmed.trim_matches(|value| value == '[' || value == ']').to_lowercase();
            continue;
        }
        let Some((raw_key, raw_value)) = trimmed.split_once('=') else { continue };
        if section != "logging" {
            continue;
        }
        let key = raw_key.trim().to_lowercase();
        let value = unquote_repo_config_value(raw_value);
        match key.as_str() {
            "session" => config.logging.session = parse_repo_config_bool(&value),
            "operations" => config.logging.operations = parse_repo_config_bool(&value),
            "plan" => config.logging.plan = parse_repo_config_bool(&value),
            "detail" if !value.is_empty() => config.logging.detail = value,
            _ => {}
        }
    }
    config
}

//#endregion ⚙️RepoConfig

//#region 🔎️ProgramLookup

/// 🔎️ The executable a program name resolves to on this machine, or `None` when nothing on `PATH`
/// answers to it.
///
/// Twin of Go's `exec.LookPath`, which every process this repository spawns goes through: a name
/// carrying a separator is taken as written, and a bare name is searched along `PATH`, on Windows
/// with every extension `PATHEXT` declares. Without this, `npx`, `npm` and `bun` — which ship as
/// `.cmd` shims — are invisible to a raw `CreateProcess`, and a spawn that Go performs happily
/// fails with "program not found".
pub fn look_path(program: &str) -> Option<PathBuf> {
    let program = program.trim();
    if program.is_empty() {
        return None;
    }
    if program.contains('/') || program.contains('\\') {
        return first_executable(Path::new(program));
    }
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path).find_map(|directory| first_executable(&directory.join(program)))
}

/// ❓️ Whether a program name resolves to an executable on this machine.
pub fn program_exists(program: &str) -> bool {
    look_path(program).is_some()
}

/// 🧩️ The candidate for one base path.
///
/// Go's `lp_windows.go` never accepts an extensionless file: a name already carrying a `PATHEXT`
/// extension is taken as written, and every other name is tried with each `PATHEXT` extension in
/// turn. An extensionless `npx` shell script sitting on a Git Bash `PATH` is therefore skipped, as
/// it is by the reference — `CreateProcess` cannot run one.
fn first_executable(candidate: &Path) -> Option<PathBuf> {
    if !cfg!(windows) {
        return candidate.is_file().then(|| candidate.to_path_buf());
    }
    let extensions: Vec<String> = std::env::var("PATHEXT")
        .unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string())
        .split(';')
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect();
    let declared = candidate.extension().and_then(|value| value.to_str()).unwrap_or_default();
    if !declared.is_empty() && extensions.iter().any(|extension| extension[1..].eq_ignore_ascii_case(declared)) && candidate.is_file() {
        return Some(candidate.to_path_buf());
    }
    for extension in &extensions {
        let mut name = candidate.as_os_str().to_os_string();
        name.push(extension);
        let extended = PathBuf::from(name);
        if extended.is_file() {
            return Some(extended);
        }
    }
    None
}

/// ▶️ A [`std::process::Command`] for a program name, resolved the way Go resolves it.
///
/// A Windows shim that ships as `.cmd` or `.bat` — `npx`, `npm`, `bun`, `uv` — cannot be handed to
/// `CreateProcess` by this runtime, so it is run through `cmd /c` exactly as the shell would; every
/// other program is spawned by the path `PATH` resolved it to, or by its bare name so an
/// unresolvable program fails where the caller expects it to.
pub fn spawn_command(program: &str) -> std::process::Command {
    let Some(target) = look_path(program) else { return std::process::Command::new(program) };
    let batch = target.extension().and_then(|value| value.to_str()).is_some_and(|value| value.eq_ignore_ascii_case("cmd") || value.eq_ignore_ascii_case("bat"));
    if !batch {
        return std::process::Command::new(target);
    }
    let mut command = std::process::Command::new(std::env::var_os("COMSPEC").unwrap_or_else(|| "cmd.exe".into()));
    command.arg("/c").arg(target);
    command
}

//#endregion 🔎️ProgramLookup

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_star_spans_directories() {
        assert!(glob_match("**/*.go", "a/b/two.go").unwrap());
        assert!(glob_match("**/*.go", "root.go").unwrap());
        assert!(!glob_match("*.go", "a/one.go").unwrap());
    }

    #[test]
    fn ignore_precedence_is_last_rule_wins() {
        let ignore = GitIgnore::compile_lines(["build/", "!build/keep.txt"]);
        assert!(ignore.matches_path("build/out.o"));
        assert!(!ignore.matches_path("build/keep.txt"));
    }

    #[test]
    fn missing_config_yields_defaults() {
        let config = parse_repo_config("");
        assert!(!config.logging.session);
        assert!(config.logging.operations);
        assert_eq!(config.logging.detail, "standard");
    }

    #[test]
    fn logging_section_overrides_defaults() {
        let config = parse_repo_config("[logging]\nsession = on\ndetail = \"full\"\n");
        assert!(config.logging.session);
        assert!(config.logging.include_native());
    }
}

//#endregion 🧪️Tests

