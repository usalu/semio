//! ⚙️ Opinionated dashboard preferences projected from shared and local command events.

use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[path = "⌨️keymap/🦀️.rs"]
pub mod keymap;

const LIMIT: u64 = 1024 * 1024;
const PREFIX: &str = "ctrl+b";
const LOCK_PATIENCE: std::time::Duration = std::time::Duration::from_secs(2);

/// 🎛️ Effective presentation and launch choices, owned by the dashboard.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preferences {
    pub language: String,
    pub appearance: String,
    pub terminology: String,
    pub renderer: String,
    pub layout: String,
    pub prefix: String,
    pub bindings: BTreeMap<String, Vec<String>>,
}

impl Default for Preferences {
    fn default() -> Self { Self { language: "en".into(), appearance: "dark".into(), terminology: "native".into(), renderer: "react".into(), layout: "tabs".into(), prefix: PREFIX.into(), bindings: BTreeMap::new() } }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<BTreeMap<String, Vec<String>>>,
}

fn invalid(message: impl Into<String>) -> io::Error { io::Error::new(io::ErrorKind::InvalidData, message.into()) }

impl Change {
    /// 🔎️ Validates an optional customization without accepting implicit nulls or unknown keys.
    pub fn parse(text: &str) -> io::Result<Self> {
        let value: serde_json::Value = serde_json::from_str(text).map_err(|error| invalid(error.to_string()))?;
        if !value.as_object().is_some_and(|object| !object.is_empty() && object.iter().all(|(key, value)| if key == "bindings" { value.is_object() } else { value.is_string() })) { return Err(invalid("preference changes must contain string values, and bindings an object")); }
        let change: Self = serde_json::from_value(value).map_err(|error| invalid(error.to_string()))?;
        change.validate()?;
        Ok(change)
    }

    fn validate(&self) -> io::Result<()> {
        let values: [(&str, &Option<String>, &[&str]); 5] = [("language", &self.language, &["en", "de"]), ("appearance", &self.appearance, &["dark", "light"]), ("terminology", &self.terminology, &["native", "reuse"]), ("renderer", &self.renderer, &["react", "wgpu-wasm", "wgpu-native"]), ("layout", &self.layout, &["tabs", "columns", "rows"])];
        if values.iter().all(|(_, value, _)| value.is_none()) && self.prefix.is_none() && self.bindings.is_none() { return Err(invalid("empty preference change")); }
        for (key, value, allowed) in values { if let Some(value) = value { if !allowed.contains(&value.as_str()) { return Err(invalid(format!("invalid {key}: {value}; choose {}", allowed.join(", ")))); } } }
        if let Some(prefix) = &self.prefix { keymap::Keymap::check_prefix(prefix).map_err(|error| invalid(format!("invalid prefix: {error}")))?; }
        if let Some(bindings) = &self.bindings {
            if bindings.is_empty() { return Err(invalid("bindings must name at least one binding")); }
            for (id, keys) in bindings { keymap::Keymap::check_binding(id, keys).map_err(|error| invalid(format!("invalid binding {id}: {error}")))?; }
        }
        Ok(())
    }
}

impl Preferences {
    /// 🧮️ Projects one preference event over the current settings.
    pub fn apply(&mut self, change: &Change) {
        for (target, value) in [(&mut self.language, &change.language), (&mut self.appearance, &change.appearance), (&mut self.terminology, &change.terminology), (&mut self.renderer, &change.renderer), (&mut self.layout, &change.layout), (&mut self.prefix, &change.prefix)] { if let Some(value) = value { target.clone_from(value); } }
        if let Some(bindings) = &change.bindings { for (id, keys) in bindings { self.bindings.insert(id.clone(), keys.clone()); } }
    }

    /// ⌨️ The effective keymap and the customizations it had to ignore, each named.
    pub fn keymap(&self) -> (keymap::Keymap, Vec<String>) { keymap::Keymap::build(&self.prefix, &self.bindings) }

    /// 🌐️ The language the dashboard speaks; changes are validated, so an unknown language never occurs.
    pub fn locale(&self) -> ui_locale::Locale { ui_locale::Locale::parse(&self.language).unwrap_or_default() }

    /// 🚀️ The parameter values these preferences pre-select among the parameters a command offers.
    pub fn launch_defaults(&self, accepted: &[crate::registry::Parameter]) -> Vec<(String, String)> {
        [("language", &self.language), ("terminology", &self.terminology), ("appearance", &self.appearance), ("renderer", &self.renderer)].into_iter()
            .filter(|(id, value)| accepted.iter().any(|parameter| parameter.id == *id && parameter.values.iter().any(|(known, _)| known == *value)))
            .map(|(id, value)| (id.to_string(), value.clone())).collect()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Event { version: u32, revision: u64, change: Change }

/// 📖️ Folds the complete events of a journal into `preferences`. A last line the writer did not finish
/// (a crash between write and newline) is ignored; the byte length up to the last complete event is returned
/// so the next writer can cut the torn tail off.
fn project(file: &mut File, preferences: &mut Preferences) -> io::Result<(u64, u64)> {
    if file.metadata()?.len() > LIMIT { return Err(invalid("preference journal exceeds 1 MiB")); }
    file.seek(SeekFrom::Start(0))?;
    let mut bytes = Vec::new(); file.take(LIMIT + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > LIMIT { return Err(invalid("preference journal exceeds 1 MiB")); }
    let complete = bytes.iter().rposition(|byte| *byte == b'\n').map_or(0, |at| at + 1);
    let text = std::str::from_utf8(&bytes[..complete]).map_err(|error| invalid(error.to_string()))?;
    let mut revision = 0;
    for line in text.lines() {
        let value: serde_json::Value = serde_json::from_str(line).map_err(|error| invalid(error.to_string()))?;
        let change = Change::parse(&value.get("change").ok_or_else(|| invalid("preference event missing change"))?.to_string())?;
        let event: Event = serde_json::from_str(line).map_err(|error| invalid(error.to_string()))?;
        if event.version != 1 || event.revision != revision + 1 { return Err(invalid("preference events must have consecutive revisions at version 1")); }
        preferences.apply(&change); revision = event.revision;
    }
    Ok((revision, complete as u64))
}

/// ⏳️ Takes a journal lock, waiting a bounded time for another process to finish publishing.
fn lock(file: &File, path: &Path, shared: bool) -> io::Result<()> {
    let started = std::time::Instant::now();
    loop {
        let attempt = if shared { file.try_lock_shared() } else { file.try_lock() };
        match attempt {
            Ok(()) => return Ok(()),
            Err(_) if started.elapsed() < LOCK_PATIENCE => std::thread::sleep(std::time::Duration::from_millis(5)),
            Err(error) => return Err(io::Error::other(format!("{} is busy: another dashboard has been publishing preferences for more than {} seconds ({error})", path.display(), LOCK_PATIENCE.as_secs()))),
        }
    }
}

/// 📼️ Replays a bounded event journal; absence means the opinionated defaults remain.
pub fn replay(path: &Path, preferences: &mut Preferences) -> io::Result<u64> {
    let mut file = match File::open(path) { Ok(file) => file, Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(0), Err(error) => return Err(error) };
    lock(&file, path, true)?;
    project(&mut file, preferences).map(|(revision, _)| revision)
}

/// 🗜️ The single event that states every setting the journal currently holds.
fn snapshot(preferences: &Preferences) -> Change {
    Change { language: Some(preferences.language.clone()), appearance: Some(preferences.appearance.clone()), terminology: Some(preferences.terminology.clone()), renderer: Some(preferences.renderer.clone()), layout: Some(preferences.layout.clone()), prefix: Some(preferences.prefix.clone()), bindings: Some(preferences.bindings.clone()).filter(|bindings| !bindings.is_empty()) }
}

fn line(revision: u64, change: &Change) -> io::Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(&Event { version: 1, revision, change: change.clone() }).map_err(|error| invalid(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// 💾️ Commits one durable event under an operating-system lock released on process death. A torn last
/// line is cut off first, and a journal that would outgrow its limit is compacted into one event.
pub fn append(path: &Path, change: &Change) -> io::Result<u64> {
    change.validate()?;
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    let mut file = OpenOptions::new().create(true).read(true).write(true).truncate(false).open(path)?;
    lock(&file, path, false)?;
    let mut current = Preferences::default();
    let (revision, complete) = project(&mut file, &mut current)?;
    if file.metadata()?.len() > complete { file.set_len(complete)?; }
    let mut bytes = line(revision + 1, change)?;
    let mut next = revision + 1;
    if complete + bytes.len() as u64 > LIMIT {
        let mut compacted = line(1, &snapshot(&current))?;
        bytes = line(2, change)?;
        compacted.extend_from_slice(&bytes);
        if compacted.len() as u64 > LIMIT { return Err(invalid("preference journal exceeds 1 MiB")); }
        file.set_len(0)?; file.seek(SeekFrom::Start(0))?;
        bytes = compacted; next = 2;
    } else {
        file.seek(SeekFrom::End(0))?;
    }
    file.write_all(&bytes)?; file.sync_data()?;
    Ok(next)
}

/// 📍️ Local-only preferences; the explicit config path overrides the workspace-local journal.
pub fn local_path(root: &Path, parsed: &crate::args::ParsedArgs) -> PathBuf {
    parsed.flag("config").map(PathBuf::from).or_else(|| std::env::var_os("SEMIO_DASHBOARD_CONFIG").map(PathBuf::from)).map_or_else(|| root.join(".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/preferences.jsonl"), |path| if path.is_absolute() { path } else { root.join(path) })
}

/// 🌍️ Workspace-shared preference events, separate from local customization.
pub fn shared_path(root: &Path) -> PathBuf { root.join(".🧬semio/🦑️repo/🎛️dashboard/⚙️preferences.jsonl") }

/// 🗨️ The language a command speaks to the developer: the stored and environment preferences, or the first language when
/// they cannot be read; it never fails, because it only words a message.
pub fn cli_locale(root: &Path, parsed: &crate::args::ParsedArgs) -> ui_locale::Locale { load_lenient(root, parsed).map_or_else(|_| ui_locale::Locale::default(), |(preferences, _)| preferences.locale()) }

/// 🥇️ Applies strict explicit flags last, with no startup questions.
pub fn apply_arguments(preferences: &mut Preferences, parsed: &crate::args::ParsedArgs) -> io::Result<()> {
    let mut map = serde_json::Map::new();
    for (key, value) in &parsed.flags {
        if ["root", "config"].contains(&key.as_str()) { if value.as_ref().is_none_or(String::is_empty) { return Err(invalid(format!("--{key} requires a path"))); } continue; }
        if ["help", "workspace"].contains(&key.as_str()) { if value.is_some() { return Err(invalid(format!("--{key} does not accept a value"))); } continue; }
        if !["language", "appearance", "terminology", "renderer", "layout", "prefix", "bindings"].contains(&key.as_str()) { return Err(invalid(format!("unknown dashboard option --{key}"))); }
        let value = value.as_ref().ok_or_else(|| invalid(format!("--{key} requires a value")))?;
        map.insert(key.clone(), if key == "bindings" { serde_json::from_str(value).map_err(|error| invalid(format!("--bindings takes a JSON object of binding ids to key lists: {error}")))? } else { serde_json::Value::String(value.clone()) });
    }
    if !map.is_empty() { preferences.apply(&Change::parse(&serde_json::Value::Object(map).to_string())?); }
    Ok(())
}

/// 🧭️ Resolves defaults, shared events, local events, environment and explicit arguments.
pub fn load(root: &Path, parsed: &crate::args::ParsedArgs) -> io::Result<Preferences> {
    let mut preferences = Preferences::default();
    replay(&shared_path(root), &mut preferences)?; replay(&local_path(root, parsed), &mut preferences)?;
    let mut map = serde_json::Map::new();
    for (key, variable) in [("language", "SEMIO_LOCALE"), ("appearance", "SEMIO_APPEARANCE"), ("terminology", "SEMIO_LOCKED_TERMINOLOGY"), ("renderer", "SEMIO_DASHBOARD_RENDERER"), ("layout", "SEMIO_DASHBOARD_LAYOUT"), ("prefix", "SEMIO_DASHBOARD_PREFIX")] {
        if let Ok(value) = std::env::var(variable) { if !value.is_empty() { map.insert(key.into(), serde_json::Value::String(value)); } }
    }
    if !map.is_empty() { preferences.apply(&Change::parse(&serde_json::Value::Object(map).to_string())?); }
    apply_arguments(&mut preferences, parsed)?;
    Ok(preferences)
}

/// 🩹️ Like [`load`], but a journal that cannot be read (torn beyond repair, locked too long, foreign
/// content) never stops the dashboard: it is skipped and named in the returned problems. Explicit
/// arguments and environment stay strict, because the developer typed them just now.
pub fn load_lenient(root: &Path, parsed: &crate::args::ParsedArgs) -> io::Result<(Preferences, Vec<String>)> {
    let mut preferences = Preferences::default();
    let mut problems = Vec::new();
    for path in [shared_path(root), local_path(root, parsed)] {
        let mut layer = preferences.clone();
        match replay(&path, &mut layer) { Ok(_) => preferences = layer, Err(error) => problems.push(format!("{}: {error}", path.display())) }
    }
    let mut map = serde_json::Map::new();
    for (key, variable) in [("language", "SEMIO_LOCALE"), ("appearance", "SEMIO_APPEARANCE"), ("terminology", "SEMIO_LOCKED_TERMINOLOGY"), ("renderer", "SEMIO_DASHBOARD_RENDERER"), ("layout", "SEMIO_DASHBOARD_LAYOUT"), ("prefix", "SEMIO_DASHBOARD_PREFIX")] {
        if let Ok(value) = std::env::var(variable) { if !value.is_empty() { map.insert(key.into(), serde_json::Value::String(value)); } }
    }
    if !map.is_empty() { preferences.apply(&Change::parse(&serde_json::Value::Object(map).to_string())?); }
    apply_arguments(&mut preferences, parsed)?;
    Ok((preferences, problems))
}

/// 📝️ Queries effective preferences or records an explicit customization command.
pub fn run(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {
    let root = parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from);
    let result = (|| -> io::Result<Preferences> {
        if parsed.segments.first().map(String::as_str) == Some("set") {
            let mut map = serde_json::Map::new();
            for (key, value) in &parsed.flags {
                if key == "bindings" { map.insert(key.clone(), value.as_deref().map_or(Ok(serde_json::Value::Null), serde_json::from_str).map_err(|error| invalid(format!("--bindings takes a JSON object of binding ids to key lists: {error}")))?); }
                else if ["language", "appearance", "terminology", "renderer", "layout", "prefix"].contains(&key.as_str()) { map.insert(key.clone(), value.clone().map_or(serde_json::Value::Null, serde_json::Value::String)); }
            }
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
