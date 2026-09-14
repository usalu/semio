//! 🦀️ Rust side of the execution-errors case. Every input of the corpus MUST be refused with the
//! verbatim message the corpus records, and a refused mutation MUST leave no trace behind it.

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

    /// 🗄️ Builds the context every input runs against.
    fn context(ctx: &Context) -> Result<RecordingContext, String> {
        RecordingContext::from_json(&fixture(ctx, "shared://🔣️repo-records.json")?).map_err(|error| error.to_string())
    }

    /// ❌️ One recorded refusal: an input, the message it owes, and whether it is a write.
    struct Refusal {
        id: String,
        source: String,
        message: String,
        mutation: bool,
    }

    /// 📥️ Reads the refusal corpus.
    fn corpus(ctx: &Context) -> Result<Vec<Refusal>, String> {
        let parsed = fixture(ctx, "local://🔣️refusals.json")?;
        let rows = parsed.get("refusals").and_then(SerdeJson::as_array).ok_or("corpus has no refusals array")?;
        rows.iter()
            .map(|entry| {
                Ok(Refusal {
                    id: entry.get("id").and_then(SerdeJson::as_str).ok_or("refusal has no id")?.to_string(),
                    source: entry.get("source").and_then(SerdeJson::as_str).ok_or("refusal has no source")?.to_string(),
                    message: entry.get("message").and_then(SerdeJson::as_str).ok_or("refusal has no message")?.to_string(),
                    mutation: entry.get("mutation").and_then(SerdeJson::as_bool).unwrap_or(false),
                })
            })
            .collect()
    }

    pub fn every_refusal_carries_its_verbatim_message(ctx: &Context) -> Result<Outcome, String> {
        let context = context(ctx)?;
        let executor = Executor::new(&context);
        let mut rows = Vec::new();
        for refusal in corpus(ctx)? {
            let said = match executor.execute(&refusal.source, &Map::new()) {
                Ok(data) => return Err(format!("{}: answered instead of refusing — {data}", refusal.id)),
                Err(error) => error.message,
            };
            if said != refusal.message {
                return Err(format!("{}: expected {:?}, got {:?}", refusal.id, refusal.message, said));
            }
            rows.push(Json::Object(vec![("input".to_string(), Json::String(refusal.id)), ("message".to_string(), Json::String(said))]));
        }
        Ok(Outcome::projection(Json::Object(vec![("refusals".to_string(), Json::Array(rows))])))
    }

    pub fn a_refusal_writes_no_event_and_changes_no_record(ctx: &Context) -> Result<Outcome, String> {
        let context = context(ctx)?;
        let before = context.snapshot();
        let executor = Executor::new(&context);
        let mut rows = Vec::new();
        for refusal in corpus(ctx)?.into_iter().filter(|refusal| refusal.mutation) {
            let said = match executor.execute(&refusal.source, &Map::new()) {
                Ok(data) => return Err(format!("{}: answered instead of refusing — {data}", refusal.id)),
                Err(error) => error.message,
            };
            rows.push(Json::Object(vec![("input".to_string(), Json::String(refusal.id)), ("message".to_string(), Json::String(said))]));
        }
        let after = context.snapshot();
        if before != after {
            return Err("a refused mutation changed the record set".to_string());
        }
        Ok(Outcome::projection(Json::Object(vec![
            ("mutations".to_string(), Json::Array(rows)),
            ("events".to_string(), Json::Array(context.events().iter().map(to_host).collect())),
            ("records".to_string(), to_host(&after)),
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
        .subject("every-refusal-carries-its-verbatim-message", subject::every_refusal_carries_its_verbatim_message)
        .subject("a-refusal-writes-no-event-and-changes-no-record", subject::a_refusal_writes_no_event_and_changes_no_record);
    adapter
}
//#endregion 🔖️Registration
