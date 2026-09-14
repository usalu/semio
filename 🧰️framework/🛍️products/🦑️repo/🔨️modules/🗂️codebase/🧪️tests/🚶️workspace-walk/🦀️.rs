//! 🦀️ Rust side of the workspace walk case. The subject halves are gated behind the `sut` feature
//! the generated host turns on for the subject role only, so the case still compiles without
//! linking the implementation under test.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_codebase::{Codebase, CodebaseContext, Scope};
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

    /// 📄️ The considered files of the whole repository, in walk order.
    fn considered(ctx: &Context) -> Result<Vec<String>, String> {
        let codebase = Codebase::new(materialize(ctx)?);
        Ok(codebase.scope_to_files(&Scope::Repo))
    }

    pub fn walk_reports_the_same_considered_files(ctx: &Context) -> Result<Outcome, String> {
        let files = considered(ctx)?.into_iter().map(Json::String).collect();
        Ok(Outcome::projection(Json::Object(vec![("files".to_string(), Json::Array(files))])))
    }

    pub fn walk_projects_the_same_bundle_and_folder_aggregates(ctx: &Context) -> Result<Outcome, String> {
        let codebase = Codebase::new(materialize(ctx)?);
        let mut context = CodebaseContext::new(&codebase);
        context.load_bundles();
        context.load_files();
        let bundles = context
            .build_bundles()
            .into_iter()
            .map(|bundle| {
                let metrics = bundle.metrics.unwrap_or_default();
                Json::String(format!("{}|{}|{}|{}|{}", bundle.id, bundle.folder, metrics.folders, metrics.files, metrics.lines))
            })
            .collect();
        let folders = context
            .build_folders()
            .into_iter()
            .map(|folder| {
                let metrics = folder.metrics.unwrap_or_default();
                Json::String(format!("{}|{}|{}|{}", folder.path, folder.id, metrics.files, metrics.lines))
            })
            .collect();
        Ok(Outcome::projection(Json::Object(vec![("bundles".to_string(), Json::Array(bundles)), ("folders".to_string(), Json::Array(folders))])))
    }

    pub fn walk_projects_the_same_file_and_folder_records(ctx: &Context) -> Result<Outcome, String> {
        let codebase = Codebase::new(materialize(ctx)?);
        let mut context = CodebaseContext::new(&codebase);
        context.load_bundles();
        context.load_files();
        let folders = context
            .build_folders()
            .into_iter()
            .map(|folder| {
                Json::String(format!(
                    "{}|{}|{}|{}|{}",
                    folder.id,
                    folder.path,
                    folder.name,
                    codebase.derive_folder_kind(&folder.path).as_str(),
                    semio_framework_repo_codebase::is_generated_folder(&folder.path)
                ))
            })
            .collect();
        let files = context
            .build_files()
            .into_iter()
            .map(|file| {
                let name = semio_framework_repo_codebase::base_of(&file.path);
                Json::String(format!(
                    "{}|{}|{}|{}|{}",
                    file.id,
                    file.path,
                    name,
                    semio_framework_repo_codebase::ext_of(&name),
                    semio_framework_repo_codebase::derive_file_kind(&name).as_str()
                ))
            })
            .collect();
        Ok(Outcome::projection(Json::Object(vec![("folders".to_string(), Json::Array(folders)), ("files".to_string(), Json::Array(files))])))
    }

    pub fn walk_projects_the_same_definitions(ctx: &Context) -> Result<Outcome, String> {
        let root = materialize(ctx)?;
        let codebase = Codebase::new(&root);
        let mut context = CodebaseContext::new(&codebase);
        context.load_bundles();
        context.load_files();
        let mut definitions = Vec::new();
        for path in &context.files {
            let Ok(content) = std::fs::read_to_string(root.join(path)) else { continue };
            let file_id = codebase.build_file_id(path);
            for definition in semio_framework_repo_codebase::file_definitions(&content, path) {
                let id = semio_framework_repo_codebase::definition_id(&file_id, &definition);
                definitions.push(Json::String(format!(
                    "{id}|{}|{}|{}|{}|{}|{}|{}",
                    definition.name,
                    definition.kind.as_str(),
                    definition.file_path,
                    definition.section_path,
                    definition.emoji,
                    definition.start_line,
                    definition.end_line
                )));
            }
        }
        Ok(Outcome::projection(Json::Object(vec![("definitions".to_string(), Json::Array(definitions))])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("walk-reports-the-same-considered-files", subject::walk_reports_the_same_considered_files)
        .subject("walk-projects-the-same-bundle-and-folder-aggregates", subject::walk_projects_the_same_bundle_and_folder_aggregates)
        .subject("walk-projects-the-same-file-and-folder-records", subject::walk_projects_the_same_file_and_folder_records)
        .subject("walk-projects-the-same-definitions", subject::walk_projects_the_same_definitions);
    adapter
}

//#endregion 🔖️Registration
