//! 🦀️ Rust side of the dashboard command-tree projection case. The subject half is gated behind
//! the `sut` feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Workspace

#[cfg(feature = "sut")]
fn materialise(ctx: &Context, name: &str, reversed: bool) -> Result<std::path::PathBuf, String> {
    let vector = ctx.fixture_json("local://🏗️workspace.json")?;
    let mut files: Vec<(String, String)> = vector.array("files").iter().map(|entry| (entry.str("path"), entry.str("content"))).collect();
    if reversed {
        files.reverse();
    }
    let root = ctx.work_dir.join(name);
    let _ = std::fs::remove_dir_all(&root);
    for (path, content) in files {
        let target = path.split('/').fold(root.clone(), |acc, segment| acc.join(segment));
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
        }
        std::fs::write(&target, content).map_err(|error| format!("cannot write {}: {error}", target.display()))?;
    }
    Ok(root)
}

//#endregion 🔖️Workspace

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_fixture_workspace_projects_its_command_tree(ctx: &Context) -> Result<Outcome, String> {
    let root = materialise(ctx, "workspace", false)?;
    let projected = parse_json(&semio_framework_repo_dashboard::command_tree::tree_json_text(&root))?;
    Ok(Outcome::projection(Json::Object(vec![("tree".to_string(), projected)])))
}

#[cfg(feature = "sut")]
fn the_projection_does_not_depend_on_directory_order(ctx: &Context) -> Result<Outcome, String> {
    let forward = semio_framework_repo_dashboard::command_tree::tree_json_text(&materialise(ctx, "forward", false)?);
    let backward = semio_framework_repo_dashboard::command_tree::tree_json_text(&materialise(ctx, "backward", true)?);
    Ok(Outcome::projection(Json::Object(vec![("stable".to_string(), Json::Bool(forward == backward)), ("tree".to_string(), parse_json(&forward)?)])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-fixture-workspace-projects-its-command-tree", the_fixture_workspace_projects_its_command_tree)
        .subject("the-projection-does-not-depend-on-directory-order", the_projection_does_not_depend_on_directory_order);
    adapter
}

//#endregion 🔖️Registration
