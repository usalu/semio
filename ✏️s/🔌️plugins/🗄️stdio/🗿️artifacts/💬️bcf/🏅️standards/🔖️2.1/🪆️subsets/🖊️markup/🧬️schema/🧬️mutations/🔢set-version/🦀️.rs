//! 🔢️ `set-version` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetVersion {
    pub(crate) version: String,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetVersion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "version", kind: "set-version", record: "SetVersion" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as protocol::Mutation<BcfSnapshot>>::Diff> {
        agg_diff(&BcfMutation::SetVersion(self.clone()), base)
    }
    fn inverse(&self, base: &BcfSnapshot) -> Vec<BcfMutation> {
        agg_inverse(&BcfMutation::SetVersion(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-version".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
