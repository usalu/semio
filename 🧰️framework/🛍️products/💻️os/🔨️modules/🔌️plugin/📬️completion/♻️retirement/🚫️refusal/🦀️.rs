//! 🚫️ Completion refusal retains the complete original emission, ephemeral lanes and diagnostic.
use crate::app::{ArtifactApp,ArtifactToolCompletionRejection,EphemeralEmit};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,sequence_birth_bytes,deferred_birth_bytes};
impl<A:ArtifactApp> RetireOwned for EphemeralEmit<A> where A::PresenceMutation:RetireOwned,A::TransientMutation:RetireOwned{
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{presence,transient,window_transient}=self;(presence,transient,window_transient).retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes::<Vec<A::PresenceMutation>>(),deferred_birth_bytes::<Vec<A::TransientMutation>>(),deferred_birth_bytes::<Vec<crate::WindowTransientMutation>>()])}
 fn controlled_retirement_supported()->bool{true}
}
impl<A:ArtifactApp> RetireOwned for ArtifactToolCompletionRejection<A> where A::Mutation:RetireOwned,A::ConfigMutation:RetireOwned,A::DraftMutation:RetireOwned,A::PresenceMutation:RetireOwned,A::TransientMutation:RetireOwned{
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{emit,ephemeral,fault}=self;(emit,ephemeral,fault).retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes::<crate::app::ArtifactMutationOutcome<A::Mutation,A::ConfigMutation,A::DraftMutation>>(),deferred_birth_bytes::<EphemeralEmit<A>>(),deferred_birth_bytes::<crate::Fault>()])}
 fn controlled_retirement_supported()->bool{true}
}
