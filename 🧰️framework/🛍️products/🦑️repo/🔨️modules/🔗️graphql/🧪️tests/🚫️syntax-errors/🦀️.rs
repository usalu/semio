//! 🦀️ Rust side of the malformed-input case. Rejection is the observation; `detail` carries the
//! verbatim message and is dropped by the `diagnostic-v1` profile before comparison.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{Context, Json, Outcome};
    use semio_framework_repo_graphql::serde_json::Value as SerdeJson;
    use semio_framework_repo_graphql::validate;

    fn corpus(ctx: &Context) -> Result<Vec<(String, String)>, String> {
        let bytes = ctx.fixture_bytes("local://🔣️malformed.json")?;
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

    pub fn malformed_inputs_are_rejected(ctx: &Context) -> Result<Outcome, String> {
        let rows = corpus(ctx)?
            .into_iter()
            .map(|(id, source)| {
                let failure = validate(&source).err();
                Json::Object(vec![
                    ("input".to_string(), Json::String(id)),
                    ("rejected".to_string(), Json::Bool(failure.is_some())),
                    ("detail".to_string(), Json::String(failure.map(|error| error.message).unwrap_or_default())),
                ])
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
    let adapter = adapter.subject("malformed-inputs-are-rejected", subject::malformed_inputs_are_rejected);
    adapter
}
//#endregion 🔖️Registration
