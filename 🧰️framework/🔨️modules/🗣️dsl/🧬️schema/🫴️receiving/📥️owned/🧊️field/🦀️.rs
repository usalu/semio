//! 🧊️ Any natively receivable tree reads the moved field in place through the field's own source port, so values, sets, maps and blocks need no per-type cursor.
use super::*;
use semio_framework_value_native_owned::native_decoding::{NativeReceive,owned::tree::NativeValueSource};
use semio_framework_value_ordered::ordered::{OrderedMap,OrderedSet};
/// 🧊️ The whole moved field is the original source of one native receiver, for literals, blocks, sets and maps alike; denied turns charge nothing and Pending keeps every owner.
pub type NativeFieldReceiving<T>=NativeSourceReceiving<FieldValue,T,WholeOrigin>;
/// 🔢️ An ordered string membership field.
pub type SetReceiving=NativeFieldReceiving<OrderedSet>;
/// 🗺️ An ordered keyed map field whose values are natively receivable.
pub type MapReceiving<V>=NativeFieldReceiving<OrderedMap<V>>;
impl DslOwnedReceivingField for OrderedSet{type Cursor=SetReceiving;fn receiving_cursor()->Self::Cursor{Default::default()}}
impl<V:NativeReceive<dyn NativeValueSource>+RetireOwned+Sync+Send+'static>DslOwnedReceivingField for OrderedMap<V>{type Cursor=MapReceiving<V>;fn receiving_cursor()->Self::Cursor{Default::default()}}
