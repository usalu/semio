//! 📥️ `insert-record` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct InsertRecord {
    pub index: usize,
    pub record: EpwRecord,
}

impl protocol::MutationKind<EpwSnapshot, EpwMutation> for InsertRecord {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "record", kind: "insert-record", record: "InsertRecord" };

    fn diff(&self, base: &EpwSnapshot) -> protocol::MutationOutcome<<EpwMutation as protocol::Mutation<EpwSnapshot>>::Diff> {
        agg_diff(&EpwMutation::InsertRecord(self.clone()), base)
    }
    fn inverse(&self, base: &EpwSnapshot) -> Vec<EpwMutation> {
        agg_inverse(&EpwMutation::InsertRecord(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-record".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
