//! 🦀️ Rust side of the host protocol conformance case. Written independently of the other four
//! adapters against the same frozen contract — pairwise equivalence is the whole point.

use semio_repo_test_host::{digest, Adapter, Context, Json, Outcome};

//#region 🔖️Scenarios
fn digest_and_fixture_resolution(ctx: &Context) -> Result<Outcome, String> {
    let vector = ctx.fixture_bytes("shared://📄️protocol-vector.txt")?;
    Ok(Outcome::projection(Json::Object(vec![
        ("vectorDigest".to_string(), Json::String(digest(&vector))),
        ("literalDigest".to_string(), Json::String(digest(b"semio"))),
        ("fixtureName".to_string(), Json::String("📄️protocol-vector.txt".to_string())),
        ("seed".to_string(), Json::Number(ctx.seed() as f64)),
        ("level".to_string(), Json::String(ctx.scenario.level.clone())),
        ("steps".to_string(), Json::Number(ctx.scenario.steps.len() as f64)),
    ])))
}

fn fixture_not_in_plan_is_an_error(ctx: &Context) -> Result<Outcome, String> {
    let reported = ctx.fixture("shared://this-fixture-is-not-declared").is_err();
    Ok(Outcome::projection(Json::Object(vec![("resolverReportedFailure".to_string(), Json::Bool(reported))])))
}

fn work_directory_is_cache_local(ctx: &Context) -> Result<Outcome, String> {
    let work_dir = ctx.work_dir.to_string_lossy().replace('\\', "/");
    Ok(Outcome::projection(Json::Object(vec![
        ("insideTestCache".to_string(), Json::Bool(work_dir.contains("/.🧬semio/🦑️repo/⚡️cache/tests/"))),
        ("hasOwnershipMarker".to_string(), Json::Bool(ctx.work_dir.join("🧾️marker.json").exists())),
    ])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("digest-and-fixture-resolution", digest_and_fixture_resolution)
        .subject("fixture-not-in-plan-is-an-error", fixture_not_in_plan_is_an_error)
        .subject("work-directory-is-cache-local", work_directory_is_cache_local)
}
//#endregion 🔖️Registration
