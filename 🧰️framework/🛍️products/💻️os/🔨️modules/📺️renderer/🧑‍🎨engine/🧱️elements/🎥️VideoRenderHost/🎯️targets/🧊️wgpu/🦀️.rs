//! 🎥️ `🎥️VideoRenderHost` for the wgpu renderer: the native host of the `media.video-render` capability (`Effect::VideoRenderExport`).
//! A guest has no canvas and no encoder, so a plugin that requested the capability hands the host a `VideoRenderProgram`; the host
//! paints every distinct scene once offscreen through the wgpu rasterizer, encodes the timeline with the framework's own AVC encoder
//! and muxes an MP4, as an event-sourced job with progress and cancellation.
//!
//! The React twin is `🎥️VideoRenderHost/🟦️.ts`; both fold the same kernel job ledger and answer the same `🧫️fixtures`.

#[path = "📖️labels/🦀️.rs"]
pub mod labels;
#[path = "🖼️program/🦀️.rs"]
pub mod program;
#[path = "🎞️render/🦀️.rs"]
pub mod render;
#[path = "🧵️jobs/🦀️.rs"]
pub mod jobs;

pub use jobs::*;
pub use labels::*;
pub use program::*;
pub use render::*;
