//! ✍️ `set-entry-data` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by
//! construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf, dsl::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEntryData {
    pub(crate) name: String,
    #[dsl(base64)]
    pub(crate) data: Vec<u8>,
}

impl protocol::MutationKind<ZipSnapshot, ZipMutation> for SetEntryData {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entry-data", kind: "set-entry-data", record: "SetEntryData" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipMutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        agg_diff(&ZipMutation::SetEntryData(self.clone()), base)
    }
    fn inverse(&self, base: &ZipSnapshot) -> Vec<ZipMutation> {
        agg_inverse(&ZipMutation::SetEntryData(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-entry-data".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
