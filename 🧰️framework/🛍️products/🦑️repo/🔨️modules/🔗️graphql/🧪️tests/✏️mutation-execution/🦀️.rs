//! 🦀️ Rust side of the mutation-execution case. It runs the whole write script against one
//! recording context and reports the payloads, the resulting records and the event trail together.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{Context, Json, Outcome};
    use semio_framework_repo_graphql::serde_json::{Map, Value as SerdeJson};
    use semio_framework_repo_graphql::{Executor, RecordingContext};

    /// 🔣️ Bridges the crate's `serde_json` projection into the host's dependency-free JSON.
    pub fn to_host(value: &SerdeJson) -> Json {
        match value {
            SerdeJson::Null => Json::Null,
            SerdeJson::Bool(flag) => Json::Bool(*flag),
            SerdeJson::Number(number) => Json::Number(number.as_f64().unwrap_or_default()),
            SerdeJson::String(text) => Json::String(text.clone()),
            SerdeJson::Array(items) => Json::Array(items.iter().map(to_host).collect()),
            SerdeJson::Object(fields) => Json::Object(fields.iter().map(|(name, item)| (name.clone(), to_host(item))).collect()),
        }
    }

    /// 📥️ Reads one JSON fixture into the crate's own JSON vocabulary.
    fn fixture(ctx: &Context, uri: &str) -> Result<SerdeJson, String> {
        let bytes = ctx.fixture_bytes(uri)?;
        let text = String::from_utf8(bytes).map_err(|error| error.to_string())?;
        semio_framework_repo_graphql::serde_json::from_str(&text).map_err(|error| error.to_string())
    }

    /// 🗄️ Builds the context the whole script runs against.
    fn context(ctx: &Context) -> Result<RecordingContext, String> {
        RecordingContext::from_json(&fixture(ctx, "shared://🔣️repo-records.json")?).map_err(|error| error.to_string())
    }

    pub fn script_changes_records_and_emits_events(ctx: &Context) -> Result<Outcome, String> {
        let script = fixture(ctx, "local://🔣️mutations.json")?;
        let context = context(ctx)?;
        let executor = Executor::new(&context);
        let mutations = script.get("mutations").and_then(SerdeJson::as_array).ok_or("script has no mutations array")?;
        let mut rows = Vec::new();
        for entry in mutations {
            let id = entry.get("id").and_then(SerdeJson::as_str).ok_or("mutation has no id")?.to_string();
            let source = entry.get("source").and_then(SerdeJson::as_str).ok_or("mutation has no source")?;
            let variables: Map<String, SerdeJson> = match entry.get("variables") {
                Some(SerdeJson::Object(fields)) => fields.clone(),
                _ => Map::new(),
            };
            let data = executor.execute(source, &variables).map_err(|error| format!("{id}: {error}"))?;
            rows.push(Json::Object(vec![("id".to_string(), Json::String(id)), ("data".to_string(), to_host(&data))]));
        }
        Ok(Outcome::projection(Json::Object(vec![
            ("mutations".to_string(), Json::Array(rows)),
            ("events".to_string(), Json::Array(context.events().iter().map(to_host).collect())),
            ("records".to_string(), to_host(&context.snapshot())),
        ])))
    }

    pub fn a_mutation_that_cannot_apply_is_refused(ctx: &Context) -> Result<Outcome, String> {
        let context = context(ctx)?;
        let executor = Executor::new(&context);
        let refusal = executor
            .execute("mutation { ticketClose(input: { year: 2026, month: 9, day: 6, slug: \"NO-SUCH-TICKET\", summary: \"never\" }) { status } }", &Map::new())
            .expect_err("a ticket that does not exist cannot be closed");
        Ok(Outcome::projection(Json::Object(vec![
            ("message".to_string(), Json::String(refusal.message)),
            ("events".to_string(), Json::Array(context.events().iter().map(to_host).collect())),
        ])))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("script-changes-records-and-emits-events", subject::script_changes_records_and_emits_events)
        .subject("a-mutation-that-cannot-apply-is-refused", subject::a_mutation_that_cannot_apply_is_refused);
    adapter
}
//#endregion 🔖️Registration
