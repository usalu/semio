//! 🦀️ Rust side of the slug vocabulary case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only, so the case still compiles
//! without linking the implementation under test.

use semio_repo_test_host::{Adapter, Json, Outcome};

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_model as model;
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "shared://🔤️slug-normalisation/🔣️vectors.json";

    fn strings(root: &Json, group: &str, list: &str) -> Vec<String> {
        let Some(section) = root.get(group) else { return Vec::new() };
        section
            .array(list)
            .into_iter()
            .map(|item| match item {
                Json::String(value) => value,
                _ => String::new(),
            })
            .collect()
    }

    fn pairs(entries: Vec<(String, Json)>) -> Json {
        Json::Object(entries)
    }

    fn normalised(root: &Json, group: &str, normalise: fn(&str) -> String) -> Json {
        let mut once = Vec::new();
        let mut twice = Vec::new();
        for input in strings(root, group, "normalise") {
            let first = normalise(&input);
            twice.push(Json::String(normalise(&first)));
            once.push(Json::String(first));
        }
        pairs(vec![("once".to_string(), Json::Array(once)), ("twice".to_string(), Json::Array(twice))])
    }

    fn resolved(root: &Json, group: &str, resolve: fn(&str) -> Result<String, model::ModelError>) -> Json {
        Json::Array(
            strings(root, group, "resolve")
                .into_iter()
                .map(|input| match resolve(&input) {
                    Ok(value) => Json::String(value),
                    Err(error) => Json::String(format!("<{}>", error.class())),
                })
                .collect(),
        )
    }

    fn rejected(root: &Json, group: &str, resolve: fn(&str) -> Result<String, model::ModelError>) -> Json {
        Json::Array(
            strings(root, group, "reject")
                .into_iter()
                .map(|input| match resolve(&input) {
                    Ok(value) => Json::String(format!("<resolved:{value}>")),
                    Err(error) => Json::String(error.class().to_string()),
                })
                .collect(),
        )
    }

    /// 🔤️ Normalises every input, then normalises the result again.
    pub fn normalisation_is_canonical_and_idempotent(ctx: &Context) -> Result<Outcome, String> {
        let root = ctx.fixture_json(VECTORS)?;
        Ok(Outcome::projection(pairs(vec![
            ("llm".to_string(), normalised(&root, "llm", model::normalize_llm_slug)),
            ("effort".to_string(), normalised(&root, "effort", model::normalize_effort_slug)),
            ("client".to_string(), normalised(&root, "client", model::normalize_client_slug)),
        ])))
    }

    /// 🎯️ Resolves every input against the allowed table.
    pub fn resolution_picks_the_longest_allowed_match(ctx: &Context) -> Result<Outcome, String> {
        let root = ctx.fixture_json(VECTORS)?;
        Ok(Outcome::projection(pairs(vec![
            ("llm".to_string(), resolved(&root, "llm", model::resolve_allowed_llm)),
            ("effort".to_string(), resolved(&root, "effort", model::resolve_allowed_effort)),
            ("client".to_string(), resolved(&root, "client", model::resolve_allowed_client)),
        ])))
    }

    /// 🚫️ Reports the error class of every input the vocabulary does not cover.
    pub fn unresolvable_input_is_an_error_class(ctx: &Context) -> Result<Outcome, String> {
        let root = ctx.fixture_json(VECTORS)?;
        Ok(Outcome::projection(pairs(vec![
            ("llm".to_string(), rejected(&root, "llm", model::resolve_allowed_llm)),
            ("effort".to_string(), rejected(&root, "effort", model::resolve_allowed_effort)),
            ("client".to_string(), rejected(&root, "client", model::resolve_allowed_client)),
        ])))
    }

    /// 📋️ Reports the vocabulary table this implementation loaded.
    pub fn the_allowed_table_is_loaded_not_restated(_ctx: &Context) -> Result<Outcome, String> {
        let list = |values: &[String]| Json::Array(values.iter().map(|value| Json::String(value.clone())).collect());
        Ok(Outcome::projection(pairs(vec![
            ("llms".to_string(), list(model::allowed_llms())),
            ("efforts".to_string(), list(model::allowed_efforts())),
            ("clients".to_string(), list(model::allowed_clients())),
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
        .subject("normalisation-is-canonical-and-idempotent", subject::normalisation_is_canonical_and_idempotent)
        .subject("resolution-picks-the-longest-allowed-match", subject::resolution_picks_the_longest_allowed_match)
        .subject("unresolvable-input-is-an-error-class", subject::unresolvable_input_is_an_error_class)
        .subject("the-allowed-table-is-loaded-not-restated", subject::the_allowed_table_is_loaded_not_restated);
    #[cfg(not(feature = "sut"))]
    let _ = Outcome::projection(Json::Null);
    registered
}
//#endregion 🔖️Registration
