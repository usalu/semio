//! 🎥️ The repo context port: the failure type and the `RepoContext` trait every repo aggregate is reached through.

use super::*;

//#region 🎥️ContextPort

/// ❌️ Every failure a context operation can report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextError {
    /// 🗨️ The failure text, reproduced verbatim into the execution error chain.
    pub message: String,
}

impl ContextError {
    /// 🔨️ Builds a context failure from any displayable value.
    pub fn new(message: impl Into<String>) -> ContextError {
        ContextError { message: message.into() }
    }
}

impl fmt::Display for ContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ContextError {}

/// 🔌️ The port every repo aggregate is reached through.
///
/// The reference implementation declares this interface next to the domain model, because its
/// executor lives in the same package as every aggregate. Here the port sits in the module that
/// *needs* it: `📐️model` is at the bottom of the dependency DAG and no crate below `🔗️graphql`
/// ever calls a context, so placing the trait here keeps the DAG acyclic without a second
/// abstraction. Both placements are behaviourally identical — the method set is the same.
pub trait RepoContext {
    /// 📁️ The absolute repository root every file URI is built against.
    fn root_dir(&self) -> String;
    /// 📜️ Every technology of the repository.
    fn technologies(&self) -> Vec<Technology>;
    /// 🔵️ Every bundle of the repository.
    fn bundles(&self) -> Vec<Bundle>;
    /// ✔️ The most recent checkpoints, newest first, capped by `limit` when given.
    fn checkpoints(&self, limit: Option<i64>) -> Result<Vec<Checkpoint>, ContextError>;
    /// 🩷️ Every folder of the repository.
    fn folders(&self) -> Vec<Folder>;
    /// 📄️ Every file of the repository.
    fn files(&self) -> Vec<File>;
    /// 💗️ Every section of the repository.
    fn sections(&self) -> Vec<Section>;
    /// 💕️ Every definition of the repository.
    fn definitions(&self) -> Vec<Definition>;
    /// 🧑️‍💻️ Every contributor of the repository.
    fn contributors(&self) -> Result<Vec<Contributor>, ContextError>;
    /// 🎯️ Every goal of the repository.
    fn goals(&self) -> Result<Vec<Goal>, ContextError>;
    /// 🎫️ Every ticket matching the date and status filters.
    fn tickets(&self, year: Option<i64>, month: Option<i64>, day: Option<i64>, status: Option<TicketStatus>) -> Result<Vec<Ticket>, ContextError>;
    /// 👮️ Every policy of the repository.
    fn policies(&self) -> Vec<Policy>;
    /// 📝️ Every draft of the repository.
    fn drafts(&self) -> Result<Vec<Draft>, ContextError>;
    /// ✅️ Every todo matching the filter.
    fn todos(&self, filter: Option<&FilterInput>) -> Result<Vec<Todo>, ContextError>;
    /// ⚖️ Every declared statute.
    fn statutes(&self) -> Vec<StatuteMeta>;
    /// 💬️ Every recorded interaction, flattened with its source artifact.
    fn interactions(&self) -> Result<Vec<InteractionResource>, ContextError>;
    /// 🔬️ Runs the analyzer over an optional scope.
    fn analyze(&self, scope: Option<&str>) -> Result<AnalyzeResult, ContextError>;
    /// 🩹️ Applies every autofix to the files of an optional scope and reports what is left.
    fn fix(&self, scope: Option<&str>) -> Result<FixResult, ContextError>;
    /// 📩️ Creates a goal.
    fn goal_create(&self, input: GoalCreateInput) -> Result<Goal, ContextError>;
    /// 📐️ Changes a goal.
    fn goal_change(&self, input: GoalChangeInput) -> Result<Goal, ContextError>;
    /// 🏁️ Closes a goal.
    fn goal_close(&self, input: GoalCloseInput) -> Result<Goal, ContextError>;
    /// ⛳️ Reopens a goal.
    fn goal_reopen(&self, input: GoalReopenInput) -> Result<Goal, ContextError>;
    /// 🔸️ Deletes a goal.
    fn goal_delete(&self, input: GoalDeleteInput) -> Result<bool, ContextError>;
    /// 🆕️ Creates a todo.
    fn todo_create(&self, input: TodoCreateInput) -> Result<Todo, ContextError>;
    /// ✏️ Changes a todo.
    fn todo_change(&self, input: TodoChangeInput) -> Result<Todo, ContextError>;
    /// 🔹️ Deletes a todo.
    fn todo_delete(&self, id: &str) -> Result<bool, ContextError>;
    /// 📙️ Creates a draft.
    fn draft_create(&self, input: DraftCreateInput) -> Result<Draft, ContextError>;
    /// 🗑️ Deletes a draft.
    fn draft_delete(&self, id: &str) -> Result<bool, ContextError>;
    /// 📬️ Opens a ticket.
    fn ticket_open(&self, input: TicketOpenInput) -> Result<Ticket, ContextError>;
    /// 📪️ Closes a ticket.
    fn ticket_close(&self, input: TicketCloseInput) -> Result<Ticket, ContextError>;
    /// 🔓️ Reopens a ticket.
    fn ticket_reopen(&self, input: TicketReopenInput) -> Result<Ticket, ContextError>;
    /// ♻️ Changes a ticket.
    fn ticket_change(&self, input: TicketChangeInput) -> Result<Ticket, ContextError>;
    /// 🔺️ Deletes a ticket.
    fn ticket_delete(&self, input: TicketDeleteInput) -> Result<bool, ContextError>;
    /// ➕️ Adds a contributor.
    fn contributor_add(&self, input: ContributorAddInput) -> Result<Contributor, ContextError>;
    /// 🚚️ Removes a contributor.
    fn contributor_remove(&self, github: &str) -> Result<(), ContextError>;
    /// 🗂️ Creates a folder.
    fn folder_create(&self, path: &str) -> Result<Folder, ContextError>;
    /// ♟️ Moves a folder.
    fn folder_move(&self, source: &str, destination: &str) -> Result<Folder, ContextError>;
    /// 🔻️ Deletes a folder.
    fn folder_delete(&self, path: &str) -> Result<(), ContextError>;
    /// ⬛️ Creates a file.
    fn file_create(&self, path: &str) -> Result<File, ContextError>;
    /// ⬜️ Moves a file.
    fn file_move(&self, source: &str, destination: &str) -> Result<File, ContextError>;
    /// 🟥️ Deletes a file.
    fn file_delete(&self, path: &str) -> Result<(), ContextError>;
    /// 📑️ Creates a section.
    fn section_create(&self, file: &str, name: &str, parent: Option<&str>) -> Result<Section, ContextError>;
    /// 🟧️ Moves a section.
    fn section_move(&self, file: &str, old_name: &str, new_name: &str) -> Result<Section, ContextError>;
    /// 🟨️ Deletes a section.
    fn section_delete(&self, file: &str, name: &str) -> Result<(), ContextError>;
    /// 🧬️ Integrates a source into a target section.
    fn integrate(&self, source: Option<&str>, target_section: Option<&str>, target_file: Option<&str>, target_parent: Option<&str>) -> Result<File, ContextError>;
    /// 🧲️ Extracts a section into a target file.
    fn extract(&self, source_file: Option<&str>, source_section: Option<&str>, target_file: Option<&str>) -> Result<File, ContextError>;
    /// 🐙️ Synchronises the management provider.
    fn sync_management(&self) -> Result<bool, ContextError>;
}

//#endregion 🎥️ContextPort
