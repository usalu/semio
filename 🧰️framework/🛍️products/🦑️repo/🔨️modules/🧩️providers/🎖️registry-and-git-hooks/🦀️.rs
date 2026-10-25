//! 🎖️ Registry and git hooks: the provider registry and the repository-managed git hook files.
//!
//! Holds the `🎖️ProviderRegistry`, `🪝️GitHookFiles` regions of the crate root; the root re-exports its items.

use super::*;

// #region 🎖️ProviderRegistry
/// 🔌️ Every registered agent-client provider, in registration order.
pub fn all_agent_client_providers() -> Vec<AgentClientProviders> {
    vec![
        AgentClientProviders::from(CopilotAgentClientProvider),
        AgentClientProviders::from(CursorAgentClientProvider),
        AgentClientProviders::from(WindsurfAgentClientProvider),
        AgentClientProviders::from(ClaudeCodeAgentClientProvider),
        AgentClientProviders::from(DroidAgentClientProvider),
        AgentClientProviders::from(CodexAgentClientProvider),
        AgentClientProviders::from(AntigravityAgentClientProvider),
        AgentClientProviders::from(KiroAgentClientProvider),
    ]
}

/// 💻️ The agent-client provider for a client slug.
pub fn get_agent_client_provider(client: &str) -> Option<AgentClientProviders> {
    all_agent_client_providers().into_iter().find(|provider| provider.kind() == client)
}

/// 🐙️ The default management provider (GitHub) bound to a runner.
pub fn default_management_provider(runner: ProcessRunners) -> ManagementProviders {
    ManagementProviders::from(GitHubManagementProvider::new(runner))
}

/// 📌️ The default version control provider (git) bound to a runner.
pub fn default_version_control_provider(runner: ProcessRunners) -> VersionControlProviders {
    VersionControlProviders::from(GitVersionControlProvider::new(runner))
}

/// 🐳️ The default sandbox provider (devcontainer).
pub fn default_sandbox_provider() -> SandboxProviders {
    SandboxProviders::from(DevcontainerSandboxProvider)
}

/// 🖥️ A management provider bound to the real machine.
pub fn system_management_provider() -> ManagementProviders {
    default_management_provider(ProcessRunners::from(SystemProcessRunner::new()))
}

/// ⚙️ A version control provider bound to the real machine.
pub fn system_version_control_provider() -> VersionControlProviders {
    default_version_control_provider(ProcessRunners::from(SystemProcessRunner::new()))
}
// #endregion 🎖️ProviderRegistry

// #region 🪝️GitHookFiles

/// 🪝️ Every git hook filename this repository may install and must never leave enabled.
pub const REPO_MANAGED_GIT_HOOKS: &[&str] = &["pre-commit", "post-commit", "prepare-commit-msg", "commit-msg", "pre-push", "pre-rebase", "post-merge", "post-checkout", "post-rewrite"];

/// 🔄️ The hooks the micro-commit workflow installs, in installation order.
pub const MICRO_COMMIT_GIT_HOOKS: &[&str] = &["prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"];

/// 📁️ The checked-in hook scripts the micro-commit workflow copies from.
pub fn micro_commit_hook_source_dir(repo_root: &std::path::Path) -> std::path::PathBuf {
    repo_root.join("🧰️framework").join("🛍️products").join("🦑️repo").join("🪝️hooks")
}

/// 🧹️ Deletes the repo-managed git hooks so commits, rebases and squashes stay unblocked, and
/// answers the names it actually removed in [`REPO_MANAGED_GIT_HOOKS`] order.
///
/// Twin of `RemoveGitHooks` in `🪝️hooks/📦️packages/🐹️go/🐹️.go`: an absent hook is skipped, and an
/// empty root removes nothing.
pub fn remove_git_hooks(repo_root: &std::path::Path) -> ProviderResult<Vec<String>> {
    if repo_root.as_os_str().is_empty() {
        return Ok(Vec::new());
    }
    let hooks_dir = repo_root.join(".git").join("hooks");
    let mut removed = Vec::new();
    for name in REPO_MANAGED_GIT_HOOKS {
        let path = hooks_dir.join(name);
        match std::fs::remove_file(&path) {
            Ok(()) => removed.push((*name).to_string()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(ProviderError::new(format!("remove git hook {name}: {error}"))),
        }
    }
    Ok(removed)
}

/// 🎣️ Copies the checked-in micro-commit hook scripts into `.git/hooks`.
///
/// Twin of `InstallMicroCommitHooks`: an empty root installs nothing, and a missing source script
/// is the refusal `read micro-commit hook <name>: <error>`.
pub fn install_micro_commit_hooks(repo_root: &std::path::Path) -> ProviderResult<()> {
    if repo_root.as_os_str().is_empty() {
        return Ok(());
    }
    let source_dir = micro_commit_hook_source_dir(repo_root);
    let hooks_dir = repo_root.join(".git").join("hooks");
    for name in MICRO_COMMIT_GIT_HOOKS {
        let body = std::fs::read(source_dir.join(name)).map_err(|error| ProviderError::new(format!("read micro-commit hook {name}: {error}")))?;
        let target = hooks_dir.join(name);
        std::fs::write(&target, &body).map_err(|error| ProviderError::new(format!("install micro-commit hook {name}: {error}")))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o755)).map_err(|error| ProviderError::new(format!("install micro-commit hook {name}: {error}")))?;
        }
    }
    Ok(())
}

// #endregion 🪝️GitHookFiles
