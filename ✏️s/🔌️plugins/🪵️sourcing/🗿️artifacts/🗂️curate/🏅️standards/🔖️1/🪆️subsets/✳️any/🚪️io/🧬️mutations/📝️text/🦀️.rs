//! ⚡️ Sourcing curate artifact — OpText/OpBinary codecs for `SourcingMutation`. Mutation
//! apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::curate::schema::mutations::SourcingMutation;
use crate::artifacts::curate::schema::mutations::{change_curated_item_count, create_curated_item, delete_curated_item};
use crate::artifacts::curate::CuratedItem;
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
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
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
mod tests {
    use super::*;

    fn every_mutation() -> Vec<SourcingMutation> {
        vec![
            SourcingMutation::CreateCuratedItem(create_curated_item::CreateCuratedItem { item: CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 3 } }),
            SourcingMutation::DeleteCuratedItem(delete_curated_item::DeleteCuratedItem { object_id: "beam-glulam-gl24h".into() }),
            SourcingMutation::ChangeCuratedItemCount(change_curated_item_count::ChangeCuratedItemCount { object_id: "beam-glulam-gl24h".into(), new_count: 5 }),
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_create_curated_item() {
        store::os_store::test_support::assert_op_line_round_trip(&SourcingMutation::CreateCuratedItem(create_curated_item::CreateCuratedItem { item: CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 3 } }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_delete_curated_item() {
        store::os_store::test_support::assert_op_line_round_trip(&SourcingMutation::DeleteCuratedItem(delete_curated_item::DeleteCuratedItem { object_id: "beam-glulam-gl24h".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_change_curated_item_count() {
        store::os_store::test_support::assert_op_line_round_trip(&SourcingMutation::ChangeCuratedItemCount(change_curated_item_count::ChangeCuratedItemCount { object_id: "beam-glulam-gl24h".into(), new_count: 5 }));
    }

    /// ⚖️ Every variant, not just the three hand-picked above — full-coverage `OpText` round trip
    /// over the closed vocabulary, one sample value per field.
    #[semio_framework_async_macros::async_test]
    async fn every_variant_op_text_round_trips() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
    }
}
//#endregion 🧪️Tests
