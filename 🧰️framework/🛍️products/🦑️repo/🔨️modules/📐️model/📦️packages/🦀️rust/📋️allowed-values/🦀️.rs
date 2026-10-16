//! 📋️ The allowed LLM, effort and client tables embedded from the schema, and the slug vocabularies derived from them.

use super::*;

//#region 📋️Allowed Values

/// 📋️ The embedded allowed-value tables, shared verbatim with the Go implementation.
#[derive(Clone, Debug, Deserialize)]
struct AllowedValues {
    llms: Vec<String>,
    efforts: Vec<String>,
    clients: Vec<String>,
}

const ALLOWED_VALUES_JSON: &str = include_str!("../../../🧬️schema/🔣️allowed-values.json");

pub(super) fn allowed_values() -> &'static AllowedValues {
    static CELL: OnceLock<AllowedValues> = OnceLock::new();
    CELL.get_or_init(|| {
        serde_json::from_str(ALLOWED_VALUES_JSON)
            .expect("🧬️schema/🔣️allowed-values.json is not a valid allowed-value table")
    })
}

/// 🤖️ The allowed LLM slugs, in declaration order.
pub fn allowed_llms() -> &'static [String] {
    &allowed_values().llms
}

/// 🏋️ The allowed reasoning-effort slugs, in declaration order.
pub fn allowed_efforts() -> &'static [String] {
    &allowed_values().efforts
}

/// 💻️ The allowed client slugs, in declaration order.
pub fn allowed_clients() -> &'static [String] {
    &allowed_values().clients
}

//#endregion 📋️Allowed Values

//#region 🔤️Slugs

/// 🔠️ Go `strings.ToUpper` — simple (non-special-casing) upper mapping, so `ß` stays `ß`.
fn go_to_upper(text: &str) -> String {
    text.chars()
        .map(|c| {
            let mut upper = c.to_uppercase();
            match (upper.next(), upper.next()) {
                (Some(single), None) => single,
                _ => c,
            }
        })
        .collect()
}

/// 🔡️ Go `strings.ToLower` — simple (non-special-casing) lower mapping.
pub(super) fn go_to_lower(text: &str) -> String {
    text.chars()
        .map(|c| {
            let mut lower = c.to_lowercase();
            match (lower.next(), lower.next()) {
                (Some(single), None) => single,
                _ => c,
            }
        })
        .collect()
}

/// 🤝️ Go `strings.EqualFold` over simple case folding.
pub(super) fn eq_fold(left: &str, right: &str) -> bool {
    go_to_lower(left) == go_to_lower(right)
}

/// 💠️ Splits camel case into `-` boundaries, upper cases, collapses every run outside
/// `[A-Z0-9]` into a single `-` and trims the result. Byte-identical to the repo's `Slugify`.
///
/// It lives here, at the bottom of the dependency DAG, because the slug vocabularies below are
/// defined in terms of it. It is public because `🎫️tickets` derives a ticket folder slug from a
/// title with exactly this rule and may not own a second copy of it.
pub fn slugify(text: &str) -> String {
    let runes: Vec<char> = text.chars().collect();
    let mut buf = String::with_capacity(text.len() + 8);
    for (index, &current) in runes.iter().enumerate() {
        if index > 0 && current.is_ascii_uppercase() {
            let previous = runes[index - 1];
            let leaves_a_word = previous.is_ascii_lowercase();
            let starts_a_word = previous.is_ascii_uppercase() && runes.get(index + 1).is_some_and(char::is_ascii_lowercase);
            if leaves_a_word || starts_a_word {
                buf.push('-');
            }
        }
        buf.push(current);
    }
    let upper = go_to_upper(&buf);
    let mut slug = String::with_capacity(upper.len());
    let mut in_separator = false;
    for character in upper.chars() {
        if character.is_ascii_uppercase() || character.is_ascii_digit() {
            slug.push(character);
            in_separator = false;
        } else if !in_separator {
            slug.push('-');
            in_separator = true;
        }
    }
    slug.trim_matches('-').to_string()
}

/// 🤖️ Canonical form of an LLM slug. Idempotent for already-normalised values.
pub fn normalize_llm_slug(llm: &str) -> String {
    go_to_lower(&slugify(llm))
}

/// 🏋️ Canonical form of a reasoning-effort slug. Idempotent for already-normalised values.
pub fn normalize_effort_slug(effort: &str) -> String {
    go_to_lower(&slugify(effort))
}

/// 💻️ Canonical form of a client slug. Idempotent for already-normalised values.
pub fn normalize_client_slug(client: &str) -> String {
    go_to_lower(&slugify(client))
}

fn resolve_by_containment(field: &'static str, slug: String, allowed: &'static [String]) -> Result<String, ModelError> {
    let mut best = "";
    for candidate in allowed {
        if slug.contains(&go_to_lower(&slugify(candidate))) && candidate.len() > best.len() {
            best = candidate;
        }
    }
    if best.is_empty() {
        return Err(ModelError::NotAllowed { field, slug, allowed: allowed.to_vec() });
    }
    Ok(best.to_string())
}

/// 🤖️ Resolves a free-form LLM name to the longest allowed slug it contains.
pub fn resolve_allowed_llm(llm: &str) -> Result<String, ModelError> {
    resolve_by_containment("llm", normalize_llm_slug(llm), allowed_llms())
}

/// 🏋️ Resolves a free-form reasoning effort to the longest allowed slug it contains. Blank
/// input resolves to a blank effort rather than an error, because effort is optional.
pub fn resolve_allowed_effort(effort: &str) -> Result<String, ModelError> {
    if effort.trim().is_empty() {
        return Ok(String::new());
    }
    resolve_by_containment("effort", normalize_effort_slug(effort), allowed_efforts())
}

/// 💻️ Resolves a free-form client name to the longest allowed slug it contains.
pub fn resolve_allowed_client(client: &str) -> Result<String, ModelError> {
    resolve_by_containment("client", normalize_client_slug(client), allowed_clients())
}

//#endregion 🔤️Slugs
