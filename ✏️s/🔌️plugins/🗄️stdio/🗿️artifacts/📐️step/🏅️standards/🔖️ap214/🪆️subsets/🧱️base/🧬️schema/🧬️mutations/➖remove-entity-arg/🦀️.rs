//! ➖️ `remove-entity-arg` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveEntityArg {
    pub id: u64,
    pub arg_index: usize,
}

impl protocol::MutationKind<StepSnapshot, StepMutation> for RemoveEntityArg {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entity-arg", kind: "remove-entity-arg", record: "RemoveEntityArg" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepMutation as Mutation<StepSnapshot>>::Diff> {
        agg_diff(&StepMutation::RemoveEntityArg(self.clone()), base)
    }
    fn inverse(&self, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&StepMutation::RemoveEntityArg(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove entity arg", "Entitätsargument entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
