//! 🦀️ Rust subject for the tree filtering case.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_tree as tree;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The committed filter fixture.
    fn fixture(ctx: &Context) -> Result<Json, String> {
        parse_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://🔎️filter-vectors.json")?))
    }

    /// 🌿️ The filter tree the whole case works on.
    fn tree_of(document: &Json) -> Result<tree::TreeNode, String> {
        let raw = document.get("tree").ok_or_else(|| "the fixture states no tree".to_string())?;
        let spec = tree::TreeNodeSpec::from_json(&raw.to_string())?;
        Ok(spec.to_tree_node())
    }

    /// 🧹️ The filter of one vector.
    fn filter_of(vector: &Json) -> Result<tree::TreeFilter, String> {
        let raw = vector.get("filter").ok_or_else(|| "the vector states no filter".to_string())?;
        Ok(tree::TreeFilterSpec::from_json(&raw.to_string())?.to_filter())
    }

    /// 📃️ Reads a string list out of a JSON object.
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

    /// 📃️ Renders a string list as the host's JSON.
    fn rows(values: &[String]) -> Json {
        Json::Array(values.iter().map(|value| Json::String(value.clone())).collect())
    }

    /// ⚖️ Fails the scenario when a projection drifts from the committed expectation.
    fn require(name: &str, actual: &[String], want: &[String]) -> Result<(), String> {
        if actual == want {
            return Ok(());
        }
        Err(format!("{name}: expected {want:?}, got {actual:?}"))
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    pub fn applies_every_filter_vector(ctx: &Context) -> Result<Outcome, String> {
        let document = fixture(ctx)?;
        let root = tree_of(&document)?;
        let mut projected: Vec<(String, Json)> = Vec::new();
        for vector in document.array("vectors") {
            let id = vector.str("id");
            let filtered = tree::filter_monorepo_tree(&root, Some(&filter_of(&vector)?));
            let outline = tree::tree_outline(&filtered);
            require(&id, &outline, &strings(&vector, "outline"))?;
            projected.push((id, rows(&outline)));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    pub fn absent_filter_returns_the_tree(ctx: &Context) -> Result<Outcome, String> {
        let document = fixture(ctx)?;
        let root = tree_of(&document)?;
        let whole = tree::tree_outline(&root);
        let without = tree::tree_outline(&tree::filter_monorepo_tree(&root, None));
        let empty = tree::tree_outline(&tree::filter_monorepo_tree(&root, Some(&tree::TreeFilterSpec::default().to_filter())));
        require("noFilter", &without, &whole)?;
        require("emptyFilter", &empty, &whole)?;
        Ok(Outcome::projection(Json::Object(vec![("noFilter".to_string(), rows(&without)), ("emptyFilter".to_string(), rows(&empty))])))
    }

    pub fn filtering_is_idempotent(ctx: &Context) -> Result<Outcome, String> {
        let document = fixture(ctx)?;
        let root = tree_of(&document)?;
        let mut checked: Vec<String> = Vec::new();
        for vector in document.array("vectors") {
            let filter = filter_of(&vector)?;
            let once = tree::filter_monorepo_tree(&root, Some(&filter));
            let twice = tree::filter_monorepo_tree(&once, Some(&filter));
            require(&vector.str("id"), &tree::tree_outline(&twice), &tree::tree_outline(&once))?;
            checked.push(vector.str("id"));
        }
        Ok(Outcome::projection(rows(&checked)))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("applies-every-filter-vector", subject::applies_every_filter_vector)
        .subject("absent-filter-returns-the-tree", subject::absent_filter_returns_the_tree)
        .subject("filtering-is-idempotent", subject::filtering_is_idempotent);
    adapter
}
//#endregion 🔖️Registration
