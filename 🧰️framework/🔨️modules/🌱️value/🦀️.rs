//! 🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak.
//!
//! Lives beside the wire contract rather than inside the os DSL because it is what a schema-less
//! payload decodes to: the authority validates it, the optimistic replica applies it, and the
//! pathmap bodies `db` stores are trees of it. The DSL's own record/field/wire types build on it
//! and stay product-side.
//!
//! 🚱️ In-place shared retirement drains fields of records without a custom destructor.
//! ```
//! #[derive(semio_framework_value::RetireOwned)]
//! #[value(crate="semio_framework_value")]
//! struct Drained(u8);
//! assert!(semio_framework_value::retirement::RetireOwned::shared_original_birth_bytes(&Drained(7)).is_some());
//! ```
//! ```compile_fail,E0283
//! #[derive(semio_framework_value::RetireOwned)]
//! #[value(crate="semio_framework_value")]
//! struct CustomDestructor(u8);
//! impl Drop for CustomDestructor { fn drop(&mut self) {} }
//! ```
//! 🎯️ Every clone cursor must author a correlated close selector.
//! ```
//! use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneCursor,RetainedCloneGrant,RetainedCloneRef,RetainedCloneStep}};
//! struct Cursor;
//! impl RetainedCloneCursor<u8> for Cursor {
//!     fn advance(&mut self,_:RetainedCloneRef<'_,u8>,_:usize,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Progress(Default::default()))}
//!     fn take(&mut self)->Option<u8>{None}
//!     fn begin_close(&mut self)->bool{true}
//!     fn terminal_is_empty(&self)->bool{true}
//!     fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
//!     fn close_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
//! }
//! assert_eq!(RetainedCloneCursor::<u8>::close_demands(&Cursor,4096).unwrap(),RetirementDemand::default());
//! ```
//! ```compile_fail,E0046
//! use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneCursor,RetainedCloneGrant,RetainedCloneRef,RetainedCloneStep}};
//! struct Cursor;
//! impl RetainedCloneCursor<u8> for Cursor {
//!     fn advance(&mut self,_:RetainedCloneRef<'_,u8>,_:usize,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Progress(Default::default()))}
//!     fn take(&mut self)->Option<u8>{None}
//!     fn begin_close(&mut self)->bool{true}
//!     fn terminal_is_empty(&self)->bool{true}
//!     fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
//! }
//! ```
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


pub use semio_framework_value_snapshot_projection::snapshot_projection;

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

