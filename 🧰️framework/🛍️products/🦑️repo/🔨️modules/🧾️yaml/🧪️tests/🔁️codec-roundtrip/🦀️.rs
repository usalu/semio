//! 🦀️ Rust side of the YAML codec case. Written against the frozen vector set, never against the
//! other adapters. The subject half is gated behind the `sut` feature so the oracle role never even
//! compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

/// 🧫️ Reads `(name, source)` for every vector of the shared vector file.
fn load_vectors(ctx: &Context) -> Result<Vec<(String, String)>, String> {
    let file = ctx.fixture_json("shared://📡️codec-vectors.json")?;
    Ok(file.array("vectors").iter().map(|vector| (vector.str("name"), vector.str("source"))).collect())
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn vectors_decode_to_the_same_value(ctx: &Context) -> Result<Outcome, String> {
    let decoded: Vec<Json> = load_vectors(ctx)?
        .into_iter()
        .map(|(name, source)| match semio_framework_repo_yaml::decode_to_canonical_json(&source) {
            Ok(canonical) => Json::String(format!("{name}={canonical}")),
            Err(_) => Json::String(format!("{name}!error")),
        })
        .collect();
    Ok(Outcome::projection(Json::Object(vec![("decoded".to_string(), Json::Array(decoded))])))
}

#[cfg(feature = "sut")]
fn decode_encode_decode_is_idempotent(ctx: &Context) -> Result<Outcome, String> {
    let re_decoded: Vec<Json> = load_vectors(ctx)?
        .into_iter()
        .map(|(name, source)| match semio_framework_repo_yaml::round_trip_to_canonical_json(&source) {
            Ok(canonical) => Json::String(format!("{name}={canonical}")),
            Err(_) => Json::String(format!("{name}!decode")),
        })
        .collect();
    Ok(Outcome::projection(Json::Object(vec![("reDecoded".to_string(), Json::Array(re_decoded))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("vectors-decode-to-the-same-value", vectors_decode_to_the_same_value)
        .subject("decode-encode-decode-is-idempotent", decode_encode_decode_is_idempotent);
    adapter
}

//#endregion 🔖️Registration
