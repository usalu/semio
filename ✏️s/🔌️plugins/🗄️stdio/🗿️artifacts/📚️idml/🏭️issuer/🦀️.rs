//! 🏭️ The canonical IDML owner supplies one genuine publication/replay semantic factory.
use crate::{IdmlSnapshot,IdmlMutation};
use semio_framework_os_kernel as protocol;
use semio_framework_os_store_engine_owners as owners;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneBirthDemand,RetainedCloneGrant,RetainedCloneProgress}};
/// 🧬️ This concrete source invokes retained decision, admitted inverse and the central typed applier.
pub type IdmlSemanticPreparation=protocol::PagedOneItemPreparationFactory<IdmlSnapshot,IdmlMutation,protocol::MutationApplyEdit<IdmlSnapshot,IdmlMutation>>;
/// 📏 Quotes the original bounded catalog and shared semantic source before construction.
pub fn owners_demand(children:RetainedCloneBirthDemand,child_copy_bytes:usize)->Result<RetirementDemand,ValueError>{owners::bounded_artifact_store_owners_shared_semantic_source_demands::<IdmlSnapshot,IdmlMutation,IdmlSemanticPreparation>(children,child_copy_bytes)}
/// 🌱 Grants actual shared publication/replay source creation without a global cold fallback.
pub fn admit_owners(grant:RetainedCloneGrant,children:RetainedCloneBirthDemand,child_copy_bytes:usize)->Result<(owners::DocumentStoreOwners<IdmlSnapshot,IdmlMutation>,RetainedCloneProgress),owners::DocumentStoreOwnersAdmissionError<IdmlSnapshot,IdmlMutation>>{owners::bounded_artifact_store_owners_with_shared_semantic_preparation::<IdmlSnapshot,IdmlMutation,IdmlSemanticPreparation>(grant,children,child_copy_bytes,IdmlSemanticPreparation::default)}

/// 🪪️ The IDML codec's explicit native source policy, shared by declaration and direct factory.
pub struct IdmlCodecOwners;
impl protocol::ArtifactCodecOwnerSource<IdmlSnapshot,IdmlMutation> for IdmlCodecOwners {
 fn demand()->Result<RetirementDemand,ValueError>{owners_demand(RetainedCloneBirthDemand{capacity_bytes:0,depth:0},0)}
 fn admit(grant:RetainedCloneGrant)->Result<(owners::DocumentStoreOwners<IdmlSnapshot,IdmlMutation>,RetainedCloneProgress),owners::DocumentStoreOwnersAdmissionError<IdmlSnapshot,IdmlMutation>>{admit_owners(grant,RetainedCloneBirthDemand{capacity_bytes:0,depth:0},0)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
