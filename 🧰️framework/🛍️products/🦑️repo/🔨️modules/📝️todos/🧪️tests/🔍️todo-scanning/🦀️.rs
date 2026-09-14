//! 🦀️ Rust side of the todo scanning case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

#[cfg(feature = "sut")]
fn tree_files(file: &Json) -> Vec<(String, String)> {
    file.array("files").iter().map(|entry| (entry.str("path"), entry.str("content"))).collect()
}

#[cfg(feature = "sut")]
fn render(todo: &semio_framework_repo_todos::Todo) -> String {
    let location = match &todo.location {
        Some(location) => format!("{}:{}:{}", location.file_path, location.line, location.column),
        None => "-".to_string(),
    };
    format!("{}|{}|{}|{}|{location}", todo.id, todo.name, todo.description, todo.parent_id)
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn every_todo_in_the_tree_is_found(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_todos as todos;
    let file = ctx.fixture_json("shared://🔍️scan-tree.json")?;
    let tree = todos::MemoryTodoTree::seeded(tree_files(&file));
    let found: Vec<Json> = todos::scan_todos(&tree).iter().map(|todo| Json::String(render(todo))).collect();
    let searches: Vec<Json> = file
        .array("searchTerms")
        .iter()
        .map(|entry| {
            let term = match entry {
                Json::String(value) => value.clone(),
                other => other.to_string(),
            };
            let names: Vec<String> = todos::search_todos(&tree, &term).iter().map(|todo| todo.name.clone()).collect();
            Json::String(format!("{term}={}", names.join(",")))
        })
        .collect();
    let walk: Vec<Json> = todos::TodoTree::walk(&tree).iter().map(|entry| Json::String(format!("{}|{}", entry.path, entry.is_dir))).collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("todos".to_string(), Json::Array(found)),
        ("searches".to_string(), Json::Array(searches)),
        ("walk".to_string(), Json::Array(walk)),
    ])))
}

#[cfg(feature = "sut")]
fn the_scan_refuses_three_kinds_of_directory(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_todos as todos;
    let file = ctx.fixture_json("shared://🔍️scan-tree.json")?;
    let root = ctx.work_dir.join("scan-tree");
    for (path, content) in tree_files(&file) {
        let target = path.split('/').fold(root.clone(), |current, segment| current.join(segment));
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        std::fs::write(&target, content).map_err(|error| error.to_string())?;
    }
    let tree = todos::FsTodoTree::new(&root);
    let walk: Vec<Json> = todos::TodoTree::walk(&tree).iter().map(|entry| Json::String(format!("{}|{}", entry.path, entry.is_dir))).collect();
    let found: Vec<Json> = todos::scan_todos(&tree).iter().map(|todo| Json::String(render(todo))).collect();
    Ok(Outcome::projection(Json::Object(vec![("walk".to_string(), Json::Array(walk)), ("todos".to_string(), Json::Array(found))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("every-todo-in-the-tree-is-found", every_todo_in_the_tree_is_found)
        .subject("the-scan-refuses-three-kinds-of-directory", the_scan_refuses_three_kinds_of_directory);
    adapter
}

//#endregion 🔖️Registration
