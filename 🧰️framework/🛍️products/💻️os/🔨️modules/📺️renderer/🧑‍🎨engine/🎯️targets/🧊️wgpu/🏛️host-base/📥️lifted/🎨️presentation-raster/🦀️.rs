//! 🎨️ Runtime host waker: the platform wake callback type of the runtime mailbox, lifted for the `host-base` crate of the renderer partition (plan section 5.2).
use super::*;

#[cfg(not(target_arch = "wasm32"))]
pub type RuntimeHostWaker = Arc<dyn Fn() + Send + Sync>;

#[cfg(target_arch = "wasm32")]
pub type RuntimeHostWaker = std::rc::Rc<dyn Fn()>;

#[cfg(not(target_arch = "wasm32"))]
static RETAINED_IMAGE_RASTER_FRAME: AtomicBool = AtomicBool::new(false);

/// 🎨️ Keeps the native retained-image decode and upload steps eligible for the next rendered frame.
#[cfg(not(target_arch = "wasm32"))]
pub fn request_retained_image_raster_frame() {
    RETAINED_IMAGE_RASTER_FRAME.store(true, Ordering::Release);
}

/// 🖼️ Consumes the frame request published by a pending retained-image raster step.
#[cfg(not(target_arch = "wasm32"))]
pub fn take_retained_image_raster_frame_request() -> bool {
    RETAINED_IMAGE_RASTER_FRAME.swap(false, Ordering::AcqRel)
}
