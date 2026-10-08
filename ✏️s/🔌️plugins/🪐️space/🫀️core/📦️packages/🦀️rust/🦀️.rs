//! 🌱️ Shared Space authority; its package has no concrete child artifact dependency.
#[cfg(feature = "component-app-assembly")]
extern crate semio_framework_os_kernel as protocol;
#[cfg(feature = "component-app-assembly")]
extern crate semio_framework_os_kernel as store;
#[path = "../../../🕰️time/🦀️.rs"]
pub mod time;
#[path = "../../../📄️documents/🦀️.rs"]
pub mod documents;
pub use documents::{prepare_space_document_sources, SpaceDocumentCodec, SpaceDocumentFormat, SpaceDocumentSource};
#[cfg(feature = "component-app-assembly")]
#[path = "../../../🦀️.rs"]
mod runtime;
#[cfg(feature = "component-app-assembly")]
pub use runtime::*;
