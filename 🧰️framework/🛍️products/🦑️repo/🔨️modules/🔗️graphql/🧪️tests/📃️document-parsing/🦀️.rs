//! 🦀️ Rust side of the document-parsing case. Written independently of the Go adapter against the
//! same frozen corpus and the same projection schema — pairwise equivalence is the point.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{Context, Json, Outcome};
    use semio_framework_repo_graphql::serde_json::Value as SerdeJson;
    use semio_framework_repo_graphql::{operation_type, parse};

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

    fn corpus(ctx: &Context) -> Result<Vec<(String, String)>, String> {
        let bytes = ctx.fixture_bytes("shared://📃️document-parsing/🔣️documents.json")?;
        let text = String::from_utf8(bytes).map_err(|error| error.to_string())?;
        let parsed: SerdeJson = semio_framework_repo_graphql::serde_json::from_str(&text).map_err(|error| error.to_string())?;
        let documents = parsed.get("documents").and_then(SerdeJson::as_array).ok_or("corpus has no documents array")?;
        documents
            .iter()
            .map(|entry| {
                let id = entry.get("id").and_then(SerdeJson::as_str).ok_or("document has no id")?.to_string();
                let source = entry.get("source").and_then(SerdeJson::as_str).ok_or("document has no source")?.to_string();
                Ok((id, source))
            })
            .collect()
    }

    pub fn corpus_projects_identically(ctx: &Context) -> Result<Outcome, String> {
        let rows = corpus(ctx)?
            .into_iter()
            .map(|(id, source)| {
                let document = parse(&source).map_err(|error| format!("{id}: {error}"))?;
                Ok(Json::Object(vec![("id".to_string(), Json::String(id)), ("document".to_string(), to_host(&document.projection()))]))
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Outcome::projection(Json::Object(vec![("documents".to_string(), Json::Array(rows))])))
    }

    pub fn operation_kind_is_recovered(ctx: &Context) -> Result<Outcome, String> {
        let rows = corpus(ctx)?
            .into_iter()
            .map(|(id, source)| {
                let operation = operation_type(&source).map_err(|error| format!("{id}: {error}"))?;
                Ok(Json::Object(vec![("id".to_string(), Json::String(id)), ("operation".to_string(), Json::String(operation))]))
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Outcome::projection(Json::Object(vec![("documents".to_string(), Json::Array(rows))])))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("corpus-projects-identically", subject::corpus_projects_identically).subject("operation-kind-is-recovered", subject::operation_kind_is_recovered);
    adapter
}
//#endregion 🔖️Registration
