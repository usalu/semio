//! 📚️ Takes stock of the workspace for the command registry: reads every source once, remembers
//! what it read in a versioned snapshot keyed by file fingerprints, and publishes registries from
//! a cancellable background job, so commands are usable before the full workspace walk completes.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs

use crate::command_tree::CommandLeaf;
use crate::registry::{segment_key, Facts, Fingerprint, GraphBasis, PlaygroundFacts, Problem, ProjectFacts, Registry, TargetFacts, TicketFacts, GRAPH_OWNERS, PLAYGROUND_SOURCE, TICKET_COMMANDS};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering}, mpsc::{self, Receiver}, Arc, Mutex};

// #region 🔖️Snapshot
const SNAPSHOT_VERSION: u32 = 3;
const SNAPSHOT_LIMIT: u64 = 64 * 1024 * 1024;
const STALE_TEMPORARY_SECONDS: u64 = 600;
const GRAPH_FILE: &str = ".nx/workspace-data/project-graph.json";
const SCRIPTS_FILE: &str = "package.json";
const PROJECT_MANIFESTS: &[&str] = &["📋️project.json", "project.json"];
const WALK_SKIP_DIRS: &[&str] = &["node_modules", "target", "dist", "build", "generated", "cache"];

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sourced<T> { fingerprint: Option<Fingerprint>, value: T }

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest { fingerprint: Fingerprint, project: Option<ProjectFacts>, problem: Option<String> }

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TicketSource { facts: TicketFacts, document: Option<Fingerprint>, commands: Option<Fingerprint>, problem: Option<String> }

/// 🧊️ Everything the sources stated at the last discovery, with the fingerprint each was read at.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    version: u32,
    root: PathBuf,
    manifests: BTreeMap<String, Manifest>,
    graph: Sourced<Vec<ProjectFacts>>,
    scripts: Sourced<Vec<String>>,
    playgrounds: Sourced<Vec<PlaygroundFacts>>,
    tickets: Vec<TicketSource>,
    folders: BTreeMap<String, Option<Fingerprint>>,
    basis: Option<GraphBasis>,
    walked: u64,
    complete: bool,
}

/// 📍️ Where the snapshot of a workspace lives.
pub fn cache_path(root: &Path) -> PathBuf { root.join(".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/commands.json") }

fn read_snapshot(path: &Path, root: &Path) -> Option<Snapshot> {
    let file = std::fs::File::open(path).ok()?;
    if file.metadata().ok()?.len() > SNAPSHOT_LIMIT { return None; }
    let snapshot: Snapshot = serde_json::from_reader(std::io::BufReader::with_capacity(1 << 20, file)).ok()?;
    (snapshot.version == SNAPSHOT_VERSION && snapshot.root == root).then_some(snapshot)
}

fn write_snapshot(path: &Path, snapshot: &Snapshot) -> std::io::Result<()> {
    let directory = path.parent().ok_or_else(|| std::io::Error::other("snapshot path has no directory"))?;
    std::fs::create_dir_all(directory)?;
    sweep_temporaries(directory);
    let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
    let temporary = path.with_extension(format!("{}-{nonce}.tmp", std::process::id()));
    let result = (|| {
        let mut writer = std::io::BufWriter::with_capacity(1 << 20, std::fs::File::create(&temporary)?);
        serde_json::to_writer(&mut writer, snapshot).map_err(std::io::Error::other)?;
        std::io::Write::flush(&mut writer)?;
        std::fs::rename(&temporary, path)
    })();
    if result.is_err() { let _ = std::fs::remove_file(&temporary); }
    result
}

/// 🧹️ Removes snapshot temporaries a killed writer left behind: any `*.tmp` beside the snapshot whose
/// modification instant is more than ten minutes away from now, in either direction of the clock.
fn sweep_temporaries(directory: &Path) {
    let now = std::time::SystemTime::now();
    for entry in std::fs::read_dir(directory).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|extension| extension != "tmp") || !entry.file_name().to_string_lossy().starts_with("commands.") { continue; }
        let Ok(modified) = entry.metadata().and_then(|metadata| metadata.modified()) else { continue };
        let distance = now.duration_since(modified).unwrap_or_else(|error| error.duration());
        if distance.as_secs() > STALE_TEMPORARY_SECONDS { let _ = std::fs::remove_file(path); }
    }
}

impl Snapshot {
    /// 🧮️ Joins the manifests and the Nx graph into the facts the registry is built from. A manifest
    /// states its own project and targets; the graph adds inferred ones and what target defaults decide.
    fn facts(&self) -> Facts {
        let mut projects: BTreeMap<String, ProjectFacts> = self.graph.value.iter().map(|project| (project.name.clone(), project.clone())).collect();
        let mut problems = Vec::new();
        for (path, manifest) in &self.manifests {
            if let Some(message) = &manifest.problem { problems.push(Problem { file: path.clone(), at: String::new(), message: message.clone() }); }
            let Some(stated) = &manifest.project else { continue };
            let name = if !stated.name.is_empty() { stated.name.clone() } else { projects.values().find(|project| project.root == stated.root).map_or_else(|| if stated.root == "." { "workspace".to_string() } else { stated.root.rsplit('/').next().unwrap_or_default().to_string() }, |project| project.name.clone()) };
            projects.retain(|known, project| known == &name || project.root != stated.root);
            let project = projects.entry(name.clone()).or_insert_with(|| ProjectFacts { name, root: stated.root.clone(), ..Default::default() });
            (project.root, project.manifest, project.dashboard) = (stated.root.clone(), path.clone(), stated.dashboard.clone());
            for target in &stated.targets {
                match project.targets.iter_mut().find(|known| known.name == target.name) {
                    Some(known) => {
                        known.continuous = target.continuous.or(known.continuous);
                        known.configurations.extend(target.configurations.iter().filter(|name| !known.configurations.contains(name)).cloned().collect::<Vec<_>>());
                        known.dashboard.clone_from(&target.dashboard);
                    }
                    None => project.targets.push(target.clone()),
                }
            }
            project.targets.sort_by(|left, right| left.name.cmp(&right.name));
        }
        problems.extend(self.tickets.iter().filter_map(|ticket| ticket.problem.as_ref().map(|message| Problem { file: format!("{}/{TICKET_COMMANDS}", ticket.facts.folder), at: String::new(), message: message.clone() })));
        Facts { projects: projects.into_values().collect(), scripts: self.scripts.value.clone(), playgrounds: self.playgrounds.value.clone(), tickets: self.tickets.iter().map(|ticket| ticket.facts.clone()).collect(), problems }
    }

    fn same_sources(&self, other: &Self) -> bool {
        self.manifests == other.manifests && self.graph == other.graph && self.scripts == other.scripts && self.playgrounds == other.playgrounds && self.tickets == other.tickets && self.basis == other.basis
    }

    fn registry(&self) -> Registry {
        let registry = Registry::build(&self.root, &self.facts());
        registry.adopt_basis(self.basis.clone());
        registry
    }

    /// 🔏️ Whether every source still has the fingerprint it was read at: nothing known changed,
    /// appeared in a known folder or vanished. A manifest in a folder no walk has seen is not covered.
    fn is_current(&self) -> bool {
        let same = |path: &str, known: Option<Fingerprint>| Fingerprint::of(&self.root.join(path)) == known;
        self.complete
            && same(SCRIPTS_FILE, self.scripts.fingerprint) && same(PLAYGROUND_SOURCE, self.playgrounds.fingerprint) && same(GRAPH_FILE, self.graph.fingerprint)
            && self.manifests.iter().all(|(path, manifest)| same(path, Some(manifest.fingerprint)))
            && self.folders.iter().all(|(path, known)| same(path, *known))
            && self.tickets.iter().all(|ticket| same(&format!("{}/{}", ticket.facts.folder, semio_framework_repo_tickets::TICKET_DOCUMENT_NAME), ticket.document) && (!ticket.facts.open || same(&format!("{}/{TICKET_COMMANDS}", ticket.facts.folder), ticket.commands)))
    }
}
// #endregion 🔖️Snapshot

// #region 🔖️Sources
fn relative(root: &Path, path: &Path) -> String {
    let parts: Vec<&str> = path.strip_prefix(root).unwrap_or(path).components().filter_map(|part| part.as_os_str().to_str()).collect();
    if parts.is_empty() { ".".to_string() } else { parts.join("/") }
}

fn should_skip_walk_dir(name: &str) -> bool { name.starts_with('.') || WALK_SKIP_DIRS.contains(&segment_key(name).as_str()) }

fn dashboard_of(value: &serde_json::Value) -> Option<serde_json::Value> { value.pointer("/metadata/semio/dashboard").cloned() }

fn read_manifest(root: &Path, path: &str) -> Option<Manifest> {
    let file = root.join(path);
    let fingerprint = Fingerprint::of(&file)?;
    let folder = path.rsplit_once('/').map_or(".", |(folder, _)| folder).to_string();
    let json = match std::fs::read_to_string(&file).map_err(|error| error.to_string()).and_then(|text| serde_json::from_str::<serde_json::Value>(&text).map_err(|error| error.to_string())) {
        Ok(json) => json,
        Err(problem) => return Some(Manifest { fingerprint, project: None, problem: Some(problem) }),
    };
    let targets = json.get("targets").and_then(serde_json::Value::as_object);
    let dashboard = dashboard_of(&json);
    if targets.is_none() && dashboard.is_none() { return Some(Manifest { fingerprint, project: None, problem: None }); }
    let mut targets: Vec<TargetFacts> = targets.into_iter().flatten().map(|(name, target)| TargetFacts { name: name.clone(), continuous: target.get("continuous").and_then(serde_json::Value::as_bool), configurations: target.get("configurations").and_then(serde_json::Value::as_object).map(|configurations| configurations.keys().cloned().collect()).unwrap_or_default(), dashboard: dashboard_of(target) }).collect();
    targets.sort_by(|left, right| left.name.cmp(&right.name));
    Some(Manifest { fingerprint, problem: None, project: Some(ProjectFacts { name: json.get("name").and_then(serde_json::Value::as_str).unwrap_or_default().to_string(), root: folder, manifest: path.to_string(), targets, dashboard }) })
}

struct CancellationReader<'a> { file: std::fs::File, cancelled: &'a AtomicBool }

impl std::io::Read for CancellationReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.cancelled.load(Ordering::Relaxed) { return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "command discovery cancelled")); }
        std::io::Read::read(&mut self.file, buffer)
    }
}

#[derive(Deserialize)]
struct Graph { nodes: BTreeMap<String, GraphNode> }
#[derive(Deserialize)]
struct GraphNode { data: GraphData }
#[derive(Deserialize)]
struct GraphData { root: String, #[serde(default)] targets: BTreeMap<String, GraphTarget>, #[serde(default)] metadata: Option<GraphMetadata> }
#[derive(Deserialize)]
struct GraphTarget { #[serde(default)] continuous: Option<bool>, #[serde(default)] configurations: Option<BTreeMap<String, serde::de::IgnoredAny>>, #[serde(default)] metadata: Option<GraphMetadata> }
#[derive(Deserialize)]
struct GraphMetadata { #[serde(default)] semio: Option<serde_json::Value> }

impl GraphMetadata { fn dashboard(self) -> Option<serde_json::Value> { self.semio?.get_mut("dashboard").map(serde_json::Value::take) } }

/// 🕸️ Reads the project graph Nx last published, waiting out a publication in progress.
fn read_graph(root: &Path, cancelled: &AtomicBool) -> Sourced<Vec<ProjectFacts>> {
    let path = root.join(GRAPH_FILE);
    let started = std::time::Instant::now();
    let (fingerprint, graph) = loop {
        if cancelled.load(Ordering::Relaxed) { return Sourced::default(); }
        let give_up = started.elapsed().as_secs() >= 2;
        let before = Fingerprint::of(&path);
        let file = match std::fs::File::open(&path) { Ok(file) => file, Err(error) if error.kind() == std::io::ErrorKind::NotFound || give_up => return Sourced::default(), Err(_) => { std::thread::sleep(std::time::Duration::from_millis(20)); continue; } };
        let result = serde_json::from_reader::<_, Graph>(std::io::BufReader::with_capacity(1 << 20, CancellationReader { file, cancelled }));
        match result { Ok(graph) if before.is_some() && Fingerprint::of(&path) == before => break (before, graph), _ if give_up => return Sourced::default(), _ => std::thread::sleep(std::time::Duration::from_millis(20)) }
    };
    let mut projects = Vec::with_capacity(graph.nodes.len());
    for (name, node) in graph.nodes {
        let folder = Path::new(&node.data.root);
        if folder.is_absolute() || folder.components().any(|part| match part { std::path::Component::Normal(value) => value.to_str().is_none_or(should_skip_walk_dir), std::path::Component::CurDir => false, _ => true }) || !root.join(folder).is_dir() { continue; }
        let targets = node.data.targets.into_iter().map(|(name, target)| TargetFacts { name, continuous: target.continuous, configurations: target.configurations.map(|configurations| configurations.into_keys().collect()).unwrap_or_default(), dashboard: target.metadata.and_then(GraphMetadata::dashboard) }).collect();
        projects.push(ProjectFacts { name, root: relative(root, &root.join(folder)), manifest: String::new(), targets, dashboard: node.data.metadata.and_then(GraphMetadata::dashboard) });
    }
    Sourced { fingerprint, value: projects }
}

/// 📜️ The root workspace scripts that call Nx; the dashboard's own entry scripts are not commands.
fn read_scripts(root: &Path) -> Sourced<Vec<String>> {
    let path = root.join(SCRIPTS_FILE);
    let fingerprint = Fingerprint::of(&path);
    let scripts = std::fs::read_to_string(path).ok().and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok()).and_then(|manifest| manifest.get("scripts").and_then(serde_json::Value::as_object).cloned()).unwrap_or_default();
    let mut names: Vec<String> = scripts.iter().filter(|(name, command)| name.as_str() != "nx" && name.as_str() != "dashboard" && !name.starts_with("dashboard:") && command.as_str().is_some_and(|command| command.starts_with("bun nx ") || command.starts_with("nx "))).map(|(name, _)| name.clone()).collect();
    names.sort();
    Sourced { fingerprint, value: names }
}

fn read_playgrounds(root: &Path) -> Sourced<Vec<PlaygroundFacts>> {
    let fingerprint = Fingerprint::of(&root.join(PLAYGROUND_SOURCE));
    let rows: Vec<serde_json::Value> = serde_json::from_str(&crate::catalog::playgrounds_json_text(root)).unwrap_or_default();
    let ports = |row: &serde_json::Value, pointer: &str| row.pointer(pointer).and_then(serde_json::Value::as_array).map(|ports| ports.iter().filter_map(|port| u16::try_from(port.as_u64()?).ok()).collect::<Vec<_>>()).unwrap_or_default();
    let port = |row: &serde_json::Value, pointer: &str| row.pointer(pointer).and_then(serde_json::Value::as_u64).and_then(|port| u16::try_from(port).ok());
    let mut value: Vec<PlaygroundFacts> = rows.iter().filter_map(|row| Some(PlaygroundFacts { variant: row.get("variant")?.as_str()?.to_string(), plugin: row.get("pluginId")?.as_str()?.to_string(), app: row.get("app").and_then(serde_json::Value::as_str).map(str::to_string), react: port(row, "/ports/react")?, wgpu: port(row, "/ports/wgpu")?, user_react: ports(row, "/userPorts/react"), user_wgpu: ports(row, "/userPorts/wgpu"), examples: row.get("examples").and_then(serde_json::Value::as_array).map(|examples| examples.iter().filter_map(|example| example.as_str().map(str::to_string)).collect()).unwrap_or_default() })).collect();
    value.sort_by(|left, right| left.variant.cmp(&right.variant));
    Sourced { fingerprint, value }
}

/// 🎫️ The ticket index of the repo domain; an open ticket also contributes its command document.
fn read_tickets(root: &Path, cancelled: &AtomicBool) -> (Vec<TicketSource>, BTreeMap<String, Option<Fingerprint>>) {
    let mut folders = BTreeMap::new();
    let mut tickets = Vec::new();
    for ticket in crate::command_tree::repo_domain::ticket_index(root) {
        if cancelled.load(Ordering::Relaxed) { break; }
        let folder = relative(root, Path::new(&ticket.folder_path));
        let open = ticket.status == "open";
        let commands_path = root.join(&folder).join(TICKET_COMMANDS);
        let commands = Fingerprint::of(&commands_path).filter(|_| open);
        let (document, problem) = match commands.map(|_| std::fs::read_to_string(&commands_path).map_err(|error| error.to_string()).and_then(|text| serde_json::from_str::<serde_json::Value>(&text).map_err(|error| error.to_string()))) { Some(Ok(document)) => (Some(document), None), Some(Err(problem)) => (None, Some(problem)), None => (None, None) };
        let mut ancestor = folder.as_str();
        for _ in 0..4 { let Some((parent, _)) = ancestor.rsplit_once('/') else { break }; ancestor = parent; folders.entry(parent.to_string()).or_insert_with(|| Fingerprint::of(&root.join(parent))); }
        tickets.push(TicketSource { document: Fingerprint::of(&root.join(&folder).join(semio_framework_repo_tickets::TICKET_DOCUMENT_NAME)), commands, problem, facts: TicketFacts { id: ticket.id, open, title: ticket.title, folder, commands: document } });
    }
    (tickets, folders)
}

/// 🚶️ Walks the workspace for project manifests on a few threads. Dot-directories — the repository
/// metadata among them — and build output are never entered; cancellation is honoured per directory.
fn walk(root: &Path, cancelled: &AtomicBool, progress: &Progress) -> Vec<String> {
    let queue: Mutex<Vec<PathBuf>> = Mutex::new(vec![root.to_path_buf()]);
    let (found, busy) = (Mutex::new(Vec::new()), AtomicU64::new(0));
    let workers = std::thread::available_parallelism().map_or(2, |count| count.get().clamp(1, 6));
    std::thread::scope(|scope| for _ in 0..workers {
        scope.spawn(|| loop {
            if cancelled.load(Ordering::Relaxed) { return; }
            let next = { let mut queue = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner); let next = queue.pop(); if next.is_some() { busy.fetch_add(1, Ordering::SeqCst); } next };
            let Some(directory) = next else { if busy.load(Ordering::SeqCst) == 0 { return; } std::thread::yield_now(); continue; };
            let (mut folders, mut manifests) = (Vec::new(), Vec::new());
            for entry in std::fs::read_dir(&directory).into_iter().flatten().flatten() {
                let Ok(kind) = entry.file_type() else { continue };
                let name = entry.file_name();
                let Some(name) = name.to_str() else { continue };
                if kind.is_dir() { if !should_skip_walk_dir(name) { folders.push(entry.path()); } } else if kind.is_file() && PROJECT_MANIFESTS.contains(&name) { manifests.push(relative(root, &entry.path())); }
            }
            progress.visited.fetch_add(1, Ordering::Relaxed);
            if !manifests.is_empty() { found.lock().unwrap_or_else(std::sync::PoisonError::into_inner).extend(manifests); }
            queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).extend(folders);
            busy.fetch_sub(1, Ordering::SeqCst);
        });
    });
    let mut found = found.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner);
    found.sort();
    found
}
// #endregion 🔖️Sources

// #region 🔖️Discovery
/// 🪜️ How far a discovery has come.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Phase { Known = 0, Sources = 1, Walk = 2, Complete = 3 }

impl Phase {
    /// 🔑️ The language-neutral name of the phase.
    pub fn key(self) -> &'static str { match self { Self::Known => "known", Self::Sources => "sources", Self::Walk => "walk", Self::Complete => "complete" } }
}

/// 📊️ The observable progress of a discovery: its phase, the folders walked so far and the folders
/// the last complete walk visited (0 when none is known).
#[derive(Debug, Default)]
pub struct Progress { phase: AtomicU8, visited: AtomicU64, expected: AtomicU64 }

impl Progress {
    pub fn phase(&self) -> Phase { match self.phase.load(Ordering::Relaxed) { 0 => Phase::Known, 1 => Phase::Sources, 2 => Phase::Walk, _ => Phase::Complete } }
    pub fn visited(&self) -> u64 { self.visited.load(Ordering::Relaxed) }
    pub fn expected(&self) -> u64 { self.expected.load(Ordering::Relaxed) }
}

fn trace(started: std::time::Instant, stage: &str) {
    if std::env::var_os("SEMIO_DASHBOARD_TRACE").is_some() { eprintln!("[semio inventory] {stage} {}us", started.elapsed().as_micros()); }
}

/// 🔭️ Reads every source. A source whose fingerprint equals the previous snapshot's is not read
/// again. `stage` receives the snapshot after the known sources and again after the walk; the
/// result is `None` when the discovery was cancelled.
fn gather(root: &Path, previous: Option<&Snapshot>, cancelled: &AtomicBool, progress: &Progress, stage: &mut dyn FnMut(&Snapshot, Phase)) -> Option<Snapshot> {
    let started = std::time::Instant::now();
    let stopped = || cancelled.load(Ordering::Relaxed);
    let mut snapshot = Snapshot { version: SNAPSHOT_VERSION, root: root.to_path_buf(), ..Default::default() };
    progress.phase.store(Phase::Sources as u8, Ordering::Relaxed);
    progress.expected.store(previous.map_or(0, |previous| previous.walked), Ordering::Relaxed);
    snapshot.scripts = read_scripts(root);
    snapshot.playgrounds = read_playgrounds(root);
    snapshot.graph = match previous { Some(previous) if previous.graph.fingerprint.is_some() && previous.graph.fingerprint == Fingerprint::of(&root.join(GRAPH_FILE)) => previous.graph.clone(), _ => read_graph(root, cancelled) };
    trace(started, "graph");
    if stopped() { return None; }
    let adopt = |snapshot: &mut Snapshot, path: String| {
        if snapshot.manifests.contains_key(&path) { return; }
        let known = previous.and_then(|previous| previous.manifests.get(&path)).filter(|known| Fingerprint::of(&root.join(&path)) == Some(known.fingerprint));
        if let Some(manifest) = known.cloned().or_else(|| read_manifest(root, &path)) { snapshot.manifests.insert(path, manifest); }
    };
    let candidates: BTreeSet<String> = previous.into_iter().flat_map(|previous| previous.manifests.keys().cloned())
        .chain(snapshot.graph.value.iter().flat_map(|project| PROJECT_MANIFESTS.iter().map(move |name| if project.root == "." { (*name).to_string() } else { format!("{}/{name}", project.root) })))
        .chain(PROJECT_MANIFESTS.iter().map(|name| (*name).to_string())).collect();
    for path in candidates { adopt(&mut snapshot, path); }
    trace(started, "manifests");
    if stopped() { return None; }
    (snapshot.tickets, snapshot.folders) = read_tickets(root, cancelled);
    trace(started, "tickets");
    if stopped() { return None; }
    let definitions = |snapshot: &Snapshot| snapshot.manifests.iter().filter(|(_, manifest)| manifest.project.is_some()).map(|(path, _)| path.clone()).chain(GRAPH_OWNERS.iter().map(|owner| (*owner).to_string())).collect::<Vec<_>>();
    let basis = |snapshot: &Snapshot| GraphBasis::read(root, &definitions(snapshot));
    snapshot.basis = basis(&snapshot);
    trace(started, "basis");
    stage(&snapshot, Phase::Sources);
    progress.phase.store(Phase::Walk as u8, Ordering::Relaxed);
    let before = snapshot.manifests.len();
    for path in walk(root, cancelled, progress) { adopt(&mut snapshot, path); }
    trace(started, "walk");
    if stopped() { return None; }
    if snapshot.manifests.len() != before { snapshot.basis = basis(&snapshot); }
    (snapshot.walked, snapshot.complete) = (progress.visited(), true);
    progress.phase.store(Phase::Complete as u8, Ordering::Relaxed);
    stage(&snapshot, Phase::Complete);
    trace(started, "complete");
    Some(snapshot)
}

/// 🧭️ Discovers the registry of a workspace from its sources alone: nothing is read from or
/// written to the snapshot.
pub fn discover(root: &Path, cancelled: &AtomicBool) -> Registry {
    gather(root, None, cancelled, &Progress::default(), &mut |_, _| {}).map_or_else(|| seed(root), |snapshot| snapshot.registry())
}

/// 🌱️ The commands known without the graph, the walk or the ticket index: workspace scripts,
/// playgrounds and what the root manifest declares.
pub fn seed(root: &Path) -> Registry {
    let mut snapshot = Snapshot { version: SNAPSHOT_VERSION, root: root.to_path_buf(), scripts: read_scripts(root), playgrounds: read_playgrounds(root), ..Default::default() };
    for name in PROJECT_MANIFESTS { if let Some(manifest) = read_manifest(root, name) { snapshot.manifests.insert((*name).to_string(), manifest); } }
    snapshot.registry()
}

/// 📖️ The registry a one-shot invocation works with: the snapshot while every source it was read
/// from is unchanged, else a fresh discovery that replaces the snapshot.
pub fn registry(root: &Path) -> Registry { registry_at(root, &cache_path(root)) }

/// 🗄️ [`registry`] with the snapshot at an explicit place.
pub fn registry_at(root: &Path, snapshot_path: &Path) -> Registry {
    let previous = read_snapshot(snapshot_path, root);
    if let Some(snapshot) = previous.as_ref().filter(|snapshot| snapshot.is_current()) { return snapshot.registry(); }
    match gather(root, previous.as_ref(), &AtomicBool::new(false), &Progress::default(), &mut |_, _| {}) {
        Some(snapshot) => { if let Err(error) = write_snapshot(snapshot_path, &snapshot) { eprintln!("[semio] command snapshot: {error}"); } snapshot.registry() }
        None => seed(root),
    }
}
// #endregion 🔖️Discovery

// #region 🔖️Job
/// 📡️ A discovery result consumed by the view without waiting for the worker.
pub enum Update { Ready(Vec<(String, CommandLeaf)>, String), Finished, Failed(String) }

/// ⏳️ One background discovery generation with explicit cancellation and observable progress.
pub struct Job {
    pub receiver: Receiver<Update>,
    pub cancelled: Arc<AtomicBool>,
    pub started: std::time::Instant,
    pub progress: Arc<Progress>,
    registry: Arc<Mutex<Option<Arc<Registry>>>>,
}

impl Job {
    /// 🎮️ The registry the job published last.
    pub fn registry(&self) -> Option<Arc<Registry>> { self.registry.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() }
}

impl Drop for Job { fn drop(&mut self) { self.cancelled.store(true, Ordering::Relaxed); } }

static PUBLISHED: Mutex<Option<Arc<Registry>>> = Mutex::new(None);

/// 📣️ The registry this process published last for a workspace.
pub fn published(root: &Path) -> Option<Arc<Registry>> {
    PUBLISHED.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone().filter(|registry| registry.root() == root)
}

/// 🚀️ Publishes the snapshot at once, then what the known sources state now, then the result of
/// the full workspace walk, all off the calling thread. `refresh` ignores the snapshot.
pub fn start(root: PathBuf, refresh: bool) -> Job { start_at(root.clone(), cache_path(&root), refresh) }

/// 🗃️ [`start`] with the snapshot at an explicit place.
pub fn start_at(root: PathBuf, snapshot_path: PathBuf, refresh: bool) -> Job {
    let (sender, receiver) = mpsc::channel();
    let (cancelled, progress, registry) = (Arc::new(AtomicBool::new(false)), Arc::new(Progress::default()), Arc::new(Mutex::new(None)));
    let (signal, observed, slot) = (cancelled.clone(), progress.clone(), registry.clone());
    std::thread::spawn(move || {
        let publish = |registry: Registry, phase: Phase| {
            let registry = Arc::new(registry);
            *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(registry.clone());
            *PUBLISHED.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(registry.clone());
            sender.send(Update::Ready(crate::command_tree::launcher(&registry), phase.key().into())).is_ok()
        };
        let previous = if refresh { None } else { read_snapshot(&snapshot_path, &root) };
        if !publish(previous.as_ref().map_or_else(|| seed(&root), Snapshot::registry), Phase::Known) { return; }
        let gathered = gather(&root, previous.as_ref(), &signal, &observed, &mut |snapshot, phase| { if phase == Phase::Complete || !previous.as_ref().is_some_and(|previous| previous.same_sources(snapshot)) { publish(snapshot.registry(), phase); } });
        if let Some(snapshot) = gathered.filter(|snapshot| previous.as_ref() != Some(snapshot)) {
            if signal.load(Ordering::Relaxed) { let _ = sender.send(Update::Finished); return; }
            if let Err(error) = write_snapshot(&snapshot_path, &snapshot) { let _ = sender.send(Update::Failed(format!("command snapshot: {error}"))); }
        }
        let _ = sender.send(Update::Finished);
    });
    Job { receiver, cancelled, started: std::time::Instant::now(), progress, registry }
}
// #endregion 🔖️Job

// #region 🔖️RunPolicy
pub use crate::registry::GRAPH_REUSE;

/// ⏱️ Whether a launcher entry is a finite task, by the registry's fact about it.
pub fn starts_from_published_graph(label: &str) -> bool {
    let registry = PUBLISHED.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
    let entries = registry.as_ref().map_or(&[][..], |registry| registry.entries());
    let exact = entries.iter().find(|entry| entry.label == label);
    exact.or_else(|| entries.iter().find(|entry| label.strip_prefix(entry.label.as_str()).is_some_and(|rest| rest.starts_with(" / ")))).is_some_and(|entry| !entry.long_running)
}

/// 🧮️ Whether the published Nx project graph still describes the workspace the registry was read from.
pub fn graph_is_current(root: &Path) -> bool { published(root).is_some_and(|registry| registry.graph_is_current()) }
// #endregion 🔖️RunPolicy

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
