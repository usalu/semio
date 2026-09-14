//! 🦀️ Rust side of the scope-identifier case: flattening and URI path decoding.

use semio_framework_repo_test_runner as subject;
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Support
fn selectors(ctx: &Context) -> Result<Vec<String>, String> {
    Ok(subject::parse_selector_vectors(&ctx.fixture_bytes("shared://🧬️scope-selectors.json")?)?.selectors)
}

fn strings(values: Vec<String>) -> Json {
    Json::Array(values.into_iter().map(Json::String).collect())
}
//#endregion 🔖️Support

//#region 🔖️Scenarios
fn flattening_agrees_across_implementations(ctx: &Context) -> Result<Outcome, String> {
    let flattened = selectors(ctx)?.iter().map(|selector| subject::flat(selector)).collect();
    Ok(Outcome::projection(Json::Object(vec![("flattened".to_string(), strings(flattened))])))
}

fn uri_path_decoding_agrees_across_implementations(ctx: &Context) -> Result<Outcome, String> {
    let decoded = selectors(ctx)?.iter().map(|selector| subject::path_from_uri_path(selector)).collect();
    Ok(Outcome::projection(Json::Object(vec![("decoded".to_string(), strings(decoded))])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("flattening-agrees-across-implementations", flattening_agrees_across_implementations)
        .subject("uri-path-decoding-agrees-across-implementations", uri_path_decoding_agrees_across_implementations)
}
//#endregion 🔖️Registration
