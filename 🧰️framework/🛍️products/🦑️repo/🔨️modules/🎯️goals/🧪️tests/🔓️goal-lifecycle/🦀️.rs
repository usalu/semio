//! 🦀️ Rust side of the goal lifecycle case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Replay

#[cfg(feature = "sut")]
fn optional(entry: &Json, key: &str) -> Option<String> {
    entry.get(key).map(|value| match value {
        Json::String(text) => text.clone(),
        other => other.to_string(),
    })
}

#[cfg(feature = "sut")]
fn replay(ctx: &Context, force_no_management: bool) -> Result<(Vec<Json>, Vec<Json>, Vec<Json>, Vec<Json>), String> {
    use semio_framework_repo_goals as goals;
    let file = ctx.fixture_json("shared://🔓️lifecycle-script.json")?;
    let author = file.str("author");
    let repo_url = file.str("repoUrl");
    let first = match file.get("firstNumber") {
        Some(Json::Number(value)) => *value as i64,
        _ => 1,
    };
    let store = goals::MemoryGoalStore::new();
    let management = goals::RecordingManagement::new(&repo_url, first);
    let emitter = goals::MemoryEmitter::new();
    let aggregate = goals::Goals::new(&store, &management, &emitter, &author);

    let mut outcomes: Vec<Json> = Vec::new();
    for step in file.array("steps") {
        let operation = step.str("op");
        let Some(input) = step.get("input") else { continue };
        let no_management = force_no_management
            || matches!(input.get("noManagement"), Some(Json::Bool(true)));
        let line = match operation.as_str() {
            "create" => {
                let create = goals::GoalCreateInput {
                    title: input.str("title"),
                    description: input.str("description"),
                    prompt: input.str("prompt"),
                    due_date: input.str("dueDate"),
                    llm: input.str("llm"),
                    effort: input.str("effort"),
                    client: input.str("client"),
                    no_management,
                    parent: input.str("parent"),
                    milestone: input.str("milestone"),
                };
                match aggregate.create(&create) {
                    Ok(goal) => format!("create ok {} {}", goal.id, goal.status.as_str()),
                    Err(error) => format!("create err {}", error.class()),
                }
            }
            "change" => {
                let change = goals::GoalChangeInput {
                    id: input.str("id"),
                    title: optional(input, "title"),
                    description: optional(input, "description"),
                    due_date: optional(input, "dueDate"),
                    llm: optional(input, "llm"),
                    effort: optional(input, "effort"),
                    parent: optional(input, "parent"),
                    no_management,
                };
                match aggregate.change(&change) {
                    Ok(goal) => format!("change ok {} {}", goal.id, goal.title),
                    Err(error) => format!("change err {}", error.class()),
                }
            }
            "close" => {
                let close = goals::GoalCloseInput { id: input.str("id"), summary: input.str("summary"), no_management };
                match aggregate.close(&close) {
                    Ok(goal) => format!("close ok {} {}", goal.id, goal.status.as_str()),
                    Err(error) => format!("close err {}", error.class()),
                }
            }
            "reopen" => {
                let reopen = goals::GoalReopenInput {
                    id: input.str("id"),
                    prompt: input.str("prompt"),
                    client: input.str("client"),
                    llm: input.str("llm"),
                    effort: input.str("effort"),
                    title: optional(input, "title"),
                    description: optional(input, "description"),
                    due_date: optional(input, "dueDate"),
                    parent: optional(input, "parent"),
                    no_management,
                };
                match aggregate.reopen(&reopen) {
                    Ok(goal) => format!("reopen ok {} {} {}", goal.id, goal.status.as_str(), goal.effort),
                    Err(error) => format!("reopen err {}", error.class()),
                }
            }
            "delete" => {
                let delete = goals::GoalDeleteInput { id: input.str("id"), no_management };
                match aggregate.delete(&delete) {
                    Ok(removed) => format!("delete ok {removed}"),
                    Err(error) => format!("delete err {}", error.class()),
                }
            }
            "read" => match aggregate.read(&input.str("id")) {
                Ok(goal) => format!("read ok {} {} {}", goal.id, goal.status.as_str(), goal.title),
                Err(error) => format!("read err {}", error.class()),
            },
            other => format!("unknown op {other}"),
        };
        outcomes.push(Json::String(line));
    }
    let documents = store.snapshot().into_iter().map(|(id, document)| Json::String(format!("{id}={document}"))).collect();
    let calls = management.calls().into_iter().map(Json::String).collect();
    let envelopes = emitter.envelopes().into_iter().map(Json::String).collect();
    Ok((outcomes, documents, calls, envelopes))
}

//#endregion 🔖️Replay

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_script_produces_one_history(ctx: &Context) -> Result<Outcome, String> {
    let (outcomes, documents, calls, envelopes) = replay(ctx, false)?;
    Ok(Outcome::projection(Json::Object(vec![
        ("outcomes".to_string(), Json::Array(outcomes)),
        ("documents".to_string(), Json::Array(documents)),
        ("calls".to_string(), Json::Array(calls)),
        ("envelopes".to_string(), Json::Array(envelopes)),
    ])))
}

#[cfg(feature = "sut")]
fn management_can_be_switched_off(ctx: &Context) -> Result<Outcome, String> {
    let (outcomes, documents, calls, _) = replay(ctx, true)?;
    Ok(Outcome::projection(Json::Object(vec![
        ("outcomes".to_string(), Json::Array(outcomes)),
        ("documents".to_string(), Json::Array(documents)),
        ("calls".to_string(), Json::Array(calls)),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-script-produces-one-history", the_script_produces_one_history)
        .subject("management-can-be-switched-off", management_can_be_switched_off);
    adapter
}

//#endregion 🔖️Registration
