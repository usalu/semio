//! ⏯️ Domain-neutral tool run contract: identity, the pure lifecycle reducer, progress and step
//! ring, trace pages with their resident store, tick codec and writer, the static
//! `ToolRunDefinition`, the framework-reserved actions, chords and EN/DE labels.
//!
//! Pure: no async, no store. Schema of record: `🧬️schema/🔣️.json`. Contract:
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` §2, §3.1, §3.2.

use semio_framework_dsl_record::FieldSpec;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::Shape;
use pack::record::{decode_record_body_exact, encode_record_body, DecodeOptions, EncodeOptions};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::OnceLock;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
#[path="🎞️tick/🛬️receive/🎟️limits/🦀️.rs"]
mod receiving_limits;
pub use receiving_limits::{ToolRunRecordReadLimits,ToolRunHistoryReadLimits,ToolRunReceivingLimits};

#[path="📊️progress/📸️clone/🦀️.rs"]
pub mod progress_clone;
#[path="📊️progress/💍️steps/➕️insert/🦀️.rs"]
pub mod step_ring_insert;
#[path="📽️tick/🔐️source/🦀️.rs"]
pub mod tick_source;

/// 🧪️ Hooks of the companion test crate `semio-framework-tool-run-tests`: the modules whose items its laws name; compiled only with the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub use super::tick_source;
}
#[path="📨️payload/🌊️stream/🦀️.rs"]
pub mod payload_stream;
use payload_stream::ToolRunPayloadUpdate;
//#region 🔖️Limits
/// 💍️ Newest steps kept by a `ToolRunStepRing`.
pub const TOOL_RUN_STEP_RING_CAPACITY: usize = 64;
/// 🧷️ Arguments one step substitutes into its reason template.
pub const TOOL_RUN_STEP_ARGS_MAX: usize = 4;
/// 🧮️ Counters one progress snapshot carries.
pub const TOOL_RUN_COUNTERS_MAX: usize = 9;
/// 🧱️ Provisional document ops one run may hold (§2.8).
pub const TOOL_RUN_PROVISIONAL_OPS_MAX: u32 = 65_536;
/// 🪆️ Provisional member ops one member-target run may hold: its finalize publishes them as ONE composed-child group in a
/// single turn, so this ceiling bounds that turn (design §20.14).
pub const TOOL_RUN_MEMBER_OPS_MAX: u32 = 4_096;
/// 🗄️ Trace records a run keeps resident (§3.2).
pub const TOOL_RUN_TRACE_RESIDENT_RECORDS: usize = 1_048_576;
/// 📄️ Ops one trace page carries.
pub const TOOL_RUN_TRACE_PAGE_OPS_MAX: usize = 4_096;
/// 📦️ Encoded bytes one trace page may take (`LARGE_PREVIEW_PATCH_BYTES`).
pub const TOOL_RUN_TRACE_PAGE_BYTES_MAX: usize = 262_144;
/// 🎞️ Encoded bytes one tick may take (one `RetainedJobPayload` page).
pub const TOOL_RUN_TICK_BYTES_MAX: usize = 262_144;
/// 🗜️ Minimum logged ops before a trace store compacts its page log.
pub const TOOL_RUN_TRACE_LOG_COMPACT_FLOOR: usize = 4_096;
/// 📣️ Minimum interval between polite status announcements (§2.6).
pub const TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS: u64 = 2_000;
/// 🚧️ First reason code reserved for framework-owned steps; plugin reasons stay below it.
pub const TOOL_RUN_RESERVED_REASON_FLOOR: u16 = 0xFF00;
/// 🔁️ Framework step: the document changed and the provisional result is being re-applied.
pub const TOOL_RUN_REASON_REBASING: u16 = 0xFF00;
/// ⚔️ Framework step: `{0}` provisional changes conflict with the current artifact.
pub const TOOL_RUN_REASON_CONFLICT: u16 = 0xFF01;
/// ✂️ Framework step: the oldest `{0}` rejected attempts were evicted from the trace.
pub const TOOL_RUN_REASON_TRACE_TRUNCATED: u16 = 0xFF02;
/// 🧯️ Framework step: the provisional op cap `{0}` was reached and the run completed.
pub const TOOL_RUN_REASON_PROVISIONAL_CAP: u16 = 0xFF03;
/// 🏷️ Prefix of the `group_id` stamped on the one `Edit` a finalized run publishes.
pub const TOOL_RUN_GROUP_ID_PREFIX: &str = "toolRun:";
//#endregion 🔖️Limits

//#region 🔖️Identity
/// 🪪️ Per-instance run id; `run` is monotone and never reused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_value::RetireOwned)]
pub struct ToolRunId {
    pub app_instance_id: u32,
    pub run: u64,
}

impl ToolRunId {
    /// 🎫️ The `group_id` of the single `Edit` this run publishes at finalize.
    /// 🧵️ Writes the exact run group identity into caller-owned fixed storage.
    pub fn group_id_into(self,buffer:&mut [u8;32])->&str{
        let prefix=TOOL_RUN_GROUP_ID_PREFIX.as_bytes();buffer[..prefix.len()].copy_from_slice(prefix);let mut digits=[0u8;20];let mut cursor=20;let mut value=self.run;loop{cursor-=1;digits[cursor]=b'0'+(value%10)as u8;value/=10;if value==0{break}}
        let length=prefix.len()+20-cursor;buffer[prefix.len()..length].copy_from_slice(&digits[cursor..]);std::str::from_utf8(&buffer[..length]).unwrap()
    }

    pub fn group_id(self) -> String {
        format!("{TOOL_RUN_GROUP_ID_PREFIX}{}", self.run)
    }
}

/// 🧿️ Run id plus the staleness generation and the committed revision the overlay was folded from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_value::RetireOwned)]
pub struct ToolRunIdentity {
    pub id: ToolRunId,
    pub generation: u32,
    pub base_revision: [u8; 32],
}

impl ToolRunIdentity {
    /// 🌱️ Generation 0 identity of a freshly started run.
    pub fn new(id: ToolRunId, base_revision: [u8; 32]) -> Self {
        Self { id, generation: 0, base_revision }
    }

    /// ⏫️ Same run, generation + 1 (reconfigure, rebase, conflict return).
    pub fn next_generation(self) -> Self {
        Self { generation: self.generation.wrapping_add(1), ..self }
    }

    /// 🥇️ Lexicographic freshness key for pages, progress and panels at `sequence`.
    pub fn freshness(self, sequence: u64) -> ToolRunFreshness {
        ToolRunFreshness { run: self.id.run, generation: self.generation, sequence }
    }
}

/// 🥈️ `(run, generation, sequence)`; renderers drop anything older than the newest seen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToolRunFreshness {
    pub run: u64,
    pub generation: u32,
    pub sequence: u64,
}
//#endregion 🔖️Identity

#[path = "🚦️lifecycle/🦀️.rs"]
mod lifecycle;
pub use lifecycle::*;

#[path = "🧮️step-progress/🦀️.rs"]
mod step_progress;
pub use step_progress::*;

#[path = "🗄️trace-store/🦀️.rs"]
mod trace_store;
pub use trace_store::*;

#[path = "⏱️tick-writer/🦀️.rs"]
mod tick_writer;
pub use tick_writer::*;

#[path = "🔤️codec/🦀️.rs"]
mod record_codec;
pub use record_codec::*;

#[path = "📐️definition/🦀️.rs"]
mod definition;
pub use definition::*;

#[path = "🏷️actions-labels/🦀️.rs"]
mod actions_labels;
pub use actions_labels::*;

#[path="♻️retirement/🦀️.rs"]
mod original_retirement;


#[cfg(test)]
#[global_allocator]
static TOOL_RUN_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

#[cfg(test)]
#[path="🪪️group/🦀️.rs"]
mod original_group_tests;
