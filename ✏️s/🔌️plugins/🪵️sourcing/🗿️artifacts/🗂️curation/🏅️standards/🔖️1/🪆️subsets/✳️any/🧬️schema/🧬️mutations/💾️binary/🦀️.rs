//! 💾️ Curation binary identities derived from the direct semantic owners.

pub const BINARY_TAGS: &[(&str, u8)] =
    &[("CreateCuratedItem", super::create_curated_item::binary::BINARY_TAG), ("DeleteCuratedItem", super::delete_curated_item::binary::BINARY_TAG), ("ChangeCuratedItemCount", super::change_curated_item_count::binary::BINARY_TAG)];
