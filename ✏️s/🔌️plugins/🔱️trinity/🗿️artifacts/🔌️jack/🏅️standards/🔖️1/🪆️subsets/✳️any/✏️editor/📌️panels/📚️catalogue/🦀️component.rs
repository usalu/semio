//! 📚️ Trinity Jack app — Catalogue panel (fixture presets, example queries, manifest kinds).

use crate::editor::jack::config::JackConfig;
use crate::editor::jack::terminology::TrinityJackLabels;
use semio_framework_plugin::{tree_item, tree_item_with_action, Label, PanelTreeBuilder, UiNode, UiTreeItemNode};
use serde_json::json;

pub(crate) async fn render(cfg: &JackConfig, labels: &TrinityJackLabels) -> UiNode {
    let jack_action = crate::editor::jack::jack_action;
    let preset_query = crate::editor::jack::commands::query::preset_query;
    let fixtures = [("nakagin", "Nakagin — Table"), ("branch-chain", "Branch — Graph")];
    let examples = [
        ("where-or", "Where Or", "MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name"),
        ("return-graph", "Return Graph", "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b"),
        ("set-label", "Set Label", "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'"),
        ("set-position", "Set Position", "MATCH (a:Piece) WHERE a.name = 'b' SET a.x = 300, a.y = 120"),
        ("create-node", "Create Node", "CREATE (n:Piece)"),
        ("create-edge", "Create Edge", "MATCH (a:Piece), (b:Piece) WHERE a.name = 'b' AND b.name != 'b' CREATE (a)-[:Connection]->(b)"),
        ("delete-leaf", "Delete Leaf", "MATCH (n:Piece) WHERE n.name = 'b' DELETE n"),
        ("merge-edge", "Merge Edge", "MERGE (x:Piece)-[:Connection]->(y:Piece)"),
    ];
    let builder = PanelTreeBuilder::new("trinity-jack-catalogue");
    let fixture_items: Vec<UiTreeItemNode> =
        fixtures.iter().map(|(id, label)| tree_item_with_action(builder.item_id("fixture", id), Label::data(*label), Some(preset_query(id).into()), jack_action("setActiveExample", Some(json!({ "exampleId": id }))))).collect();
    let example_items: Vec<UiTreeItemNode> =
        examples.iter().map(|(id, label, query)| tree_item_with_action(builder.item_id("example", id), Label::data(*label), Some((*query).into()), jack_action("loadExampleQuery", Some(json!({ "query": query }))))).collect();
    let selected = if cfg.active_fixture_id.is_empty() { vec![] } else { vec![builder.item_id("fixture", &cfg.active_fixture_id)] };
    builder
        .section("trinity-jack-catalogue.fixtures", Some(labels.fixtures.into()), true, fixture_items)
        .section("trinity-jack-catalogue.examples", Some(labels.example_queries.into()), true, example_items)
        .section(
            "trinity-jack-catalogue.kinds",
            Some(labels.manifest_kinds.into()),
            false,
            vec![tree_item("trinity-jack-catalogue.piece", labels.piece), tree_item("trinity-jack-catalogue.connection", labels.connection), tree_item("trinity-jack-catalogue.connector", labels.connector)],
        )
        .selected(selected)
        .build()
}
