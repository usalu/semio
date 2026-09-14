//! 🦀️ Rust subject for the verb → document → engine → renderer round trip. Gated behind the `sut`
//! feature like every subject, so the oracle role never links the crate under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_cli::repo_cli;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The frozen repository every vector executes against.
    fn records(ctx: &Context) -> Result<String, String> {
        Ok(String::from_utf8_lossy(&ctx.fixture_bytes("local://🗄️repo-records.json")?).to_string())
    }

    /// 📥️ The committed vectors every scenario reads.
    fn vectors(ctx: &Context) -> Result<Vec<Json>, String> {
        Ok(ctx.fixture_json("local://🔁️verb-queries.json")?.array("vectors"))
    }

    /// ▶️ Executes one vector and returns its three renderings and exit code.
    fn roundtrip(records: &str, vector: &Json) -> Result<Json, String> {
        let variables = vector.get("variables").cloned().unwrap_or(Json::Object(Vec::new())).to_string();
        let encoded = repo_cli::graphql_roundtrip_json(records, &vector.str("query"), &variables)?;
        parse_json(&encoded)
    }

    /// ❓️ Whether the vector expects a refusal.
    fn refuses(vector: &Json) -> bool {
        matches!(vector.get("expectError"), Some(Json::Bool(true)))
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    /// 🔁️ Every accepting document executes and renders in all three formats.
    pub fn every_verb_document_executes_and_renders(ctx: &Context) -> Result<Outcome, String> {
        let records = records(ctx)?;
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            if refuses(&vector) {
                continue;
            }
            let result = roundtrip(&records, &vector)?;
            let exit = match result.get("exitCode") {
                Some(Json::Number(value)) => *value as i64,
                _ => -1,
            };
            if exit != 0 {
                return Err(format!("{}: expected a successful stream, got exit {exit}", vector.str("id")));
            }
            let ndjson = result.str("ndjson");
            if ndjson.lines().count() != 1 {
                return Err(format!("{}: NDJSON must carry exactly one line, got {}", vector.str("id"), ndjson.lines().count()));
            }
            parse_json(ndjson.trim_end()).map_err(|error| format!("{}: NDJSON is not one JSON document: {error}", vector.str("id")))?;
            if let Some(markers) = vector.get("mustContain") {
                for format in ["ndjson", "markdown", "human"] {
                    let marker = markers.str(format);
                    if marker.is_empty() {
                        continue;
                    }
                    if !result.str(format).contains(&marker) {
                        return Err(format!("{}: the {format} rendering is missing {marker}", vector.str("id")));
                    }
                }
            }
            rows.push(Json::Object(vec![
                ("id".to_string(), Json::String(vector.str("id"))),
                ("verb".to_string(), Json::String(vector.str("verb"))),
                ("ndjsonLines".to_string(), Json::Number(1.0)),
                ("markdownLines".to_string(), Json::Number(result.str("markdown").lines().count() as f64)),
                ("humanLines".to_string(), Json::Number(result.str("human").lines().count() as f64)),
            ]));
        }
        Ok(Outcome::projection(Json::Object(vec![("roundtrips".to_string(), Json::Array(rows))])))
    }

    /// ❌️ A refused document produces an error event and a non-zero exit code.
    pub fn an_execution_failure_becomes_a_failing_stream(ctx: &Context) -> Result<Outcome, String> {
        let records = records(ctx)?;
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            if !refuses(&vector) {
                continue;
            }
            let result = roundtrip(&records, &vector)?;
            let exit = match result.get("exitCode") {
                Some(Json::Number(value)) => *value as i64,
                _ => -1,
            };
            if exit != 1 {
                return Err(format!("{}: a refusal owes exit code 1, got {exit}", vector.str("id")));
            }
            if !result.str("ndjson").is_empty() {
                return Err(format!("{}: a refusal must write nothing to standard output", vector.str("id")));
            }
            rows.push(Json::Object(vec![("id".to_string(), Json::String(vector.str("id"))), ("exitCode".to_string(), Json::Number(1.0))]));
        }
        Ok(Outcome::projection(Json::Object(vec![("refused".to_string(), Json::Array(rows))])))
    }

    /// 🔁️ The same document rendered twice produces the same bytes.
    pub fn rendering_is_deterministic(ctx: &Context) -> Result<Outcome, String> {
        let records = records(ctx)?;
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            if refuses(&vector) {
                continue;
            }
            if roundtrip(&records, &vector)? != roundtrip(&records, &vector)? {
                return Err(format!("{}: rendering is not deterministic", vector.str("id")));
            }
            rows.push(Json::String(vector.str("id")));
        }
        Ok(Outcome::projection(Json::Object(vec![("stable".to_string(), Json::Array(rows))])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("every-verb-document-executes-and-renders", subject::every_verb_document_executes_and_renders)
        .subject("an-execution-failure-becomes-a-failing-stream", subject::an_execution_failure_becomes_a_failing_stream)
        .subject("rendering-is-deterministic", subject::rendering_is_deterministic);
    adapter
}
//#endregion 🔖️Registration
