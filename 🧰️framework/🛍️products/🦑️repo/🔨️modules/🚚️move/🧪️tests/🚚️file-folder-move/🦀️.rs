//! 🦀️ Rust side of the file and folder move case. The subject half is gated behind the `sut`
//! feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Helpers

/// 🗄️ Builds the in-memory workspace one `before` tree describes.
#[cfg(feature = "sut")]
fn workspace_of(tree: &Json) -> semio_framework_repo_move::Workspace {
    let mut workspace = semio_framework_repo_move::Workspace::new();
    for directory in tree.array("directories") {
        if let Json::String(path) = directory {
            workspace.insert_directory(&path);
        }
    }
    if let Some(Json::Object(files)) = tree.get("files") {
        for (path, content) in files {
            if let Json::String(text) = content {
                workspace.insert_file(path, text);
            }
        }
    }
    workspace
}

/// 🧾️ Renders a workspace the way every implementation of this case reports it.
#[cfg(feature = "sut")]
fn render(workspace: &semio_framework_repo_move::Workspace) -> Json {
    let mut directories = Vec::new();
    for (path, entry) in workspace.iter() {
        if matches!(entry, semio_framework_repo_move::Entry::Directory) {
            directories.push(Json::String(path.clone()));
        }
    }
    let files: Vec<(String, Json)> = workspace.files().into_iter().map(|(path, content)| (path, Json::String(content))).collect();
    Json::Object(vec![("directories".to_string(), Json::Array(directories)), ("files".to_string(), Json::Object(files))])
}

/// 🚚️ Plans one move of the recorded kind.
#[cfg(feature = "sut")]
fn plan(workspace: &semio_framework_repo_move::Workspace, kind: &str, source: &str, target: &str) -> Result<semio_framework_repo_move::Plan, String> {
    let planned = if kind == "folder" {
        semio_framework_repo_move::plan_folder_move(workspace, source, target)
    } else {
        semio_framework_repo_move::plan_file_move(workspace, source, target)
    };
    planned.map_err(|error| error.0)
}

/// 🎬️ Applies a plan to a copy of the workspace.
#[cfg(feature = "sut")]
fn applied(workspace: &semio_framework_repo_move::Workspace, plan: &semio_framework_repo_move::Plan) -> Result<semio_framework_repo_move::Workspace, String> {
    let mut copy = workspace.clone();
    semio_framework_repo_move::execute(plan, &mut copy).map_err(|error| error.0)?;
    Ok(copy)
}

/// ⚖️ Fails the scenario when the produced value is not the recorded one.
#[cfg(feature = "sut")]
fn expect(name: &str, field: &str, expected: &Json, produced: &Json) -> Result<(), String> {
    if expected == produced {
        return Ok(());
    }
    Err(format!("{name}: {field} differs from the recorded expectation\nrecorded: {}\nproduced: {}", expected.to_string(), produced.to_string()))
}

//#endregion 🔖️Helpers

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn a_move_carries_the_subtree_and_the_docs(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🚚️file-folder-move-trees.json")?;
    let mut moves = Vec::new();
    for vector in file.array("cases") {
        let name = vector.str("name");
        let before = workspace_of(vector.get("before").unwrap_or(&Json::Null));
        let planned = plan(&before, &vector.str("kind"), &vector.str("source"), &vector.str("target")).map_err(|error| format!("{name}: {error}"))?;
        let after = render(&applied(&before, &planned)?);
        let messages = Json::Array(planned.lines().into_iter().map(Json::String).collect());
        expect(&name, "workspace", vector.get("after").unwrap_or(&Json::Null), &after)?;
        expect(&name, "output lines", vector.get("messages").unwrap_or(&Json::Null), &messages)?;
        moves.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("after".to_string(), after), ("messages".to_string(), messages)]));
    }
    Ok(Outcome::projection(Json::Object(vec![("moves".to_string(), Json::Array(moves))])))
}

#[cfg(feature = "sut")]
fn moving_back_restores_the_tree(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🚚️file-folder-move-trees.json")?;
    let mut round_trips = Vec::new();
    for vector in file.array("cases") {
        let name = vector.str("name");
        let kind = vector.str("kind");
        let source = vector.str("source");
        let target = vector.str("target");
        let before = workspace_of(vector.get("before").unwrap_or(&Json::Null));
        let forward = plan(&before, &kind, &source, &target).map_err(|error| format!("{name}: {error}"))?;
        let moved = applied(&before, &forward)?;
        let backward = plan(&moved, &kind, &target, &source).map_err(|error| format!("{name}: {error}"))?;
        let restored = render(&applied(&moved, &backward)?);
        expect(&name, "restored workspace", &render(&before), &restored)?;
        round_trips.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("restored".to_string(), restored)]));
    }
    Ok(Outcome::projection(Json::Object(vec![("roundTrips".to_string(), Json::Array(round_trips))])))
}

#[cfg(feature = "sut")]
fn an_occupied_target_is_refused(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🚚️file-folder-move-trees.json")?;
    let first = file.array("cases").into_iter().next().ok_or_else(|| "the vector set carries no case".to_string())?;
    let workspace = workspace_of(first.get("before").unwrap_or(&Json::Null));
    let mut refusals = Vec::new();
    for vector in file.array("errors") {
        let name = vector.str("name");
        let message = match plan(&workspace, &vector.str("kind"), &vector.str("source"), &vector.str("target")) {
            Ok(_) => return Err(format!("{name}: the move was accepted but the vector records a refusal")),
            Err(error) => error,
        };
        expect(&name, "refusal message", &Json::String(vector.str("expectedError")), &Json::String(message.clone()))?;
        refusals.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("message".to_string(), Json::String(message))]));
    }
    Ok(Outcome::projection(Json::Object(vec![("refusals".to_string(), Json::Array(refusals))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("a-move-carries-the-subtree-and-the-docs", a_move_carries_the_subtree_and_the_docs)
        .subject("moving-back-restores-the-tree", moving_back_restores_the_tree)
        .subject("an-occupied-target-is-refused", an_occupied_target_is_refused);
    adapter
}

//#endregion 🔖️Registration
