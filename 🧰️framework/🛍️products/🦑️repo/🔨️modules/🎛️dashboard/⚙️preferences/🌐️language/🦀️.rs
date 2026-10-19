//! 🌐️ The language chain: where the dashboard takes its language from when no preference event,
//! environment override or argument names one. The system variables and the fallback order are data
//! in the sibling `🔣️.json`; no language is hard-coded and `Locale` has no `Default`.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/⚙️preferences/🌐️language/🔣️.json

use serde::Deserialize;
use std::sync::OnceLock;
use ui_locale::Locale;

const DECLARATION: &str = include_str!("🔣️.json");

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    #[serde(rename = "$schema", default)]
    _schema: Option<String>,
    #[serde(rename = "systemVariables")]
    system_variables: Vec<String>,
    fallback: Vec<String>,
}

/// 🔗️ The declared chain: system variables first, then the fallback languages in preference order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chain { system_variables: Vec<String>, fallback: Vec<Locale> }

/// 🔎️ Reads a variable of the environment; an empty value is no value.
pub type Lookup<'a> = &'a dyn Fn(&str) -> Option<String>;

/// 🌍️ The process environment as a lookup.
pub fn process_environment(name: &str) -> Option<String> { std::env::var(name).ok().filter(|value| !value.is_empty()) }

/// 🗣️ Names the supported language of a system value: `de_DE.UTF-8`, `de-AT` and `de@euro` name `de`; `C`, `POSIX` and unsupported languages name none.
pub fn negotiate(value: &str) -> Option<Locale> {
    let language = value.split(['.', '@']).next()?.split(['_', '-']).next()?;
    Locale::parse(&language.to_ascii_lowercase())
}

fn is_variable_name(name: &str) -> bool { name.starts_with(|c: char| c.is_ascii_uppercase()) && name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') }

impl Chain {
    /// 🔍️ Reads a declaration; every fallback must be a supported language and at least one is required.
    pub fn parse(text: &str) -> Result<Self, String> {
        let document: Document = serde_json::from_str(text).map_err(|error| format!("the language chain is not valid: {error}"))?;
        let fallback = document.fallback.iter().map(|id| Locale::parse(id).ok_or_else(|| format!("the language chain falls back to unsupported language {id:?}"))).collect::<Result<Vec<_>, _>>()?;
        if fallback.is_empty() { return Err("the language chain declares no fallback language".into()); }
        if fallback.iter().enumerate().any(|(at, locale)| fallback[..at].contains(locale)) { return Err("the language chain repeats a fallback language".into()); }
        if let Some(name) = document.system_variables.iter().enumerate().find_map(|(at, name)| (!is_variable_name(name) || document.system_variables[..at].contains(name)).then_some(name)) { return Err(format!("the language chain names {name:?}, which is not a distinct environment variable")); }
        Ok(Self { system_variables: document.system_variables, fallback })
    }

    /// 🖥️ The first system variable that names a supported language.
    pub fn system(&self, lookup: Lookup<'_>) -> Option<Locale> { self.system_variables.iter().find_map(|name| lookup(name).as_deref().and_then(negotiate)) }

    /// 🥇️ The first declared fallback language.
    pub fn fallback(&self) -> Locale { self.fallback[0] }

    /// 🧭️ The system language, or the first declared fallback when the system names none.
    pub fn resolve(&self, lookup: Lookup<'_>) -> Locale { self.system(lookup).unwrap_or_else(|| self.fallback()) }
}

/// 📜️ The chain shipped with the dashboard.
pub fn declared() -> &'static Chain {
    static CHAIN: OnceLock<Chain> = OnceLock::new();
    CHAIN.get_or_init(|| Chain::parse(DECLARATION).expect("the shipped language chain is valid"))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
