//! 🚫️ Completion refusal retains the complete original emission, ephemeral lanes and diagnostic (lifted from the WP1 completion tree; WP2 wires it into decl-app).
use crate::ArtifactApp;
use crate::tool_contract::ArtifactToolCompletionRejection;
use crate::app_emission::EphemeralEmit;
use semio_framework_value::retirement::RetireOwned;
use semio_plugin_decl_emission::ArtifactMutationOutcome;
type Fields<A>=(ArtifactMutationOutcome<<A as ArtifactApp>::Mutation,<A as ArtifactApp>::ConfigMutation,<A as ArtifactApp>::DraftMutation>,EphemeralEmit<A>,semio_framework::Fault);
impl<A:ArtifactApp> RetireOwned for ArtifactToolCompletionRejection<A> where A::Mutation:RetireOwned,A::ConfigMutation:RetireOwned,A::DraftMutation:RetireOwned,A::PresenceMutation:RetireOwned,A::TransientMutation:RetireOwned{
 type RetirementCursor=<Fields<A> as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{let Self{emit,ephemeral,fault}=self;(emit,ephemeral,fault).retirement_seed()}
 fn controlled_retirement_supported()->bool{true}
}
