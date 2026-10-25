//! 📡️ Emitter and management of the production context: the coordinator event emitter and the provider-backed goal management. See [`super`].

use super::*;

//#region 🗄️FsRepoContext
/// 🦑️ The emitter that forwards an aggregate event to the repository coordinator.
#[derive(Debug, Clone, Copy, Default)]
pub struct CoordinatorEmitter;

impl semio_framework_repo_events::Emitter for CoordinatorEmitter {
    fn emit(&self, kind: &str, source: &str, payload: &Json) {
        semio_framework_repo_events::emit(kind, source, payload);
    }
}

/// 🐙️ The goal management port answered by the repository's management provider.
pub struct ProviderManagement {
    pub(super) provider: ManagementProviders,
    pub(super) repo_url: String,
}

impl ProviderManagement {
    /// 🆕️ Binds the provider the machine is configured for and the repository URL milestones hang off.
    pub fn system() -> ProviderManagement {
        ProviderManagement { provider: semio_framework_repo_providers::system_management_provider(), repo_url: VersionControlProvider::repo_url(&semio_framework_repo_providers::system_version_control_provider()).unwrap_or_default() }
    }
}

impl semio_framework_repo_goals::ManagementPort for ProviderManagement {
    fn create_milestone(&self, title: &str, description: &str) -> Result<i64, GoalError> {
        ManagementProvider::create_milestone(&self.provider, title, description).map_err(|error| GoalError::Port(error.message))
    }
    fn update_milestone(&self, number: i64, title: &str, description: &str, state: &str, due_on: &str) -> Result<(), GoalError> {
        ManagementProvider::update_milestone(&self.provider, number, title, description, state, due_on).map_err(|error| GoalError::Port(error.message))
    }
    fn delete_milestone(&self, number: i64) -> Result<(), GoalError> {
        ManagementProvider::delete_milestone(&self.provider, number).map_err(|error| GoalError::Port(error.message))
    }
    fn create_goal_issue(&self, title: &str, description: &str, milestone: Option<i64>) -> Result<String, GoalError> {
        ManagementProvider::create_goal_issue(&self.provider, title, description, milestone).map_err(|error| GoalError::Port(error.message))
    }
    fn update_goal_issue(&self, issue_url: &str, title: &str, description: &str) -> Result<(), GoalError> {
        ManagementProvider::update_goal_issue(&self.provider, issue_url, title, description).map_err(|error| GoalError::Port(error.message))
    }
    fn close_issue(&self, issue_url: &str) -> Result<(), GoalError> {
        ManagementProvider::close_issue(&self.provider, issue_url).map_err(|error| GoalError::Port(error.message))
    }
    fn reopen_issue(&self, issue_url: &str) -> Result<(), GoalError> {
        ManagementProvider::reopen_issue(&self.provider, issue_url).map_err(|error| GoalError::Port(error.message))
    }
    fn delete_issue(&self, number: &str) -> Result<(), GoalError> {
        ManagementProvider::delete_issue(&self.provider, number).map_err(|error| GoalError::Port(error.message))
    }
    fn add_sub_issue(&self, parent_issue_url: &str, child_issue_url: &str) -> Result<(), GoalError> {
        ManagementProvider::add_sub_issue(&self.provider, parent_issue_url, child_issue_url).map_err(|error| GoalError::Port(error.message))
    }
    fn repo_url(&self) -> String {
        self.repo_url.clone()
    }
}
//#endregion 🗄️FsRepoContext
