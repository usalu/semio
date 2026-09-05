
use crate::artifacts::csv::CsvSnapshot;
use crate::artifacts::csv::schema::mutations::{CsvMutation, apply_csv_mutation};

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut CsvSnapshot, mutation: &CsvMutation) {
    apply_csv_mutation(projection, mutation);
}

//#region 🔖️Payload

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: CsvSnapshot,
}

impl protocol::MutationKind<CsvSnapshot, CsvMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &CsvSnapshot) -> protocol::MutationOutcome<<CsvMutation as protocol::Mutation<CsvSnapshot>>::Diff> {
        crate::artifacts::csv::schema::mutations::agg_diff(&CsvMutation::SetSnapshot(SetSnapshot { snapshot: self.snapshot.clone() }), base)
    }
    fn inverse(&self, base: &CsvSnapshot) -> Vec<CsvMutation> {
        crate::artifacts::csv::schema::mutations::agg_inverse(&CsvMutation::SetSnapshot(SetSnapshot { snapshot: self.snapshot.clone() }), base)
    }
    fn label(&self) -> String { "set-snapshot".to_string() }
    fn target(&self) -> Vec<String> { Vec::new() }
}
//#endregion 🔖️Payload
