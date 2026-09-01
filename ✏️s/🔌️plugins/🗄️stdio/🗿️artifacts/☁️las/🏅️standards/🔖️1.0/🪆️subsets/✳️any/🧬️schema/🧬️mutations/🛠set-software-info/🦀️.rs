//! 🛠️ `set-software-info` — its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.
//!
//! 🛠️ Sets §2.3 Generating Software.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSoftwareInfo {
    pub generating_software: String,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetSoftwareInfo {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "software-info", kind: "set-software-info", record: "SetSoftwareInfo" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as protocol::Mutation<LasSnapshot>>::Diff> {
        agg_diff(&LasMutation::SetSoftwareInfo(self.clone()), base)
    }
    fn inverse(&self, base: &LasSnapshot) -> Vec<LasMutation> {
        agg_inverse(&LasMutation::SetSoftwareInfo(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-software-info".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
