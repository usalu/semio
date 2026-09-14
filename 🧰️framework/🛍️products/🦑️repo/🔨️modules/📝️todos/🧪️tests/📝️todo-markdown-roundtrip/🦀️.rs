//! 🦀️ Rust side of the todo line grammar case. The subject half is gated behind the `sut` feature
//! so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn rewriting_a_line_and_reading_it_back_agrees(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_todos as todos;
    let file = ctx.fixture_json("shared://📝️line-vectors.json")?;
    let markdown = file.str("markdown");
    let markdown_path = file.str("markdownPath");
    let markdown_parent = markdown_path.rsplit_once('/').map(|(parent, _)| parent.to_string()).unwrap_or_default();
    let source = file.str("source");
    let source_path = file.str("sourcePath");

    let render = |todo: &todos::Todo| -> String {
        let location = match &todo.location {
            Some(location) => format!("{}:{}:{}", location.file_path, location.line, location.column),
            None => "-".to_string(),
        };
        format!("{}|{}|{}|{location}", todo.name, todo.description, todo.parent_id)
    };
    let names = |document: &str, in_markdown: bool| -> String {
        let parsed = if in_markdown { todos::parse_todo_markdown(document, &markdown_parent) } else { todos::parse_todo_comments(document, &source_path) };
        parsed.iter().map(|todo| format!("{}:{}", todo.name, todo.description)).collect::<Vec<_>>().join(",")
    };

    let markdown_parse: Vec<Json> = todos::parse_todo_markdown(&markdown, &markdown_parent).iter().map(|todo| Json::String(render(todo))).collect();
    let source_parse: Vec<Json> = todos::parse_todo_comments(&source, &source_path).iter().map(|todo| Json::String(render(todo))).collect();

    let mut markdown_rewrites: Vec<Json> = Vec::new();
    let mut markdown_reparsed: Vec<Json> = Vec::new();
    for rewrite in file.array("markdownRewrites") {
        match todos::replace_in_markdown(&markdown, &rewrite.str("oldName"), &rewrite.str("newName"), &rewrite.str("newDescription")) {
            Ok(document) => {
                markdown_reparsed.push(Json::String(names(&document, true)));
                markdown_rewrites.push(Json::String(format!("ok:{document}")));
            }
            Err(error) => {
                markdown_reparsed.push(Json::String("-".to_string()));
                markdown_rewrites.push(Json::String(format!("err:{}", error.class())));
            }
        }
    }

    let mut source_rewrites: Vec<Json> = Vec::new();
    let mut source_reparsed: Vec<Json> = Vec::new();
    for rewrite in file.array("sourceRewrites") {
        let line = match rewrite.get("line") {
            Some(Json::Number(value)) => *value as i64,
            _ => 0,
        };
        match todos::replace_in_file(&source, line, &rewrite.str("newName"), &rewrite.str("newDescription")) {
            Ok(document) => {
                source_reparsed.push(Json::String(names(&document, false)));
                source_rewrites.push(Json::String(format!("ok:{document}")));
            }
            Err(error) => {
                source_reparsed.push(Json::String("-".to_string()));
                source_rewrites.push(Json::String(format!("err:{}", error.class())));
            }
        }
    }

    let markdown_removals: Vec<Json> = file
        .array("markdownRewrites")
        .iter()
        .map(|rewrite| Json::String(todos::remove_from_markdown(&markdown, &rewrite.str("oldName"))))
        .collect();
    let source_removals: Vec<Json> = file
        .array("sourceRewrites")
        .iter()
        .map(|rewrite| {
            let line = match rewrite.get("line") {
                Some(Json::Number(value)) => *value as i64,
                _ => 0,
            };
            Json::String(todos::remove_from_file(&source, line))
        })
        .collect();

    let parts: Vec<Json> = source
        .split('\n')
        .map(|line| match todos::split_todo_comment_parts(line) {
            Some((prefix, name, description)) => Json::String(format!("{prefix}|{name}|{description}")),
            None => Json::String("-".to_string()),
        })
        .collect();

    let openers: Vec<Json> = file
        .array("openerPaths")
        .iter()
        .map(|entry| {
            let path = match entry {
                Json::String(value) => value.clone(),
                other => other.to_string(),
            };
            Json::String(format!("{path}={}", todos::todo_comment_opener(&path)))
        })
        .collect();

    Ok(Outcome::projection(Json::Object(vec![
        ("markdownParse".to_string(), Json::Array(markdown_parse)),
        ("sourceParse".to_string(), Json::Array(source_parse)),
        ("markdownRewrites".to_string(), Json::Array(markdown_rewrites)),
        ("markdownReparsed".to_string(), Json::Array(markdown_reparsed)),
        ("sourceRewrites".to_string(), Json::Array(source_rewrites)),
        ("sourceReparsed".to_string(), Json::Array(source_reparsed)),
        ("markdownRemovals".to_string(), Json::Array(markdown_removals)),
        ("sourceRemovals".to_string(), Json::Array(source_removals)),
        ("parts".to_string(), Json::Array(parts)),
        ("openers".to_string(), Json::Array(openers)),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("rewriting-a-line-and-reading-it-back-agrees", rewriting_a_line_and_reading_it_back_agrees);
    adapter
}

//#endregion 🔖️Registration
