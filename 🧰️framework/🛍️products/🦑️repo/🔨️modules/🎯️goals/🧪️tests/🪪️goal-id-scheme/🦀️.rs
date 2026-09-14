//! 🦀️ Rust side of the goal identifier scheme case. The subject half is gated behind the `sut`
//! feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn identifiers_resolve_to_one_place(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_goals as goals;
    let file = ctx.fixture_json("shared://🪪️id-vectors.json")?;
    let paths: Vec<Json> = file
        .array("paths")
        .iter()
        .map(|entry| {
            let path = match entry {
                Json::String(value) => value.clone(),
                other => other.to_string(),
            };
            Json::String(format!(
                "{path}|{}|{}|{}|{}|{}|{}|{}",
                goals::goal_depth(&path),
                goals::is_root_goal(&path),
                goals::is_first_gen_goal(&path),
                goals::is_deeper_goal(&path),
                goals::goal_id_for_filesystem(&path),
                goals::root_goal_id(&path),
                goals::parent_goal_id(&path)
            ))
        })
        .collect();
    let compose: Vec<Json> = file
        .array("paths")
        .iter()
        .map(|entry| {
            let path = match entry {
                Json::String(value) => value.clone(),
                other => other.to_string(),
            };
            let composed = goals::goal_path_to_compose_id(&path);
            Json::String(format!("{path}={composed}|{}", goals::compose_id_to_goal_path(&composed)))
        })
        .collect();
    let titles: Vec<Json> = file
        .array("titles")
        .iter()
        .map(|vector| Json::String(format!("{}+{}={}", vector.str("parent"), vector.str("title"), goals::compose_goal_id(&vector.str("parent"), &vector.str("title")))))
        .collect();
    let references: Vec<Json> = file
        .array("references")
        .iter()
        .map(|entry| {
            let reference = match entry {
                Json::String(value) => value.clone(),
                other => other.to_string(),
            };
            let milestone = goals::parse_milestone_number(&reference).map(|number| number.to_string()).unwrap_or_else(|error| error.class().to_string());
            let issue = goals::parse_issue_number(&reference).map(|number| number.to_string()).unwrap_or_else(|error| error.class().to_string());
            Json::String(format!("{reference}={milestone}|{issue}"))
        })
        .collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("paths".to_string(), Json::Array(paths)),
        ("compose".to_string(), Json::Array(compose)),
        ("titles".to_string(), Json::Array(titles)),
        ("references".to_string(), Json::Array(references)),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("identifiers-resolve-to-one-place", identifiers_resolve_to_one_place);
    adapter
}

//#endregion 🔖️Registration
