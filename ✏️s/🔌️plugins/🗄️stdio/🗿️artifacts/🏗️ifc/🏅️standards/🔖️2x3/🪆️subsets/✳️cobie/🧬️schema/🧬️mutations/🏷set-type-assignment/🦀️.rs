//! 🏷️ `set-type-assignment` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTypeAssignment {
        pub(crate) id: u64,
        pub(crate) assignment: Option<CobieTypeAssignment>,
    }

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3CobieMutation> for SetTypeAssignment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "type-assignment", kind: "set-type-assignment", record: "SetTypeAssignment" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<<Ifc2x3CobieMutation as protocol::Mutation<Ifc2x3Snapshot>>::Diff> {
        agg_diff(&Ifc2x3CobieMutation::SetTypeAssignment(self.clone()), base)
    }
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Ifc2x3CobieMutation> {
        agg_inverse(&Ifc2x3CobieMutation::SetTypeAssignment(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-type-assignment".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
