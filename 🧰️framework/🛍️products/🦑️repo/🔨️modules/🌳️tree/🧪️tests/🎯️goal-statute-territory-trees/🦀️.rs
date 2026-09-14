//! 🦀️ Rust subject for the goal, statute and territory tree case.

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

    /// 🎨️ The stub the feature states, standing in for the CLI renderers.
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
    //#endregion 🔖️Stub

    //#region 🔖️Helpers
    /// 📥️ The committed catalog, statute list and territory forest.
    fn inputs(ctx: &Context) -> Result<Json, String> {
        parse_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://🎯️goal-statute-territory.json")?))
    }

    /// 📥️ The committed expectations.
    fn expected(ctx: &Context) -> Result<Json, String> {
        parse_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://📤️goal-statute-territory-expectations.json")?))
    }

    /// 📥️ The record set the goals and tickets come from.
    fn records(ctx: &Context) -> Result<tree::MemoryTreeSource, String> {
        tree::MemoryTreeSource::from_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://🌳️tree-source.json")?))
    }

    /// 📜️ The catalog the statute ports are satisfied with.
    fn catalog(ctx: &Context) -> Result<tree::MemoryStatuteCatalog, String> {
        let document = inputs(ctx)?;
        let raw = document.get("catalog").ok_or_else(|| "the fixture states no catalog".to_string())?;
        tree::MemoryStatuteCatalog::from_json(&raw.to_string())
    }

    /// 🎯️ The goal tree every goal scenario works on.
    fn goal_tree(ctx: &Context) -> Result<Vec<tree::GoalNode>, String> {
        let source = records(ctx)?;
        Ok(tree::build_goal_tree(&source.goals, &source.tickets))
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

    /// 📏️ Splits a rendered document into its lines.
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

    /// 🧾️ The outline of a node forest.
    fn forest_outline(forest: &[tree::TreeNode]) -> Vec<String> {
        forest.iter().flat_map(|node| tree::tree_outline(node)).collect()
    }

    /// 🎯️ The outline of a goal forest, goals before their tickets.
    fn goal_outline(goals: &[tree::GoalNode], depth: usize, out: &mut Vec<String>) {
        for goal in goals {
            out.push(format!("{depth}|goal|{}|{}", goal.id, goal.title));
            goal_outline(goal.children.as_deref().unwrap_or(&[]), depth + 1, out);
            ticket_outline(goal.tickets.as_deref().unwrap_or(&[]), depth + 1, out);
        }
    }

    /// 🎫️ The outline of a ticket forest.
    fn ticket_outline(tickets: &[tree::TicketNode], depth: usize, out: &mut Vec<String>) {
        for ticket in tickets {
            out.push(format!("{depth}|ticket|{}|{}", ticket.id, ticket.slug));
            ticket_outline(ticket.children.as_deref().unwrap_or(&[]), depth + 1, out);
        }
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    pub fn builds_the_goal_tree(ctx: &Context) -> Result<Outcome, String> {
        let forest = goal_tree(ctx)?;
        let mut outline: Vec<String> = Vec::new();
        goal_outline(&forest, 0, &mut outline);
        require("goalTree", &outline, &strings(&expected(ctx)?, "goalTree"))?;
        Ok(Outcome::projection(rows(&outline)))
    }

    pub fn renders_the_goal_tree(ctx: &Context) -> Result<Outcome, String> {
        let forest = goal_tree(ctx)?;
        let document = expected(ctx)?;
        let text = lines(&tree::render_goal_tree_nodes(&forest, tree::TreeRenderFormat::Text, &Stub));
        let markdown = lines(&tree::render_goal_tree_nodes(&forest, tree::TreeRenderFormat::Markdown, &Stub));
        require("goalTreeText", &text, &strings(&document, "goalTreeText"))?;
        require("goalTreeMarkdown", &markdown, &strings(&document, "goalTreeMarkdown"))?;
        Ok(Outcome::projection(Json::Object(vec![("text".to_string(), rows(&text)), ("markdown".to_string(), rows(&markdown))])))
    }

    pub fn builds_the_statute_tree(ctx: &Context) -> Result<Outcome, String> {
        let document = inputs(ctx)?;
        let statutes = tree::decode_statutes(&document.get("statutes").map(Json::to_string).unwrap_or_else(|| "[]".to_string()))?;
        let outline = forest_outline(&tree::build_statute_tree(&statutes, &catalog(ctx)?));
        require("statuteTree", &outline, &strings(&expected(ctx)?, "statuteTree"))?;
        Ok(Outcome::projection(rows(&outline)))
    }

    pub fn builds_the_territory_tree(ctx: &Context) -> Result<Outcome, String> {
        let document = inputs(ctx)?;
        let territories = tree::decode_territories(&document.get("territories").map(Json::to_string).unwrap_or_else(|| "[]".to_string()))?;
        let outline = forest_outline(&tree::build_territory_tree(&territories, &catalog(ctx)?));
        require("territoryTree", &outline, &strings(&expected(ctx)?, "territoryTree"))?;
        Ok(Outcome::projection(rows(&outline)))
    }

    pub fn groups_statutes_by_entity_kind(ctx: &Context) -> Result<Outcome, String> {
        let document = inputs(ctx)?;
        let territories = tree::decode_territories(&document.get("territories").map(Json::to_string).unwrap_or_else(|| "[]".to_string()))?;
        let outline = forest_outline(&tree::build_policy_entity_kind_tree(&territories, &catalog(ctx)?));
        require("entityKindTree", &outline, &strings(&expected(ctx)?, "entityKindTree"))?;
        Ok(Outcome::projection(rows(&outline)))
    }

    pub fn counts_open_subgoals_and_tickets(ctx: &Context) -> Result<Outcome, String> {
        let forest = goal_tree(ctx)?;
        let counted: Vec<String> = forest
            .iter()
            .map(|goal| format!("{}|{}|{}", goal.id, tree::count_open_subgoals(goal), tree::count_open_tickets(goal)))
            .collect();
        let want: Vec<String> = expected(ctx)?
            .array("openSubgoals")
            .into_iter()
            .map(|row| {
                let subgoals = row.get("subgoals").map(Json::to_string).unwrap_or_default();
                let tickets = row.get("tickets").map(Json::to_string).unwrap_or_default();
                format!("{}|{subgoals}|{tickets}", row.str("goal"))
            })
            .collect();
        require("openCounts", &counted, &want)?;
        Ok(Outcome::projection(rows(&counted)))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("builds-the-goal-tree", subject::builds_the_goal_tree)
        .subject("renders-the-goal-tree", subject::renders_the_goal_tree)
        .subject("builds-the-statute-tree", subject::builds_the_statute_tree)
        .subject("builds-the-territory-tree", subject::builds_the_territory_tree)
        .subject("groups-statutes-by-entity-kind", subject::groups_statutes_by_entity_kind)
        .subject("counts-open-subgoals-and-tickets", subject::counts_open_subgoals_and_tickets);
    adapter
}
//#endregion 🔖️Registration
