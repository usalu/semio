//! 🦀️ Rust side of the query-execution case. It runs the owner's own executor over the frozen
//! repository records and emits the `data` payload of every query — nothing else.

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

    /// ⚙️ Runs every query whose id the predicate accepts.
    fn run(ctx: &Context, keep: fn(&SerdeJson) -> bool) -> Result<Outcome, String> {
        let records = fixture(ctx, "shared://🔣️repo-records.json")?;
        let corpus = fixture(ctx, "shared://▶️query-execution/🔣️queries.json")?;
        let context = RecordingContext::from_json(&records).map_err(|error| error.to_string())?;
        let executor = Executor::new(&context);
        let queries = corpus.get("queries").and_then(SerdeJson::as_array).ok_or("corpus has no queries array")?;
        let mut rows = Vec::new();
        for entry in queries.iter().filter(|entry| keep(entry)) {
            let id = entry.get("id").and_then(SerdeJson::as_str).ok_or("query has no id")?.to_string();
            let source = entry.get("source").and_then(SerdeJson::as_str).ok_or("query has no source")?;
            let variables: Map<String, SerdeJson> = match entry.get("variables") {
                Some(SerdeJson::Object(fields)) => fields.clone(),
                _ => Map::new(),
            };
            let data = executor.execute(source, &variables).map_err(|error| format!("{id}: {error}"))?;
            rows.push(Json::Object(vec![("id".to_string(), Json::String(id)), ("data".to_string(), to_host(&data))]));
        }
        Ok(Outcome::projection(Json::Object(vec![("queries".to_string(), Json::Array(rows))])))
    }

    /// 🔷️ Whether a query carries an argument or a variable.
    fn parameterised(entry: &SerdeJson) -> bool {
        entry.get("variables").is_some() || entry.get("source").and_then(SerdeJson::as_str).is_some_and(|source| source.contains('('))
    }

    pub fn corpus_executes_identically(ctx: &Context) -> Result<Outcome, String> {
        run(ctx, |_| true)
    }

    pub fn arguments_coerce_before_resolution(ctx: &Context) -> Result<Outcome, String> {
        run(ctx, parameterised)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("corpus-executes-identically", subject::corpus_executes_identically)
        .subject("arguments-coerce-before-resolution", subject::arguments_coerce_before_resolution);
    adapter
}
//#endregion 🔖️Registration
