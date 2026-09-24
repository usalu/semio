//! 🦀️ Rust side of the goal document codec case. The subject half is gated behind the `sut`
//! feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn stored_documents_round_trip(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_goals as goals;
    let file = ctx.fixture_json("shared://🎯️goal-documents.json")?;
    let mut documents = Vec::new();
    let mut members = Vec::new();
    for entry in file.array("documents") {
        let id = entry.str("id");
        let source = entry.str("json");
        let stored_parent = semio_repo_test_host::parse_json(&source)?.str("parent");
        let mut goal = goals::decode_goal(&id, &source).map_err(|error| format!("{id}: {error}"))?;
        let derived_parent = goal.parent.clone();
        // 🧭️The stored `parent` is already a compose identifier, so putting it back exercises the
        // encoder's leave-it-alone branch and makes the byte claim about the real file exact.
        goal.parent = stored_parent.clone();
        let encoded = goals::encode_goal(&goal).map_err(|error| format!("{id}: {error}"))?;
        let management = goal.management.clone().unwrap_or_default();
        documents.push(Json::String(format!("{id}={encoded}")));
        members.push(Json::String(format!(
            "{id}|{}|{}|{}|{}|{}|{}|{}|{}",
            goal.status.as_str(),
            goal.llm,
            goal.client,
            goal.dates.due,
            derived_parent,
            stored_parent,
            management.milestone,
            management.issue
        )));
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("documents".to_string(), Json::Array(documents)),
        ("members".to_string(), Json::Array(members)),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("stored-documents-round-trip", stored_documents_round_trip);
    adapter
}

//#endregion 🔖️Registration
