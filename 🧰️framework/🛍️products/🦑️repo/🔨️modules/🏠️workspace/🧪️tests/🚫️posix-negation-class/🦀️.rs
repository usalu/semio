//! 🦀️ Rust side of the glob matching case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

/// 🃏️ One verdict per vector, rendered the way every implementation of this case reports it.
#[cfg(feature = "sut")]
fn verdicts_for(vectors: &[Json]) -> Vec<Json> {
    vectors
        .iter()
        .map(|vector| {
            let name = vector.str("name");
            match semio_framework_repo_workspace::glob_match(&vector.str("pattern"), &vector.str("path")) {
                Ok(matched) => Json::String(format!("{name}={matched}")),
                Err(_) => Json::String(format!("{name}=error")),
            }
        })
        .collect()
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_posix_negation_spelling_is_accepted(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📡️glob-vectors.json")?;
    Ok(Outcome::projection(Json::Object(vec![("verdicts".to_string(), Json::Array(verdicts_for(&file.array("divergentVectors"))))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-posix-negation-spelling-is-accepted", the_posix_negation_spelling_is_accepted);
    adapter
}

//#endregion 🔖️Registration
