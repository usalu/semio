//! 🪪️ Command identity of the registry: the verbs, the command id grammar, invocation parsing and formatting, taxonomy segments and the lexical predicates of declarations. See [`super`].


// #region 🔖️Identity
/// 🗂️ The verbs a command is filed under, in launcher order; anything else is [`FALLBACK_VERB`].
pub const VERBS: &[&str] = &["setup", "start", "dev", "serve", "watch", "activate", "prepare", "run", "build", "package", "test", "smoke", "check", "typecheck", "verify", "gate", "lint", "format", "generate", "publish", "deploy", "preview", "bench", "clean"];

/// 🧺️ The verb of a name that begins with no known verb.
pub const FALLBACK_VERB: &str = "task";

/// 🦑️ The branches the repo domain contributes, listed after every verb.
pub const REPO_VERBS: &[&str] = &["tickets", "draft", "goals", "analyze", "tree", "statutes"];

pub(super) const RESERVED_KINDS: &[&str] = &["playground", "tool", "compound", "group", "ticket"];

/// 🔑️ Whether text after `repo:` is a repo-domain action key rather than a target of a project named `repo`.
fn is_action_key(text: &str) -> bool {
    let scope = |text: &str| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_lowercase());
    matches!(text, "goals.list" | "goals.tree" | "tree.monorepo" | "tree.goal" | "tree.statute" | "tree.territory" | "statutes.catalog" | "draft.start" | "draft.discard")
        || text.strip_prefix("analyze:").is_some_and(scope)
        || ["ticket.show:", "ticket.files:", "ticket.close:", "ticket.reopen:"].iter().any(|head| text.strip_prefix(head).is_some_and(|id| !id.is_empty()))
}

/// 🆔️ The identity of one command, in the grammar of `#/$defs/CommandId`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommandId {
    Target { project: String, target: String, configuration: Option<String> },
    Playground { variant: String },
    Tool { project: String, id: String },
    Compound { project: String, id: String },
    Group { project: String, id: String },
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

pub(super) fn segment(key: impl Into<String>, label: impl Into<String>) -> Segment { Segment { key: key.into(), label: label.into() } }

/// 🏘️ The taxonomy segments of a project root with the structural folders dropped.
pub fn owner_segments(root: &str) -> Vec<Segment> {
    let segments: Vec<Segment> = root.split('/').filter(|part| !part.is_empty() && *part != ".").map(|part| segment(segment_key(part), part)).filter(|part| !part.key.is_empty() && !TAXONOMY_SKIP_KEYS.contains(&part.key.as_str())).collect();
    if segments.is_empty() { vec![segment("workspace", "workspace")] } else { segments }
}

pub(super) fn short_name(project: &str) -> &str { project.rsplit('/').next().unwrap_or(project) }

pub(super) fn is_slug(text: &str) -> bool { !text.is_empty() && text.len() <= 64 && text.split(['.', '_', '-']).all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())) }

pub(super) fn is_verb(text: &str) -> bool { text.len() <= 32 && text.bytes().next().is_some_and(|byte| byte.is_ascii_lowercase()) && text.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') }

pub(super) fn is_env_name(text: &str) -> bool { text.bytes().next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_') && text.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_') }

pub(super) fn is_flag(text: &str) -> bool { let name = text.strip_prefix("--").or_else(|| text.strip_prefix('-')).unwrap_or_default(); name.bytes().next().is_some_and(|byte| byte.is_ascii_alphabetic()) && !text.contains(|c: char| c.is_whitespace() || c == '\0') }

pub(super) fn is_value_flag(text: &str) -> bool { let name = text.strip_suffix('=').unwrap_or(text); is_flag(name) && name.trim_start_matches('-').bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-')) }

pub(super) fn is_reference(text: &str) -> bool { text.split_once(':').is_some_and(|(head, rest)| !head.is_empty() && !rest.is_empty() && !text.contains(char::is_whitespace) && !["compound", "group"].contains(&head) && !(head == "repo" && is_action_key(rest))) }

/// 🌟️ Whether a project name matches an build project pattern, where `*` stands for any run of characters.
pub(super) fn glob(pattern: &str, name: &str) -> bool {
    match pattern.split_once('*') {
        None => pattern == name,
        Some((head, rest)) => name.strip_prefix(head).is_some_and(|tail| (0..=tail.len()).filter(|at| tail.is_char_boundary(*at)).any(|at| glob(rest, &tail[at..]))),
    }
}

pub(super) fn is_relative(text: &str) -> bool { !text.starts_with(['/', '~', '\\']) && !text.contains(['\\', '\0']) && !(text.len() > 1 && text.as_bytes()[1] == b':' && text.as_bytes()[0].is_ascii_alphabetic()) && text.split('/').all(|part| part != "..") }

/// 🚫️ The runner variables the dashboard owns; a declaration never stores them.
pub const NOISE_ENV: &[&str] = &["SEMIO_BUILD_TERMINAL", "SEMIO_BUILD_REUSE_GRAPH", "NX_DAEMON", "NX_ISOLATE_PLUGINS", "NX_CACHE_PROJECT_GRAPH", "NX_TUI", "NX_NATIVE_COMMAND_RUNNER", "NX_PLUGIN_NO_TIMEOUTS", "NX_FORCE_REUSE_CACHED_GRAPH", "FORCE_COLOR"];

const ABSOLUTE_ROOTS: &[&str] = &["Users", "home", "root", "private", "tmp", "var", "opt", "usr", "etc", "mnt", "workspaces", "Volumes"];

pub(super) fn is_absolute_literal(text: &str) -> bool {
    let bytes = text.as_bytes();
    text.starts_with("~/") || text.starts_with("~\\") || (bytes.len() > 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && matches!(bytes[2], b'/' | b'\\')) || text.strip_prefix('/').and_then(|rest| rest.split_once('/')).is_some_and(|(head, _)| ABSOLUTE_ROOTS.contains(&head))
}
// #endregion 🔖️Identity
