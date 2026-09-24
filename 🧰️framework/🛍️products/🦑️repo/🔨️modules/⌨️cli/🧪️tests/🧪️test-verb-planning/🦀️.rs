//! 🦀️ Rust subject for the `test` verb planning case. Gated behind the `sut` feature like every
//! subject, so the oracle role never links the crate under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_cli::repo_cli;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The committed document every scenario reads.
    fn document(ctx: &Context) -> Result<Json, String> {
        ctx.fixture_json("shared://🧪️test-verb-planning/🧪️test-verb-vectors.json")
    }

    /// 🌍️ The frozen snapshot, as the JSON text the verb entry point takes.
    fn snapshot(document: &Json) -> String {
        document.get("snapshot").cloned().unwrap_or(Json::Null).to_string()
    }

    /// 📜️ A JSON string array as a `Vec<String>`.
    fn strings(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .into_iter()
            .map(|item| match item {
                Json::String(text) => text,
                other => other.to_string(),
            })
            .collect()
    }

    /// 🧾️ The announcement of one vector: its lines and its refusals.
    fn plan(document: &Json, vector: &Json) -> Result<Json, String> {
        let encoded = repo_cli::test_verb_lines_json(&snapshot(document), &strings(vector, "operands"))?;
        parse_json(&encoded)
    }

    /// ⚖️ The stated announcement of one vector, in the same shape a plan projects.
    fn stated(vector: &Json) -> Json {
        Json::Object(vec![
            ("lines".to_string(), Json::Array(strings(vector, "lines").into_iter().map(Json::String).collect())),
            ("problems".to_string(), Json::Array(strings(vector, "problems").into_iter().map(Json::String).collect())),
        ])
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    /// 🧪️ Every operand list plans the invocations it states, in order.
    pub fn operands_plan_their_stated_invocations(ctx: &Context) -> Result<Outcome, String> {
        let document = document(ctx)?;
        let mut rows: Vec<(String, Json)> = Vec::new();
        for vector in document.array("vectors") {
            let actual = plan(&document, &vector)?;
            let wanted = stated(&vector);
            if actual.to_string() != wanted.to_string() {
                return Err(format!("{}: expected {} got {}", vector.str("id"), wanted.to_string(), actual.to_string()));
            }
            rows.push((vector.str("id"), actual));
        }
        Ok(Outcome::projection(Json::Object(rows)))
    }

    /// 🚫️ A scope with no detectable runner refuses and announces nothing for itself.
    pub fn unplannable_scopes_refuse_instead_of_running(ctx: &Context) -> Result<Outcome, String> {
        let document = document(ctx)?;
        let mut rows: Vec<(String, Json)> = Vec::new();
        for vector in document.array("vectors") {
            let wanted = strings(&vector, "problems");
            if wanted.is_empty() {
                continue;
            }
            let actual = plan(&document, &vector)?;
            let problems: Vec<String> = actual.array("problems").into_iter().map(|item| match item { Json::String(text) => text, other => other.to_string() }).collect();
            if problems != wanted {
                return Err(format!("{}: expected problems {wanted:?} got {problems:?}", vector.str("id")));
            }
            if !actual.array("lines").is_empty() {
                return Err(format!("{}: a refusing vector announced a runner", vector.str("id")));
            }
            rows.push((vector.str("id"), Json::Array(problems.into_iter().map(Json::String).collect())));
        }
        Ok(Outcome::projection(Json::Object(rows)))
    }

    /// 🔁️ Planning the same operands twice announces the same lines.
    pub fn planning_is_deterministic(ctx: &Context) -> Result<Outcome, String> {
        let document = document(ctx)?;
        let mut rows: Vec<Json> = Vec::new();
        for vector in document.array("vectors") {
            let first = repo_cli::test_verb_lines_json(&snapshot(&document), &strings(&vector, "operands"))?;
            let second = repo_cli::test_verb_lines_json(&snapshot(&document), &strings(&vector, "operands"))?;
            if first != second {
                return Err(format!("{}: planning is not deterministic", vector.str("id")));
            }
            rows.push(Json::String(vector.str("id")));
        }
        Ok(Outcome::projection(Json::Object(vec![("stable".to_string(), Json::Array(rows))])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧪️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("operands-plan-their-stated-invocations", subject::operands_plan_their_stated_invocations)
        .subject("unplannable-scopes-refuse-instead-of-running", subject::unplannable_scopes_refuse_instead_of_running)
        .subject("planning-is-deterministic", subject::planning_is_deterministic);
    adapter
}
//#endregion 🔖️Registration
