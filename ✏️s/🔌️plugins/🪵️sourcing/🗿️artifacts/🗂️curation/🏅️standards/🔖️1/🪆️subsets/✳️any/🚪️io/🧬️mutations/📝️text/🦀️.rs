//! ⚡️ Sourcing curation artifact — OpText/OpBinary codecs for `SourcingMutation`. Mutation
//! apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::schema::mutations::SourcingMutation;
use crate::schema::mutations::{change_curated_item_count, create_curated_item, delete_curated_item};
use crate::CuratedItem;
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `SourcingMutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only; `SourcingMutation` itself,
/// and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum SourcingMutationDsl {
    CreateCuratedItem {
        #[dsl(block)]
        item: CuratedItem,
    },
    DeleteCuratedItem {
        object_id: String,
    },
    ChangeCuratedItemCount {
        object_id: String,
        new_count: u32,
    },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for SourcingMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for SourcingMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn sourcing_mutation_to_dsl(mutation: &SourcingMutation) -> SourcingMutationDsl {
    match mutation {
        SourcingMutation::CreateCuratedItem(payload) => SourcingMutationDsl::CreateCuratedItem { item: payload.item.clone() },
        SourcingMutation::DeleteCuratedItem(payload) => SourcingMutationDsl::DeleteCuratedItem { object_id: payload.object_id.clone() },
        SourcingMutation::ChangeCuratedItemCount(payload) => SourcingMutationDsl::ChangeCuratedItemCount { object_id: payload.object_id.clone(), new_count: payload.new_count },
    }
}

fn sourcing_mutation_from_dsl(mutation: SourcingMutationDsl) -> SourcingMutation {
    match mutation {
        SourcingMutationDsl::CreateCuratedItem { item } => SourcingMutation::CreateCuratedItem(create_curated_item::CreateCuratedItem { item }),
        SourcingMutationDsl::DeleteCuratedItem { object_id } => SourcingMutation::DeleteCuratedItem(delete_curated_item::DeleteCuratedItem { object_id }),
        SourcingMutationDsl::ChangeCuratedItemCount { object_id, new_count } => SourcingMutation::ChangeCuratedItemCount(change_curated_item_count::ChangeCuratedItemCount { object_id, new_count }),
    }
}

impl OpText for SourcingMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(sourcing_mutation_from_dsl(<SourcingMutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <SourcingMutationDsl as OpText>::print_op(&sourcing_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `SourcingMutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for SourcingMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        sourcing_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(sourcing_mutation_from_dsl(SourcingMutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
