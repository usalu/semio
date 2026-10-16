//! 🔭️ Provider interfaces: the management, version control, sandbox and editor provider traits plus the downward ports.
//!
//! Holds the `🔭️ProviderInterfaces`, `🔌️Ports` regions of the crate root; the root re-exports its items.

use super::*;

// #region 🔭️ProviderInterfaces
/// 💿️ A management issue record.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementIssue {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub milestone: Option<ManagementIssueMilestone>,
    #[serde(default)]
    pub labels: Vec<ManagementLabel>,
}

/// 🎯️ The milestone an issue carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementIssueMilestone {
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub title: String,
}

/// 🎯️ A management milestone record. Field names mirror the GitHub REST payload the `gh` CLI relays.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementMilestone {
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub url: String,
    #[serde(default, rename = "due_on")]
    pub due_on: String,
    #[serde(default)]
    pub state: String,
}

/// 🏷️ A management label record.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementLabel {
    #[serde(default)]
    pub name: String,
}

/// 🐙️ Issue and milestone management (GitHub, Jira, Trello, Linear, …).
#[dyn_enum]
pub trait ManagementProvider {
    fn kind(&self) -> &'static str;
    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> ProviderResult<String>;
    fn close_issue(&self, issue_url: &str) -> ProviderResult<()>;
    fn reopen_issue(&self, issue_url: &str) -> ProviderResult<()>;
    fn delete_issue(&self, issue_url_or_number: &str) -> ProviderResult<()>;
    fn update_issue_title(&self, issue_url: &str, title: &str) -> ProviderResult<()>;
    fn update_issue_body(&self, issue_url: &str, body: &str) -> ProviderResult<()>;
    fn get_issue_details(&self, issue_url: &str) -> ProviderResult<Option<ManagementIssue>>;
    fn get_issue_node_id(&self, issue_url: &str) -> ProviderResult<String>;
    fn get_issue_parent_url(&self, issue_url: &str) -> ProviderResult<String>;
    fn add_comment(&self, issue_url: &str, comment: &str) -> ProviderResult<()>;
    fn add_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()>;
    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()>;
    fn add_issue_to_project(&self, issue_url: &str);
    fn assign_issue_to_current_user(&self, issue_url: &str);
    fn add_sub_issue(&self, parent_issue_url: &str, child_issue_url: &str) -> ProviderResult<()>;
    fn update_issue_milestone(&self, issue_url: &str, milestone_title: &str) -> ProviderResult<()>;
    fn clear_issue_milestone(&self, issue_url: &str) -> ProviderResult<()>;
    fn create_milestone(&self, title: &str, description: &str) -> ProviderResult<i64>;
    fn update_milestone(&self, number: i64, title: &str, description: &str, state: &str, due_on: &str) -> ProviderResult<()>;
    fn delete_milestone(&self, number: i64) -> ProviderResult<()>;
    fn get_milestone(&self, number: i64) -> ProviderResult<Option<ManagementMilestone>>;
    fn get_milestone_title(&self, number: i64) -> ProviderResult<String>;
    fn find_milestone_by_title(&self, title: &str) -> ProviderResult<Option<ManagementMilestone>>;
    fn list_issues_for_label_sync(&self) -> ProviderResult<Vec<ManagementIssue>>;
    fn list_open_issues_with_label(&self, label: &str) -> ProviderResult<Vec<String>>;
    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>>;
    fn create_repo_label(&self, name: &str) -> ProviderResult<()>;
    fn delete_repo_label(&self, name: &str) -> ProviderResult<()>;
    fn sync_repo_label_catalog(&self, valid_labels: &[String]) -> ProviderResult<()>;
    fn create_goal_issue(&self, title: &str, description: &str, milestone: Option<i64>) -> ProviderResult<String>;
    fn update_goal_issue(&self, issue_url: &str, title: &str, description: &str) -> ProviderResult<()>;
    fn get_current_user(&self) -> String;
}

/// 🔌️ Version control operations (git, …).
#[dyn_enum]
pub trait VersionControlProvider {
    fn kind(&self) -> &'static str;
    fn repo_url(&self) -> ProviderResult<String>;
    fn checkpoint(&self, repo_root: &str, description: &str) -> ProviderResult<String>;
    fn current_checkpoint(&self, repo_root: &str) -> ProviderResult<String>;
    fn checkin(&self, repo_root: &str, contributor: &str) -> ProviderResult<()>;
    fn checkout(&self, repo_root: &str, contributor: &str, description: &str) -> ProviderResult<String>;
    fn current_branch(&self, repo_root: &str) -> ProviderResult<String>;
    fn staged_files(&self, repo_root: &str) -> ProviderResult<Vec<String>>;
    fn stage_all(&self, repo_root: &str) -> ProviderResult<()>;
}

/// 🐳️ Sandbox and container operations (devcontainer, podman, …).
#[dyn_enum]
pub trait SandboxProvider {
    fn kind(&self) -> &'static str;
}

/// 📝️ Editor and agent integrations.
#[dyn_enum]
pub trait EditorProvider {
    fn kind(&self) -> &'static str;
    fn resolve_native_event(&self, native_event: &str, tool_kind: ToolKind) -> ProviderResult<(HookEvent, String)>;
    fn format_hook_output(&self, hook_event_name: &str, result: &HookResult) -> String;
    fn native_event_from_hook_event(&self, event: HookEvent, parent_info: &str) -> String;
}
// #endregion 🔭️ProviderInterfaces

// #region 🔌️Ports
/// 🪝️ The formatting half of an editor, declared here so `🔨️modules/🪝️hooks` depends downward on
/// `🧩️providers` instead of the other way round — fix 5 of `📓️go-region-dependency-graph.md`.
pub trait HookFormatter {
    /// 🧾️ Renders a hook result for one native hook event name.
    fn format_hook_output(&self, hook_event_name: &str, result: &HookResult) -> String;
    /// 🔤️ The native event name an IDE expects for a neutral hook event.
    fn native_event_from_hook_event(&self, event: HookEvent, parent_info: &str) -> String;
}

impl HookFormatter for EditorProviders {
    fn format_hook_output(&self, hook_event_name: &str, result: &HookResult) -> String {
        EditorProvider::format_hook_output(self, hook_event_name, result)
    }

    fn native_event_from_hook_event(&self, event: HookEvent, parent_info: &str) -> String {
        EditorProvider::native_event_from_hook_event(self, event, parent_info)
    }
}

/// 🎫️ The narrow issue surface `🔨️modules/🎫️tickets` needs. Declaring it here breaks the
/// `providers → tickets` edge: the GitHub DTOs live with the provider that parses them, and tickets
/// consumes this port — fix 5 of `📓️go-region-dependency-graph.md`.
pub trait IssueTracker {
    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> ProviderResult<String>;
    fn close_issue(&self, issue_url: &str) -> ProviderResult<()>;
    fn reopen_issue(&self, issue_url: &str) -> ProviderResult<()>;
    fn get_issue_details(&self, issue_url: &str) -> ProviderResult<Option<ManagementIssue>>;
    fn add_comment(&self, issue_url: &str, comment: &str) -> ProviderResult<()>;
    fn add_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()>;
    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()>;
    fn find_milestone_by_title(&self, title: &str) -> ProviderResult<Option<ManagementMilestone>>;
    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>>;
}

impl IssueTracker for ManagementProviders {
    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> ProviderResult<String> {
        ManagementProvider::create_issue(self, title, body, milestone)
    }
    fn close_issue(&self, issue_url: &str) -> ProviderResult<()> {
        ManagementProvider::close_issue(self, issue_url)
    }
    fn reopen_issue(&self, issue_url: &str) -> ProviderResult<()> {
        ManagementProvider::reopen_issue(self, issue_url)
    }
    fn get_issue_details(&self, issue_url: &str) -> ProviderResult<Option<ManagementIssue>> {
        ManagementProvider::get_issue_details(self, issue_url)
    }
    fn add_comment(&self, issue_url: &str, comment: &str) -> ProviderResult<()> {
        ManagementProvider::add_comment(self, issue_url, comment)
    }
    fn add_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()> {
        ManagementProvider::add_labels(self, issue_url, labels)
    }
    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()> {
        ManagementProvider::remove_labels(self, issue_url, labels)
    }
    fn find_milestone_by_title(&self, title: &str) -> ProviderResult<Option<ManagementMilestone>> {
        ManagementProvider::find_milestone_by_title(self, title)
    }
    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>> {
        ManagementProvider::list_repo_labels(self)
    }
}
// #endregion 🔌️Ports
