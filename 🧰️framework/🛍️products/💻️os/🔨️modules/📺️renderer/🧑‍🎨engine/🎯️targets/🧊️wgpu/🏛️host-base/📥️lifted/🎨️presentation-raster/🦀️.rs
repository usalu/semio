//! 🎨️ Runtime host waker: the platform wake callback type of the runtime mailbox, lifted for the `host-base` crate of the renderer partition (plan section 5.2).
use super::*;

#[cfg(not(target_arch = "wasm32"))]
pub type RuntimeHostWaker = Arc<dyn Fn() + Send + Sync>;

#[cfg(target_arch = "wasm32")]
pub type RuntimeHostWaker = std::rc::Rc<dyn Fn()>;
