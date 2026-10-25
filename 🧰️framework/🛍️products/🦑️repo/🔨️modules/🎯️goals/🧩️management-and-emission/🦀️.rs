//! 🧩️ Management and emission of the goals: the management port with its null and recording implementations and the memory emitter. See [`super`].

use super::*;

//#region 🧩️Management
/// ⛳️ The provider that owns milestones and issues for the goals of this repository.
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
    /// 🐛️ Deletes an issue by its number.
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
    pub(super) repo_url: String,
    pub(super) next: RefCell<i64>,
    calls: RefCell<Vec<String>>,
}

impl RecordingManagement {
    /// 🔢️ A provider that numbers milestones and issues from `first`.
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
    /// 🪵️ An emitter with an empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📩️ Every envelope emitted so far, in order.
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
