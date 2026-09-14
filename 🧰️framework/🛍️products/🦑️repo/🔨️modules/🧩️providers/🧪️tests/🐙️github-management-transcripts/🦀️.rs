//! 🦀️ Rust side of the GitHub management transcript case. The subject halves are gated behind the
//! `sut` feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_providers::{GitHubManagementProvider, ManagementProvider, ProcessRunners, RecordedProcessRunner};
    use semio_repo_test_host::{Context, Json, Outcome};

    const TRANSCRIPTS: &str = "local://🎞️gh-transcripts.json";

    fn provider(ctx: &Context) -> Result<GitHubManagementProvider, String> {
        let transcript = ctx.fixture_json(TRANSCRIPTS)?.get(&ctx.scenario.id).cloned().ok_or_else(|| format!("transcript fixture carries no entry for scenario {}", ctx.scenario.id))?;
        let runner = RecordedProcessRunner::from_json(&transcript.to_string()).map_err(|error| error.message)?;
        Ok(GitHubManagementProvider::new(ProcessRunners::from(runner)))
    }

    fn argv(provider: &GitHubManagementProvider) -> Json {
        Json::Array(provider.issued().into_iter().map(|call| Json::Array(call.into_iter().map(Json::String).collect())).collect())
    }

    fn text(value: &str) -> Json {
        Json::String(value.to_string())
    }

    /// 💿️ An issue view is parsed into every member the provider promises.
    pub fn issue_view_is_parsed(ctx: &Context) -> Result<Outcome, String> {
        let provider = provider(ctx)?;
        let issue = provider.get_issue_details("https://github.com/usalu/semio/issues/412").map_err(|error| error.message)?.ok_or("no issue parsed")?;
        let milestone = match issue.milestone {
            Some(milestone) => Json::Object(vec![("number".to_string(), Json::Number(milestone.number as f64)), ("title".to_string(), text(&milestone.title))]),
            None => Json::Null,
        };
        Ok(Outcome::projection(Json::Object(vec![
            (
                "issue".to_string(),
                Json::Object(vec![
                    ("url".to_string(), text(&issue.url)),
                    ("state".to_string(), text(&issue.state)),
                    ("title".to_string(), text(&issue.title)),
                    ("body".to_string(), text(&issue.body)),
                    ("milestone".to_string(), milestone),
                    ("labels".to_string(), Json::Array(issue.labels.iter().map(|label| text(&label.name)).collect())),
                ]),
            ),
            ("argv".to_string(), argv(&provider)),
        ])))
    }

    /// 🎯️ The paginated milestone stream is scanned line by line.
    pub fn milestone_list_is_scanned_for_a_title(ctx: &Context) -> Result<Outcome, String> {
        let provider = provider(ctx)?;
        let milestone = provider.find_milestone_by_title("26/09").map_err(|error| error.message)?.ok_or("no milestone matched")?;
        Ok(Outcome::projection(Json::Object(vec![
            (
                "milestone".to_string(),
                Json::Object(vec![
                    ("number".to_string(), Json::Number(milestone.number as f64)),
                    ("title".to_string(), text(&milestone.title)),
                    ("description".to_string(), text(&milestone.description)),
                    ("url".to_string(), text(&milestone.url)),
                    ("dueOn".to_string(), text(&milestone.due_on)),
                    ("state".to_string(), text(&milestone.state)),
                ]),
            ),
            ("argv".to_string(), argv(&provider)),
        ])))
    }

    /// 🏷️ The repository label catalogue is parsed out of the label list.
    pub fn label_catalog_is_listed(ctx: &Context) -> Result<Outcome, String> {
        let provider = provider(ctx)?;
        let labels = provider.list_repo_labels().map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![("labels".to_string(), Json::Array(labels.iter().map(|label| text(&label.name)).collect())), ("argv".to_string(), argv(&provider))])))
    }

    /// 🆕️ Creating an issue resolves the milestone title, extracts the url and follows up.
    pub fn create_issue_resolves_the_milestone_title_first(ctx: &Context) -> Result<Outcome, String> {
        let provider = provider(ctx)?;
        let url = provider.create_issue("Split the godfile", "One package per domain.", Some(17)).map_err(|error| error.message)?;
        Ok(Outcome::projection(Json::Object(vec![("url".to_string(), text(&url)), ("argv".to_string(), argv(&provider))])))
    }

    /// ⚠️ A non-zero exit is a failure carrying the trimmed stderr.
    pub fn a_failing_call_carries_the_trimmed_stderr(ctx: &Context) -> Result<Outcome, String> {
        let provider = provider(ctx)?;
        let outcome = provider.get_issue_details("https://github.com/usalu/semio/issues/999");
        let (failed, message) = match outcome {
            Ok(_) => (false, String::new()),
            Err(error) => (true, error.message),
        };
        Ok(Outcome::projection(Json::Object(vec![("failed".to_string(), Json::Bool(failed)), ("message".to_string(), Json::String(message)), ("argv".to_string(), argv(&provider))])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered
        .subject("issue-view-is-parsed", subject::issue_view_is_parsed)
        .subject("milestone-list-is-scanned-for-a-title", subject::milestone_list_is_scanned_for_a_title)
        .subject("label-catalog-is-listed", subject::label_catalog_is_listed)
        .subject("create-issue-resolves-the-milestone-title-first", subject::create_issue_resolves_the_milestone_title_first)
        .subject("a-failing-call-carries-the-trimmed-stderr", subject::a_failing_call_carries_the_trimmed_stderr);
    registered
}
//#endregion 🔖️Registration
