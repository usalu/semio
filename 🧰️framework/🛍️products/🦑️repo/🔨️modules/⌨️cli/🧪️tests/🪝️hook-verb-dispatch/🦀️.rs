//! 🦀️ Rust subject for the `hook` verb dispatch case. Gated behind the `sut` feature like every
//! subject, so the oracle role never links the crate under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_cli::repo_cli;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The committed invocations every scenario reads.
    fn vectors(ctx: &Context) -> Result<Vec<Json>, String> {
        Ok(ctx.fixture_json("shared://🪝️hook-verb-dispatch/🪝️hook-invocations.json")?.array("vectors"))
    }

    /// 🪝️ The bytes and exit code one invocation writes.
    fn dispatch(vector: &Json) -> Result<Json, String> {
        let request = vector.get("request").cloned().unwrap_or(Json::Null).to_string();
        parse_json(&repo_cli::hook_verb_dispatch_json(&request)?)
    }

    /// ❓️ Whether a vector states that the verb refuses it.
    fn refuses(vector: &Json) -> bool {
        matches!(vector.get("refused"), Some(Json::Bool(true)))
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    /// 🖨️ Every accepted invocation writes the bytes and exit code its client reads.
    pub fn every_invocation_writes_its_bytes(ctx: &Context) -> Result<Outcome, String> {
        let mut rows: Vec<(String, Json)> = Vec::new();
        for vector in vectors(ctx)? {
            if refuses(&vector) {
                continue;
            }
            rows.push((vector.str("id"), dispatch(&vector)?));
        }
        Ok(Outcome::projection(Json::Object(rows)))
    }

    /// 🚫️ An event slug no client owns is refused before the hook domain sees it.
    pub fn a_refusal_never_reaches_the_domain(ctx: &Context) -> Result<Outcome, String> {
        let mut rows: Vec<(String, Json)> = Vec::new();
        for vector in vectors(ctx)? {
            if !refuses(&vector) {
                continue;
            }
            match dispatch(&vector) {
                Ok(answer) => return Err(format!("{}: expected a refusal, got {}", vector.str("id"), answer.to_string())),
                Err(message) => rows.push((vector.str("id"), Json::Bool(!message.is_empty()))),
            }
        }
        Ok(Outcome::projection(Json::Object(rows)))
    }

    /// 🔁️ Dispatching the same invocation twice writes the same bytes.
    pub fn dispatch_is_deterministic(ctx: &Context) -> Result<Outcome, String> {
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            if refuses(&vector) {
                continue;
            }
            if dispatch(&vector)?.to_string() != dispatch(&vector)?.to_string() {
                return Err(format!("{}: dispatch is not deterministic", vector.str("id")));
            }
            rows.push(Json::String(vector.str("id")));
        }
        Ok(Outcome::projection(Json::Object(vec![("stable".to_string(), Json::Array(rows))])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🪝️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("every-invocation-writes-its-bytes", subject::every_invocation_writes_its_bytes)
        .subject("a-refusal-never-reaches-the-domain", subject::a_refusal_never_reaches_the_domain)
        .subject("dispatch-is-deterministic", subject::dispatch_is_deterministic);
    adapter
}
//#endregion 🔖️Registration
