//! 📦️ `add-stored-entry` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by
//! construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddStoredEntry {
    pub(crate) entry: ZipEntry,
}

impl protocol::MutationKind<ZipSnapshot, ZipIso21320Mutation> for AddStoredEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "stored-entry", kind: "add-stored-entry", record: "AddStoredEntry" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        agg_diff(&ZipIso21320Mutation::AddStoredEntry(self.clone()), base)
    }
    fn inverse(&self, base: &ZipSnapshot) -> Vec<ZipIso21320Mutation> {
        agg_inverse(&ZipIso21320Mutation::AddStoredEntry(self.clone()), base)
    }
    fn label(&self) -> String {
        "add-stored-entry".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
