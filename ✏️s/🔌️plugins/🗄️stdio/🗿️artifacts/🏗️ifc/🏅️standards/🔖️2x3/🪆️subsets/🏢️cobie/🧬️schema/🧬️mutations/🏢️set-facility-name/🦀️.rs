//! ⚙️ `set-facility-name` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFacilityName {
        pub(crate) building: u64,
        pub(crate) name: Option<String>,
    }

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3CobieMutation> for SetFacilityName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "facility-name", kind: "set-facility-name", record: "SetFacilityName" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<<Ifc2x3CobieMutation as protocol::Mutation<Ifc2x3Snapshot>>::Diff> {
        agg_diff(&Ifc2x3CobieMutation::SetFacilityName(self.clone()), base)
    }
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Ifc2x3CobieMutation> {
        agg_inverse(&Ifc2x3CobieMutation::SetFacilityName(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-facility-name".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
