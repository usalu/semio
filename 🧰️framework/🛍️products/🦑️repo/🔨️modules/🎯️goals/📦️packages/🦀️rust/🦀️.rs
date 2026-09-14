//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//! 🎯️ Goals of the `🦑️repo` product: the `GOAL/SUBGOAL` identifier scheme, the `🎯️goal.json`
//! document codec, the goal/ticket tree and the open/change/close/reopen/delete lifecycle.
//!
//! Every effect is a port. A goal document is read and written through [`GoalStore`], the
//! management provider that owns milestones and issues is [`ManagementPort`], and the coordinator
//! is `semio_framework_repo_events::Emitter`. [`MemoryGoalStore`], [`RecordingManagement`] and
//! [`MemoryEmitter`] are the in-memory implementations the language-agnostic tests run against, so
//! nothing in this crate needs a repository on disk, a `gh` binary or a network to be judged.
//!
//! The wire contract is `🧬️schema/🔣️.json`; the fixtures under `🧫️fixtures` are real documents
//! taken from this repository's own `🎯️goals` tree.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use semio_framework_repo_events::Emitter;
use semio_framework_repo_identity as identity;
use semio_framework_repo_model as model;

pub use model::{Goal, GoalChangeInput, GoalCloseInput, GoalCreateInput, GoalDates, GoalDeleteInput, GoalManagementData, GoalNode, GoalReopenInput, GoalStatus, TicketNode};

//#region ❗️Errors

/// ❗️ Everything this crate can refuse, as a class rather than a rendered string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoalError {
    /// 🕳️ A required create input was blank. Carries the field name Go names in its message.
    Missing(&'static str),
    /// 🚫️ A slug is outside the allowed vocabulary of `📐️model`.
    NotAllowed(String),
    /// 📛️ A goal already exists at that identifier.
    AlreadyExists(String),
    /// 🔍️ No goal document at that identifier.
    NotFound(String),
    /// 🔁️ The lifecycle transition is a no-operation.
    AlreadyInState(&'static str),
    /// 🔤️ A title is blank or is a slug rather than a titleized phrase.
    Title(&'static str),
    /// 🔢️ A milestone or issue reference carries no number.
    Unparsable(String),
    /// 📄️ The document is not the JSON this codec accepts.
    Document(String),
    /// 🗄️ The store or the management provider refused.
    Port(String),
}

impl std::fmt::Display for GoalError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoalError::Missing(field) => write!(formatter, "missing {field}"),
            GoalError::NotAllowed(message) => write!(formatter, "{message}"),
            GoalError::AlreadyExists(id) => write!(formatter, "goal with id {id} already exists"),
            GoalError::NotFound(id) => write!(formatter, "goal file not found: {id}"),
            GoalError::AlreadyInState(state) => write!(formatter, "goal is already {state}"),
            GoalError::Title(message) => write!(formatter, "{message}"),
            GoalError::Unparsable(value) => write!(formatter, "could not parse number from {value}"),
            GoalError::Document(message) => write!(formatter, "{message}"),
            GoalError::Port(message) => write!(formatter, "{message}"),
        }
    }
}

impl GoalError {
    /// 🏷️ The stable class of the refusal, for a projection that must not depend on wording.
    pub fn class(&self) -> &'static str {
        match self {
            GoalError::Missing(_) => "missing",
            GoalError::NotAllowed(_) => "not-allowed",
            GoalError::AlreadyExists(_) => "already-exists",
            GoalError::NotFound(_) => "not-found",
            GoalError::AlreadyInState(_) => "already-in-state",
            GoalError::Title(_) => "title",
            GoalError::Unparsable(_) => "unparsable",
            GoalError::Document(_) => "document",
            GoalError::Port(_) => "port",
        }
    }
}

impl From<model::ModelError> for GoalError {
    fn from(error: model::ModelError) -> Self {
        GoalError::NotAllowed(error.to_string())
    }
}

//#endregion ❗️Errors

//#region 🪪️IdScheme

/// 📏️ How many `/` separated ancestors an identifier carries.
pub fn goal_depth(goal_id: &str) -> usize {
    goal_id.matches('/').count()
}

/// 🌱️ A goal with no ancestor, which is the one that owns a management milestone.
pub fn is_root_goal(goal_id: &str) -> bool {
    goal_depth(goal_id) == 0
}

/// 🔹️ A direct child of a root goal, which is the one whose issue joins the root milestone.
pub fn is_first_gen_goal(goal_id: &str) -> bool {
    goal_depth(goal_id) == 1
}

/// 🔸️ A goal at depth two or deeper, whose issue is a sub-issue of its parent's issue.
pub fn is_deeper_goal(goal_id: &str) -> bool {
    goal_depth(goal_id) >= 2
}

/// 🎯️ The compose identifier of a `GOAL/SUBGOAL` path, one `🎯️`-tagged segment per level.
pub fn goal_path_to_compose_id(goal_path: &str) -> String {
    identity::goal_path_to_compose_id(goal_path)
}

/// 🛤️ The `GOAL/SUBGOAL` path of a compose identifier, empty when it carries no goal segment.
pub fn compose_id_to_goal_path(compose_id: &str) -> String {
    identity::compose_id_to_goal_segments(compose_id).join("/")
}

/// 🛤️ The filesystem path form of any identifier, accepting both the path and the compose form.
pub fn goal_id_for_filesystem(goal_id: &str) -> String {
    let goal_id = goal_id.trim();
    if goal_id.is_empty() {
        return String::new();
    }
    if goal_id.contains('/') {
        return goal_id.to_string();
    }
    if goal_id.contains(&identity::emoji_text(identity::entity("goal"))) {
        let path = compose_id_to_goal_path(goal_id);
        if !path.is_empty() && path != goal_id {
            return path;
        }
    }
    goal_id.to_string()
}

/// 🔺️ The outermost ancestor of an identifier, in filesystem path form.
pub fn root_goal_id(goal_id: &str) -> String {
    let goal_id = goal_id_for_filesystem(goal_id);
    match goal_id.find('/') {
        Some(index) => goal_id[..index].to_string(),
        None => goal_id,
    }
}

/// 🔻️ The immediate ancestor of an identifier, empty for a root goal.
pub fn parent_goal_id(goal_id: &str) -> String {
    let goal_id = goal_id_for_filesystem(goal_id);
    match goal_id.rfind('/') {
        Some(index) => goal_id[..index].to_string(),
        None => String::new(),
    }
}

/// 🧬️ The identifier a title takes under a parent: the parent path, `/`, and the title's slug.
pub fn compose_goal_id(parent: &str, title: &str) -> String {
    let slug = identity::slugify(title);
    if parent.is_empty() {
        slug
    } else {
        format!("{parent}/{slug}")
    }
}

/// 🔢️ The trailing number of a milestone reference, accepting a bare number or a URL.
pub fn parse_milestone_number(milestone: &str) -> Result<i64, GoalError> {
    parse_trailing_number(milestone)
}

/// 🔢️ The trailing number of an issue URL.
pub fn parse_issue_number(issue_url: &str) -> Result<i64, GoalError> {
    parse_trailing_number(issue_url)
}

fn parse_trailing_number(value: &str) -> Result<i64, GoalError> {
    if let Ok(number) = value.parse::<i64>() {
        return Ok(number);
    }
    if let Some(last) = value.split('/').next_back() {
        if let Ok(number) = last.parse::<i64>() {
            return Ok(number);
        }
    }
    Err(GoalError::Unparsable(value.to_string()))
}

//#endregion 🪪️IdScheme

//#region 📄️DocumentCodec

/// 📥️ Decodes a `🎯️goal.json` document, deriving the identifier and parent from the path the
/// document was found at, exactly as the reader does — the stored `parent` member is the compose
/// form and is authoritative only for the document, never for the tree.
pub fn decode_goal(id: &str, document: &str) -> Result<Goal, GoalError> {
    let mut goal: Goal = serde_json::from_str(document).map_err(|error| GoalError::Document(error.to_string()))?;
    goal.id = id.to_string();
    goal.parent = parent_goal_id(id);
    Ok(goal)
}

/// 📤️ Encodes a goal back into its document: two-space indentation, members in declaration order,
/// a trailing newline, and the `parent` member rewritten from the filesystem path into the compose
/// identifier the stored document carries.
pub fn encode_goal(goal: &Goal) -> Result<String, GoalError> {
    let mut stored = goal.clone();
    if !stored.parent.is_empty() && !stored.parent.contains(&identity::emoji_text(identity::entity("goal"))) {
        stored.parent = goal_path_to_compose_id(&stored.parent);
    }
    let text = serde_json::to_string_pretty(&stored).map_err(|error| GoalError::Document(error.to_string()))?;
    Ok(format!("{text}\n"))
}

//#endregion 📄️DocumentCodec

//#region 🗄️Store

/// 🗄️ Where goal documents live. One entry per `GOAL/SUBGOAL` identifier.
pub trait GoalStore {
    /// 📖️ The document at an identifier.
    fn read(&self, id: &str) -> Result<String, GoalError>;
    /// 💾️ Replaces or creates the document at an identifier.
    fn write(&self, id: &str, document: &str) -> Result<(), GoalError>;
    /// 🔍️ Whether a document exists at an identifier.
    fn exists(&self, id: &str) -> bool;
    /// 🚚️ Moves a goal and everything below it to a new identifier.
    fn rename(&self, from: &str, to: &str) -> Result<(), GoalError>;
    /// 🗑️ Removes a goal and everything below it.
    fn remove(&self, id: &str) -> Result<(), GoalError>;
    /// 📋️ Every identifier that carries a document, in ascending order.
    fn ids(&self) -> Vec<String>;
}

/// 🧠️ The in-memory store the language-agnostic tests run the lifecycle against.
#[derive(Debug, Default)]
pub struct MemoryGoalStore {
    documents: RefCell<BTreeMap<String, String>>,
}

impl MemoryGoalStore {
    /// 🆕️ An empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🌱️ A store seeded with `(identifier, document)` pairs.
    pub fn seeded<I: IntoIterator<Item = (String, String)>>(entries: I) -> Self {
        let store = Self::new();
        for (id, document) in entries {
            store.documents.borrow_mut().insert(id, document);
        }
        store
    }

    /// 📸️ Every stored `(identifier, document)` pair, in identifier order.
    pub fn snapshot(&self) -> Vec<(String, String)> {
        self.documents.borrow().iter().map(|(id, document)| (id.clone(), document.clone())).collect()
    }
}

impl GoalStore for MemoryGoalStore {
    fn read(&self, id: &str) -> Result<String, GoalError> {
        self.documents.borrow().get(id).cloned().ok_or_else(|| GoalError::NotFound(id.to_string()))
    }

    fn write(&self, id: &str, document: &str) -> Result<(), GoalError> {
        self.documents.borrow_mut().insert(id.to_string(), document.to_string());
        Ok(())
    }

    fn exists(&self, id: &str) -> bool {
        self.documents.borrow().contains_key(id)
    }

    fn rename(&self, from: &str, to: &str) -> Result<(), GoalError> {
        let mut documents = self.documents.borrow_mut();
        let moved: Vec<String> = documents.keys().filter(|key| key.as_str() == from || key.starts_with(&format!("{from}/"))).cloned().collect();
        if moved.is_empty() {
            return Err(GoalError::NotFound(from.to_string()));
        }
        for key in moved {
            let document = documents.remove(&key).unwrap_or_default();
            documents.insert(format!("{to}{}", &key[from.len()..]), document);
        }
        Ok(())
    }

    fn remove(&self, id: &str) -> Result<(), GoalError> {
        let mut documents = self.documents.borrow_mut();
        let removed: Vec<String> = documents.keys().filter(|key| key.as_str() == id || key.starts_with(&format!("{id}/"))).cloned().collect();
        for key in removed {
            documents.remove(&key);
        }
        Ok(())
    }

    fn ids(&self) -> Vec<String> {
        self.documents.borrow().keys().cloned().collect()
    }
}

/// 💽️ The store over a real `.🧬semio/🦑️repo/🎯️goals` tree.
#[derive(Debug, Clone)]
pub struct FsGoalStore {
    root: PathBuf,
}

impl FsGoalStore {
    /// 🆕️ The store of the goals directory of a repository root.
    pub fn new(repo_root: &Path) -> Self {
        Self { root: semio_framework_repo_workspace::goals_dir_for_root(repo_root) }
    }

    /// 📁️ The directory of one goal.
    pub fn directory(&self, id: &str) -> PathBuf {
        id.split('/').fold(self.root.clone(), |path, segment| path.join(segment))
    }

    fn document_path(&self, id: &str) -> PathBuf {
        self.directory(id).join("🎯️goal.json")
    }
}

impl GoalStore for FsGoalStore {
    fn read(&self, id: &str) -> Result<String, GoalError> {
        std::fs::read_to_string(self.document_path(id)).map_err(|_| GoalError::NotFound(id.to_string()))
    }

    fn write(&self, id: &str, document: &str) -> Result<(), GoalError> {
        let path = self.document_path(id);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| GoalError::Port(error.to_string()))?;
        }
        std::fs::write(path, document).map_err(|error| GoalError::Port(error.to_string()))
    }

    fn exists(&self, id: &str) -> bool {
        self.document_path(id).exists()
    }

    fn rename(&self, from: &str, to: &str) -> Result<(), GoalError> {
        let target = self.directory(to);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|error| GoalError::Port(error.to_string()))?;
        }
        std::fs::rename(self.directory(from), target).map_err(|error| GoalError::Port(error.to_string()))
    }

    fn remove(&self, id: &str) -> Result<(), GoalError> {
        std::fs::remove_dir_all(self.directory(id)).map_err(|error| GoalError::Port(error.to_string()))
    }

    fn ids(&self) -> Vec<String> {
        let mut found = Vec::new();
        collect_goal_ids(&self.root, "", &mut found);
        found.sort();
        found
    }
}

fn collect_goal_ids(directory: &Path, prefix: &str, found: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(directory) else { return };
    for entry in entries.flatten() {
        if !entry.path().is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let id = if prefix.is_empty() { name } else { format!("{prefix}/{name}") };
        if entry.path().join("🎯️goal.json").exists() {
            found.push(id.clone());
        }
        collect_goal_ids(&entry.path(), &id, found);
    }
}

//#endregion 🗄️Store

//#region 🧩️Management

/// 🧩️ The provider that owns milestones and issues for the goals of this repository.
pub trait ManagementPort {
    /// 🏁️ Creates a milestone and returns its number.
    fn create_milestone(&self, title: &str, description: &str) -> Result<i64, GoalError>;
    /// 🔁️ Updates a milestone. Blank members are left untouched.
    fn update_milestone(&self, number: i64, title: &str, description: &str, state: &str, due_on: &str) -> Result<(), GoalError>;
    /// 🗑️ Deletes a milestone.
    fn delete_milestone(&self, number: i64) -> Result<(), GoalError>;
    /// 🆕️ Creates the issue of a goal, optionally under a milestone, and returns its URL.
    fn create_goal_issue(&self, title: &str, description: &str, milestone: Option<i64>) -> Result<String, GoalError>;
    /// 🔷️ Updates the title and body of a goal issue.
    fn update_goal_issue(&self, issue_url: &str, title: &str, description: &str) -> Result<(), GoalError>;
    /// 📪️ Closes an issue.
    fn close_issue(&self, issue_url: &str) -> Result<(), GoalError>;
    /// 🔓️ Reopens an issue.
    fn reopen_issue(&self, issue_url: &str) -> Result<(), GoalError>;
    /// 🗑️ Deletes an issue by its number.
    fn delete_issue(&self, number: &str) -> Result<(), GoalError>;
    /// ➕️ Nests a child issue under a parent issue.
    fn add_sub_issue(&self, parent_issue_url: &str, child_issue_url: &str) -> Result<(), GoalError>;
    /// 🌐️ The repository URL milestone references are built from.
    fn repo_url(&self) -> String;
}

/// 🚫️ The provider that does nothing, for a lifecycle run with management switched off.
#[derive(Debug, Clone, Default)]
pub struct NullManagement;

impl ManagementPort for NullManagement {
    fn create_milestone(&self, _title: &str, _description: &str) -> Result<i64, GoalError> {
        Ok(0)
    }
    fn update_milestone(&self, _number: i64, _title: &str, _description: &str, _state: &str, _due_on: &str) -> Result<(), GoalError> {
        Ok(())
    }
    fn delete_milestone(&self, _number: i64) -> Result<(), GoalError> {
        Ok(())
    }
    fn create_goal_issue(&self, _title: &str, _description: &str, _milestone: Option<i64>) -> Result<String, GoalError> {
        Ok(String::new())
    }
    fn update_goal_issue(&self, _issue_url: &str, _title: &str, _description: &str) -> Result<(), GoalError> {
        Ok(())
    }
    fn close_issue(&self, _issue_url: &str) -> Result<(), GoalError> {
        Ok(())
    }
    fn reopen_issue(&self, _issue_url: &str) -> Result<(), GoalError> {
        Ok(())
    }
    fn delete_issue(&self, _number: &str) -> Result<(), GoalError> {
        Ok(())
    }
    fn add_sub_issue(&self, _parent_issue_url: &str, _child_issue_url: &str) -> Result<(), GoalError> {
        Ok(())
    }
    fn repo_url(&self) -> String {
        String::new()
    }
}

/// 📝️ The in-memory provider: hands out deterministic milestone numbers and issue URLs and keeps
/// an ordered log of every call, so a lifecycle scenario can assert what the provider was asked to
/// do without a `gh` binary or a network.
#[derive(Debug)]
pub struct RecordingManagement {
    repo_url: String,
    next: RefCell<i64>,
    calls: RefCell<Vec<String>>,
}

impl RecordingManagement {
    /// 🆕️ A provider that numbers milestones and issues from `first`.
    pub fn new(repo_url: &str, first: i64) -> Self {
        Self { repo_url: repo_url.to_string(), next: RefCell::new(first), calls: RefCell::new(Vec::new()) }
    }

    /// 📜️ Every call made so far, in order.
    pub fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }

    fn take_number(&self) -> i64 {
        let mut next = self.next.borrow_mut();
        let number = *next;
        *next += 1;
        number
    }

    fn record(&self, call: String) {
        self.calls.borrow_mut().push(call);
    }
}

impl ManagementPort for RecordingManagement {
    fn create_milestone(&self, title: &str, description: &str) -> Result<i64, GoalError> {
        let number = self.take_number();
        self.record(format!("create-milestone {number} {title} {description}"));
        Ok(number)
    }
    fn update_milestone(&self, number: i64, title: &str, description: &str, state: &str, due_on: &str) -> Result<(), GoalError> {
        self.record(format!("update-milestone {number} {title} {description} {state} {due_on}"));
        Ok(())
    }
    fn delete_milestone(&self, number: i64) -> Result<(), GoalError> {
        self.record(format!("delete-milestone {number}"));
        Ok(())
    }
    fn create_goal_issue(&self, title: &str, description: &str, milestone: Option<i64>) -> Result<String, GoalError> {
        let number = self.take_number();
        let milestone = milestone.map_or_else(|| "-".to_string(), |value| value.to_string());
        self.record(format!("create-goal-issue {number} {title} {description} {milestone}"));
        Ok(format!("{}/issues/{number}", self.repo_url))
    }
    fn update_goal_issue(&self, issue_url: &str, title: &str, description: &str) -> Result<(), GoalError> {
        self.record(format!("update-goal-issue {issue_url} {title} {description}"));
        Ok(())
    }
    fn close_issue(&self, issue_url: &str) -> Result<(), GoalError> {
        self.record(format!("close-issue {issue_url}"));
        Ok(())
    }
    fn reopen_issue(&self, issue_url: &str) -> Result<(), GoalError> {
        self.record(format!("reopen-issue {issue_url}"));
        Ok(())
    }
    fn delete_issue(&self, number: &str) -> Result<(), GoalError> {
        self.record(format!("delete-issue {number}"));
        Ok(())
    }
    fn add_sub_issue(&self, parent_issue_url: &str, child_issue_url: &str) -> Result<(), GoalError> {
        self.record(format!("add-sub-issue {parent_issue_url} {child_issue_url}"));
        Ok(())
    }
    fn repo_url(&self) -> String {
        self.repo_url.clone()
    }
}

//#endregion 🧩️Management

//#region 📡️Emission

/// 🧠️ The in-memory emitter: keeps `kind\tsource\tpayload` lines instead of reaching a coordinator.
#[derive(Debug, Default)]
pub struct MemoryEmitter {
    envelopes: RefCell<Vec<String>>,
}

impl MemoryEmitter {
    /// 🆕️ An emitter with an empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📜️ Every envelope emitted so far, in order.
    pub fn envelopes(&self) -> Vec<String> {
        self.envelopes.borrow().clone()
    }
}

impl Emitter for MemoryEmitter {
    fn emit(&self, kind: &str, source: &str, payload: &serde_json::Value) {
        self.envelopes.borrow_mut().push(format!("{kind}\t{source}\t{payload}"));
    }
}

//#endregion 📡️Emission

//#region 🔓️Lifecycle

/// 🎯️ The goal aggregate bound to its ports.
pub struct Goals<'a> {
    store: &'a dyn GoalStore,
    management: &'a dyn ManagementPort,
    emitter: &'a dyn Emitter,
    author: String,
}

impl<'a> Goals<'a> {
    /// 🆕️ Binds the aggregate to a store, a management provider, an emitter and an author alias.
    pub fn new(store: &'a dyn GoalStore, management: &'a dyn ManagementPort, emitter: &'a dyn Emitter, author: &str) -> Self {
        Self { store, management, emitter, author: author.to_string() }
    }

    /// 📋️ Every goal, identifier ascending.
    pub fn list(&self) -> Result<Vec<Goal>, GoalError> {
        self.store.ids().iter().map(|id| self.read(id)).collect()
    }

    /// 📖️ One goal by identifier, accepting the compose form.
    pub fn read(&self, id: &str) -> Result<Goal, GoalError> {
        let id = goal_id_for_filesystem(id);
        decode_goal(&id, &self.store.read(&id)?)
    }

    /// 🔎️ Every goal whose identifier, title, description or status contains the term, compared
    /// case-insensitively. A blank term keeps everything.
    pub fn search(&self, term: &str) -> Result<Vec<Goal>, GoalError> {
        let needle = term.to_lowercase();
        Ok(self
            .list()?
            .into_iter()
            .filter(|goal| {
                needle.is_empty() || format!("{} {} {} {}", goal.id, goal.title, goal.description, goal.status.as_str()).to_lowercase().contains(&needle)
            })
            .collect())
    }

    /// 🆕️ Opens a goal: validates the inputs, resolves the slug vocabularies, refuses a duplicate
    /// identifier, asks the management provider for the milestone or issue the depth calls for,
    /// persists the document and emits `goal.open.ended`.
    pub fn create(&self, input: &GoalCreateInput) -> Result<Goal, GoalError> {
        if input.title.is_empty() {
            return Err(GoalError::Missing("title"));
        }
        if input.description.is_empty() {
            return Err(GoalError::Missing("description"));
        }
        if input.prompt.is_empty() {
            return Err(GoalError::Missing("prompt"));
        }
        if input.due_date.is_empty() {
            return Err(GoalError::Missing("due date"));
        }
        if input.llm.is_empty() {
            return Err(GoalError::Missing("llm"));
        }
        if input.client.is_empty() {
            return Err(GoalError::Missing("client"));
        }
        let llm = model::resolve_allowed_llm(&input.llm)?;
        let effort = if input.effort.is_empty() { String::new() } else { model::resolve_allowed_effort(&input.effort)? };
        let client = model::resolve_allowed_client(&input.client)?;
        let id = compose_goal_id(&input.parent, &input.title);
        if self.store.exists(&id) {
            return Err(GoalError::AlreadyExists(id));
        }
        let mut goal = Goal {
            title: input.title.clone(),
            description: input.description.clone(),
            prompt: input.prompt.clone(),
            status: GoalStatus::Open,
            summary: String::new(),
            due_date: String::new(),
            dates: GoalDates { due: input.due_date.clone() },
            client,
            llm: llm.clone(),
            effort: effort.clone(),
            parent: input.parent.clone(),
            management: None,
            id: id.clone(),
            path: String::new(),
        };
        if !input.no_management {
            goal.management = Some(self.open_management(&id, input)?);
        }
        self.store.write(&id, &encode_goal(&goal)?)?;
        self.emit(
            semio_framework_repo_events::GOAL_OPEN_ENDED,
            &semio_framework_repo_events::GoalOpenPayload {
                id: goal.id.clone(),
                title: goal.title.clone(),
                description: goal.description.clone(),
                llm,
                effort,
                parent: goal.parent.clone(),
                author: self.author.clone(),
            },
        );
        Ok(goal)
    }

    fn open_management(&self, id: &str, input: &GoalCreateInput) -> Result<GoalManagementData, GoalError> {
        if is_root_goal(id) {
            if !input.milestone.is_empty() {
                return Ok(GoalManagementData { milestone: input.milestone.clone(), issue: String::new() });
            }
            let number = self.management.create_milestone(&input.title, &input.description)?;
            if !input.due_date.is_empty() {
                let _ = self.management.update_milestone(number, "", "", "", &input.due_date);
            }
            return Ok(GoalManagementData { milestone: format!("{}/milestone/{number}", self.management.repo_url()), issue: String::new() });
        }
        if is_first_gen_goal(id) {
            let milestone = self.read(&root_goal_id(id)).ok().and_then(|root| root.management).map(|data| data.milestone).filter(|value| !value.is_empty());
            let milestone = milestone.and_then(|value| parse_milestone_number(&value).ok());
            let issue = self.management.create_goal_issue(&input.title, &input.description, milestone)?;
            return Ok(GoalManagementData { milestone: String::new(), issue });
        }
        let issue = self.management.create_goal_issue(&input.title, &input.description, None)?;
        if let Ok(parent) = self.read(&parent_goal_id(id)) {
            if let Some(data) = parent.management {
                if !data.issue.is_empty() {
                    let _ = self.management.add_sub_issue(&data.issue, &issue);
                }
            }
        }
        Ok(GoalManagementData { milestone: String::new(), issue })
    }

    /// ♻️ Changes a goal: retitling moves the document, re-parenting moves it too, and the
    /// management provider is told about the new title, body, state and due date.
    pub fn change(&self, input: &GoalChangeInput) -> Result<Goal, GoalError> {
        let mut goal = self.read(&input.id)?;
        if let Some(title) = &input.title {
            self.retitle(&mut goal, title)?;
        }
        if let Some(description) = &input.description {
            goal.description = description.clone();
        }
        if let Some(due_date) = &input.due_date {
            goal.dates.due = due_date.clone();
        }
        if let Some(llm) = &input.llm {
            goal.llm = model::resolve_allowed_llm(llm)?;
        }
        if let Some(effort) = &input.effort {
            goal.effort = model::resolve_allowed_effort(effort)?;
        }
        if let Some(parent) = &input.parent {
            let slug = goal.id.rsplit('/').next().unwrap_or(&goal.id).to_string();
            let target = if parent.is_empty() { slug } else { format!("{parent}/{slug}") };
            self.relocate(&mut goal, &target)?;
            goal.parent = parent.clone();
        }
        if !input.no_management {
            self.sync_management(&goal, goal.status.as_str(), &goal.dates.due.clone())?;
        }
        self.store.write(&goal.id, &encode_goal(&goal)?)?;
        self.emit(
            semio_framework_repo_events::GOAL_CHANGE_ENDED,
            &semio_framework_repo_events::GoalChangePayload {
                id: goal.id.clone(),
                title: input.title.clone(),
                description: input.description.clone(),
                llm: input.llm.clone(),
                effort: input.effort.clone(),
                parent: input.parent.clone(),
                author: self.author.clone(),
            },
        );
        Ok(goal)
    }

    /// 📪️ Closes a goal, refusing a goal that is already closed.
    pub fn close(&self, input: &GoalCloseInput) -> Result<Goal, GoalError> {
        let mut goal = self.read(&input.id)?;
        if goal.status == GoalStatus::Closed {
            return Err(GoalError::AlreadyInState("closed"));
        }
        goal.status = GoalStatus::Closed;
        goal.summary = input.summary.clone();
        if !input.no_management {
            if let Some(data) = goal.management.clone() {
                if is_root_goal(&goal.id) && !data.milestone.is_empty() {
                    if let Ok(number) = parse_milestone_number(&data.milestone) {
                        self.management.update_milestone(number, &goal.title, &goal.description, "closed", "")?;
                    }
                } else if !is_root_goal(&goal.id) && !data.issue.is_empty() {
                    self.management.close_issue(&data.issue)?;
                }
            }
        }
        self.store.write(&goal.id, &encode_goal(&goal)?)?;
        self.emit(
            semio_framework_repo_events::GOAL_CLOSE_ENDED,
            &semio_framework_repo_events::GoalClosePayload { id: goal.id.clone(), summary: goal.summary.clone(), author: self.author.clone() },
        );
        Ok(goal)
    }

    /// 🔓️ Reopens a goal, refusing a goal that is already open, and replaces the interaction
    /// members (prompt, llm, client, effort) with the ones the reopen carries.
    pub fn reopen(&self, input: &GoalReopenInput) -> Result<Goal, GoalError> {
        let mut goal = self.read(&input.id)?;
        if goal.status == GoalStatus::Open {
            return Err(GoalError::AlreadyInState("open"));
        }
        goal.status = GoalStatus::Open;
        if let Some(title) = &input.title {
            self.retitle(&mut goal, title)?;
        }
        if let Some(description) = &input.description {
            goal.description = description.clone();
        }
        if let Some(due_date) = &input.due_date {
            goal.dates.due = due_date.clone();
        }
        if let Some(parent) = &input.parent {
            goal.parent = parent.clone();
        }
        goal.prompt = input.prompt.clone();
        goal.llm = input.llm.clone();
        goal.client = input.client.clone();
        if !input.effort.is_empty() {
            goal.effort = model::resolve_allowed_effort(&input.effort)?;
        }
        if !input.no_management {
            self.sync_management(&goal, "open", &goal.dates.due.clone())?;
        }
        self.store.write(&goal.id, &encode_goal(&goal)?)?;
        self.emit(
            semio_framework_repo_events::GOAL_REOPEN_ENDED,
            &semio_framework_repo_events::GoalReopenPayload {
                id: goal.id.clone(),
                prompt: input.prompt.clone(),
                client: input.client.clone(),
                llm: input.llm.clone(),
                effort: goal.effort.clone(),
                author: self.author.clone(),
            },
        );
        Ok(goal)
    }

    /// 🗑️ Deletes a goal and everything below it, after retiring its milestone or issue.
    pub fn delete(&self, input: &GoalDeleteInput) -> Result<bool, GoalError> {
        let goal = self.read(&input.id)?;
        if !input.no_management {
            if let Some(data) = goal.management.clone() {
                if is_root_goal(&goal.id) && !data.milestone.is_empty() {
                    if let Ok(number) = parse_milestone_number(&data.milestone) {
                        self.management.delete_milestone(number)?;
                    }
                } else if !is_root_goal(&goal.id) && !data.issue.is_empty() {
                    let number = data.issue.rsplit('/').next().unwrap_or_default().to_string();
                    self.management.delete_issue(&number)?;
                }
            }
        }
        self.store.remove(&goal.id)?;
        Ok(true)
    }

    fn sync_management(&self, goal: &Goal, state: &str, due: &str) -> Result<(), GoalError> {
        let Some(data) = goal.management.clone() else { return Ok(()) };
        if is_root_goal(&goal.id) && !data.milestone.is_empty() {
            if let Ok(number) = parse_milestone_number(&data.milestone) {
                self.management.update_milestone(number, &goal.title, &goal.description, state, due)?;
            }
        } else if !is_root_goal(&goal.id) && !data.issue.is_empty() {
            self.management.update_goal_issue(&data.issue, &goal.title, &goal.description)?;
        }
        Ok(())
    }

    fn retitle(&self, goal: &mut Goal, title: &str) -> Result<(), GoalError> {
        let title = title.trim();
        if title.is_empty() {
            return Err(GoalError::Title("goal title is required"));
        }
        let slug = identity::slugify(title);
        if title == slug {
            return Err(GoalError::Title("goal title must be titleized (e.g. \"Some Title on Something\") and NOT an all-caps slug"));
        }
        if title == slug.to_lowercase() {
            return Err(GoalError::Title("goal title must be titleized (e.g. \"Some Title on Something\") and NOT a slug"));
        }
        let target = compose_goal_id(&goal.parent, title);
        self.relocate(goal, &target)?;
        goal.title = title.to_string();
        Ok(())
    }

    fn relocate(&self, goal: &mut Goal, target: &str) -> Result<(), GoalError> {
        if target == goal.id {
            return Ok(());
        }
        if self.store.exists(target) {
            return Err(GoalError::AlreadyExists(target.to_string()));
        }
        self.store.rename(&goal.id, target)?;
        goal.id = target.to_string();
        Ok(())
    }

    fn emit<T: serde::Serialize>(&self, kind: &str, payload: &T) {
        let Ok(value) = serde_json::to_value(payload) else { return };
        self.emitter.emit(kind, "repo-cli", &value);
    }
}

//#endregion 🔓️Lifecycle

//#region 🌳️Tree

/// 🌱️ The flat goal record the tree is assembled from.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GoalSeed {
    /// 🪪️ The `GOAL/SUBGOAL` identifier.
    #[serde(default)]
    pub id: String,
    /// 🔤️ The human title.
    #[serde(default)]
    pub title: String,
    /// 🚦️ `open` or `closed`.
    #[serde(default)]
    pub status: String,
    /// 📅️ The due date the tree sorts by.
    #[serde(rename = "dueDate", default)]
    pub due_date: String,
    /// 🕰️ When the goal was created.
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    /// 📝️ The description.
    #[serde(default)]
    pub description: String,
}

/// 🎫️ The flat ticket record the tree is assembled from.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TicketSeed {
    /// 🪪️ The ticket identifier.
    #[serde(default)]
    pub id: String,
    /// 🔤️ The ticket slug.
    #[serde(default)]
    pub slug: String,
    /// 🚦️ `open` or `closed`.
    #[serde(default)]
    pub status: String,
    /// 🔤️ The human title.
    #[serde(default)]
    pub title: String,
    /// 🎯️ The goal the ticket hangs under, empty for none.
    #[serde(default)]
    pub goal: String,
    /// 🧬️ The parent ticket, empty for a root ticket.
    #[serde(default)]
    pub parent: String,
    /// 📝️ The prompt the ticket was opened with.
    #[serde(default)]
    pub prompt: String,
    /// 📄️ The closing summary.
    #[serde(default)]
    pub summary: String,
    /// 🕰️ When the ticket was created.
    #[serde(default)]
    pub created: String,
    /// 🏁️ When the ticket was finished.
    #[serde(default)]
    pub finished: String,
}

/// 🖨️ How a rendered tree lays its lines out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeFormat {
    /// 🌲️ Box-drawing connectors, one indentation step per level.
    Text,
    /// 📝️ Markdown list items, two spaces per level.
    Markdown,
}

/// 🖋️ Renders the content of one tree line. Entity rendering is `🪪️identity`'s concern, so the
/// tree takes the line renderer as a port instead of owning a presentation vocabulary.
pub trait TreeLines {
    /// 🎯️ The line of a goal node.
    fn goal(&self, node: &GoalNode) -> String;
    /// 🎫️ The line of a ticket node.
    fn ticket(&self, node: &TicketNode) -> String;
}

/// 🔤️ The renderer that states the identifying members and nothing else, so a rendering scenario
/// judges the shape of the tree rather than a presentation vocabulary.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlainTreeLines;

impl TreeLines for PlainTreeLines {
    fn goal(&self, node: &GoalNode) -> String {
        format!("goal {} {} [{}] {}", node.id, node.title, node.status, node.due_date)
    }

    fn ticket(&self, node: &TicketNode) -> String {
        format!("ticket {} {} [{}] {}", node.slug, node.title, node.status, node.uri)
    }
}

/// 🌳️ Assembles the goal forest: every goal hangs under the goal its identifier prefix names, the
/// siblings sort by due date and then identifier, every ticket hangs under its goal and then under
/// its parent ticket, and the tickets that reference no known goal end up under a trailing
/// `No Goal` node.
pub fn build_goal_tree(goals: &[GoalSeed], tickets: &[TicketSeed]) -> Vec<GoalNode> {
    let mut nodes: BTreeMap<String, GoalNode> = BTreeMap::new();
    for seed in goals {
        nodes.insert(
            seed.id.clone(),
            GoalNode {
                id: seed.id.clone(),
                title: seed.title.clone(),
                status: seed.status.clone(),
                due_date: seed.due_date.clone(),
                created_at: seed.created_at.clone(),
                description: seed.description.clone(),
                children: None,
                tickets: None,
            },
        );
    }

    let mut ticket_nodes: Vec<TicketNode> = Vec::new();
    for seed in tickets {
        ticket_nodes.push(TicketNode {
            id: seed.id.clone(),
            slug: seed.slug.clone(),
            status: seed.status.clone(),
            title: seed.title.clone(),
            uri: format!("repo://ticket/{}{}", identity::emoji_text(identity::entity("ticket")), identity::flat(&seed.slug)),
            goal_id: seed.goal.clone(),
            parent_id: seed.parent.clone(),
            children: None,
            created: seed.created.clone(),
            finished: seed.finished.clone(),
            description: seed.prompt.clone(),
            summary: seed.summary.clone(),
        });
    }

    let known: Vec<String> = nodes.keys().cloned().collect();
    let mut per_goal: BTreeMap<String, Vec<TicketNode>> = BTreeMap::new();
    let mut orphans: Vec<TicketNode> = Vec::new();
    for node in ticket_nodes {
        if !node.goal_id.is_empty() && known.contains(&node.goal_id) {
            per_goal.entry(node.goal_id.clone()).or_default().push(node);
        } else {
            orphans.push(node);
        }
    }
    for (id, owned) in per_goal {
        if let Some(node) = nodes.get_mut(&id) {
            node.tickets = Some(nest_tickets(&owned));
        }
    }

    let mut child_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut root_ids: Vec<String> = Vec::new();
    for id in &known {
        let parent = match id.rfind('/') {
            Some(index) => id[..index].to_string(),
            None => String::new(),
        };
        if !parent.is_empty() && &parent != id && known.contains(&parent) {
            child_ids.entry(parent).or_default().push(id.clone());
        } else {
            root_ids.push(id.clone());
        }
    }
    let mut roots: Vec<GoalNode> = root_ids.iter().filter_map(|id| assemble_goal_node(id, &nodes, &child_ids)).collect();
    sort_goal_nodes(&mut roots);

    if !orphans.is_empty() {
        roots.push(GoalNode {
            id: String::new(),
            title: "No Goal".to_string(),
            status: String::new(),
            due_date: String::new(),
            created_at: String::new(),
            description: String::new(),
            children: None,
            tickets: Some(nest_tickets(&orphans)),
        });
    }
    roots
}

fn assemble_goal_node(id: &str, nodes: &BTreeMap<String, GoalNode>, child_ids: &BTreeMap<String, Vec<String>>) -> Option<GoalNode> {
    let mut node = nodes.get(id)?.clone();
    let children: Vec<GoalNode> = child_ids
        .get(id)
        .map(|ids| ids.iter().filter_map(|child| assemble_goal_node(child, nodes, child_ids)).collect())
        .unwrap_or_default();
    node.children = if children.is_empty() { None } else { Some(children) };
    Some(node)
}

fn nest_tickets(nodes: &[TicketNode]) -> Vec<TicketNode> {
    let ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let by_id: BTreeMap<String, TicketNode> = nodes.iter().map(|node| (node.id.clone(), node.clone())).collect();
    let mut child_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut root_ids: Vec<String> = Vec::new();
    for node in nodes {
        if !node.parent_id.is_empty() && ids.contains(&node.parent_id) && node.parent_id != node.id {
            child_ids.entry(node.parent_id.clone()).or_default().push(node.id.clone());
        } else {
            root_ids.push(node.id.clone());
        }
    }
    root_ids.iter().filter_map(|id| assemble_ticket_node(id, &by_id, &child_ids)).collect()
}

fn assemble_ticket_node(id: &str, nodes: &BTreeMap<String, TicketNode>, child_ids: &BTreeMap<String, Vec<String>>) -> Option<TicketNode> {
    let mut node = nodes.get(id)?.clone();
    let children: Vec<TicketNode> = child_ids
        .get(id)
        .map(|ids| ids.iter().filter_map(|child| assemble_ticket_node(child, nodes, child_ids)).collect())
        .unwrap_or_default();
    node.children = if children.is_empty() { None } else { Some(children) };
    Some(node)
}

fn sort_goal_nodes(nodes: &mut [GoalNode]) {
    nodes.sort_by(|left, right| {
        if left.due_date != right.due_date {
            if left.due_date.is_empty() {
                return std::cmp::Ordering::Greater;
            }
            if right.due_date.is_empty() {
                return std::cmp::Ordering::Less;
            }
            return left.due_date.cmp(&right.due_date);
        }
        left.id.cmp(&right.id)
    });
    for node in nodes.iter_mut() {
        if let Some(children) = node.children.as_mut() {
            sort_goal_nodes(children);
        }
    }
}

/// 🔢️ How many open goals hang below a node, at any depth.
pub fn count_open_subgoals(node: &GoalNode) -> usize {
    node.children
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|child| usize::from(child.status == "open") + count_open_subgoals(child))
        .sum()
}

/// 🔢️ How many open tickets hang below a node, at any depth, through goals and tickets alike.
pub fn count_open_tickets(node: &GoalNode) -> usize {
    fn tickets(nodes: &[TicketNode]) -> usize {
        nodes.iter().map(|node| usize::from(node.status == "open") + tickets(node.children.as_deref().unwrap_or_default())).sum()
    }
    tickets(node.tickets.as_deref().unwrap_or_default())
        + node.children.as_deref().unwrap_or_default().iter().map(count_open_tickets).sum::<usize>()
}

/// 🎨️ Renders a goal forest, goals first and their tickets after them at every level.
pub fn render_goal_tree(roots: &[GoalNode], format: TreeFormat, lines: &dyn TreeLines) -> String {
    let mut out = String::new();
    let last = roots.len().saturating_sub(1);
    for (index, root) in roots.iter().enumerate() {
        render_goal_node(root, "", index == last, true, format, lines, &mut out);
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn render_goal_node(node: &GoalNode, prefix: &str, is_last: bool, is_root: bool, format: TreeFormat, lines: &dyn TreeLines, out: &mut String) {
    let content = lines.goal(node);
    let children = node.children.as_deref().unwrap_or_default();
    let tickets = node.tickets.as_deref().unwrap_or_default();
    let total = children.len() + tickets.len();
    let next = write_line(out, prefix, &content, is_last, is_root, format);
    for (index, child) in children.iter().enumerate() {
        render_goal_node(child, &next, index == total - 1, false, format, lines, out);
    }
    for (index, ticket) in tickets.iter().enumerate() {
        render_ticket_node(ticket, &next, children.len() + index == total - 1, format, lines, out);
    }
}

fn render_ticket_node(node: &TicketNode, prefix: &str, is_last: bool, format: TreeFormat, lines: &dyn TreeLines, out: &mut String) {
    let content = lines.ticket(node);
    let children = node.children.as_deref().unwrap_or_default();
    let next = write_line(out, prefix, &content, is_last, false, format);
    let last = children.len().saturating_sub(1);
    for (index, child) in children.iter().enumerate() {
        render_ticket_node(child, &next, index == last, format, lines, out);
    }
}

fn write_line(out: &mut String, prefix: &str, content: &str, is_last: bool, is_root: bool, format: TreeFormat) -> String {
    match format {
        TreeFormat::Markdown => {
            out.push_str(prefix);
            out.push_str("- ");
            out.push_str(content);
            out.push('\n');
            format!("{prefix}  ")
        }
        TreeFormat::Text => {
            let connector = if is_root {
                ""
            } else if is_last {
                "└️─️─️ "
            } else {
                "├️─️─️ "
            };
            out.push_str(prefix);
            out.push_str(connector);
            out.push_str(content);
            out.push('\n');
            if is_root {
                prefix.to_string()
            } else if is_last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│️   ")
            }
        }
    }
}

//#endregion 🌳️Tree
