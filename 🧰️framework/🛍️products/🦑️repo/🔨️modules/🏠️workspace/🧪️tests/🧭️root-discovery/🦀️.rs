//! 🦀️ Rust side of the root discovery case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

#[cfg(feature = "sut")]
use std::path::{Path, PathBuf};

//#region 🔖️Trees

/// 🌳️ Creates every declared directory and file of one tree description under `base`.
#[cfg(feature = "sut")]
fn materialize(base: &Path, tree: &Json) -> Result<(), String> {
    for directory in tree.array("directories") {
        let Json::String(relative) = directory else { continue };
        std::fs::create_dir_all(base.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR))).map_err(|error| error.to_string())?;
    }
    if let Some(Json::Object(files)) = tree.get("files") {
        for (relative, content) in files {
            let full = base.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
            if let Some(parent) = full.parent() {
                std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            let text = match content {
                Json::String(value) => value.clone(),
                other => other.to_string(),
            };
            std::fs::write(&full, text).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

/// ➗️ Renders `target` relative to `base` with forward slashes, resolving both first.
#[cfg(feature = "sut")]
fn relative_slash(base: &Path, target: &Path) -> String {
    let resolved_base = canonical(base);
    let resolved_target = canonical(target);
    match resolved_target.strip_prefix(&resolved_base) {
        Ok(relative) => {
            let text = relative.to_string_lossy().replace('\\', "/");
            if text.is_empty() {
                ".".to_string()
            } else {
                text
            }
        }
        Err(_) => "outside".to_string(),
    }
}

/// 🧹️ Resolves a path, dropping the Windows verbatim prefix canonicalisation adds.
#[cfg(feature = "sut")]
fn canonical(path: &Path) -> PathBuf {
    match std::fs::canonicalize(path) {
        Ok(resolved) => {
            let text = resolved.to_string_lossy().to_string();
            PathBuf::from(text.strip_prefix(r"\\?\").unwrap_or(&text).to_string())
        }
        Err(_) => path.to_path_buf(),
    }
}

/// 📋️ Renders the settings the way every implementation of this case reports them.
#[cfg(feature = "sut")]
fn render_config(config: &semio_framework_repo_workspace::RepoConfig) -> String {
    format!(
        "session={} operations={} plan={} detail={}",
        config.logging.session, config.logging.operations, config.logging.plan, config.logging.detail
    )
}

//#endregion 🔖️Trees

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_first_marker_upwards_wins(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📡️root-discovery-trees.json")?;
    let mut roots = Vec::new();
    for tree in file.array("trees") {
        let name = tree.str("name");
        let base = ctx.work_dir.join(format!("🌳️{name}"));
        materialize(&base, &tree)?;
        let start = base.join(tree.str("startAt").replace('/', std::path::MAIN_SEPARATOR_STR));
        let found = semio_framework_repo_workspace::find_repo_root(&start);
        roots.push(Json::String(format!("{name}={}", relative_slash(&base, &found))));
    }
    Ok(Outcome::projection(Json::Object(vec![("roots".to_string(), Json::Array(roots))])))
}

#[cfg(feature = "sut")]
fn the_layout_vocabulary_is_fixed(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_workspace as ws;
    let root = ctx.work_dir.join("🏠️layout");
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let layout = vec![
        Json::String(format!("semio={}", relative_slash(&root, &ws::semio_dir_for_root(&root)))),
        Json::String(format!("repoMeta={}", relative_slash(&root, &ws::repo_meta_dir_for_root(&root)))),
        Json::String(format!("tickets={}", relative_slash(&root, &ws::tickets_dir_for_root(&root)))),
        Json::String(format!("goals={}", relative_slash(&root, &ws::goals_dir_for_root(&root)))),
        Json::String(format!("devs={}", relative_slash(&root, &ws::devs_dir_for_root(&root)))),
        Json::String(format!("filesIndex={}", relative_slash(&root, &ws::files_index_for_root(&root)))),
        Json::String(format!("config={}", relative_slash(&root, &ws::repo_meta_path_for_root(&root, ws::CONFIG_FILE_NAME)))),
    ];
    Ok(Outcome::projection(Json::Object(vec![("layout".to_string(), Json::Array(layout))])))
}

#[cfg(feature = "sut")]
fn settings_fall_back_to_the_defaults(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_workspace as ws;
    let file = ctx.fixture_json("shared://📡️root-discovery-trees.json")?;
    let mut configs = Vec::new();
    for document in file.array("configDocuments") {
        let name = document.str("name");
        let root = ctx.work_dir.join(format!("📋️{name}"));
        let meta_dir = ws::repo_meta_dir_for_root(&root);
        std::fs::create_dir_all(&meta_dir).map_err(|error| error.to_string())?;
        if let Some(Json::String(body)) = document.get("document") {
            std::fs::write(meta_dir.join(ws::CONFIG_FILE_NAME), body).map_err(|error| error.to_string())?;
        }
        configs.push(Json::String(format!("{name}={}", render_config(&ws::load_repo_config(&root)))));
    }
    Ok(Outcome::projection(Json::Object(vec![("configs".to_string(), Json::Array(configs))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-first-marker-upwards-wins", the_first_marker_upwards_wins)
        .subject("the-layout-vocabulary-is-fixed", the_layout_vocabulary_is_fixed)
        .subject("settings-fall-back-to-the-defaults", settings_fall_back_to_the_defaults);
    adapter
}

//#endregion 🔖️Registration
