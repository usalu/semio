//! 📚️ Trinity Jack app — Catalogue panel (fixture presets, example queries, manifest kinds).

use crate::editor::jack::config::JackConfig;
use crate::editor::jack::terminology::TrinityJackLabels;
use semio_framework_plugin::{tree_item, tree_item_with_action, PanelTreeBuilder};

pub(crate) fn render(cfg: &JackConfig, labels: &TrinityJackLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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
    let builder = PanelTreeBuilder::new("trinity-jack-catalogue")?;
    let fixture_items = crate::editor::jack::ui_node_list(fixtures.iter().map(|(id, label)| {
        let args = crate::editor::jack::ui_value_map([("exampleId", crate::editor::jack::ui_value_text(id)?)])?;
        tree_item_with_action(builder.item_id("fixture", id)?, crate::editor::jack::ui_label(label)?, Some(preset_query(id).into()), jack_action("setActiveExample", Some(args))?)
    }))?;
    let example_items = crate::editor::jack::ui_node_list(examples.iter().map(|(id, label, query)| {
        let args = crate::editor::jack::ui_value_map([("query", crate::editor::jack::ui_value_text(query)?)])?;
        tree_item_with_action(builder.item_id("example", id)?, crate::editor::jack::ui_label(label)?, Some((*query).into()), jack_action("loadExampleQuery", Some(args))?)
    }))?;
    let kind_items = crate::editor::jack::ui_node_list([
        tree_item("trinity-jack-catalogue.piece", crate::editor::jack::ui_label(labels.piece.as_str())?),
        tree_item("trinity-jack-catalogue.connection", crate::editor::jack::ui_label(labels.connection.as_str())?),
        tree_item("trinity-jack-catalogue.connector", crate::editor::jack::ui_label(labels.connector.as_str())?),
    ])?;
    let builder = builder
        .section("trinity-jack-catalogue.fixtures", Some(labels.fixtures.as_str().into()), true, fixture_items)?
        .section("trinity-jack-catalogue.examples", Some(labels.example_queries.as_str().into()), true, example_items)?
        .section("trinity-jack-catalogue.kinds", Some(labels.manifest_kinds.as_str().into()), false, kind_items)?;
    let builder = if cfg.active_fixture_id.is_empty() {
        builder
    } else {
        let selected_id = builder.item_id("fixture", &cfg.active_fixture_id)?;
        builder.selected([selected_id])?
    };
    builder.build()
}
