//! 🧪️ Independent native conformance of real public document custody, paging and accessibility.
#![cfg(test)]
use semio_framework_ui_contract::*;
use std::mem::size_of;
fn leaf_record(id:u64,key:&str)->UiNodeRecord{UiNodeRecord::with_component(UiNodeId(id),UiText::try_from_str(key).unwrap(),Component::Separator(SeparatorProps{}))}
#[path="🔖️root-reservation/🦀️.rs"]
mod root_reservation;
#[path="🔄️refresh/🦀️.rs"]
mod refresh;
#[path="📃️paged-text/🦀️.rs"]
mod paged_text;
#[path="🚚️owner-move/🦀️.rs"]
mod owner_move;
#[path="🩹️patch-pages/🦀️.rs"]
mod patch_pages;
#[path="🧩️whole-patch/🦀️.rs"]
mod whole_patch;
#[path="🌳️typed-descendants/🦀️.rs"]
mod typed_descendants;
#[path="♿️accessibility/🦀️.rs"]
mod accessibility;
#[path="🗂️records/🦀️.rs"]
mod records;
