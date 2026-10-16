//! 📥️ Trait impls of `📬️publication` whose trait or type belongs to `os-store-one-item-publication` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs`.
use semio_framework_os_store_one_item::ArtifactStoreOneItemFootprint;
use semio_framework_os_store_one_item::ArtifactStoreOneItemGrant;
use crate::ArtifactStoreOneItemPreparation;
use crate::ArtifactStoreOneItemPreparationRequest;
use semio_framework_os_store_command::HistoryLane;
use semio_framework_os_spr::Mutation;
use semio_framework_value::retained_clone::RetainedCloneBirthDemand;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::ValueError;

/// 📨 Owned member wire, transferred without decoding or cloning at the composition boundary.
#[derive(Debug)]
pub struct MemberStoreOneItemWire {
    pub schema: String,
    pub bytes: Vec<u8>,
}

/// 🧾 Exact member revision and grouped-history metadata accompanying one owned wire item.
pub struct MemberStoreOneItemWireRequest {
    pub operation: semio_framework_job::OperationId,
    pub expected_generation: u64,
    pub expected_revision: [u8; 32],
    pub actor: semio_framework_value::SharedUtf8,
    pub group_id: Option<String>,
    pub wire: MemberStoreOneItemWire,
}

/// 🏭 A member's retained decoder and semantic preparation, never a generic `decode_op` fallback.
pub trait MemberStoreOneItemWirePreparationFactory<P, Mutation>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn preflight(&self, wire: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String>;
    fn begin_demand(&self, wire: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError>;
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire, Mutation>)>;
}
