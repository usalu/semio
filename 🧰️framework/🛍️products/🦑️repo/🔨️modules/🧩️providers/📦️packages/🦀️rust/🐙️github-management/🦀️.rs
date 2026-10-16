//! 🐙️ GitHub management: the gh-backed and null management providers.
//!
//! Holds the `🐙️GitHubManagementProvider` region of the crate root; the root re-exports its items.

use super::*;

// #region 🐙️GitHubManagementProvider
/// 🔗️ Parses a GitHub issue URL out of `gh issue create` output.
pub fn extract_issue_url(output: &str) -> String {
    let output = output.trim();
    if output.is_empty() {
        return String::new();
    }
    if let Some(first) = output.split_whitespace().next() {
        if first.starts_with("https://") {
            return first.to_string();
        }
    }
    const PREFIX: &str = "https://github.com/";
    if let Some(index) = output.find(PREFIX) {
        let mut rest = &output[index..];
        if let Some(end) = rest.find([' ', '\t', '\r', '\n']) {
            if end > 0 {
                rest = &rest[..end];
            }
        }
        if rest.contains("/issues/") {
            return rest.to_string();
        }
    }
    String::new()
}

/// 🐙️ GitHub management through the `gh` CLI.
pub struct GitHubManagementProvider {
    pub(super) runner: ProcessRunners,
}

impl GitHubManagementProvider {
    /// 🆕️ Binds the provider to a process runner.
    pub fn new(runner: ProcessRunners) -> GitHubManagementProvider {
        GitHubManagementProvider { runner }
    }

    /// 📜️ Every argv this provider issued, in order.
    pub fn issued(&self) -> Vec<Vec<String>> {
        self.runner.issued()
    }

    fn gh(&self, args: &[&str]) -> ProcessOutcome {
        let owned: Vec<String> = args.iter().map(|arg| (*arg).to_string()).collect();
        self.runner.run(&ProcessRequest::new("gh", &owned, ""))
    }

    fn gh_owned(&self, args: &[String]) -> ProcessOutcome {
        self.runner.run(&ProcessRequest::new("gh", args, ""))
    }

    fn checked(&self, args: &[&str], failure: &str) -> ProviderResult<String> {
        let outcome = self.gh(args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("{failure}: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout)
    }

    fn checked_owned(&self, args: &[String], failure: &str) -> ProviderResult<String> {
        let outcome = self.gh_owned(args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("{failure}: {}", outcome.stderr.trim())));
        }
        Ok(outcome.stdout)
    }
}

impl ManagementProvider for GitHubManagementProvider {
    fn kind(&self) -> &'static str {
        "github"
    }

    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> ProviderResult<String> {
        let mut args = vec!["issue".to_string(), "create".to_string(), "--title".to_string(), title.to_string(), "--body".to_string(), body.to_string(), "--label".to_string(), "ticket".to_string()];
        if let Some(number) = milestone {
            if let Ok(milestone_title) = self.get_milestone_title(number) {
                args.push("--milestone".to_string());
                args.push(milestone_title);
            }
        }
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh issue create failed: {}", outcome.stderr.trim())));
        }
        let mut issue_url = extract_issue_url(&outcome.stdout);
        if issue_url.is_empty() {
            issue_url = extract_issue_url(&outcome.stderr);
        }
        if issue_url.is_empty() {
            return Err(ProviderError::new(format!("gh issue create succeeded but no issue url in output: stdout={:?} stderr={:?}", outcome.stdout.trim(), outcome.stderr.trim())));
        }
        self.add_issue_to_project(&issue_url);
        self.assign_issue_to_current_user(&issue_url);
        Ok(issue_url)
    }

    fn close_issue(&self, issue_url: &str) -> ProviderResult<()> {
        self.checked(&["issue", "close", issue_url], "gh issue close failed").map(|_| ())
    }

    fn reopen_issue(&self, issue_url: &str) -> ProviderResult<()> {
        self.checked(&["issue", "reopen", issue_url], "gh issue reopen failed").map(|_| ())
    }

    fn delete_issue(&self, issue_url_or_number: &str) -> ProviderResult<()> {
        let outcome = self.gh(&["issue", "delete", issue_url_or_number, "--yes"]);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh issue delete failed: {}", outcome.stderr)));
        }
        Ok(())
    }

    fn update_issue_title(&self, issue_url: &str, title: &str) -> ProviderResult<()> {
        self.checked(&["issue", "edit", issue_url, "--title", title], "gh issue edit failed").map(|_| ())
    }

    fn update_issue_body(&self, issue_url: &str, body: &str) -> ProviderResult<()> {
        self.checked(&["issue", "edit", issue_url, "--body", body], "gh issue edit body failed").map(|_| ())
    }

    fn get_issue_details(&self, issue_url: &str) -> ProviderResult<Option<ManagementIssue>> {
        let stdout = self.checked(&["issue", "view", issue_url, "--json", "url,state,milestone,labels,title,body"], "gh issue view failed")?;
        let issue: ManagementIssue = serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh issue view output: {error}")))?;
        Ok(Some(issue))
    }

    fn get_issue_node_id(&self, issue_url: &str) -> ProviderResult<String> {
        Ok(self.checked(&["issue", "view", issue_url, "--json", "id", "--jq", ".id"], "gh issue view failed")?.trim().to_string())
    }

    fn get_issue_parent_url(&self, issue_url: &str) -> ProviderResult<String> {
        let query = "query($url: URI!) {\n\t\tresource(url: $url) {\n\t\t\t... on Issue {\n\t\t\t\tparent {\n\t\t\t\t\turl\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}";
        let stdout = self.checked_owned(&["api".to_string(), "graphql".to_string(), "-f".to_string(), format!("url={issue_url}"), "-f".to_string(), format!("query={query}")], "gh api issue parent query failed")?;
        let parsed: Value = serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse issue parent response: {error}")))?;
        Ok(parsed.get("data").and_then(|data| data.get("resource")).and_then(|resource| resource.get("parent")).and_then(|parent| parent.get("url")).and_then(Value::as_str).unwrap_or_default().to_string())
    }

    fn add_comment(&self, issue_url: &str, comment: &str) -> ProviderResult<()> {
        let path = std::env::temp_dir().join(format!("gh-comment-{}.md", std::process::id()));
        std::fs::write(&path, comment).map_err(|error| ProviderError::new(format!("write comment temp file: {error}")))?;
        let body_file = path.to_string_lossy().to_string();
        let outcome = self.gh_owned(&["issue".to_string(), "comment".to_string(), issue_url.to_string(), "--body-file".to_string(), body_file]);
        let _ = std::fs::remove_file(&path);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh issue comment failed: {}", outcome.stderr.trim())));
        }
        Ok(())
    }

    fn add_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()> {
        if labels.is_empty() {
            return Ok(());
        }
        let mut args = vec!["issue".to_string(), "edit".to_string(), issue_url.to_string()];
        for label in labels {
            args.push("--add-label".to_string());
            args.push(label.clone());
        }
        self.checked_owned(&args, "gh issue edit failed").map(|_| ())
    }

    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> ProviderResult<()> {
        if labels.is_empty() {
            return Ok(());
        }
        let mut args = vec!["issue".to_string(), "edit".to_string(), issue_url.to_string()];
        for label in labels {
            args.push("--remove-label".to_string());
            args.push(label.clone());
        }
        self.checked_owned(&args, "gh issue edit remove-label failed").map(|_| ())
    }

    fn add_issue_to_project(&self, issue_url: &str) {
        if issue_url.is_empty() {
            return;
        }
        let _ = self.gh(&["project", "item-add", "2", "--owner", "usalu", "--url", issue_url]);
    }

    fn assign_issue_to_current_user(&self, issue_url: &str) {
        if issue_url.is_empty() {
            return;
        }
        let user = self.get_current_user();
        if user.is_empty() {
            return;
        }
        let _ = self.gh(&["issue", "edit", issue_url, "--add-assignee", &user]);
    }

    fn add_sub_issue(&self, parent_issue_url: &str, child_issue_url: &str) -> ProviderResult<()> {
        let parent_node_id = self.get_issue_node_id(parent_issue_url).map_err(|error| ProviderError::new(format!("failed to get parent node ID: {error}")))?;
        let child_node_id = self.get_issue_node_id(child_issue_url).map_err(|error| ProviderError::new(format!("failed to get child node ID: {error}")))?;
        let query = "mutation($parentId: ID!, $childId: ID!) {\n\t\taddSubIssue(input: { issueId: $parentId, subIssueId: $childId }) {\n\t\t\tsubIssue { url }\n\t\t}\n\t}";
        let args = vec![
            "api".to_string(),
            "graphql".to_string(),
            "-H".to_string(),
            "GraphQL-Features:sub_issues".to_string(),
            "-f".to_string(),
            format!("parentId={parent_node_id}"),
            "-f".to_string(),
            format!("childId={child_node_id}"),
            "-f".to_string(),
            format!("query={query}"),
        ];
        self.checked_owned(&args, "gh api addSubIssue failed").map(|_| ())
    }

    fn update_issue_milestone(&self, issue_url: &str, milestone_title: &str) -> ProviderResult<()> {
        if milestone_title.trim().is_empty() {
            return Err(ProviderError::new("milestone title is required"));
        }
        self.checked(&["issue", "edit", issue_url, "--milestone", milestone_title], "gh issue edit milestone failed").map(|_| ())
    }

    fn clear_issue_milestone(&self, issue_url: &str) -> ProviderResult<()> {
        let issue_node_id = self.get_issue_node_id(issue_url)?;
        let query = "mutation($issueId: ID!) {\n\t\tupdateIssue(input: { id: $issueId, milestoneId: null }) {\n\t\t\tissue { id }\n\t\t}\n\t}";
        let args = vec!["api".to_string(), "graphql".to_string(), "-f".to_string(), format!("issueId={issue_node_id}"), "-f".to_string(), format!("query={query}")];
        self.checked_owned(&args, "gh api updateIssue clear milestone failed").map(|_| ())
    }

    fn create_milestone(&self, title: &str, description: &str) -> ProviderResult<i64> {
        let args = vec!["api".to_string(), "repos/:owner/:repo/milestones".to_string(), "-f".to_string(), format!("title={title}"), "-f".to_string(), format!("description={description}"), "--jq".to_string(), ".number".to_string()];
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh api milestone create failed: {}", outcome.stderr)));
        }
        Ok(outcome.stdout.trim().parse::<i64>().unwrap_or(0))
    }

    fn update_milestone(&self, number: i64, title: &str, description: &str, state: &str, due_on: &str) -> ProviderResult<()> {
        let mut args = vec!["api".to_string(), format!("repos/:owner/:repo/milestones/{number}"), "-X".to_string(), "PATCH".to_string()];
        if !title.is_empty() {
            args.push("-f".to_string());
            args.push(format!("title={title}"));
        }
        if !description.is_empty() {
            args.push("-f".to_string());
            args.push(format!("description={description}"));
        }
        if !state.is_empty() {
            args.push("-f".to_string());
            args.push(format!("state={state}"));
        }
        if !due_on.is_empty() {
            let normalized = if due_on.contains('T') { due_on.to_string() } else { format!("{due_on}T00:00:00Z") };
            args.push("-f".to_string());
            args.push(format!("due_on={normalized}"));
        }
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh api milestone update failed: {}", outcome.stderr)));
        }
        Ok(())
    }

    fn delete_milestone(&self, number: i64) -> ProviderResult<()> {
        let args = vec!["api".to_string(), format!("repos/:owner/:repo/milestones/{number}"), "-X".to_string(), "DELETE".to_string()];
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh api milestone delete failed: {}", outcome.stderr)));
        }
        Ok(())
    }

    fn get_milestone(&self, number: i64) -> ProviderResult<Option<ManagementMilestone>> {
        let args = vec!["api".to_string(), format!("repos/:owner/:repo/milestones/{number}")];
        let stdout = self.checked_owned(&args, "gh api milestone get failed")?;
        let milestone: ManagementMilestone = serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh milestone output: {error}")))?;
        Ok(Some(milestone))
    }

    fn get_milestone_title(&self, number: i64) -> ProviderResult<String> {
        let args = vec!["api".to_string(), format!("repos/{{owner}}/{{repo}}/milestones/{number}"), "--jq".to_string(), ".title".to_string()];
        Ok(self.checked_owned(&args, "gh api milestone failed")?.trim().to_string())
    }

    fn find_milestone_by_title(&self, title: &str) -> ProviderResult<Option<ManagementMilestone>> {
        if title.trim().is_empty() {
            return Ok(None);
        }
        let stdout = self.checked(&["api", "repos/:owner/:repo/milestones", "-X", "GET", "-F", "state=all", "-F", "per_page=100", "--paginate", "--jq", ".[]"], "gh api milestone list failed")?;
        for line in stdout.lines() {
            let Ok(milestone) = serde_json::from_str::<ManagementMilestone>(line) else { continue };
            if milestone.title == title {
                return Ok(Some(milestone));
            }
        }
        Ok(None)
    }

    fn list_issues_for_label_sync(&self) -> ProviderResult<Vec<ManagementIssue>> {
        let stdout = self.checked(&["issue", "list", "--state", "all", "--json", "url,labels", "--limit", "1000"], "gh issue list for label sync failed")?;
        serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh issue list output: {error}")))
    }

    fn list_open_issues_with_label(&self, label: &str) -> ProviderResult<Vec<String>> {
        let stdout = self.checked(&["issue", "list", "--label", label, "--state", "open", "--json", "url", "--limit", "1000"], "gh issue list failed")?;
        let issues: Vec<ManagementIssue> = serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh issue list output: {error}")))?;
        Ok(issues.into_iter().map(|issue| issue.url).collect())
    }

    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>> {
        let stdout = self.checked(&["label", "list", "--json", "name", "--limit", "1000"], "gh label list failed")?;
        serde_json::from_str(&stdout).map_err(|error| ProviderError::new(format!("failed to parse gh label list output: {error}")))
    }

    fn create_repo_label(&self, name: &str) -> ProviderResult<()> {
        if name.trim().is_empty() {
            return Err(ProviderError::new("label name is required"));
        }
        self.checked(&["label", "create", name, "--color", "1d76db", "--description", "Compose technology or bundle"], "gh label create failed").map(|_| ())
    }

    fn delete_repo_label(&self, name: &str) -> ProviderResult<()> {
        if name.trim().is_empty() {
            return Err(ProviderError::new("label name is required"));
        }
        self.checked(&["label", "delete", name, "--yes"], "gh label delete failed").map(|_| ())
    }

    fn sync_repo_label_catalog(&self, valid_labels: &[String]) -> ProviderResult<()> {
        let existing: Vec<String> = self.list_repo_labels()?.into_iter().map(|label| label.name).collect();
        let mut missing: Vec<&String> = valid_labels.iter().filter(|label| label.starts_with('@') && !existing.contains(label)).collect();
        missing.sort();
        for label in missing {
            let _ = self.create_repo_label(label);
        }
        let mut stale: Vec<&String> = existing.iter().filter(|label| label.starts_with('@') && !valid_labels.contains(label)).collect();
        stale.sort();
        for label in stale {
            let _ = self.delete_repo_label(label);
        }
        Ok(())
    }

    fn create_goal_issue(&self, title: &str, description: &str, milestone: Option<i64>) -> ProviderResult<String> {
        let mut args = vec!["issue".to_string(), "create".to_string(), "--title".to_string(), title.to_string(), "--body".to_string(), description.to_string(), "--label".to_string(), "goal".to_string()];
        if let Some(number) = milestone {
            let milestone_title = self.get_milestone_title(number).map_err(|error| ProviderError::new(format!("could not resolve milestone {number}: {error}")))?;
            args.push("--milestone".to_string());
            args.push(milestone_title);
        }
        let outcome = self.gh_owned(&args);
        if outcome.status != 0 {
            return Err(ProviderError::new(format!("gh issue create failed: {}", outcome.stderr.trim())));
        }
        let issue_url = outcome.stdout.trim().to_string();
        if !issue_url.is_empty() {
            self.add_issue_to_project(&issue_url);
        }
        Ok(issue_url)
    }

    fn update_goal_issue(&self, issue_url: &str, title: &str, description: &str) -> ProviderResult<()> {
        let mut args = vec!["issue".to_string(), "edit".to_string(), issue_url.to_string()];
        if !title.is_empty() {
            args.push("--title".to_string());
            args.push(title.to_string());
        }
        if !description.is_empty() {
            args.push("--body".to_string());
            args.push(description.to_string());
        }
        self.checked_owned(&args, "gh issue edit failed").map(|_| ())
    }

    fn get_current_user(&self) -> String {
        let outcome = self.gh(&["api", "user", "--jq", ".login"]);
        if outcome.status != 0 {
            return String::new();
        }
        outcome.stdout.trim().to_string()
    }
}

/// 💻️ A management provider that does nothing and reports nothing.
#[derive(Debug, Default)]
pub struct NullManagementProvider;

impl ManagementProvider for NullManagementProvider {
    fn kind(&self) -> &'static str {
        "none"
    }
    fn create_issue(&self, _title: &str, _body: &str, _milestone: Option<i64>) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn close_issue(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn reopen_issue(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn delete_issue(&self, _issue_url_or_number: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn update_issue_title(&self, _issue_url: &str, _title: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn update_issue_body(&self, _issue_url: &str, _body: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn get_issue_details(&self, _issue_url: &str) -> ProviderResult<Option<ManagementIssue>> {
        Ok(None)
    }
    fn get_issue_node_id(&self, _issue_url: &str) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn get_issue_parent_url(&self, _issue_url: &str) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn add_comment(&self, _issue_url: &str, _comment: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn add_labels(&self, _issue_url: &str, _labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }
    fn remove_labels(&self, _issue_url: &str, _labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }
    fn add_issue_to_project(&self, _issue_url: &str) {}
    fn assign_issue_to_current_user(&self, _issue_url: &str) {}
    fn add_sub_issue(&self, _parent_issue_url: &str, _child_issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn update_issue_milestone(&self, _issue_url: &str, _milestone_title: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn clear_issue_milestone(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn create_milestone(&self, _title: &str, _description: &str) -> ProviderResult<i64> {
        Ok(0)
    }
    fn update_milestone(&self, _number: i64, _title: &str, _description: &str, _state: &str, _due_on: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn delete_milestone(&self, _number: i64) -> ProviderResult<()> {
        Ok(())
    }
    fn get_milestone(&self, _number: i64) -> ProviderResult<Option<ManagementMilestone>> {
        Ok(None)
    }
    fn get_milestone_title(&self, _number: i64) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn find_milestone_by_title(&self, _title: &str) -> ProviderResult<Option<ManagementMilestone>> {
        Ok(None)
    }
    fn list_issues_for_label_sync(&self) -> ProviderResult<Vec<ManagementIssue>> {
        Ok(Vec::new())
    }
    fn list_open_issues_with_label(&self, _label: &str) -> ProviderResult<Vec<String>> {
        Ok(Vec::new())
    }
    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>> {
        Ok(Vec::new())
    }
    fn create_repo_label(&self, _name: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn delete_repo_label(&self, _name: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn sync_repo_label_catalog(&self, _valid_labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }
    fn create_goal_issue(&self, _title: &str, _description: &str, _milestone: Option<i64>) -> ProviderResult<String> {
        Ok(String::new())
    }
    fn update_goal_issue(&self, _issue_url: &str, _title: &str, _description: &str) -> ProviderResult<()> {
        Ok(())
    }
    fn get_current_user(&self) -> String {
        String::new()
    }
}

dyn_enum_close! {
    pub enum ManagementProviders: ManagementProvider {
        GitHub(GitHubManagementProvider),
        Null(NullManagementProvider),
    }
}
// #endregion 🐙️GitHubManagementProvider
