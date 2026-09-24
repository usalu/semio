//! 🦀️ Rust side of the subset-boundary case. Both edges are recorded, with the verbatim diagnostic:
//! there is no conforming reference for a deliberate subset, so Go and Rust judge each other.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{Context, Json, Outcome};
    use semio_framework_repo_graphql::parse;
    use semio_framework_repo_graphql::serde_json::Value as SerdeJson;

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
        let bytes = ctx.fixture_bytes("shared://🙅️unsupported-syntax/🔣️divergences.json")?;
        let text = String::from_utf8(bytes).map_err(|error| error.to_string())?;
        let parsed: SerdeJson = semio_framework_repo_graphql::serde_json::from_str(&text).map_err(|error| error.to_string())?;
        let inputs = parsed.get("inputs").and_then(SerdeJson::as_array).ok_or("corpus has no inputs array")?;
        inputs
            .iter()
            .map(|entry| {
                let id = entry.get("id").and_then(SerdeJson::as_str).ok_or("input has no id")?.to_string();
                let source = entry.get("source").and_then(SerdeJson::as_str).ok_or("input has no source")?.to_string();
                Ok((id, source))
            })
            .collect()
    }

    pub fn subset_boundary_is_identical(ctx: &Context) -> Result<Outcome, String> {
        let rows = corpus(ctx)?
            .into_iter()
            .map(|(id, source)| {
                let mut row = vec![("input".to_string(), Json::String(id))];
                match parse(&source) {
                    Ok(document) => {
                        row.push(("accepted".to_string(), Json::Bool(true)));
                        row.push(("message".to_string(), Json::String(String::new())));
                        row.push(("document".to_string(), to_host(&document.projection())));
                    }
                    Err(error) => {
                        row.push(("accepted".to_string(), Json::Bool(false)));
                        row.push(("message".to_string(), Json::String(error.message)));
                    }
                }
                Json::Object(row)
            })
            .collect::<Vec<_>>();
        Ok(Outcome::projection(Json::Object(vec![("inputs".to_string(), Json::Array(rows))])))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("subset-boundary-is-identical", subject::subset_boundary_is_identical);
    adapter
}
//#endregion 🔖️Registration
