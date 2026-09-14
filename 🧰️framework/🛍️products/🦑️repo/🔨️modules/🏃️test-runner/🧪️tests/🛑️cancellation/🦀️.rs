//! 🦀️ Rust side of the cancellation case: partial outcomes, a progress stream, and refusals.

use semio_framework_repo_test_runner as subject;
use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Support
fn vectors(ctx: &Context) -> Result<subject::CancellationVectors, String> {
    subject::parse_cancellation_vectors(&ctx.fixture_bytes("shared://🛑️cancellation-vectors.json")?)
}

fn report_json(report: &subject::ExecutionReport) -> Result<Json, String> {
    parse_json(&subject::report_to_json_text(report))
}
//#endregion 🔖️Support

//#region 🔖️Scenarios
fn cancelling_mid_plan_keeps_completed_outcomes(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let runner = subject::RecordedProcessRunner::new(fixture.transcripts.clone());
    let cancellation = subject::CancellationToken::new();
    let mut finished = 0usize;
    let report = {
        let token = cancellation.clone();
        let limit = fixture.cancel_after;
        let mut progress = move |event: subject::ProgressEvent| {
            if let subject::ProgressEvent::InvocationFinished { .. } = event {
                finished += 1;
                if finished >= limit {
                    token.cancel();
                }
            }
        };
        subject::execute_plan(&fixture.plan, &runner, &cancellation, &mut progress)
    };
    if !report.cancelled || report.completed != fixture.cancel_after || report.outcomes.len() != fixture.cancel_after || report.total != fixture.plan.invocations.len() {
        return Err(format!("cancelled {}, completed {} of {}, outcomes {}", report.cancelled, report.completed, report.total, report.outcomes.len()));
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("report".to_string(), report_json(&report)?),
        ("cancelledAfter".to_string(), Json::Number(fixture.cancel_after as f64)),
        ("skipped".to_string(), Json::Number((report.total - report.completed) as f64)),
    ])))
}

fn progress_is_reported_before_and_after_every_invocation(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let runner = subject::RecordedProcessRunner::new(fixture.transcripts.clone());
    let mut events: Vec<subject::ProgressEvent> = Vec::new();
    let report = subject::execute_plan(&fixture.plan, &runner, &subject::CancellationToken::new(), &mut |event| events.push(event));
    let expected = 2 + 2 * fixture.plan.invocations.len();
    if report.cancelled || report.completed != fixture.plan.invocations.len() || events.len() != expected {
        return Err(format!("completed {} of {}, cancelled {}, {} event(s) where {expected} were expected", report.completed, report.total, report.cancelled, events.len()));
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("events".to_string(), parse_json(&subject::progress_to_json_text(&events))?),
        ("completed".to_string(), Json::Number(report.completed as f64)),
        ("cancelled".to_string(), Json::Bool(report.cancelled)),
    ])))
}

fn a_missing_transcript_is_a_problem_not_a_pass(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let runner = subject::RecordedProcessRunner::default();
    let report = subject::execute_plan(&fixture.plan, &runner, &subject::CancellationToken::new(), &mut |_| {});
    let statuses = report
        .outcomes
        .iter()
        .map(|outcome| Json::String(if outcome.status == subject::RunStatus::NotRun { "not-run".to_string() } else { "ran".to_string() }))
        .collect();
    if report.problems.len() != fixture.plan.invocations.len() || report.outcomes.iter().any(|outcome| outcome.status != subject::RunStatus::NotRun) {
        return Err(format!("{} problem(s) for {} invocation(s)", report.problems.len(), fixture.plan.invocations.len()));
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("statuses".to_string(), Json::Array(statuses)),
        ("problems".to_string(), Json::Array(report.problems.iter().map(|problem| Json::String(problem.clone())).collect())),
        ("anyPassed".to_string(), Json::Bool(report.outcomes.iter().any(|outcome| outcome.status == subject::RunStatus::Passed))),
    ])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("cancelling-mid-plan-keeps-completed-outcomes", cancelling_mid_plan_keeps_completed_outcomes)
        .subject("progress-is-reported-before-and-after-every-invocation", progress_is_reported_before_and_after_every_invocation)
        .subject("a-missing-transcript-is-a-problem-not-a-pass", a_missing_transcript_is_a_problem_not_a_pass)
}
//#endregion 🔖️Registration
