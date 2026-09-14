//! 🦀️ Rust side of the ignore integration case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only, so the case still compiles
//! without linking the implementation under test.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_codebase::{is_generated_folder, Codebase};
    use semio_repo_test_host::{Context, Json, Outcome};
    use std::path::PathBuf;

    const TREE: &str = "shared://📡️repo-tree.json";
    const VECTORS: &str = "shared://📡️ignore-vectors.json";

    /// 🌳️ Materialises the committed tree into a private directory and returns its root.
    fn materialize(ctx: &Context) -> Result<PathBuf, String> {
        let root = ctx.work_dir.join("🌳️tree");
        if root.exists() {
            std::fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
        }
        for entry in ctx.fixture_json(TREE)?.array("entries") {
            let path = root.join(entry.str("path"));
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            std::fs::write(&path, entry.str("content")).map_err(|error| error.to_string())?;
        }
        Ok(root)
    }

    pub fn every_vector_gets_the_same_three_verdicts(ctx: &Context) -> Result<Outcome, String> {
        let codebase = Codebase::new(materialize(ctx)?);
        let verdicts = ctx
            .fixture_json(VECTORS)?
            .array("vectors")
            .into_iter()
            .map(|vector| {
                let path = vector.str("path");
                Json::String(format!("{}={},{},{}", vector.str("name"), codebase.is_repo_excluded_path(&path), codebase.is_gitignored(&path), is_generated_folder(&path)))
            })
            .collect();
        Ok(Outcome::projection(Json::Object(vec![("verdicts".to_string(), Json::Array(verdicts))])))
    }

    pub fn filtering_a_walk_drops_the_same_paths(ctx: &Context) -> Result<Outcome, String> {
        let codebase = Codebase::new(materialize(ctx)?);
        let ignore_patterns = vec!["**/node_modules/**".to_string(), "**/.venv/**".to_string()];
        let raw = codebase.glob_by_extension("**/*", &["ts", "tsx", "py", "cs", "go", "rs"], &ignore_patterns, true);
        let kept = codebase.filter_considered_files(&raw);
        let dropped: Vec<Json> = raw.iter().filter(|path| !kept.contains(path)).cloned().map(Json::String).collect();
        Ok(Outcome::projection(Json::Object(vec![
            ("kept".to_string(), Json::Array(kept.into_iter().map(Json::String).collect())),
            ("dropped".to_string(), Json::Array(dropped)),
        ])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("every-vector-gets-the-same-three-verdicts", subject::every_vector_gets_the_same_three_verdicts)
        .subject("filtering-a-walk-drops-the-same-paths", subject::filtering_a_walk_drops_the_same_paths);
    adapter
}

//#endregion 🔖️Registration
