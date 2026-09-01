//! 🧾️ `set-has-header` — its own mutation leaf. The aggregate's original `diff`/`inverse` bodies were
//! lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetHasHeader {
    pub has_header: bool,
}

impl protocol::MutationKind<CsvSnapshot, CsvMutation> for SetHasHeader {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "has-header", kind: "set-has-header", record: "SetHasHeader" };

    fn diff(&self, base: &CsvSnapshot) -> protocol::MutationOutcome<<CsvMutation as protocol::Mutation<CsvSnapshot>>::Diff> {
        agg_diff(&CsvMutation::SetHasHeader(self.clone()), base)
    }
    fn inverse(&self, base: &CsvSnapshot) -> Vec<CsvMutation> {
        agg_inverse(&CsvMutation::SetHasHeader(self.clone()), base)
    }
    fn label(&self) -> String { "set-has-header".to_string() }
    fn target(&self) -> Vec<String> { Vec::new() }
}
//#endregion 🔖️Payload
