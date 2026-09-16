//! 📚️ Trinity Jack app — Catalogue panel (fixture presets, example queries, manifest kinds).

use crate::editor::jack::terminology::TrinityJackLabels;
use semio_framework_plugin::{tree_item, tree_item_with_action, ui_node_list, PanelTreeBuilder, TreeWindows};

//#region 🔖️Render
/// 📚️ One bundled graph fixture the catalogue can load.
struct FixturePreset {
    id: &'static str,
    label: &'static str,
}

/// 📚️ One canned Jack query the catalogue can run.
struct ExampleQuery {
    id: &'static str,
    label: &'static str,
    query: &'static str,
}

const FIXTURES: [FixturePreset; 2] = [FixturePreset { id: "nakagin", label: "Nakagin — Table" }, FixturePreset { id: "branch-chain", label: "Branch — Graph" }];

const EXAMPLES: [ExampleQuery; 8] = [
    ExampleQuery { id: "where-or", label: "Where Or", query: "MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name" },
    ExampleQuery { id: "return-graph", label: "Return Graph", query: "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b" },
    ExampleQuery { id: "set-label", label: "Set Label", query: "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'" },
    ExampleQuery { id: "set-position", label: "Set Position", query: "MATCH (a:Piece) WHERE a.name = 'b' SET a.x = 300, a.y = 120" },
    ExampleQuery { id: "create-node", label: "Create Node", query: "CREATE (n:Piece)" },
    ExampleQuery { id: "create-edge", label: "Create Edge", query: "MATCH (a:Piece), (b:Piece) WHERE a.name = 'b' AND b.name != 'b' CREATE (a)-[:Connection]->(b)" },
    ExampleQuery { id: "delete-leaf", label: "Delete Leaf", query: "MATCH (n:Piece) WHERE n.name = 'b' DELETE n" },
    ExampleQuery { id: "merge-edge", label: "Merge Edge", query: "MERGE (x:Piece)-[:Connection]->(y:Piece)" },
];

fn fixture_row(preset: &FixturePreset) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let args = crate::editor::jack::ui_value_map([("exampleId", crate::editor::jack::ui_value_text(preset.id)?)])?;
    tree_item_with_action(
        format!("trinity-jack-catalogue.fixture.{}", preset.id),
        crate::editor::jack::ui_label(preset.label)?,
        Some(crate::editor::jack::commands::preset_query(preset.id).into()),
        crate::editor::jack::jack_action("setActiveExample", Some(args))?,
    )
}

fn example_row(example: &ExampleQuery) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let args = crate::editor::jack::ui_value_map([("query", crate::editor::jack::ui_value_text(example.query)?)])?;
    tree_item_with_action(format!("trinity-jack-catalogue.example.{}", example.id), crate::editor::jack::ui_label(example.label)?, Some(example.query.into()), crate::editor::jack::jack_action("runQuery", Some(args))?)
}

pub(crate) fn render(labels: &TrinityJackLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let kind_items = ui_node_list([
        tree_item("trinity-jack-catalogue.piece", crate::editor::jack::ui_label(labels.piece.as_str())?),
        tree_item("trinity-jack-catalogue.connection", crate::editor::jack::ui_label(labels.connection.as_str())?),
        tree_item("trinity-jack-catalogue.connector", crate::editor::jack::ui_label(labels.connector.as_str())?),
    ])?;
    PanelTreeBuilder::new("trinity-jack-catalogue")?
        .window_section(windows, "trinity-jack-catalogue.fixtures", Some(crate::editor::jack::ui_label(labels.fixtures.as_str())?), true, &FIXTURES, fixture_row)?
        .window_section(windows, "trinity-jack-catalogue.examples", Some(crate::editor::jack::ui_label(labels.example_queries.as_str())?), true, &EXAMPLES, example_row)?
        .section("trinity-jack-catalogue.kinds", Some(crate::editor::jack::ui_label(labels.manifest_kinds.as_str())?), false, kind_items)?
        .build()
}
//#endregion 🔖️Render
