//! 🔓️ The open, close, reopen and change lifecycle, the ticket service and the real-tree purge.

use super::*;

//#region 🔓️Lifecycle

/// 📬️ Everything an open request carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketOpenRequest {
    pub emoji: String,
    pub title: String,
    pub prompt: String,
    #[serde(default)]
    pub llm: String,
    #[serde(default)]
    pub effort: String,
    pub client: String,
    pub goal: String,
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub no_issue: bool,
    #[serde(default)]
    pub no_management: bool,
    #[serde(default)]
    pub issue: String,
    #[serde(default)]
    pub session: String,
    #[serde(default)]
    pub plan_id: String,
    #[serde(default)]
    pub spec_id: String,
    #[serde(default)]
    pub draft: String,
}

/// 📦️ Everything a close request carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketCloseRequest {
    pub id: String,
    pub summary: String,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub no_management: bool,
    #[serde(default)]
    pub bulk: bool,
}

/// 🎒️ Everything a reopen request carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketReopenRequest {
    pub id: String,
    pub prompt: String,
    #[serde(default)]
    pub llm: String,
    #[serde(default)]
    pub effort: String,
    pub client: String,
    #[serde(default)]
    pub goal: String,
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub no_management: bool,
    #[serde(default)]
    pub session: String,
    #[serde(default)]
    pub plan_id: String,
    #[serde(default)]
    pub spec_id: String,
    #[serde(default)]
    pub draft: String,
}

/// ♻️ Everything a change request carries. An absent member leaves the ticket's value alone.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketChangeRequest {
    pub id: String,
    #[serde(default)]
    pub llm: Option<String>,
    #[serde(default)]
    pub effort: Option<String>,
    #[serde(default)]
    pub client: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub goal: Option<String>,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub no_management: bool,
}

/// 🏁️ The outcome of one lifecycle operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketOutcome {
    pub id: String,
    pub rel_path: String,
    pub status: String,
    pub document: String,
    #[serde(default, skip_serializing_if = "TransactionJournal::is_empty")]
    pub journal: TransactionJournal,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

impl TransactionJournal {
    /// 🈳️ Whether nothing was recorded.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// 🧹️ What an artifact purge removed.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurgeReport {
    pub removed_directories: Vec<String>,
    pub removed_files: Vec<String>,
}

/// 📏️ A file above this size is purged from a closed ticket folder.
pub const OVERSIZED_FILE_BYTES: u64 = 5 << 20;
/// 🗃️ A subfolder whose files total above this size is purged from a closed ticket folder.
pub const OVERSIZED_FOLDER_BYTES: u64 = 10 << 20;

/// 🪪️ Request-scoped interaction facts, captured without mutating service or ambient repository state.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionMetadata { pub author: String, pub system: String, pub checkpoint: String }

/// 🎫️ The ticket domain bound to one store, one issue tracker, one clock and one event sink.
pub struct TicketService<'a, S: TicketStore, T: IssueTracker, C: Clock, E: EventSink> {
    pub layout: TicketLayout,
    pub store: &'a S,
    pub tracker: &'a T,
    pub clock: &'a C,
    pub events: &'a E,
    pub roots: PlanRoots,
    pub mcp_client: McpClientKind,
    pub interaction_metadata: InteractionMetadata,
}

impl<'a, S: TicketStore, T: IssueTracker, C: Clock, E: EventSink> TicketService<'a, S, T, C, E> {
    /// 🆕️ Binds the domain to its ports.
    pub fn new(layout: TicketLayout, store: &'a S, tracker: &'a T, clock: &'a C, events: &'a E) -> TicketService<'a, S, T, C, E> {
        TicketService { layout, store, tracker, clock, events, roots: PlanRoots { repo_root: String::new(), home_dir: String::new() }, mcp_client: McpClientKind::Generic, interaction_metadata: InteractionMetadata::default() }
    }

    /// 🗺️ Sets the roots plan resolution reads.
    pub fn with_roots(mut self, roots: PlanRoots) -> Self {
        self.roots = roots;
        self
    }

    /// 🔌️ Sets the MCP surface a plan or spec id is resolved against.
    pub fn with_mcp_client(mut self, mcp_client: McpClientKind) -> Self {
        self.mcp_client = mcp_client;
        self
    }

    /// 🛎️ Captures interaction metadata on this private service value.
    pub fn with_interaction_metadata(mut self, metadata: InteractionMetadata) -> Self { self.interaction_metadata = metadata; self }

    /// 📖️ Reads one ticket, filling in every derived path.
    pub fn read(&self, id: &TicketId) -> TicketResult<Ticket> {
        let directory = drafts::owned_ticket_path(&self.layout, id)?;
        drafts::existing_ancestry(self.store, &directory)?;
        let document_path = self.layout.document_path(id);
        if !self.store.exists(&document_path) {
            return Err(TicketError::not_found(format!("ticket not found: {document_path}")));
        }
        if !self.store.is_file(&document_path) { return Err(TicketError::invalid("physical ticket document required")); }
        let mut ticket = decode_ticket_document(&self.store.read(&document_path)?)?;
        self.hydrate(&mut ticket, id);
        Ok(ticket)
    }

    /// 💾️ Writes one ticket back, refusing a status the vocabulary does not carry.
    pub fn save(&self, ticket: &Ticket) -> TicketResult<()> {
        let id = TicketId::new(ticket.year, ticket.month, ticket.day, &ticket.slug);
        let directory = drafts::owned_ticket_path(&self.layout, &id)?;
        if ticket.json_path != self.layout.document_path(&id) { return Err(TicketError::invalid("ticket document must belong to the service layout")); }
        drafts::existing_ancestry(self.store, &directory)?;
        if self.store.exists(&ticket.json_path) && !self.store.is_file(&ticket.json_path) { return Err(TicketError::invalid("physical ticket document required")); }
        self.store.write(&ticket.json_path, &encode_ticket_document(ticket))
    }

    /// ▪️ Every ticket, optionally narrowed to a year, a month and a day.
    pub fn list(&self, year: Option<i64>, month: Option<i64>, day: Option<i64>) -> TicketResult<Vec<Ticket>> {
        let tickets_dir = self.layout.tickets_dir();
        if !self.store.exists(&tickets_dir) {
            return Ok(Vec::new());
        }
        let mut tickets: Vec<Ticket> = Vec::new();
        for year_value in self.dated_children(&tickets_dir, EMOJI_YEAR, year) {
            let year_path = join_path(&tickets_dir, &format_year_dir(year_value));
            for month_value in self.dated_children(&year_path, EMOJI_MONTH, month) {
                let month_path = join_path(&year_path, &format_month_dir(month_value));
                for day_value in self.dated_children(&month_path, EMOJI_DAY, day) {
                    let day_path = join_path(&month_path, &format_day_dir(day_value));
                    for slug in self.slugs_under(&day_path, "") {
                        let id = TicketId::new(year_value, month_value, day_value, slug);
                        if let Ok(ticket) = self.read(&id) {
                            tickets.push(ticket);
                        }
                    }
                }
            }
        }
        Ok(tickets)
    }

    /// 🔎️ The most recently created ticket whose slug matches, searched newest first.
    pub fn find_by_slug(&self, slug: &str) -> TicketResult<Ticket> {
        let tickets = self.list(None, None, None)?;
        for ticket in tickets.iter().rev() {
            if ticket.slug == slug || base_name(&ticket.slug) == slug {
                return Ok(ticket.clone());
            }
        }
        Err(TicketError::not_found(format!("ticket not found: {slug}")))
    }

    /// 🕰️ The newest ticket by date, then by slug.
    pub fn latest(&self) -> TicketResult<Ticket> {
        let mut tickets = self.list(None, None, None)?;
        if tickets.is_empty() {
            return Err(TicketError::not_found("no tickets found"));
        }
        tickets.sort_by(|left, right| (right.year, right.month, right.day, &right.slug).cmp(&(left.year, left.month, left.day, &left.slug)));
        Ok(tickets.remove(0))
    }

    /// 🔍️ Every ticket a query keeps, in listing order.
    pub fn search(&self, query: &TicketQuery) -> TicketResult<Vec<Ticket>> {
        Ok(self.list(None, None, None)?.into_iter().filter(|ticket| query.matches(ticket)).collect())
    }

    /// 📂️ Opens a ticket: validates, materialises the folder, attaches a plan, syncs the issue and emits.
    pub fn open(&self, request: &TicketOpenRequest) -> TicketResult<TicketOutcome> {
        if request.goal.trim().is_empty() {
            return Err(TicketError::invalid("ticket goal is required"));
        }
        let mut slug = validate_ticket_emoji_title(&request.emoji, &request.title)?;
        let llm = resolve_optional(&request.llm, semio_framework_repo_model::resolve_allowed_llm)?;
        let effort = resolve_optional(&request.effort, semio_framework_repo_model::resolve_allowed_effort)?;
        let client = semio_framework_repo_model::resolve_allowed_client(&request.client).map_err(|error| TicketError::invalid(error.to_string()))?;

        let (mut year, mut month, mut day) = self.clock.today();
        if !request.parent.trim().is_empty() {
            let parent = self.find_by_slug(request.parent.trim())?;
            year = parent.year;
            month = parent.month;
            day = parent.day;
            slug = format!("{}/{slug}", parent.slug);
        }
        let id = TicketId::new(year, month, day, slug);
        let ticket_dir = drafts::owned_ticket_path(&self.layout, &id)?;
        drafts::existing_ancestry(self.store, &ticket_dir)?;
        if self.store.exists(&ticket_dir) {
            return Err(TicketError::conflict(format!("ticket folder already exists: {ticket_dir}")));
        }
        let draft = drafts::prepare(self.store, &self.layout, &id, &request.draft)?;
        self.store.create_directory(&ticket_dir)?;
        let mut journal = TransactionJournal::default();
        self.store.write(&self.layout.important_path(&id), "")?;
        journal.record("create-important", &self.layout.important_path(&id), "ok");

        let mut ticket = Ticket {
            year,
            month,
            day,
            slug: id.slug.clone(),
            title: request.title.trim().to_string(),
            emoji: request.emoji.trim().to_string(),
            status: TicketStatus::Open,
            description: request.prompt.clone(),
            summary: String::new(),
            management: None,
            goal: request.goal.clone(),
            parent: request.parent.clone(),
            plan: None,
            sessions: Vec::new(),
            interactions: vec![Interaction {
                kind: "ticket.open".to_string(),
                date: self.clock.stamp(),
                author: self.interaction_metadata.author.clone(),
                system: self.interaction_metadata.system.clone(),
                client: client.clone(),
                checkpoint: self.interaction_metadata.checkpoint.clone(),
                prompt: request.prompt.clone(),
                summary: String::new(),
                llm: llm.clone(),
                effort: effort.clone(),
                files: Vec::new(),
            }],
            agents: Vec::new(),
            folder_path: ticket_dir,
            json_path: self.layout.document_path(&id),
            important_path: self.layout.important_path(&id),
        };
        append_session_id(&mut ticket, &request.session);
        apply_ticket_plan_from_ids(self.store, &self.roots, &mut ticket, self.mcp_client, &request.plan_id, &request.spec_id)?;

        drafts::attach(self.store, draft)?;
        let mut warnings: Vec<String> = Vec::new();
        if !request.no_issue && !request.no_management {
            let milestone = milestone_number_for_title(self.tracker, &request.goal);
            let issue_title = ticket.title.clone();
            warnings.extend(ensure_ticket_issue(self.tracker, &mut ticket, &issue_title, &request.prompt, &request.issue, milestone, false));
        }
        self.save(&ticket)?;
        self.emit_ticket_event(
            semio_framework_repo_events::TICKET_OPEN_ENDED,
            &id,
            serde_json::json!({ "title": ticket.title, "prompt": request.prompt, "llm": llm, "effort": effort, "client": client, "goal": request.goal, "parent": request.parent }),
        );
        Ok(self.outcome(&id, &ticket, journal, warnings))
    }

    /// 📪️ Closes a ticket: consumes the important document, moves the plan in, syncs and emits.
    pub fn close(&self, request: &TicketCloseRequest) -> TicketResult<TicketOutcome> {
        let id = TicketId::parse(&request.id)?;
        let mut ticket = self.read(&id)?;
        if ticket.status != TicketStatus::Open {
            return Err(TicketError::conflict("ticket is not open"));
        }
        let files = normalize_ticket_file_inputs(&request.files);
        let mut summary = request.summary.clone();
        if request.bulk {
            if summary.is_empty() {
                summary = "Bulk close".to_string();
            }
        } else {
            if summary.is_empty() {
                return Err(TicketError::invalid("summary is required to finish a ticket"));
            }
            if files.is_empty() {
                return Err(TicketError::invalid("at least one file is required to finish a ticket"));
            }
        }
        let mut journal = TransactionJournal::default();
        let preimage = inspect_important_document(self.store, &ticket.important_path.clone(), &mut journal)?;
        move_ticket_plan_into_folder(self.store, &mut ticket)?;

        let mut warnings: Vec<String> = Vec::new();
        if !request.no_management {
            warnings.extend(close_ticket_issue(self.tracker, &ticket, &summary, &[], request.bulk));
        }
        remove_important_document(self.store, &preimage, &mut journal)?;

        let previous = ticket.clone();
        ticket.summary = summary.clone();
        ticket.status = TicketStatus::Closed;
        ticket.interactions.push(Interaction {
            kind: "ticket.close".to_string(),
            date: self.clock.stamp(),
            author: self.interaction_metadata.author.clone(),
            system: self.interaction_metadata.system.clone(),
            client: previous.interactions.last().map(|interaction| interaction.client.clone()).unwrap_or_default(),
            checkpoint: self.interaction_metadata.checkpoint.clone(),
            prompt: String::new(),
            summary: summary.clone(),
            llm: String::new(),
            effort: String::new(),
            files: files.iter().map(|path| InteractionFile { path: path.clone(), id: String::new(), uri: String::new() }).collect(),
        });
        if let Err(error) = self.save(&ticket) {
            journal.record("save", &ticket.json_path, "failed");
            let restore = restore_important_document(self.store, &preimage, &mut journal);
            ticket = previous;
            let _ = ticket;
            return match restore {
                Ok(()) => Err(error),
                Err(restore) => Err(TicketError::store(format!("save ticket: {error}; restore important document: {restore}"))),
            };
        }
        journal.record("save", &ticket.json_path, "ok");
        self.emit_ticket_event(semio_framework_repo_events::TICKET_CLOSE_ENDED, &id, serde_json::json!({ "summary": summary, "files": files }));
        Ok(self.outcome(&id, &ticket, journal, warnings))
    }

    /// ❗️ Reopens a closed ticket, creating the important document transactionally.
    pub fn reopen(&self, request: &TicketReopenRequest) -> TicketResult<TicketOutcome> {
        let id = TicketId::parse(&request.id)?;
        let mut ticket = self.read(&id)?;
        if ticket.status != TicketStatus::Closed {
            return Err(TicketError::conflict("ticket is already open"));
        }
        if request.client.trim().is_empty() {
            return Err(TicketError::invalid("client is required"));
        }
        let llm = resolve_optional(&request.llm, semio_framework_repo_model::resolve_allowed_llm)?;
        let effort = resolve_optional(&request.effort, semio_framework_repo_model::resolve_allowed_effort)?;
        let client = semio_framework_repo_model::resolve_allowed_client(&request.client).map_err(|error| TicketError::invalid(error.to_string()))?;
        if !request.goal.trim().is_empty() {
            ticket.goal = request.goal.trim().to_string();
        }
        if !request.parent.trim().is_empty() {
            ticket.parent = request.parent.trim().to_string();
        }
        apply_ticket_plan_from_ids(self.store, &self.roots, &mut ticket, self.mcp_client, &request.plan_id, &request.spec_id)?;

        let mut journal = TransactionJournal::default();
        let draft = drafts::prepare(self.store, &self.layout, &id, &request.draft)?;
        let creation = ensure_important_document(self.store, &ticket.important_path.clone(), &mut journal)?;

        let previous = ticket.clone();
        ticket.status = TicketStatus::Open;
        ticket.interactions.push(Interaction {
            kind: "ticket.reopen".to_string(),
            date: self.clock.stamp(),
            author: self.interaction_metadata.author.clone(),
            system: self.interaction_metadata.system.clone(),
            client: client.clone(),
            checkpoint: self.interaction_metadata.checkpoint.clone(),
            prompt: request.prompt.clone(),
            summary: String::new(),
            llm: llm.clone(),
            effort: effort.clone(),
            files: Vec::new(),
        });
        append_session_id(&mut ticket, &request.session);

        drafts::attach(self.store, draft)?;
        let mut warnings: Vec<String> = Vec::new();
        if !request.no_management {
            warnings.extend(ensure_ticket_issue(self.tracker, &mut ticket, &previous.title, &request.prompt, "", None, true));
            if let Some(management) = ticket.management.as_ref().filter(|management| !management.issue.is_empty()) {
                if let Err(error) = self.tracker.add_comment(&management.issue, &format_prompt_heading(&request.prompt)) {
                    warnings.push(format!("Failed to add prompt comment to GitHub issue: {}", error.message));
                }
            }
        }
        if let Err(error) = self.save(&ticket) {
            journal.record("save", &ticket.json_path, "failed");
            let rollback = rollback_important_creation(self.store, &creation, &mut journal);
            return match rollback {
                Ok(()) => Err(error),
                Err(rollback) => Err(TicketError::store(format!("save ticket: {error}; rollback important document: {rollback}"))),
            };
        }
        journal.record("save", &ticket.json_path, "ok");
        self.emit_ticket_event(semio_framework_repo_events::TICKET_REOPEN_ENDED, &id, serde_json::json!({ "prompt": request.prompt, "llm": llm, "effort": effort, "client": client }));
        Ok(self.outcome(&id, &ticket, journal, warnings))
    }

    /// 🔁️ Changes a ticket in place, renaming its folder when the title changes its slug.
    pub fn change(&self, request: &TicketChangeRequest) -> TicketResult<TicketOutcome> {
        let id = TicketId::parse(&request.id)?;
        let llm = request.llm.as_ref().map(|value| resolve_optional(value, semio_framework_repo_model::resolve_allowed_llm)).transpose()?;
        let effort = request.effort.as_ref().map(|value| resolve_optional(value, semio_framework_repo_model::resolve_allowed_effort)).transpose()?;
        let client = request.client.as_ref().map(|value| semio_framework_repo_model::resolve_allowed_client(value).map_err(|error| TicketError::invalid(error.to_string()))).transpose()?;
        let mut ticket = self.read(&id)?;
        let mut journal = TransactionJournal::default();
        let mut next_id = id.clone();
        if let Some(title) = request.title.as_ref() {
            let title = title.trim();
            if title.is_empty() { return Err(TicketError::invalid("ticket title is required")); }
            let mut slug = ticket_slug_from_title(title)?;
            if let Some(parent) = id.parent_slug() { slug = format!("{parent}/{slug}"); }
            next_id = TicketId::new(id.year, id.month, id.day, slug);
            if next_id.slug != id.slug {
                let target = drafts::owned_ticket_path(&self.layout, &next_id)?;
                drafts::existing_ancestry(self.store, &target)?;
                if self.store.exists(&target) { return Err(TicketError::conflict(format!("ticket folder already exists: {target}"))); }
                let source = drafts::owned_ticket_path(&self.layout, &id)?;
                drafts::projection(self.store, &source, &target)?;
                self.store.rename(&source, &target)?;
                journal.record("rename", &target, "ok");
            }
            ticket.title = title.to_string();
        }
        if let Some(description) = request.description.as_ref() { ticket.description = description.clone(); }
        if let Some(goal) = request.goal.as_ref() { ticket.goal = goal.clone(); }
        if let Some(parent) = request.parent.as_ref() { ticket.parent = parent.clone(); }
        if let Some(interaction) = ticket.interactions.last_mut() {
            if let Some(description) = request.description.as_ref() { interaction.prompt = description.clone(); }
            if let Some(value) = llm { interaction.llm = value; }
            if let Some(value) = effort { interaction.effort = value; }
            if let Some(value) = client { interaction.client = value; }
        }
        self.hydrate(&mut ticket, &next_id); self.save(&ticket)?;
        journal.record("save", &ticket.json_path, "ok");
        let mut warnings = Vec::new();
        if request.title.is_some() && !request.no_management {
            if let Some(management) = ticket.management.as_ref().filter(|value| !value.issue.is_empty()) {
                if let Err(error) = self.tracker.update_issue_title(&management.issue, &ticket.title) { warnings.push(format!("Failed to update issue title: {}", error.message)); }
            }
        }
        self.emit_ticket_event(semio_framework_repo_events::TICKET_CHANGE_ENDED, &next_id, serde_json::json!({ "title": ticket.title, "goal": ticket.goal }));
        Ok(self.outcome(&next_id, &ticket, journal, warnings))
    }

    /// 🗑️ Deletes the remote issue before removing this exact owned ticket subtree.
    pub fn delete(&self, id: &TicketId, no_management: bool) -> TicketResult<()> {
        let directory = drafts::owned_ticket_path(&self.layout, id)?;
        drafts::existing_ancestry(self.store, &directory)?;
        let ticket = self.read(id)?;
        if !self.store.is_directory(&directory) { return Err(TicketError::invalid("physical ticket directory required")); }
        if !no_management {
            if let Some(management) = ticket.management.as_ref().filter(|value| !value.issue.is_empty()) { self.tracker.delete_issue(&management.issue).map_err(|error| TicketError::store(error.message))?; }
        }
        self.store.remove_tree(&directory)
    }

    /// 🧽️ Deletes oversized artifacts from one ticket folder, deepest folder first.
    pub fn purge_artifacts(&self, id: &TicketId) -> TicketResult<PurgeReport> {
        let root = self.layout.ticket_dir(id);
        let mut report = PurgeReport::default();
        if !self.store.is_directory(&root) {
            return Ok(report);
        }
        let mut directories: Vec<String> = Vec::new();
        let mut files: Vec<(String, u64)> = Vec::new();
        self.collect_tree(&root, &mut directories, &mut files);
        let mut directory_sizes: BTreeMap<String, u64> = BTreeMap::new();
        for (path, size) in &files {
            let mut parent = dir_name(path);
            while parent.starts_with(&root) {
                *directory_sizes.entry(parent.clone()).or_default() += size;
                if parent == root {
                    break;
                }
                parent = dir_name(&parent);
            }
        }
        directories.sort_by_key(|path| std::cmp::Reverse(path.matches('/').count()));
        let mut deleted: Vec<String> = Vec::new();
        for directory in directories {
            if deleted.iter().any(|removed| directory.starts_with(&format!("{removed}/"))) {
                continue;
            }
            if directory_sizes.get(&directory).copied().unwrap_or_default() <= OVERSIZED_FOLDER_BYTES {
                continue;
            }
            self.store.remove_tree(&directory)?;
            report.removed_directories.push(directory.clone());
            deleted.push(directory);
        }
        for (path, size) in files {
            if deleted.iter().any(|removed| path == *removed || path.starts_with(&format!("{removed}/"))) {
                continue;
            }
            if base_name(&path) == TICKET_DOCUMENT_NAME || size <= OVERSIZED_FILE_BYTES {
                continue;
            }
            self.store.remove_file(&path)?;
            report.removed_files.push(path);
        }
        report.removed_directories.sort();
        report.removed_files.sort();
        Ok(report)
    }

    fn collect_tree(&self, root: &str, directories: &mut Vec<String>, files: &mut Vec<(String, u64)>) {
        let Ok(entries) = self.store.entries(root) else { return };
        for entry in entries {
            let path = join_path(root, &entry.name);
            match entry.kind {
                NodeKind::Directory => {
                    directories.push(path.clone());
                    self.collect_tree(&path, directories, files);
                }
                NodeKind::File => files.push((path.clone(), self.store.metadata(&path).map(|metadata| metadata.size).unwrap_or_default())),
                NodeKind::Symlink => {}
            }
        }
    }

    fn hydrate(&self, ticket: &mut Ticket, id: &TicketId) {
        ticket.year = id.year;
        ticket.month = id.month;
        ticket.day = id.day;
        ticket.slug = id.slug.clone();
        ticket.folder_path = self.layout.ticket_dir(id);
        ticket.json_path = self.layout.document_path(id);
        ticket.important_path = self.layout.important_path(id);
    }

    pub(super) fn outcome(&self, id: &TicketId, ticket: &Ticket, journal: TransactionJournal, warnings: Vec<String>) -> TicketOutcome {
        TicketOutcome { id: id.id(), rel_path: id.rel_path(), status: ticket.status.as_str().to_string(), document: encode_ticket_document(ticket), journal, warnings }
    }

    fn emit_ticket_event(&self, kind: &str, id: &TicketId, extra: Value) {
        let mut payload = serde_json::json!({ "id": id.rel_path(), "year": id.year, "month": id.month, "day": id.day, "slug": id.slug });
        if !self.interaction_metadata.author.is_empty() { payload["author"] = Value::String(self.interaction_metadata.author.clone()); }
        if let (Some(target), Value::Object(members)) = (payload.as_object_mut(), extra) {
            for (name, value) in members {
                target.insert(name, value);
            }
        }
        self.events.emit(kind, EVENT_SOURCE, &payload.to_string());
    }

    fn dated_children(&self, path: &str, prefix: &str, pinned: Option<i64>) -> Vec<i64> {
        if let Some(value) = pinned {
            let candidate = join_path(path, &format!("{prefix}{}", pad_number(value, 2)));
            return if self.store.exists(&candidate) { vec![value] } else { Vec::new() };
        }
        let Ok(entries) = self.store.entries(path) else { return Vec::new() };
        let mut values: Vec<i64> = entries.into_iter().filter(|entry| entry.kind == NodeKind::Directory).filter_map(|entry| parse_dated_dir(&entry.name, prefix).ok()).collect();
        values.sort_unstable();
        values
    }

    fn slugs_under(&self, day_path: &str, prefix: &str) -> Vec<String> {
        let root = if prefix.is_empty() { day_path.to_string() } else { join_path(day_path, prefix) };
        let Ok(entries) = self.store.entries(&root) else { return Vec::new() };
        let mut slugs: Vec<String> = Vec::new();
        for entry in entries {
            if entry.kind != NodeKind::Directory || entry.name.starts_with('.') || SKIPPED_WALK_DIRS.contains(&entry.name.as_str()) {
                continue;
            }
            let slug = if prefix.is_empty() { entry.name.clone() } else { format!("{prefix}/{}", entry.name) };
            if self.store.exists(&join_path(&join_path(day_path, &slug), TICKET_DOCUMENT_NAME)) {
                slugs.push(slug);
            } else {
                slugs.extend(self.slugs_under(day_path, &slug));
            }
        }
        slugs
    }
}

fn resolve_optional(value: &str, resolve: fn(&str) -> Result<String, semio_framework_repo_model::ModelError>) -> TicketResult<String> {
    if value.trim().is_empty() {
        return Ok(String::new());
    }
    resolve(value).map_err(|error| TicketError::invalid(error.to_string()))
}

/// 📥️ Reads an open request from JSON text. Concrete on purpose: no external type leaves this crate.
pub fn parse_open_request(text: &str) -> TicketResult<TicketOpenRequest> {
    serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))
}

/// 🔣️ Reads a close request from JSON text.
pub fn parse_close_request(text: &str) -> TicketResult<TicketCloseRequest> {
    serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))
}

/// 📃️ Reads a reopen request from JSON text.
pub fn parse_reopen_request(text: &str) -> TicketResult<TicketReopenRequest> {
    serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))
}

/// 💬️ Reads a change request from JSON text.
pub fn parse_change_request(text: &str) -> TicketResult<TicketChangeRequest> {
    serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))
}

/// 📤️ Renders an outcome as JSON text.
pub fn encode_outcome(outcome: &TicketOutcome) -> String {
    serde_json::to_string(outcome).unwrap_or_default()
}

//#endregion 🔓️Lifecycle

//#region 🧹️RealPurge

/// 🌳️ Every slug directory under a dated `🎫️tickets` tree, in `YY/MM/DD/SLUG` walk order.
///
/// Twin of `collectTicketFolderRoots` in `📦️packages/🐹️go/🐹️.go`: four levels deep, directories
/// only, dot-prefixed slugs skipped, an unreadable level silently contributing nothing.
pub fn ticket_folder_roots(tickets_dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut roots = Vec::new();
    for year in sorted_directories(tickets_dir) {
        for month in sorted_directories(&year) {
            for day in sorted_directories(&month) {
                for slug in sorted_directories(&day) {
                    if slug.file_name().is_none_or(|name| name.to_string_lossy().starts_with('.')) {
                        continue;
                    }
                    roots.push(slug);
                }
            }
        }
    }
    roots
}

/// 📁️ The child directories of a path, in name order; an unreadable path contributes none.
fn sorted_directories(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    let Ok(entries) = std::fs::read_dir(path) else { return Vec::new() };
    let mut found: Vec<std::path::PathBuf> = entries.flatten().filter(|entry| entry.path().is_dir()).map(|entry| entry.path()).collect();
    found.sort();
    found
}

/// 🧼️ Deletes the oversized artifacts of one real ticket folder, deepest subfolder first.
///
/// Twin of `PurgeOversizedTicketArtifacts` in `📦️packages/🐹️go/🐹️.go`. A missing folder is a
/// no-operation, symlinks are never followed, `🎫️ticket.json` is never removed and a file inside
/// an already-deleted subtree is not visited again.
pub fn purge_oversized_artifacts(ticket_dir: &std::path::Path) -> Result<PurgeReport, String> {
    let mut report = PurgeReport::default();
    let Ok(metadata) = std::fs::symlink_metadata(ticket_dir) else { return Ok(report) };
    if !metadata.is_dir() {
        return Err(format!("ticket path is not a directory: {}", ticket_dir.display()));
    }
    let mut directories: Vec<std::path::PathBuf> = Vec::new();
    let mut files: Vec<(std::path::PathBuf, u64)> = Vec::new();
    collect_real_tree(ticket_dir, &mut directories, &mut files);
    let mut directory_sizes: BTreeMap<std::path::PathBuf, u64> = BTreeMap::new();
    for (path, size) in &files {
        let mut parent = path.parent().map(std::path::Path::to_path_buf);
        while let Some(current) = parent {
            if !current.starts_with(ticket_dir) {
                break;
            }
            *directory_sizes.entry(current.clone()).or_default() += size;
            if current == ticket_dir {
                break;
            }
            parent = current.parent().map(std::path::Path::to_path_buf);
        }
    }
    directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    let mut deleted: Vec<std::path::PathBuf> = Vec::new();
    for directory in directories {
        if deleted.iter().any(|removed| directory.starts_with(removed)) {
            continue;
        }
        if directory_sizes.get(&directory).copied().unwrap_or_default() <= OVERSIZED_FOLDER_BYTES {
            continue;
        }
        std::fs::remove_dir_all(&directory).map_err(|error| format!("delete oversized ticket folder {}: {error}", directory.display()))?;
        report.removed_directories.push(directory.to_string_lossy().replace('\\', "/"));
        deleted.push(directory);
    }
    for (path, size) in files {
        if deleted.iter().any(|removed| path.starts_with(removed)) {
            continue;
        }
        if path.file_name().is_some_and(|name| name == std::ffi::OsStr::new(TICKET_DOCUMENT_NAME)) || size <= OVERSIZED_FILE_BYTES {
            continue;
        }
        std::fs::remove_file(&path).map_err(|error| format!("delete oversized ticket file {}: {error}", path.display()))?;
        report.removed_files.push(path.to_string_lossy().replace('\\', "/"));
    }
    report.removed_directories.sort();
    report.removed_files.sort();
    Ok(report)
}

/// 🚶️ Every directory below `root` and every regular file with its size, symlinks skipped.
fn collect_real_tree(root: &std::path::Path, directories: &mut Vec<std::path::PathBuf>, files: &mut Vec<(std::path::PathBuf, u64)>) {
    let Ok(entries) = std::fs::read_dir(root) else { return };
    let mut children: Vec<std::path::PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
    children.sort();
    for child in children {
        let Ok(metadata) = std::fs::symlink_metadata(&child) else { continue };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            directories.push(child.clone());
            collect_real_tree(&child, directories, files);
        } else {
            files.push((child, metadata.len()));
        }
    }
}

//#endregion 🧹️RealPurge
