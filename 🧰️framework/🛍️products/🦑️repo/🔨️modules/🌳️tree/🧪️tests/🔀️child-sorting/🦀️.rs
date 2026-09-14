//! 🦀️ Rust subject for the child sorting case.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_tree as tree;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The committed sorting fixture.
    fn fixture(ctx: &Context) -> Result<Json, String> {
        parse_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://🔀️sort-vectors.json")?))
    }

    /// 🌿️ The tree of one vector.
    fn tree_of(vector: &Json) -> Result<tree::TreeNode, String> {
        let raw = vector.get("tree").ok_or_else(|| "the vector states no tree".to_string())?;
        Ok(tree::TreeNodeSpec::from_json(&raw.to_string())?.to_tree_node())
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
    pub fn sorts_every_vector(ctx: &Context) -> Result<Outcome, String> {
        let mut projected: Vec<(String, Json)> = Vec::new();
        for vector in fixture(ctx)?.array("vectors") {
            let mut node = tree_of(&vector)?;
            tree::sort_tree_children(&mut node);
            let outline = tree::tree_outline(&node);
            require(&vector.str("id"), &outline, &strings(&vector, "outline"))?;
            projected.push((vector.str("id"), rows(&outline)));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    pub fn sorting_is_idempotent(ctx: &Context) -> Result<Outcome, String> {
        let mut checked: Vec<String> = Vec::new();
        for vector in fixture(ctx)?.array("vectors") {
            let mut node = tree_of(&vector)?;
            tree::sort_tree_children(&mut node);
            let once = tree::tree_outline(&node);
            tree::sort_tree_children(&mut node);
            require(&vector.str("id"), &tree::tree_outline(&node), &once)?;
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
    let adapter = adapter.subject("sorts-every-vector", subject::sorts_every_vector).subject("sorting-is-idempotent", subject::sorting_is_idempotent);
    adapter
}
//#endregion 🔖️Registration
