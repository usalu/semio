//! 💾️ The important-document transaction and the plan source resolution.

use super::*;

//#region 💾️Important

/// 📓️ One recorded step of an important-document transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalStep {
    pub op: String,
    pub path: String,
    pub outcome: String,
}

/// 📓️ The ordered record of every step a transaction attempted, in the order it attempted them.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionJournal {
    pub steps: Vec<JournalStep>,
}

impl TransactionJournal {
    /// ➕️ Records one step.
    pub fn record(&mut self, op: &str, path: &str, outcome: &str) {
        self.steps.push(JournalStep { op: op.to_string(), path: path.to_string(), outcome: outcome.to_string() });
    }

    /// 🔤️ The step sequence as `op:outcome` pairs, for a compact projection.
    pub fn trace(&self) -> Vec<String> {
        self.steps.iter().map(|step| format!("{}:{}", step.op, step.outcome)).collect()
    }
}

/// 🖼️ The exact empty-document bundle a close must find and may restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportantPreimage {
    pub path: String,
    pub dir: String,
    pub mode: u32,
}

/// ✨️ What a reopen created, so a failed save removes exactly that and nothing else.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportantCreation {
    pub path: String,
    pub dir: String,
    pub file_created: bool,
    pub dir_created: bool,
}

/// 🔎️ Requires the canonical important document to be an exact empty regular-file bundle.
pub fn inspect_important_document<S: TicketStore>(store: &S, path: &str, journal: &mut TransactionJournal) -> TicketResult<ImportantPreimage> {
    let dir = dir_name(path);
    let Some(metadata) = store.metadata(path) else {
        journal.record("inspect", path, "failed");
        return Err(TicketError::not_found(format!("cannot finish ticket: required important document {path} is unavailable")));
    };
    if metadata.kind != NodeKind::File {
        journal.record("inspect", path, "failed");
        return Err(TicketError::invalid(format!("cannot finish ticket: important document {path} is not a regular file")));
    }
    let content = store.read(path)?;
    if !content.is_empty() {
        journal.record("inspect", path, "failed");
        return Err(TicketError::invalid(format!("cannot finish ticket: important document {path} is not empty")));
    }
    let entries = store.entries(&dir)?;
    let expected = base_name(path);
    if entries.len() != 1 || entries[0].name != expected {
        journal.record("inspect", &dir, "failed");
        return Err(TicketError::invalid(format!("cannot finish ticket: important directory {dir} must contain only {expected}")));
    }
    journal.record("inspect", path, "ok");
    Ok(ImportantPreimage { path: path.to_string(), dir, mode: metadata.mode })
}

/// 🔄️ Restores an empty document without overwriting an existing node.
pub fn restore_important_document<S: TicketStore>(store: &S, preimage: &ImportantPreimage, journal: &mut TransactionJournal) -> TicketResult<()> {
    store.create_directory(&preimage.dir)?;
    store.create_exclusive(&preimage.path, preimage.mode & 0o777)?;
    journal.record("restore", &preimage.path, "ok");
    Ok(())
}

/// 🗑️ Removes the exact empty document bundle, restoring it when the directory will not go.
pub fn remove_important_document<S: TicketStore>(store: &S, preimage: &ImportantPreimage, journal: &mut TransactionJournal) -> TicketResult<()> {
    store.remove_file(&preimage.path)?;
    journal.record("remove-file", &preimage.path, "ok");
    if let Err(error) = store.remove_directory(&preimage.dir) {
        journal.record("remove-dir", &preimage.dir, "failed");
        return match restore_important_document(store, preimage, journal) {
            Ok(()) => Err(TicketError::store(format!("remove important directory: {error}"))),
            Err(restore) => Err(TicketError::store(format!("remove important directory: {error}; restore important document: {restore}"))),
        };
    }
    journal.record("remove-dir", &preimage.dir, "ok");
    Ok(())
}

/// ✨️ Preserves an existing regular document or exclusively creates an empty one.
pub fn ensure_important_document<S: TicketStore>(store: &S, path: &str, journal: &mut TransactionJournal) -> TicketResult<ImportantCreation> {
    let dir = dir_name(path);
    let mut creation = ImportantCreation { path: path.to_string(), dir: dir.clone(), file_created: false, dir_created: false };
    if let Some(metadata) = store.metadata(path) {
        if metadata.kind != NodeKind::File {
            journal.record("ensure", path, "failed");
            return Err(TicketError::invalid(format!("important document {path} is not a regular file")));
        }
        store.read(path)?;
        journal.record("ensure", path, "preserved");
        return Ok(creation);
    }
    match store.metadata(&dir) {
        Some(metadata) if metadata.kind == NodeKind::Directory => {}
        Some(_) => {
            journal.record("ensure-dir", &dir, "failed");
            return Err(TicketError::invalid(format!("important directory {dir} is not a physical directory")));
        }
        None => {
            store.create_directory(&dir)?;
            creation.dir_created = true;
            journal.record("ensure-dir", &dir, "created");
        }
    }
    if let Err(error) = store.create_exclusive(path, 0o644) {
        if creation.dir_created {
            let _ = store.remove_directory(&dir);
        }
        journal.record("ensure", path, "failed");
        return Err(error);
    }
    creation.file_created = true;
    journal.record("ensure", path, "created");
    Ok(creation)
}

/// ↩️ Removes only the nodes the current reopen created.
pub fn rollback_important_creation<S: TicketStore>(store: &S, creation: &ImportantCreation, journal: &mut TransactionJournal) -> TicketResult<()> {
    if creation.file_created && store.exists(&creation.path) {
        store.remove_file(&creation.path)?;
        journal.record("rollback-file", &creation.path, "ok");
    }
    if creation.dir_created && store.exists(&creation.dir) {
        store.remove_directory(&creation.dir)?;
        journal.record("rollback-dir", &creation.dir, "ok");
    }
    Ok(())
}

//#endregion 💾️Important

//#region 🗺️PlanSource

/// 🗺️ Where a plan or spec id resolves to, and whether it names a directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanSource {
    pub path: String,
    pub is_directory: bool,
}

/// 🏠️ The two roots plan resolution reads: the repository and the contributor's home directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanRoots {
    pub repo_root: String,
    pub home_dir: String,
}

/// 🏷️ The value stored in `ticket.json` for the plan attachment of a client.
pub fn plan_client_tag(kind: McpClientKind) -> &'static str {
    match kind {
        McpClientKind::Cursor => "cursor",
        McpClientKind::Kiro => "kiro",
        McpClientKind::Copilot => "copilot",
        McpClientKind::Claude => "claude",
        McpClientKind::Codex => "codex",
        McpClientKind::Generic => "generic",
    }
}

/// 🗺️ Resolves a plan or spec id to a store path, per agent client.
///
/// Kiro requires the directory `<root>/.kiro/specs/<id>`; Copilot, Claude and Codex each name one
/// file under the contributor's home directory. Cursor and the generic surface carry no plan
/// attachment: a ticket owns the `*.plan.md` files of its own folder and no plan is read from a
/// client directory inside the repository.
pub fn resolve_plan_source<S: TicketStore>(store: &S, roots: &PlanRoots, kind: McpClientKind, id: &str) -> TicketResult<PlanSource> {
    let id = id.trim();
    if id.is_empty() {
        return Err(TicketError::invalid("plan or spec id is empty"));
    }
    let root = roots.repo_root.trim();
    if root.is_empty() {
        return Err(TicketError::invalid("repository root is not set"));
    }
    match kind {
        McpClientKind::Kiro => {
            let directory = join_path(root, &format!(".kiro/specs/{id}"));
            match store.metadata(&directory) {
                Some(metadata) if metadata.kind == NodeKind::Directory => Ok(PlanSource { path: directory, is_directory: true }),
                Some(_) => Err(TicketError::invalid(format!("Kiro spec {id:?} is not a directory"))),
                None => Err(TicketError::not_found(format!("Kiro spec {id:?}: no such directory"))),
            }
        }
        McpClientKind::Copilot | McpClientKind::Claude | McpClientKind::Codex => {
            let home = roots.home_dir.trim();
            if home.is_empty() {
                return Err(TicketError::invalid("home directory is not set"));
            }
            let repo_base = base_name(root);
            let path = match kind {
                McpClientKind::Copilot => join_path(home, &format!(".copilot/projects/{repo_base}/memory/{id}.md")),
                McpClientKind::Claude => join_path(home, &format!(".claude/plans/{id}.md")),
                _ => join_path(home, &format!(".codex/memory/{repo_base}/{id}.md")),
            };
            match store.metadata(&path) {
                Some(metadata) if metadata.kind == NodeKind::Directory => Err(TicketError::invalid(format!("expected file at {path}"))),
                Some(_) => Ok(PlanSource { path, is_directory: false }),
                None => Err(TicketError::not_found(format!("plan file for id {id:?}: no such file"))),
            }
        }
        McpClientKind::Cursor | McpClientKind::Generic => Err(TicketError::invalid(format!("plan or spec attachment is not supported for mcp kind {:?}", kind.as_str()))),
    }
}

/// 📎️ Resolves `plan_id` or `spec_id` and attaches it to a ticket before it is saved.
pub fn apply_ticket_plan_from_ids<S: TicketStore>(store: &S, roots: &PlanRoots, ticket: &mut Ticket, kind: McpClientKind, plan_id: &str, spec_id: &str) -> TicketResult<()> {
    let plan_id = plan_id.trim();
    let spec_id = spec_id.trim();
    if plan_id.is_empty() && spec_id.is_empty() {
        return Ok(());
    }
    if !plan_id.is_empty() && !spec_id.is_empty() {
        return Err(TicketError::invalid("pass only one of plan_id or spec_id"));
    }
    let id = if kind == McpClientKind::Kiro {
        if !plan_id.is_empty() {
            return Err(TicketError::invalid("use spec_id for Kiro, not plan_id"));
        }
        spec_id
    } else {
        if !spec_id.is_empty() {
            return Err(TicketError::invalid("use plan_id for this client, not spec_id"));
        }
        plan_id
    };
    let source = resolve_plan_source(store, roots, kind, id)?;
    ticket.plan = Some(TicketPlan { client: plan_client_tag(kind).to_string(), id: id.to_string(), source: source.path, local: String::new() });
    Ok(())
}

/// 📦️ Moves an attached plan or spec into the ticket folder as the ticket closes.
pub fn move_ticket_plan_into_folder<S: TicketStore>(store: &S, ticket: &mut Ticket) -> TicketResult<()> {
    let Some(plan) = ticket.plan.clone() else { return Ok(()) };
    if plan.source.trim().is_empty() {
        return Ok(());
    }
    if ticket.folder_path.is_empty() {
        return Err(TicketError::invalid("ticket folder path is empty"));
    }
    let source = normalize_separators(&plan.source);
    let destination_name = base_name(&source);
    let destination = join_path(&ticket.folder_path, &destination_name);
    if !store.exists(&source) {
        if store.exists(&destination) {
            if let Some(plan) = ticket.plan.as_mut() {
                plan.local = destination_name;
                plan.source = String::new();
            }
            return Ok(());
        }
        return Err(TicketError::not_found(format!("plan source {source:?} missing and destination {destination:?} not found")));
    }
    store.rename(&source, &destination)?;
    if let Some(plan) = ticket.plan.as_mut() {
        plan.local = destination_name;
        plan.source = String::new();
    }
    Ok(())
}

/// 📝️ Strips YAML frontmatter from plan markdown.
pub fn strip_plan_frontmatter(content: &str) -> String {
    if !content.starts_with("---") {
        return content.trim().to_string();
    }
    let Some(end) = content[3..].find("\n---") else { return content.trim().to_string() };
    let mut rest = &content[3 + end + 4..];
    if let Some(stripped) = rest.strip_prefix("\r\n") {
        rest = stripped;
    } else if let Some(stripped) = rest.strip_prefix('\n') {
        rest = stripped;
    }
    rest.trim().to_string()
}

/// 📝️ Formats one plan file as a collapsible GitHub markdown section.
pub fn format_plan_file_section(name: &str, raw: &str) -> String {
    let body = strip_plan_frontmatter(raw);
    if body.trim().is_empty() {
        return String::new();
    }
    format!("<details>\n<summary>{name}</summary>\n\n{body}\n\n</details>")
}

/// 📝️ Builds the issue comment body of a bound plan or spec.
pub fn format_plan_comment<S: TicketStore>(store: &S, source: &str) -> TicketResult<String> {
    let source = source.trim();
    if source.is_empty() {
        return Ok(String::new());
    }
    let Some(metadata) = store.metadata(source) else {
        return Err(TicketError::not_found(format!("plan source {source} does not exist")));
    };
    let mut sections: Vec<String> = Vec::new();
    if metadata.kind == NodeKind::Directory {
        let mut names: Vec<String> = store.entries(source)?.into_iter().filter(|entry| entry.kind == NodeKind::File && entry.name.ends_with(".md")).map(|entry| entry.name).collect();
        names.sort();
        for name in names {
            let section = format_plan_file_section(&name, &store.read(&join_path(source, &name))?);
            if !section.is_empty() {
                sections.push(section);
            }
        }
    } else {
        let section = format_plan_file_section(&base_name(source), &store.read(source)?);
        if !section.is_empty() {
            sections.push(section);
        }
    }
    if sections.is_empty() {
        return Ok(String::new());
    }
    Ok(format!("# 📋️ Plan\n\n{}", sections.join("\n\n")))
}

//#endregion 🗺️PlanSource
