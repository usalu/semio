//! 🌐️ Resolves the dashboard language by the one language protocol of the repository (`⚙️runtime/🗣️language/🔣️.json`) without selecting a default.
//! The explicit variable, the preference list and the first set locale variable decide; the operating system list is read by the `repo` door
//! that starts the dashboard and hands its choice over as the explicit variable, so a dashboard started without any choice asks instead.
use serde::Deserialize;
use std::sync::OnceLock;
use ui_locale::Locale;
const DECLARATION: &str = include_str!("🔣️.json");
const PROTOCOL: &str = include_str!("../../../../../../🔨️modules/⚙️runtime/🗣️language/🔣️.json");
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Choice { pub language: String, pub name: String }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document { #[serde(rename="$schema",default)] _schema: Option<String>, choices: Vec<Choice>, prompt: Vec<String>, messages: std::collections::BTreeMap<String,String> }
/// 📜️ The part of the language protocol the dashboard reads.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Protocol { #[serde(rename="explicitVariable")] explicit_variable: String, #[serde(rename="preferenceListVariable")] preference_list_variable: String, #[serde(rename="preferenceListSeparator")] preference_list_separator: String, #[serde(rename="environmentOrder")] environment_order: Vec<String>, #[serde(rename="subtagSeparators")] subtag_separators: String, #[serde(rename="neutralTags")] neutral_tags: Vec<String> }
/// 🔗️ The protocol and the explicit language choices; order never implies selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chain { protocol: Protocol, choices: Vec<Choice>, prompt: Vec<String>, messages: std::collections::BTreeMap<String,String> }
pub type Lookup<'a> = &'a dyn Fn(&str) -> Option<String>;
/// 🌍️ Reads a nonempty system setting.
pub fn process_environment(name: &str) -> Option<String> { std::env::var(name).ok().filter(|value| !value.is_empty()) }
/// 📖️ The validated language protocol of the repository.
pub fn protocol() -> &'static Protocol { static PROTOCOL_ROW:OnceLock<Protocol>=OnceLock::new(); PROTOCOL_ROW.get_or_init(||serde_json::from_str(PROTOCOL).expect("valid language protocol")) }
impl Protocol {
    /// 🔤️ The language subtag of a locale or tag; neutral locales such as C and POSIX carry none.
    pub fn subtag(&self, value: &str) -> Option<String> { let tag=value.trim().to_lowercase().split(|c| self.subtag_separators.contains(c)).next().unwrap_or("").to_string(); (!tag.is_empty() && !self.neutral_tags.contains(&tag)).then_some(tag) }
    /// 🏷️ The variable that chooses the language explicitly.
    pub fn explicit_variable(&self) -> &str { &self.explicit_variable }
}
/// 🗣️ Names a supported language; C, POSIX and unsupported locales resolve to none.
pub fn negotiate(value: &str) -> Option<Locale> { protocol().subtag(value).and_then(|tag| Locale::parse(&tag)) }
fn bounded(text: &str, maximum: usize) -> bool { !text.is_empty() && text.chars().count()<=maximum && !text.chars().any(char::is_control) }
impl Chain {
    /// 🔍️ Validates the closed authored choices and bilingual startup prompt.
    pub fn parse(text: &str) -> Result<Self,String> {
        let document: Document=serde_json::from_str(text).map_err(|error| error.to_string())?;
        if document.choices.len()!=2 || document.choices.iter().enumerate().any(|(at,choice)| Locale::parse(&choice.language).is_none() || !bounded(&choice.name,64) || document.choices[..at].iter().any(|previous| previous.language==choice.language)) { return Err("distinct supported language choices required".into()); }
        if document.prompt.len()!=3 || document.prompt.iter().any(|line| !bounded(line,256)) { return Err("bounded bilingual language prompt required".into()); }
        let keys=["saving","saveFailed","disconnected","terminalUnavailable","required","cancelled"];if document.messages.len()!=keys.len() || keys.iter().any(|key|document.messages.get(*key).is_none_or(|value|!bounded(value,256))){return Err("closed bilingual language messages required".into());}
        Ok(Self{protocol:protocol().clone(),choices:document.choices,prompt:document.prompt,messages:document.messages})
    }
    /// 🖥️ Resolves the explicit variable, then the first offered entry of the preference list, then the first set locale variable, without a fallback.
    pub fn resolve(&self, lookup: Lookup<'_>) -> Option<Locale> {
        let p=&self.protocol;
        if let Some(explicit)=lookup(&p.explicit_variable).filter(|value| !value.is_empty()) { return negotiate(&explicit); }
        if let Some(locale)=lookup(&p.preference_list_variable).unwrap_or_default().split(p.preference_list_separator.as_str()).find_map(negotiate) { return Some(locale); }
        p.environment_order.iter().find_map(|name| lookup(name).filter(|value| !value.is_empty())).and_then(|value| negotiate(&value))
    }
    pub fn choices(&self) -> &[Choice] { &self.choices }
    pub fn prompt(&self) -> &[String] { &self.prompt }
    pub fn message(&self,key:&str)->&str { self.messages.get(key).expect("authored language message") }
}
/// 📜️ Reads one validated authored language policy.
pub fn declared() -> &'static Chain { static CHAIN:OnceLock<Chain>=OnceLock::new(); CHAIN.get_or_init(||Chain::parse(DECLARATION).expect("valid authored language choices")) }
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"] mod tests;
