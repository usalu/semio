//! 🪪️ Semantic canonical authority and typed identities retain their original domain owners.
#[path="🪪️identity/🦀️.rs"]
mod identity;
pub use identity::ArtifactCanonicalEditIdentityCursor;
#[path="🛂️authority/🦀️.rs"]
mod authority;
pub use authority::{ArtifactCanonicalEditAuthority,ArtifactCanonicalEditAuthorityCursor};
use super::{Edit,MutationMeta,ForeignTarget};
use crate::{PayloadHash,UndoPolicy};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,sequence,deferred,sequence_birth_bytes,deferred_birth_bytes_for};

semio_framework_value::artifact_retire_struct!(MutationMeta{mutation_id,dependencies,base_version,author_id,timestamp,undo_policy,payload_hash,semantic_kind,label,group_id,origin,transaction});
semio_framework_value::artifact_retire_struct!(ForeignTarget{artifact_id,artifact_kind,dialect});
impl RetireOwned for PayloadHash {
 fn retirement(self)->Box<dyn RetirementCursor>{self.0.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.0.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{<[u8;32]>::controlled_retirement_supported()}
}
semio_framework_value::artifact_retire_leaf!(UndoPolicy);
impl<M:RetireOwned> RetireOwned for Edit<M> {
 fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.id),deferred(self.actor),deferred(self.line),deferred(self.forwards),deferred(self.inverse),deferred(self.mutation_meta),deferred(self.verb),deferred(self.sequence_number),deferred(self.started_at),deferred(self.finished_at)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.id),deferred_birth_bytes_for(&self.actor),deferred_birth_bytes_for(&self.line),deferred_birth_bytes_for(&self.forwards),deferred_birth_bytes_for(&self.inverse),deferred_birth_bytes_for(&self.mutation_meta),deferred_birth_bytes_for(&self.verb),deferred_birth_bytes_for(&self.sequence_number),deferred_birth_bytes_for(&self.started_at),deferred_birth_bytes_for(&self.finished_at)])}
 fn controlled_retirement_supported()->bool{M::controlled_retirement_supported()}
}
