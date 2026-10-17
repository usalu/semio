//! 🧩️ Jack example fixture: the example snapshot and its resolved in-memory graph.

use super::*;

// #endregion 🔖️LanguageService


/// 🧩️ [`example_graph_snapshot`] as a resolved in-memory [`Graph`].
pub fn example_graph() -> Graph {
    example_graph_snapshot().into_graph().expect("jack example fixture")
}

// #endregion 🔖️SpannedAst

/// 🧩️ Typed example graph for language service and playground consumers.
pub fn example_graph_snapshot()->crate::standards::v1::subsets::any::schema::ast::JackQueryGraphSnapshot {
    crate::standards::v1::subsets::any::schema::ast::JackQueryGraphSnapshot{schema:JackSnapshot::SCHEMA.into(),name:"jack-example".into(),manifest_id:Some("nakagin".into()),manifest:Manifest::nakagin_default(),camera:Camera::default(),nodes:vec![
            Node {
                id: "root".into(),
                kind: "Piece".into(),
                name: "core".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            },
            Node {
                id: "child".into(),
                kind: "Piece".into(),
                name: "leaf".into(),
                x: 160.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
            },
        ], edges: vec![Edge { id: "e1".into(), kind: "Connection".into(), source: "root@out".into(), target: "child@in".into(), properties: PropertyBag::new() }],root_node_id:Some("root".into()),query:String::new()}
}
