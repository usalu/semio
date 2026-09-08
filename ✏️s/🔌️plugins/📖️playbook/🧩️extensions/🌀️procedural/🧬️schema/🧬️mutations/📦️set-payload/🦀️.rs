//! 📦️ Replace the procedural module's render payload with an invertible document mutation.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[dsl(keyword = "set-payload")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPayload {
    #[dsl(block)]
    pub payload: ModuleRenderPayload,
}

impl protocol::MutationKind<ModuleRenderPayload, ModulePayloadMutation> for SetPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "payload", kind: "set-payload", record: "SetPayload" };
    fn diff(&self, _base: &ModuleRenderPayload) -> protocol::MutationOutcome<ModulePayloadDiff> { protocol::MutationOutcome::new(ModulePayloadDiff { payload: Some(self.payload.clone()) }) }
    fn inverse(&self, base: &ModuleRenderPayload) -> Vec<ModulePayloadMutation> { vec![ModulePayloadMutation::SetPayload(SetPayload { payload: base.clone() })] }
    fn label(&self) -> String { "Set Payload".into() }
    fn target(&self) -> Vec<String> { vec!["payload".into()] }
}
