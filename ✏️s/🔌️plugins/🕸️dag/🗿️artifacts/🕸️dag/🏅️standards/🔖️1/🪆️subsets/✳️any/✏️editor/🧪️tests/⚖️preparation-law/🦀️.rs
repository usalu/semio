//! ⚖️ Runtime proof that the DAG parent's document lane stays an explicit fail-closed denial: its mutation vocabulary is uninhabited, every graph edit is a child-lane leaf, so no document preparation factory exists to bind.
use semio_framework_plugin::ArtifactEditor;
use semio_s_artifact_dag_dag::editor::dag::DagPlayApp;

/// 🕳️ The uninhabited parent vocabulary supplies no document preparation factory.
#[test]
fn document_lane_publishes_no_preparation_factory_for_the_uninhabited_vocabulary() {
    assert!(<DagPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_none());
}
