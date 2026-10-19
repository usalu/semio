//! 🌱️ Canonical bounded UI values, collection arenas, serde and exact retirement.
use serde::{Deserialize,Deserializer,Serialize,Serializer};
use std::{fmt,mem::size_of,sync::{LazyLock,Mutex}};
pub use semio_framework_ui_contract_budget::*;
pub use semio_framework_ui_contract_text::{UiText,UiTextOriginalCopy,UI_TEXT_CLIP_MARK};
pub use semio_framework_ui_contract_identity::{UiNodeId,UiRevision,SurfaceId};
pub const UI_VALUE_NONE:usize=usize::MAX;
use semio_framework_value::list as fixed_list_storage;
pub use fixed_list_storage::{PagedListAllocationError,PagedListError,PagedListProgress,PagedListRefusalKind};

#[path = "🔢️fixed-list/🦀️.rs"]
mod fixed_list;
pub use fixed_list::{UiFixedList, UiFixedListIter, UiFixedListIterMut};

#[path = "🗺️fixed-map-and-bytes/🦀️.rs"]
mod fixed_map_and_bytes;
pub use fixed_map_and_bytes::{UiFixedBytes, UiFixedMap};

#[path = "🏟️collection-arena/🦀️.rs"]
mod collection_arena;
use collection_arena::{
    with_ui_value_arena, UiCollectionHandle, UiCollectionKind, UiPageValue, UiValueArena,
    UI_VALUE_ARENA,
};
pub use collection_arena::{
    close_ui_value_page_one, resident_static_backing_bytes, ui_value_headroom, UiValueHeadroom,
    UI_VALUE_AGGREGATE_BYTES,
};

#[path = "📃️ui-list/🦀️.rs"]
mod ui_list;
pub use ui_list::{UiList, UiListBuilder, UiListCursor};

#[path = "🗝️ui-map/🦀️.rs"]
mod ui_map;
pub use ui_map::{UiMap, UiMapBuilder, UiMapCursor};

//#endregion 📋️FixedListOwnership

#[path = "♻️retirement/🦀️.rs"]
mod retirement;
pub use retirement::{close_ui_value_page_with_grant,UiValueRetirement,UiValueRetirementStep};
pub use retirement::{UiArenaHandback,UiArenaHandbacks,UiTypedRetire,UiTypedRetirementCursor};

//#region 🔖️Value
/// 🧬️ A neutral, JSON-shaped value — the ONE recursive type in this crate. Every node in
/// `📃️document.rs` avoids inline recursion by addressing children through [`crate::UiNodeId`] instead
/// of nesting a node inside another; `UiValue` is the deliberate exception because it does not
/// describe document shape at all, it describes an arbitrary opaque payload (action args, extension
/// props) that genuinely IS JSON-shaped, and owned fixed list/map pages give the schema an indirection
/// to resolve (heap-allocated, not an inline field) rather than the infinitely-sized-struct problem
/// direct node-in-node nesting would create.
///
/// ⚠️ The os-kernel's `DslValue` (`🧰️framework/🔨️modules/🌱️value/🦀️.rs`) must NEVER appear in
/// this crate — this crate has no such dependency and stays `wasm32-wasip2`/`wasm32-unknown-unknown`
/// safe by construction. `From`/`Into` conversions between `UiValue` and `DslValue` belong in the
/// os-kernel crate, never here.
// 🌱️ No `ToValue`/`FromValue` on `UiValue` — see the DslValue prohibition above. `#[serde(untagged)]`
// also has no `#[value(...)]` equivalent regardless (would need a hand-written impl).
#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(untagged)]
#[expect(clippy::large_enum_variant, reason = "Text stays in its fixed inline byte ceiling; collection payloads use separately credited arena handles.")]
pub enum UiValue {
    #[default]
    Null,
    Bool(bool),
    Number(f64),
    Text(UiText),
    List(UiList),
    Map(UiMap),
}

impl UiValue {
    /// 🎟️ Attempts an alias of this exact immutable value without blocking its issuing arena.
    pub fn try_credited_clone(&self)->Result<Option<Self>,&'static str>{
        let mut arena=match UI_VALUE_ARENA.try_lock(){Ok(arena)=>arena,Err(std::sync::TryLockError::WouldBlock)=>return Ok(None),Err(std::sync::TryLockError::Poisoned(_))=>return Err("UI value alias arena is poisoned")};
        arena.try_clone_value(self).map(Some).ok_or("UI value exact alias admission failed")
    }
    pub fn credited_clone(&self) -> Option<Self> {
        with_ui_value_arena(|arena| arena.try_clone_value(self))
    }
}

/// 🔢️ A `UiValue` serializes as the JSON text React's wire carries: a number goes through the value module's one rule
/// ([`semio_framework_value::json_integer`]) — `7.0` is written `7`, a non-finite number `null` — so every JSON projection of a UI
/// value (reconciled action arguments, extension params, snapshots) reads back exactly as `JSON.parse(JSON.stringify(v))`.
impl Serialize for UiValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            UiValue::Null => serializer.serialize_unit(),
            UiValue::Bool(value) => serializer.serialize_bool(*value),
            UiValue::Number(value) => match semio_framework_value::json_integer(*value) {
                Some(semio_framework_value::JsonInteger::Unsigned(integer)) => serializer.serialize_u64(integer),
                Some(semio_framework_value::JsonInteger::Signed(integer)) => serializer.serialize_i64(integer),
                None if value.is_finite() => serializer.serialize_f64(*value),
                None => serializer.serialize_unit(),
            },
            UiValue::Text(value) => value.serialize(serializer),
            UiValue::List(value) => value.serialize(serializer),
            UiValue::Map(value) => value.serialize(serializer),
        }
    }
}

//#endregion 🔖️Value

pub mod retirement_protocol{pub use crate::retirement::typed::{done,split,field_step,maximum_depth,assert_copy,UI_TYPED_RETIREMENT_DEPTH};}
#[path="📋️copy/🦀️.rs"]
pub mod copy_protocol;
#[path="⚖️compare/🦀️.rs"]
pub mod compare_protocol;
pub use copy_protocol::UiComponentCopyProgress;
pub use compare_protocol::UiComponentCompareProgress;

#[cfg(any(test,feature="testkit"))]
#[path="🧪️observer/🦀️.rs"]
pub mod test_observer;

#[cfg(any(test, feature = "testkit"))]
pub mod testkit {
    pub use super::{fixed_list::*, fixed_map_and_bytes::*, collection_arena::*, ui_list::*, ui_map::*, retirement::*};
}
