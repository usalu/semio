//! 🦀️ Rust side of the technology and bundle detection case. The subject halves are gated behind
//! the `sut` feature the generated host turns on for the subject role only, so the case still
//! compiles without linking the implementation under test.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_codebase::{normalize_bundle_label, Codebase};
    use semio_repo_test_host::{Context, Json, Outcome};
    use std::path::PathBuf;

    const TREE: &str = "shared://📡️repo-tree.json";

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

    pub fn technologies_and_bundles_agree(ctx: &Context) -> Result<Outcome, String> {
        let codebase = Codebase::new(materialize(ctx)?);
        let mut technologies = Vec::new();
        let mut bundles = Vec::new();
        for technology in codebase.technologies() {
            technologies.push(Json::String(format!("{}|{}|{}|{}", technology.name, technology.root, technology.kind, technology.emoji)));
            for bundle in technology.bundles.clone().unwrap_or_default() {
                bundles.push(Json::String(format!("{}|{}|{}|{}|{}|{}", bundle.name, bundle.root, bundle.kind, bundle.emoji, bundle.source_root, bundle.tags.join(","))));
            }
        }
        Ok(Outcome::projection(Json::Object(vec![("technologies".to_string(), Json::Array(technologies)), ("bundles".to_string(), Json::Array(bundles))])))
    }

    pub fn bundle_ids_and_labels_agree(ctx: &Context) -> Result<Outcome, String> {
        let codebase = Codebase::new(materialize(ctx)?);
        let bundles = codebase
            .bundles()
            .into_iter()
            .map(|bundle| Json::String(format!("{}|{}|{}", bundle.name, codebase.bundle_id(&bundle), normalize_bundle_label(&bundle.name))))
            .collect();
        Ok(Outcome::projection(Json::Object(vec![("bundles".to_string(), Json::Array(bundles))])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("technologies-and-bundles-agree", subject::technologies_and_bundles_agree)
        .subject("bundle-ids-and-labels-agree", subject::bundle_ids_and_labels_agree);
    adapter
}

//#endregion 🔖️Registration
