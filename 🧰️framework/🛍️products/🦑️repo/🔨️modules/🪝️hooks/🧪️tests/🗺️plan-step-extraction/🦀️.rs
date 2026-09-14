//! 🦀️ Rust side of the plan step extraction case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_hooks::{extract_plan_steps, merge_plan_steps, TicketAgentPlanStep, HookPlanStep};
    use semio_framework_repo_hooks::serde_json::{self, Value};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    const VECTORS: &str = "local://🗺️plans.json";

    fn vectors(ctx: &Context) -> Result<Value, String> {
        let bytes = ctx.fixture_bytes(VECTORS)?;
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())
    }

    fn text(value: &Value, key: &str) -> String {
        value.get(key).and_then(Value::as_str).unwrap_or_default().to_string()
    }

    fn rows(value: &Value, key: &str) -> Vec<Value> {
        value.get(key).and_then(Value::as_array).cloned().unwrap_or_default()
    }

    /// 📨️ A JSON `null` in the fixture means the IDE sent no payload at all, not a payload of `null`.
    fn payload(vector: &Value) -> Option<Value> {
        vector.get("input").filter(|value| !value.is_null()).cloned()
    }

    fn as_projection(value: &Value) -> Result<Json, String> {
        parse_json(&value.to_string())
    }

    fn merge_inputs(vector: &Value) -> Result<(Vec<TicketAgentPlanStep>, Vec<HookPlanStep>), String> {
        let existing: Vec<TicketAgentPlanStep> = serde_json::from_value(vector.get("existing").cloned().unwrap_or(Value::Array(Vec::new()))).map_err(|error| error.to_string())?;
        let incoming: Vec<HookPlanStep> = serde_json::from_value(vector.get("incoming").cloned().unwrap_or(Value::Array(Vec::new()))).map_err(|error| error.to_string())?;
        Ok((existing, incoming))
    }

    /// 🗺️ Every payload shape yields the same steps.
    pub fn every_payload_shape_yields_the_same_steps(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "payloads") {
            let steps = extract_plan_steps(payload(&vector).as_ref(), &text(&vector, "toolArgs"));
            projected.push((text(&vector, "id"), as_projection(&serde_json::to_value(&steps).map_err(|error| error.to_string())?)?));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🔀️ Folding an incoming plan keeps every lifecycle timestamp.
    pub fn folding_a_plan_keeps_its_history(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "merges") {
            let (existing, incoming) = merge_inputs(&vector)?;
            let merged = merge_plan_steps(&existing, &incoming, &text(&vector, "second"));
            projected.push((text(&vector, "id"), as_projection(&serde_json::to_value(&merged).map_err(|error| error.to_string())?)?));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// ♻️ Folding the same plan again changes nothing.
    pub fn folding_the_same_plan_twice_changes_nothing(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "merges") {
            let (existing, incoming) = merge_inputs(&vector)?;
            let first = merge_plan_steps(&existing, &incoming, &text(&vector, "second"));
            let second = merge_plan_steps(&first, &incoming, "2099-01-01T00:00:00Z");
            projected.push((text(&vector, "id"), Json::Object(vec![("stable".to_string(), Json::Bool(first == second)), ("second".to_string(), as_projection(&serde_json::to_value(&second).map_err(|error| error.to_string())?)?)])));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered
        .subject("every-payload-shape-yields-the-same-steps", subject::every_payload_shape_yields_the_same_steps)
        .subject("folding-a-plan-keeps-its-history", subject::folding_a_plan_keeps_its_history)
        .subject("folding-the-same-plan-twice-changes-nothing", subject::folding_the_same_plan_twice_changes_nothing);
    registered
}
//#endregion 🔖️Registration
