//! 🦀️ Rust side of the ignore precedence case. The subject half is gated behind the `sut` feature
//! so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

#[cfg(feature = "sut")]
fn verdicts_for(vectors: &[Json]) -> Vec<Json> {
    vectors
        .iter()
        .map(|vector| {
            let name = vector.str("name");
            let rules: Vec<String> = vector
                .array("rules")
                .iter()
                .map(|rule| match rule {
                    Json::String(text) => text.clone(),
                    other => other.to_string(),
                })
                .collect();
            let matcher = semio_framework_repo_workspace::GitIgnore::compile_lines(rules.iter().map(String::as_str));
            Json::String(format!("{name}={}", matcher.matches_path(&vector.str("path"))))
        })
        .collect()
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn vectors_are_ignored_the_same_way(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📡️ignore-vectors.json")?;
    Ok(Outcome::projection(Json::Object(vec![("verdicts".to_string(), Json::Array(verdicts_for(&file.array("vectors"))))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("vectors-are-ignored-the-same-way", vectors_are_ignored_the_same_way);
    adapter
}

//#endregion 🔖️Registration
