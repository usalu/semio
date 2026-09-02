//! 📝️ Curate text identities derived from the direct semantic owners.

pub const TEXT_OPCODES: &[(&str, &str)] =
    &[("CreateCuratedItem", super::create_curated_item::text::TEXT_OPCODE), ("DeleteCuratedItem", super::delete_curated_item::text::TEXT_OPCODE), ("ChangeCuratedItemCount", super::change_curated_item_count::text::TEXT_OPCODE)];
