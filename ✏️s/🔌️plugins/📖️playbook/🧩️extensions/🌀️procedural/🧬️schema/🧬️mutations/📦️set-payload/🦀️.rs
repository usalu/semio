//! 📦️ Replace the procedural module's render payload with an invertible document mutation.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[dsl(keyword = "set-payload")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPayload {
    #[dsl(block)]
    pub payload: ModuleRenderPayload,
}

impl protocol::MutationKind<ModuleRenderPayload, ModulePayloadMutation> for SetPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "payload", kind: "set-payload", record: "SetPayload" };
    fn diff(&self, base: &ModuleRenderPayload) -> protocol::MutationOutcome<ModulePayloadDiff> {
        protocol::MutationOutcome::new(ModulePayloadDiff {
            example_id: (base.example_id != self.payload.example_id).then(|| self.payload.example_id.clone()),
            params: (base.params != self.payload.params).then(|| self.payload.params.clone()),
            question_id: (base.question_id != self.payload.question_id).then(|| self.payload.question_id.clone()),
            controller_id: (base.controller_id != self.payload.controller_id).then(|| self.payload.controller_id.clone()),
            surface: (base.surface != self.payload.surface).then(|| self.payload.surface.clone()),
            interactive: (base.interactive != self.payload.interactive).then_some(self.payload.interactive),
        })
    }
    fn inverse(&self, base: &ModuleRenderPayload) -> Result<Vec<ModulePayloadMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ModulePayloadMutation::SetPayload(SetPayload { payload: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Payload", "Nutzlast setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["payload".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = default_payload();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&ModulePayloadMutation::SetPayload(SetPayload { payload: ModuleRenderPayload { surface: "edit".into(), interactive: false, ..default_payload() } }), &base).await;
    }
}
