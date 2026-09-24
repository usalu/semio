//! 🌳️ The wizard command tree: the runtime repo walk that turns every nx target into a wizard
//! path, the playground development leaves injected from the generated catalog, and the repo-domain
//! leaves (tickets, goals, analyze, tree, statutes) that the Rust domain crates answer in process.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

// #region 🔖️Types
/// ▶️ One runnable shell invocation built by the wizard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    pub cmd: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
}

/// 🍃️ What activating a wizard leaf does: run a process in a pseudo-terminal window, or call the
/// repo domain in process and show what it answered in an output window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandLeaf {
    Process(CommandSpec),
    Repo(RepoAction),
}

/// 🌳️ One node in the runtime-discovered command tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandNode {
    pub key: String,
    pub label: String,
    pub children: Vec<CommandNode>,
    pub leaf: Option<CommandLeaf>,
}
// #endregion 🔖️Types

// #region 🔖️Discover
const PROJECT_MANIFESTS: &[&str] = &["📋️project.json", "project.json"];
const WALK_SKIP_DIRS: &[&str] = &["node_modules", "target", ".git", "dist", "build", "generated", "cache"];
const TAXONOMY_SKIP_KEYS: &[&str] = &["packages", "modules", "products", "plugins", "artifacts", "standards", "subsets", "extensions", "targets"];

const VERB_ORDER: &[&str] = &["dev", "build", "test", "verify", "gate", "lint", "format", "generate", "publish", "tickets", "goals", "analyze", "tree", "statutes"];

/// 🧭️ Walks the repo at `root` and builds the wizard command tree.
pub fn discover(root: &Path) -> CommandNode {
    let mut trie = TrieNode::default();
    collect_project_targets(root, root, &mut trie);
    inject_playground_dev(root, &mut trie);
    inject_repo_domain(root, &mut trie, repo_implementation());
    let mut root_node = trie.into_command_node("root", "semio");
    sort_tree(&mut root_node, 0);
    root_node
}
// #endregion 🔖️Discover

// #region 🔖️Trie
#[derive(Default)]
struct TrieNode {
    label: String,
    children: BTreeMap<String, TrieNode>,
    leaf: Option<CommandLeaf>,
}

impl TrieNode {
    fn insert_path(&mut self, path: &[Segment], leaf: CommandLeaf) {
        if path.is_empty() {
            self.leaf = Some(leaf);
            return;
        }
        let head = &path[0];
        let child = self.children.entry(head.key.clone()).or_insert_with(|| TrieNode { label: head.label.clone(), ..Default::default() });
        child.insert_path(&path[1..], leaf);
    }

    fn into_command_node(self, key: &str, label: &str) -> CommandNode {
        CommandNode {
            key: key.to_string(),
            label: label.to_string(),
            children: self
                .children
                .into_iter()
                .map(|(k, n)| {
                    let child_label = if n.label.is_empty() { k.clone() } else { n.label.clone() };
                    n.into_command_node(&k, &child_label)
                })
                .collect(),
            leaf: self.leaf,
        }
    }
}

#[derive(Clone)]
struct Segment {
    key: String,
    label: String,
}

fn segment(key: impl Into<String>, label: impl Into<String>) -> Segment {
    Segment { key: key.into(), label: label.into() }
}
// #endregion 🔖️Trie

// #region 🔖️Walk
fn collect_project_targets(root: &Path, dir: &Path, trie: &mut TrieNode) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if should_skip_walk_dir(name) {
                continue;
            }
            collect_project_targets(root, &path, trie);
            continue;
        }
        if !PROJECT_MANIFESTS.iter().any(|m| path.file_name().and_then(|s| s.to_str()) == Some(*m)) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        let project_name = json.get("name").and_then(|v| v.as_str()).unwrap_or("workspace");
        let targets = json.get("targets").and_then(|v| v.as_object());
        if targets.is_none() {
            continue;
        }
        let manifest_dir = path.parent().unwrap_or(dir);
        let segments = taxonomy_segments(root, manifest_dir);
        for (target, _) in targets.unwrap() {
            let spec = CommandSpec { cmd: "bun".into(), args: vec!["nx".into(), "run".into(), format!("{project_name}:{target}")], cwd: root.to_path_buf(), env: Vec::new() };
            let mut path_segments = vec![segment(target.clone(), target.clone())];
            path_segments.extend(segments.clone());
            trie.insert_path(&path_segments, CommandLeaf::Process(spec));
        }
    }
}

fn should_skip_walk_dir(name: &str) -> bool {
    let key = segment_key(name);
    WALK_SKIP_DIRS.iter().any(|s| key == *s) || name.starts_with('.') && name != ".semio"
}

fn should_skip_taxonomy_segment(name: &str) -> bool {
    let key = segment_key(name);
    TAXONOMY_SKIP_KEYS.iter().any(|s| key == *s)
}

fn taxonomy_segments(root: &Path, manifest_dir: &Path) -> Vec<Segment> {
    let rel = manifest_dir.strip_prefix(root).unwrap_or(manifest_dir);
    rel.components().filter_map(|c| c.as_os_str().to_str()).filter(|c| !c.is_empty()).filter(|c| !should_skip_taxonomy_segment(c)).map(|c| segment(segment_key(c), c.to_string())).filter(|s| !s.key.is_empty()).collect()
}

fn segment_key(component: &str) -> String {
    let s = component.trim();
    let start = s.find(|c: char| c.is_ascii_alphanumeric()).unwrap_or(s.len());
    s[start..].to_ascii_lowercase()
}

fn inject_playground_dev(root: &Path, trie: &mut TrieNode) {
    let catalog = crate::catalog::load_playground_catalog(root);
    for row in catalog {
        for renderer in ["react", "wgpu-wasm", "wgpu-native"] {
            let env = crate::env_contract::build_dev_env(&row.variant, Some(&row), &crate::env_contract::DevOptions { renderer: renderer.into(), ..Default::default() });
            let spec = CommandSpec { cmd: "bun".into(), args: vec!["nx".into(), "run".into(), "@semio-tech/framework-os-dev:dev".into()], cwd: root.to_path_buf(), env };
            let path = vec![segment("dev", "dev"), segment(segment_key(&row.plugin_id), row.plugin_id.clone()), segment(segment_key(&row.variant), row.variant.clone()), segment(renderer, renderer)];
            trie.insert_path(&path, CommandLeaf::Process(spec));
        }
    }
}

fn verb_rank(key: &str) -> usize {
    VERB_ORDER.iter().position(|v| *v == key).unwrap_or(VERB_ORDER.len() + 1)
}

fn sort_tree(node: &mut CommandNode, depth: usize) {
    if depth == 0 {
        node.children.sort_by(|a, b| verb_rank(&a.key).cmp(&verb_rank(&b.key)).then_with(|| a.label.cmp(&b.label)));
    } else {
        node.children.sort_by(|a, b| a.label.cmp(&b.label));
    }
    for child in &mut node.children {
        sort_tree(child, depth + 1);
    }
}
// #endregion 🔖️Walk

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
#[derive(Debug, Clone, PartialEq, Eq)]
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

fn inject_repo_domain(root: &Path, trie: &mut TrieNode, implementation: RepoImplementation) {
    let mut push = |path: Vec<Segment>, action: RepoAction| {
        let leaf = match implementation {
            RepoImplementation::Rust => CommandLeaf::Repo(action),
            RepoImplementation::Go => CommandLeaf::Process(CommandSpec {
                cmd: go_binary_path(root).display().to_string(),
                args: action.go_argv(),
                cwd: root.to_path_buf(),
                env: Vec::new(),
            }),
        };
        trie.insert_path(&path, leaf);
    };

    for ticket in repo_domain::ticket_index(root) {
        let label = format!("{} [{}] {}", ticket.id, ticket.status, ticket.title);
        let head = vec![segment("tickets", "tickets"), segment(ticket.id.clone(), label)];
        for (key, action) in [
            ("show", RepoAction::TicketShow { id: ticket.id.clone() }),
            ("files", RepoAction::TicketFiles { id: ticket.id.clone() }),
            ("close", RepoAction::TicketClose { id: ticket.id.clone() }),
            ("reopen", RepoAction::TicketReopen { id: ticket.id.clone() }),
        ] {
            let mut path = head.clone();
            path.push(segment(key, key));
            push(path, action);
        }
    }

    push(vec![segment("goals", "goals"), segment("list", "list")], RepoAction::GoalsList);
    push(vec![segment("goals", "goals"), segment("tree", "tree")], RepoAction::GoalsTree);

    for (scope, _) in ANALYZE_SCOPES {
        push(vec![segment("analyze", "analyze"), segment(*scope, *scope)], RepoAction::Analyze { scope: (*scope).to_string() });
    }

    for (key, action) in [
        ("monorepo", RepoAction::TreeMonorepo),
        ("goal", RepoAction::TreeGoal),
        ("statute", RepoAction::TreeStatute),
        ("territory", RepoAction::TreeTerritory),
    ] {
        push(vec![segment("tree", "tree"), segment(key, key)], action);
    }

    push(vec![segment("statutes", "statutes"), segment("catalog", "catalog")], RepoAction::StatutesCatalog);
}
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

// #region 🔖️Projection
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
        CommandLeaf::Process(spec) => serde_json::json!({
            "kind": "process",
            "cmd": spec.cmd,
            "args": spec.args,
            "cwd": relative_display(root, &spec.cwd),
            "env": spec.env.iter().map(|(key, value)| serde_json::json!({ "name": key, "value": value })).collect::<Vec<_>>(),
        }),
        CommandLeaf::Repo(action) => serde_json::json!({
            "kind": "repo",
            "action": action_key(action),
            "goArgv": action.go_argv(),
        }),
    }
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
// #endregion 🔖️Projection

// #region 🔖️Command
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
