//! 📄️ `set-snapshot` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by
//! construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub(crate) snapshot: ZipSnapshot,
}

impl protocol::MutationKind<ZipSnapshot, ZipIso21320Mutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        agg_diff(&ZipIso21320Mutation::SetSnapshot(self.clone()), base)
    }
    fn inverse(&self, base: &ZipSnapshot) -> Vec<ZipIso21320Mutation> {
        agg_inverse(&ZipIso21320Mutation::SetSnapshot(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-snapshot".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
