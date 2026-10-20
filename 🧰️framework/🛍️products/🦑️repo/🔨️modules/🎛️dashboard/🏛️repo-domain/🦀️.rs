//! 🏛️ The repo domain behind the dashboard's repo commands: tickets, goals, analysis, trees and
//! statutes, answered in process by the Rust domain crates or, with `SEMIO_REPO_IMPLEMENTATION=go`,
//! by the `repo` binary in a pseudo-terminal window. The command registry owns which of these
//! operations are commands; this module owns what each one does.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs

use std::path::{Path, PathBuf};

// #region 🔖️RepoImplementation
/// 🧬️ Which repo implementation the dashboard's repo-domain leaves run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoImplementation {
    /// 🦀️ The Rust domain crates, called in process (the default).
    Rust,
    /// 🐹️ The Go `repo` binary, spawned into a pseudo-terminal window.
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

/// 📍️ Where the Go `repo` binary lives once its build target has run.
pub fn go_binary_path(root: &Path) -> PathBuf {
    let name = if cfg!(windows) { "repo.exe" } else { "repo" };
    let mode = if std::env::var("SEMIO_BUILD_MODE").as_deref() == Ok("release") { "release" } else { "prototype" };
    root.join(REPO_BIN_CACHE_DIR).join(mode).join("repo").join(name)
}
// #endregion 🔖️RepoImplementation

// #region 🔖️RepoAction
/// 🦑️ One repo-domain operation a wizard leaf activates.
///
/// The Rust implementation answers every variant in process through the domain crates — the same
/// functions the `semio` verbs call — and the Go implementation answers the same variant by
/// spawning `repo` with [`RepoAction::go_argv`] into a pseudo-terminal window.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RepoAction {
    TicketShow { id: String },
    TicketFiles { id: String },
    TicketClose { id: String },
    TicketReopen { id: String, prompt: String },
    DraftStart { title: String },
    DraftDiscard { slug: String },
    GoalsList,
    GoalsTree,
    Analyze { scope: String, #[serde(deserialize_with = "explicit_policy")] policy: Option<String> },
    TreeMonorepo,
    TreeGoal,
    TreeStatute,
    TreeTerritory,
    StatutesCatalog,
}

impl RepoAction {
    /// ⚠️ Whether the operation changes repository state, so a view asks before running it.
    pub fn mutates(&self) -> bool { matches!(self, RepoAction::TicketClose { .. } | RepoAction::TicketReopen { .. } | RepoAction::DraftStart { .. } | RepoAction::DraftDiscard { .. }) }

    /// 🛡️ Checks the schema's text, identity and supported analysis choices.
    pub fn validate(&self) -> Result<(), String> {
        let human = |value: &str| !value.trim().is_empty() && value.chars().count() <= 1024 && !value.chars().any(|c| c <= '\u{1f}' || c == '\u{7f}');
        let slug = |value: &str| value.len() <= 1024 && value.split('-').all(|word| !word.is_empty() && word.bytes().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()));
        let ticket = |id: &str| { let parts = id.split('/').collect::<Vec<_>>(); parts.len() >= 4 && parts[..3].iter().all(|part| part.len() == 2 && part.bytes().all(|c| c.is_ascii_digit())) && parts[3..].iter().all(|part| slug(part)) };
        let valid = match self {
            Self::TicketShow { id } | Self::TicketFiles { id } | Self::TicketClose { id } => ticket(id),
            Self::TicketReopen { id, prompt } => ticket(id) && human(prompt),
            Self::DraftStart { title } => human(title),
            Self::DraftDiscard { slug: value } => slug(value),
            Self::Analyze { scope, policy } => ANALYZE_SCOPES.iter().any(|(name, _)| name == scope) && policy.as_deref().is_none_or(|value| value == "code"),
            _ => true,
        };
        if valid { Ok(()) } else { Err("invalid repo action parameters".into()) }
    }

    /// 🎛️ Binds the request's typed values before either implementation is selected.
    pub fn selected(&self, value: &impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let required = |name: &str| value(name).ok_or_else(|| format!("required parameter {name}"));
        let selected = match self {
            Self::DraftStart { .. } => Self::DraftStart { title: required("title")? },
            Self::DraftDiscard { .. } => Self::DraftDiscard { slug: required("slug")? },
            Self::TicketReopen { id, .. } => Self::TicketReopen { id: id.clone(), prompt: required("prompt")? },
            Self::Analyze { scope, .. } => Self::Analyze { scope: scope.clone(), policy: value("policy").filter(|policy| policy != "all") },
            _ => self.clone(),
        };
        selected.validate()?;
        Ok(selected)
    }

    /// 📁️ Admits filesystem targets before either implementation receives the action.
    pub fn admit_root(&self, root: &Path) -> Result<(), String> {
        self.validate()?;
        match self {
            Self::DraftStart { title } => { draft_path(root, &semio_framework_repo_identity::slugify(title))?; },
            Self::DraftDiscard { slug } => { draft_path(root, slug)?; },
            _ => {},
        }
        Ok(())
    }

    /// 🧷️ The `repo` argv that performs the same operation in the Go implementation.
    pub fn go_argv(&self) -> Vec<String> {
        let owned = |parts: &[&str]| parts.iter().map(|part| (*part).to_string()).collect::<Vec<String>>();
        match self {
            RepoAction::TicketShow { id } => owned(&["ticket", "show", id]),
            RepoAction::TicketFiles { id } => owned(&["ticket", "files", id]),
            RepoAction::TicketClose { id } => owned(&["ticket", "close", id, "--summary", DASHBOARD_CLOSE_SUMMARY, "--no-management"]),
            RepoAction::TicketReopen { id, prompt } => owned(&["ticket", "reopen", id, "--prompt", prompt, "--client", DASHBOARD_CLIENT, "--no-management"]),
            RepoAction::DraftStart { title } => owned(&["draft", "create", title]),
            RepoAction::DraftDiscard { slug } => owned(&["draft", "delete", slug]),
            RepoAction::GoalsList => owned(&["goal", "list"]),
            RepoAction::GoalsTree => owned(&["goal", "tree"]),
            RepoAction::Analyze { scope, policy } => match policy { Some(policy) => owned(&["analyze", scope, "--policy", policy]), None => owned(&["analyze", scope]) },
            RepoAction::TreeMonorepo => owned(&["tree", "monorepo"]),
            RepoAction::TreeGoal => owned(&["tree", "goal"]),
            RepoAction::TreeStatute => owned(&["tree", "statute"]),
            RepoAction::TreeTerritory => owned(&["tree", "territory"]),
            RepoAction::StatutesCatalog => owned(&["statute", "list"]),
        }
    }

    /// ⚙️ Runs the operation against the Rust domain crates and renders what it answered.
    pub fn execute(&self, root: &Path) -> String {
        self.execute_result(root).unwrap_or_else(|error| format!("{error}\n"))
    }

    /// 🧾️ Preserves operation failures for supervised process exit status.
    pub fn execute_result(&self, root: &Path) -> Result<String, String> {
        self.validate()?;
        Ok(match self {
            RepoAction::TicketShow { id } => ticket_show(root, id),
            RepoAction::TicketFiles { id } => ticket_files(root, id),
            RepoAction::TicketClose { id } => ticket_close(root, id),
            RepoAction::TicketReopen { id, prompt } => return ticket_reopen(root, id, prompt),
            RepoAction::DraftStart { title } => return draft_start(root, title),
            RepoAction::DraftDiscard { slug } => return draft_discard(root, slug),
            RepoAction::GoalsList => goals_list(root),
            RepoAction::GoalsTree => goals_tree(root),
            RepoAction::Analyze { scope, .. } => analyze(root, scope),
            RepoAction::TreeMonorepo => tree_monorepo(root),
            RepoAction::TreeGoal => tree_goal(root),
            RepoAction::TreeStatute => tree_statute(root),
            RepoAction::TreeTerritory => tree_territory(root),
            RepoAction::StatutesCatalog => statutes_catalog(),
        })
    }
}

fn explicit_policy<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> { serde::Deserialize::deserialize(deserializer) }

/// 🪪️ The client alias a dashboard-driven ticket interaction is recorded under.
pub const DASHBOARD_CLIENT: &str = "claude-code";

/// 📪️ The summary a dashboard-driven close records. The in-process leaf closes in bulk, which the
/// ticket lifecycle answers with its own default summary; the `repo` argv states it, because
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

// #region 🔖️Adapters
use semio_framework_repo_codebase::{Codebase, CodebaseContext};
use semio_framework_repo_goals as goals;
use semio_framework_repo_model::TicketStatus;
use semio_framework_repo_statutes as statutes;
use semio_framework_repo_tickets as tickets;
use semio_framework_repo_tree as tree;

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

/// 🗂️ Every ticket on disk, newest first, as the wizard lists them.
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

/// 📄️ The stored document of one ticket.
pub fn ticket_show(root: &Path, id: &str) -> String {
    with_service(root, |service| match tickets::TicketId::parse(id).and_then(|parsed| service.read(&parsed)) {
        Ok(ticket) => tickets::encode_ticket_document(&ticket),
        Err(error) => format!("ticket {id}: {}\n", error.message),
    })
}

/// 🧺️ Every file the ticket folder carries, repository-relative, in path order.
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

/// 🔒️ Closes a ticket through the ticket lifecycle, as a bulk close: the wizard collects no
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
pub fn ticket_reopen(root: &Path, id: &str, prompt: &str) -> Result<String, String> {
    with_service(root, |service| {
        let request = ticket_reopen_request(id, prompt);
        match service.reopen(&request) {
            Ok(outcome) => Ok(format!("reopened {} -> {}\n{}\n", outcome.id, outcome.status, outcome.warnings.join("\n"))),
            Err(error) => Err(format!("reopen {id}: {}", error.message)),
        }
    })
}

fn ticket_reopen_request(id: &str, prompt: &str) -> tickets::TicketReopenRequest { tickets::TicketReopenRequest { id: id.into(), prompt: prompt.into(), client: DASHBOARD_CLIENT.into(), no_management: true, ..Default::default() } }

fn draft_path(root: &Path, slug: &str) -> Result<PathBuf, String> {
    RepoAction::DraftDiscard { slug: slug.into() }.validate()?;
    let root = root.canonicalize().map_err(|error| error.to_string())?;
    let path = root.join(".🧬semio/🦑️repo/✍️notes").join(slug);
    if path.as_os_str().to_string_lossy().encode_utf16().count() > 256 { return Err("invalid draft path: exceeds 256 characters".into()); }
    let mut ancestor = path.as_path();
    loop {
        match std::fs::symlink_metadata(ancestor) {
            Ok(metadata) if metadata.file_type().is_symlink() => return Err("invalid draft path: symlink".into()),
            Ok(_) => { if !ancestor.canonicalize().map_err(|error| error.to_string())?.starts_with(&root) { return Err("invalid draft path: outside workspace".into()); } },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
            Err(error) => return Err(error.to_string()),
        }
        if ancestor == root { break; }
        ancestor = ancestor.parent().ok_or("invalid draft path")?;
    }
    Ok(path)
}

/// ✍️ Starts an empty draft through the filesystem-only domain service.
pub fn draft_start(root: &Path, title: &str) -> Result<String, String> {
    let slug = semio_framework_repo_identity::slugify(title);
    draft_path(root, &slug)?;
    match semio_framework_repo_todos::create_draft(&semio_framework_repo_todos::FsDraftStore::new(root), title, &[]) {
        Ok(draft) => Ok(format!("created draft {}\n", draft.id)),
        Err(error) => Err(format!("draft start: {error}")),
    }
}

/// 🗑️ Discards one explicitly selected draft through the filesystem-only domain service.
pub fn draft_discard(root: &Path, slug: &str) -> Result<String, String> {
    draft_path(root, slug)?;
    match semio_framework_repo_todos::delete_draft(&semio_framework_repo_todos::FsDraftStore::new(root), slug) {
        Ok(()) => Ok(format!("discarded draft {slug}\n")),
        Err(error) => Err(format!("draft discard: {error}")),
    }
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

/// ⚖️ Runs the statute analysis over one scope of the codebase and lists every breach.
pub fn analyze(root: &Path, scope: &str) -> String {
    println!("[dashboard] analyze {scope}: scanning workspace");
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

/// 🏞️ The monorepo tree as an indented outline.
pub fn tree_monorepo(root: &Path) -> String {
    let source = tree_source(root);
    let node = tree::build_monorepo_tree(&source, tree::TreeBuildOptions::default());
    format!("{}\n", tree::tree_outline(&node).join("\n"))
}

/// 🌴️ The goal tree of the tree crate's own projection.
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

/// 📚️ Every declared statute with its policy and priority.
pub fn statutes_catalog() -> String {
    statutes::statutes().iter().map(|meta| format!("{} [{}] {:?} autofixable={}\n", meta.kind, meta.policy_id, meta.priority, meta.autofixable)).collect()
}
// #endregion 🔖️Adapters

// #region 🔖️Key
/// 🔑️ The stable key of a repo action, used by the projection and by the wizard's window title.
pub fn action_key(action: &RepoAction) -> String {
    match action {
        RepoAction::TicketShow { id } => format!("ticket.show:{id}"),
        RepoAction::TicketFiles { id } => format!("ticket.files:{id}"),
        RepoAction::TicketClose { id } => format!("ticket.close:{id}"),
        RepoAction::TicketReopen { id, .. } => format!("ticket.reopen:{id}"),
        RepoAction::DraftStart { .. } => "draft.start".into(),
        RepoAction::DraftDiscard { .. } => "draft.discard".into(),
        RepoAction::GoalsList => "goals.list".to_string(),
        RepoAction::GoalsTree => "goals.tree".to_string(),
        RepoAction::Analyze { scope, .. } => format!("analyze:{scope}"),
        RepoAction::TreeMonorepo => "tree.monorepo".to_string(),
        RepoAction::TreeGoal => "tree.goal".to_string(),
        RepoAction::TreeStatute => "tree.statute".to_string(),
        RepoAction::TreeTerritory => "tree.territory".to_string(),
        RepoAction::StatutesCatalog => "statutes.catalog".to_string(),
    }
}
// #endregion 🔖️Key

// #region 🔖️Command
/// 🛡️ Executes an owned repo action in a managed process so expensive queries remain cancellable.
pub fn run_action(root: &Path, parsed: &dashboard_core::args::ParsedArgs) -> i32 {
    match parsed.flag("action").and_then(|value| serde_json::from_str::<RepoAction>(value).ok()) {
        Some(action) => match action.execute_result(root) { Ok(output) => { println!("{output}"); 0 }, Err(error) => { eprintln!("[dashboard] {error}"); 1 } },
        _ => { eprintln!("[dashboard] invalid repo action"); 2 }
    }
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
