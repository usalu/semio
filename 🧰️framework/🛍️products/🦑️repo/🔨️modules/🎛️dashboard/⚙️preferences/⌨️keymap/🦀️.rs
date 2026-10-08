//! ⌨️ The declarative keymap: a prefix key and bindings as data with action ids. The defaults ship as
//! data; the user's customizations arrive as preference events and are folded over them. Nothing
//! here knows a terminal, a widget or a language, so every implementation resolves the same keys.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/⚙️preferences/⌨️keymap/🔣️.json

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::OnceLock;

const DEFAULTS: &str = include_str!("🔣️.json");
const NAMED: &[&str] = &["enter", "esc", "tab", "backtab", "backspace", "delete", "insert", "up", "down", "left", "right", "home", "end", "pageup", "pagedown"];
const INDISTINGUISHABLE: &[char] = &['i', 'j', 'm', 'h'];

// #region 🔖️Key
/// 🔑️ What a key is called: a character or a named key.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KeyName { Char(char), Named(String) }

/// 🎹️ One key press: a name and the modifiers held with it, in the canonical form every comparison uses.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeySpec { pub name: KeyName, pub ctrl: bool, pub alt: bool, pub shift: bool }

impl KeySpec {
    /// 🔍️ Reads `ctrl+b`, `alt+x`, `shift+tab`, `Q`, `|` or `f5`; the error names what is wrong.
    pub fn parse(text: &str) -> Result<Self, String> {
        let (mut ctrl, mut alt, mut shift) = (false, false, false);
        let mut rest = text;
        while let Some((head, tail)) = rest.split_once('+').filter(|(_, tail)| !tail.is_empty()) {
            match head {
                "ctrl" => ctrl = true,
                "alt" => alt = true,
                "shift" => shift = true,
                _ => return Err(format!("unknown modifier {head:?} in key {text:?}")),
            }
            rest = tail;
        }
        let mut chars = rest.chars();
        let name = match (chars.next(), chars.next()) {
            (None, _) => return Err("a key is never empty".into()),
            (Some(c), None) if c.is_whitespace() => return Err(format!("write the space key as `space`, not {text:?}")),
            (Some(c), None) if c.is_control() => return Err(format!("key {text:?} is a control character")),
            (Some(c), None) => KeyName::Char(c),
            (Some(_), Some(_)) => Self::named(rest).ok_or_else(|| format!("unknown key name {rest:?}; choose a character, space, {}, or f1 to f24", NAMED.join(", ")))?,
        };
        Self { name, ctrl, alt, shift }.canonical().map_err(|reason| format!("{reason}: {text:?}"))
    }

    fn named(name: &str) -> Option<KeyName> {
        if name == "space" { return Some(KeyName::Char(' ')); }
        if NAMED.contains(&name) { return Some(KeyName::Named(name.into())); }
        name.strip_prefix('f').and_then(|number| number.parse::<u8>().ok()).filter(|number| (1..=24).contains(number) && name.len() == 1 + number.to_string().len()).map(|_| KeyName::Named(name.into()))
    }

    /// 🧭️ Folds equivalent spellings into one: shifted letters are capitals, shifted tab is backtab, control letters are lowercase.
    pub fn canonical(mut self) -> Result<Self, String> {
        match &mut self.name {
            KeyName::Char(c) if self.shift => {
                if self.ctrl { return Err("a control letter cannot also be shifted".into()); }
                if !c.is_ascii_alphabetic() { return Err("shift applies to letters and named keys; write the shifted character itself".into()); }
                *c = c.to_ascii_uppercase();
                self.shift = false;
            }
            KeyName::Char(c) if self.ctrl => {
                if *c != ' ' && !c.is_ascii_alphabetic() { return Err("control applies to letters and space".into()); }
                *c = c.to_ascii_lowercase();
                if INDISTINGUISHABLE.contains(c) { return Err("a terminal sends this control letter as tab, enter or backspace".into()); }
            }
            KeyName::Named(name) if name == "tab" && self.shift => { *name = "backtab".into(); self.shift = false; }
            KeyName::Named(name) if name == "backtab" => self.shift = false,
            _ => {}
        }
        Ok(self)
    }

    /// 🔤️ Whether pressing it only types: a character without control or alt.
    pub fn is_plain_printable(&self) -> bool { matches!(self.name, KeyName::Char(_)) && !self.ctrl && !self.alt }

    /// 🗣️ The words a person reads for the key: `Ctrl+B`, `Shift+Tab`, `PgUp`, `Space`.
    pub fn label(&self) -> String {
        let (mut prefix, mut shift) = (String::new(), self.shift);
        if self.ctrl { prefix.push_str("Ctrl+"); }
        if self.alt { prefix.push_str("Alt+"); }
        let base = match &self.name {
            KeyName::Char(' ') => "Space".to_string(),
            KeyName::Char(c) if self.ctrl => c.to_ascii_uppercase().to_string(),
            KeyName::Char(c) => c.to_string(),
            KeyName::Named(name) => match name.as_str() {
                "backtab" => { shift = true; "Tab".into() }
                "pageup" => "PgUp".into(),
                "pagedown" => "PgDn".into(),
                other if other.starts_with('f') && other.len() > 1 => other.to_ascii_uppercase(),
                other => { let mut letters = other.chars(); letters.next().map(|first| first.to_ascii_uppercase().to_string() + letters.as_str()).unwrap_or_default() }
            },
        };
        if shift { prefix.push_str("Shift+"); }
        prefix + &base
    }
}

impl fmt::Display for KeySpec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ctrl { formatter.write_str("ctrl+")?; }
        if self.alt { formatter.write_str("alt+")?; }
        if self.shift { formatter.write_str("shift+")?; }
        match &self.name {
            KeyName::Char(' ') => formatter.write_str("space"),
            KeyName::Char(c) => write!(formatter, "{c}"),
            KeyName::Named(name) => formatter.write_str(name),
        }
    }
}
// #endregion 🔖️Key

// #region 🔖️Scope
/// 🎚️ Where a binding applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    /// 🥇️ Pressed after the prefix key; the only scope a running terminal's keyboard honours.
    Prefix,
    /// 🪟️ Pressed directly while no terminal owns the keyboard: windows and tabs.
    Window,
    /// 📋️ Pressed directly inside the launcher, task list, settings and help.
    View,
}

impl Scope {
    pub const ALL: [Scope; 3] = [Scope::Prefix, Scope::Window, Scope::View];

    pub fn as_str(self) -> &'static str { match self { Self::Prefix => "prefix", Self::Window => "window", Self::View => "view" } }

    fn direct(self) -> bool { self != Self::Prefix }
}
// #endregion 🔖️Scope

// #region 🔖️Keymap
/// 🔗️ One action in one scope and the keys that trigger it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binding { pub scope: Scope, pub action: String, pub keys: Vec<KeySpec> }

impl Binding {
    /// 🪪️ The identity customizations name: `<scope>.<action>`.
    pub fn id(&self) -> String { format!("{}.{}", self.scope.as_str(), self.action) }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    #[serde(rename = "$schema", default)]
    _schema: Option<String>,
    prefix: String,
    bindings: Vec<BindingDocument>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BindingDocument { scope: Scope, action: String, keys: Vec<String> }

/// 🗂️ The effective keymap: a prefix key and every binding, resolvable by scope and key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keymap { prefix: KeySpec, bindings: Vec<Binding> }

/// 📌️ The pseudo action the prefix key triggers after itself: the prefix key goes to the terminal.
pub const SEND_PREFIX: &str = "send-prefix";

impl Keymap {
    /// 📦️ The shipped defaults; they are checked conflict-free by the tests.
    pub fn defaults() -> Self {
        static DEFAULT: OnceLock<Keymap> = OnceLock::new();
        DEFAULT.get_or_init(|| {
            let document: Document = serde_json::from_str(DEFAULTS).expect("the shipped keymap is valid JSON of its schema");
            let bindings = document.bindings.into_iter().map(|row| Binding { scope: row.scope, action: row.action, keys: row.keys.iter().map(|key| KeySpec::parse(key).expect("the shipped keymap holds valid keys")).collect() }).collect();
            Self { prefix: KeySpec::parse(&document.prefix).expect("the shipped prefix is valid"), bindings }
        }).clone()
    }

    /// 🧮️ Folds the user's prefix and per-binding keys over the defaults. A customization that is
    /// invalid or collides with another key is ignored and named in the returned problems, so the
    /// keyboard always works and nothing is dropped silently.
    pub fn build(prefix: &str, overrides: &BTreeMap<String, Vec<String>>) -> (Self, Vec<String>) {
        let base = Self::defaults();
        let (mut keymap, mut problems, mut changed) = (base.clone(), Vec::new(), BTreeSet::new());
        match Self::check_prefix(prefix) {
            Ok(key) if key != base.prefix => { keymap.prefix = key; changed.insert("prefix".to_string()); }
            Ok(_) => {}
            Err(error) => problems.push(format!("prefix: {error}")),
        }
        for (id, keys) in overrides {
            match Self::check_binding(id, keys) {
                Ok(keys) => {
                    if let Some(binding) = keymap.bindings.iter_mut().find(|binding| &binding.id() == id) { binding.keys = keys; changed.insert(id.clone()); }
                }
                Err(error) => problems.push(format!("{id}: {error}")),
            }
        }
        while let Some((scope, key, ids)) = keymap.conflict() {
            let culprits: Vec<String> = ids.iter().filter(|id| changed.contains(*id)).cloned().collect();
            if culprits.is_empty() { break; }
            for id in &culprits {
                changed.remove(id);
                if id == "prefix" { keymap.prefix = base.prefix.clone(); } else if let (Some(binding), Some(original)) = (keymap.bindings.iter_mut().find(|binding| &binding.id() == id), base.bindings.iter().find(|binding| &binding.id() == id)) { binding.keys.clone_from(&original.keys); }
            }
            problems.push(format!("{} is bound twice in {} ({}); the customization of {} is ignored", key.label(), scope.as_str(), ids.join(", "), culprits.join(", ")));
        }
        (keymap, problems)
    }

    /// 🛂️ Validates a prefix customization: it must be a key the terminal can tell from typing.
    pub fn check_prefix(text: &str) -> Result<KeySpec, String> {
        let key = KeySpec::parse(text)?;
        if key.is_plain_printable() { return Err(format!("{text:?} only types; the prefix needs ctrl, alt or a named key")); }
        Ok(key)
    }

    /// 🔏️ Validates the keys of one customized binding: it exists, has keys, repeats none and in a
    /// direct scope never binds a key that only types.
    pub fn check_binding(id: &str, keys: &[String]) -> Result<Vec<KeySpec>, String> {
        let scope = Self::defaults().bindings.iter().find(|binding| binding.id() == id).map(|binding| binding.scope).ok_or_else(|| format!("no such binding; choose one of {}", Self::defaults().bindings.iter().map(Binding::id).collect::<Vec<_>>().join(", ")))?;
        if keys.is_empty() { return Err("a binding needs at least one key, otherwise its action is unreachable".into()); }
        let mut parsed: Vec<KeySpec> = Vec::new();
        for text in keys {
            let key = KeySpec::parse(text)?;
            if scope.direct() && key.is_plain_printable() { return Err(format!("{text:?} only types; a {} binding needs ctrl, alt or a named key", scope.as_str())); }
            if parsed.contains(&key) { return Err(format!("{text:?} is listed twice")); }
            parsed.push(key);
        }
        Ok(parsed)
    }

    fn conflict(&self) -> Option<(Scope, KeySpec, Vec<String>)> {
        for scope in Scope::ALL {
            let mut holders: BTreeMap<KeySpec, Vec<String>> = BTreeMap::new();
            holders.entry(self.prefix.clone()).or_default().push("prefix".into());
            for binding in self.bindings.iter().filter(|binding| binding.scope == scope) { for key in &binding.keys { holders.entry(key.clone()).or_default().push(binding.id()); } }
            if let Some((key, ids)) = holders.into_iter().find(|(_, ids)| ids.len() > 1) { return Some((scope, key, ids)); }
        }
        None
    }

    /// 🪄️ The key that arms the prefix scope and, pressed twice, goes to the terminal.
    pub fn prefix(&self) -> &KeySpec { &self.prefix }

    /// 🎯️ Whether the key is the prefix key.
    pub fn is_prefix(&self, key: &KeySpec) -> bool { &self.prefix == key }

    /// 🕹️ The action a key triggers in a scope. In the prefix scope the prefix key itself is [`SEND_PREFIX`].
    pub fn resolve(&self, scope: Scope, key: &KeySpec) -> Option<&str> {
        if scope == Scope::Prefix && &self.prefix == key { return Some(SEND_PREFIX); }
        self.bindings.iter().find(|binding| binding.scope == scope && binding.keys.contains(key)).map(|binding| binding.action.as_str())
    }

    /// 📜️ Every binding of a scope in the order the defaults state them.
    pub fn bindings(&self, scope: Scope) -> impl Iterator<Item = &Binding> { self.bindings.iter().filter(move |binding| binding.scope == scope) }

    /// 📢️ The keys of an action as people read them, `n` or `] / o`, with the prefix key first for the prefix scope.
    pub fn keys_label(&self, scope: Scope, action: &str) -> Option<String> {
        let keys = if action == SEND_PREFIX && scope == Scope::Prefix { vec![self.prefix.clone()] } else { self.bindings(scope).find(|binding| binding.action == action)?.keys.clone() };
        let list = keys.iter().map(KeySpec::label).collect::<Vec<_>>().join(" / ");
        Some(if scope == Scope::Prefix { format!("{} {list}", self.prefix.label()) } else { list })
    }

    /// 🔁️ Every action id of a scope, [`SEND_PREFIX`] included for the prefix scope.
    pub fn actions(&self, scope: Scope) -> Vec<String> {
        let mut actions: Vec<String> = self.bindings(scope).map(|binding| binding.action.clone()).collect();
        if scope == Scope::Prefix { actions.push(SEND_PREFIX.into()); }
        actions
    }
}
// #endregion 🔖️Keymap

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
