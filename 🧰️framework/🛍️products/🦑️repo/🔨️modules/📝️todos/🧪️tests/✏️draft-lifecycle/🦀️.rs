//! 🦀️ Rust side of the draft lifecycle case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_draft_script_produces_one_history(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_todos as todos;
    let file = ctx.fixture_json("shared://✏️draft-script.json")?;
    let store = todos::MemoryDraftStore::new();

    let mut outcomes: Vec<Json> = Vec::new();
    for step in file.array("steps") {
        let input = step.get("input").cloned().unwrap_or(Json::Object(Vec::new()));
        let line = match step.str("op").as_str() {
            "list" => format!("list ok {}", todos::list_drafts(&store).iter().map(|draft| draft.id.clone()).collect::<Vec<_>>().join(",")),
            "create" => {
                let files: Vec<(String, String)> = input.array("files").iter().map(|entry| (entry.str("path"), entry.str("content"))).collect();
                match todos::create_draft(&store, &input.str("title"), &files) {
                    Ok(draft) => format!("create ok {} {} {}", draft.id, todos::draft_id(&draft), todos::draft_uri(&draft)),
                    Err(error) => format!("create err {}", error.class()),
                }
            }
            "delete" => match todos::delete_draft(&store, &input.str("id")) {
                Ok(()) => format!("delete ok {}", todos::DraftStore::exists(&store, &input.str("id"))),
                Err(error) => format!("delete err {}", error.class()),
            },
            other => format!("unknown op {other}"),
        };
        outcomes.push(Json::String(line));
    }

    let files: Vec<Json> = todos::DraftStore::ids(&store)
        .iter()
        .flat_map(|id| todos::DraftStore::files(&store, id).into_iter().map(move |(name, content)| Json::String(format!("{id}/{name}={content}"))))
        .collect();
    Ok(Outcome::projection(Json::Object(vec![("outcomes".to_string(), Json::Array(outcomes)), ("files".to_string(), Json::Array(files))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("the-draft-script-produces-one-history", the_draft_script_produces_one_history);
    adapter
}

//#endregion 🔖️Registration
