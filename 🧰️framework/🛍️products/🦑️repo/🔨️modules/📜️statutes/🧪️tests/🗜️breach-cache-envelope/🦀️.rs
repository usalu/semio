//! 🦀️ Rust side of the breach cache envelope case. The subject half is gated behind the `sut`
//! feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

/// 🔤️ A string element of an array, or its rendering.
#[cfg(feature = "sut")]
fn text(value: &Json) -> String {
    match value {
        Json::String(inner) => inner.clone(),
        other => other.to_string(),
    }
}

/// 📦️ The concatenated members every payload compresses to, each length-prefixed so one stream
/// carries the whole vector set for the reference to inflate back.
#[cfg(feature = "sut")]
fn frame(members: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&(members.len() as u32).to_be_bytes());
    for member in members {
        out.extend_from_slice(&(member.len() as u32).to_be_bytes());
        out.extend_from_slice(member);
    }
    out
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_digest_is_sha_256(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_statutes as statutes;
    let file = ctx.fixture_json("shared://🗜️breach-cache-envelope/🔣️vectors.json")?;
    let digests: Vec<Json> = file.array("digests").iter().map(|vector| Json::String(statutes::sha256_hex(text(vector).as_bytes()))).collect();
    let wire = file.str("envelope");
    let envelope = statutes::parse_breach_cache(&wire)?;
    if statutes::encode_breach_cache_json(&envelope)? != wire {
        return Err("the envelope did not round trip through its canonical encoding".to_string());
    }
    if statutes::breach_cache_digest(&envelope)? != statutes::sha256_hex(wire.as_bytes()) {
        return Err("the envelope digest is not the digest of its canonical encoding".to_string());
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("digests".to_string(), Json::Array(digests)),
        ("envelopeDigest".to_string(), Json::String(statutes::sha256_hex(wire.as_bytes()))),
    ])))
}

#[cfg(feature = "sut")]
fn a_member_inflates_anywhere(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_statutes as statutes;
    let file = ctx.fixture_json("shared://🗜️breach-cache-envelope/🔣️vectors.json")?;
    let payloads: Vec<String> = file.array("payloads").iter().map(text).collect();
    let mut members = Vec::new();
    let mut recovered: Vec<Json> = Vec::new();
    for payload in &payloads {
        let member = statutes::gzip_encode(payload.as_bytes());
        let back = statutes::gzip_decode(&member)?;
        if back != payload.as_bytes() {
            return Err(format!("a member did not inflate back to its payload: {payload}"));
        }
        recovered.push(Json::String(statutes::sha256_hex(&back)));
        members.push(member);
    }
    Ok(Outcome::with_raw(
        frame(&members),
        Json::Object(vec![("recovered".to_string(), Json::Array(recovered)), ("count".to_string(), Json::String(payloads.len().to_string()))]),
    ))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("the-digest-is-sha-256", the_digest_is_sha_256).subject("a-member-inflates-anywhere", a_member_inflates_anywhere);
    adapter
}

//#endregion 🔖️Registration
