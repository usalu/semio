
use super::*;

/// 🧪️ The device-touching branch (`upload` with non-empty bytes, growth policy, the `contents()`
/// copy) needs a live Metal device — exercised from `🍎️backend.rs`'s gated test module instead of
/// here, since a `&Device` cannot be conjured without one. This only checks construction.
#[test]
fn a_fresh_grow_buffer_starts_with_no_backing_buffer() {
    let buffer = GrowBuffer::default();
    assert!(buffer.buffer().is_none());
    let frame = FrameBuffers::default();
    assert!(frame.quad_instances.buffer().is_none());
    assert!(frame.world_instances.buffer().is_none());
}
