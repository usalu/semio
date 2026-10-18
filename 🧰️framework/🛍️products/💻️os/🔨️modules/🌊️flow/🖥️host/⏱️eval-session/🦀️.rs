//! ⏱️ Behaviour of `FlowEvalSession`: baseline capture, ticks, window latches, preview retention and close steps.

use super::*;

#[path = "🔌️construction-and-geometry-port/🦀️.rs"]
mod construction_and_geometry_port;

#[path = "📨️invocation-origin-reply-and-snapshot-intake/🦀️.rs"]
mod invocation_origin_reply_and_snapshot_intake;

#[path = "🌡️baseline-tick-sync-and-eval-json/🦀️.rs"]
mod baseline_tick_sync_and_eval_json;

#[path = "🔒️window-tick-latch/🦀️.rs"]
mod window_tick_latch;

#[path = "🧩️extension-generation-and-preview-retention/🦀️.rs"]
mod extension_generation_and_preview_retention;

#[path = "🔭️preview-tessellation-cancellation-and-status/🦀️.rs"]
mod preview_tessellation_cancellation_and_status;

#[path = "🏁️close-and-retirement/🦀️.rs"]
mod close_and_retirement;
