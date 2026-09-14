//! 🦀️ Rust subject for the monorepo tree build case. Gated behind the `sut` feature like every
//! subject, so the oracle role never links the crate under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_tree as tree;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use std::collections::BTreeMap;

    //#region 🔖️Stub
    /// 🔑️ The first non-empty identifying string of a node's data, as the feature states it.
    fn key(data: &BTreeMap<String, tree::Value>) -> String {
        for name in ["name", "title", "slug", "id", "path"] {
            if let Some(value) = data.get(name).and_then(|value| value.as_str()) {
                if !value.is_empty() {
                    return value.to_string();
                }
            }
        }
        String::new()
    }

    /// 🎨️ The stub the feature states, standing in for 🪪️identity and the CLI renderers.
    struct Stub;

    impl tree::EntityRenderer for Stub {
        fn human(&self, kind: &str, data: &BTreeMap<String, tree::Value>) -> String {
            format!("{kind}#{}", key(data))
        }
        fn markdown(&self, kind: &str, data: &BTreeMap<String, tree::Value>) -> String {
            format!("- {}", self.markdown_link(kind, data))
        }
        fn markdown_link(&self, kind: &str, data: &BTreeMap<String, tree::Value>) -> String {
            format!("[{kind}#{}]", key(data))
        }
    }

    impl tree::ArtifactIdentifier for Stub {
        fn artifact_id(&self, kind: &str, data: &BTreeMap<String, tree::Value>) -> String {
            format!("{}/{kind}:{}", data.get("parentId").and_then(|value| value.as_str()).unwrap_or(""), key(data))
        }
    }
    //#endregion 🔖️Stub

    //#region 🔖️Helpers
    /// 📥️ The committed record set every scenario projects.
    fn records(ctx: &Context) -> Result<tree::MemoryTreeSource, String> {
        tree::MemoryTreeSource::from_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://🌳️tree-source.json")?))
    }

    /// 📥️ The committed expectations every scenario is held against.
    fn expected(ctx: &Context) -> Result<Json, String> {
        parse_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://📤️tree-build-expectations.json")?))
    }

    /// 📃️ Reads a string list out of the expectations document.
    fn strings(document: &Json, field: &str) -> Vec<String> {
        document
            .array(field)
            .into_iter()
            .map(|row| match row {
                Json::String(value) => value,
                other => other.to_string(),
            })
            .collect()
    }

    /// ⚖️ Fails the scenario when a projection drifts from the committed expectation.
    fn require(name: &str, actual: &[String], want: &[String]) -> Result<(), String> {
        if actual == want {
            return Ok(());
        }
        Err(format!("{name}: expected {want:?}, got {actual:?}"))
    }

    /// 📃️ Renders a string list as the host's JSON.
    fn rows(values: &[String]) -> Json {
        Json::Array(values.iter().map(|value| Json::String(value.clone())).collect())
    }

    /// 📏️ Splits a rendered document into the lines the expectation states.
    fn lines(text: &str) -> Vec<String> {
        text.split('\n').map(str::to_string).collect()
    }

    /// 🧬️ Flattens a stamped tree into `<kind>|<id>|<parentId>` rows.
    fn stamps(node: &tree::TreeNode, out: &mut Vec<String>) {
        let parent = node.data.as_ref().and_then(|data| data.get("parentId")).and_then(|value| value.as_str()).unwrap_or("");
        out.push(format!("{}|{}|{parent}", node.kind.as_str(), node.id));
        for child in tree::children_of(node) {
            stamps(child, out);
        }
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    pub fn projects_the_record_set(ctx: &Context) -> Result<Outcome, String> {
        let built = tree::build_monorepo_tree(&records(ctx)?, tree::TreeBuildOptions::default());
        let outline = tree::tree_outline(&built);
        require("outline", &outline, &strings(&expected(ctx)?, "outline"))?;
        Ok(Outcome::projection(rows(&outline)))
    }

    pub fn includes_sections_when_requested(ctx: &Context) -> Result<Outcome, String> {
        let built = tree::build_monorepo_tree(&records(ctx)?, tree::TreeBuildOptions { include_sections: true });
        let outline = tree::tree_outline(&built);
        require("outlineWithSections", &outline, &strings(&expected(ctx)?, "outlineWithSections"))?;
        Ok(Outcome::projection(rows(&outline)))
    }

    pub fn renders_text_and_markdown(ctx: &Context) -> Result<Outcome, String> {
        let built = tree::build_monorepo_tree(&records(ctx)?, tree::TreeBuildOptions::default());
        let document = expected(ctx)?;
        let text = lines(&tree::render_monorepo_tree(&built, &Stub));
        let markdown = lines(&tree::render_monorepo_tree_markdown(&built, &Stub));
        require("text", &text, &strings(&document, "text"))?;
        require("markdown", &markdown, &strings(&document, "markdown"))?;
        Ok(Outcome::projection(Json::Object(vec![("text".to_string(), rows(&text)), ("markdown".to_string(), rows(&markdown))])))
    }

    pub fn stamps_parent_artifact_ids(ctx: &Context) -> Result<Outcome, String> {
        let mut built = tree::build_monorepo_tree(&records(ctx)?, tree::TreeBuildOptions::default());
        tree::propagate_parent_ids(&mut built, "", &Stub);
        let mut stamped: Vec<String> = Vec::new();
        stamps(&built, &mut stamped);
        require("parentIds", &stamped, &strings(&expected(ctx)?, "parentIds"))?;
        Ok(Outcome::projection(rows(&stamped)))
    }

    pub fn caches_by_content_digest(ctx: &Context) -> Result<Outcome, String> {
        let source = records(ctx)?;
        let plain = tree::build_monorepo_tree(&source, tree::TreeBuildOptions::default());
        let sectioned = tree::build_monorepo_tree(&source, tree::TreeBuildOptions { include_sections: true });
        let again = tree::build_monorepo_tree(&source, tree::TreeBuildOptions::default());
        let document = expected(ctx)?;
        let plain_digest = tree::tree_content_digest(&plain);
        let sectioned_digest = tree::tree_content_digest(&sectioned);
        let want_plain = document.str("contentDigest");
        let want_sectioned = document.str("contentDigestWithSections");
        require(
            "contentDigest",
            &[plain_digest.clone(), sectioned_digest.clone(), tree::tree_content_digest(&again)],
            &[want_plain.clone(), want_sectioned.clone(), want_plain],
        )?;
        let meta = tree::TreeCacheMeta::of(&plain, "fingerprint-a", false);
        let mut stale = meta.clone();
        stale.schema_version = tree::TREE_CACHE_SCHEMA_VERSION - 1;
        let decisions = vec![
            format!("same:{}", tree::tree_cache_is_valid(&meta, "fingerprint-a", false)),
            format!("otherFingerprint:{}", tree::tree_cache_is_valid(&meta, "fingerprint-b", false)),
            format!("otherSections:{}", tree::tree_cache_is_valid(&meta, "fingerprint-a", true)),
            format!("otherSchema:{}", tree::tree_cache_is_valid(&stale, "fingerprint-a", false)),
        ];
        require(
            "cacheDecisions",
            &decisions,
            &["same:true".to_string(), "otherFingerprint:false".to_string(), "otherSections:false".to_string(), "otherSchema:false".to_string()],
        )?;
        if plain_digest == sectioned_digest {
            return Err("the sectioned tree must not share the digest of the plain tree".to_string());
        }
        Ok(Outcome::projection(Json::Object(vec![
            ("digest".to_string(), Json::String(plain_digest)),
            ("digestWithSections".to_string(), Json::String(sectioned_digest)),
            ("decisions".to_string(), rows(&decisions)),
        ])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("projects-the-record-set", subject::projects_the_record_set)
        .subject("includes-sections-when-requested", subject::includes_sections_when_requested)
        .subject("renders-text-and-markdown", subject::renders_text_and_markdown)
        .subject("stamps-parent-artifact-ids", subject::stamps_parent_artifact_ids)
        .subject("caches-by-content-digest", subject::caches_by_content_digest);
    adapter
}
//#endregion 🔖️Registration
