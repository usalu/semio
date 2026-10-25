//! 🧰️ Verb support: the operand, flag and vocabulary readers every verb handler shares.

use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use semio_framework_repo_identity::flat;

/// 🎫️ A `YY/MM/DD/SLUG` operand split into its date and slug, zeros when it is no path.
pub(super) fn ticket_path(raw: &str) -> Option<(i64, i64, i64, String)> {
    let parts: Vec<&str> = raw.split('/').collect();
    if parts.len() < 4 {
        return None;
    }
    Some((parts[0].parse().unwrap_or_default(), parts[1].parse().unwrap_or_default(), parts[2].parse().unwrap_or_default(), parts[3..].join("/")))
}

/// 🔤️ The first of a flag and an operand that is present.
pub(super) fn flag_or_arg(invocation: &Invocation<'_>, flag: &str, index: usize) -> String {
    let value = invocation.text(flag);
    if value.is_empty() {
        invocation.arg(index)
    } else {
        value
    }
}

/// 🧲️ A vocabulary value from its flag, its switches or the first operand that contains a slug,
/// twin of `extractLLMFromArgs` and friends.
fn extract_vocabulary(invocation: &Invocation<'_>, flag: &str, allowed: &[String], normalize: fn(&str) -> String, args: Vec<String>) -> (String, Vec<String>) {
    let value = invocation.text(flag);
    if !value.is_empty() {
        return (value, args);
    }
    if let Some(found) = allowed.iter().find(|slug| invocation.flag(slug)) {
        return (found.clone(), args);
    }
    let mut remaining = Vec::new();
    let mut found = String::new();
    for arg in args {
        if !found.is_empty() {
            remaining.push(arg);
            continue;
        }
        let normalized = normalize(&arg);
        let best = allowed.iter().filter(|slug| normalized.contains(&normalize(slug))).max_by_key(|slug| slug.len()).cloned();
        match best {
            Some(slug) => found = slug,
            None => remaining.push(arg),
        }
    }
    (found, remaining)
}

pub(super) fn extract_client(invocation: &Invocation<'_>, args: Vec<String>) -> (String, Vec<String>) {
    extract_vocabulary(invocation, "client", semio_framework_repo_model::allowed_clients(), semio_framework_repo_model::normalize_client_slug, args)
}

pub(super) fn extract_llm(invocation: &Invocation<'_>, args: Vec<String>) -> (String, Vec<String>) {
    extract_vocabulary(invocation, "llm", semio_framework_repo_model::allowed_llms(), semio_framework_repo_model::normalize_llm_slug, args)
}

pub(super) fn extract_effort(invocation: &Invocation<'_>, args: Vec<String>) -> (String, Vec<String>) {
    extract_vocabulary(invocation, "effort", semio_framework_repo_model::allowed_efforts(), semio_framework_repo_model::normalize_effort_slug, args)
}

/// 📣️ The wire spelling of a client enum value.
pub(super) fn client_enum(client: &str) -> String {
    client.replace('-', "_").to_uppercase()
}

pub(super) fn missing_client() -> CliError {
    CliError::Message(format!("missing client. Use --client <value>, --<client-name> flag, or positional arg. Allowed: {}", semio_framework_repo_model::allowed_clients().join(", ")))
}

pub(super) fn missing_llm() -> CliError {
    CliError::Message(format!("missing llm. Use --llm <value>, --<llm-name> flag, or positional arg. Allowed: {}", semio_framework_repo_model::allowed_llms().join(", ")))
}

/// 🧾️ Prints the lines a move plan reported.
pub(super) fn print_lines(invocation: &mut Invocation<'_>, lines: Vec<String>) {
    for line in lines {
        invocation.println(&line);
    }
}

/// 🧭️ The section name a section slug resolves to in a file, twin of `ResolveSectionName`.
pub fn resolve_section_name(root: &std::path::Path, file_path: &str, slug: &str) -> String {
    let fallback = semio_framework_repo_workspace_unslugify(slug);
    let Ok(content) = std::fs::read_to_string(root.join(file_path)) else { return fallback };
    fn find(sections: &[semio_framework_repo_model::Section], slug: &str) -> Option<String> {
        for section in sections {
            if flat(&section.name) == flat(slug) || semio_framework_repo_identity::slugify(&section.name).eq_ignore_ascii_case(slug) {
                return Some(section.name.clone());
            }
            if let Some(found) = find(&section.children, slug) {
                return Some(found);
            }
        }
        None
    }
    find(&semio_framework_repo_languages::parse_sections(&content, file_path), slug).unwrap_or(fallback)
}

/// 📖️ A slug read back as words, twin of `UnSlugify`.
fn semio_framework_repo_workspace_unslugify(slug: &str) -> String {
    slug.split(['-', '_']).filter(|word| !word.is_empty()).map(|word| {
        let mut characters = word.chars();
        match characters.next() {
            Some(first) => first.to_uppercase().collect::<String>() + &characters.as_str().to_lowercase(),
            None => String::new(),
        }
    }).collect::<Vec<String>>().join(" ")
}
