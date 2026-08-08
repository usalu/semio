//! 🔺️ S Home launcher artifact — operation diff laws (constitutional: diff).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeDocument;

//#region 🔖️MutationDiff
impl protocol::MutationDiff<SHomeDocument> for SHomeMutation {
    fn apply(&self, projection: &SHomeDocument) -> SHomeDocument {
        match self {
            SHomeMutation::NoMutation => projection.clone(),
            SHomeMutation::SetCatalogGeneration { value } => SHomeDocument { catalog_generation: *value, ..projection.clone() },
        }
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, SHomeMutation::NoMutation) {
            *self = other;
        }
    }
}
//#endregion 🔖️MutationDiff


/// 🔺️ Home launcher diff fragment — `SHomeMutation` is its own idempotent diff.
pub type SHomeDiff = SHomeMutation;
