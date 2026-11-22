//! 🎥️ Video render host: the native `media.video-render` job — scene programs painted through the wgpu rasterizer, encoded by the framework's AVC encoder, muxed as MP4.

#[path = "../../../🧱️elements/🎥️VideoRenderHost/🎯️targets/🧊️wgpu/🦀️.rs"]
mod video_render_host;
pub use video_render_host::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
