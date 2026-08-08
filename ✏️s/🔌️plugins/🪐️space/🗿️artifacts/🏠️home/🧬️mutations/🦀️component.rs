//! ⚡️ S Home launcher artifact — operation enum + laws (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::home::SHomeDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
/// @emoji 🔢️ The Home launcher's only document operation: pins the catalog-generation counter that forces a
/// re-materialize of the studio list after a create/import/delete side-effect on the catalog port.
/// It is its own {@link protocol::MutationDiff} (idempotent set), so forward/backward are symmetric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SHomeMutation {
    /// 🫙️ The identity operation — an `MutationDiff` needs `Default`; never emitted by `handle`.
    #[default]
    NoMutation,
    SetCatalogGeneration {
        value: u64,
    },
}





impl protocol::Mutation<SHomeDocument> for SHomeMutation {
    type Diff = SHomeMutation;

    fn diff(&self, _projection: &SHomeDocument) -> SHomeMutation {
        self.clone()
    }

    fn inverse(&self, projection: &SHomeDocument) -> Vec<Self> {
        vec![SHomeMutation::SetCatalogGeneration { value: projection.catalog_generation }]
    }
}
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SHomeMutation::NoMutation);
        store::test_support::assert_op_line_round_trip(&SHomeMutation::SetCatalogGeneration { value: 7 });
    }
}
//#endregion 🧪️Tests


pub fn apply_shome_mutation(projection: &mut SHomeDocument, mutation: &SHomeMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_shome_mutation(projection: &SHomeDocument, mutation: &SHomeMutation) -> Vec<SHomeMutation> {
    mutation.inverse(projection)
}
