//! 🦀️ Rust side of the checkpoint listing case. The subject half is gated behind the `sut`
//! feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

#[cfg(feature = "sut")]
fn render(checkpoint: &semio_framework_repo_contributors::Checkpoint) -> String {
    format!(
        "{}|{}|{}|{}",
        checkpoint.sha,
        checkpoint.author_id.clone().unwrap_or_default(),
        checkpoint.date,
        checkpoint.title
    )
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_log_parses_into_checkpoints(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_contributors as contributors;
    let file = ctx.fixture_json("shared://🏁️checkpoint-log.json")?;
    let log = file.str("log");
    let source = contributors::MemoryCheckpointSource::new(&log);
    let checkpoints = contributors::list_checkpoints(&source, None);
    let parsed: Vec<Json> = checkpoints.iter().map(|checkpoint| Json::String(render(checkpoint))).collect();
    let identifiers: Vec<Json> = checkpoints.iter().map(|checkpoint| Json::String(format!("{}={}", checkpoint.sha, contributors::checkpoint_id(checkpoint)))).collect();
    let refusals: Vec<Json> = file
        .array("malformed")
        .iter()
        .map(|entry| {
            let line = match entry {
                Json::String(text) => text.clone(),
                other => other.to_string(),
            };
            Json::String(format!("{line}={}", contributors::parse_checkpoint_log(&line).len()))
        })
        .collect();
    let limited: Vec<Json> = contributors::list_checkpoints(&source, Some(3)).iter().map(|checkpoint| Json::String(render(checkpoint))).collect();
    let searched: Vec<Json> = contributors::search_checkpoints(&source, None, "🚩️59").iter().map(|checkpoint| Json::String(render(checkpoint))).collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("checkpoints".to_string(), Json::Array(parsed)),
        ("identifiers".to_string(), Json::Array(identifiers)),
        ("refusals".to_string(), Json::Array(refusals)),
        ("limited".to_string(), Json::Array(limited)),
        ("searched".to_string(), Json::Array(searched)),
    ])))
}

#[cfg(feature = "sut")]
fn a_limit_is_a_prefix_of_the_listing(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_contributors as contributors;
    let file = ctx.fixture_json("shared://🏁️checkpoint-log.json")?;
    let source = contributors::MemoryCheckpointSource::new(&file.str("log"));
    let all: Vec<String> = contributors::list_checkpoints(&source, None).iter().map(render).collect();
    let prefixes: Vec<Json> = (0..=all.len())
        .map(|limit| {
            let limited: Vec<String> = contributors::list_checkpoints(&source, Some(limit)).iter().map(render).collect();
            Json::String(format!("{limit}={}", limited == all[..limit.min(all.len())]))
        })
        .collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("prefixes".to_string(), Json::Array(prefixes)),
        ("total".to_string(), Json::Number(all.len() as f64)),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-log-parses-into-checkpoints", the_log_parses_into_checkpoints)
        .subject("a-limit-is-a-prefix-of-the-listing", a_limit_is_a_prefix_of_the_listing);
    adapter
}

//#endregion 🔖️Registration
