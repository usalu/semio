//! 🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak. [Design notes](README.md#source-notes-1).

//#region 🗂️OrderedOwnership

#[path = "🗂️ordered/🔢️numeric/🧮️scratch/🦀️.rs"]
pub mod numeric_scratch;
#[path="🔑️hashed/🧮️scratch/🦀️.rs"]
pub mod scratch_hash;
#[path = "📦️paged/🦀️.rs"]
pub mod paged;
pub use paged::{ByteReadSource,ByteReadRange};
#[path = "📝️shared-utf8/🦀️.rs"]
pub mod shared_utf8;
pub use shared_utf8::SharedUtf8;
//#endregion 🗂️OrderedOwnership

pub use semio_framework_value_intrinsic::bytes;



#[path = "🧬️clone/🦀️.rs"]
pub mod bounded_clone;

pub use semio_framework_value_native_encoding::native_encoding;
pub use native_encoding::{NativeEncodeControl,NativeEncodeContinuation,NativeScopedEncodeContinuation,NativeScopedEncodePauseFailure};



#[path = "⚠️refusal/🦀️.rs"]
pub mod refusal;
pub use refusal::{ValueError, ValueRefusalKind};

//#region 🔁️Codec
pub use semio_framework_value_intrinsic::{equal_f32,equal_f64,DslValue,DslValueSource,Number,JsonInteger,json_integer,edit_through_value,ControlledValueHasher,DecodedValue,IntrinsicRetirement,FromValue,ToValue,ValueEdit,ValueShape};
pub use semio_framework_value_intrinsic::{controlled_sequence,path_index};
pub use semio_framework_value_intrinsic::dsl_value;
/// 🧬 `FromValue` signatures take the control and its progress record, so every `FromValue` derive output names them through the consumer's value root.
pub use semio_framework_value_intrinsic::{NativeDecodeControl,NativeDecodeProgress};
//#endregion 🔁️Codec





#[path="📣️publication/🌳️root/🦀️.rs"]
pub mod publication_root;
pub use publication_root::{RetainedPublicationDecision,RetainedPublicationRoot};

pub use semio_framework_value_original_slot::initialization;
pub use semio_framework_value_intrinsic::native_decoding::OriginalFloatWord;

pub use semio_framework_value_equality::{OriginalValueEquality,ValueEqualityStep};
