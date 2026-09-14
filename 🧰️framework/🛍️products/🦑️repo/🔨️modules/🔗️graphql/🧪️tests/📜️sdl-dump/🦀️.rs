//! 🦀️ Rust side of the sdl-dump case. It reports the inventory of the schema the executor builds
//! in code — never of the committed document, which is the oracle's input, not the subject's.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{Context, Json, Outcome};
    use semio_framework_repo_graphql::serde_json::Value as SerdeJson;
    use semio_framework_repo_graphql::{build_schema, schema_inventory};

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

    pub fn served_schema_matches_the_committed_sdl(_ctx: &Context) -> Result<Outcome, String> {
        Ok(Outcome::projection(to_host(&schema_inventory(&build_schema()))))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("served-schema-matches-the-committed-sdl", subject::served_schema_matches_the_committed_sdl);
    adapter
}
//#endregion 🔖️Registration
