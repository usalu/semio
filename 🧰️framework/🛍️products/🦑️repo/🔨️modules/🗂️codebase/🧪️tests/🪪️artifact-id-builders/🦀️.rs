//! 🦀️ Rust side of the artifact id builders case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only, so the case still compiles
//! without linking the implementation under test.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_codebase::{build_definition_id, build_section_id, Codebase, ModelDefinitionKind};
    use semio_repo_test_host::{Context, Json, Outcome};
    use std::path::PathBuf;

    const TREE: &str = "shared://📡️repo-tree.json";
    const VECTORS: &str = "shared://📡️artifact-id-vectors.json";

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

    /// 🏷️ The definition kind a vector names.
    fn definition_kind(raw: &str) -> ModelDefinitionKind {
        match raw {
            "interface" => ModelDefinitionKind::Interface,
            "constant" => ModelDefinitionKind::Constant,
            "test" => ModelDefinitionKind::Test,
            _ => ModelDefinitionKind::Implementation,
        }
    }

    /// 🔖️ The section segments a vector carries.
    fn section_path(vector: &Json) -> Vec<String> {
        vector
            .array("sectionPath")
            .into_iter()
            .map(|segment| match segment {
                Json::String(value) => value,
                _ => String::new(),
            })
            .collect()
    }

    pub fn ids_agree_for_every_vector(ctx: &Context) -> Result<Outcome, String> {
        let codebase = Codebase::new(materialize(ctx)?);
        let mut ids = Vec::new();
        for vector in ctx.fixture_json(VECTORS)?.array("vectors") {
            let name = vector.str("name");
            let path = vector.str("path");
            let id = match vector.str("kind").as_str() {
                "folder" => codebase.build_folder_id(&path),
                "file" => codebase.build_file_id(&path),
                "section" => build_section_id(&codebase.build_file_id(&path), &section_path(&vector)),
                "definition" => build_definition_id(&codebase.build_file_id(&path), &section_path(&vector), &vector.str("definitionName"), definition_kind(&vector.str("definitionKind"))),
                other => return Err(format!("vector {name} names an unknown id kind {other}")),
            };
            ids.push(Json::String(format!("{name}={id}")));
        }
        Ok(Outcome::projection(Json::Object(vec![("ids".to_string(), Json::Array(ids))])))
    }

    pub fn uris_agree_for_every_file_vector(ctx: &Context) -> Result<Outcome, String> {
        let codebase = Codebase::new(materialize(ctx)?);
        let uris = ctx
            .fixture_json(VECTORS)?
            .array("vectors")
            .into_iter()
            .filter(|vector| vector.str("kind") == "file")
            .map(|vector| Json::String(format!("{}={}", vector.str("name"), codebase.file_uri(&vector.str("path")))))
            .collect();
        Ok(Outcome::projection(Json::Object(vec![("uris".to_string(), Json::Array(uris))])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("ids-agree-for-every-vector", subject::ids_agree_for_every_vector)
        .subject("uris-agree-for-every-file-vector", subject::uris_agree_for_every_file_vector);
    adapter
}

//#endregion 🔖️Registration
