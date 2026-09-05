//! ➖️ `remove-entry` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEntry {
    pub(crate) name: String,
}

impl protocol::MutationKind<ZipSnapshot, ZipIso21320Mutation> for RemoveEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entry", kind: "remove-entry", record: "RemoveEntry" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        agg_diff(&ZipIso21320Mutation::RemoveEntry(self.clone()), base)
    }
    fn inverse(&self, base: &ZipSnapshot) -> Vec<ZipIso21320Mutation> {
        agg_inverse(&ZipIso21320Mutation::RemoveEntry(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-entry".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
