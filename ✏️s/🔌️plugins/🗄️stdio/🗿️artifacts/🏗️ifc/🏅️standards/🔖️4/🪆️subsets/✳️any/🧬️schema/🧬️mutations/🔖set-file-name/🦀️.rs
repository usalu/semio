//! 🔖️ `set-file-name` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFileName {
    pub(crate) values: Vec<IfcValue>,
}

impl protocol::MutationKind<IfcSnapshot, IfcMutation> for SetFileName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "file-name", kind: "set-file-name", record: "SetFileName" };

    fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<<IfcMutation as protocol::Mutation<IfcSnapshot>>::Diff> {
        agg_diff(&IfcMutation::SetFileName(self.clone()), base)
    }
    fn inverse(&self, base: &IfcSnapshot) -> Vec<IfcMutation> {
        agg_inverse(&IfcMutation::SetFileName(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-file-name".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
