//! 🧪️ Independent native conformance of real public document custody, paging and accessibility.
#![cfg(test)]
use semio_framework_ui_contract::*;
use std::mem::size_of;
fn leaf_record(id:u64,key:&str)->UiNodeRecord{UiNodeRecord::with_component(UiNodeId(id),UiText::try_from_str(key).unwrap(),Component::Separator(SeparatorProps{}))}
#[path="../🎟️resident/🌳️root/🧪️tests/🔖️root-reservation/🦀️.rs"]
mod root_reservation;
#[path="../🎟️resident/🔄️refresh/🧪️tests/🔄️refresh-conformance/🦀️.rs"]
mod refresh;
#[path="../🧪️tests/📃️paged-text/🦀️.rs"]
mod paged_text;
#[path="../🧪️tests/🚚️owner-move/🦀️.rs"]
mod owner_move;
#[path="../♻️retirement/🩹️patch/🧪️tests/🩹️patch-pages/🦀️.rs"]
mod patch_pages;
#[path="../♻️retirement/🩹️patch/📨️pending/📦️whole/🧪️tests/🧩️whole-patch/🦀️.rs"]
mod whole_patch;
#[path="../♻️retirement/🌳️typed/🧪️tests/🌳️typed-descendants/🦀️.rs"]
mod typed_descendants;
#[path="../🧪️tests/♿️accessibility/🦀️.rs"]
mod accessibility;
#[path="🗂️records/🦀️.rs"]
mod records;
