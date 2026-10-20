//! 🌐️ Resolves explicit or supported system language without selecting a default.
use serde::Deserialize;
use std::sync::OnceLock;
use ui_locale::Locale;
const DECLARATION: &str = include_str!("🔣️.json");
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Choice { pub language: String, pub name: String }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document { #[serde(rename="$schema",default)] _schema: Option<String>, #[serde(rename="systemVariables")] system_variables: Vec<String>, choices: Vec<Choice>, prompt: Vec<String>, messages: std::collections::BTreeMap<String,String> }
/// 🔗️ Supported system variables and explicit language choices; order never implies selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chain { system_variables: Vec<String>, choices: Vec<Choice>, prompt: Vec<String>, messages: std::collections::BTreeMap<String,String> }
pub type Lookup<'a> = &'a dyn Fn(&str) -> Option<String>;
/// 🌍️ Reads a nonempty system setting.
pub fn process_environment(name: &str) -> Option<String> { std::env::var(name).ok().filter(|value| !value.is_empty()) }
/// 🗣️ Names a supported system language; C, POSIX and unsupported locales resolve to none.
pub fn negotiate(value: &str) -> Option<Locale> { let language=value.split(['.','@']).next()?.split(['_','-']).next()?; Locale::parse(&language.to_ascii_lowercase()) }
fn bounded(text: &str, maximum: usize) -> bool { !text.is_empty() && text.chars().count()<=maximum && !text.chars().any(char::is_control) }
impl Chain {
    /// 🔍️ Validates the closed authored choices and bilingual startup prompt.
    pub fn parse(text: &str) -> Result<Self,String> {
        let document: Document=serde_json::from_str(text).map_err(|error| error.to_string())?;
        if document.choices.len()!=2 || document.choices.iter().enumerate().any(|(at,choice)| Locale::parse(&choice.language).is_none() || !bounded(&choice.name,64) || document.choices[..at].iter().any(|previous| previous.language==choice.language)) { return Err("distinct supported language choices required".into()); }
        if document.prompt.len()!=3 || document.prompt.iter().any(|line| !bounded(line,256)) { return Err("bounded bilingual language prompt required".into()); }
        if document.system_variables.iter().enumerate().any(|(at,name)| !name.starts_with(|c:char| c.is_ascii_uppercase()) || !name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c=='_') || document.system_variables[..at].contains(name)) { return Err("distinct system variable names required".into()); }
        let keys=["saving","saveFailed","disconnected","terminalUnavailable","required","cancelled"];if document.messages.len()!=keys.len() || keys.iter().any(|key|document.messages.get(*key).is_none_or(|value|!bounded(value,256))){return Err("closed bilingual language messages required".into());}
        Ok(Self{system_variables:document.system_variables,choices:document.choices,prompt:document.prompt,messages:document.messages})
    }
    /// 🖥️ Resolves the first supported system setting without a fallback.
    pub fn resolve(&self, lookup: Lookup<'_>) -> Option<Locale> { self.system_variables.iter().find_map(|name| lookup(name).as_deref().and_then(negotiate)) }
    pub fn choices(&self) -> &[Choice] { &self.choices }
    pub fn prompt(&self) -> &[String] { &self.prompt }
    pub fn message(&self,key:&str)->&str { self.messages.get(key).expect("authored language message") }
}
/// 📜️ Reads one validated authored language policy.
pub fn declared() -> &'static Chain { static CHAIN:OnceLock<Chain>=OnceLock::new(); CHAIN.get_or_init(||Chain::parse(DECLARATION).expect("valid authored language choices")) }
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"] mod tests;
