//! 📥️ `insert-record` — its own mutation leaf. The aggregate's original `diff`/`inverse` bodies were
//! lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRecord {
    pub index: usize,
    pub record: CsvRecord,
}

impl protocol::MutationKind<CsvSnapshot, CsvMutation> for InsertRecord {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "record", kind: "insert-record", record: "InsertRecord" };

    fn diff(&self, base: &CsvSnapshot) -> protocol::MutationOutcome<<CsvMutation as protocol::Mutation<CsvSnapshot>>::Diff> {
        agg_diff(&CsvMutation::InsertRecord(self.clone()), base)
    }
    fn inverse(&self, base: &CsvSnapshot) -> Vec<CsvMutation> {
        agg_inverse(&CsvMutation::InsertRecord(self.clone()), base)
    }
    fn label(&self) -> String { "insert-record".to_string() }
    fn target(&self) -> Vec<String> { Vec::new() }
}
//#endregion 🔖️Payload
