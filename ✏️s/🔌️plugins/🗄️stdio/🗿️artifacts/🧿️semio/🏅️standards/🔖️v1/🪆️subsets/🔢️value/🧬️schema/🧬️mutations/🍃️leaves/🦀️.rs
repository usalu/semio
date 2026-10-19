//! 🍃️ The seven semio value mutation leaves with their diffs and inverses, generic over the dispatching aggregate, held below the aggregate crate so it stays inside the source gate.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "../🧭️path-vocabulary/🦀️.rs"]
pub mod path;

#[path = "../➕insert-list-item/🦀️.rs"]
pub mod insert_list_item;
#[path = "../➖remove-list-item/🦀️.rs"]
pub mod remove_list_item;
#[path = "../✖️remove-map-entry/🦀️.rs"]
pub mod remove_map_entry;
#[path = "../✂️remove-node/🦀️.rs"]
pub mod remove_node;
#[path = "../🗝️set-map-entry/🦀️.rs"]
pub mod set_map_entry;
#[path = "../🧷set-node/🦀️.rs"]
pub mod set_node;
#[path = "../🔁set-value/🦀️.rs"]
pub mod set_value;
