//! 🦀️ Rust side of the issue synchronisation case. The `IssueTracker` port is driven by a scripted
//! recorder, so nothing spawns `gh` and every interaction is observable in the order it happened.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_tickets::{milestone_number_for_title, sync_close_issue, sync_open_issue, RecordedIssueTracker};
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "local://🐙️transcripts.json";

    fn strings(values: Vec<String>) -> Json {
        Json::Array(values.into_iter().map(Json::String).collect())
    }

    fn label_list(vectors: &Json) -> Vec<String> {
        vectors
            .array("labels")
            .into_iter()
            .map(|item| match item {
                Json::String(text) => text,
                other => other.to_string(),
            })
            .collect()
    }

    fn tracker(vectors: &Json, scenario: &str) -> Result<RecordedIssueTracker, String> {
        let scripts = vectors.get("scripts").cloned().unwrap_or(Json::Null);
        let script = scripts.get(scenario).map(Json::to_string).ok_or_else(|| format!("no script for scenario {scenario}"))?;
        RecordedIssueTracker::from_json(&script).map_err(|error| error.message)
    }

    /// 🆕️ A new ticket creates one issue.
    pub fn a_new_ticket_creates_one_issue(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let recorder = tracker(&vectors, "a-new-ticket-creates-one-issue")?;
        let milestone = milestone_number_for_title(&recorder, &vectors.str("goal"));
        let outcome = sync_open_issue(&recorder, "", &vectors.str("title"), &vectors.str("prompt"), "", milestone, false);
        Ok(Outcome::projection(Json::Object(vec![
            ("calls".to_string(), strings(recorder.calls())),
            ("issue".to_string(), Json::String(outcome.issue)),
            ("warnings".to_string(), strings(outcome.warnings)),
            ("milestone".to_string(), Json::String(milestone.map(|number| number.to_string()).unwrap_or_default())),
        ])))
    }

    /// 🔓️ An existing open issue is left alone.
    pub fn an_existing_open_issue_is_left_alone(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let recorder = tracker(&vectors, "an-existing-open-issue-is-left-alone")?;
        let outcome = sync_open_issue(&recorder, &vectors.str("issueUrl"), &vectors.str("title"), &vectors.str("prompt"), "", None, true);
        Ok(Outcome::projection(Json::Object(vec![("calls".to_string(), strings(recorder.calls())), ("issue".to_string(), Json::String(outcome.issue)), ("warnings".to_string(), strings(outcome.warnings))])))
    }

    /// ♻️ A closed issue is reopened.
    pub fn a_closed_issue_is_reopened(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let recorder = tracker(&vectors, "a-closed-issue-is-reopened")?;
        let outcome = sync_open_issue(&recorder, &vectors.str("issueUrl"), &vectors.str("title"), &vectors.str("prompt"), "", None, true);
        Ok(Outcome::projection(Json::Object(vec![("calls".to_string(), strings(recorder.calls())), ("issue".to_string(), Json::String(outcome.issue)), ("warnings".to_string(), strings(outcome.warnings))])))
    }

    /// 📪️ A close comments, labels and closes.
    pub fn a_close_comments_labels_and_closes(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let normal = tracker(&vectors, "a-close-comments-labels-and-closes")?;
        let bulk = tracker(&vectors, "a-close-comments-labels-and-closes")?;
        let labels = label_list(&vectors);
        let normal_warnings = sync_close_issue(&normal, &vectors.str("issueUrl"), &vectors.str("summary"), &labels, false);
        let bulk_warnings = sync_close_issue(&bulk, &vectors.str("issueUrl"), &vectors.str("summary"), &labels, true);
        Ok(Outcome::projection(Json::Object(vec![
            ("normalCalls".to_string(), strings(normal.calls())),
            ("normalWarnings".to_string(), strings(normal_warnings)),
            ("bulkCalls".to_string(), strings(bulk.calls())),
            ("bulkWarnings".to_string(), strings(bulk_warnings)),
        ])))
    }

    /// ⚠️ Every failure becomes a warning.
    pub fn every_failure_becomes_a_warning(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let creating = tracker(&vectors, "every-failure-becomes-a-warning")?;
        let reopening = tracker(&vectors, "every-failure-becomes-a-warning")?;
        let closing = tracker(&vectors, "every-failure-becomes-a-warning")?;
        let created = sync_open_issue(&creating, "", &vectors.str("title"), &vectors.str("prompt"), "", None, false);
        let reopened = sync_open_issue(&reopening, &vectors.str("issueUrl"), &vectors.str("title"), &vectors.str("prompt"), "", None, true);
        let close_warnings = sync_close_issue(&closing, &vectors.str("issueUrl"), &vectors.str("summary"), &label_list(&vectors), false);
        Ok(Outcome::projection(Json::Object(vec![
            ("createWarnings".to_string(), strings(created.warnings)),
            ("createIssue".to_string(), Json::String(created.issue)),
            ("reopenWarnings".to_string(), strings(reopened.warnings)),
            ("reopenCalls".to_string(), strings(reopening.calls())),
            ("closeWarnings".to_string(), strings(close_warnings)),
            ("closeCalls".to_string(), strings(closing.calls())),
        ])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered
        .subject("a-new-ticket-creates-one-issue", subject::a_new_ticket_creates_one_issue)
        .subject("an-existing-open-issue-is-left-alone", subject::an_existing_open_issue_is_left_alone)
        .subject("a-closed-issue-is-reopened", subject::a_closed_issue_is_reopened)
        .subject("a-close-comments-labels-and-closes", subject::a_close_comments_labels_and_closes)
        .subject("every-failure-becomes-a-warning", subject::every_failure_becomes_a_warning);
    registered
}
//#endregion 🔖️Registration
