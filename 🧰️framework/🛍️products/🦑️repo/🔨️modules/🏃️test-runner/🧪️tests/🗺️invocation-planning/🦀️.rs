//! 🦀️ Rust side of the invocation-planning case: scope in, ordered argv out, nothing executed.

use semio_framework_repo_test_runner as subject;
use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Support
fn vectors(ctx: &Context) -> Result<subject::PlanningVectors, String> {
    subject::parse_planning_vectors(&ctx.fixture_bytes("shared://🗺️planning-vectors.json")?)
}

fn plan_json(plan: &subject::InvocationPlan) -> Result<Json, String> {
    parse_json(&subject::plan_to_json_text(plan))
}

fn plan_of(fixture: &subject::PlanningVectors, id: &str) -> subject::InvocationPlan {
    fixture
        .vectors
        .iter()
        .find(|vector| vector.id == id)
        .map_or_else(subject::InvocationPlan::default, |vector| subject::plan_scope(&fixture.snapshot, &vector.scope))
}
//#endregion 🔖️Support

//#region 🔖️Scenarios
fn scope_plans_match_the_frozen_argv(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let mut rows = Vec::new();
    let mut wrong = Vec::new();
    for vector in &fixture.vectors {
        let planned = subject::plan_scope(&fixture.snapshot, &vector.scope);
        if !vector.expected.as_ref().is_some_and(|expected| *expected == planned) {
            wrong.push(format!("{}: planned {}", vector.id, subject::plan_to_json_text(&planned)));
        }
        rows.push(Json::Object(vec![("id".to_string(), Json::String(vector.id.clone())), ("plan".to_string(), plan_json(&planned)?)]));
    }
    if !wrong.is_empty() {
        return Err(wrong.join(" | "));
    }
    Ok(Outcome::projection(Json::Object(vec![("planned".to_string(), Json::Array(rows))])))
}

fn a_scope_with_no_runner_is_refused(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let mut rows = Vec::new();
    for vector in &fixture.vectors {
        let declares_problem = vector.expected.as_ref().is_some_and(|expected| !expected.problems.is_empty());
        if !declares_problem {
            continue;
        }
        let planned = subject::plan_scope(&fixture.snapshot, &vector.scope);
        let expected = vector.expected.clone().unwrap_or_default();
        if planned.problems != expected.problems || !planned.invocations.is_empty() {
            return Err(format!("{}: refusal is {:?} with {} invocation(s)", vector.id, planned.problems, planned.invocations.len()));
        }
        rows.push(Json::Object(vec![
            ("id".to_string(), Json::String(vector.id.clone())),
            ("problems".to_string(), Json::Array(planned.problems.iter().map(|problem| Json::String(problem.clone())).collect())),
            ("invocations".to_string(), Json::Number(planned.invocations.len() as f64)),
        ]));
    }
    if rows.is_empty() {
        return Err("the planning fixture declares no refusal vector".to_string());
    }
    Ok(Outcome::projection(Json::Object(vec![("refused".to_string(), Json::Array(rows))])))
}

fn narrowing_never_widens_the_plan(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let widths: Vec<usize> = ["whole-repository", "technology-semio", "bundle-go", "file-go-narrows-to-its-directory"]
        .iter()
        .map(|id| plan_of(&fixture, id).invocations.len())
        .collect();
    let monotonic = widths.windows(2).all(|pair| pair[0] >= pair[1]);
    let repeated = plan_of(&fixture, "whole-repository") == plan_of(&fixture, "whole-repository");
    if !monotonic || !repeated {
        return Err(format!("narrowing widths {widths:?}, repeatable {repeated}"));
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("widths".to_string(), Json::Array(widths.iter().map(|width| Json::Number(*width as f64)).collect())),
        ("narrowingIsMonotonic".to_string(), Json::Bool(monotonic)),
        ("planningIsRepeatable".to_string(), Json::Bool(repeated)),
    ])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("scope-plans-match-the-frozen-argv", scope_plans_match_the_frozen_argv)
        .subject("a-scope-with-no-runner-is-refused", a_scope_with_no_runner_is_refused)
        .subject("narrowing-never-widens-the-plan", narrowing_never_widens_the_plan)
}
//#endregion 🔖️Registration
