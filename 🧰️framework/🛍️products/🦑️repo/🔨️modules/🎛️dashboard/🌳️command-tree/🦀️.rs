//! 🌳️ The workspace command tree: the command registry projected as the wizard's steps — the verb,
//! the owner taxonomy and the command — plus the repo-domain actions (tickets, goals, analyze, tree,
//! statutes) that the Rust domain crates answer in process.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🔣️.json

use crate::registry::{Entry, Kind, Registry, RunPolicy};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// #region 🔖️Types
/// ▶️ One runnable process of a command.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommandSpec {
    pub cmd: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
}

/// 🍃️ What activating a wizard leaf does: run a process in a pseudo-terminal window, run several
/// processes side by side, or call the repo domain in process and show what it answered.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CommandLeaf {
    Process(CommandSpec),
    Compound(Vec<CommandSpec>),
    Repo(RepoAction),
}

/// 🌿️ One node of the command tree.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommandNode {
    pub key: String,
    pub label: String,
    pub children: Vec<CommandNode>,
    pub leaf: Option<CommandLeaf>,
}
// #endregion 🔖️Types

// #region 🔖️Discover
/// 🧭️ Discovers the commands of the workspace at `root` and projects them as the wizard tree.
pub fn discover(root: &Path) -> CommandNode { discover_cancellable(root, &std::sync::atomic::AtomicBool::new(false)) }

/// ⏳️ Discovers current commands with cooperative cancellation.
pub fn discover_cancellable(root: &Path, cancelled: &std::sync::atomic::AtomicBool) -> CommandNode { tree(&crate::inventory::discover(root, cancelled), true) }

/// 🌱️ The commands known without reading the workspace: scripts, playgrounds and the root manifest.
pub fn seed(root: &Path) -> CommandNode { tree(&crate::inventory::seed(root), true) }
// #endregion 🔖️Discover

// #region 🔖️Projection
/// 🎚️ The presentation a playground leaf starts with until the developer's preferences are bound to it.
fn presentation() -> Vec<(String, String)> {
    vec![("language".into(), ui_locale::Locale::ALL[0].as_str().into()), ("terminology".into(), "native".into()), ("appearance".into(), "dark".into())]
}

/// 🍂️ The wizard leaves of one registry command with the steps each adds below the command's own:
/// a playground offers one leaf per renderer, every other command one leaf resolved with its defaults.
/// A command that needs a typed parameter has no leaf here.
fn leaves(registry: &Registry, entry: &Entry) -> Vec<(Option<&'static str>, CommandLeaf)> {
    let spec = |process: crate::registry::LaunchProcess| CommandSpec { cmd: process.cmd, args: process.args, cwd: process.cwd, env: process.env };
    if let (Some(action), RepoImplementation::Rust) = (entry.repo_action(), repo_implementation()) { return vec![(None, CommandLeaf::Repo(action.clone()))]; }
    if entry.kind == Kind::Playground {
        return ["react", "wgpu-wasm", "wgpu-native"].into_iter().filter_map(|renderer| {
            let chosen: Vec<(String, String)> = std::iter::once(("renderer".to_string(), renderer.to_string())).chain(presentation()).collect();
            registry.resolve_with(&entry.id, &chosen, &[], RunPolicy::default()).ok().map(|mut launch| (Some(renderer), CommandLeaf::Process(spec(launch.processes.remove(0)))))
        }).collect();
    }
    match registry.resolve_with(&entry.id, &[], &[], RunPolicy::default()) {
        Ok(launch) if entry.is_compound() => vec![(None, CommandLeaf::Compound(launch.processes.into_iter().map(spec).collect()))],
        Ok(mut launch) => vec![(None, CommandLeaf::Process(spec(launch.processes.remove(0))))],
        Err(_) => Vec::new(),
    }
}

/// 🔎️ The flat launcher of the default searchable commands: `verb / owner / … / command` and the leaf it runs.
pub fn launcher(registry: &Registry) -> Vec<(String, CommandLeaf)> {
    registry.entries().iter().filter(|entry| entry.listed).flat_map(|entry| leaves(registry, entry).into_iter().map(move |(step, leaf)| (step.map_or_else(|| entry.label.clone(), |step| format!("{} / {step}", entry.label)), leaf))).collect()
}

#[derive(Default)]
struct TrieNode { label: String, children: BTreeMap<String, TrieNode>, leaf: Option<CommandLeaf> }

impl TrieNode {
    fn insert(&mut self, path: &mut dyn Iterator<Item = (&str, &str)>, leaf: CommandLeaf) {
        match path.next() {
            None => self.leaf = Some(leaf),
            Some((key, label)) => self.children.entry(key.to_string()).or_insert_with(|| TrieNode { label: label.to_string(), ..Default::default() }).insert(path, leaf),
        }
    }

    fn into_command_node(self, key: &str, depth: usize) -> CommandNode {
        let mut children: Vec<CommandNode> = self.children.into_iter().map(|(key, node)| node.into_command_node(&key, depth + 1)).collect();
        if depth == 0 { children.sort_by(|left, right| crate::registry::verb_rank(&left.key).cmp(&crate::registry::verb_rank(&right.key)).then_with(|| left.label.cmp(&right.label))); } else { children.sort_by(|left, right| left.label.cmp(&right.label)); }
        CommandNode { key: key.to_string(), label: if self.label.is_empty() { key.to_string() } else { self.label }, children, leaf: self.leaf }
    }
}

/// 🌳️ The registry as the wizard tree: the verb in launcher order at the root, then each command's
/// place in the owner taxonomy, sorted alphabetically. `unlisted` adds the commands of closed tickets.
pub fn tree(registry: &Registry, unlisted: bool) -> CommandNode {
    let mut trie = TrieNode { label: "semio".into(), ..Default::default() };
    for entry in registry.entries().iter().filter(|entry| unlisted || entry.listed) {
        for (step, leaf) in leaves(registry, entry) {
            trie.insert(&mut std::iter::once((entry.verb.as_str(), entry.verb.as_str())).chain(entry.path.iter().map(|part| (part.key.as_str(), part.label.as_str()))).chain(step.map(|step| (step, step))), leaf);
        }
    }
    trie.into_command_node("root", 0)
}
// #endregion 🔖️Projection

// #region 🔖️RepoImplementation
/// 🧬️ Which repo implementation the dashboard's repo-domain leaves run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoImplementation {
    /// 🦀️ The Rust domain crates, called in process (the default).
    Rust,
    /// 🐹️ The Go `semio-repo` binary, spawned into a pseudo-terminal window.
    Go,
}

/// 🔀️ Reads `SEMIO_REPO_IMPLEMENTATION`; anything other than `go` selects the Rust crates.
pub fn repo_implementation() -> RepoImplementation {
    match std::env::var("SEMIO_REPO_IMPLEMENTATION").unwrap_or_default().trim().to_ascii_lowercase().as_str() {
        "go" => RepoImplementation::Go,
        _ => RepoImplementation::Rust,
    }
}

/// 🗃️ Build output never lives in the taxonomy tree, so the Go binaries land in the marked cache.
pub const REPO_BIN_CACHE_DIR: &str = ".🧬semio/🦑️repo/⚡️cache/🗃️bin";

/// 🐹️ Where the Go `semio-repo` binary lives once its build target has run.
pub fn go_binary_path(root: &Path) -> PathBuf {
    let name = if cfg!(windows) { "semio-repo.exe" } else { "semio-repo" };
    root.join(REPO_BIN_CACHE_DIR).join(name)
}
// #endregion 🔖️RepoImplementation

// #region 🔖️RepoAction
/// 🦑️ One repo-domain operation a wizard leaf activates.
///
/// The Rust implementation answers every variant in process through the domain crates — the same
/// functions the `semio` verbs call — and the Go implementation answers the same variant by
/// spawning `semio-repo` with [`RepoAction::go_argv`] into a pseudo-terminal window.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RepoAction {
    TicketShow { id: String },
    TicketFiles { id: String },
    TicketClose { id: String },
    TicketReopen { id: String },
    GoalsList,
    GoalsTree,
    Analyze { scope: String },
    TreeMonorepo,
    TreeGoal,
    TreeStatute,
    TreeTerritory,
    StatutesCatalog,
}

impl RepoAction {
    /// ⚠️ Whether the operation changes repository state, so a view asks before running it.
    pub fn mutates(&self) -> bool { matches!(self, RepoAction::TicketClose { .. } | RepoAction::TicketReopen { .. }) }

    /// 🐹️ The `semio-repo` argv that performs the same operation in the Go implementation.
    pub fn go_argv(&self) -> Vec<String> {
        let owned = |parts: &[&str]| parts.iter().map(|part| (*part).to_string()).collect::<Vec<String>>();
        match self {
            RepoAction::TicketShow { id } => owned(&["ticket", "show", id]),
            RepoAction::TicketFiles { id } => owned(&["ticket", "files", id]),
            RepoAction::TicketClose { id } => owned(&["ticket", "close", id, "--summary", DASHBOARD_CLOSE_SUMMARY, "--no-management"]),
            RepoAction::TicketReopen { id } => owned(&["ticket", "reopen", id, "--client", DASHBOARD_CLIENT, "--no-management"]),
            RepoAction::GoalsList => owned(&["goal", "list"]),
            RepoAction::GoalsTree => owned(&["goal", "tree"]),
            RepoAction::Analyze { scope } => owned(&["analyze", scope]),
            RepoAction::TreeMonorepo => owned(&["tree", "monorepo"]),
            RepoAction::TreeGoal => owned(&["tree", "goal"]),
            RepoAction::TreeStatute => owned(&["tree", "statute"]),
            RepoAction::TreeTerritory => owned(&["tree", "territory"]),
            RepoAction::StatutesCatalog => owned(&["statute", "list"]),
        }
    }

    /// 🦀️ Runs the operation against the Rust domain crates and renders what it answered.
    pub fn execute(&self, root: &Path) -> String {
        match self {
            RepoAction::TicketShow { id } => repo_domain::ticket_show(root, id),
            RepoAction::TicketFiles { id } => repo_domain::ticket_files(root, id),
            RepoAction::TicketClose { id } => repo_domain::ticket_close(root, id),
            RepoAction::TicketReopen { id } => repo_domain::ticket_reopen(root, id),
            RepoAction::GoalsList => repo_domain::goals_list(root),
            RepoAction::GoalsTree => repo_domain::goals_tree(root),
            RepoAction::Analyze { scope } => repo_domain::analyze(root, scope),
            RepoAction::TreeMonorepo => repo_domain::tree_monorepo(root),
            RepoAction::TreeGoal => repo_domain::tree_goal(root),
            RepoAction::TreeStatute => repo_domain::tree_statute(root),
            RepoAction::TreeTerritory => repo_domain::tree_territory(root),
            RepoAction::StatutesCatalog => repo_domain::statutes_catalog(),
        }
    }
}

/// 🪪️ The client alias a dashboard-driven ticket interaction is recorded under.
pub const DASHBOARD_CLIENT: &str = "claude-code";

/// 📪️ The summary a dashboard-driven close records. The in-process leaf closes in bulk, which the
/// ticket lifecycle answers with its own default summary; the `semio-repo` argv states it, because
/// `ticket close` has no spelling for a single-ticket bulk close — `TicketCloseInput.all` means
/// "every open ticket" — so the Go leaf still needs the file list the CLI asks for.
pub const DASHBOARD_CLOSE_SUMMARY: &str = "Closed from the semio dashboard";

/// 🔬️ Every analysis scope the `analyze` branch offers, with the extensions it reads.
pub const ANALYZE_SCOPES: &[(&str, &[&str])] = &[
    ("all", &["rs", "ts", "tsx", "go", "py", "cs", "md", "json"]),
    ("rust", &["rs"]),
    ("typescript", &["ts", "tsx"]),
    ("go", &["go"]),
    ("python", &["py"]),
    ("markdown", &["md"]),
];

// #endregion 🔖️RepoAction

// #region 🔖️RepoDomain
/// 🦑️ The in-process adapters between the wizard and the Rust repo-domain crates.
pub mod repo_domain {
    use super::{DASHBOARD_CLIENT, ANALYZE_SCOPES};
    use semio_framework_repo_codebase::{Codebase, CodebaseContext};
    use semio_framework_repo_goals as goals;
    use semio_framework_repo_model::TicketStatus;
    use semio_framework_repo_statutes as statutes;
    use semio_framework_repo_tickets as tickets;
    use semio_framework_repo_tree as tree;
    use std::path::Path;

    /// 🎫️ The identifying members of one ticket the wizard offers.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TicketEntry {
        pub id: String,
        pub status: String,
        pub title: String,
        pub goal: String,
        pub folder_path: String,
    }

    /// ⏰️ The wall clock the ticket lifecycle reads when the dashboard drives it.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct SystemClock;

    impl tickets::Clock for SystemClock {
        fn today(&self) -> (i64, i64, i64) {
            let (year, month, day, _, _, _) = civil_now();
            (year % 100, month, day)
        }

        fn stamp(&self) -> String {
            let (year, month, day, hour, minute, second) = civil_now();
            format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}")
        }
    }

    /// 🗓️ The current UTC instant as `(year, month, day, hour, minute, second)`, by the civil-from-days
    /// algorithm, so no date library enters the runtime.
    fn civil_now() -> (i64, i64, i64, i64, i64, i64) {
        let seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|value| value.as_secs() as i64).unwrap_or_default();
        let days = seconds.div_euclid(86_400);
        let rest = seconds.rem_euclid(86_400);
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z.rem_euclid(146_097);
        let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = if month <= 2 { y + 1 } else { y };
        (year, month, day, rest / 3_600, (rest % 3_600) / 60, rest % 60)
    }

    /// 📡️ The emitter that forwards a goal event to the repository coordinator.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct CoordinatorEmitter;

    impl semio_framework_repo_events::Emitter for CoordinatorEmitter {
        fn emit(&self, kind: &str, source: &str, payload: &serde_json::Value) {
            semio_framework_repo_events::emit(kind, source, payload);
        }
    }

    fn repo_meta_dir(root: &Path) -> String {
        format!("{}/.🧬semio/🦑️repo", root.display().to_string().replace('\\', "/"))
    }

    fn layout(root: &Path) -> tickets::TicketLayout {
        tickets::TicketLayout::new(repo_meta_dir(root))
    }

    /// 🎫️ Every ticket on disk, newest first, as the wizard lists them.
    pub fn ticket_index(root: &Path) -> Vec<TicketEntry> {
        let store = tickets::FileTicketStore::new();
        let tracker = semio_framework_repo_providers::system_management_provider();
        let clock = SystemClock;
        let events = tickets::CoordinatorEventSink;
        let service = tickets::TicketService::new(layout(root), &store, &tracker, &clock, &events);
        let mut entries: Vec<TicketEntry> = service
            .list(None, None, None)
            .unwrap_or_default()
            .into_iter()
            .map(|ticket| TicketEntry {
                id: format!("{:02}/{:02}/{:02}/{}", ticket.year, ticket.month, ticket.day, ticket.slug),
                status: match ticket.status {
                    TicketStatus::Open => "open".to_string(),
                    TicketStatus::Closed => "closed".to_string(),
                },
                title: ticket.title,
                goal: ticket.goal,
                folder_path: ticket.folder_path,
            })
            .collect();
        entries.sort_by(|left, right| right.id.cmp(&left.id));
        entries
    }

    fn with_service<R>(root: &Path, body: impl FnOnce(&tickets::TicketService<'_, tickets::FileTicketStore, tickets::ManagementProviders, SystemClock, tickets::CoordinatorEventSink>) -> R) -> R {
        let store = tickets::FileTicketStore::new();
        let tracker = semio_framework_repo_providers::system_management_provider();
        let clock = SystemClock;
        let events = tickets::CoordinatorEventSink;
        let service = tickets::TicketService::new(layout(root), &store, &tracker, &clock, &events);
        body(&service)
    }

    /// 🎫️ The stored document of one ticket.
    pub fn ticket_show(root: &Path, id: &str) -> String {
        with_service(root, |service| match tickets::TicketId::parse(id).and_then(|parsed| service.read(&parsed)) {
            Ok(ticket) => tickets::encode_ticket_document(&ticket),
            Err(error) => format!("ticket {id}: {}\n", error.message),
        })
    }

    /// 📄️ Every file the ticket folder carries, repository-relative, in path order.
    pub fn ticket_files(root: &Path, id: &str) -> String {
        let entry = ticket_index(root).into_iter().find(|entry| entry.id == id);
        let Some(entry) = entry else { return format!("ticket {id}: not found\n") };
        let store = tickets::FileTicketStore::new();
        let mut found = Vec::new();
        collect_files(&store, &entry.folder_path, &mut found);
        found.sort();
        let prefix = format!("{}/", root.display().to_string().replace('\\', "/"));
        let listed: Vec<String> = found.into_iter().map(|path| path.strip_prefix(&prefix).map(str::to_string).unwrap_or(path)).collect();
        format!("{}\n{}\n", entry.id, listed.join("\n"))
    }

    fn collect_files<S: tickets::TicketStore>(store: &S, directory: &str, found: &mut Vec<String>) {
        let Ok(entries) = store.entries(directory) else { return };
        for entry in entries {
            let child = format!("{directory}/{}", entry.name);
            match entry.kind {
                tickets::NodeKind::Directory => collect_files(store, &child, found),
                _ => found.push(child),
            }
        }
    }

    /// 📪️ Closes a ticket through the ticket lifecycle, as a bulk close: the wizard collects no
    /// summary and no file list, and never reaches the issue tracker from a keystroke.
    pub fn ticket_close(root: &Path, id: &str) -> String {
        with_service(root, |service| {
            let request = tickets::TicketCloseRequest { id: id.to_string(), summary: String::new(), files: Vec::new(), no_management: true, bulk: true };
            match service.close(&request) {
                Ok(outcome) => format!("closed {} -> {}\n{}\n", outcome.id, outcome.status, outcome.warnings.join("\n")),
                Err(error) => format!("close {id}: {}\n", error.message),
            }
        })
    }

    /// 🔓️ Reopens a closed ticket through the ticket lifecycle.
    pub fn ticket_reopen(root: &Path, id: &str) -> String {
        with_service(root, |service| {
            let request = tickets::TicketReopenRequest { id: id.to_string(), prompt: "Reopened from the semio dashboard".to_string(), client: DASHBOARD_CLIENT.to_string(), no_management: true, ..Default::default() };
            match service.reopen(&request) {
                Ok(outcome) => format!("reopened {} -> {}\n{}\n", outcome.id, outcome.status, outcome.warnings.join("\n")),
                Err(error) => format!("reopen {id}: {}\n", error.message),
            }
        })
    }

    fn goal_seeds(root: &Path) -> Vec<goals::GoalSeed> {
        let store = goals::FsGoalStore::new(root);
        let management = goals::NullManagement;
        let emitter = CoordinatorEmitter;
        let aggregate = goals::Goals::new(&store, &management, &emitter, DASHBOARD_CLIENT);
        aggregate
            .list()
            .unwrap_or_default()
            .into_iter()
            .map(|goal| goals::GoalSeed { id: goal.id, title: goal.title, status: goal.status.as_str().to_string(), due_date: if goal.dates.due.is_empty() { goal.due_date } else { goal.dates.due }, created_at: String::new(), description: goal.description })
            .collect()
    }

    fn ticket_seeds(root: &Path) -> Vec<goals::TicketSeed> {
        ticket_index(root)
            .into_iter()
            .map(|entry| goals::TicketSeed { id: entry.id.clone(), slug: entry.id, status: entry.status, title: entry.title, goal: entry.goal, ..Default::default() })
            .collect()
    }

    /// 🎯️ Every goal, identifier ascending.
    pub fn goals_list(root: &Path) -> String {
        goal_seeds(root).iter().map(|seed| format!("{} [{}] {}\n", seed.id, seed.status, seed.title)).collect()
    }

    /// 🌳️ The goal forest with the tickets hanging under it.
    pub fn goals_tree(root: &Path) -> String {
        let roots = goals::build_goal_tree(&goal_seeds(root), &ticket_seeds(root));
        goals::render_goal_tree(&roots, goals::TreeFormat::Text, &goals::PlainTreeLines)
    }

    fn scope_files(root: &Path, scope: &str) -> Vec<String> {
        let extensions = ANALYZE_SCOPES.iter().find(|(name, _)| *name == scope).map_or(&["rs"][..], |(_, extensions)| *extensions);
        Codebase::new(root).glob_by_extension(".", extensions, &[], true)
    }

    /// 🔬️ Runs the statute analysis over one scope of the codebase and lists every breach.
    pub fn analyze(root: &Path, scope: &str) -> String {
        let files = scope_files(root, scope);
        let sources: Vec<statutes::SourceFile> = files
            .iter()
            .filter_map(|path| std::fs::read_to_string(root.join(path)).ok().map(|content| statutes::SourceFile { path: path.clone(), content }))
            .collect();
        let considered = sources.len();
        let set = statutes::SourceSet::new(sources);
        let breachs = statutes::filter_ignored(statutes::analyze(&set), &set);
        let mut out = format!("analyze {scope}: {considered} files, {} breaches\n", breachs.len());
        for breach in &breachs {
            out.push_str(&format!("{} {}:{} {}\n", breach.kind, breach.scope, breach.line, breach.summary));
        }
        out
    }

    fn tree_source(root: &Path) -> tree::MemoryTreeSource {
        let codebase = Codebase::new(root);
        let mut context = CodebaseContext::new(&codebase);
        context.load_bundles();
        context.load_files();
        let technologies = codebase
            .technologies()
            .iter()
            .map(|technology| tree::TechnologyRecord {
                id: codebase.technology_id(technology),
                name: technology.name.clone(),
                uri: format!("repo://technology/{}", technology.name),
                kind: technology.kind.as_str().to_string(),
                emoji: technology.emoji.clone(),
                bundles: technology
                    .bundles
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|bundle| tree::BundleRecord {
                        id: codebase.bundle_id(bundle),
                        name: bundle.name.clone(),
                        uri: format!("repo://bundle/{}", bundle.name),
                        kind: bundle.kind.as_str().to_string(),
                        emoji: bundle.emoji.clone(),
                        root: bundle.root.clone(),
                        source_root: bundle.source_root.clone(),
                    })
                    .collect(),
            })
            .collect();
        let folders = context.build_folders().into_iter().map(|folder| tree::FolderRecord { id: folder.id, path: folder.path.clone(), name: folder.name, uri: folder.uri, kind: "folder".to_string(), parent_id: folder.parent_id.unwrap_or_default() }).collect();
        let files = context
            .build_files()
            .into_iter()
            .map(|file| {
                let name = file.path.rsplit('/').next().unwrap_or(&file.path).to_string();
                tree::FileRecord { id: file.id, path: file.path, name, uri: file.uri, kind: "file".to_string(), parent_id: file.folder_id.unwrap_or_default() }
            })
            .collect();
        let goal_records = goal_seeds(root)
            .into_iter()
            .map(|seed| tree::GoalRecord { id: seed.id.clone(), title: seed.title, uri: format!("repo://goal/{}", seed.id), status: seed.status, due_date: seed.due_date, created_at: seed.created_at, description: seed.description, ..Default::default() })
            .collect();
        let ticket_records = ticket_index(root)
            .into_iter()
            .map(|entry| tree::TicketRecord { id: entry.id.clone(), slug: entry.id.clone(), title: entry.title, uri: format!("repo://ticket/{}", entry.id), goal: entry.goal, status: entry.status, ..Default::default() })
            .collect();
        tree::MemoryTreeSource { technologies, folders, files, goals: goal_records, tickets: ticket_records, ..Default::default() }
    }

    /// 🌳️ The monorepo tree as an indented outline.
    pub fn tree_monorepo(root: &Path) -> String {
        let source = tree_source(root);
        let node = tree::build_monorepo_tree(&source, tree::TreeBuildOptions::default());
        format!("{}\n", tree::tree_outline(&node).join("\n"))
    }

    /// 🌳️ The goal tree of the tree crate's own projection.
    pub fn tree_goal(root: &Path) -> String {
        let source = tree_source(root);
        let roots = tree::build_goal_tree(&source.goals, &source.tickets);
        let mut lines = Vec::new();
        for goal in &roots {
            lines.push(format!("{} [{}] open-subgoals={} open-tickets={}", goal.id, goal.status, tree::count_open_subgoals(goal), tree::count_open_tickets(goal)));
        }
        format!("{}\n", lines.join("\n"))
    }

    /// 📜️ The statute tree as an indented outline.
    pub fn tree_statute(_root: &Path) -> String {
        let roots = tree::build_statute_tree(&tree::declared_statutes(), &tree::DeclaredStatuteCatalog);
        outline_of(&roots)
    }

    /// 🗺️ The territory tree as an indented outline.
    pub fn tree_territory(_root: &Path) -> String {
        let roots = tree::build_territory_tree(&tree::declared_territories(), &tree::DeclaredStatuteCatalog);
        outline_of(&roots)
    }

    fn outline_of(roots: &[tree::TreeNode]) -> String {
        let mut lines = Vec::new();
        for node in roots {
            lines.extend(tree::tree_outline(node));
        }
        format!("{}\n", lines.join("\n"))
    }

    /// 📜️ Every declared statute with its policy and priority.
    pub fn statutes_catalog() -> String {
        statutes::statutes().iter().map(|meta| format!("{} [{}] {:?} autofixable={}\n", meta.kind, meta.policy_id, meta.priority, meta.autofixable)).collect()
    }
}
// #endregion 🔖️RepoDomain

// #region 🔖️Document
/// 🔣️ The discovered tree as the JSON document `🧬️schema/🔣️.json` describes.
pub fn tree_json(root: &Path, node: &CommandNode) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert("key".to_string(), serde_json::Value::String(node.key.clone()));
    object.insert("label".to_string(), serde_json::Value::String(node.label.clone()));
    if let Some(leaf) = &node.leaf {
        object.insert("leaf".to_string(), leaf_json(root, leaf));
    }
    if !node.children.is_empty() {
        object.insert("children".to_string(), serde_json::Value::Array(node.children.iter().map(|child| tree_json(root, child)).collect()));
    }
    serde_json::Value::Object(object)
}

/// 🔣️ The discovered tree of a repository root as the pretty JSON document `semio command-tree
/// --dump-tree` prints, so a client never has to name the encoder crate itself.
pub fn tree_json_text(root: &Path) -> String {
    serde_json::to_string_pretty(&tree_json(root, &discover(root))).unwrap_or_else(|error| format!("{{\"error\":\"{error}\"}}"))
}

fn leaf_json(root: &Path, leaf: &CommandLeaf) -> serde_json::Value {
    match leaf {
        CommandLeaf::Process(spec) => process_json(root, spec),
        CommandLeaf::Compound(specs) => serde_json::json!({ "kind": "compound", "processes": specs.iter().map(|spec| process_json(root, spec)).collect::<Vec<_>>() }),
        CommandLeaf::Repo(action) => serde_json::json!({
            "kind": "repo",
            "action": action_key(action),
            "goArgv": action.go_argv(),
        }),
    }
}

fn process_json(root: &Path, spec: &CommandSpec) -> serde_json::Value {
    serde_json::json!({
        "kind": "process",
        "cmd": spec.cmd,
        "args": spec.args,
        "cwd": relative_display(root, &spec.cwd),
        "env": spec.env.iter().map(|(key, value)| serde_json::json!({ "name": key, "value": value })).collect::<Vec<_>>(),
    })
}

fn relative_display(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).map_or_else(|_| path.display().to_string().replace('\\', "/"), |rest| rest.display().to_string().replace('\\', "/"))
}

/// 🔑️ The stable key of a repo action, used by the projection and by the wizard's window title.
pub fn action_key(action: &RepoAction) -> String {
    match action {
        RepoAction::TicketShow { id } => format!("ticket.show:{id}"),
        RepoAction::TicketFiles { id } => format!("ticket.files:{id}"),
        RepoAction::TicketClose { id } => format!("ticket.close:{id}"),
        RepoAction::TicketReopen { id } => format!("ticket.reopen:{id}"),
        RepoAction::GoalsList => "goals.list".to_string(),
        RepoAction::GoalsTree => "goals.tree".to_string(),
        RepoAction::Analyze { scope } => format!("analyze:{scope}"),
        RepoAction::TreeMonorepo => "tree.monorepo".to_string(),
        RepoAction::TreeGoal => "tree.goal".to_string(),
        RepoAction::TreeStatute => "tree.statute".to_string(),
        RepoAction::TreeTerritory => "tree.territory".to_string(),
        RepoAction::StatutesCatalog => "statutes.catalog".to_string(),
    }
}
// #endregion 🔖️Document

// #region 🔖️Command
/// 🦀️ Executes an owned repo action in a managed process so expensive queries remain cancellable.
pub fn run_action(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {
    match parsed.flag("action").and_then(|value| serde_json::from_str::<RepoAction>(value).ok()) {
        Some(action) => { println!("{}", action.execute(root)); 0 },
        None => { eprintln!("[dashboard] invalid repo action"); 2 }
    }
}

/// 🌳️ Presents the discovered command tree without entering the interactive dashboard.
pub fn run(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {
    let root = parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from);
    if parsed.has_flag("dump-tree") {
        println!("{}", tree_json_text(&root));
        return 0;
    }
    for line in outline(&discover(&root), 0) {
        println!("{line}");
    }
    0
}

fn outline(node: &CommandNode, depth: usize) -> Vec<String> {
    let mut lines = vec![format!("{}{}", "  ".repeat(depth), node.label)];
    for child in &node.children {
        lines.extend(outline(child, depth + 1));
    }
    lines
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
