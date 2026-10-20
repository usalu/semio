//! 🧪️ The semio value per-mutation inverse laws, over the mutation crate's public API.

extern crate semio_framework_os_kernel as protocol;

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, SemioValueSnapshot, ValueId, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};

/// 🧱️ The committed base every inverse law runs against: a two-entry map root and one graph node.
pub(crate) fn base_fixture() -> SemioValueSnapshot {
    let int = |lexeme: &str| SemioValue::Int { lexeme: lexeme.into() };
    let root = SemioValue::Map { entries: vec![SemioValueEntry { key: "a".into(), value: int("1") }, SemioValueEntry { key: "list".into(), value: SemioValue::List { items: vec![int("1"), int("2")] } }] };
    SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes: vec![SemioValueNode { id: ValueId::new("n1"), value: SemioValue::Str { value: "hello".into() } }] }
}

#[path = "../../✂️remove-node/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_node;
#[path = "../../✖️remove-map-entry/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_map_entry;
#[path = "../../➕insert-list-item/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_insert_list_item;
#[path = "../../➖remove-list-item/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_list_item;
#[path = "../../🔁set-value/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_value;
#[path = "../../🗝️set-map-entry/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_map_entry;
#[path = "../../🧷set-node/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_node;
