//! 🔓️ The open, close, reopen and change lifecycle, the ticket service and the generated-output purge.

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

//#region 🧹️PurgeGenerated

/// 🗑️ The one folder of a ticket whose content a purge may remove.
pub const TICKET_GENERATED_DIRECTORY: &str = "🗑️generated";
/// 📏️ A generated file above this size is purged.
pub const OVERSIZED_FILE_BYTES: u64 = 5 << 20;
/// 🗃️ A generated folder whose files total above this size is purged.
pub const OVERSIZED_FOLDER_BYTES: u64 = 10 << 20;
/// ⏱️ How recently a generated file may have been written before its ticket is left alone.
pub const LIVE_WRITER_WINDOW: std::time::Duration = std::time::Duration::from_secs(600);

/// 🎚️ Which part of an eligible generated folder is removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurgeSelection {
    /// 📏️ Files above 5 MiB and folders above 10 MiB.
    Oversized,
    /// 🧽️ Everything below the generated folder.
    Generated,
}

/// 🗑️ One planned removal: a path below the tickets folder, its kind and the bytes it frees.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurgeRemoval {
    pub path: String,
    pub kind: String,
    pub bytes: u64,
}

/// ⏭️ Why a whole ticket is left alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurgeSkip {
    pub ticket: String,
    pub reason: String,
}

/// 🧹️ What a purge removes (applied) or would remove (dry run), sorted by path and ticket.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurgePlan {
    pub removals: Vec<PurgeRemoval>,
    pub skips: Vec<PurgeSkip>,
}

/// 🔎️ Answers which files below a repository-relative folder the git index tracks.
pub trait TrackedFiles {
    /// 🗂️ Repository-relative, slash-separated paths of the tracked files below `folder`; an error means "unknown".
    fn tracked(&self, root: &std::path::Path, folder: &str) -> Result<BTreeSet<String>, String>;
}

/// 🌿️ The read-only `git ls-files` implementation of [`TrackedFiles`], which cannot climb above the root.
pub struct GitTrackedFiles;

impl TrackedFiles for GitTrackedFiles {
    fn tracked(&self, root: &std::path::Path, folder: &str) -> Result<BTreeSet<String>, String> {
        let ceiling = root.parent().map(std::path::Path::to_path_buf).unwrap_or_default();
        let output = std::process::Command::new("git")
            .args(["-c", "core.quotepath=off", "ls-files", "-z", "--cached", "--", folder])
            .current_dir(root)
            .env_clear()
            .envs(std::env::vars_os().filter(|(name, _)| !name.to_string_lossy().to_ascii_uppercase().starts_with("GIT_")))
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("GIT_CEILING_DIRECTORIES", ceiling)
            .output()
            .map_err(|error| format!("git ls-files in {}: {error}", root.display()))?;
        if !output.status.success() {
            return Err(format!("git ls-files in {} exited with {}", root.display(), output.status));
        }
        Ok(String::from_utf8_lossy(&output.stdout).split('\0').filter(|entry| !entry.is_empty()).map(str::to_string).collect())
    }
}

/// 🧭️ The whole input of a purge; without `apply` nothing on disk changes.
pub struct PurgeOptions<'a> {
    pub root: std::path::PathBuf,
    pub tickets_dir: std::path::PathBuf,
    pub selection: PurgeSelection,
    pub apply: bool,
    pub now: std::time::SystemTime,
    pub tracked: &'a dyn TrackedFiles,
}

/// 📸️ The shape of a generated folder at one moment: an apply that finds it changed was racing a writer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct GeneratedSnapshot {
    files: usize,
    bytes: u64,
    newest: Option<std::time::SystemTime>,
}

/// 🚶️ The real directories and regular files below `root` (links are never followed) and their snapshot.
fn walk_generated(root: &std::path::Path) -> Result<(Vec<std::path::PathBuf>, Vec<(std::path::PathBuf, u64)>, GeneratedSnapshot), String> {
    let mut directories = vec![root.to_path_buf()];
    let mut files = Vec::new();
    let mut snapshot = GeneratedSnapshot::default();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let entries = std::fs::read_dir(&directory).map_err(|error| format!("read {}: {error}", directory.display()))?;
        for entry in entries {
            let path = entry.map_err(|error| format!("read {}: {error}", directory.display()))?.path();
            let metadata = match std::fs::symlink_metadata(&path) {
                Ok(metadata) => metadata,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(format!("stat {}: {error}", path.display())),
            };
            if metadata.file_type().is_symlink() {
                continue;
            }
            if metadata.is_dir() {
                directories.push(path.clone());
                pending.push(path);
            } else if metadata.is_file() {
                snapshot.files += 1;
                snapshot.bytes += metadata.len();
                let modified = metadata.modified().map_err(|error| format!("modified time of {}: {error}", path.display()))?;
                snapshot.newest = Some(snapshot.newest.map_or(modified, |newest| newest.max(modified)));
                files.push((path, metadata.len()));
            }
        }
    }
    Ok((directories, files, snapshot))
}

/// 🧭️ A slash-separated path of `path` below `base`, or the whole path where it lies elsewhere.
fn slash_relative(base: &std::path::Path, path: &std::path::Path) -> String {
    let relative = path.strip_prefix(base).unwrap_or(path);
    relative.components().map(|component| component.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/")
}

/// 🔓️ Whether the ticket document says exactly status closed.
fn ticket_manifest_is_closed(ticket_dir: &std::path::Path) -> bool {
    let Ok(text) = std::fs::read_to_string(ticket_dir.join(TICKET_DOCUMENT_NAME)) else { return false };
    serde_json::from_str::<Value>(&text).ok().and_then(|document| document.get("status").and_then(Value::as_str).map(|status| status == "closed")).unwrap_or(false)
}

/// 🧮️ What to remove below one generated folder, or why the ticket is left whole.
fn plan_generated_output(options: &PurgeOptions<'_>, generated: &std::path::Path) -> Result<Result<(Vec<PurgeRemoval>, GeneratedSnapshot), &'static str>, String> {
    let Ok(tracked) = options.tracked.tracked(&options.root, &slash_relative(&options.root, generated)) else { return Ok(Err("tracked-unknown")) };
    let (mut directories, files, snapshot) = walk_generated(generated)?;
    if snapshot.newest.is_some_and(|newest| options.now.duration_since(newest).map_or(true, |age| age < LIVE_WRITER_WINDOW)) {
        return Ok(Err("live-writer"));
    }
    let everything = options.selection == PurgeSelection::Generated;
    let mut bytes_below: BTreeMap<std::path::PathBuf, u64> = BTreeMap::new();
    let mut tracked_below: BTreeSet<std::path::PathBuf> = BTreeSet::new();
    let mut tracked_file: BTreeSet<std::path::PathBuf> = BTreeSet::new();
    for (path, size) in &files {
        let is_tracked = tracked.contains(&slash_relative(&options.root, path));
        if is_tracked {
            tracked_file.insert(path.clone());
        }
        let mut parent = path.parent();
        while let Some(directory) = parent {
            *bytes_below.entry(directory.to_path_buf()).or_default() += size;
            if is_tracked {
                tracked_below.insert(directory.to_path_buf());
            }
            if directory == generated {
                break;
            }
            parent = directory.parent();
        }
    }
    directories.sort_by_key(|path| (path.components().count(), path.clone()));
    let mut removed: Vec<std::path::PathBuf> = Vec::new();
    let mut removals = Vec::new();
    for directory in directories {
        if (directory == generated && !everything) || removed.iter().any(|path| directory.starts_with(path)) || tracked_below.contains(&directory) {
            continue;
        }
        let bytes = bytes_below.get(&directory).copied().unwrap_or_default();
        if everything || bytes > OVERSIZED_FOLDER_BYTES {
            removals.push(PurgeRemoval { path: slash_relative(&options.tickets_dir, &directory), kind: "directory".to_string(), bytes });
            removed.push(directory);
        }
    }
    for (path, size) in files {
        if removed.iter().any(|directory| path.starts_with(directory)) || tracked_file.contains(&path) {
            continue;
        }
        if everything || size > OVERSIZED_FILE_BYTES {
            removals.push(PurgeRemoval { path: slash_relative(&options.tickets_dir, &path), kind: "file".to_string(), bytes: size });
        }
    }
    Ok(Ok((removals, snapshot)))
}

/// 🧹️ Plans, and with `options.apply` performs, the removal of generated output of the given ticket folders.
///
/// Only the direct `🗑️generated` folder of a CLOSED ticket is eligible; tracked files, links, files
/// outside it and folders with a recent writer are never touched. Twin of
/// `PurgeTicketGeneratedOutput` in `📦️packages/🐹️go/🔓️finish-reopen/🐹️.go`.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json (PurgePolicy)
pub fn purge_ticket_generated_output(options: &PurgeOptions<'_>, ticket_dirs: &[std::path::PathBuf]) -> Result<PurgePlan, String> {
    let resolved = |path: &std::path::Path| std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let options = PurgeOptions { root: resolved(&options.root), tickets_dir: resolved(&options.tickets_dir), selection: options.selection, apply: options.apply, now: options.now, tracked: options.tracked };
    let mut plan = PurgePlan::default();
    let mut failures: Vec<String> = Vec::new();
    for ticket_dir in ticket_dirs {
        let name = ticket_dir.file_name().map(std::ffi::OsStr::to_os_string).unwrap_or_default();
        let ticket_dir = resolved(ticket_dir.parent().unwrap_or_else(|| std::path::Path::new("."))).join(name);
        let ticket = slash_relative(&options.tickets_dir, &ticket_dir);
        let mut skip = |reason: &str| plan.skips.push(PurgeSkip { ticket: ticket.clone(), reason: reason.to_string() });
        if !std::fs::symlink_metadata(&ticket_dir).is_ok_and(|metadata| metadata.is_dir()) {
            skip("not-directory");
            continue;
        }
        if !ticket_manifest_is_closed(&ticket_dir) {
            skip("not-closed");
            continue;
        }
        let generated = ticket_dir.join(TICKET_GENERATED_DIRECTORY);
        let metadata = match std::fs::symlink_metadata(&generated) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                failures.push(format!("stat {}: {error}", generated.display()));
                continue;
            }
        };
        if metadata.file_type().is_symlink() {
            skip("symlink-root");
            continue;
        }
        if !metadata.is_dir() {
            skip("not-directory");
            continue;
        }
        let (removals, snapshot) = match plan_generated_output(&options, &generated) {
            Ok(Ok(planned)) => planned,
            Ok(Err(reason)) => {
                skip(reason);
                continue;
            }
            Err(error) => {
                failures.push(error);
                continue;
            }
        };
        if options.apply {
            if walk_generated(&generated).map_or(true, |(_, _, current)| current != snapshot) {
                skip("live-writer");
                continue;
            }
            for removal in &removals {
                let path = options.tickets_dir.join(&removal.path);
                if !path.starts_with(&generated) {
                    failures.push(format!("refusing to remove {} outside {}", path.display(), generated.display()));
                    continue;
                }
                let outcome = if removal.kind == "directory" { std::fs::remove_dir_all(&path) } else { std::fs::remove_file(&path) };
                if let Err(error) = outcome {
                    failures.push(format!("remove {}: {error}", path.display()));
                }
            }
        }
        plan.removals.extend(removals);
    }
    plan.removals.sort_by(|left, right| left.path.cmp(&right.path));
    plan.skips.sort_by(|left, right| left.ticket.cmp(&right.ticket));
    if failures.is_empty() { Ok(plan) } else { Err(failures.join("; ")) }
}

/// 🧹️ Purges every closed ticket below `options.tickets_dir`.
pub fn purge_closed_ticket_generated_output(options: &PurgeOptions<'_>) -> Result<PurgePlan, String> {
    purge_ticket_generated_output(options, &ticket_folder_roots(&options.tickets_dir))
}

/// 🌳️ Every slug directory under a dated `🎫️tickets` tree, in `YY/MM/DD/SLUG` walk order.
///
/// Twin of `CollectTicketFolderRoots` in `📦️packages/🐹️go/🔓️finish-reopen/🐹️.go`: four levels deep,
/// directories only, dot-prefixed slugs skipped, an unreadable level silently contributing nothing.
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

//#endregion 🧹️PurgeGenerated
