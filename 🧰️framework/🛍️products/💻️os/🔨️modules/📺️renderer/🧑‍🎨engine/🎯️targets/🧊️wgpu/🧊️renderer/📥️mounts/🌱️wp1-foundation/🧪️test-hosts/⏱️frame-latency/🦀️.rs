//! ⏱️ Test host of the renderer hub. The frame latency diagnostics law names the frame core types (`AppPresentPhase`, `FrameTransaction…`), so it cannot live in `host-base`.
//! The host re-exports the module under test, so the law keeps its `use super::*;` and its module path while the law's file stays with its element.

pub(crate) use crate::frame_latency::*;

#[path = "../../../../../../../🧪️tests/⏱️frame-latency-diagnostics/🦀️.rs"]
mod tests;
