//! 🌱️ Shared Space authority; its package has no concrete child artifact dependency.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[path = "../../🕰️time/🦀️.rs"]
pub mod time;
#[path = "../../📄️documents/🦀️.rs"]
pub mod documents;
pub use documents::{prepare_space_document_sources, SpaceDocumentCodec, SpaceDocumentFormat, SpaceDocumentSource};
#[path = "../../🦀️.rs"]
mod runtime;
pub use runtime::*;
