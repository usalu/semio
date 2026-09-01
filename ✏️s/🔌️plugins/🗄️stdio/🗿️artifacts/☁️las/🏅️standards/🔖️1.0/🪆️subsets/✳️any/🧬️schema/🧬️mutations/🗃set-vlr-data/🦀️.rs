//! 🗃️ `set-vlr-data` — its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.
//!
//! 📦️ Replaces a VLR's payload bytes.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetVlrData {
    pub index: usize,
    pub data: Vec<u8>,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetVlrData {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "vlr-data", kind: "set-vlr-data", record: "SetVlrData" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as protocol::Mutation<LasSnapshot>>::Diff> {
        agg_diff(&LasMutation::SetVlrData(self.clone()), base)
    }
    fn inverse(&self, base: &LasSnapshot) -> Vec<LasMutation> {
        agg_inverse(&LasMutation::SetVlrData(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-vlr-data".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
