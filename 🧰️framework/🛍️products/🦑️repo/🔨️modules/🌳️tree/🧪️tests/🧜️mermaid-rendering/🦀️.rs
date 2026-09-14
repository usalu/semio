//! 🦀️ Rust subject for the Mermaid treemap rendering case.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_tree as tree;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The committed Mermaid fixture.
    fn fixture(ctx: &Context) -> Result<Json, String> {
        parse_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://🧜️mermaid-vectors.json")?))
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

    /// 📏️ Splits a rendered diagram into its lines.
    fn lines(text: &str) -> Vec<String> {
        text.split('\n').map(str::to_string).collect()
    }

    /// ⚖️ Fails the scenario when a projection drifts from the committed expectation.
    fn require(name: &str, actual: &[String], want: &[String]) -> Result<(), String> {
        if actual == want {
            return Ok(());
        }
        Err(format!("{name}: expected {want:?}, got {actual:?}"))
    }

    /// 🏷️ Collects every label of a treemap, depth first.
    fn labels(node: &tree::MermaidNode, out: &mut Vec<String>) {
        out.push(node.label.clone());
        for child in &node.children {
            labels(child, out);
        }
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    pub fn renders_every_treemap(ctx: &Context) -> Result<Outcome, String> {
        let mut projected: Vec<(String, Json)> = Vec::new();
        for vector in fixture(ctx)?.array("vectors") {
            let raw = vector.get("treemap").ok_or_else(|| "the vector states no treemap".to_string())?;
            let treemap = tree::MermaidTreemap::from_json(&raw.to_string())?;
            let rendered = lines(&tree::render_mermaid_treemap(&treemap));
            require(&vector.str("id"), &rendered, &strings(&vector, "lines"))?;
            projected.push((vector.str("id"), rows(&rendered)));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    pub fn projects_a_tree_into_a_treemap(ctx: &Context) -> Result<Outcome, String> {
        let document = fixture(ctx)?;
        let projection = document.get("projection").ok_or_else(|| "the fixture states no projection".to_string())?;
        let raw = projection.get("tree").ok_or_else(|| "the projection states no tree".to_string())?;
        let root = tree::TreeNodeSpec::from_json(&raw.to_string())?.to_tree_node();
        let treemap = tree::mermaid_treemap_from_tree(&root, &projection.str("title"), &projection.str("weightKey"));
        let rendered = lines(&tree::render_mermaid_treemap(&treemap));
        require("projection", &rendered, &strings(projection, "lines"))?;
        Ok(Outcome::projection(rows(&rendered)))
    }

    pub fn escapes_quotes_in_labels(ctx: &Context) -> Result<Outcome, String> {
        let document = fixture(ctx)?;
        let mut candidates: Vec<String> = vec!["\"\"\"".to_string()];
        for vector in document.array("vectors") {
            let raw = vector.get("treemap").ok_or_else(|| "the vector states no treemap".to_string())?;
            let treemap = tree::MermaidTreemap::from_json(&raw.to_string())?;
            candidates.push(treemap.title.clone());
            for node in &treemap.nodes {
                labels(node, &mut candidates);
            }
        }
        let mut escaped: Vec<String> = Vec::new();
        for candidate in &candidates {
            let once = tree::mermaid_escape_label(candidate);
            if once.contains('"') {
                return Err(format!("a double quote survived the escape of {candidate:?}"));
            }
            if tree::mermaid_escape_label(&once) != once {
                return Err(format!("escaping {candidate:?} is not idempotent"));
            }
            escaped.push(once);
        }
        Ok(Outcome::projection(rows(&escaped)))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("renders-every-treemap", subject::renders_every_treemap)
        .subject("projects-a-tree-into-a-treemap", subject::projects_a_tree_into_a_treemap)
        .subject("escapes-quotes-in-labels", subject::escapes_quotes_in_labels);
    adapter
}
//#endregion 🔖️Registration
