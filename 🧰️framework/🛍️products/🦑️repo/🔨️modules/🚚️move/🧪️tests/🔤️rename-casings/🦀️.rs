//! 🦀️ Rust side of the rename casings case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Trees

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

/// 🧾️ Renders a workspace the way every implementation of these cases reports it.
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

/// 🎬️ Applies a plan to a copy of the workspace.
#[cfg(feature = "sut")]
fn applied(workspace: &semio_framework_repo_move::Workspace, plan: &semio_framework_repo_move::Plan) -> Result<semio_framework_repo_move::Workspace, String> {
    let mut copy = workspace.clone();
    semio_framework_repo_move::execute(plan, &mut copy).map_err(|error| error.0)?;
    Ok(copy)
}

/// ↩️ The scope a reverse rename names: the same path with its last segment rewritten, because the
/// forward rename renames the scope root itself whenever that root carries the token.
#[cfg(feature = "sut")]
fn reverse_scope(scope: &str, old: &str, new: &str) -> String {
    if scope.is_empty() {
        return String::new();
    }
    match scope.rfind('/') {
        Some(index) => format!("{}/{}", &scope[..index], semio_framework_repo_move::apply_rename_casings(&scope[index + 1..], old, new)),
        None => semio_framework_repo_move::apply_rename_casings(scope, old, new),
    }
}

/// ⚖️ Fails the scenario when the produced value is not the recorded one.
#[cfg(feature = "sut")]
fn expect(name: &str, field: &str, expected: &Json, produced: &Json) -> Result<(), String> {
    if expected == produced {
        return Ok(());
    }
    Err(format!("{name}: {field} differs from the recorded expectation\nrecorded: {}\nproduced: {}", expected.to_string(), produced.to_string()))
}

//#endregion 🔖️Trees

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn every_spelling_folds_the_same_way(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🔤️rename-vectors.json")?;
    let mut rewrites = Vec::new();
    for vector in file.array("tokenVectors") {
        let name = vector.str("name");
        let produced = semio_framework_repo_move::apply_rename_casings(&vector.str("content"), &vector.str("old"), &vector.str("new"));
        expect(&name, "rewritten content", &Json::String(vector.str("expected")), &Json::String(produced.clone()))?;
        rewrites.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("rewritten".to_string(), Json::String(produced))]));
    }
    Ok(Outcome::projection(Json::Object(vec![("rewrites".to_string(), Json::Array(rewrites))])))
}

#[cfg(feature = "sut")]
fn a_workspace_renames_deepest_first(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🔤️rename-vectors.json")?;
    let mut trees = Vec::new();
    for vector in file.array("trees") {
        let name = vector.str("name");
        let before = workspace_of(vector.get("before").unwrap_or(&Json::Null));
        let plan = semio_framework_repo_move::plan_rename(&before, &vector.str("old"), &vector.str("new"), &vector.str("scope")).map_err(|error| format!("{name}: {}", error.0))?;
        let after = render(&applied(&before, &plan)?);
        let stats = Json::Object(plan.stats.iter().map(|(key, value)| (key.clone(), Json::Number(*value as f64))).collect());
        let messages = Json::Array(plan.lines().into_iter().map(Json::String).collect());
        expect(&name, "workspace", vector.get("after").unwrap_or(&Json::Null), &after)?;
        expect(&name, "counters", vector.get("stats").unwrap_or(&Json::Null), &stats)?;
        expect(&name, "output lines", vector.get("messages").unwrap_or(&Json::Null), &messages)?;
        trees.push(Json::Object(vec![
            ("name".to_string(), Json::String(name)),
            ("after".to_string(), after),
            ("stats".to_string(), stats),
            ("messages".to_string(), messages),
        ]));
    }
    Ok(Outcome::projection(Json::Object(vec![("trees".to_string(), Json::Array(trees))])))
}

#[cfg(feature = "sut")]
fn renaming_back_restores_the_workspace(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🔤️rename-vectors.json")?;
    let mut round_trips = Vec::new();
    for vector in file.array("trees") {
        let name = vector.str("name");
        let before = workspace_of(vector.get("before").unwrap_or(&Json::Null));
        let old = vector.str("old");
        let new = vector.str("new");
        let scope = vector.str("scope");
        let forward = semio_framework_repo_move::plan_rename(&before, &old, &new, &scope).map_err(|error| format!("{name}: {}", error.0))?;
        let renamed = applied(&before, &forward)?;
        let back = semio_framework_repo_move::plan_rename(&renamed, &new, &old, &reverse_scope(&scope, &old, &new)).map_err(|error| format!("{name}: {}", error.0))?;
        let restored = render(&applied(&renamed, &back)?);
        expect(&name, "restored workspace", &render(&before), &restored)?;
        round_trips.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("restored".to_string(), restored)]));
    }
    Ok(Outcome::projection(Json::Object(vec![("roundTrips".to_string(), Json::Array(round_trips))])))
}

#[cfg(feature = "sut")]
fn a_refused_rename_names_its_reason(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🔤️rename-vectors.json")?;
    let mut refusals = Vec::new();
    for vector in file.array("errors") {
        let name = vector.str("name");
        let before = workspace_of(vector.get("before").unwrap_or(&Json::Null));
        let message = match semio_framework_repo_move::plan_rename(&before, &vector.str("old"), &vector.str("new"), &vector.str("scope")) {
            Ok(_) => return Err(format!("{name}: the rename was accepted but the vector records a refusal")),
            Err(error) => error.0,
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
        .subject("every-spelling-folds-the-same-way", every_spelling_folds_the_same_way)
        .subject("a-workspace-renames-deepest-first", a_workspace_renames_deepest_first)
        .subject("renaming-back-restores-the-workspace", renaming_back_restores_the_workspace)
        .subject("a-refused-rename-names-its-reason", a_refused_rename_names_its_reason);
    adapter
}

//#endregion 🔖️Registration
