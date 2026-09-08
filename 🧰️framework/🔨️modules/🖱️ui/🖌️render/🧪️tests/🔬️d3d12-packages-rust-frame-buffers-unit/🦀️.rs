
use super::*;

/// 🧪️ The device-touching branches (`upload`/`begin_frame`/`allocate_pair`) need a live D3D12
/// device — exercised from `🪟️backend.rs`'s gated test module instead of here, since neither a
/// `&Device` nor a `D3D12_CPU_DESCRIPTOR_HANDLE` can be conjured without one. This only checks
/// construction.
#[test]
fn a_fresh_grow_buffer_starts_with_no_backing_buffer() {
    let buffer = GrowBuffer::default();
    assert!(buffer.buffer().is_none());
    assert!(buffer.gpu_address().is_none());
    let frame = FrameBuffers::default();
    assert!(frame.quad_instances.buffer().is_none());
    assert!(frame.world_instances.buffer().is_none());
}

#[test]
fn a_fresh_frame_descriptor_allocator_starts_with_no_heap() {
    let descriptors = FrameDescriptors::default();
    assert!(descriptors.heap().is_none());
}
