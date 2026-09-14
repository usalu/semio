//! 🦀️ Rust side of the MCP client identity case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_providers::{hook_client_for_mcp_kind, mcp_kind_from_resolved_client, mcp_server_name, parse_mcp_client_kind};
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "local://🪪️client-kinds.json";

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

    fn parsed(input: &str) -> String {
        match parse_mcp_client_kind(input) {
            Ok(kind) => kind.as_str().to_string(),
            Err(_) => "refused".to_string(),
        }
    }

    /// 🪪️ Every accepted spelling parses to one kind.
    pub fn every_accepted_spelling_parses_to_one_kind(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        Ok(Outcome::projection(Json::Array(strings(&vectors, "inputs").iter().map(|input| Json::String(format!("{input:?}={}", parsed(input)))).collect())))
    }

    /// 🏷️ Each kind names one server and one hook client.
    pub fn each_kind_names_one_server_and_one_hook_client(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let mut projected = Vec::new();
        for slug in strings(&vectors, "kinds") {
            let kind = parse_mcp_client_kind(&slug).map_err(|error| error.message)?;
            projected.push((slug, Json::Object(vec![("server".to_string(), Json::String(mcp_server_name(kind).to_string())), ("hookClient".to_string(), Json::String(hook_client_for_mcp_kind(kind).to_string()))])));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🔁️ A resolved client slug maps back to a kind.
    pub fn a_resolved_client_maps_back_to_a_kind(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        Ok(Outcome::projection(Json::Array(
            strings(&vectors, "resolvedClients").iter().map(|client| Json::String(format!("{client:?}={}", mcp_kind_from_resolved_client(client).as_str()))).collect(),
        )))
    }

    /// ⚠️ An unknown spelling is refused.
    pub fn an_unknown_spelling_is_refused(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let refused: Vec<Json> = strings(&vectors, "inputs").iter().filter(|input| parsed(input) == "refused").map(|input| Json::String(input.clone())).collect();
        Ok(Outcome::projection(Json::Object(vec![
            ("refused".to_string(), Json::Array(refused)),
            ("messageForUnknown".to_string(), Json::String(parse_mcp_client_kind("unknown").err().map(|error| error.message).unwrap_or_default())),
        ])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered
        .subject("every-accepted-spelling-parses-to-one-kind", subject::every_accepted_spelling_parses_to_one_kind)
        .subject("each-kind-names-one-server-and-one-hook-client", subject::each_kind_names_one_server_and_one_hook_client)
        .subject("a-resolved-client-maps-back-to-a-kind", subject::a_resolved_client_maps_back_to_a_kind)
        .subject("an-unknown-spelling-is-refused", subject::an_unknown_spelling_is_refused);
    registered
}
//#endregion 🔖️Registration
