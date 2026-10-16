//! 🕸️ Independently composable DAG data and mutation contracts.
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
pub use semio_framework_os_kernel::{os_dsl, os_pack, os_spr, os_store};
#[path = "🧵️retained/🦀️.rs"]
mod retained;
pub use retained::DagMemberStoreProducer;
#[path = "🖼️assets/🎬️demo/🦀️.rs"]
mod demo;
pub use demo::dag_demo_host_snapshot;
use crate::os_store::document_materialize::create_document_envelope;
use crate::os_store::{store_schema::ArtifactEnvelope, ArtifactStore};
use semio_framework_os_infinite_directed_presentation::*;
pub use semio_framework_artifact_infinite_dag_vcs::{
    ActorId,
    DAG_DOCUMENT_SCHEMA,
    ChangedNodeAbbreviation,
    ChangedNodeIcon,
    ChangedNodeName,
    ChangedNodeOperatorKind,
    ChangeNodeAbbreviation,
    ChangeNodeIcon,
    ChangeNodeName,
    ChangeNodeOperatorKind,
    ConnectNodes,
    CreateNode,
    DagDelta,
    DagDiff,
    DagEdgePatch,
    DagMutation,
    DagNodePatch,
    DagSnapshot,
    DeleteNode,
    DisconnectNodes,
    MoveNode,
    MovedNode,
    RenameNode,
    RenamedNode,
    ReorderNodes,
    ReplaceNodeKind,
    ReplaceNodeProperties,
    ReplacedNodeKind,
    ReplacedNodeProperties,
    ResizeNode,
    ResizedNode,
    RewrittenEdgeEndpoint,
    dag_document_from_host_snapshot,
    dag_index_from_wire,
    dag_host_snapshot_from_document,
    dag_index_to_wire,
    empty_dag_document,
};

// #region 🔖️ArtifactStore
/// 🌱️ The demo document seed (nodes/edges from `🕸️demo.dag`), sharing the fixture's example.
pub fn default_dag_document() -> DagSnapshot {
    dag_document_from_host_snapshot(&dag_demo_host_snapshot())
}

pub type DagEnvelope = ArtifactEnvelope<DagSnapshot, DagMutation>;
pub type DagStore = ArtifactStore<DagSnapshot, DagMutation>;

/// 🏭️ Creates a DAG store with the exact snapshot, mutation, and cursor retirement owners installed.
pub async fn create_dag_store(id: &str, snapshot: DagSnapshot, actor: ActorId) -> Result<DagStore, crate::os_store::VcsError> {
    let mut store = DagStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(create_document_envelope(DAG_DOCUMENT_SCHEMA, id, snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), actor).await?;
    crate::os_store::install_funded_member_store_owners(&mut store,&crate::DagMemberStoreProducer)?;
    Ok(store)
}

#[cfg(test)]
fn opened_dag_test_actor() -> ActorId {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🪪️demo-ownership/🔣️.json")).unwrap();
    ActorId(fixture["openedActor"].as_str().unwrap().into())
}

#[cfg(test)]
fn close_dag_test_store(mut store: DagStore) {
    for _ in 0..65_536 {
        let demand=store.close_owned_demands(4096).expect("original store close demands");
        let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let step=store.close_owned_store_step(grant).expect("original store admitted close");assert!(step.progress().fits(grant));
        if store.close_owned_store_terminal_is_empty(){return;}
    }
    panic!("DAG store did not reach original terminal emptiness");
}
// #endregion 🔖️ArtifactStore

#[cfg(test)]
#[path = "🧪️tests/🕸️dag/🦀️.rs"]
mod package_tests;

#[cfg(test)]
#[path = "🌿️vcs/🧪️tests/🔬️dag-vcs/🦀️.rs"]
mod dag_vcs_tests;

#[cfg(test)]
#[path = "🌿️vcs/🧪️tests/🔬️dag-direct/🦀️.rs"]
mod dag_direct_tests;

#[cfg(test)]
#[path = "🌿️vcs/🧪️tests/🔬️leaf-contracts/🦀️.rs"]
mod dag_leaf_contract_tests;

/// 🎬️ Canonical generic DAG example owned by the framework artifact and shared with its editors.
pub const DAG_DEMO_TEXT: &str = include_str!("🖼️assets/🎬️demo/🗣️.dsl.semio");

#[cfg(test)]
#[path = "./🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub(crate) mod sqlite_snapshot_tests;

#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
#[path="🌿️vcs/🕸️browser/🦀️.rs"]
mod browser;
