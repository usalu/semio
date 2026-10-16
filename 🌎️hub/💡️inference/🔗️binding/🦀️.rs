//! 🔗️ The neutral binding port between the hub's retained inference runtime and the artifact that owns a composed map document.
//!
//! The hub inference runtime never names an artifact crate. It reaches the artifact through two
//! registrations the composition root supplies: [`MapInferenceProposalBindingV1`], an object-safe
//! port for the byte-level proposal half (native execution, the server-stamped effect, the undo
//! effect, the preview and the composed members), and [`ComposedMapCommitBindingV1`], the typed
//! port the retained fixed-three committer is generic over (the parent, drawing and value member
//! Stores and their exact work). See `🪓️monolith-split/RESULT-RELOCATE-HUB-GIS-BINDING.md`.

use std::sync::Arc;

use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_spr::{HybridLogicalTimestamp, Mutation, OpBinary, OpText};
use semio_framework_os_kernel::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory;
use semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot;
use semio_framework_os_kernel::os_store::ArtifactStore;
use semio_framework_plugin::{ArtifactInferenceExecutionError, ArtifactInferenceService, WireArtifactInferenceBudget};
use semio_framework_value::{FromValue, ToValue};

/// 🚫️ Why a binding refused; the hub maps each refusal onto its own stable route or commit taxonomy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapBindingRefusalV1 {
    Undecodable,
    Execution,
    Arity,
    Inapplicable,
    Composition,
    Bounds,
}

/// 🧮️ One completed native inference: its canonical result bytes and its sole canonical proposal bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapInferenceRunV1 {
    pub result: Vec<u8>,
    pub proposal: Vec<u8>,
}

/// ✍️ The canonical forward and inverse payloads of the server-derived approval effect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapProposalEffectV1 {
    pub forward: Vec<u8>,
    pub inverse: Vec<u8>,
}

/// ↩️ The rebuilt original effect and the canonical undo payloads derived from its retained inverse.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapUndoEffectV1 {
    pub original_forward: Vec<u8>,
    pub original_inverse: Vec<u8>,
    pub forward: Vec<u8>,
    pub inverse: Vec<u8>,
}

/// 🗺️ The exact region one offered proposal would create, as the owner preview shows it.
#[derive(Clone, Debug, PartialEq)]
pub struct MapRegionPreviewV1 {
    pub region_id: String,
    pub ring: [[f64; 2]; 5],
}

/// 🔌️ Object-safe proposal half of one composed map artifact, registered by the composition root.
pub trait MapInferenceProposalBindingV1: Send + Sync {
    /// 🧷️ The literal native executable the frozen catalog binding must name.
    fn inference_service(&self) -> ArtifactInferenceService;
    /// 🧾️ The exact member ids a fixed-three commit requires, in stable-member order.
    fn commit_members(&self) -> [&'static str; 2];
    /// 🕸️ The composed child members one canonical parent pack owns, in stable-member order.
    fn composed_children(&self, pack: &[u8]) -> Result<Vec<String>, MapBindingRefusalV1>;
    /// ⚙️ Runs the native executable under `checkpoint` and derives its sole canonical proposal.
    fn infer(&self, pack: &[u8], job_id: &str, budget: &WireArtifactInferenceBudget, checkpoint: &mut dyn FnMut(u64) -> Result<(), ArtifactInferenceExecutionError>) -> Result<MapInferenceRunV1, MapBindingRefusalV1>;
    /// 🖋️ Rebuilds the proposal and its inverse deterministically for the server-stamped approval command.
    fn approval_effect(&self, pack: &[u8], job_id: &str, budget: &WireArtifactInferenceBudget) -> Result<MapProposalEffectV1, MapBindingRefusalV1>;
    /// 🔙 Rebuilds the original effect from its retained inverse against the current parent pack.
    fn undo_effect(&self, pack: &[u8], original_inverse_payload: &[u8], original_job_id: &str) -> Result<MapUndoEffectV1, MapBindingRefusalV1>;
    /// 🖼️ Decodes one offered proposal into the exact region preview it would create.
    fn preview(&self, job_id: &str, proposal: &[u8]) -> Result<MapRegionPreviewV1, MapBindingRefusalV1>;
}

/// 🏠️ Typed half of one composed map artifact: the parent, drawing and value member types the
/// retained fixed-three committer is generic over, with the exact work it publishes.
///
/// The associated type bounds are the union of what `ArtifactStore`, the durable three-Store
/// assembly, the commit host, the actor-owned recovery owner and the fixed-three WAL verification
/// require of their parameters.
pub trait ComposedMapCommitBindingV1: Sized + Send + Sync + 'static {
    type ParentSnapshot: ArtifactPack + ArtifactStoreSnapshot + Clone + PartialEq + ToValue + FromValue + Send + Sync + 'static;
    type ParentMutation: Mutation<Self::ParentSnapshot> + OpBinary + OpText + Clone + ToValue + FromValue + Send + Sync + 'static;
    type DrawingSnapshot: ArtifactPack + ArtifactStoreSnapshot + Clone + PartialEq + ToValue + FromValue + Send + Sync + 'static;
    type DrawingMutation: Mutation<Self::DrawingSnapshot> + OpBinary + OpText + Clone + ToValue + FromValue + Send + Sync + 'static;
    type ValueSnapshot: ArtifactPack + ArtifactStoreSnapshot + Clone + PartialEq + ToValue + FromValue + Send + Sync + 'static;
    type ValueMutation: Mutation<Self::ValueSnapshot> + OpBinary + OpText + Clone + ToValue + FromValue + Send + Sync + 'static;

    /// 🎯️ The exact member ids a fixed-three undo requires, in stable-member order.
    const COMMIT_MEMBERS: [&'static str; 2];

    /// 📦️ Decodes one canonical parent pack.
    fn decode_parent(pack: &[u8]) -> Result<Self::ParentSnapshot, MapBindingRefusalV1>;
    /// 🧬️ Derives the drawing and value members the parent determines.
    fn derived_members(parent: &Self::ParentSnapshot) -> (Self::DrawingSnapshot, Self::ValueSnapshot);
    /// 🏗️ Materializes the three member Stores of one parent under the exact actor.
    fn member_stores(parent_id: &str, parent: Self::ParentSnapshot, actor: &str) -> ComposedMapMemberStoresV1<Self>;
    /// 🏃️ Runs the native executable once and builds the approval work from its canonical result.
    fn executed_approval_work(pack: &[u8], job_id: &str, budget: &WireArtifactInferenceBudget) -> Result<(Self::ParentSnapshot, ComposedMapWorkV1<Self>), MapBindingRefusalV1>;
    /// 🧱️ Builds the approval work of one decoded parent directly.
    fn approval_work(parent: &Self::ParentSnapshot, job_id: &str) -> Result<ComposedMapWorkV1<Self>, MapBindingRefusalV1>;
    /// ⏪️ Rebuilds the original work from its retained inverse and derives the undo payloads.
    fn undo_work(parent: &Self::ParentSnapshot, inverse_payload: &[u8], original_job_id: &str) -> Result<ComposedMapUndoWorkV1<Self>, MapBindingRefusalV1>;
    /// 📍 The stamped parent preparation port of the retained fixed-three assembly.
    fn parent_preparation_factory(mutation_id: String, timestamp: HybridLogicalTimestamp) -> Arc<dyn ArtifactStoreOneItemPreparationFactory<Self::ParentSnapshot, Self::ParentMutation>>;
    /// 🎨 The stamped drawing preparation port of the retained fixed-three assembly.
    fn drawing_preparation_factory(mutation_id: String, timestamp: HybridLogicalTimestamp) -> Arc<dyn ArtifactStoreOneItemPreparationFactory<Self::DrawingSnapshot, Self::DrawingMutation>>;
    /// 🔢 The stamped value preparation port of the retained fixed-three assembly.
    fn value_preparation_factory(mutation_id: String, timestamp: HybridLogicalTimestamp) -> Arc<dyn ArtifactStoreOneItemPreparationFactory<Self::ValueSnapshot, Self::ValueMutation>>;
}

/// 🧳 The three member Stores of one mounted parent.
pub struct ComposedMapMemberStoresV1<B: ComposedMapCommitBindingV1> {
    pub parent: ArtifactStore<B::ParentSnapshot, B::ParentMutation>,
    pub drawing: ArtifactStore<B::DrawingSnapshot, B::DrawingMutation>,
    pub value: ArtifactStore<B::ValueSnapshot, B::ValueMutation>,
}

/// 🧩️ One typed parent+drawing+value work group with its canonical parent payloads.
pub struct ComposedMapWorkV1<B: ComposedMapCommitBindingV1> {
    pub parent: B::ParentMutation,
    pub parent_inverse: Vec<B::ParentMutation>,
    pub parent_json: Vec<u8>,
    pub parent_inverse_json: Vec<u8>,
    pub drawing_child_id: String,
    pub drawing: B::DrawingMutation,
    pub drawing_inverse: Vec<B::DrawingMutation>,
    pub value_child_id: String,
    pub value: B::ValueMutation,
    pub value_inverse: Vec<B::ValueMutation>,
}

/// 🔁️ The rebuilt original work plus the canonical undo forward and inverse payloads.
pub struct ComposedMapUndoWorkV1<B: ComposedMapCommitBindingV1> {
    pub forward_json: Vec<u8>,
    pub inverse_json: Vec<u8>,
    pub work: ComposedMapWorkV1<B>,
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
