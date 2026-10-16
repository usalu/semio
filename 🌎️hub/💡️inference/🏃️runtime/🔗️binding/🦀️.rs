//! 🔗️ The runtime's view of the registered composed-map binding: Store, assembly, host and recovery projections over [`ComposedMapCommitBindingV1`], the retained document Stores, the commit identity and its operation. No artifact crate is named here; the GIS Map registration lives in `semio-hub-gis` (`🌎️hub/🧩️compositions/🌍️gis/💡️inference`).

use std::sync::Arc;
use directory::os_directory::{ArtifactFrontier, DocumentScope};
use semio_hub_inference_binding::ComposedMapCommitBindingV1;
use crate::approval_contract::{GisMapApprovalIngressAuthorityV1, GisMapDocumentWriteLeaseV1};
use semio_hub_inference::wal::InferenceDocumentFenceV1;
use semio_hub_inference::InferencePrivateBytesV1;

pub type GisMapParentSnapshotV1<B> = <B as ComposedMapCommitBindingV1>::ParentSnapshot;
pub type GisMapParentStoreV1<B> = directory::os_store::ArtifactStore<<B as ComposedMapCommitBindingV1>::ParentSnapshot, <B as ComposedMapCommitBindingV1>::ParentMutation>;
pub type GisMapDrawingStoreV1<B> = directory::os_store::ArtifactStore<<B as ComposedMapCommitBindingV1>::DrawingSnapshot, <B as ComposedMapCommitBindingV1>::DrawingMutation>;
pub type GisMapValueStoreV1<B> = directory::os_store::ArtifactStore<<B as ComposedMapCommitBindingV1>::ValueSnapshot, <B as ComposedMapCommitBindingV1>::ValueMutation>;
pub type GisMapAssemblyV1<B> = directory::os_store::durable_group::DurableOwnedThreeStoreMapAssemblyV1<
    <B as ComposedMapCommitBindingV1>::ParentSnapshot,
    <B as ComposedMapCommitBindingV1>::ParentMutation,
    <B as ComposedMapCommitBindingV1>::DrawingSnapshot,
    <B as ComposedMapCommitBindingV1>::DrawingMutation,
    <B as ComposedMapCommitBindingV1>::ValueSnapshot,
    <B as ComposedMapCommitBindingV1>::ValueMutation,
>;
pub type GisMapCommitHostV1<B> = directory::os_store::durable_group::DurableOwnedMapCommitHostV1<
    <B as ComposedMapCommitBindingV1>::ParentSnapshot,
    <B as ComposedMapCommitBindingV1>::ParentMutation,
    <B as ComposedMapCommitBindingV1>::DrawingSnapshot,
    <B as ComposedMapCommitBindingV1>::DrawingMutation,
    <B as ComposedMapCommitBindingV1>::ValueSnapshot,
    <B as ComposedMapCommitBindingV1>::ValueMutation,
>;
pub type GisMapRecoveryOwnerV1<B> = db::document::ArtifactDurableGroupRecoveryOwnerV1<
    <B as ComposedMapCommitBindingV1>::ParentSnapshot,
    <B as ComposedMapCommitBindingV1>::ParentMutation,
    <B as ComposedMapCommitBindingV1>::DrawingSnapshot,
    <B as ComposedMapCommitBindingV1>::DrawingMutation,
    <B as ComposedMapCommitBindingV1>::ValueSnapshot,
    <B as ComposedMapCommitBindingV1>::ValueMutation,
>;
pub type GisMapJournalReceiptV1 = directory::os_store::durable_group::DurableOwnedGroupJournalReceiptV1;

pub struct GisMapDocumentStoresV1<B: ComposedMapCommitBindingV1> {
    pub parent: Option<GisMapParentStoreV1<B>>,
    pub drawing: Option<GisMapDrawingStoreV1<B>>,
    pub value: Option<GisMapValueStoreV1<B>>,
    pub handle: db::ArtifactHandle,
    pub scope: DocumentScope,
    pub generation: u64,
    pub document_write: Arc<tokio::sync::Mutex<()>>,
    pub fence: Arc<InferenceDocumentFenceV1>,
}

#[derive(Clone)]
pub struct GisMapCommitIdentityV1 {
    pub scope: DocumentScope,
    pub actor: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub job_id: String,
    pub proposal_hash: String,
    pub base_frontier: ArtifactFrontier,
    pub descriptor_digest: String,
    pub base_digest: String,
    pub timestamp: protocol::HybridLogicalTimestamp,
    pub journal_now_ms: u64,
    pub document_write: Arc<tokio::sync::Mutex<()>>,
    pub ingress: Option<Arc<dyn GisMapApprovalIngressAuthorityV1>>,
    pub operation: GisMapCommitOperationV1,
}

#[derive(Clone)]
pub enum GisMapCommitOperationV1 {
    Approval,
    Undo { target_id: String, idempotency_key: String, original_job_id: String, original_command: Arc<InferencePrivateBytesV1> },
}

pub struct GisMapClosingStoresV1<B: ComposedMapCommitBindingV1> {
    pub owners: GisMapDocumentStoresV1<B>,
    pub phase: u8,
    pub document_write: Option<GisMapDocumentWriteLeaseV1>,
}
