//! ➖️ `remove-entry` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, dsl::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-entry")]
pub struct RemoveEntry {
    pub(crate) name: String,
}

impl protocol::MutationKind<ZipSnapshot, ZipMutation> for RemoveEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "entry", kind: "remove-entry", record: "RemoveEntry" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipMutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        agg_diff(&ZipMutation::RemoveEntry(self.clone()), base)
    }
    fn inverse(&self, base: &ZipSnapshot) -> Vec<ZipMutation> {
        agg_inverse(&ZipMutation::RemoveEntry(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Remove entry", "Eintrag entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
