//! 🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak. [Design notes](README.md#source-notes-1).

//#region 🗂️OrderedOwnership

#[path = "🗂️ordered/🔢️numeric/🧮️scratch/🦀️.rs"]
pub mod numeric_scratch;
#[path="🔑️hashed/🧮️scratch/🦀️.rs"]
pub mod scratch_hash;
pub use semio_framework_value_paged as paged;
pub use paged::{ByteReadSource,ByteReadRange};
pub use semio_framework_value_shared_utf8 as shared_utf8;
pub use shared_utf8::SharedUtf8;
//#endregion 🗂️OrderedOwnership

pub use semio_framework_value_intrinsic::bytes;



#[path = "🧬️clone/🦀️.rs"]
pub mod bounded_clone;
#[path="🧾️birth-ledger/🦀️.rs"]
pub mod birth_ledger;
pub use birth_ledger::OriginalBirthLedger;
pub use semio_framework_value_retirement_contract::{ReplyStash,Slot,SlotGuard};

pub use semio_framework_value_native_encoding::native_encoding;
pub use native_encoding::{NativeEncodeControl,NativeEncodeContinuation,NativeScopedEncodeContinuation,NativeScopedEncodePauseFailure};



/// ⚠️ Canonical original refusal contracts, owned by the control contract.
pub mod refusal{pub use semio_framework_value_control_contract::{ValueError,ValueRefusalKind};}
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

pub use semio_framework_value_equality::{OriginalValueEquality};

/// 🐛️ [DEBUG] Temporary thread-local unit histogram that names the dominant publication phases of the 3D mutation-latency regression; remove it with its call sites once the ladder is measured.
pub mod debug_histogram{
 use std::cell::Cell;
 /// 🏷️ [DEBUG] Every counted label in print order; an unknown label counts as the last one.
 pub const DEBUG_UNIT_LABELS:[&str;67]=["apply.census","apply.item_close.retiring","apply.item_close.preparation","apply.folding","apply.flip_cursor","apply.begin_next","apply.preparation_loop","apply.fold","apply.cursor","apply.terminal","preflight.digest.take","preflight.digest.release","preflight.digest.admit_edit","preflight.digest.admission","preflight.digest.seal_birth","preflight.digest.seal.countStart","preflight.digest.seal.count","preflight.digest.seal.header","preflight.digest.seal.hashStart","preflight.digest.seal.hash","preflight.digest.seal.takeOwner","preflight.digest.seal.sourceClose","preflight.digest.seal.finalize","preflight.digest.seal.ready","preflight.digest.seal.hashPending","preflight.digest.seal.closing","preflight.digest.other","preflight.begin_digest","preflight.diagnostics","preflight.flip","publishing.message","publishing.metadata","publishing.commit","publishing.diagnostics","publishing.transfer","result.blocked","result.progress","result.published","result.refused","result.terminal","result.error","run.scope","run.steps","run.stop.more","run.stop.work","run.stop.items","run.stop.blocked","run.stop.other","closing.publication_failure","closing.actor_capture_pending","closing.actor_capture","closing.reserved_producer","closing.raw_input","closing.refusal_capture","closing.pending_publication","closing.pending_child","closing.publication_retirement","closing.publication","closing.completion_retirement","closing.completion","closing.output_retirement","closing.output_chunks","closing.none","closing.not_ready","digest.items","digest.copy_bytes","other"];
 thread_local!{static DEBUG_UNITS:[Cell<u64>;67]=const{[const{Cell::new(0)};67]};}
 /// ➕️ [DEBUG] Adds `count` under `label` on this thread without allocating.
 pub fn debug_unit_add(label:&'static str,count:u64){let index=DEBUG_UNIT_LABELS.iter().position(|known|*known==label).unwrap_or(DEBUG_UNIT_LABELS.len()-1);DEBUG_UNITS.with(|units|units[index].set(units[index].get().saturating_add(count)));}
 /// 🔢️ [DEBUG] Counts one unit under `label` on this thread.
 pub fn debug_unit(label:&'static str){debug_unit_add(label,1)}
 /// 🧹️ [DEBUG] Clears this thread's histogram.
 pub fn debug_units_reset(){DEBUG_UNITS.with(|units|units.iter().for_each(|unit|unit.set(0)));}
 /// 🖨️ [DEBUG] Renders this thread's nonzero counts as `[DEBUG] label=count ...`.
 pub fn debug_units_report()->String{DEBUG_UNITS.with(|units|DEBUG_UNIT_LABELS.iter().zip(units.iter()).filter(|(_,unit)|unit.get()!=0).fold(String::from("[DEBUG]"),|report,(label,unit)|format!("{report} {label}={}",unit.get())))}
}
