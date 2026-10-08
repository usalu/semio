//! 🎮️ The canonical command registry: every runnable thing of the workspace is one entry with an
//! identity, a verb, a label, the parameters it accepts and the launch it resolves to. Entries come
//! from Nx targets, root workspace scripts, the playground catalog, the `metadata.semio.dashboard`
//! declarations of project manifests, the `🎮️commands.json` of open tickets and the repo domain.
//! Nothing else is a command.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📚️inventory/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌳️command-tree/🦀️.rs

use crate::command_tree::RepoAction;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use ui_locale::Locale;

// #region 🔖️Wire
/// 🏷️ What a running task is called; tab text and window title are pure functions of it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskLabel {
    pub verb: String,
    pub owner: Vec<String>,
    pub subject: String,
    pub qualifier: String,
    pub parameters: Vec<(String, String)>,
    pub members: u16,
}

/// 🟢️ The resolved readiness of a process: its output shows `http://<local host>:<port>`, and the
/// ready address is that match followed by `path`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ready {
    pub port: u16,
    pub path: String,
}
// #endregion 🔖️Wire

// #region 🔖️Identity
/// 🗂️ The verbs a command is filed under, in launcher order; anything else is [`FALLBACK_VERB`].
pub const VERBS: &[&str] = &["setup", "start", "dev", "serve", "watch", "activate", "prepare", "run", "build", "package", "test", "smoke", "check", "typecheck", "verify", "gate", "lint", "format", "generate", "publish", "deploy", "preview", "bench", "clean"];
/// 🧺️ The verb of a name that begins with no known verb.
pub const FALLBACK_VERB: &str = "task";
/// 🦑️ The branches the repo domain contributes, listed after every verb.
pub const REPO_VERBS: &[&str] = &["tickets", "goals", "analyze", "tree", "statutes"];
const RESERVED_KINDS: &[&str] = &["playground", "tool", "compound", "group", "script", "ticket"];

/// 🦑️ Whether text after `repo:` is a repo-domain action key rather than a target of a project named `repo`.
fn is_action_key(text: &str) -> bool {
    let scope = |text: &str| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_lowercase());
    matches!(text, "goals.list" | "goals.tree" | "tree.monorepo" | "tree.goal" | "tree.statute" | "tree.territory" | "statutes.catalog")
        || text.strip_prefix("analyze:").is_some_and(scope)
        || ["ticket.show:", "ticket.files:", "ticket.close:", "ticket.reopen:"].iter().any(|head| text.strip_prefix(head).is_some_and(|id| !id.is_empty()))
}

/// 🪪️ The identity of one command, in the grammar of `#/$defs/CommandId`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommandId {
    Target { project: String, target: String, configuration: Option<String> },
    Playground { variant: String },
    Tool { project: String, id: String },
    Compound { project: String, id: String },
    Group { project: String, id: String },
    Script { name: String },
    Ticket { ticket: String, id: String },
    Repo { action: String },
}

impl CommandId {
    /// 🔍️ Reads an identity; the error names what is wrong with the text.
    pub fn parse(text: &str) -> Result<Self, String> {
        if text.is_empty() || text.chars().any(char::is_whitespace) { return Err(format!("command id {text:?} must be one word")); }
        let Some((head, rest)) = text.split_once(':') else { return Err(format!("command id {text:?} needs a kind or a project before `:`")) };
        if head.is_empty() || rest.is_empty() { return Err(format!("command id {text:?} has an empty part")); }
        let owned = |kind: &str| -> Result<(String, String), String> {
            let (project, id) = rest.rsplit_once('/').ok_or_else(|| format!("{kind} id {text:?} must be `{kind}:<project>/<id>`"))?;
            if project.is_empty() || !is_slug(id) { return Err(format!("{kind} id {text:?} must be `{kind}:<project>/<id>`")); }
            Ok((project.to_string(), id.to_string()))
        };
        Ok(match head {
            "playground" => if rest.contains([':', '/']) { return Err(format!("playground id {text:?} must be `playground:<variant>`")) } else { Self::Playground { variant: rest.into() } },
            "tool" => { let (project, id) = owned("tool")?; Self::Tool { project, id } }
            "compound" => { let (project, id) = owned("compound")?; Self::Compound { project, id } }
            "group" => { let (project, id) = owned("group")?; Self::Group { project, id } }
            "script" => Self::Script { name: rest.into() },
            "repo" if is_action_key(rest) => Self::Repo { action: rest.into() },
            "ticket" => {
                let (ticket, id) = owned("ticket")?;
                let parts: Vec<&str> = ticket.split('/').collect();
                if parts.len() != 4 || parts[..3].iter().any(|part| part.len() != 2 || !part.bytes().all(|byte| byte.is_ascii_digit())) || parts[3].is_empty() { return Err(format!("ticket id {text:?} must be `ticket:<YY/MM/DD/SLUG>/<id>`")); }
                Self::Ticket { ticket, id }
            }
            project => match rest.split_once(':') {
                Some((target, configuration)) if !target.is_empty() && !configuration.is_empty() => Self::Target { project: project.into(), target: target.into(), configuration: Some(configuration.into()) },
                Some(_) => return Err(format!("command id {text:?} has an empty part")),
                None => Self::Target { project: project.into(), target: rest.into(), configuration: None },
            },
        })
    }
}

impl std::fmt::Display for CommandId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Target { project, target, configuration: Some(configuration) } => write!(formatter, "{project}:{target}:{configuration}"),
            Self::Target { project, target, configuration: None } => write!(formatter, "{project}:{target}"),
            Self::Playground { variant } => write!(formatter, "playground:{variant}"),
            Self::Tool { project, id } => write!(formatter, "tool:{project}/{id}"),
            Self::Compound { project, id } => write!(formatter, "compound:{project}/{id}"),
            Self::Group { project, id } => write!(formatter, "group:{project}/{id}"),
            Self::Script { name } => write!(formatter, "script:{name}"),
            Self::Ticket { ticket, id } => write!(formatter, "ticket:{ticket}/{id}"),
            Self::Repo { action } => write!(formatter, "repo:{action}"),
        }
    }
}

/// 🧾️ Reads `<id> key=value …` as an identity and its chosen parameters.
pub fn parse_invocation(text: &str) -> Result<(String, Vec<(String, String)>), String> {
    let mut words = split_arguments(text)?.into_iter();
    let id = words.next().ok_or("an invocation starts with a command id")?;
    CommandId::parse(&id)?;
    let parameters = words.map(|word| word.split_once('=').filter(|(key, _)| is_slug(key)).map(|(key, value)| (key.to_string(), value.to_string())).ok_or_else(|| format!("{word:?} is not a `key=value` parameter"))).collect::<Result<_, _>>()?;
    Ok((id, parameters))
}

/// 🖨️ Writes an identity and its chosen parameters as `<id> key=value …`, quoting where needed.
pub fn format_invocation(id: &str, parameters: &[(String, String)]) -> String {
    let quote = |value: &str| if value.is_empty() || value.contains(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == '\\') { format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\"")) } else { value.to_string() };
    std::iter::once(id.to_string()).chain(parameters.iter().map(|(key, value)| format!("{key}={}", quote(value)))).collect::<Vec<_>>().join(" ")
}

/// ✂️ Splits typed extra arguments into words: whitespace separates, single and double quotes
/// group, a backslash keeps the next character outside single quotes.
pub fn split_arguments(text: &str) -> Result<Vec<String>, String> {
    let (mut words, mut word, mut open, mut quote) = (Vec::new(), String::new(), false, None::<char>);
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        match (quote, c) {
            (Some(mark), c) if c == mark => quote = None,
            (Some('\''), c) => word.push(c),
            (_, '\\') => { word.push(chars.next().ok_or("a trailing backslash escapes nothing")?); open = true; }
            (Some(_), c) => word.push(c),
            (None, '"' | '\'') => { quote = Some(c); open = true; }
            (None, c) if c.is_whitespace() => if open { words.push(std::mem::take(&mut word)); open = false; },
            (None, c) => { word.push(c); open = true; }
        }
    }
    if quote.is_some() { return Err("a quote is never closed".into()); }
    if open { words.push(word); }
    Ok(words)
}

/// 🔤️ The verb a name performs: the declared one, else its leading word when that is a known verb.
pub fn verb_of(declared: Option<&str>, name: &str) -> String {
    if let Some(verb) = declared { return verb.to_string(); }
    let key = segment_key(name);
    let head = key.split(|c: char| !c.is_ascii_alphanumeric()).next().unwrap_or_default();
    VERBS.iter().copied().find(|verb| *verb == head || head.strip_suffix('s') == Some(*verb)).unwrap_or(FALLBACK_VERB).to_string()
}

/// 🥇️ Where a verb sorts: launcher order, the fallback, the repo branches, then any declared verb.
pub fn verb_rank(verb: &str) -> usize {
    VERBS.iter().chain([FALLBACK_VERB].iter()).chain(REPO_VERBS).position(|known| *known == verb).unwrap_or(usize::MAX)
}

/// 🧼️ A path component without its leading emoji, lowercased: the stable key of a taxonomy segment.
pub fn segment_key(component: &str) -> String {
    let trimmed = component.trim();
    trimmed[trimmed.find(|c: char| c.is_ascii_alphanumeric()).unwrap_or(trimmed.len())..].to_ascii_lowercase()
}

const TAXONOMY_SKIP_KEYS: &[&str] = &["packages", "modules", "products", "plugins", "artifacts", "standards", "subsets", "extensions", "targets"];

/// 🧭️ One step of a command's place in the launcher tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment { pub key: String, pub label: String }

fn segment(key: impl Into<String>, label: impl Into<String>) -> Segment { Segment { key: key.into(), label: label.into() } }

/// 🏘️ The taxonomy segments of a project root with the structural folders dropped.
pub fn owner_segments(root: &str) -> Vec<Segment> {
    let segments: Vec<Segment> = root.split('/').filter(|part| !part.is_empty() && *part != ".").map(|part| segment(segment_key(part), part)).filter(|part| !part.key.is_empty() && !TAXONOMY_SKIP_KEYS.contains(&part.key.as_str())).collect();
    if segments.is_empty() { vec![segment("workspace", "workspace")] } else { segments }
}

fn short_name(project: &str) -> &str { project.rsplit('/').next().unwrap_or(project) }

fn is_slug(text: &str) -> bool { !text.is_empty() && text.len() <= 64 && text.split(['.', '_', '-']).all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())) }
fn is_verb(text: &str) -> bool { text.len() <= 32 && text.bytes().next().is_some_and(|byte| byte.is_ascii_lowercase()) && text.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') }
fn is_env_name(text: &str) -> bool { text.bytes().next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_') && text.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_') }
fn is_flag(text: &str) -> bool { let name = text.strip_prefix("--").or_else(|| text.strip_prefix('-')).unwrap_or_default(); name.bytes().next().is_some_and(|byte| byte.is_ascii_alphabetic()) && !text.contains(|c: char| c.is_whitespace() || c == '\0') }
fn is_value_flag(text: &str) -> bool { let name = text.strip_suffix('=').unwrap_or(text); is_flag(name) && name.trim_start_matches('-').bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-')) }
fn is_reference(text: &str) -> bool { text.split_once(':').is_some_and(|(head, rest)| !head.is_empty() && !rest.is_empty() && !text.contains(char::is_whitespace) && !["compound", "group", "script"].contains(&head) && !(head == "repo" && is_action_key(rest))) }

/// 🌟️ Whether a project name matches an Nx project pattern, where `*` stands for any run of characters.
fn glob(pattern: &str, name: &str) -> bool {
    match pattern.split_once('*') {
        None => pattern == name,
        Some((head, rest)) => name.strip_prefix(head).is_some_and(|tail| (0..=tail.len()).filter(|at| tail.is_char_boundary(*at)).any(|at| glob(rest, &tail[at..]))),
    }
}
fn is_relative(text: &str) -> bool { !text.starts_with(['/', '~', '\\']) && !text.contains(['\\', '\0']) && !(text.len() > 1 && text.as_bytes()[1] == b':' && text.as_bytes()[0].is_ascii_alphabetic()) && text.split('/').all(|part| part != "..") }

/// 🚫️ The runner variables the dashboard owns; a declaration never stores them.
pub const NOISE_ENV: &[&str] = &["NX_DAEMON", "NX_ISOLATE_PLUGINS", "NX_CACHE_PROJECT_GRAPH", "NX_TUI", "NX_NATIVE_COMMAND_RUNNER", "NX_PLUGIN_NO_TIMEOUTS", "NX_FORCE_REUSE_CACHED_GRAPH", "FORCE_COLOR"];
const ABSOLUTE_ROOTS: &[&str] = &["Users", "home", "root", "private", "tmp", "var", "opt", "usr", "etc", "mnt", "workspaces", "Volumes"];

fn is_absolute_literal(text: &str) -> bool {
    let bytes = text.as_bytes();
    text.starts_with("~/") || text.starts_with("~\\") || (bytes.len() > 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && matches!(bytes[2], b'/' | b'\\')) || text.strip_prefix('/').and_then(|rest| rest.split_once('/')).is_some_and(|(head, _)| ABSOLUTE_ROOTS.contains(&head))
}
// #endregion 🔖️Identity

// #region 🔖️Labels
/// 📐️ The cells a tab may take, glyph and close control included.
pub const TAB_CELLS: usize = 24;
const TAIL_CELLS: usize = 6;

/// 🗣️ The word a verb is shown as; identifiers stay language-neutral.
pub fn verb_text(verb: &str, locale: Locale) -> &str {
    if locale != Locale::De { return verb; }
    match verb {
        "setup" => "einrichten", "start" => "starten", "dev" => "entwickeln", "serve" => "ausliefern", "watch" => "beobachten", "activate" => "aktivieren", "prepare" => "vorbereiten", "run" => "ausführen", "build" => "bauen", "package" => "paketieren", "test" => "testen", "smoke" => "rauchtest", "check" => "prüfen", "typecheck" => "typprüfung", "verify" => "verifizieren", "gate" => "freigabe", "lint" => "linten", "format" => "formatieren", "generate" => "generieren", "publish" => "veröffentlichen", "deploy" => "bereitstellen", "preview" => "vorschau", "bench" => "messen", "clean" => "aufräumen", "task" => "aufgabe", "goals" => "ziele", "analyze" => "analysieren", "tree" => "baum", "statutes" => "statuten",
        other => other,
    }
}

fn cells(text: &str) -> usize { usize::from(ui_tui::tui::text::display_width(text)) }

fn take_cells(text: &str, budget: usize) -> &str {
    let mut used = 0;
    for (index, c) in text.char_indices() {
        let width = cells(c.encode_utf8(&mut [0; 4]));
        if used + width > budget { return &text[..index]; }
        used += width;
    }
    text
}

fn take_cells_back(text: &str, budget: usize) -> &str {
    let mut used = 0;
    for (index, c) in text.char_indices().rev() {
        let width = cells(c.encode_utf8(&mut [0; 4]));
        if used + width > budget { return &text[index + c.len_utf8()..]; }
        used += width;
    }
    text
}

/// 🤏️ Keeps a text within `budget` cells by eliding its middle and keeping its last cells.
pub fn elide_middle(text: &str, budget: usize) -> String {
    if cells(text) <= budget { return text.to_string(); }
    if budget == 0 { return String::new(); }
    let tail = take_cells_back(text, TAIL_CELLS.min(budget.saturating_sub(2)));
    format!("{}…{tail}", take_cells(text, budget - 1 - cells(tail)))
}

/// 🔚️ Keeps a text within `budget` cells by eliding its end.
pub fn elide_end(text: &str, budget: usize) -> String {
    if cells(text) <= budget { return text.to_string(); }
    if budget == 0 { String::new() } else { format!("{}…", take_cells(text, budget - 1)) }
}

/// 📑️ The tab text of a task, `verb subject [qualifier] [×members]`, within `budget` cells: the
/// subject gives way first (middle elided, last cells kept), then the qualifier; the verb never does.
pub fn tab_text(label: &TaskLabel, locale: Locale, budget: usize) -> String {
    let verb = verb_text(&label.verb, locale);
    let members = if label.members > 0 { format!(" ×{}", label.members) } else { String::new() };
    let join = |subject: &str, qualifier: &str| format!("{verb} {subject}{}{qualifier}{members}", if qualifier.is_empty() { "" } else { " " });
    let fixed = cells(verb) + 1 + cells(&members);
    let qualified = if label.qualifier.is_empty() { 0 } else { 1 + cells(&label.qualifier) };
    if fixed + cells(&label.subject) + qualified <= budget { return join(&label.subject, &label.qualifier); }
    let subject = elide_middle(&label.subject, budget.saturating_sub(fixed + qualified).max(TAIL_CELLS + 2));
    if fixed + cells(&subject) + qualified <= budget { return join(&subject, &label.qualifier); }
    let room = budget.saturating_sub(fixed + cells(&subject) + 1);
    let qualifier = if room < 2 { String::new() } else { elide_end(&label.qualifier, room) };
    let text = join(&subject, &qualifier);
    if cells(&text) <= budget { text } else { format!("{verb} {}", elide_end(&text[verb.len() + 1..], budget.saturating_sub(cells(verb) + 1).max(1))) }
}

/// 🧮️ The tab texts of tasks in creation order: a text an earlier task already shows gets ` ·2`,
/// ` ·3`, … and stays within the budget.
pub fn tab_texts(labels: &[&TaskLabel], locale: Locale, budget: usize) -> Vec<String> {
    let plain: Vec<String> = labels.iter().map(|label| tab_text(label, locale, budget)).collect();
    plain.iter().enumerate().map(|(index, text)| match plain[..index].iter().filter(|earlier| *earlier == text).count() {
        0 => text.clone(),
        earlier => { let suffix = format!(" ·{}", earlier + 1); format!("{}{suffix}", tab_text(labels[index], locale, budget.saturating_sub(cells(&suffix)))) }
    }).collect()
}

/// 🪟️ The window title of a task: verb, owner path, subject, qualifier and chosen parameters.
pub fn window_title(label: &TaskLabel, locale: Locale) -> String {
    let mut parts = vec![verb_text(&label.verb, locale).to_string()];
    if !label.owner.is_empty() && label.owner.last() != Some(&label.subject) { parts.push(label.owner.join(" / ")); }
    parts.push(if label.members > 0 { format!("{} ×{}", label.subject, label.members) } else { label.subject.clone() });
    if !label.qualifier.is_empty() { parts.push(label.qualifier.clone()); }
    parts.extend(label.parameters.iter().map(|(key, value)| format!("{key}={value}")));
    parts.join(" · ")
}
// #endregion 🔖️Labels

// #region 🔖️Declarations
/// 🎚️ The three kinds of things chosen at launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterKind { Choice, Text, Flag }

/// 🛑️ Whether stopping one member of a compound stops the others.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stop { Together, #[default] Independent }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AppliesTo { verbs: Option<Vec<String>>, playground: Option<bool> }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ValueDeclaration { id: String, env: Option<BTreeMap<String, String>>, nx_flags: Option<Vec<String>>, args: Option<Vec<String>> }

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ParameterDeclaration {
    id: String,
    kind: ParameterKind,
    applies_to: Option<AppliesTo>,
    default: Option<serde_json::Value>,
    required: Option<bool>,
    values: Option<Vec<ValueDeclaration>>,
    env: Option<BTreeMap<String, String>>,
    nx_flags: Option<Vec<String>>,
    args: Option<Vec<String>>,
    value_env: Option<String>,
    value_flag: Option<String>,
    value_positional: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ReadyDeclaration { port: Option<u32>, port_env: Option<String>, path: Option<String> }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TargetDeclaration { verb: Option<String>, ready: Option<ReadyDeclaration>, #[serde(default)] requires: Vec<String>, #[serde(default)] parameters: Vec<ParameterDeclaration> }

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ToolDeclaration { id: String, verb: Option<String>, command: Vec<String>, #[serde(default)] cwd: String, #[serde(default)] env: BTreeMap<String, String>, #[serde(default)] continuous: bool, ready: Option<ReadyDeclaration>, #[serde(default)] requires: Vec<String>, #[serde(default)] parameters: Vec<ParameterDeclaration> }

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct MemberDeclaration { run: String, #[serde(default)] parameters: BTreeMap<String, serde_json::Value> }

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CompoundDeclaration { id: String, verb: Option<String>, stop: Option<Stop>, members: Vec<MemberDeclaration> }

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GroupDeclaration { id: String, target: String, projects: Vec<String> }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ProjectDeclaration { #[serde(default)] parameters: Vec<ParameterDeclaration>, #[serde(default)] tools: Vec<ToolDeclaration>, #[serde(default)] compounds: Vec<CompoundDeclaration>, #[serde(default)] groups: Vec<GroupDeclaration> }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct TicketDeclaration { #[serde(rename = "$schema")] _schema: Option<String>, #[serde(default)] tools: Vec<ToolDeclaration>, #[serde(default)] compounds: Vec<CompoundDeclaration> }

/// 🚨️ One defect of a declaration or of a reference, with the file that declares it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Problem { pub file: String, pub at: String, pub message: String }

impl std::fmt::Display for Problem {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts = [self.file.as_str(), self.at.as_str(), self.message.as_str()];
        formatter.write_str(&parts.iter().filter(|part| !part.is_empty()).copied().collect::<Vec<_>>().join(": "))
    }
}
// #endregion 🔖️Declarations

// #region 🔖️Facts
/// 📦️ What the sources state, before any command is derived: the serialisable input of [`Registry::build`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Facts {
    pub projects: Vec<ProjectFacts>,
    pub scripts: Vec<String>,
    pub playgrounds: Vec<PlaygroundFacts>,
    pub tickets: Vec<TicketFacts>,
    pub problems: Vec<Problem>,
}

/// 📋️ One Nx project: its root, the file that declares it and its targets.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectFacts {
    pub name: String,
    pub root: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub manifest: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<TargetFacts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<serde_json::Value>,
}

/// 🎯️ One Nx target with the facts the registry reads from it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TargetFacts {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuous: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub configurations: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<serde_json::Value>,
}

/// 🛝️ One playground variant of the generated catalog.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaygroundFacts {
    pub variant: String,
    pub plugin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    pub react: u16,
    pub wgpu: u16,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_react: Vec<u16>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_wgpu: Vec<u16>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,
}

/// 🎫️ One ticket of the index; an open ticket carries its `🎮️commands.json` document.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TicketFacts {
    pub id: String,
    pub open: bool,
    pub title: String,
    pub folder: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands: Option<serde_json::Value>,
}

/// 📄️ The name of a ticket's command document.
pub const TICKET_COMMANDS: &str = "🎮️commands.json";
// #endregion 🔖️Facts

// #region 🔖️Model
/// 🧬️ What kind of thing a command is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind { Target, Playground, Tool, Compound, Group, Script, Ticket, Repo }

/// 🧲️ Where a parameter of a command comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Origin { Axis, Target, Tool, Playground, Configuration, Member }

/// 💥️ What a chosen value does to the launch: `env` joins the environment, `nx_flags` go before
/// `--`, `args` after it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Effect { pub env: Vec<(String, String)>, pub nx_flags: Vec<String>, pub args: Vec<String> }

/// 📮️ Where typed text goes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Destination { #[default] Token, Env(String), Flag(String), Positional }

/// 🎛️ One parameter a command accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub id: String,
    pub kind: ParameterKind,
    pub origin: Origin,
    pub default: Option<String>,
    pub required: bool,
    pub values: Vec<(String, Effect)>,
    pub effect: Effect,
    pub destination: Destination,
    verbs: Option<Vec<String>>,
    playground: bool,
    scoped: bool,
}

impl Parameter {
    fn choice(id: &str, origin: Origin, default: Option<&str>, values: Vec<(String, Effect)>) -> Self { Self { id: id.into(), kind: ParameterKind::Choice, origin, default: default.map(str::to_string), required: false, values, effect: Effect::default(), destination: Destination::Token, verbs: None, playground: false, scoped: false } }

    /// 🎯️ Whether a global axis is offered on a command.
    fn applies(&self, verb: &str, playground: bool) -> bool {
        if playground { return self.playground; }
        !self.scoped || self.verbs.as_ref().is_some_and(|verbs| verbs.iter().any(|known| known == verb))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Value { text: String, stated: bool }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Text { Declared(String), Literal(String) }

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReadySpec { port: Option<u16>, port_env: Option<String>, path: String }

#[derive(Debug, Clone, PartialEq, Eq)]
struct Member { run: String, pins: Vec<(String, String)> }

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Target { project: String, target: String, configurations: Vec<String>, ready: Option<ReadySpec>, requires: Vec<String> },
    Group { target: String, projects: Vec<String> },
    Script { name: String },
    Playground(PlaygroundFacts),
    Tool { command: Vec<String>, cwd: String, env: Vec<(String, String)>, ready: Option<ReadySpec>, requires: Vec<String> },
    Compound { members: Vec<Member>, stop: Stop },
    Repo(RepoAction),
}

/// 📇️ One command of the registry.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub id: String,
    pub kind: Kind,
    pub verb: String,
    pub path: Vec<Segment>,
    pub label: String,
    pub owner: Vec<String>,
    pub subject: String,
    pub qualifier: String,
    pub long_running: bool,
    /// 👁️ Whether the command belongs to the default searchable set; a closed ticket's does not.
    pub listed: bool,
    /// ⚠️ Whether running the command changes repository state and deserves a confirmation.
    pub mutating: bool,
    pub source: String,
    own: Vec<Parameter>,
    action: Action,
    haystack: String,
}

/// ▶️ One process of a resolved launch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchProcess {
    pub command_id: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
    pub label: TaskLabel,
    pub ready: Option<Ready>,
    pub long_running: bool,
}

/// 🚀️ `(id, chosen parameters, extra arguments)` resolved: the processes in start order, the
/// services that must be ready first and how the group stops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Launch {
    pub command_id: String,
    pub label: TaskLabel,
    pub processes: Vec<LaunchProcess>,
    pub requires: Vec<Launch>,
    pub group: Option<String>,
    pub stop: Stop,
}
// #endregion 🔖️Model

// #region 🔖️RunPolicy
/// 🧮️ The variable that lets Nx start a task from its published project graph instead of rebuilding it.
pub const GRAPH_REUSE: (&str, &str) = ("NX_FORCE_REUSE_CACHED_GRAPH", "true");
/// 🖥️ The variables that keep terminal rendering and process lifecycle with the dashboard.
pub const RUNNER_ENV: &[(&str, &str)] = &[("NX_NATIVE_COMMAND_RUNNER", "false"), ("NX_TUI", "false")];
const GRAPH_FILE: &str = ".nx/workspace-data/project-graph.json";
const FILE_MAP: &str = ".nx/workspace-data/file-map.json";
/// 🧱️ The workspace files besides project manifests that define the project graph.
pub const GRAPH_OWNERS: &[&str] = &["nx.json", "package.json"];

/// ⚖️ What the dashboard decides for every process it starts, in one place.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RunPolicy {
    /// ♻️ Finite Nx tasks start from the published project graph.
    pub reuse_published_graph: bool,
}

impl RunPolicy {
    fn apply(self, env: &mut Vec<(String, String)>, nx: bool, long_running: bool) {
        for (key, value) in RUNNER_ENV { set_env(env, key, value); }
        if nx && !long_running && self.reuse_published_graph { set_env(env, GRAPH_REUSE.0, GRAPH_REUSE.1); }
    }
}

fn set_env(env: &mut Vec<(String, String)>, key: &str, value: &str) {
    match env.iter_mut().find(|(name, _)| name == key) { Some(pair) => pair.1 = value.to_string(), None => env.push((key.to_string(), value.to_string())) }
}

/// 🔏️ The length and modification instant of a file: equal fingerprints mean an unchanged file,
/// whatever the clock did in between.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fingerprint { pub len: u64, pub modified: u64 }

impl Fingerprint {
    /// 🔬️ Reads the fingerprint of a path; a missing path has none.
    pub fn of(path: &Path) -> Option<Self> {
        let metadata = std::fs::metadata(path).ok()?;
        Some(Self { len: if metadata.is_dir() { 0 } else { metadata.len() }, modified: metadata.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok()?.as_nanos() as u64 })
    }
}

/// 🧾️ What Nx recorded when it published its project graph: the content hash of every file that
/// defines projects. The graph is current while those files still hash to the recorded values.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphBasis { pub graph: Fingerprint, pub file_map: Fingerprint, pub hashes: BTreeMap<String, u64> }

impl GraphBasis {
    /// 📖️ Reads the recorded hashes of `files` from Nx's file map next to the published graph.
    pub fn read(root: &Path, files: &[String]) -> Option<Self> {
        let (graph, file_map) = (Fingerprint::of(&root.join(GRAPH_FILE))?, Fingerprint::of(&root.join(FILE_MAP))?);
        let bytes = std::fs::read(root.join(FILE_MAP)).ok()?;
        let wanted: HashSet<&str> = files.iter().map(String::as_str).collect();
        let hashes = std::cell::RefCell::new(BTreeMap::new());
        serde::de::DeserializeSeed::deserialize(FileMapSeed { depth: 0, wanted: &wanted, hashes: &hashes }, &mut serde_json::Deserializer::from_slice(&bytes)).ok()?;
        (Fingerprint::of(&root.join(FILE_MAP)) == Some(file_map)).then(|| Self { graph, file_map, hashes: hashes.into_inner() })
    }
}

#[derive(Clone, Copy)]
struct FileMapSeed<'a> { depth: u8, wanted: &'a HashSet<&'a str>, hashes: &'a std::cell::RefCell<BTreeMap<String, u64>> }

#[derive(Deserialize)]
struct FileRow<'a> { #[serde(borrow)] file: std::borrow::Cow<'a, str>, #[serde(borrow)] hash: std::borrow::Cow<'a, str> }

impl<'de> serde::de::DeserializeSeed<'de> for FileMapSeed<'_> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> { deserializer.deserialize_any(self) }
}

impl<'de> serde::de::Visitor<'de> for FileMapSeed<'_> {
    type Value = ();
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("the Nx file map") }
    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        while let Some(key) = map.next_key::<std::borrow::Cow<'de, str>>()? {
            let descend = match self.depth { 0 => key == "fileMap", 1 => key == "projectFileMap" || key == "nonProjectFiles", _ => true };
            if descend { map.next_value_seed(Self { depth: self.depth + 1, ..self })?; } else { map.next_value::<serde::de::IgnoredAny>()?; }
        }
        Ok(())
    }
    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut rows: A) -> Result<(), A::Error> {
        while let Some(row) = rows.next_element::<FileRow<'de>>()? {
            if self.wanted.contains(row.file.as_ref()) { if let Ok(hash) = row.hash.parse() { self.hashes.borrow_mut().insert(row.file.into_owned(), hash); } }
        }
        Ok(())
    }
}
// #endregion 🔖️RunPolicy

// #region 🔖️Hash
const XXH_SECRET: [u8; 192] = [
    0xb8, 0xfe, 0x6c, 0x39, 0x23, 0xa4, 0x4b, 0xbe, 0x7c, 0x01, 0x81, 0x2c, 0xf7, 0x21, 0xad, 0x1c, 0xde, 0xd4, 0x6d, 0xe9, 0x83, 0x90, 0x97, 0xdb, 0x72, 0x40, 0xa4, 0xa4, 0xb7, 0xb3, 0x67, 0x1f,
    0xcb, 0x79, 0xe6, 0x4e, 0xcc, 0xc0, 0xe5, 0x78, 0x82, 0x5a, 0xd0, 0x7d, 0xcc, 0xff, 0x72, 0x21, 0xb8, 0x08, 0x46, 0x74, 0xf7, 0x43, 0x24, 0x8e, 0xe0, 0x35, 0x90, 0xe6, 0x81, 0x3a, 0x26, 0x4c,
    0x3c, 0x28, 0x52, 0xbb, 0x91, 0xc3, 0x00, 0xcb, 0x88, 0xd0, 0x65, 0x8b, 0x1b, 0x53, 0x2e, 0xa3, 0x71, 0x64, 0x48, 0x97, 0xa2, 0x0d, 0xf9, 0x4e, 0x38, 0x19, 0xef, 0x46, 0xa9, 0xde, 0xac, 0xd8,
    0xa8, 0xfa, 0x76, 0x3f, 0xe3, 0x9c, 0x34, 0x3f, 0xf9, 0xdc, 0xbb, 0xc7, 0xc7, 0x0b, 0x4f, 0x1d, 0x8a, 0x51, 0xe0, 0x4b, 0xcd, 0xb4, 0x59, 0x31, 0xc8, 0x9f, 0x7e, 0xc9, 0xd9, 0x78, 0x73, 0x64,
    0xea, 0xc5, 0xac, 0x83, 0x34, 0xd3, 0xeb, 0xc3, 0xc5, 0x81, 0xa0, 0xff, 0xfa, 0x13, 0x63, 0xeb, 0x17, 0x0d, 0xdd, 0x51, 0xb7, 0xf0, 0xda, 0x49, 0xd3, 0x16, 0x55, 0x26, 0x29, 0xd4, 0x68, 0x9e,
    0x2b, 0x16, 0xbe, 0x58, 0x7d, 0x47, 0xa1, 0xfc, 0x8f, 0xf8, 0xb8, 0xd1, 0x7a, 0xd0, 0x31, 0xce, 0x45, 0xcb, 0x3a, 0x8f, 0x95, 0x16, 0x04, 0x28, 0xaf, 0xd7, 0xfb, 0xca, 0xbb, 0x4b, 0x40, 0x7e,
];
const XXH_PRIME32: [u64; 3] = [0x9E37_79B1, 0x85EB_CA77, 0xC2B2_AE3D];
const XXH_PRIME64: [u64; 5] = [0x9E37_79B1_85EB_CA87, 0xC2B2_AE3D_27D4_EB4F, 0x1656_67B1_9E37_79F9, 0x85EB_CA77_C2B2_AE63, 0x27D4_EB2F_1656_67C5];
const XXH_MIX: [u64; 2] = [0x1656_6791_9E37_79F9, 0x9FB2_1C65_1E98_DF25];

/// #️⃣ The 64-bit XXH3 hash of a byte string with the default secret and seed 0: the content
/// hash Nx records per workspace file.
///
/// @see https://github.com/Cyan4973/xxHash/blob/dev/doc/xxhash_spec.md
pub fn xxh3_64(input: &[u8]) -> u64 {
    let word = |bytes: &[u8], at: usize| u64::from_le_bytes(bytes[at..at + 8].try_into().expect("eight bytes"));
    let half = |bytes: &[u8], at: usize| u64::from(u32::from_le_bytes(bytes[at..at + 4].try_into().expect("four bytes")));
    let fold = |left: u64, right: u64| { let product = u128::from(left) * u128::from(right); product as u64 ^ (product >> 64) as u64 };
    let avalanche = |mut hash: u64| { hash ^= hash >> 37; hash = hash.wrapping_mul(XXH_MIX[0]); hash ^ hash >> 32 };
    let mix = |at: usize, secret: usize| fold(word(input, at) ^ word(&XXH_SECRET, secret), word(input, at + 8) ^ word(&XXH_SECRET, secret + 8));
    let len = input.len();
    let seeded = (len as u64).wrapping_mul(XXH_PRIME64[0]);
    match len {
        0 => { let mut hash = word(&XXH_SECRET, 56) ^ word(&XXH_SECRET, 64); hash ^= hash >> 33; hash = hash.wrapping_mul(XXH_PRIME64[1]); hash ^= hash >> 29; hash = hash.wrapping_mul(XXH_PRIME64[2]); hash ^ hash >> 32 }
        1..=3 => {
            let combined = u64::from(input[0]) << 16 | u64::from(input[len >> 1]) << 24 | u64::from(input[len - 1]) | (len as u64) << 8;
            let mut hash = combined ^ (half(&XXH_SECRET, 0) ^ half(&XXH_SECRET, 4));
            hash ^= hash >> 33; hash = hash.wrapping_mul(XXH_PRIME64[1]); hash ^= hash >> 29; hash = hash.wrapping_mul(XXH_PRIME64[2]); hash ^ hash >> 32
        }
        4..=8 => {
            let mut hash = (half(input, len - 4) + (half(input, 0) << 32)) ^ (word(&XXH_SECRET, 8) ^ word(&XXH_SECRET, 16));
            hash ^= hash.rotate_left(49) ^ hash.rotate_left(24); hash = hash.wrapping_mul(XXH_MIX[1]); hash ^= (hash >> 35).wrapping_add(len as u64); hash = hash.wrapping_mul(XXH_MIX[1]); hash ^ hash >> 28
        }
        9..=16 => {
            let low = word(input, 0) ^ (word(&XXH_SECRET, 24) ^ word(&XXH_SECRET, 32));
            let high = word(input, len - 8) ^ (word(&XXH_SECRET, 40) ^ word(&XXH_SECRET, 48));
            avalanche((len as u64).wrapping_add(low.swap_bytes()).wrapping_add(high).wrapping_add(fold(low, high)))
        }
        17..=128 => {
            let mut hash = seeded;
            for round in (0..(len - 1) / 32 + 1).rev() { hash = hash.wrapping_add(mix(16 * round, 32 * round)).wrapping_add(mix(len - 16 * (round + 1), 32 * round + 16)); }
            avalanche(hash)
        }
        129..=240 => {
            let mut hash = seeded;
            for round in 0..8 { hash = hash.wrapping_add(mix(16 * round, 16 * round)); }
            hash = avalanche(hash);
            for round in 8..len / 16 { hash = hash.wrapping_add(mix(16 * round, 16 * (round - 8) + 3)); }
            avalanche(hash.wrapping_add(mix(len - 16, 119)))
        }
        _ => {
            let mut lanes = [XXH_PRIME32[2], XXH_PRIME64[0], XXH_PRIME64[1], XXH_PRIME64[2], XXH_PRIME64[3], XXH_PRIME32[1], XXH_PRIME64[4], XXH_PRIME32[0]];
            let stripe = |lanes: &mut [u64; 8], at: usize, secret: usize| for lane in 0..8 {
                let value = word(input, at + 8 * lane);
                let keyed = value ^ word(&XXH_SECRET, secret + 8 * lane);
                lanes[lane ^ 1] = lanes[lane ^ 1].wrapping_add(value);
                lanes[lane] = lanes[lane].wrapping_add((keyed & 0xFFFF_FFFF).wrapping_mul(keyed >> 32));
            };
            let blocks = (len - 1) / 1024;
            for block in 0..blocks {
                for index in 0..16 { stripe(&mut lanes, block * 1024 + index * 64, index * 8); }
                for lane in 0..8 { lanes[lane] = ((lanes[lane] ^ lanes[lane] >> 47) ^ word(&XXH_SECRET, 128 + 8 * lane)).wrapping_mul(XXH_PRIME32[0]); }
            }
            for index in 0..(len - 1 - blocks * 1024) / 64 { stripe(&mut lanes, blocks * 1024 + index * 64, index * 8); }
            stripe(&mut lanes, len - 64, 121);
            avalanche((0..4).fold(seeded, |hash, pair| hash.wrapping_add(fold(lanes[2 * pair] ^ word(&XXH_SECRET, 11 + 16 * pair), lanes[2 * pair + 1] ^ word(&XXH_SECRET, 19 + 16 * pair)))))
        }
    }
}
// #endregion 🔖️Hash

// #region 🔖️Tokens
/// 🧩️ The `{name}` tokens of a text, in order of appearance.
pub fn tokens(text: &str) -> Vec<&str> {
    let (mut found, mut rest) = (Vec::new(), text);
    while let Some(start) = rest.find('{') {
        match rest[start + 1..].find('}').map(|end| &rest[start + 1..start + 1 + end]) {
            Some(name) if is_slug(name) => { found.push(name); rest = &rest[start + name.len() + 2..]; }
            _ => rest = &rest[start + 1..],
        }
    }
    found
}

fn substitute(text: &str, value: &dyn Fn(&str) -> Result<String, String>) -> Result<String, String> {
    let (mut output, mut rest) = (String::with_capacity(text.len()), text);
    while let Some(start) = rest.find('{') {
        output.push_str(&rest[..start]);
        match rest[start + 1..].find('}').map(|end| &rest[start + 1..start + 1 + end]) {
            Some(name) if is_slug(name) => { output.push_str(&value(name)?); rest = &rest[start + name.len() + 2..]; }
            _ => { output.push('{'); rest = &rest[start + 1..]; }
        }
    }
    output.push_str(rest);
    Ok(output)
}
// #endregion 🔖️Tokens

// #region 🔖️Validation
struct Scope<'a> { file: &'a str, at: String, problems: &'a mut Vec<Problem> }

impl Scope<'_> {
    fn report(&mut self, message: impl Into<String>) { self.problems.push(Problem { file: self.file.to_string(), at: self.at.clone(), message: message.into() }); }
    fn within(&mut self, part: impl std::fmt::Display) -> Scope<'_> { Scope { file: self.file, at: if self.at.is_empty() { part.to_string() } else { format!("{}.{part}", self.at) }, problems: self.problems } }
    fn text(&mut self, what: &str, value: &str) { if value.contains('\0') || is_absolute_literal(value) { self.report(format!("{what} {value:?} is an absolute path or carries a NUL; write `{{workspace}}/…`")); } }
    fn env(&mut self, env: &BTreeMap<String, String>) -> Vec<(String, String)> {
        for (name, value) in env {
            if !is_env_name(name) { self.report(format!("environment name {name:?} is not a variable name")); }
            if NOISE_ENV.contains(&name.as_str()) { self.report(format!("{name} belongs to the dashboard's run policy and is never declared")); }
            self.text("environment value", value);
        }
        env.iter().map(|(name, value)| (name.clone(), value.clone())).collect()
    }
    fn effect(&mut self, env: Option<&BTreeMap<String, String>>, nx_flags: Option<&Vec<String>>, args: Option<&Vec<String>>) -> Effect {
        for flag in nx_flags.into_iter().flatten() { if !is_flag(flag) { self.report(format!("nxFlags entry {flag:?} is not a flag")); } }
        for arg in args.into_iter().flatten() { self.text("argument", arg); }
        Effect { env: env.map(|env| self.env(env)).unwrap_or_default(), nx_flags: nx_flags.cloned().unwrap_or_default(), args: args.cloned().unwrap_or_default() }
    }
    fn verb(&mut self, verb: Option<&String>) { if let Some(verb) = verb { if !is_verb(verb) { self.report(format!("verb {verb:?} must be a lowercase word")); } } }
    fn id(&mut self, id: &str) -> bool { if is_slug(id) { true } else { self.report(format!("id {id:?} must be lowercase words joined by `-`, `.` or `_`")); false } }
    fn references(&mut self, what: &str, references: &[String]) {
        for (index, reference) in references.iter().enumerate() {
            if !is_reference(reference) { self.report(format!("{what} {reference:?} must be `<project>:<target>[:configuration]`, `playground:<variant>`, `tool:<project>/<id>` or `ticket:<ticket id>/<id>`")); }
            if references[..index].contains(reference) { self.report(format!("{what} {reference:?} is listed twice")); }
        }
    }
    fn ready(&mut self, ready: Option<&ReadyDeclaration>) -> Option<ReadySpec> {
        let ready = ready?;
        let mut scope = self.within("ready");
        if ready.port.is_none() && ready.port_env.is_none() { scope.report("needs `port` or `portEnv`"); }
        if ready.port.is_some_and(|port| port == 0 || port > 65535) { scope.report("`port` must be between 1 and 65535"); }
        if let Some(name) = &ready.port_env { if !is_env_name(name) && !is_slug(name) { scope.report(format!("portEnv {name:?} names neither a variable nor a parameter")); } }
        if let Some(path) = &ready.path { if !path.starts_with(['/', '?', '#']) || path.contains(|c: char| c.is_whitespace() || c == '\0') { scope.report(format!("path {path:?} must start with `/`, `?` or `#` and contain no whitespace")); } }
        Some(ReadySpec { port: ready.port.and_then(|port| u16::try_from(port).ok()).filter(|port| *port > 0), port_env: ready.port_env.clone(), path: ready.path.clone().unwrap_or_default() })
    }
    fn parameters(&mut self, declarations: &[ParameterDeclaration], origin: Origin) -> Vec<Parameter> {
        let mut parameters: Vec<Parameter> = Vec::new();
        for (index, declaration) in declarations.iter().enumerate() {
            let mut scope = self.within(format!("parameters[{index}]"));
            let before = scope.problems.len();
            scope.id(&declaration.id);
            if ["workspace", "port"].contains(&declaration.id.as_str()) { scope.report(format!("parameter id {:?} is a built-in token", declaration.id)); }
            if parameters.iter().any(|known| known.id == declaration.id) { scope.report(format!("parameter {:?} is declared twice", declaration.id)); }
            if declaration.applies_to.is_some() && origin != Origin::Axis { scope.report("`appliesTo` belongs to the global axes of the workspace root project"); }
            let applies = declaration.applies_to.clone().unwrap_or_default();
            if declaration.applies_to.is_some() && applies.verbs.is_none() && applies.playground.is_none() { scope.report("`appliesTo` needs `verbs` or `playground`"); }
            if declaration.applies_to.is_some() && applies.verbs.is_none() && applies.playground == Some(false) { scope.report("`appliesTo` selects nothing"); }
            for verb in applies.verbs.iter().flatten() { if !is_verb(verb) { scope.report(format!("verb {verb:?} must be a lowercase word")); } }
            if applies.verbs.as_ref().is_some_and(Vec::is_empty) { scope.report("`appliesTo.verbs` lists no verb"); }
            let forbid = |scope: &mut Scope<'_>, kind: &str, members: &[(&str, bool)]| for (name, present) in members { if *present { scope.report(format!("a {kind} has no `{name}`")); } };
            let (effects, values, destinations) = ([("env", declaration.env.is_some()), ("nxFlags", declaration.nx_flags.is_some()), ("args", declaration.args.is_some())], [("values", declaration.values.is_some())], [("valueEnv", declaration.value_env.is_some()), ("valueFlag", declaration.value_flag.is_some()), ("valuePositional", declaration.value_positional.is_some())]);
            let mut parameter = Parameter { id: declaration.id.clone(), kind: declaration.kind, origin, default: None, required: declaration.required.unwrap_or(false), values: Vec::new(), effect: Effect::default(), destination: Destination::Token, verbs: applies.verbs.clone(), playground: applies.playground.unwrap_or(false), scoped: declaration.applies_to.is_some() };
            match declaration.kind {
                ParameterKind::Choice => {
                    forbid(&mut scope, "choice", &effects); forbid(&mut scope, "choice", &destinations);
                    let values = declaration.values.clone().unwrap_or_default();
                    if values.is_empty() { scope.report("a choice needs `values`"); }
                    for (position, value) in values.iter().enumerate() {
                        let mut scope = scope.within(format!("values[{position}]"));
                        if value.id.is_empty() || value.id.len() > 128 || value.id.contains(|c: char| c.is_whitespace() || c == '=' || c == '\0') { scope.report(format!("value id {:?} must be one word without `=`", value.id)); }
                        if values[..position].iter().any(|known| known.id == value.id) { scope.report(format!("value {:?} is declared twice", value.id)); }
                        let effect = scope.effect(value.env.as_ref(), value.nx_flags.as_ref(), value.args.as_ref());
                        parameter.values.push((value.id.clone(), effect));
                    }
                    match &declaration.default { None => {} Some(serde_json::Value::String(value)) if values.iter().any(|known| &known.id == value) => parameter.default = Some(value.clone()), Some(other) => scope.report(format!("default {other} is not one of the values")) }
                }
                ParameterKind::Flag => {
                    forbid(&mut scope, "flag", &values); forbid(&mut scope, "flag", &destinations);
                    if effects.iter().all(|(_, present)| !present) { scope.report("a flag needs `env`, `nxFlags` or `args`"); }
                    parameter.effect = scope.effect(declaration.env.as_ref(), declaration.nx_flags.as_ref(), declaration.args.as_ref());
                    match &declaration.default { None | Some(serde_json::Value::Bool(false)) => {} Some(serde_json::Value::Bool(true)) => parameter.default = Some("true".into()), Some(other) => scope.report(format!("default {other} must be `true` or `false`")) }
                }
                ParameterKind::Text => {
                    forbid(&mut scope, "text", &effects); forbid(&mut scope, "text", &values);
                    if destinations.iter().filter(|(_, present)| *present).count() > 1 { scope.report("a text goes to one of `valueEnv`, `valueFlag` or `valuePositional`"); }
                    if declaration.value_positional == Some(false) { scope.report("`valuePositional` is `true` or absent"); }
                    parameter.destination = match (&declaration.value_env, &declaration.value_flag, declaration.value_positional) {
                        (Some(name), _, _) => { if !is_env_name(name) { scope.report(format!("valueEnv {name:?} is not a variable name")); } if NOISE_ENV.contains(&name.as_str()) { scope.report(format!("{name} belongs to the dashboard's run policy and is never declared")); } Destination::Env(name.clone()) }
                        (_, Some(flag), _) => { if !is_value_flag(flag) { scope.report(format!("valueFlag {flag:?} is not a flag")); } Destination::Flag(flag.clone()) }
                        (_, _, Some(true)) => Destination::Positional,
                        _ => Destination::Token,
                    };
                    match &declaration.default { None => {} Some(serde_json::Value::String(value)) => { scope.text("default", value); parameter.default = Some(value.clone()); } Some(other) => scope.report(format!("default {other} must be text")) }
                }
            }
            if scope.problems.len() == before { parameters.push(parameter); }
        }
        parameters
    }
}

fn decode<T: serde::de::DeserializeOwned>(file: &str, at: &str, value: &serde_json::Value, problems: &mut Vec<Problem>) -> Option<T> {
    serde_json::from_value(value.clone()).map_err(|error| problems.push(Problem { file: file.to_string(), at: at.to_string(), message: error.to_string() })).ok()
}

fn pin_text(value: &serde_json::Value) -> String { value.as_str().map_or_else(|| value.to_string(), str::to_string) }
// #endregion 🔖️Validation

// #region 🔖️Registry
/// 📚️ Every command of one workspace, searchable and resolvable.
#[derive(Debug, Default)]
pub struct Registry {
    root: PathBuf,
    entries: Vec<Entry>,
    index: HashMap<String, usize>,
    axes: Vec<Parameter>,
    axes_source: String,
    problems: Vec<Problem>,
    definitions: Vec<String>,
    basis: std::sync::Mutex<Option<GraphBasis>>,
}

/// 🎠️ The generated catalog the playground commands come from.
pub const PLAYGROUND_SOURCE: &str = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🚀️playgrounds.json";
const RENDERERS: &[&str] = &["react", "wgpu-wasm", "wgpu-native"];
const PLAYGROUND_PARAMETERS: &[&str] = &["renderer", "example", "user-slot", "app-role", "language", "terminology", "appearance"];
const CONFIGURATION: &str = "configuration";
const LABEL_CELLS: usize = 72;

/// 🧵️ The Nx target that runs a playground variant with a renderer.
pub fn playground_target(variant: &str, renderer: &str) -> Option<String> {
    match renderer { "react" => Some(format!("dev-{variant}-react-dev")), "wgpu" | "wgpu-wasm" => Some(format!("dev-{variant}-wgpu-dev")), "wgpu-native" => Some(format!("run-{variant}-native-dev")), _ => None }
}

impl Registry {
    /// 🏗️ Derives every command from what the sources state. Invalid declarations become
    /// [`Registry::problems`] that name their file; everything valid stays runnable.
    pub fn build(root: &Path, facts: &Facts) -> Self {
        let mut problems = facts.problems.clone();
        let mut entries: Vec<Entry> = Vec::new();
        let (mut axes, mut axes_source) = (Vec::new(), String::new());
        let source = |project: &ProjectFacts| if project.manifest.is_empty() { format!("{GRAPH_FILE} (project {})", project.name) } else { project.manifest.clone() };
        for project in &facts.projects {
            let file = source(project);
            if RESERVED_KINDS.contains(&project.name.as_str()) || project.name.contains(|c: char| c == ':' || c.is_whitespace()) { problems.push(Problem { file: file.clone(), at: "name".into(), message: format!("project name {:?} cannot be told from a command kind", project.name) }); continue; }
            let owner = owner_segments(&project.root);
            let keys: Vec<String> = owner.iter().map(|part| part.key.clone()).collect();
            for target in &project.targets {
                let at = format!("targets.{}.metadata.semio.dashboard", target.name);
                let declaration: TargetDeclaration = target.dashboard.as_ref().and_then(|value| decode(&file, &at, value, &mut problems)).unwrap_or_default();
                let mut scope = Scope { file: &file, at, problems: &mut problems };
                scope.verb(declaration.verb.as_ref());
                scope.references("requires", &declaration.requires);
                let ready = scope.ready(declaration.ready.as_ref());
                let own = scope.parameters(&declaration.parameters, Origin::Target);
                let verb = verb_of(declaration.verb.as_deref().filter(|verb| is_verb(verb)), &target.name);
                let mut path = owner.clone();
                path.push(segment(format!(":{}", target.name), target.name.as_str()));
                let qualifier = target.name.strip_prefix(verb.as_str()).map_or(target.name.as_str(), |rest| rest.trim_start_matches(['-', ':', '_'])).to_string();
                entries.push(Entry::new(format!("{}:{}", project.name, target.name), Kind::Target, verb, path, keys.clone(), short_name(&project.name).into(), qualifier, target.continuous.unwrap_or(false) || ready.is_some(), &file, own, Action::Target { project: project.name.clone(), target: target.name.clone(), configurations: target.configurations.clone(), ready, requires: declaration.requires }));
            }
            let Some(declaration) = project.dashboard.as_ref().and_then(|value| decode::<ProjectDeclaration>(&file, "metadata.semio.dashboard", value, &mut problems)) else { continue };
            let mut scope = Scope { file: &file, at: "metadata.semio.dashboard".into(), problems: &mut problems };
            if !declaration.parameters.is_empty() {
                if project.root == "." || project.root.is_empty() { axes = scope.parameters(&declaration.parameters, Origin::Axis); axes_source.clone_from(&file); } else { scope.report("`parameters` are the global axes and belong to the workspace root project"); }
            }
            Self::tools(&mut scope, &mut entries, &declaration.tools, &owner, &keys, &|id| format!("tool:{}/{id}", project.name), Kind::Tool);
            Self::compounds(&mut scope, &mut entries, &declaration.compounds, &owner, &keys, &|id| format!("compound:{}/{id}", project.name), Kind::Compound);
            for (index, group) in declaration.groups.iter().enumerate() {
                let mut scope = scope.within(format!("groups[{index}]"));
                let valid = scope.id(&group.id);
                let word = |text: &str| !text.is_empty() && !text.contains(|c: char| c.is_whitespace() || c == ',');
                if !word(&group.target) { scope.report(format!("target {:?} must be one target name", group.target)); }
                if group.projects.is_empty() { scope.report("a group needs `projects`"); }
                for (position, name) in group.projects.iter().enumerate() { if !word(name) || group.projects[..position].contains(name) { scope.report(format!("project {name:?} must be one project name, listed once")); } }
                if !valid { continue; }
                let verb = verb_of(None, &group.target);
                let mut path = owner.clone();
                path.push(segment(format!("group:{}", group.id), group.id.as_str()));
                let qualifier = group.target.strip_prefix(verb.as_str()).map_or(group.target.as_str(), |rest| rest.trim_start_matches(['-', ':', '_'])).to_string();
                entries.push(Entry::new(format!("group:{}/{}", project.name, group.id), Kind::Group, verb, path, keys.clone(), group.id.clone(), qualifier, false, &file, Vec::new(), Action::Group { target: group.target.clone(), projects: group.projects.clone() }));
            }
        }
        for name in &facts.scripts {
            let mut parts = name.split(':');
            let head = parts.next().unwrap_or(name);
            let verb = verb_of(None, head);
            let mut rest: Vec<&str> = parts.collect();
            if verb != head { rest.insert(0, head); }
            let mut path = vec![segment("scripts", "workspace scripts")];
            if rest.is_empty() { path.push(segment("workspace", "workspace")); } else { path.extend(rest.iter().map(|part| segment(*part, *part))); }
            path.push(segment("run", "▶ run"));
            entries.push(Entry::new(format!("script:{name}"), Kind::Script, verb, path, vec!["scripts".into()], if rest.is_empty() { "workspace".into() } else { rest.join(":") }, String::new(), false, "package.json", Vec::new(), Action::Script { name: name.clone() }));
        }
        for playground in &facts.playgrounds {
            let path = vec![segment(segment_key(&playground.plugin), playground.plugin.as_str()), segment(segment_key(&playground.variant), playground.variant.as_str())];
            entries.push(Entry::new(format!("playground:{}", playground.variant), Kind::Playground, "dev".into(), path, vec![segment_key(&playground.plugin)], playground.variant.clone(), String::new(), true, PLAYGROUND_SOURCE, Vec::new(), Action::Playground(playground.clone())));
        }
        for ticket in &facts.tickets {
            let head = segment(ticket.id.as_str(), format!("{} [{}] {}", ticket.id, if ticket.open { "open" } else { "closed" }, elide_end(&ticket.title, LABEL_CELLS)));
            let actions = [("show", RepoAction::TicketShow { id: ticket.id.clone() }, false), ("files", RepoAction::TicketFiles { id: ticket.id.clone() }, false), if ticket.open { ("close", RepoAction::TicketClose { id: ticket.id.clone() }, true) } else { ("reopen", RepoAction::TicketReopen { id: ticket.id.clone() }, true) }];
            for (key, action, mutating) in actions {
                let mut entry = Entry::new(format!("repo:{}", crate::command_tree::action_key(&action)), Kind::Repo, "tickets".into(), vec![head.clone(), segment(key, key)], Vec::new(), ticket.id.clone(), key.into(), false, "", Vec::new(), Action::Repo(action));
                (entry.listed, entry.mutating) = (ticket.open, mutating);
                entries.push(entry);
            }
            let (Some(document), true) = (&ticket.commands, ticket.open) else { continue };
            let file = format!("{}/{TICKET_COMMANDS}", ticket.folder);
            let Some(declaration) = decode::<TicketDeclaration>(&file, "", document, &mut problems) else { continue };
            let mut scope = Scope { file: &file, at: String::new(), problems: &mut problems };
            let (owner, keys) = (vec![segment("tickets", "tickets"), segment(ticket.id.as_str(), ticket.id.as_str())], vec!["tickets".to_string(), ticket.id.clone()]);
            Self::tools(&mut scope, &mut entries, &declaration.tools, &owner, &keys, &|id| format!("ticket:{}/{id}", ticket.id), Kind::Ticket);
            Self::compounds(&mut scope, &mut entries, &declaration.compounds, &owner, &keys, &|id| format!("ticket:{}/{id}", ticket.id), Kind::Ticket);
        }
        let fixed = [("goals", "list", RepoAction::GoalsList), ("goals", "tree", RepoAction::GoalsTree), ("tree", "monorepo", RepoAction::TreeMonorepo), ("tree", "goal", RepoAction::TreeGoal), ("tree", "statute", RepoAction::TreeStatute), ("tree", "territory", RepoAction::TreeTerritory), ("statutes", "catalog", RepoAction::StatutesCatalog)];
        let scopes = crate::command_tree::ANALYZE_SCOPES.iter().map(|(scope, _)| ("analyze", *scope, RepoAction::Analyze { scope: (*scope).to_string() }));
        for (verb, key, action) in fixed.into_iter().chain(scopes) {
            entries.push(Entry::new(format!("repo:{}", crate::command_tree::action_key(&action)), Kind::Repo, verb.into(), vec![segment(key, key)], Vec::new(), key.into(), String::new(), false, "", Vec::new(), Action::Repo(action)));
        }
        entries.sort_by(|left, right| verb_rank(&left.verb).cmp(&verb_rank(&right.verb)).then_with(|| left.verb.cmp(&right.verb)).then_with(|| left.label.cmp(&right.label)).then_with(|| left.id.cmp(&right.id)));
        let mut index = HashMap::with_capacity(entries.len());
        entries.retain(|entry| {
            let fresh = !index.contains_key(&entry.id);
            if fresh { index.insert(entry.id.clone(), index.len()); } else { problems.push(Problem { file: entry.source.clone(), at: entry.id.clone(), message: "command id is declared twice".into() }); }
            fresh
        });
        let mut definitions: Vec<String> = facts.projects.iter().filter(|project| !project.manifest.is_empty()).map(|project| project.manifest.clone()).chain(GRAPH_OWNERS.iter().map(|owner| (*owner).to_string())).collect();
        definitions.sort(); definitions.dedup();
        let mut registry = Self { root: root.to_path_buf(), entries, index, axes, axes_source, problems, definitions, basis: std::sync::Mutex::new(None) };
        registry.settle();
        registry
    }

    fn tools(scope: &mut Scope<'_>, entries: &mut Vec<Entry>, tools: &[ToolDeclaration], owner: &[Segment], keys: &[String], id_of: &dyn Fn(&str) -> String, kind: Kind) {
        for (index, tool) in tools.iter().enumerate() {
            let mut scope = scope.within(format!("tools[{index}]"));
            let valid = scope.id(&tool.id);
            scope.verb(tool.verb.as_ref());
            if tool.command.is_empty() || tool.command[0].is_empty() { scope.report("a tool needs a `command`"); }
            for word in &tool.command { scope.text("command word", word); }
            if !is_relative(&tool.cwd) { scope.report(format!("cwd {:?} must be workspace-relative", tool.cwd)); }
            scope.references("requires", &tool.requires);
            let env = scope.env(&tool.env);
            let ready = scope.ready(tool.ready.as_ref());
            let own = scope.parameters(&tool.parameters, Origin::Tool);
            for parameter in &own { if !parameter.effect.nx_flags.is_empty() || parameter.values.iter().any(|(_, effect)| !effect.nx_flags.is_empty()) { scope.report(format!("parameter {:?} declares nxFlags, which only commands Nx runs accept", parameter.id)); } }
            if !valid || tool.command.first().is_none_or(String::is_empty) { continue; }
            let mut path = owner.to_vec();
            path.push(segment(format!("tool:{}", tool.id), tool.id.as_str()));
            entries.push(Entry::new(id_of(&tool.id), kind, verb_of(tool.verb.as_deref().filter(|verb| is_verb(verb)), &tool.id), path, keys.to_vec(), tool.id.clone(), String::new(), tool.continuous || ready.is_some(), scope.file, own, Action::Tool { command: tool.command.clone(), cwd: tool.cwd.clone(), env, ready, requires: tool.requires.clone() }));
        }
    }

    fn compounds(scope: &mut Scope<'_>, entries: &mut Vec<Entry>, compounds: &[CompoundDeclaration], owner: &[Segment], keys: &[String], id_of: &dyn Fn(&str) -> String, kind: Kind) {
        for (index, compound) in compounds.iter().enumerate() {
            let mut scope = scope.within(format!("compounds[{index}]"));
            let valid = scope.id(&compound.id);
            scope.verb(compound.verb.as_ref());
            if compound.members.is_empty() { scope.report("a compound needs `members`"); }
            let mut members = Vec::new();
            for (position, member) in compound.members.iter().enumerate() {
                let mut scope = scope.within(format!("members[{position}]"));
                scope.references("member", std::slice::from_ref(&member.run));
                for (key, value) in &member.parameters {
                    if !is_slug(key) { scope.report(format!("parameter name {key:?} must be a parameter id")); }
                    if !(value.is_string() || value.is_boolean() || value.is_i64() || value.is_u64()) { scope.report(format!("parameter {key:?} must be text, a number or a boolean")); }
                    if let Some(text) = value.as_str() { scope.text("parameter value", text); }
                }
                members.push(Member { run: member.run.clone(), pins: member.parameters.iter().map(|(key, value)| (key.clone(), pin_text(value))).collect() });
            }
            if !valid || members.is_empty() { continue; }
            let mut path = owner.to_vec();
            path.push(segment(format!("compound:{}", compound.id), compound.id.as_str()));
            let verb = compound.verb.clone().filter(|verb| is_verb(verb)).unwrap_or_else(|| match verb_of(None, &compound.id) { verb if verb == FALLBACK_VERB => String::new(), verb => verb });
            entries.push(Entry::new(id_of(&compound.id), kind, verb, path, keys.to_vec(), compound.id.clone(), String::new(), false, scope.file, Vec::new(), Action::Compound { members, stop: compound.stop.unwrap_or_default() }));
        }
    }

    /// 🔗️ Settles what depends on other commands: a compound's verb and duration follow its
    /// members, every reference and token must name something that exists.
    fn settle(&mut self) {
        let mut problems = Vec::new();
        let mut names: Vec<&str> = if self.entries.iter().any(|entry| matches!(entry.action, Action::Group { .. })) { self.entries.iter().filter_map(|entry| match &entry.action { Action::Target { project, .. } => Some(project.as_str()), _ => None }).collect() } else { Vec::new() };
        names.sort_unstable(); names.dedup();
        let names: Vec<String> = names.into_iter().map(str::to_string).collect();
        let names: Vec<&str> = names.iter().map(String::as_str).collect();
        for position in 0..self.entries.len() {
            let entry = &self.entries[position];
            let mut report = |at: String, message: String| problems.push(Problem { file: entry.source.clone(), at, message });
            let mut update = None;
            match &entry.action {
                Action::Compound { members, .. } => {
                    let found: Vec<Option<&Entry>> = members.iter().map(|member| self.member(&member.run).map(|(entry, _)| entry)).collect();
                    for (member, found) in members.iter().zip(&found) {
                        match found {
                            None => report(entry.id.clone(), format!("member {:?} names no command", member.run)),
                            Some(target) if matches!(target.action, Action::Compound { .. }) => report(entry.id.clone(), format!("member {:?} is a compound; compounds do not nest", member.run)),
                            Some(target) => { let accepted = self.parameters(target); for (key, _) in &member.pins { if !accepted.iter().any(|parameter| &parameter.id == key) { report(entry.id.clone(), format!("member {:?} has no parameter {key:?}", member.run)); } } }
                        }
                    }
                    let verb = if entry.verb.is_empty() { found.iter().flatten().next().map_or_else(|| FALLBACK_VERB.to_string(), |first| first.verb.clone()) } else { entry.verb.clone() };
                    update = Some((verb, found.iter().flatten().any(|member| member.long_running)));
                }
                Action::Group { target, projects } => {
                    let excluded = |name: &str| projects.iter().filter_map(|pattern| pattern.strip_prefix('!')).any(|pattern| glob(pattern, name));
                    let mut continuous = false;
                    for pattern in projects {
                        let matched: Vec<&str> = names.iter().copied().filter(|name| glob(pattern.trim_start_matches('!'), name)).collect();
                        if matched.is_empty() { report(entry.id.clone(), format!("project {pattern:?} names no project")); continue; }
                        if pattern.starts_with('!') { continue; }
                        let members: Vec<&Entry> = matched.iter().filter(|name| !excluded(name)).filter_map(|name| self.find(&format!("{name}:{target}"))).collect();
                        if members.is_empty() { report(entry.id.clone(), format!("project {pattern:?} has no target {target:?}")); }
                        continuous |= members.iter().any(|member| member.long_running);
                    }
                    update = Some((entry.verb.clone(), continuous));
                }
                Action::Target { ready, requires, .. } | Action::Tool { ready, requires, .. } => {
                    for reference in requires { if self.member(reference).is_none() { report(entry.id.clone(), format!("requires {reference:?} names no command")); } }
                    let accepted = self.parameters(entry);
                    let known = |name: &str| name == "workspace" || accepted.iter().any(|parameter| parameter.id == name);
                    let mut texts: Vec<&String> = accepted.iter().flat_map(|parameter| parameter.values.iter().map(|(_, effect)| effect).chain([&parameter.effect])).flat_map(|effect| effect.env.iter().map(|(_, value)| value).chain(&effect.args)).collect();
                    if let Action::Tool { command, cwd, env, .. } = &entry.action { texts.extend(command.iter().chain([cwd]).chain(env.iter().map(|(_, value)| value))); }
                    for text in texts { for token in tokens(text) { if token == "port" { if ready.is_none() { report(entry.id.clone(), "`{port}` needs a `ready` declaration".into()); } } else if !known(token) { report(entry.id.clone(), format!("token `{{{token}}}` names no parameter of this command")); } } }
                    for default in accepted.iter().filter_map(|parameter| parameter.default.as_ref()) { for token in tokens(default) { if !known(token) { report(entry.id.clone(), format!("token `{{{token}}}` in a default names no parameter of this command")); } } }
                    for parameter in entry.own.iter().filter(|parameter| parameter.kind == ParameterKind::Text && parameter.destination == Destination::Token) {
                        let mut places: Vec<&String> = accepted.iter().flat_map(|other| other.values.iter().map(|(_, effect)| effect).chain([&other.effect])).flat_map(|effect| effect.env.iter().map(|(_, value)| value).chain(&effect.args)).chain(accepted.iter().filter_map(|other| other.default.as_ref())).collect();
                        if let Action::Tool { command, cwd, env, .. } = &entry.action { places.extend(command.iter().chain([cwd]).chain(env.iter().map(|(_, value)| value))); }
                        let named = ready.as_ref().is_some_and(|ready| ready.port_env.as_deref() == Some(parameter.id.as_str()));
                        if !named && !places.iter().any(|text| tokens(text).contains(&parameter.id.as_str())) { report(entry.id.clone(), format!("text parameter {:?} goes nowhere: declare `valueEnv`, `valueFlag`, `valuePositional` or use `{{{}}}`", parameter.id, parameter.id)); }
                    }
                    if let Some(ReadySpec { port: None, port_env: Some(name), .. }) = ready {
                        let produced = accepted.iter().any(|parameter| &parameter.id == name || parameter.destination == Destination::Env(name.clone()) || parameter.values.iter().map(|(_, effect)| effect).chain([&parameter.effect]).any(|effect| effect.env.iter().any(|(key, _)| key == name)));
                        let stated = matches!(&entry.action, Action::Tool { env, .. } if env.iter().any(|(key, _)| key == name));
                        if !produced && !stated { report(entry.id.clone(), format!("ready.portEnv {name:?} names no parameter and no declared variable, and no `port` is given")); }
                    }
                }
                _ => {}
            }
            if let Some((verb, long_running)) = update {
                let entry = &mut self.entries[position];
                if entry.verb != verb { entry.verb = verb; entry.relabel(); }
                entry.long_running = long_running;
            }
        }
        for axis in &self.axes { if PLAYGROUND_PARAMETERS.contains(&axis.id.as_str()) || axis.id == CONFIGURATION { problems.push(Problem { file: self.axes_source.clone(), at: format!("metadata.semio.dashboard.parameters.{}", axis.id), message: "the id is a built-in parameter".into() }); } }
        self.problems.extend(problems);
        let order = |entry: &Entry| (verb_rank(&entry.verb), entry.verb.clone(), entry.label.clone(), entry.id.clone());
        if !self.entries.is_sorted_by_key(order) { self.entries.sort_by_key(order); self.index = self.entries.iter().enumerate().map(|(position, entry)| (entry.id.clone(), position)).collect(); }
    }

    /// 🌍️ The workspace root the commands run in.
    pub fn root(&self) -> &Path { &self.root }
    /// 📜️ Every command, sorted by verb in launcher order and then by label.
    pub fn entries(&self) -> &[Entry] { &self.entries }
    /// 🚨️ Every invalid declaration and dangling reference, each naming its file.
    pub fn problems(&self) -> &[Problem] { &self.problems }
    /// 🧾️ The workspace-relative files that define the project graph: every manifest plus the workspace configuration.
    pub fn definitions(&self) -> &[String] { &self.definitions }
    /// 🌐️ The global axes the workspace root project declares.
    pub fn axes(&self) -> &[Parameter] { &self.axes }

    /// 🔎️ The command with this identity. `<project>:<target>:<configuration>` finds its target.
    pub fn find(&self, id: &str) -> Option<&Entry> { self.member(id).map(|(entry, _)| entry) }

    fn member<'a, 'b>(&'a self, reference: &'b str) -> Option<(&'a Entry, Option<&'b str>)> {
        if let Some(position) = self.index.get(reference) { return Some((&self.entries[*position], None)); }
        let (head, configuration) = reference.rsplit_once(':')?;
        let entry = &self.entries[*self.index.get(head)?];
        matches!(&entry.action, Action::Target { configurations, .. } if configurations.iter().any(|known| known == configuration)).then_some((entry, Some(configuration)))
    }

    /// 🔦️ The positions of the commands whose label or identity contains every word, in registry
    /// order. Closed tickets only take part when `unlisted` is set.
    pub fn search(&self, words: &[&str], unlisted: bool) -> Vec<usize> {
        let needles: Vec<String> = words.iter().flat_map(|word| word.split_whitespace()).map(str::to_lowercase).collect();
        self.entries.iter().enumerate().filter(|(_, entry)| (unlisted || entry.listed) && needles.iter().all(|needle| entry.haystack.contains(needle.as_str()))).map(|(position, _)| position).collect()
    }

    /// 🎛️ Every parameter a command accepts, in the order their effects apply: the global axes
    /// that apply to it, its own, then the built-in ones.
    pub fn parameters(&self, entry: &Entry) -> Vec<Parameter> {
        let axes = |playground: bool, own: &[Parameter]| self.axes.iter().filter(|axis| axis.applies(&entry.verb, playground) && !own.iter().any(|parameter| parameter.id == axis.id) && !PLAYGROUND_PARAMETERS.contains(&axis.id.as_str()) && axis.id != CONFIGURATION).cloned().collect::<Vec<_>>();
        match &entry.action {
            Action::Target { configurations, .. } => {
                let mut parameters = axes(false, &entry.own);
                parameters.extend(entry.own.iter().cloned());
                if !configurations.is_empty() && !entry.own.iter().any(|parameter| parameter.id == CONFIGURATION) { parameters.push(Parameter::choice(CONFIGURATION, Origin::Configuration, None, configurations.iter().map(|name| (name.clone(), Effect::default())).collect())); }
                parameters
            }
            Action::Group { .. } => axes(false, &[]),
            Action::Tool { .. } => entry.own.clone(),
            Action::Playground(playground) => {
                let plain = |values: &[&str]| values.iter().map(|value| ((*value).to_string(), Effect::default())).collect::<Vec<_>>();
                let mut parameters = vec![Parameter::choice("renderer", Origin::Playground, Some(RENDERERS[0]), plain(RENDERERS))];
                if !playground.examples.is_empty() { parameters.push(Parameter::choice("example", Origin::Playground, None, playground.examples.iter().map(|example| (example.clone(), Effect::default())).collect())); }
                let slots = playground.user_react.len().max(playground.user_wgpu.len());
                if slots > 0 { parameters.push(Parameter::choice("user-slot", Origin::Playground, None, (1..=slots).map(|slot| (slot.to_string(), Effect::default())).collect())); }
                parameters.push(Parameter::choice("app-role", Origin::Playground, None, vec![("viewer".into(), Effect { env: vec![("SEMIO_APP_ROLE".into(), "viewer".into())], ..Default::default() })]));
                parameters.push(Parameter::choice("language", Origin::Playground, None, Locale::ALL.iter().map(|locale| (locale.as_str().to_string(), Effect::default())).collect()));
                parameters.push(Parameter::choice("terminology", Origin::Playground, None, plain(&["native", "reuse"])));
                parameters.push(Parameter::choice("appearance", Origin::Playground, None, plain(&["dark", "light"])));
                parameters.extend(axes(true, &[]));
                parameters
            }
            Action::Compound { members, .. } => {
                let mut parameters: Vec<Parameter> = Vec::new();
                for member in members {
                    let Some((target, _)) = self.member(&member.run) else { continue };
                    if matches!(target.action, Action::Compound { .. }) { continue; }
                    for mut parameter in self.parameters(target) {
                        if member.pins.iter().any(|(key, _)| key == &parameter.id) || parameters.iter().any(|known| known.id == parameter.id) { continue; }
                        (parameter.origin, parameter.required) = (Origin::Member, false);
                        parameters.push(parameter);
                    }
                }
                parameters
            }
            Action::Script { .. } | Action::Repo(_) => Vec::new(),
        }
    }

    /// ♻️ Whether Nx may start a finite task from its published project graph: the graph and the
    /// file map Nx published with it are still the ones on disk, and every project manifest, the
    /// workspace configuration and the root package manifest still hash to what Nx recorded then.
    /// `SEMIO_DASHBOARD_GRAPH=fresh` always answers no.
    pub fn graph_is_current(&self) -> bool {
        if std::env::var("SEMIO_DASHBOARD_GRAPH").is_ok_and(|value| value == "fresh") { return false; }
        let (Some(graph), Some(file_map)) = (Fingerprint::of(&self.root.join(GRAPH_FILE)), Fingerprint::of(&self.root.join(FILE_MAP))) else { return false };
        let mut basis = self.basis.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !basis.as_ref().is_some_and(|known| known.graph == graph && known.file_map == file_map) { *basis = GraphBasis::read(&self.root, &self.definitions); }
        let Some(basis) = basis.as_ref().filter(|known| known.graph == graph && known.file_map == file_map) else { return false };
        self.definitions.len() > GRAPH_OWNERS.len() && self.definitions.iter().all(|file| basis.hashes.get(file).is_some_and(|recorded| std::fs::read(self.root.join(file)).is_ok_and(|bytes| xxh3_64(&bytes) == *recorded)))
    }

    /// 🧷️ Adopts what Nx recorded with its published graph, read ahead of the first launch.
    pub fn adopt_basis(&self, basis: Option<GraphBasis>) { *self.basis.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = basis; }
    /// 📌️ What Nx recorded with its published graph, as far as this registry has read it.
    pub fn basis(&self) -> Option<GraphBasis> { self.basis.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() }

    /// 🚀️ Resolves a command with the chosen parameters and extra arguments under the dashboard's
    /// current run policy.
    pub fn resolve(&self, id: &str, chosen: &[(String, String)], extra: &[String]) -> Result<Launch, String> {
        let launch = self.resolve_with(id, chosen, extra, RunPolicy::default())?;
        let reuses = |launch: &Launch| launch.processes.iter().any(|process| !process.long_running && invokes_nx(&process.cmd, &process.args));
        if !(reuses(&launch) || launch.requires.iter().any(reuses)) || !self.graph_is_current() { return Ok(launch); }
        self.resolve_with(id, chosen, extra, RunPolicy { reuse_published_graph: true })
    }

    /// 🧪️ Resolves a command under an explicit run policy: a pure function of the registry.
    pub fn resolve_with(&self, id: &str, chosen: &[(String, String)], extra: &[String], policy: RunPolicy) -> Result<Launch, String> {
        self.launch(id, chosen, extra, policy, &mut Vec::new())
    }

    fn launch(&self, id: &str, chosen: &[(String, String)], extra: &[String], policy: RunPolicy, stack: &mut Vec<String>) -> Result<Launch, String> {
        let (entry, configuration) = self.member(id).ok_or_else(|| format!("unknown command {id:?}"))?;
        if stack.iter().any(|known| known == &entry.id) { return Err(format!("{} requires itself through {}", entry.id, stack.join(" → "))); }
        stack.push(entry.id.clone());
        let mut requires: Vec<String> = Vec::new();
        let (processes, group, stop) = match &entry.action {
            Action::Compound { members, stop } => {
                if !extra.is_empty() { return Err(format!("{} is a compound and takes no extra arguments", entry.id)); }
                let accepted = self.parameters(entry);
                for (key, _) in chosen { if !accepted.iter().any(|parameter| &parameter.id == key) { return Err(unknown_parameter(&entry.id, key, &accepted)); } }
                let mut processes = Vec::new();
                for member in members {
                    let (target, configuration) = self.member(&member.run).ok_or_else(|| format!("{}: member {:?} names no command", entry.id, member.run))?;
                    if matches!(target.action, Action::Compound { .. }) { return Err(format!("{}: member {:?} is a compound; compounds do not nest", entry.id, member.run)); }
                    let offered = self.parameters(target);
                    let passed: Vec<(String, String)> = chosen.iter().filter(|(key, _)| offered.iter().any(|parameter| &parameter.id == key) && !member.pins.iter().any(|(pin, _)| pin == key)).cloned().collect();
                    let (process, needs) = self.process(target, configuration, &passed, &member.pins, &[], policy).map_err(|error| format!("{}: member {}: {error}", entry.id, member.run))?;
                    for need in needs { if !requires.contains(&need) { requires.push(need); } }
                    processes.push(process);
                }
                requires.retain(|need| !members.iter().any(|member| self.member(&member.run).map(|(entry, _)| &entry.id) == self.member(need).map(|(entry, _)| &entry.id)));
                (processes, Some(entry.id.clone()), *stop)
            }
            _ => { let (process, needs) = self.process(entry, configuration, chosen, &[], extra, policy)?; requires = needs; (vec![process], None, Stop::Independent) }
        };
        let requires = requires.iter().map(|need| self.launch(need, &[], &[], policy, stack)).collect::<Result<Vec<_>, _>>()?;
        stack.pop();
        let label = match &entry.action {
            Action::Compound { .. } => TaskLabel { verb: entry.verb.clone(), owner: entry.owner.clone(), subject: entry.subject.clone(), qualifier: String::new(), parameters: chosen.to_vec(), members: processes.len() as u16 },
            _ => processes[0].label.clone(),
        };
        Ok(Launch { command_id: if group.is_some() { entry.id.clone() } else { processes[0].command_id.clone() }, label, processes, requires, group, stop })
    }

    fn process(&self, entry: &Entry, configuration: Option<&str>, chosen: &[(String, String)], pins: &[(String, String)], extra: &[String], policy: RunPolicy) -> Result<(LaunchProcess, Vec<String>), String> {
        let parameters = self.parameters(entry);
        let workspace = self.root.display().to_string();
        let mut pins = pins.to_vec();
        if let Some(configuration) = configuration { pins.push((CONFIGURATION.into(), configuration.to_string())); }
        for (key, value) in chosen.iter().chain(&pins) {
            if !parameters.iter().any(|parameter| &parameter.id == key) { return Err(unknown_parameter(&entry.id, key, &parameters)); }
            if value.contains('\0') { return Err(format!("{}: parameter {key:?} carries a NUL", entry.id)); }
        }
        let mut values: Vec<Option<Value>> = Vec::with_capacity(parameters.len());
        for parameter in &parameters {
            let typed = chosen.iter().rev().find(|(key, _)| key == &parameter.id).map(|(_, text)| Value { text: text.clone(), stated: true });
            let declared = pins.iter().find(|(key, _)| key == &parameter.id).map(|(_, text)| (text, true)).or_else(|| parameter.default.as_ref().map(|text| (text, false)));
            let value = match (typed, declared) {
                (Some(value), _) => Some(value),
                (None, Some((text, stated))) if parameter.kind == ParameterKind::Text => Some(Value { stated, text: substitute(text, &|name| if name == "workspace" { Ok(workspace.clone()) } else { parameters.iter().zip(&values).find(|(known, _)| known.id == name).and_then(|(_, value)| value.as_ref().map(|value| value.text.clone())).ok_or_else(|| format!("{}: parameter {:?} uses `{{{name}}}`, which has no value before it", entry.id, parameter.id)) })? }),
                (None, Some((text, stated))) => Some(Value { text: text.clone(), stated }),
                (None, None) => None,
            };
            match (&value, parameter.kind) {
                (None, _) if parameter.required => return Err(format!("{}: parameter {:?} is required", entry.id, parameter.id)),
                (Some(value), ParameterKind::Choice) if !parameter.values.iter().any(|(known, _)| known == &value.text) => return Err(format!("{}: parameter {:?} has no value {:?}; choose {}", entry.id, parameter.id, value.text, parameter.values.iter().map(|(known, _)| known.as_str()).collect::<Vec<_>>().join(", "))),
                (Some(value), ParameterKind::Flag) if value.text != "true" && value.text != "false" => return Err(format!("{}: flag {:?} is `true` or `false`, not {:?}", entry.id, parameter.id, value.text)),
                _ => {}
            }
            values.push(value);
        }
        let value_of = |id: &str| parameters.iter().zip(&values).find(|(parameter, _)| parameter.id == id).and_then(|(_, value)| value.as_ref().map(|value| value.text.as_str()));
        let (mut env, mut nx_flags, mut args): (Vec<(String, Text)>, Vec<String>, Vec<Text>) = Default::default();
        let put = |env: &mut Vec<(String, Text)>, key: &str, text: Text| match env.iter_mut().find(|(name, _)| name == key) { Some(pair) => pair.1 = text, None => env.push((key.to_string(), text)) };
        if let Action::Tool { env: stated, .. } = &entry.action { for (key, text) in stated { put(&mut env, key, Text::Declared(text.clone())); } }
        for (parameter, value) in parameters.iter().zip(&values) {
            let Some(Value { text: value, .. }) = value else { continue };
            let effect = match parameter.kind { ParameterKind::Choice => parameter.values.iter().find(|(known, _)| known == value).map(|(_, effect)| effect), ParameterKind::Flag => (value == "true").then_some(&parameter.effect), ParameterKind::Text => None };
            for (key, text) in effect.iter().flat_map(|effect| &effect.env) { put(&mut env, key, Text::Declared(text.clone())); }
            nx_flags.extend(effect.iter().flat_map(|effect| effect.nx_flags.iter().cloned()));
            args.extend(effect.iter().flat_map(|effect| effect.args.iter().cloned().map(Text::Declared)));
            match (&parameter.destination, parameter.kind) {
                (Destination::Env(name), ParameterKind::Text) => put(&mut env, name, Text::Literal(value.clone())),
                (Destination::Flag(flag), ParameterKind::Text) if flag.ends_with('=') => args.push(Text::Literal(format!("{flag}{value}"))),
                (Destination::Flag(flag), ParameterKind::Text) => args.extend([Text::Literal(flag.clone()), Text::Literal(value.clone())]),
                (Destination::Positional, ParameterKind::Text) => args.push(Text::Literal(value.clone())),
                _ => {}
            }
        }
        let named = |name: &str| -> Result<String, String> { if name == "workspace" { Ok(workspace.clone()) } else { value_of(name).map(str::to_string).ok_or_else(|| format!("{}: `{{{name}}}` has no value; choose parameter {name:?}", entry.id)) } };
        let (ready_spec, requires) = match &entry.action { Action::Target { ready, requires, .. } | Action::Tool { ready, requires, .. } => (ready.clone(), requires.clone()), _ => (None, Vec::new()) };
        let mut ready = None;
        if let Some(spec) = &ready_spec {
            let stated = match spec.port_env.as_ref().and_then(|name| env.iter().find(|(key, _)| key == name).map(|(_, text)| text).cloned().or_else(|| value_of(name).map(|value| Text::Literal(value.to_string())))) {
                Some(Text::Declared(text)) => Some(substitute(&text, &|name| if name == "port" { Err(format!("{}: the ready port is defined through itself", entry.id)) } else { named(name) })?),
                Some(Text::Literal(text)) => Some(text),
                None => None,
            };
            let port = match stated {
                Some(text) => text.parse::<u16>().ok().filter(|port| *port > 0).ok_or_else(|| format!("{}: ready port {text:?} is not a port", entry.id))?,
                None => spec.port.ok_or_else(|| format!("{}: ready.portEnv {:?} has no value; choose it or declare `port`", entry.id, spec.port_env.clone().unwrap_or_default()))?,
            };
            if let Some(name) = spec.port_env.as_ref().filter(|name| is_env_name(name) && !env.iter().any(|(key, _)| &key == name) && !parameters.iter().any(|parameter| &&parameter.id == name)) { env.push((name.clone(), Text::Literal(port.to_string()))); }
            ready = Some(Ready { port, path: spec.path.clone() });
        }
        let port = ready.as_ref().map(|ready| ready.port);
        let full = |text: &Text| -> Result<String, String> { match text { Text::Literal(text) => Ok(text.clone()), Text::Declared(text) => substitute(text, &|name| if name == "port" { port.map(|port| port.to_string()).ok_or_else(|| format!("{}: `{{port}}` needs a `ready` declaration", entry.id)) } else { named(name) }) } };
        let mut env = env.iter().map(|(key, text)| Ok((key.clone(), full(text)?))).collect::<Result<Vec<(String, String)>, String>>()?;
        let mut args = args.iter().map(&full).collect::<Result<Vec<_>, _>>()?;
        args.extend(extra.iter().cloned());
        let tail = |mut head: Vec<String>, nx_flags: Vec<String>, args: Vec<String>| { head.extend(nx_flags); if !args.is_empty() { head.push("--".into()); head.extend(args); } head };
        let mut label = TaskLabel { verb: entry.verb.clone(), owner: entry.owner.clone(), subject: entry.subject.clone(), qualifier: entry.qualifier.clone(), parameters: Vec::new(), members: 0 };
        let mut command_id = entry.id.clone();
        let mut hidden: &[&str] = &[];
        let (cmd, args, cwd, nx) = match &entry.action {
            Action::Target { project, target, .. } => {
                let configuration = value_of(CONFIGURATION).filter(|_| parameters.iter().any(|parameter| parameter.origin == Origin::Configuration));
                if let Some(configuration) = configuration { command_id = format!("{command_id}:{configuration}"); label.qualifier = if label.qualifier.is_empty() { configuration.to_string() } else { format!("{}:{configuration}", label.qualifier) }; hidden = &[CONFIGURATION]; }
                ("bun".to_string(), tail(vec!["nx".into(), "run".into(), configuration.map_or_else(|| format!("{project}:{target}"), |configuration| format!("{project}:{target}:{configuration}"))], nx_flags, args), self.root.clone(), true)
            }
            Action::Group { target, projects } => ("bun".to_string(), tail(vec!["nx".into(), "run-many".into(), "-t".into(), target.clone(), "-p".into(), projects.join(",")], nx_flags, args), self.root.clone(), true),
            Action::Script { name } => { let mut words = vec!["run".to_string(), name.clone()]; words.extend(args); ("bun".to_string(), words, self.root.clone(), true) }
            Action::Tool { command, cwd, .. } => {
                let mut words = command.iter().map(|word| full(&Text::Declared(word.clone()))).collect::<Result<Vec<_>, _>>()?;
                let cmd = words.remove(0);
                words.extend(args);
                let nx = invokes_nx(&cmd, &words);
                (cmd, words, full(&Text::Declared(cwd.clone()))?.split('/').filter(|part| !part.is_empty()).fold(self.root.clone(), |path, part| path.join(part)), nx)
            }
            Action::Playground(playground) => {
                let renderer = value_of("renderer").unwrap_or(RENDERERS[0]);
                let slot = value_of("user-slot").and_then(|slot| slot.parse::<usize>().ok());
                let (port, slots) = if renderer == "react" { (playground.react, &playground.user_react) } else { (playground.wgpu, &playground.user_wgpu) };
                let port = match slot { Some(slot) => *slots.get(slot.wrapping_sub(1)).ok_or_else(|| format!("{}: renderer {renderer} has no user slot {slot}", entry.id))?, None => port };
                let lock = |id: &str| value_of(id).map_or(crate::options::Lock::All, |value| crate::options::Lock::Individual(value.to_string()));
                let row = crate::catalog::PlaygroundEntry { variant: playground.variant.clone(), plugin_id: playground.plugin.clone(), app: playground.app.clone(), ports: crate::catalog::Ports { react: playground.react.into(), wgpu: playground.wgpu.into() }, ..Default::default() };
                let options = crate::env_contract::DevOptions { renderer: renderer.to_string(), port: Some(port), example: lock("example"), language: lock("language"), terminology: lock("terminology"), appearance: lock("appearance"), ..Default::default() };
                let stated = std::mem::replace(&mut env, crate::env_contract::build_dev_env(&playground.variant, Some(&row), &options));
                for (key, value) in stated { set_env(&mut env, &key, &value); }
                let target = playground_target(&playground.variant, renderer).ok_or_else(|| format!("{}: unknown renderer {renderer:?}", entry.id))?;
                let mut args = args;
                if renderer == "wgpu-native" { if let Some(example) = value_of("example") { args.splice(0..0, ["--example".to_string(), example.to_string()]); } } else { ready = Some(Ready { port, path: String::new() }); }
                label.subject = format!("{}·{renderer}", playground.variant);
                label.qualifier = value_of("example").map(|example| format!("#{example}")).into_iter().chain(slot.map(|slot| format!("@{slot}"))).collect::<Vec<_>>().join(" ");
                hidden = &["renderer", "example", "user-slot"];
                ("bun".to_string(), tail(vec!["nx".into(), "run".into(), format!("@semio-tech/framework-os-dev:{target}")], nx_flags, args), self.root.clone(), true)
            }
            Action::Repo(action) => {
                if !extra.is_empty() { return Err(format!("{} takes no extra arguments", entry.id)); }
                match crate::command_tree::repo_implementation() {
                    crate::command_tree::RepoImplementation::Go => (crate::command_tree::go_binary_path(&self.root).display().to_string(), action.go_argv(), self.root.clone(), false),
                    crate::command_tree::RepoImplementation::Rust => (std::env::current_exe().map_err(|error| format!("{}: {error}", entry.id))?.display().to_string(), vec!["repo-view".into(), "--action".into(), serde_json::to_string(action).map_err(|error| error.to_string())?], self.root.clone(), false),
                }
            }
            Action::Compound { .. } => return Err(format!("{}: compounds do not nest", entry.id)),
        };
        if !matches!(entry.action, Action::Repo(_)) { policy.apply(&mut env, nx, entry.long_running); }
        label.parameters = parameters.iter().zip(&values).filter_map(|(parameter, value)| value.as_ref().filter(|value| value.stated && !hidden.contains(&parameter.id.as_str()) && value.text != "false" && Some(&value.text) != parameter.default.as_ref()).map(|value| (parameter.id.clone(), value.text.clone()))).collect();
        Ok((LaunchProcess { command_id, cmd, args, cwd, env, label, ready, long_running: entry.long_running }, requires))
    }

    /// 🧐️ The whole-workspace proof: every declaration is valid, every reference names a command,
    /// every command resolves with its defaults and no label outgrows the launcher.
    pub fn check(&self) -> Vec<Problem> {
        let mut problems = self.problems.clone();
        for entry in &self.entries {
            let mut report = |message: String| problems.push(Problem { file: entry.source.clone(), at: entry.id.clone(), message });
            if matches!(entry.action, Action::Repo(_)) { continue; }
            let accepted = self.parameters(entry);
            let chosen: Vec<(String, String)> = accepted.iter().filter(|parameter| parameter.required && parameter.default.is_none()).map(|parameter| (parameter.id.clone(), parameter.values.first().map_or_else(|| if parameter.kind == ParameterKind::Flag { "true".to_string() } else { "1".to_string() }, |(value, _)| value.clone()))).collect();
            match self.resolve_with(&entry.id, &chosen, &[], RunPolicy::default()) {
                Err(error) if !self.problems.iter().any(|problem| problem.at == entry.id) => report(error),
                Ok(launch) => for process in &launch.processes { for (key, _) in &process.env { if NOISE_ENV.contains(&key.as_str()) && !RUNNER_ENV.iter().any(|(owned, _)| owned == key) { report(format!("resolved environment carries {key}")); } } },
                Err(_) => {}
            }
            if entry.label.chars().count() > 240 { report(format!("label is {} characters long", entry.label.chars().count())); }
        }
        problems
    }

    /// 🗺️ A resolved launch as `#/$defs/Launch` describes it, working directories workspace-relative.
    pub fn launch_json(&self, launch: &Launch) -> serde_json::Value {
        let label = |label: &TaskLabel| serde_json::json!({ "verb": label.verb, "owner": label.owner, "subject": label.subject, "qualifier": label.qualifier, "parameters": label.parameters, "members": label.members });
        let ready = |ready: &Ready| serde_json::json!({ "port": ready.port, "path": ready.path });
        let processes: Vec<serde_json::Value> = launch.processes.iter().map(|process| {
            let cwd = process.cwd.strip_prefix(&self.root).unwrap_or(&process.cwd).components().filter_map(|part| part.as_os_str().to_str()).collect::<Vec<_>>().join("/");
            let mut value = serde_json::json!({ "commandId": process.command_id, "cmd": process.cmd, "args": process.args, "cwd": cwd, "env": process.env, "label": label(&process.label), "longRunning": process.long_running });
            if let Some(known) = &process.ready { value["ready"] = ready(known); }
            value
        }).collect();
        let mut value = serde_json::json!({ "commandId": launch.command_id, "label": label(&launch.label), "processes": processes, "requires": launch.requires.iter().map(|required| self.launch_json(required)).collect::<Vec<_>>(), "stop": launch.stop });
        if let Some(group) = &launch.group { value["group"] = group.clone().into(); }
        value
    }

    /// 🔣️ One command as `#/$defs/RegistryEntry` describes it.
    pub fn entry_json(&self, entry: &Entry) -> serde_json::Value {
        let ready = matches!(&entry.action, Action::Playground(_) | Action::Target { ready: Some(_), .. } | Action::Tool { ready: Some(_), .. }).then(|| self.resolve_with(&entry.id, &[], &[], RunPolicy::default()).ok()).flatten().and_then(|launch| launch.processes.into_iter().find_map(|process| process.ready));
        let mut value = serde_json::json!({
            "id": entry.id, "kind": entry.kind, "verb": entry.verb, "label": entry.label, "owner": entry.owner, "longRunning": entry.long_running, "listed": entry.listed, "mutating": entry.mutating, "source": entry.source,
            "parameters": self.parameters(entry).iter().map(|parameter| { let mut row = serde_json::json!({ "id": parameter.id, "kind": parameter.kind, "values": parameter.values.iter().map(|(value, _)| value).collect::<Vec<_>>(), "required": parameter.required, "origin": parameter.origin }); if let Some(default) = &parameter.default { row["default"] = default.clone().into(); } row }).collect::<Vec<_>>(),
        });
        if let Some(ready) = ready { value["ready"] = serde_json::json!({ "port": ready.port, "path": ready.path }); }
        value
    }
}

impl Entry {
    #[allow(clippy::too_many_arguments)]
    fn new(id: String, kind: Kind, verb: String, path: Vec<Segment>, owner: Vec<String>, subject: String, qualifier: String, long_running: bool, source: &str, own: Vec<Parameter>, action: Action) -> Self {
        let mut entry = Self { id, kind, verb, path, label: String::new(), owner, subject, qualifier, long_running, listed: true, mutating: false, source: source.to_string(), own, action, haystack: String::new() };
        entry.relabel();
        entry
    }

    fn relabel(&mut self) {
        self.label = std::iter::once(self.verb.as_str()).chain(self.path.iter().map(|part| part.label.as_str())).collect::<Vec<_>>().join(" / ");
        self.haystack = format!("{} {}", self.label, self.id).to_lowercase();
    }

    /// 🦑️ The repo-domain action a repo command performs in process.
    pub fn repo_action(&self) -> Option<&RepoAction> { match &self.action { Action::Repo(action) => Some(action), _ => None } }
    /// 🧩️ Whether the command starts several processes.
    pub fn is_compound(&self) -> bool { matches!(self.action, Action::Compound { .. }) }
    /// 🛝️ The playground variant a playground command runs.
    pub fn playground(&self) -> Option<&PlaygroundFacts> { match &self.action { Action::Playground(playground) => Some(playground), _ => None } }
}

fn unknown_parameter(id: &str, key: &str, accepted: &[Parameter]) -> String {
    format!("{id}: unknown parameter {key:?}; it accepts {}", if accepted.is_empty() { "none".to_string() } else { accepted.iter().map(|parameter| parameter.id.as_str()).collect::<Vec<_>>().join(", ") })
}

fn invokes_nx(cmd: &str, args: &[String]) -> bool { cmd == "nx" || cmd == "bun" && args.first().is_some_and(|first| first == "nx" || first == "run") }
// #endregion 🔖️Registry

// #region 🔖️Command
/// ⌨️ `semio commands [words…] [--json] [--all] [--root PATH]` lists or searches the registry;
/// `--check` proves every declaration of the workspace; `--resolve <id> [key=value…] [--extra "words"]`
/// prints what a command would start without starting it. `--refresh` discovers afresh instead of
/// trusting the snapshot, `--snapshot PATH` keeps the snapshot elsewhere.
pub fn run(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {
    let root = parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from);
    let snapshot = parsed.flag("snapshot").map_or_else(|| crate::inventory::cache_path(&root), PathBuf::from);
    let registry = if parsed.has_flag("refresh") || parsed.has_flag("check") { crate::inventory::discover(&root, &std::sync::atomic::AtomicBool::new(false)) } else { crate::inventory::registry_at(&root, &snapshot) };
    if let Some(id) = parsed.flag("resolve") {
        let chosen: Vec<(String, String)> = parsed.segments.iter().filter_map(|word| word.split_once('=')).map(|(key, value)| (key.to_string(), value.to_string())).collect();
        let resolved = split_arguments(parsed.flag("extra").unwrap_or_default()).and_then(|extra| if parsed.has_flag("fresh-graph") { registry.resolve_with(id, &chosen, &extra, RunPolicy::default()) } else { registry.resolve(id, &chosen, &extra) });
        return match resolved { Ok(launch) => { println!("{}", registry.launch_json(&launch)); 0 } Err(error) => { eprintln!("[semio] {error}"); 1 } };
    }
    if parsed.has_flag("check") {
        let problems = registry.check();
        for problem in &problems { eprintln!("{problem}"); }
        println!("{} commands, {} problems", registry.entries().len(), problems.len());
        return i32::from(!problems.is_empty());
    }
    let words: Vec<&str> = parsed.segments.iter().map(String::as_str).chain(["json", "all", "refresh"].iter().filter_map(|flag| parsed.flag(flag))).collect();
    let found = registry.search(&words, parsed.has_flag("all"));
    if parsed.has_flag("json") {
        println!("{}", serde_json::Value::Array(found.iter().map(|position| registry.entry_json(&registry.entries()[*position])).collect()));
        return 0;
    }
    for position in found {
        let entry = &registry.entries()[position];
        let parameters = registry.parameters(entry).iter().map(|parameter| if parameter.values.is_empty() { parameter.id.clone() } else { format!("{}={}", parameter.id, parameter.values.iter().map(|(value, _)| value.as_str()).collect::<Vec<_>>().join("|")) }).collect::<Vec<_>>().join(" ");
        println!("{}\t{}\t{}{}{}", entry.id, entry.label, if entry.long_running { "long-running" } else { "finite" }, if entry.mutating { " mutating" } else { "" }, if parameters.is_empty() { String::new() } else { format!("\t{parameters}") });
    }
    0
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
