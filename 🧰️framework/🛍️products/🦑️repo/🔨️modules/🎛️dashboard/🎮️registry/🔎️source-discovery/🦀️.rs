//! 🔎️ Lazy, cancellable suggestions from current physical authored sources and registered owners.

use super::*;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, SystemTime};
use std::io::Read;

/// 📄️ One editable source value and its actual deepest registered owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion { pub value: String, pub project: String, pub directory: String }

/// 📊️ A bounded suggestion page, including incomplete discovery and failures.
#[derive(Debug, Clone, Default)]
pub struct Suggestions { pub values: Vec<Suggestion>, pub inspected: usize, pub scanning: bool, pub truncated: bool, pub cancelled: bool, pub problem: Option<String> }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SourcePolicy { maximum_entries: usize, maximum_rows: usize, maximum_source_bytes: usize, maximum_read_bytes: usize, maximum_results: usize, maximum_query_characters: usize, maximum_query_words: usize, maximum_watch_checks: usize, maximum_scopes: usize, poll_milliseconds: u64, excluded_roots: Vec<String>, excluded_directories: Vec<String> }

#[derive(Debug, Clone)]
struct SourceRecord { kind: SuggestionSource, suggestion: Suggestion, search: String }

#[derive(Debug, Default)]
struct SourceSnapshot { rows: Vec<SourceRecord>, inspected: usize, scanning: bool, truncated: bool, problem: Option<String> }

#[derive(Debug)]
pub(super) struct SourceDiscovery { users: HashSet<u64>, refresh: Arc<AtomicBool>, state: Arc<Mutex<SourceSnapshot>>, cancel: Arc<AtomicBool> }

impl Drop for SourceDiscovery { fn drop(&mut self) { self.cancel.store(true, Ordering::Relaxed); } }

fn source_policy() -> &'static SourcePolicy {
    static POLICY: std::sync::OnceLock<SourcePolicy> = std::sync::OnceLock::new();
    POLICY.get_or_init(|| { let schema: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🔎️source-discovery/🔣️.json")).expect("source discovery schema"); serde_json::from_value(schema["$defs"]["Policy"]["const"].clone()).expect("source discovery bounds") })
}

fn source_owner<'a>(projects: &'a [(String, String)], path: &str) -> Option<&'a str> {
    projects.iter().filter(|(_, root)| root == "." || root.is_empty() || path == root || path.strip_prefix(root).is_some_and(|tail| tail.starts_with('/'))).max_by_key(|(_, root)| if root == "." { 0 } else { root.len() }).map(|(id, _)| id.as_str())
}

pub(super) fn source_path(value: &str) -> bool { !value.contains(['\\', ':']) && !value.chars().any(|character| character <= '\u{1f}' || character == '\u{7f}') && !Path::new(value).components().any(|part| !matches!(part, std::path::Component::Normal(_) | std::path::Component::CurDir)) }

fn source_attributes(windows: bool, attributes: u32, symlink: bool) -> bool { !symlink && (!windows || attributes & 0x400 == 0) }

fn physical_source(metadata: &std::fs::Metadata) -> bool {
    #[cfg(windows)] { use std::os::windows::fs::MetadataExt; source_attributes(true, metadata.file_attributes(), metadata.is_symlink()) }
    #[cfg(not(windows))] { source_attributes(false, 0, metadata.is_symlink()) }
}

fn bun_test_import(source: &str) -> bool {
    let bytes = source.as_bytes(); let mut index = 0; let mut importing = false; let mut statement = true; let mut head = false; let mut named = false; let mut binding = false; let mut runtime = false; let mut type_pending = false; let mut operand = true;
    while index < bytes.len() {
        if bytes[index..].starts_with(b"//") { while index < bytes.len() && bytes[index] != b'\n' { index += 1; } }
        else if bytes[index..].starts_with(b"/*") { index += 2; while index + 1 < bytes.len() && !bytes[index..].starts_with(b"*/") { index += 1; } index = (index + 2).min(bytes.len()); }
        else if bytes[index] == b'/' && operand {
            index += 1; let mut class = false;
            while index < bytes.len() { if bytes[index] == b'\\' { index = (index + 2).min(bytes.len()); continue; } if bytes[index] == b'[' { class = true; } if bytes[index] == b']' { class = false; } if bytes[index] == b'\n' { break; } if bytes[index] == b'/' && !class { index += 1; break; } index += 1; }
            importing = false; statement = false; operand = false;
        }
        else if matches!(bytes[index], b'\'' | b'"' | b'`') {
            let quote = bytes[index]; let start = index + 1; index += 1;
            while index < bytes.len() && bytes[index] != quote { if bytes[index] == b'\\' { index += 1; } index += 1; }
            if importing && runtime && quote != b'`' && bytes.get(start..index) == Some(b"bun:test") { return true; }
            importing = false; statement = false; operand = false; index = (index + 1).min(bytes.len());
        } else if bytes[index].is_ascii_alphabetic() || bytes[index] == b'_' {
            let start = index; while index < bytes.len() && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_') { index += 1; }
            let word = &bytes[start..index];
            operand = matches!(word, b"return" | b"throw" | b"case" | b"yield" | b"await" | b"void" | b"typeof" | b"delete" | b"new" | b"in" | b"instanceof");
            if word == b"import" && statement { importing = true; head = true; named = false; runtime = false; binding = false; type_pending = false; }
            else if importing && head { if word == b"type" { importing = false; } else { runtime = true; } head = false; }
            else if importing && named && binding { if word == b"type" { type_pending = true; } else { runtime = true; } binding = false; }
            else if importing && type_pending { if word == b"as" { runtime = true; } type_pending = false; }
            statement = false;
        } else { if !bytes[index].is_ascii_whitespace() { operand = !bytes[index].is_ascii_digit() && !matches!(bytes[index], b')' | b']' | b'}' | b'.'); } if bytes[index] == b';' { importing = false; statement = true; } if bytes[index] == b'(' { importing = false; } if bytes[index] == b'{' && importing { head = false; named = true; binding = true; } if bytes[index] == b',' && named { binding = true; type_pending = false; } if bytes[index] == b'}' { named = false; } if bytes[index] == b'*' && importing { runtime = true; head = false; } if bytes[index] == b'\n' && !importing { statement = true; } index += 1; }
    }
    false
}

fn source_kind(path: &str, source: &str) -> Option<SuggestionSource> {
    let name = path.rsplit('/').next()?;
    if name == "📜️script.ts" { Some(SuggestionSource::Scripts) }
    else if name.starts_with("tsconfig") && name.ends_with(".json") { Some(SuggestionSource::Configs) }
    else if [".ts", ".tsx", ".js", ".jsx", ".mts", ".mjs"].iter().any(|suffix| name.ends_with(suffix)) && bun_test_import(source) { Some(SuggestionSource::Tests) }
    else { None }
}

fn source_scan(root: &Path, projects: &[(String, String)], scope: Option<&str>, policy: &SourcePolicy, state: &Mutex<SourceSnapshot>, cancel: &AtomicBool) -> Vec<(PathBuf, Option<SystemTime>)> {
    let excluded = policy.excluded_roots.iter().cloned().collect::<HashSet<_>>(); let boundaries = policy.excluded_directories.iter().cloned().collect::<HashSet<_>>();
    let mut watched = Vec::new(); let mut pending = vec![root.to_path_buf()]; let mut inspected = 0; let mut read = 0;
    if let Some(scope) = scope {
        let Some((_, owner)) = projects.iter().find(|(id, _)| id == scope) else { state.lock().unwrap().problem = Some(format!("Unknown project {scope}")); return watched };
        if !source_path(owner) { state.lock().unwrap().problem = Some(format!("Invalid source project root {owner}")); return watched; }
        let mut boundary = root.to_path_buf(); let mut ancestors = vec![boundary.clone()];
        for component in Path::new(owner).components() {
            boundary.push(component.as_os_str()); ancestors.push(boundary.clone());
            if boundaries.contains(&component.as_os_str().to_string_lossy().to_string()) || boundary.strip_prefix(root).ok().is_some_and(|path| excluded.contains(&path.to_string_lossy().replace('\\', "/"))) { return watched; }
        }
        for boundary in ancestors {
            match std::fs::symlink_metadata(&boundary) { Ok(facts) if facts.is_dir() && physical_source(&facts) => {}, _ => { state.lock().unwrap().problem = Some(format!("Source boundary is not a physical directory: {}", boundary.display())); return watched; } }
        }
        pending = vec![root.join(owner)];
    }
    while let Some(directory) = pending.pop() {
        if cancel.load(Ordering::Relaxed) { break; }
        let facts = match std::fs::symlink_metadata(&directory) { Ok(facts) if facts.is_dir() && physical_source(&facts) => facts, Ok(_) => continue, Err(error) => { state.lock().unwrap().problem = Some(error.to_string()); continue; } };
        watched.push((directory.clone(), facts.modified().ok()));
        let entries = match std::fs::read_dir(&directory) { Ok(entries) => entries, Err(error) => { state.lock().unwrap().problem = Some(error.to_string()); continue; } };
        for entry in entries {
            if cancel.load(Ordering::Relaxed) { break; }
            inspected += 1;
            if inspected % 128 == 0 { state.lock().unwrap().inspected = inspected; std::thread::yield_now(); }
            if inspected > policy.maximum_entries { state.lock().unwrap().truncated = true; return watched; }
            let entry = match entry { Ok(entry) => entry, Err(error) => { state.lock().unwrap().problem = Some(error.to_string()); continue; } };
            let path = entry.path(); let Some(relative) = path.strip_prefix(root).ok().and_then(Path::to_str).map(|text| text.replace('\\', "/")) else { continue };
            if !source_path(&relative) { continue; }
            if path.as_os_str().to_string_lossy().encode_utf16().count() > 256 { state.lock().unwrap().truncated = true; continue; }
            let facts = match std::fs::symlink_metadata(&path) { Ok(facts) if physical_source(&facts) => facts, _ => continue };
            let name = entry.file_name().to_string_lossy().to_string(); if !source_path(&name) { continue; }
            if facts.is_dir() {
                if boundaries.contains(&name) || excluded.contains(&relative) { continue; }
                if scope.is_some() && projects.iter().any(|(id, owner)| Some(id.as_str()) != scope && owner == &relative) { continue; }
                pending.push(path); continue;
            }
            if !facts.is_file() { continue; }
            let Some(project) = source_owner(projects, &relative) else { continue };
            if scope.is_some_and(|scope| project != scope) { continue; }
            let candidate = name == "📜️script.ts" || name.starts_with("tsconfig") && name.ends_with(".json") || [".ts", ".tsx", ".js", ".jsx", ".mts", ".mjs"].iter().any(|suffix| name.ends_with(suffix));
            if !candidate { continue; }
            let mut source = String::new();
            if name != "📜️script.ts" && !(name.starts_with("tsconfig") && name.ends_with(".json")) {
                let file = match std::fs::File::open(&path) { Ok(file) => file, Err(error) => { state.lock().unwrap().problem = Some(error.to_string()); continue; } };
                if read >= policy.maximum_read_bytes { state.lock().unwrap().truncated = true; continue; }
                let maximum = policy.maximum_source_bytes.min(policy.maximum_read_bytes - read);
                let mut bytes = Vec::new(); if let Err(error) = file.take(maximum as u64).read_to_end(&mut bytes) { state.lock().unwrap().problem = Some(error.to_string()); continue; }
                if std::fs::metadata(&path).is_ok_and(|facts| facts.len() > bytes.len() as u64) { state.lock().unwrap().truncated = true; }
                read += bytes.len(); source = String::from_utf8_lossy(&bytes).into_owned();
            }
            watched.push((path.clone(), std::fs::metadata(&path).and_then(|facts| facts.modified()).ok()));
            if let Some(kind) = source_kind(&relative, &source) {
                let mut snapshot = state.lock().unwrap();
                if snapshot.rows.len() >= policy.maximum_rows { snapshot.truncated = true; return watched; }
                let suggestion = Suggestion { value: relative, project: project.into(), directory: projects.iter().find(|(id, _)| id == project).map_or(".", |(_, root)| root.as_str()).to_string() };
                let search = format!("{} {}", suggestion.value, suggestion.project).to_lowercase();
                snapshot.rows.push(SourceRecord { kind, suggestion, search });
            }
            if inspected % 128 == 0 { state.lock().unwrap().inspected = inspected; std::thread::yield_now(); }
        }
    }
    state.lock().unwrap().inspected = inspected;
    watched
}

fn source_changes(watched: &[(PathBuf, Option<SystemTime>)], cursor: &mut usize, maximum: usize, cancel: &AtomicBool) -> bool {
    if watched.is_empty() { return false; }
    for _ in 0..maximum.min(watched.len()) {
        if cancel.load(Ordering::Relaxed) { return false; }
        let (path, previous) = &watched[*cursor % watched.len()]; *cursor = (*cursor + 1) % watched.len();
        if std::fs::symlink_metadata(path).and_then(|facts| if !physical_source(&facts) { Err(std::io::Error::other("source changed to link")) } else { facts.modified() }).ok() != *previous { return true; }
    }
    false
}

impl Registry {
    fn suggestion_key(parameter: &Parameter, chosen: &[(String, String)]) -> String { parameter.suggestions.as_ref().and_then(|spec| spec.project_parameter.as_ref()).and_then(|id| chosen.iter().find(|(key, _)| key == id).map(|(_, value)| value.as_str())).filter(|value| !value.is_empty()).unwrap_or("*").to_string() }

    /// 🚪️ Releases one form's subscriptions; another form can keep the same scope running.
    pub fn release_suggestions(&self, session: u64) {
        self.source_cancelled.lock().unwrap().remove(&session);
        self.source_discovery.lock().unwrap().retain(|_, discovery| { discovery.users.remove(&session); !discovery.users.is_empty() });
    }

    /// 🛑️ Cancels this form's search without cancelling another subscriber's work.
    pub fn cancel_suggestions(&self, session: u64) {
        self.release_suggestions(session);
        self.source_cancelled.lock().unwrap().insert(session);
    }

    /// ♻️ Starts a fresh physical scan when this form explicitly refreshes.
    pub fn refresh_suggestions(&self, parameter: &Parameter, chosen: &[(String, String)], session: u64) {
        self.source_cancelled.lock().unwrap().remove(&session);
        if let Some(discovery) = self.source_discovery.lock().unwrap().get(&Self::suggestion_key(parameter, chosen)) { discovery.refresh.store(true, Ordering::Relaxed); discovery.state.lock().unwrap().scanning = true; }
    }

    /// 📡️ A compact change token lets the existing UI event loop repaint asynchronous progress.
    pub fn suggestions_revision(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        let discovery = self.source_discovery.lock().unwrap(); let mut keys = discovery.keys().collect::<Vec<_>>(); keys.sort();
        for key in keys { let known = &discovery[key]; let snapshot = known.state.lock().unwrap(); (key, snapshot.inspected, snapshot.rows.len(), snapshot.scanning, snapshot.truncated, &snapshot.problem, known.cancel.load(Ordering::Relaxed)).hash(&mut hash); }
        hash.finish()
    }

    /// 💡️ Offers bounded editable values without changing command arguments or execution authority.
    pub fn suggestions(&self, parameter: &Parameter, chosen: &[(String, String)], query: &str, session: u64) -> Suggestions {
        let Some(spec) = &parameter.suggestions else { return Suggestions::default() };
        let maximum = source_policy().maximum_results;
        let search = query.chars().take(source_policy().maximum_query_characters).collect::<String>();
        let query_limited = query.chars().nth(source_policy().maximum_query_characters).is_some() || search.split_whitespace().count() > source_policy().maximum_query_words;
        let needles = search.split_whitespace().take(source_policy().maximum_query_words).map(str::to_lowercase).collect::<Vec<_>>();
        let matches = |row: &Suggestion| { let text = format!("{} {}", row.value, row.project).to_lowercase(); needles.iter().all(|needle| text.contains(needle)) };
        if spec.source == SuggestionSource::Projects { let values = self.source_projects.iter().map(|(id, root)| Suggestion { value: id.clone(), project: id.clone(), directory: root.clone() }).filter(matches).collect::<Vec<_>>(); let truncated = query_limited || values.len() > maximum; return Suggestions { values: values.into_iter().take(maximum).collect(), truncated, ..Suggestions::default() }; }
        if self.source_cancelled.lock().unwrap().contains(&session) { return Suggestions { cancelled: true, ..Suggestions::default() }; }
        let scope = spec.project_parameter.as_ref().and_then(|id| chosen.iter().find(|(key, _)| key == id).map(|(_, value)| value.as_str())).filter(|value| !value.is_empty());
        if scope.is_some_and(|scope| !self.source_projects.iter().any(|(id, _)| id == scope)) { return Suggestions { problem: Some(format!("Unknown project {}", scope.unwrap())), ..Suggestions::default() }; }
        let mut discovery = self.source_discovery.lock().unwrap();
        let key = Self::suggestion_key(parameter, chosen);
        discovery.retain(|known, discovery| { if known != &key { discovery.users.remove(&session); } !discovery.users.is_empty() });
        if !discovery.contains_key(&key) {
            if discovery.len() >= source_policy().maximum_scopes { return Suggestions { problem: Some("Source discovery scope limit reached; cancel another search".into()), truncated: true, ..Suggestions::default() }; }
            let state = Arc::new(Mutex::new(SourceSnapshot { scanning: true, ..SourceSnapshot::default() })); let cancel = Arc::new(AtomicBool::new(false)); let refresh = Arc::new(AtomicBool::new(false));
            let (root, projects, selected, shared, stop, requested) = (self.root.clone(), self.source_projects.clone(), scope.map(str::to_string), state.clone(), cancel.clone(), refresh.clone());
            std::thread::spawn(move || {
                let policy = source_policy();
                loop {
                    let watched = source_scan(&root, &projects, selected.as_deref(), &policy, &shared, &stop);
                    { let mut snapshot = shared.lock().unwrap(); snapshot.rows.sort_by(|left, right| left.suggestion.value.cmp(&right.suggestion.value)); snapshot.scanning = false; }
                    if stop.load(Ordering::Relaxed) { break; }
                    if requested.swap(false, Ordering::Relaxed) { *shared.lock().unwrap() = SourceSnapshot { scanning: true, ..SourceSnapshot::default() }; continue; }
                    let mut cursor = 0;
                    loop {
                        std::thread::sleep(Duration::from_millis(policy.poll_milliseconds));
                        if stop.load(Ordering::Relaxed) { return; }
                        let changed = requested.swap(false, Ordering::Relaxed) || source_changes(&watched, &mut cursor, policy.maximum_watch_checks, &stop);
                        if changed { *shared.lock().unwrap() = SourceSnapshot { scanning: true, ..SourceSnapshot::default() }; break; }
                    }
                }
            });
            discovery.insert(key.clone(), SourceDiscovery { users: HashSet::from([session]), refresh, state, cancel });
        }
        let known = discovery.get_mut(&key).unwrap(); known.users.insert(session); let snapshot = known.state.lock().unwrap();
        let mut values = snapshot.rows.iter().filter(|row| row.kind == spec.source && needles.iter().all(|needle| row.search.contains(needle))).take(maximum + 1).map(|row| row.suggestion.clone()).collect::<Vec<_>>(); values.sort_by(|left, right| left.value.cmp(&right.value)); values.dedup();
        let truncated = query_limited || snapshot.truncated || values.len() > maximum; values.truncate(maximum);
        Suggestions { values, inspected: snapshot.inspected, scanning: snapshot.scanning, truncated, cancelled: known.cancel.load(Ordering::Relaxed), problem: snapshot.problem.clone() }
    }
}
