//! 🦀️ Rust side of the todo lifecycle case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Replay

#[cfg(feature = "sut")]
fn optional(input: &Json, key: &str) -> Option<String> {
    match input.get(key) {
        Some(Json::String(value)) => Some(value.clone()),
        _ => None,
    }
}

//#endregion 🔖️Replay

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_script_produces_one_history(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_todos as todos;
    let file = ctx.fixture_json("shared://🔓️lifecycle-script.json")?;
    let seeds: Vec<(String, String)> = file.array("files").iter().map(|entry| (entry.str("path"), entry.str("content"))).collect();
    let tree = todos::MemoryTodoTree::seeded(seeds);
    let emitter = todos::MemoryEmitter::new();
    let opener = todos::RecordingTicketOpener::new();
    let aggregate = todos::Todos::new(&tree, &emitter, &file.str("author"));

    let mut outcomes: Vec<Json> = Vec::new();
    for step in file.array("steps") {
        let input = step.get("input").cloned().unwrap_or(Json::Object(Vec::new()));
        let line = match step.str("op").as_str() {
            "list" => {
                let names: Vec<String> = aggregate.list().iter().map(|todo| todo.id.clone()).collect();
                format!("list ok {}", names.join(","))
            }
            "find" => match aggregate.find(&input.str("id")) {
                Ok(todo) => format!("find ok {} {} {}", todo.id, todo.name, todo.description),
                Err(error) => format!("find err {}", error.class()),
            },
            "create" => {
                let create = todos::TodoCreateInput { parent_id: input.str("parentId"), name: input.str("name"), description: input.str("description") };
                match aggregate.create(&create) {
                    Ok(todo) => format!("create ok {} {}", todo.id, todo.parent_id),
                    Err(error) => format!("create err {}", error.class()),
                }
            }
            "change" => {
                let change = todos::TodoChangeInput { id: input.str("id"), name: optional(&input, "name"), description: optional(&input, "description") };
                match aggregate.change(&change) {
                    Ok(todo) => format!("change ok {} {} {}", todo.id, todo.name, todo.description),
                    Err(error) => format!("change err {}", error.class()),
                }
            }
            "delete" => match aggregate.delete(&input.str("id")) {
                Ok(removed) => format!("delete ok {removed}"),
                Err(error) => format!("delete err {}", error.class()),
            },
            "to-ticket" => match aggregate.to_ticket(&input.str("id"), &opener, &input.str("title"), &input.str("prompt")) {
                Ok(ticket) => format!("to-ticket ok {ticket}"),
                Err(error) => format!("to-ticket err {}", error.class()),
            },
            other => format!("unknown op {other}"),
        };
        outcomes.push(Json::String(line));
    }

    let files: Vec<Json> = tree.snapshot().iter().map(|(path, content)| Json::String(format!("{path}={content}"))).collect();
    let tickets: Vec<Json> = opener.opened().into_iter().map(Json::String).collect();
    let envelopes: Vec<Json> = emitter.envelopes().into_iter().map(Json::String).collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("outcomes".to_string(), Json::Array(outcomes)),
        ("files".to_string(), Json::Array(files)),
        ("tickets".to_string(), Json::Array(tickets)),
        ("envelopes".to_string(), Json::Array(envelopes)),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("the-script-produces-one-history", the_script_produces_one_history);
    adapter
}

//#endregion 🔖️Registration
