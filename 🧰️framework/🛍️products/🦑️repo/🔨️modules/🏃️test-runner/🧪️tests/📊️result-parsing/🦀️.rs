//! 🦀️ Rust side of the result-parsing case: every runner dialect folded into one outcome model.

use semio_framework_repo_test_runner as subject;
use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Support
fn vectors(ctx: &Context) -> Result<subject::TranscriptVectors, String> {
    subject::parse_transcript_vectors(&ctx.fixture_bytes("shared://📜️runner-transcripts.json")?)
}

fn outcome_json(outcome: &subject::TestOutcome) -> Result<Json, String> {
    parse_json(&subject::outcome_to_json_text(outcome))
}
//#endregion 🔖️Support

//#region 🔖️Scenarios
fn every_dialect_folds_into_one_model(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let mut rows = Vec::new();
    for vector in &fixture.vectors {
        let parsed = subject::parse_outcome(vector.runner, &vector.output);
        rows.push(Json::Object(vec![("id".to_string(), Json::String(vector.id.clone())), ("outcome".to_string(), outcome_json(&parsed)?)]));
    }
    Ok(Outcome::projection(Json::Object(vec![("parsed".to_string(), Json::Array(rows))])))
}

fn totals_are_recomputed_not_trusted(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let mut rows = Vec::new();
    for vector in &fixture.vectors {
        let parsed = subject::parse_outcome(vector.runner, &vector.output);
        let passed = parsed.tests.iter().filter(|test| test.status == subject::TestStatus::Passed).count();
        let failed = parsed.tests.iter().filter(|test| test.status == subject::TestStatus::Failed).count();
        let skipped = parsed.tests.iter().filter(|test| test.status == subject::TestStatus::Skipped).count();
        rows.push(Json::Object(vec![
            ("id".to_string(), Json::String(vector.id.clone())),
            ("total".to_string(), Json::Number(parsed.totals.total as f64)),
            ("passed".to_string(), Json::Number(parsed.totals.passed as f64)),
            ("failed".to_string(), Json::Number(parsed.totals.failed as f64)),
            ("skipped".to_string(), Json::Number(parsed.totals.skipped as f64)),
            (
                "recountAgrees".to_string(),
                Json::Bool(passed == parsed.totals.passed && failed == parsed.totals.failed && skipped == parsed.totals.skipped && parsed.tests.len() == parsed.totals.total),
            ),
        ]));
        if passed != parsed.totals.passed || failed != parsed.totals.failed || skipped != parsed.totals.skipped || parsed.tests.len() != parsed.totals.total {
            return Err(format!("{}: totals disagree with the parsed tests", vector.id));
        }
    }
    Ok(Outcome::projection(Json::Object(vec![("totals".to_string(), Json::Array(rows))])))
}

fn unparseable_output_is_an_empty_run(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let vector = fixture
        .vectors
        .iter()
        .find(|vector| vector.id == "unparseable-output-yields-no-tests")
        .ok_or_else(|| "the transcript fixture has no unparseable vector".to_string())?;
    let parsed = subject::parse_outcome(vector.runner, &vector.output);
    if !parsed.tests.is_empty() || parsed.status != subject::RunStatus::Failed || parsed.totals.total != 0 {
        return Err(format!("unparseable output produced {} test(s) and status {:?}", parsed.tests.len(), parsed.status));
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("tests".to_string(), Json::Number(parsed.tests.len() as f64)),
        ("status".to_string(), Json::String(if parsed.status == subject::RunStatus::Failed { "failed".to_string() } else { "not-failed".to_string() })),
        ("total".to_string(), Json::Number(parsed.totals.total as f64)),
        ("exitStatus".to_string(), Json::Number(parsed.exit_status.unwrap_or_default() as f64)),
    ])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("every-dialect-folds-into-one-model", every_dialect_folds_into_one_model)
        .subject("totals-are-recomputed-not-trusted", totals_are_recomputed_not_trusted)
        .subject("unparseable-output-is-an-empty-run", unparseable_output_is_an_empty_run)
}
//#endregion 🔖️Registration
