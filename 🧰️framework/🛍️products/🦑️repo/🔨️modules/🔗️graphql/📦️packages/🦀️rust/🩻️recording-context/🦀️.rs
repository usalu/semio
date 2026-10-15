//! 🩻️ The in-memory recording context: a `RepoContext` that answers from a mutable record set and records every write.

use super::*;

//#region 🩻️RecordingContext

/// 🗄️ The mutable record set a [`RecordingContext`] answers from.
#[derive(Clone, Debug, Default)]
struct Records {
    pub(crate) root_dir: String,
    now: String,
    pub(crate) technologies: Vec<Technology>,
    pub(crate) bundles: Vec<Bundle>,
    pub(crate) checkpoints: Vec<Checkpoint>,
    folders: Vec<Folder>,
    pub(crate) files: Vec<File>,
    pub(crate) sections: Vec<Section>,
    pub(crate) definitions: Vec<Definition>,
    pub(crate) contributors: Vec<Contributor>,
    pub(crate) goals: Vec<Goal>,
    pub(crate) tickets: Vec<Ticket>,
    policies: Vec<Policy>,
    pub(crate) drafts: Vec<Draft>,
    pub(crate) todos: Vec<Todo>,
    pub(crate) statutes: Vec<StatuteMeta>,
    pub(crate) interactions: Vec<InteractionResource>,
    pub(crate) analyze: Option<AnalyzeResult>,
    pub(crate) events: Vec<Json>,
}

/// 🎫️ Decodes one ticket record, restoring the fields the wire shape derives from its path.
fn ticket_from_json(value: &Json) -> Result<Ticket, ContextError> {
    let mut ticket: Ticket = serde_json::from_value(value.clone()).map_err(|error| ContextError::new(error.to_string()))?;
    ticket.year = value.get("year").and_then(Json::as_i64).unwrap_or_default();
    ticket.month = value.get("month").and_then(Json::as_i64).unwrap_or_default();
    ticket.day = value.get("day").and_then(Json::as_i64).unwrap_or_default();
    ticket.slug = value.get("slug").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.parent = value.get("parent").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.folder_path = value.get("folderPath").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.json_path = value.get("jsonPath").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.important_path = value.get("importantPath").and_then(Json::as_str).unwrap_or_default().to_string();
    ticket.interactions = value.get("interactions").map_or_else(|| Ok(Vec::new()), |rows| serde_json::from_value(rows.clone())).map_err(|error| ContextError::new(error.to_string()))?;
    ticket.agents = value.get("agents").map_or_else(|| Ok(Vec::new()), |rows| serde_json::from_value(rows.clone())).map_err(|error| ContextError::new(error.to_string()))?;
    Ok(ticket)
}

/// 🎯️ Decodes one goal record, restoring the identifier the wire shape derives from its path.
fn goal_from_json(value: &Json) -> Result<Goal, ContextError> {
    let mut goal: Goal = serde_json::from_value(value.clone()).map_err(|error| ContextError::new(error.to_string()))?;
    goal.id = value.get("id").and_then(Json::as_str).unwrap_or_default().to_string();
    goal.path = value.get("path").and_then(Json::as_str).unwrap_or_default().to_string();
    Ok(goal)
}

/// 📥️ Decodes one array member list, defaulting to empty when the key is absent.
pub(crate) fn rows<T: serde::de::DeserializeOwned>(source: &Json, key: &str) -> Result<Vec<T>, ContextError> {
    match source.get(key) {
        None | Some(Json::Null) => Ok(Vec::new()),
        Some(value) => serde_json::from_value(value.clone()).map_err(|error| ContextError::new(format!("{key}: {error}"))),
    }
}

/// 🩻️ A fixture-backed context that answers every read from a record set and records every write.
///
/// It is the executor's language-agnostic test double: the same record file drives the Rust
/// subject and the TypeScript oracle, so what the two disagree about is the executor, never the
/// filesystem. Every mutation appends an event to [`RecordingContext::events`] and updates the
/// records, so a mutation case asserts both the returned aggregate and the trail it left.
pub struct RecordingContext {
    records: std::cell::RefCell<Records>,
}

impl RecordingContext {
    /// 📥️ Builds a context from one record document.
    pub fn from_json(source: &Json) -> Result<RecordingContext, ContextError> {
        let tickets = match source.get("tickets") {
            None | Some(Json::Null) => Vec::new(),
            Some(Json::Array(items)) => items.iter().map(ticket_from_json).collect::<Result<Vec<_>, _>>()?,
            Some(_) => return Err(ContextError::new("tickets must be an array")),
        };
        let goals = match source.get("goals") {
            None | Some(Json::Null) => Vec::new(),
            Some(Json::Array(items)) => items.iter().map(goal_from_json).collect::<Result<Vec<_>, _>>()?,
            Some(_) => return Err(ContextError::new("goals must be an array")),
        };
        let records = Records {
            root_dir: source.get("rootDir").and_then(Json::as_str).unwrap_or_default().to_string(),
            now: source.get("now").and_then(Json::as_str).unwrap_or("2026-01-01 00:00:00").to_string(),
            technologies: rows(source, "technologies")?,
            bundles: rows(source, "bundles")?,
            checkpoints: rows(source, "checkpoints")?,
            folders: rows(source, "folders")?,
            files: rows(source, "files")?,
            sections: rows(source, "sections")?,
            definitions: rows(source, "definitions")?,
            contributors: rows(source, "contributors")?,
            goals,
            tickets,
            policies: rows(source, "policies")?,
            drafts: rows(source, "drafts")?,
            todos: rows(source, "todos")?,
            statutes: rows(source, "statutes")?,
            interactions: rows(source, "interactions")?,
            analyze: source.get("analyze").filter(|value| !value.is_null()).map(|value| serde_json::from_value(value.clone())).transpose().map_err(|error| ContextError::new(error.to_string()))?,
            events: Vec::new(),
        };
        Ok(RecordingContext { records: std::cell::RefCell::new(records) })
    }

    /// 📥️ Builds a context from one record document's text.
    pub fn from_text(source: &str) -> Result<RecordingContext, ContextError> {
        let value: Json = serde_json::from_str(source).map_err(|error| ContextError::new(error.to_string()))?;
        RecordingContext::from_json(&value)
    }

    /// 📜️ The events every mutation appended, in the order they happened.
    pub fn events(&self) -> Vec<Json> {
        self.records.borrow().events.clone()
    }

    /// 🗄️ The mutable record set as it stands now, for a mutation case to assert on.
    pub fn snapshot(&self) -> Json {
        let records = self.records.borrow();
        let mut fields = Map::new();
        fields.insert("goals".to_string(), to_json(&records.goals));
        fields.insert(
            "tickets".to_string(),
            Json::Array(
                records
                    .tickets
                    .iter()
                    .map(|ticket| {
                        let mut row = body(ticket);
                        row.insert("year".to_string(), Json::from(ticket.year));
                        row.insert("month".to_string(), Json::from(ticket.month));
                        row.insert("day".to_string(), Json::from(ticket.day));
                        row.insert("slug".to_string(), Json::String(ticket.slug.clone()));
                        row.insert("interactions".to_string(), to_json(&ticket.interactions));
                        Json::Object(row)
                    })
                    .collect(),
            ),
        );
        fields.insert("todos".to_string(), to_json(&records.todos));
        fields.insert("drafts".to_string(), to_json(&records.drafts));
        fields.insert("contributors".to_string(), to_json(&records.contributors));
        fields.insert("folders".to_string(), to_json(&records.folders));
        fields.insert("files".to_string(), to_json(&records.files));
        fields.insert("sections".to_string(), to_json(&records.sections));
        Json::Object(fields)
    }

    /// 📝️ Appends one event.
    fn record(&self, kind: &str, payload: Json) {
        let mut fields = Map::new();
        fields.insert("kind".to_string(), Json::String(kind.to_string()));
        fields.insert("payload".to_string(), payload);
        self.records.borrow_mut().events.push(Json::Object(fields));
    }

    /// 🔤️ The identifier a created aggregate derives from its title.
    fn derive_id(title: &str) -> String {
        title.to_lowercase().split(|character: char| !character.is_alphanumeric()).filter(|part| !part.is_empty()).collect::<Vec<_>>().join("-")
    }
}

impl RepoContext for RecordingContext {
    fn root_dir(&self) -> String {
        self.records.borrow().root_dir.clone()
    }

    fn technologies(&self) -> Vec<Technology> {
        self.records.borrow().technologies.clone()
    }

    fn bundles(&self) -> Vec<Bundle> {
        self.records.borrow().bundles.clone()
    }

    fn checkpoints(&self, limit: Option<i64>) -> Result<Vec<Checkpoint>, ContextError> {
        let checkpoints = self.records.borrow().checkpoints.clone();
        Ok(match limit {
            Some(limit) if limit >= 0 => checkpoints.into_iter().take(limit.unsigned_abs() as usize).collect(),
            _ => checkpoints,
        })
    }

    fn folders(&self) -> Vec<Folder> {
        self.records.borrow().folders.clone()
    }

    fn files(&self) -> Vec<File> {
        self.records.borrow().files.clone()
    }

    fn sections(&self) -> Vec<Section> {
        self.records.borrow().sections.clone()
    }

    fn definitions(&self) -> Vec<Definition> {
        self.records.borrow().definitions.clone()
    }

    fn contributors(&self) -> Result<Vec<Contributor>, ContextError> {
        Ok(self.records.borrow().contributors.clone())
    }

    fn goals(&self) -> Result<Vec<Goal>, ContextError> {
        Ok(self.records.borrow().goals.clone())
    }

    fn tickets(&self, year: Option<i64>, month: Option<i64>, day: Option<i64>, status: Option<TicketStatus>) -> Result<Vec<Ticket>, ContextError> {
        Ok(self
            .records
            .borrow()
            .tickets
            .iter()
            .filter(|ticket| year.is_none_or(|value| ticket.year == value))
            .filter(|ticket| month.is_none_or(|value| ticket.month == value))
            .filter(|ticket| day.is_none_or(|value| ticket.day == value))
            .filter(|ticket| status.is_none_or(|value| ticket.status == value))
            .cloned()
            .collect())
    }

    fn policies(&self) -> Vec<Policy> {
        self.records.borrow().policies.clone()
    }

    fn drafts(&self) -> Result<Vec<Draft>, ContextError> {
        Ok(self.records.borrow().drafts.clone())
    }

    fn todos(&self, filter: Option<&FilterInput>) -> Result<Vec<Todo>, ContextError> {
        Ok(self.records.borrow().todos.iter().filter(|todo| matches_filter(&todo.name, filter)).cloned().collect())
    }

    fn statutes(&self) -> Vec<StatuteMeta> {
        self.records.borrow().statutes.clone()
    }

    fn interactions(&self) -> Result<Vec<InteractionResource>, ContextError> {
        Ok(self.records.borrow().interactions.clone())
    }

    fn analyze(&self, scope: Option<&str>) -> Result<AnalyzeResult, ContextError> {
        let records = self.records.borrow();
        let result = records.analyze.clone().unwrap_or(AnalyzeResult { breachs: Some(Vec::new()), metrics: Some(AnalyzeMetrics { total: 0, by_priority: Some(PriorityCount::default()), autofixable: 0 }) });
        let Some(scope) = scope else { return Ok(result) };
        let breachs: Vec<Breach> = result.breachs.unwrap_or_default().into_iter().filter(|breach| breach.scope.starts_with(scope)).collect();
        Ok(AnalyzeResult { metrics: result.metrics, breachs: Some(breachs) })
    }

    fn goal_create(&self, input: GoalCreateInput) -> Result<Goal, ContextError> {
        let goal = Goal {
            title: input.title.clone(),
            description: input.description.clone(),
            prompt: input.prompt.clone(),
            status: GoalStatus::Open,
            summary: String::new(),
            due_date: input.due_date.clone(),
            dates: GoalDates { due: input.due_date.clone() },
            client: input.client.clone(),
            llm: input.llm.clone(),
            effort: input.effort.clone(),
            parent: input.parent.clone(),
            management: Some(GoalManagementData { milestone: input.milestone.clone(), issue: String::new() }),
            id: RecordingContext::derive_id(&input.title),
            path: String::new(),
        };
        self.records.borrow_mut().goals.push(goal.clone());
        self.record("goal.create", to_json(&input));
        Ok(goal)
    }

    fn goal_change(&self, input: GoalChangeInput) -> Result<Goal, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(goal) = records.goals.iter_mut().find(|goal| goal.id == input.id) else {
            return Err(ContextError::new(format!("goal {} not found", input.id)));
        };
        if let Some(title) = &input.title {
            goal.title.clone_from(title);
        }
        if let Some(description) = &input.description {
            goal.description.clone_from(description);
        }
        if let Some(due) = &input.due_date {
            goal.due_date.clone_from(due);
            goal.dates.due.clone_from(due);
        }
        if let Some(llm) = &input.llm {
            goal.llm.clone_from(llm);
        }
        if let Some(effort) = &input.effort {
            goal.effort.clone_from(effort);
        }
        if let Some(parent) = &input.parent {
            goal.parent.clone_from(parent);
        }
        let changed = goal.clone();
        drop(records);
        self.record("goal.change", to_json(&input));
        Ok(changed)
    }

    fn goal_close(&self, input: GoalCloseInput) -> Result<Goal, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(goal) = records.goals.iter_mut().find(|goal| goal.id == input.id) else {
            return Err(ContextError::new(format!("goal {} not found", input.id)));
        };
        goal.status = GoalStatus::Closed;
        goal.summary.clone_from(&input.summary);
        let changed = goal.clone();
        drop(records);
        self.record("goal.close", to_json(&input));
        Ok(changed)
    }

    fn goal_reopen(&self, input: GoalReopenInput) -> Result<Goal, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(goal) = records.goals.iter_mut().find(|goal| goal.id == input.id) else {
            return Err(ContextError::new(format!("goal {} not found", input.id)));
        };
        goal.status = GoalStatus::Open;
        goal.prompt.clone_from(&input.prompt);
        goal.client.clone_from(&input.client);
        goal.llm.clone_from(&input.llm);
        let changed = goal.clone();
        drop(records);
        self.record("goal.reopen", to_json(&input));
        Ok(changed)
    }

    fn goal_delete(&self, input: GoalDeleteInput) -> Result<bool, ContextError> {
        let before = self.records.borrow().goals.len();
        self.records.borrow_mut().goals.retain(|goal| goal.id != input.id);
        let removed = self.records.borrow().goals.len() < before;
        self.record("goal.delete", to_json(&input));
        Ok(removed)
    }

    fn todo_create(&self, input: TodoCreateInput) -> Result<Todo, ContextError> {
        let todo = Todo { id: RecordingContext::derive_id(&input.name), name: input.name.clone(), description: input.description.clone(), parent_id: input.parent_id.clone(), location: None };
        self.records.borrow_mut().todos.push(todo.clone());
        self.record("todo.create", to_json(&input));
        Ok(todo)
    }

    fn todo_change(&self, input: TodoChangeInput) -> Result<Todo, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(todo) = records.todos.iter_mut().find(|todo| todo.id == input.id) else {
            return Err(ContextError::new(format!("todo {} not found", input.id)));
        };
        if let Some(name) = &input.name {
            todo.name.clone_from(name);
        }
        if let Some(description) = &input.description {
            todo.description.clone_from(description);
        }
        let changed = todo.clone();
        drop(records);
        self.record("todo.change", to_json(&input));
        Ok(changed)
    }

    fn todo_delete(&self, id: &str) -> Result<bool, ContextError> {
        let before = self.records.borrow().todos.len();
        self.records.borrow_mut().todos.retain(|todo| todo.id != id);
        let removed = self.records.borrow().todos.len() < before;
        self.record("todo.delete", Json::String(id.to_string()));
        Ok(removed)
    }

    fn draft_create(&self, input: DraftCreateInput) -> Result<Draft, ContextError> {
        let draft = Draft { id: RecordingContext::derive_id(&input.title) };
        self.records.borrow_mut().drafts.push(draft.clone());
        self.record("draft.create", to_json(&input));
        Ok(draft)
    }

    fn draft_delete(&self, id: &str) -> Result<bool, ContextError> {
        let before = self.records.borrow().drafts.len();
        self.records.borrow_mut().drafts.retain(|draft| draft.id != id);
        let removed = self.records.borrow().drafts.len() < before;
        self.record("draft.delete", Json::String(id.to_string()));
        Ok(removed)
    }

    fn ticket_open(&self, input: TicketOpenInput) -> Result<Ticket, ContextError> {
        let now = self.records.borrow().now.clone();
        let (year, month, day) = (
            now.get(0..4).and_then(|value| value.parse().ok()).unwrap_or(0),
            now.get(5..7).and_then(|value| value.parse().ok()).unwrap_or(0),
            now.get(8..10).and_then(|value| value.parse().ok()).unwrap_or(0),
        );
        let slug = RecordingContext::derive_id(&input.title).to_uppercase();
        let ticket = Ticket {
            year,
            month,
            day,
            slug: slug.clone(),
            title: input.title.clone(),
            emoji: input.emoji.clone(),
            status: TicketStatus::Open,
            description: input.prompt.clone(),
            summary: String::new(),
            management: Some(TicketManagementData { issue: input.issue.clone() }),
            goal: input.goal.clone(),
            parent: input.parent.clone(),
            plan: None,
            sessions: Vec::new(),
            interactions: vec![Interaction {
                kind: "ticket.open".to_string(),
                date: now,
                author: input.client.clone(),
                system: String::new(),
                client: input.client.clone(),
                checkpoint: String::new(),
                prompt: input.prompt.clone(),
                summary: String::new(),
                llm: input.llm.clone(),
                effort: input.effort.clone(),
                files: Vec::new(),
            }],
            agents: Vec::new(),
            folder_path: format!("{year:04}/{month:02}/{day:02}/{slug}"),
            json_path: String::new(),
            important_path: String::new(),
        };
        self.records.borrow_mut().tickets.push(ticket.clone());
        self.record("ticket.open", to_json(&input));
        Ok(ticket)
    }

    fn ticket_close(&self, input: TicketCloseInput) -> Result<Ticket, ContextError> {
        let now = self.records.borrow().now.clone();
        let mut records = self.records.borrow_mut();
        let Some(ticket) = records.tickets.iter_mut().find(|ticket| ticket.year == input.year && ticket.month == input.month && ticket.day == input.day && ticket.slug == input.slug) else {
            return Err(ContextError::new(format!("ticket {}/{}/{}/{} not found", input.year, input.month, input.day, input.slug)));
        };
        ticket.status = TicketStatus::Closed;
        ticket.summary.clone_from(&input.summary);
        if let Some(title) = &input.title {
            ticket.title.clone_from(title);
        }
        ticket.interactions.push(Interaction {
            kind: "ticket.close".to_string(),
            date: now,
            author: String::new(),
            system: String::new(),
            client: String::new(),
            checkpoint: String::new(),
            prompt: String::new(),
            summary: input.summary.clone(),
            llm: String::new(),
            effort: String::new(),
            files: input.files.clone().unwrap_or_default().into_iter().map(|path| InteractionFile { path, id: String::new(), uri: String::new() }).collect(),
        });
        let changed = ticket.clone();
        drop(records);
        self.record("ticket.close", to_json(&input));
        Ok(changed)
    }

    fn ticket_reopen(&self, input: TicketReopenInput) -> Result<Ticket, ContextError> {
        let now = self.records.borrow().now.clone();
        let mut records = self.records.borrow_mut();
        let Some(ticket) = records.tickets.iter_mut().find(|ticket| ticket.year == input.year && ticket.month == input.month && ticket.day == input.day && ticket.slug == input.slug) else {
            return Err(ContextError::new(format!("ticket {}/{}/{}/{} not found", input.year, input.month, input.day, input.slug)));
        };
        ticket.status = TicketStatus::Open;
        if let Some(title) = &input.title {
            ticket.title.clone_from(title);
        }
        ticket.interactions.push(Interaction {
            kind: "ticket.open".to_string(),
            date: now,
            author: input.client.clone(),
            system: String::new(),
            client: input.client.clone(),
            checkpoint: String::new(),
            prompt: input.prompt.clone(),
            summary: String::new(),
            llm: input.llm.clone(),
            effort: input.effort.clone(),
            files: Vec::new(),
        });
        let changed = ticket.clone();
        drop(records);
        self.record("ticket.reopen", to_json(&input));
        Ok(changed)
    }

    fn ticket_change(&self, input: TicketChangeInput) -> Result<Ticket, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(ticket) = records.tickets.iter_mut().find(|ticket| ticket.year == input.year && ticket.month == input.month && ticket.day == input.day && ticket.slug == input.slug) else {
            return Err(ContextError::new(format!("ticket {}/{}/{}/{} not found", input.year, input.month, input.day, input.slug)));
        };
        if let Some(title) = &input.title {
            ticket.title.clone_from(title);
        }
        if let Some(prompt) = &input.prompt {
            ticket.description.clone_from(prompt);
        }
        if let Some(goal) = &input.goal {
            ticket.goal.clone_from(goal);
        }
        if let Some(parent) = &input.parent {
            ticket.parent.clone_from(parent);
        }
        let changed = ticket.clone();
        drop(records);
        self.record("ticket.change", to_json(&input));
        Ok(changed)
    }

    fn ticket_delete(&self, input: TicketDeleteInput) -> Result<bool, ContextError> {
        let before = self.records.borrow().tickets.len();
        self.records.borrow_mut().tickets.retain(|ticket| !(ticket.year == input.year && ticket.month == input.month && ticket.day == input.day && ticket.slug == input.slug));
        let removed = self.records.borrow().tickets.len() < before;
        self.record("ticket.delete", to_json(&input));
        Ok(removed)
    }

    fn contributor_add(&self, input: ContributorAddInput) -> Result<Contributor, ContextError> {
        let contributor = Contributor {
            alias: input.github.clone(),
            aliases: Vec::new(),
            emoji: String::new(),
            github: input.github.clone(),
            githubs: Vec::new(),
            name: input.name.clone().unwrap_or_default(),
            names: input.names.clone(),
            email: input.email.clone().unwrap_or_default(),
            emails: input.emails.clone(),
            links: BTreeMap::new(),
            fingerprint: input.fingerprint.clone().unwrap_or_default(),
            fingerprints: input.fingerprints.clone(),
            contributions: ContributorContributionsStorage::default(),
        };
        self.records.borrow_mut().contributors.push(contributor.clone());
        self.record("contributor.add", to_json(&input));
        Ok(contributor)
    }

    fn contributor_remove(&self, github: &str) -> Result<(), ContextError> {
        self.records.borrow_mut().contributors.retain(|contributor| contributor.github != github);
        self.record("contributor.remove", Json::String(github.to_string()));
        Ok(())
    }

    fn folder_create(&self, path: &str) -> Result<Folder, ContextError> {
        let folder = Folder {
            id: RecordingContext::derive_id(path),
            path: path.to_string(),
            uri: file_uri(&self.root_dir(), path),
            name: path.rsplit('/').next().unwrap_or(path).to_string(),
            parent_id: None,
            bundle_id: None,
            kind: FolderKind::Organization,
            emoji: String::new(),
            ignored: false,
            generated: false,
        };
        self.records.borrow_mut().folders.push(folder.clone());
        self.record("folder.create", Json::String(path.to_string()));
        Ok(folder)
    }

    fn folder_move(&self, source: &str, destination: &str) -> Result<Folder, ContextError> {
        let root = self.root_dir();
        let mut records = self.records.borrow_mut();
        let Some(folder) = records.folders.iter_mut().find(|folder| folder.path == source) else {
            return Err(ContextError::new(format!("folder {source} not found")));
        };
        folder.path = destination.to_string();
        folder.uri = file_uri(&root, destination);
        folder.name = destination.rsplit('/').next().unwrap_or(destination).to_string();
        let moved = folder.clone();
        drop(records);
        self.record("folder.move", Json::Array(vec![Json::String(source.to_string()), Json::String(destination.to_string())]));
        Ok(moved)
    }

    fn folder_delete(&self, path: &str) -> Result<(), ContextError> {
        self.records.borrow_mut().folders.retain(|folder| folder.path != path);
        self.record("folder.delete", Json::String(path.to_string()));
        Ok(())
    }

    fn file_create(&self, path: &str) -> Result<File, ContextError> {
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        let file = File {
            id: RecordingContext::derive_id(path),
            path: path.to_string(),
            uri: file_uri(&self.root_dir(), path),
            extension: name.rfind('.').map(|index| name[index..].to_string()).unwrap_or_default(),
            name,
            folder_id: None,
            bundle_id: None,
            kind: "code".to_string(),
            emoji: String::new(),
            ignored: false,
            generated: false,
        };
        self.records.borrow_mut().files.push(file.clone());
        self.record("file.create", Json::String(path.to_string()));
        Ok(file)
    }

    fn file_move(&self, source: &str, destination: &str) -> Result<File, ContextError> {
        let root = self.root_dir();
        let mut records = self.records.borrow_mut();
        let Some(file) = records.files.iter_mut().find(|file| file.path == source) else {
            return Err(ContextError::new(format!("file {source} not found")));
        };
        file.path = destination.to_string();
        file.uri = file_uri(&root, destination);
        file.name = destination.rsplit('/').next().unwrap_or(destination).to_string();
        let moved = file.clone();
        drop(records);
        self.record("file.move", Json::Array(vec![Json::String(source.to_string()), Json::String(destination.to_string())]));
        Ok(moved)
    }

    fn file_delete(&self, path: &str) -> Result<(), ContextError> {
        self.records.borrow_mut().files.retain(|file| file.path != path);
        self.record("file.delete", Json::String(path.to_string()));
        Ok(())
    }

    fn section_create(&self, file: &str, name: &str, parent: Option<&str>) -> Result<Section, ContextError> {
        let path = parent.map_or_else(|| name.to_string(), |parent| format!("{parent}#{name}"));
        let section = Section {
            id: String::new(),
            name: name.to_string(),
            path,
            file_path: file.to_string(),
            emoji: String::new(),
            start_line: 0,
            end_line: 0,
            start_index: 0,
            end_index: 0,
            children: Vec::new(),
            definitions: Vec::new(),
        };
        self.records.borrow_mut().sections.push(section.clone());
        self.record("section.create", Json::Array(vec![Json::String(file.to_string()), Json::String(name.to_string())]));
        Ok(section)
    }

    fn section_move(&self, file: &str, old_name: &str, new_name: &str) -> Result<Section, ContextError> {
        let mut records = self.records.borrow_mut();
        let Some(section) = records.sections.iter_mut().find(|section| section.file_path == file && section.name == old_name) else {
            return Err(ContextError::new(format!("section {old_name} not found in {file}")));
        };
        section.name = new_name.to_string();
        section.path = new_name.to_string();
        let moved = section.clone();
        drop(records);
        self.record("section.move", Json::Array(vec![Json::String(file.to_string()), Json::String(old_name.to_string()), Json::String(new_name.to_string())]));
        Ok(moved)
    }

    fn section_delete(&self, file: &str, name: &str) -> Result<(), ContextError> {
        self.records.borrow_mut().sections.retain(|section| !(section.file_path == file && section.name == name));
        self.record("section.delete", Json::Array(vec![Json::String(file.to_string()), Json::String(name.to_string())]));
        Ok(())
    }

    fn integrate(&self, source: Option<&str>, target_section: Option<&str>, target_file: Option<&str>, target_parent: Option<&str>) -> Result<File, ContextError> {
        let target = target_file.unwrap_or_default();
        self.record(
            "integrate",
            Json::Array(
                [source, target_section, target_file, target_parent]
                    .into_iter()
                    .map(|value| value.map_or(Json::Null, |text| Json::String(text.to_string())))
                    .collect(),
            ),
        );
        self.records.borrow().files.iter().find(|file| file.path == target).cloned().ok_or_else(|| ContextError::new(format!("file {target} not found")))
    }

    fn extract(&self, source_file: Option<&str>, source_section: Option<&str>, target_file: Option<&str>) -> Result<File, ContextError> {
        let target = target_file.unwrap_or_default();
        self.record(
            "extract",
            Json::Array([source_file, source_section, target_file].into_iter().map(|value| value.map_or(Json::Null, |text| Json::String(text.to_string()))).collect()),
        );
        let existing = self.records.borrow().files.iter().find(|file| file.path == target).cloned();
        match existing {
            Some(file) => Ok(file),
            None => self.file_create(target),
        }
    }

    fn sync_management(&self) -> Result<bool, ContextError> {
        self.record("sync.management", Json::Null);
        Ok(true)
    }
}

//#endregion 🩻️RecordingContext
