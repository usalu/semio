//! ➕️ `insert-entity` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertEntity {
    pub(crate) index: usize,
    pub(crate) entity: IfcEntity,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for InsertEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "entity", kind: "insert-entity", record: "InsertEntity" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<<IfcMutation as protocol::Mutation<IfcSnapshot>>::Diff> {
        agg_diff(&IfcMutation::InsertEntity(self.clone()), base)
    }
    fn inverse(&self, base: &IfcSnapshot) -> Vec<IfcMutation> {
        agg_inverse(&IfcMutation::InsertEntity(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-entity".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
