//! ⚙️ Opinionated dashboard preferences projected from shared and local command events.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

const LIMIT: u64 = 1024 * 1024;

/// 🎛️ Effective presentation and launch choices, owned by the dashboard.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preferences {
    pub language: String,
    pub appearance: String,
    pub terminology: String,
    pub renderer: String,
    pub layout: String,
}

impl Default for Preferences {
    fn default() -> Self { Self { language: "en".into(), appearance: "dark".into(), terminology: "native".into(), renderer: "react".into(), layout: "tabs".into() } }
}

/// ✉️ One validated preference command, matching the language-neutral schema.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Change {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appearance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminology: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renderer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
}

fn invalid(message: impl Into<String>) -> io::Error { io::Error::new(io::ErrorKind::InvalidData, message.into()) }

impl Change {
    /// 🔎️ Validates an optional customization without accepting implicit nulls or unknown keys.
    pub fn parse(text: &str) -> io::Result<Self> {
        let value: serde_json::Value = serde_json::from_str(text).map_err(|error| invalid(error.to_string()))?;
        if !value.as_object().is_some_and(|object| !object.is_empty() && object.values().all(serde_json::Value::is_string)) { return Err(invalid("preference changes must contain string values")); }
        let change: Self = serde_json::from_value(value).map_err(|error| invalid(error.to_string()))?;
        change.validate()?;
        Ok(change)
    }

    fn validate(&self) -> io::Result<()> {
        let values: [(&str, &Option<String>, &[&str]); 5] = [("language", &self.language, &["en", "de"]), ("appearance", &self.appearance, &["dark", "light"]), ("terminology", &self.terminology, &["native", "reuse"]), ("renderer", &self.renderer, &["react", "wgpu-wasm", "wgpu-native"]), ("layout", &self.layout, &["tabs", "columns", "rows"])];
        if values.iter().all(|(_, value, _)| value.is_none()) { return Err(invalid("empty preference change")); }
        for (key, value, allowed) in values { if let Some(value) = value { if !allowed.contains(&value.as_str()) { return Err(invalid(format!("invalid {key}: {value}; choose {}", allowed.join(", ")))); } } }
        Ok(())
    }
}

impl Preferences {
    /// 🧮️ Projects one preference event over the current settings.
    pub fn apply(&mut self, change: &Change) {
        for (target, value) in [(&mut self.language, &change.language), (&mut self.appearance, &change.appearance), (&mut self.terminology, &change.terminology), (&mut self.renderer, &change.renderer), (&mut self.layout, &change.layout)] { if let Some(value) = value { target.clone_from(value); } }
    }

    /// 🚀️ Binds launch defaults only where a command declares that presentation axis.
    pub fn bind(&self, command: &mut crate::command_tree::CommandSpec) {
        for (name, value) in [("SEMIO_LOCKED_LOCALE", &self.language), ("SEMIO_LOCALE", &self.language), ("VITE_SEMIO_LOCKED_LOCALE", &self.language), ("SEMIO_LOCKED_TERMINOLOGY", &self.terminology), ("VITE_SEMIO_LOCKED_TERMINOLOGY", &self.terminology), ("SEMIO_LOCKED_APPEARANCE", &self.appearance)] {
            for (key, existing) in &mut command.env { if key == name { existing.clone_from(value); } }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Event { version: u32, revision: u64, change: Change }

fn project(file: &mut File, preferences: &mut Preferences) -> io::Result<u64> {
    if file.metadata()?.len() > LIMIT { return Err(invalid("preference journal exceeds 1 MiB")); }
    file.seek(SeekFrom::Start(0))?;
    let mut text = String::new(); file.take(LIMIT + 1).read_to_string(&mut text)?;
    if text.len() as u64 > LIMIT || !text.is_empty() && !text.ends_with('\n') { return Err(invalid("incomplete preference event journal")); }
    let mut revision = 0;
    for line in text.lines() {
        let value: serde_json::Value = serde_json::from_str(line).map_err(|error| invalid(error.to_string()))?;
        let change = Change::parse(&value.get("change").ok_or_else(|| invalid("preference event missing change"))?.to_string())?;
        let event: Event = serde_json::from_str(line).map_err(|error| invalid(error.to_string()))?;
        if event.version != 1 || event.revision != revision + 1 { return Err(invalid("preference events must have consecutive revisions at version 1")); }
        preferences.apply(&change); revision = event.revision;
    }
    Ok(revision)
}

/// 📖️ Replays a bounded event journal; absence means the opinionated defaults remain.
pub fn replay(path: &Path, preferences: &mut Preferences) -> io::Result<u64> {
    let mut file = match File::open(path) { Ok(file) => file, Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(0), Err(error) => return Err(error) };
    file.try_lock_shared().map_err(|error| io::Error::other(format!("preferences are being published: {error}")))?;
    project(&mut file, preferences)
}

/// 💾️ Commits one durable event under an operating-system lock released on process death.
pub fn append(path: &Path, change: &Change) -> io::Result<u64> {
    change.validate()?;
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    let mut file = OpenOptions::new().create(true).read(true).append(true).open(path)?;
    let started = std::time::Instant::now();
    loop { match file.try_lock() { Ok(()) => break, Err(_) if started.elapsed().as_secs() < 2 => std::thread::sleep(std::time::Duration::from_millis(5)), Err(error) => return Err(io::Error::other(format!("preference publication busy: {error}"))) } }
    let revision = project(&mut file, &mut Preferences::default())? + 1;
    let mut bytes = serde_json::to_vec(&Event { version: 1, revision, change: change.clone() }).map_err(|error| invalid(error.to_string()))?;
    bytes.push(b'\n');
    if file.metadata()?.len() + bytes.len() as u64 > LIMIT { return Err(invalid("preference journal exceeds 1 MiB")); }
    file.write_all(&bytes)?; file.sync_data()?;
    Ok(revision)
}

/// 📍️ Local-only preferences; the explicit config path overrides the workspace-local journal.
pub fn local_path(root: &Path, parsed: &crate::args::ParsedArgs) -> PathBuf {
    parsed.flag("config").map(PathBuf::from).or_else(|| std::env::var_os("SEMIO_DASHBOARD_CONFIG").map(PathBuf::from)).map(|path| if path.is_absolute() { path } else { root.join(path) }).unwrap_or_else(|| root.join(".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/preferences.jsonl"))
}

/// 🌍️ Workspace-shared preference events, separate from local customization.
pub fn shared_path(root: &Path) -> PathBuf { root.join(".🧬semio/🦑️repo/🎛️dashboard/⚙️preferences.jsonl") }

/// 🥇️ Applies strict explicit flags last, with no startup questions.
pub fn apply_arguments(preferences: &mut Preferences, parsed: &crate::args::ParsedArgs) -> io::Result<()> {
    let mut map = serde_json::Map::new();
    for (key, value) in &parsed.flags {
        if ["root", "config"].contains(&key.as_str()) { if value.as_ref().is_none_or(String::is_empty) { return Err(invalid(format!("--{key} requires a path"))); } continue; }
        if ["help", "workspace"].contains(&key.as_str()) { if value.is_some() { return Err(invalid(format!("--{key} does not accept a value"))); } continue; }
        if !["language", "appearance", "terminology", "renderer", "layout"].contains(&key.as_str()) { return Err(invalid(format!("unknown dashboard option --{key}"))); }
        let value = value.as_ref().ok_or_else(|| invalid(format!("--{key} requires a value")))?;
        map.insert(key.clone(), serde_json::Value::String(value.clone()));
    }
    if !map.is_empty() { preferences.apply(&Change::parse(&serde_json::Value::Object(map).to_string())?); }
    Ok(())
}

/// 🧭️ Resolves defaults, shared events, local events, environment and explicit arguments.
pub fn load(root: &Path, parsed: &crate::args::ParsedArgs) -> io::Result<Preferences> {
    let mut preferences = Preferences::default();
    replay(&shared_path(root), &mut preferences)?; replay(&local_path(root, parsed), &mut preferences)?;
    let mut map = serde_json::Map::new();
    for (key, variable) in [("language", "SEMIO_LOCALE"), ("appearance", "SEMIO_APPEARANCE"), ("terminology", "SEMIO_LOCKED_TERMINOLOGY"), ("renderer", "SEMIO_DASHBOARD_RENDERER"), ("layout", "SEMIO_DASHBOARD_LAYOUT")] {
        if let Ok(value) = std::env::var(variable) { if !value.is_empty() { map.insert(key.into(), serde_json::Value::String(value)); } }
    }
    if !map.is_empty() { preferences.apply(&Change::parse(&serde_json::Value::Object(map).to_string())?); }
    apply_arguments(&mut preferences, parsed)?;
    Ok(preferences)
}

/// ⌨️ Queries effective preferences or records an explicit customization command.
pub fn run(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {
    let root = parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from);
    let result = (|| -> io::Result<Preferences> {
        if parsed.segments.first().map(String::as_str) == Some("set") {
            let mut map = serde_json::Map::new();
            for (key, value) in &parsed.flags { if ["language", "appearance", "terminology", "renderer", "layout"].contains(&key.as_str()) { map.insert(key.clone(), value.clone().map_or(serde_json::Value::Null, serde_json::Value::String)); } }
            let change = Change::parse(&serde_json::Value::Object(map).to_string())?;
            apply_arguments(&mut Preferences::default(), parsed)?;
            append(&if parsed.has_flag("workspace") { shared_path(&root) } else { local_path(&root, parsed) }, &change)?;
        } else if !parsed.segments.is_empty() && parsed.segments[0] != "show" { return Err(invalid("preferences accepts show or set")); }
        load(&root, parsed)
    })();
    match result { Ok(preferences) => { println!("{}", serde_json::to_string(&preferences).unwrap_or_default()); 0 }, Err(error) => { eprintln!("[dashboard] {error}"); 2 } }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
