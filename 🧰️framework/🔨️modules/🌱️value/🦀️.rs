//! 🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak.
//!
//! Lives beside the wire contract rather than inside the os DSL because it is what a schema-less
//! payload decodes to: the authority validates it, the optimistic replica applies it, and the
//! pathmap bodies `db` stores are trees of it. The DSL's own record/field/wire types build on it
//! and stay product-side.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

//#region 🗂️OrderedOwnership
pub use semio_framework_value_ordered::ordered;

#[path = "🗂️ordered/🔢️numeric/🧮️scratch/🦀️.rs"]
pub mod numeric_scratch;
#[path="🔑️hashed/🧮️scratch/🦀️.rs"]
pub mod scratch_hash;
#[path = "📦️paged/🦀️.rs"]
pub mod paged;
#[path = "📝️shared-utf8/🦀️.rs"]
pub mod shared_utf8;
pub use shared_utf8::SharedUtf8;
//#endregion 🗂️OrderedOwnership

pub use semio_framework_value_intrinsic::bytes;


#[path="📽️projection/🦀️.rs"]
pub mod snapshot_projection;

#[path = "🧬️clone/🦀️.rs"]
pub mod bounded_clone;

pub use semio_framework_value_native_encoding::native_encoding;
pub use native_encoding::{NativeEncodeControl,NativeEncodeContinuation};



#[path = "⚠️refusal/🦀️.rs"]
pub mod refusal;
pub use refusal::{ValueError, ValueRefusalKind};

//#region 🔁️Codec
pub use semio_framework_value_intrinsic::{DslValue,DslValueSource,Number,JsonInteger,json_integer,edit_through_value,ControlledValueHasher,DecodedValue,IntrinsicRetirement,FromValue,ToValue,ValueEdit,ValueShape};
pub use semio_framework_value_intrinsic::dsl_value;
/// 🧬 `FromValue` signatures take the control and its progress record, so every `FromValue` derive output names them through the consumer's value root.
pub use semio_framework_value_intrinsic::{NativeDecodeControl,NativeDecodeProgress};
//#endregion 🔁️Codec



pub use semio_framework_value_paged_text::paged_text;

