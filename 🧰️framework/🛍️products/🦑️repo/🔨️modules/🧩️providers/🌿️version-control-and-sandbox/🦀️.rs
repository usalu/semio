//! 🎛️ Version control and sandbox: the git version control provider and the devcontainer sandbox provider.
//!
//! Holds the `🌿️GitVersionControlProvider`, `⛑️DevcontainerSandboxProvider` regions of the crate root; the root re-exports its items.

use super::*;

// #region 🌿️GitVersionControlProvider
/// 🌿️ Version control through the `git` CLI.
pub struct GitVersionControlProvider {
    pub(super) runner: ProcessRunners,
}

impl GitVersionControlProvider {
    /// 🆕️ Binds the provider to a process runner.
    pub fn new(runner: ProcessRunners) -> GitVersionControlProvider {
        GitVersionControlProvider { runner }
    }

    /// 📜️ Every argv this provider issued, in order.
    pub fn issued(&self) -> Vec<Vec<String>> {
        self.runner.issued()
    }

    fn git(&self, args: &[&str], repo_root: &str) -> ProcessOutcome {
        let owned: Vec<String> = args.iter().map(|arg| (*arg).to_string()).collect();
        self.runner.run(&ProcessRequest::new("git", &owned, repo_root))
    }
}

impl VersionControlProvider for GitVersionControlProvider {
    fn kind(&self) -> &'static str {
        "git"
    }

    fn repo_url(&self) -> ProviderResult<String> {
        let outcome = self.runner.run(&ProcessRequest::new("gh", &["repo".to_string(), "view".to_string(), "--json".to_string(), "url".to_string(), "--jq".to_string(), ".url".to_string()], ""));
        if outcome.status != 0 {
            return Err(ProviderError::new(outcome.stderr.trim().to_string()));
        }
        Ok(outcome.stdout.trim().to_string())
    }

    fn checkpoint(&self, repo_root: &str, description: &str) -> ProviderResult<String> {
        if self.git(&["diff", "--cached", "--quiet"], repo_root).status == 0 {
            self.stage_all(repo_root).map_err(|error| ProviderError::new(format!("stage all failed: {error}")))?;
        }
        let outcome = self.git(&["commit", "-m", description], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git commit failed (exit {}): {}", outcome.status, outcome.stderr.trim())));
        }
        self.current_checkpoint(repo_root).map_err(|error| ProviderError::new(format!("failed to get checkpoint sha after checkpoint: {error}")))
    }

    fn current_checkpoint(&self, repo_root: &str) -> ProviderResult<String> {
        let outcome = self.git(&["rev-parse", "HEAD"], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git rev-parse HEAD failed: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout.trim().to_string())
    }

    fn checkin(&self, repo_root: &str, contributor: &str) -> ProviderResult<()> {
        let contributor_branch = format!("{contributor}/latest");
        let fetched = self.git(&["fetch", "origin", "main"], repo_root);
        if fetched.status != 0 {
            return Err(ProviderError::new(format!("git fetch origin main failed: {}", fetched.stderr.trim())));
        }
        let current_branch = self.current_branch(repo_root)?;
        if current_branch != contributor_branch {
            let switched = self.git(&["switch", &contributor_branch], repo_root);
            if switched.status != 0 {
                let created = self.git(&["switch", "-c", &contributor_branch], repo_root);
                if created.status != 0 {
                    return Err(ProviderError::new(format!("git switch to {contributor_branch} failed: {}", created.stderr.trim())));
                }
            }
        }
        let merged = self.git(&["merge", "--ff-only", "origin/main"], repo_root);
        if merged.status != 0 {
            return Err(ProviderError::new(format!("git fast-forward merge to main failed: {}", merged.stderr.trim())));
        }
        Ok(())
    }

    fn checkout(&self, repo_root: &str, contributor: &str, description: &str) -> ProviderResult<String> {
        let contributor_branch = format!("{contributor}/latest");
        let archive_branch = archive_branch_name(contributor, utc_year_month_day());
        if self.git(&["branch", &archive_branch, &contributor_branch], repo_root).status != 0 {
            for suffix in 2..=99 {
                let candidate = format!("{archive_branch}-{suffix}");
                if self.git(&["branch", &candidate, &contributor_branch], repo_root).status == 0 {
                    break;
                }
            }
        }
        let switched = self.git(&["switch", "main"], repo_root);
        if switched.status != 0 {
            return Err(ProviderError::new(format!("git switch to main failed: {}", switched.stderr.trim())));
        }
        let squashed = self.git(&["merge", "--squash", &contributor_branch], repo_root);
        if squashed.status != 0 {
            return Err(ProviderError::new(format!("git squash merge failed: {}", squashed.stderr.trim())));
        }
        let committed = self.git(&["commit", "-m", description], repo_root);
        if committed.status != 0 {
            return Err(ProviderError::new(format!("git commit squash merge failed: {}", committed.stderr.trim())));
        }
        self.current_checkpoint(repo_root)
    }

    fn current_branch(&self, repo_root: &str) -> ProviderResult<String> {
        let outcome = self.git(&["rev-parse", "--abbrev-ref", "HEAD"], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git rev-parse --abbrev-ref HEAD failed: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout.trim().to_string())
    }

    fn staged_files(&self, repo_root: &str) -> ProviderResult<Vec<String>> {
        let outcome = self.git(&["diff", "--cached", "--name-only"], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git diff --cached --name-only failed: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout.replace("\r\n", "\n").split('\n').filter(|line| !line.is_empty()).map(str::to_string).collect())
    }

    fn stage_all(&self, repo_root: &str) -> ProviderResult<()> {
        let outcome = self.git(&["add", "-A"], repo_root);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("git add -A failed: {}", outcome.stderr.trim())));
        }
        Ok(())
    }
}

/// 🗓️ The `contributor/YYYY/MM/DD` archive branch name a checkout parks the previous work on.
pub fn archive_branch_name(contributor: &str, (year, month, day): (i64, i64, i64)) -> String {
    format!("{contributor}/{year:04}/{month:02}/{day:02}")
}

/// 🕰️ Today in UTC as `(year, month, day)`, from the civil-from-days algorithm — no time crate.
pub fn utc_year_month_day() -> (i64, i64, i64) {
    let seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |delta| delta.as_secs() as i64);
    civil_from_days(seconds.div_euclid(86_400))
}

pub(super) fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 { shifted_month + 3 } else { shifted_month - 9 };
    (if month <= 2 { year + 1 } else { year }, month, day)
}

dyn_enum_close! {
    pub enum VersionControlProviders: VersionControlProvider {
        Git(GitVersionControlProvider),
    }
}
// #endregion 🌿️GitVersionControlProvider

// #region ⛑️DevcontainerSandboxProvider
/// 🐳️ Devcontainer sandbox.
#[derive(Debug, Default)]
pub struct DevcontainerSandboxProvider;

impl SandboxProvider for DevcontainerSandboxProvider {
    fn kind(&self) -> &'static str {
        "devcontainer"
    }
}

dyn_enum_close! {
    pub enum SandboxProviders: SandboxProvider {
        Devcontainer(DevcontainerSandboxProvider),
    }
}
// #endregion ⛑️DevcontainerSandboxProvider
