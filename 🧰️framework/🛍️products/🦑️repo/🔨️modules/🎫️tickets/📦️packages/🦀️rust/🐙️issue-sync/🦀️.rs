//! 🐙️ Issue tracker synchronisation for ticket open and close.

use super::*;

//#region 🐙️IssueSync

/// 🤖️ The prompt section heading of an issue body or comment.
pub fn format_prompt_heading(body: &str) -> String {
    if body.is_empty() {
        "# 🤖️ Prompt".to_string()
    } else {
        format!("# 🤖️ Prompt\n\n{body}")
    }
}

/// 🔍️ The summary section heading of a closing comment.
pub fn format_summary_heading(body: &str) -> String {
    if body.is_empty() {
        "# 🔍️ Summary".to_string()
    } else {
        format!("# 🔍️ Summary\n\n{body}")
    }
}

/// 🐙️ Creates, links or reopens the issue of a ticket, returning the warnings it swallowed.
///
/// A management failure is never fatal to the ticket: the Go twin logs and continues, and so does
/// this, which is why the warnings come back as data instead of as an error.
pub fn ensure_ticket_issue<T: IssueTracker>(tracker: &T, ticket: &mut Ticket, title: &str, prompt: &str, issue: &str, milestone: Option<i64>, reopen_if_closed: bool) -> Vec<String> {
    let mut warnings: Vec<String> = Vec::new();
    let issue = issue.trim();
    if !issue.is_empty() {
        ticket.management = Some(TicketManagementData { issue: issue.to_string() });
        return warnings;
    }
    if let Some(existing) = ticket.management.clone().filter(|management| !management.issue.is_empty()) {
        if reopen_if_closed {
            match tracker.get_issue_details(&existing.issue) {
                Ok(Some(remote)) if remote.state.eq_ignore_ascii_case("closed") => {
                    if let Err(error) = tracker.reopen_issue(&existing.issue) {
                        warnings.push(format!("reopen github issue: {}", error.message));
                    }
                }
                Ok(_) => {}
                Err(error) => warnings.push(format!("read github issue: {}", error.message)),
            }
        }
        return warnings;
    }
    match tracker.create_issue(title, &format_prompt_heading(prompt), milestone) {
        Ok(url) if url.trim().is_empty() => warnings.push("github issue create returned empty url".to_string()),
        Ok(url) => ticket.management = Some(TicketManagementData { issue: url }),
        Err(error) => warnings.push(format!("Failed to create GitHub issue: {}", error.message)),
    }
    warnings
}

/// 📪️ Comments the summary onto the issue, labels it with the touched bundles and closes it.
pub fn close_ticket_issue<T: IssueTracker>(tracker: &T, ticket: &Ticket, summary: &str, labels: &[String], bulk: bool) -> Vec<String> {
    let mut warnings: Vec<String> = Vec::new();
    let Some(management) = ticket.management.as_ref().filter(|management| !management.issue.is_empty()) else { return warnings };
    let issue_url = management.issue.as_str();
    if !bulk {
        if !labels.is_empty() {
            if let Err(error) = tracker.add_labels(issue_url, labels) {
                warnings.push(format!("Failed to add labels to GitHub issue: {}", error.message));
            }
        }
        if let Err(error) = tracker.add_comment(issue_url, &format_summary_heading(summary)) {
            warnings.push(format!("Failed to add summary and metrics comment to GitHub issue: {}", error.message));
        }
    }
    if let Err(error) = tracker.close_issue(issue_url) {
        warnings.push(format!("Failed to close GitHub issue: {}", error.message));
    }
    warnings
}

/// 🎯️ Looks a milestone number up by title, swallowing a lookup failure the way the Go twin does.
pub fn milestone_number_for_title<T: IssueTracker>(tracker: &T, title: &str) -> Option<i64> {
    tracker.find_milestone_by_title(title).ok().flatten().map(|milestone| milestone.number)
}

/// 🎞️ What a recorded issue tracker answers, so an issue-synchronisation case needs no `gh`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueTrackerScript {
    /// 🔗️ The URL `create_issue` returns. An empty string reproduces the empty-url defect.
    #[serde(default)]
    pub created_issue_url: String,
    /// 🔎️ The state `get_issue_details` reports, or empty for an issue that cannot be read.
    #[serde(default)]
    pub issue_state: String,
    /// 🎯️ The milestone number `find_milestone_by_title` answers for the ticket's goal.
    #[serde(default)]
    pub milestone: Option<i64>,
    /// 🎯️ The title that milestone carries.
    #[serde(default)]
    pub milestone_title: String,
    /// 💣️ Method names that fail instead of answering.
    #[serde(default)]
    pub failures: Vec<String>,
}

/// 🎞️ A fixture-driven issue tracker that records every interaction in the order it happened.
///
/// It is deliberately a port-level recording rather than a process transcript: the GitHub provider
/// writes its comment bodies to a temporary file whose name carries the process id, so a recorded
/// argv could never match twice. What a ticket owes its issue tracker is WHICH calls it makes, in
/// which order, with which arguments — and that is exactly what this records.
#[derive(Debug, Default)]
pub struct RecordedIssueTracker {
    pub(super) script: IssueTrackerScript,
    pub(super) calls: RefCell<Vec<String>>,
}

impl RecordedIssueTracker {
    /// 🆕️ Binds a tracker to a script.
    pub fn new(script: IssueTrackerScript) -> RecordedIssueTracker {
        RecordedIssueTracker { script, calls: RefCell::new(Vec::new()) }
    }

    /// 📥️ Reads a script from JSON text.
    pub fn from_json(text: &str) -> TicketResult<RecordedIssueTracker> {
        serde_json::from_str(text).map(RecordedIssueTracker::new).map_err(|error| TicketError::invalid(error.to_string()))
    }

    /// 📼️ Every interaction so far, in order, as `method|argument|argument`.
    pub fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }

    pub(super) fn record(&self, call: String) {
        self.calls.borrow_mut().push(call);
    }

    fn fails(&self, method: &str) -> Option<ProviderError> {
        self.script.failures.iter().any(|candidate| candidate == method).then(|| ProviderError::new(format!("{method} failed")))
    }
}

impl IssueTracker for RecordedIssueTracker {
    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> Result<String, ProviderError> {
        self.record(format!("create_issue|{title}|{body}|{}", milestone.map(|number| number.to_string()).unwrap_or_default()));
        match self.fails("create_issue") {
            Some(error) => Err(error),
            None => Ok(self.script.created_issue_url.clone()),
        }
    }

    fn close_issue(&self, issue_url: &str) -> Result<(), ProviderError> {
        self.record(format!("close_issue|{issue_url}"));
        self.fails("close_issue").map_or(Ok(()), Err)
    }

    fn reopen_issue(&self, issue_url: &str) -> Result<(), ProviderError> {
        self.record(format!("reopen_issue|{issue_url}"));
        self.fails("reopen_issue").map_or(Ok(()), Err)
    }

    fn update_issue_title(&self, issue_url: &str, title: &str) -> ProviderResult<()> { self.record(format!("update_issue_title|{issue_url}|{title}")); self.fails("update_issue_title").map_or(Ok(()), Err) }
    fn delete_issue(&self, issue_url: &str) -> ProviderResult<()> { self.record(format!("delete_issue|{issue_url}")); self.fails("delete_issue").map_or(Ok(()), Err) }

    fn get_issue_details(&self, issue_url: &str) -> Result<Option<ManagementIssue>, ProviderError> {
        self.record(format!("get_issue_details|{issue_url}"));
        if let Some(error) = self.fails("get_issue_details") {
            return Err(error);
        }
        if self.script.issue_state.is_empty() {
            return Ok(None);
        }
        Ok(Some(ManagementIssue { url: issue_url.to_string(), state: self.script.issue_state.clone(), ..ManagementIssue::default() }))
    }

    fn add_comment(&self, issue_url: &str, comment: &str) -> Result<(), ProviderError> {
        self.record(format!("add_comment|{issue_url}|{comment}"));
        self.fails("add_comment").map_or(Ok(()), Err)
    }

    fn add_labels(&self, issue_url: &str, labels: &[String]) -> Result<(), ProviderError> {
        self.record(format!("add_labels|{issue_url}|{}", labels.join(",")));
        self.fails("add_labels").map_or(Ok(()), Err)
    }

    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> Result<(), ProviderError> {
        self.record(format!("remove_labels|{issue_url}|{}", labels.join(",")));
        self.fails("remove_labels").map_or(Ok(()), Err)
    }

    fn find_milestone_by_title(&self, title: &str) -> Result<Option<ManagementMilestone>, ProviderError> {
        self.record(format!("find_milestone_by_title|{title}"));
        if let Some(error) = self.fails("find_milestone_by_title") {
            return Err(error);
        }
        Ok(self.script.milestone.map(|number| ManagementMilestone { number, title: self.script.milestone_title.clone(), ..ManagementMilestone::default() }))
    }

    fn list_repo_labels(&self) -> Result<Vec<ManagementLabel>, ProviderError> {
        self.record("list_repo_labels".to_string());
        self.fails("list_repo_labels").map_or(Ok(Vec::new()), Err)
    }
}

/// 🐙️ What one issue synchronisation left behind: the link the ticket now carries and the warnings.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueSyncOutcome {
    pub issue: String,
    pub warnings: Vec<String>,
}

/// 🐙️ Synchronises the issue of a ticket that is being opened or reopened, without a ticket in hand.
///
/// The ticket-shaped entry point is [`ensure_ticket_issue`]; this one takes the two members it
/// actually reads, so a caller that has only the link — a test host, a CLI verb — needs no model type.
pub fn sync_open_issue<T: IssueTracker>(tracker: &T, existing_issue: &str, title: &str, prompt: &str, issue: &str, milestone: Option<i64>, reopen_if_closed: bool) -> IssueSyncOutcome {
    let mut ticket = Ticket {
        year: 0,
        month: 0,
        day: 0,
        slug: String::new(),
        title: title.to_string(),
        emoji: String::new(),
        status: TicketStatus::Open,
        description: String::new(),
        summary: String::new(),
        management: (!existing_issue.is_empty()).then(|| TicketManagementData { issue: existing_issue.to_string() }),
        goal: String::new(),
        parent: String::new(),
        plan: None,
        sessions: Vec::new(),
        interactions: Vec::new(),
        agents: Vec::new(),
        folder_path: String::new(),
        json_path: String::new(),
        important_path: String::new(),
    };
    let warnings = ensure_ticket_issue(tracker, &mut ticket, title, prompt, issue, milestone, reopen_if_closed);
    IssueSyncOutcome { issue: ticket.management.map(|management| management.issue).unwrap_or_default(), warnings }
}

/// 📪️ Synchronises the issue of a ticket that is being closed, without a ticket in hand.
pub fn sync_close_issue<T: IssueTracker>(tracker: &T, issue_url: &str, summary: &str, labels: &[String], bulk: bool) -> Vec<String> {
    let ticket = Ticket {
        year: 0,
        month: 0,
        day: 0,
        slug: String::new(),
        title: String::new(),
        emoji: String::new(),
        status: TicketStatus::Closed,
        description: String::new(),
        summary: String::new(),
        management: (!issue_url.is_empty()).then(|| TicketManagementData { issue: issue_url.to_string() }),
        goal: String::new(),
        parent: String::new(),
        plan: None,
        sessions: Vec::new(),
        interactions: Vec::new(),
        agents: Vec::new(),
        folder_path: String::new(),
        json_path: String::new(),
        important_path: String::new(),
    };
    close_ticket_issue(tracker, &ticket, summary, labels, bulk)
}

//#endregion 🐙️IssueSync
