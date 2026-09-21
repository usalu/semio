use super::*;

trait DagChildOwnerOracle {
    fn expected() -> serde_json::Value;
}

struct SerdeJsonDagChildOwnerOracle;

impl DagChildOwnerOracle for SerdeJsonDagChildOwnerOracle {
    fn expected() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral DAG child-owner fixture")
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_kind_declares_the_graph_dag_component_kind() {
    assert_eq!(artifact_kind().id, "graph.dag");
    assert_eq!(artifact_kind().schema, DAG_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_matches_artifact_schema() {
    assert_eq!(default_snapshot().schema, DAG_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn node_edge_content_round_trips_through_the_composed_child_snapshot() {
    let document = default_snapshot();
    let scene = dag_working_scene(&document);
    let content = dag_content_snapshot_from_working(&scene.nodes, &scene.edges);
    let (nodes, edges) = working_from_dag_content_snapshot(&content);
    assert_eq!(nodes, scene.nodes);
    assert_eq!(edges, scene.edges);
}

#[semio_framework_async_macros::async_test]
async fn dag_working_scene_is_owned_by_the_exact_snapshot_child() {
    let owned = dag_content_child_with_owner(Vec::new(), Vec::new());
    let wire = dsl::json::to_json_string(&owned);
    let reconstructed: DagContentChild = dsl::json::from_json_str(&wire).expect("DAG child wire roundtrip");
    let observed = serde_json::json!({
        "ownedHasScene": owned.local_owner::<DagWorkingScene>().is_some(),
        "wireIdentityMatches": owned == reconstructed,
        "wireHasScene": reconstructed.local_owner::<DagWorkingScene>().is_some(),
    });

    assert_eq!(observed, SerdeJsonDagChildOwnerOracle::expected());
}

#[test]
fn temporary_regenerate_mutation_fixtures() {
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.file_name().is_some_and(|name| name == "🔣️.json") {
                out.push(path);
            }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets");
    let mut files = Vec::new();
    walk(&root, &mut files);
    for path in files {
        let text = std::fs::read_to_string(&path).expect("fixture reads");
        let parent = path.parent().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let canonical = if parent == "🔺️diff" {
            dsl::os_pack::from_json_str::<crate::DagDiff>(&text).ok().map(|value| dsl::os_pack::to_json_string(&value))
        } else if parent == "🦠️mutation" {
            dsl::os_pack::from_json_str::<crate::DagMutation>(&text).ok().map(|value| dsl::os_pack::to_json_string(&value))
        } else {
            None
        };
        let Some(canonical) = canonical else { continue };
        let reparsed: serde_json::Value = serde_json::from_str(&canonical).expect("canonical reparses");
        let original: serde_json::Value = serde_json::from_str(&text).expect("original reparses");
        if reparsed != original {
            std::fs::write(&path, format!("{}\n", serde_json::to_string_pretty(&reparsed).expect("pretty"))).expect("fixture writes");
            println!("[DEBUG] rewrote {}", path.display());
        }
    }
}
