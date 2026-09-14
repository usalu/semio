//! 🦀️ Rust side of the definition-kind derivation case. The subject halves are gated behind the
//! `sut` feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_model as model;
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "local://🔣️vectors.json";

    const DECLARED: [&str; 4] = ["implementation", "interface", "constant", "test"];

    fn keywords(ctx: &Context) -> Result<Vec<String>, String> {
        Ok(ctx
            .fixture_json(VECTORS)?
            .array("keywords")
            .into_iter()
            .map(|item| match item {
                Json::String(value) => value,
                _ => String::new(),
            })
            .collect())
    }

    /// 🧩️ Derives the kind of every keyword.
    pub fn every_keyword_derives_one_kind(ctx: &Context) -> Result<Outcome, String> {
        let derived = keywords(ctx)?
            .into_iter()
            .map(|keyword| {
                let kind = model::derive_definition_kind(&keyword);
                Json::Object(vec![
                    ("keyword".to_string(), Json::String(keyword)),
                    ("kind".to_string(), Json::String(kind.to_string())),
                ])
            })
            .collect();
        Ok(Outcome::projection(Json::Array(derived)))
    }

    /// ✅️ Checks that the derivation is total and stays inside the declared vocabulary.
    pub fn derivation_is_total_and_valid(ctx: &Context) -> Result<Outcome, String> {
        let all = keywords(ctx)?;
        let count = all.len();
        let declared = all
            .iter()
            .all(|keyword| DECLARED.contains(&model::derive_definition_kind(keyword).as_str()));
        Ok(Outcome::projection(Json::Object(vec![
            ("keywords".to_string(), Json::Number(count as f64)),
            ("allInsideDeclaredKinds".to_string(), Json::Bool(declared)),
            ("declaredKinds".to_string(), Json::Array(DECLARED.iter().map(|kind| Json::String((*kind).to_string())).collect())),
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
        .subject("every-keyword-derives-one-kind", subject::every_keyword_derives_one_kind)
        .subject("derivation-is-total-and-valid", subject::derivation_is_total_and_valid);
    registered
}
//#endregion 🔖️Registration
