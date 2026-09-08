
use super::*;

#[semio_framework_async_macros::async_test]
async fn interaction_definition_round_trips_through_json() {
    let def = InteractionDefinition {
        id: "graph".into(),
        label: LocalizedLabel::native("Graph", "Graph"),
        granularities: vec![
            GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
            GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
        ],
        hierarchy: HierarchyProvider::Topology,
        hover: HoverSpec::default(),
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
            transitive: false,
            broadcast: true,
        },
    };
    let json = serde_json::to_string(&def).expect("serializes");
    assert!(json.contains("\"iconId\""), "{json}");
    assert!(json.contains("\"granularities\""), "{json}");
    let parsed: InteractionDefinition = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(parsed, def);
}

#[semio_framework_async_macros::async_test]
async fn outline_projects_id_granularity_ids_and_selection_only() {
    let def = InteractionDefinition {
        id: "graph".into(),
        label: LocalizedLabel::native("Graph", "Graph"),
        granularities: vec![
            GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
            GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
        ],
        hierarchy: HierarchyProvider::Flat,
        hover: HoverSpec::default(),
        selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
    };
    let outline = def.outline().await;
    assert_eq!(outline.id, "graph");
    assert_eq!(outline.granularity_ids, vec!["node".to_string(), "edge".to_string()]);
    assert_eq!(outline.selection, def.selection);
}
