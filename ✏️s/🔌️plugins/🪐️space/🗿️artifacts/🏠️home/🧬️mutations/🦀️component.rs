//! ⚡️ S Home launcher artifact — operation enum + laws (constitutional: op).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::home::diff::{diff_set_snapshot, SHomeDiff};
use crate::artifacts::home::SHomeSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
/// @emoji 🔢️ The Home launcher's document operation: pins the catalog-generation counter that forces a
/// re-materialize of the studio list after a create/import/delete side-effect on the catalog port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SHomeMutation {
    /// 🫙️ The identity operation — never emitted by `handle`.
    #[default]
    NoMutation,
    SetCatalogGeneration {
        value: u64,
    },
    SetSnapshot {
        snapshot: SHomeSnapshot,
    },
}

impl protocol::Mutation<SHomeSnapshot> for SHomeMutation {
    type Diff = SHomeDiff;

    fn diff(&self, _snapshot: &SHomeSnapshot) -> Self::Diff {
        match self {
            SHomeMutation::NoMutation => SHomeDiff::default(),
            SHomeMutation::SetCatalogGeneration { value } => SHomeDiff { catalog_generation: Some(*value), ..Default::default() },
            SHomeMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &SHomeSnapshot) -> Vec<Self> {
        inverse_shome_mutation(snapshot, self)
    }
}
//#endregion 🔖️Types

pub fn apply_shome_mutation(snapshot: &mut SHomeSnapshot, mutation: &SHomeMutation) {
    match mutation {
        SHomeMutation::NoMutation => {}
        SHomeMutation::SetCatalogGeneration { value } => snapshot.catalog_generation = *value,
        SHomeMutation::SetSnapshot { snapshot: replacement } => *snapshot = replacement.clone(),
    }
}

pub fn inverse_shome_mutation(snapshot: &SHomeSnapshot, mutation: &SHomeMutation) -> Vec<SHomeMutation> {
    match mutation {
        SHomeMutation::NoMutation => vec![SHomeMutation::NoMutation],
        SHomeMutation::SetCatalogGeneration { .. } => vec![SHomeMutation::SetCatalogGeneration { value: snapshot.catalog_generation }],
        SHomeMutation::SetSnapshot { .. } => vec![SHomeMutation::SetSnapshot { snapshot: snapshot.clone() }],
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SHomeMutation::NoMutation);
        store::os_store::test_support::assert_op_line_round_trip(&SHomeMutation::SetCatalogGeneration { value: 7 });
        store::os_store::test_support::assert_op_line_round_trip(&SHomeMutation::SetSnapshot { snapshot: SHomeSnapshot::default() });
    }
}
//#endregion 🧪️Tests
