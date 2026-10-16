//! 🗃️ Flow operator catalogue support: the extension manifest schema, the process-wide operator registry with its catalogue projections, and the channel-eval bridge.

use semio_framework_artifact_flow_flow::artifact;

#[path = "../📔️registry/🦀️.rs"]
pub mod registry;
pub use registry::*;

#[path = "../🗂️catalogue/🦀️.rs"]
pub mod catalogue;
pub use catalogue::*;

#[path = "../🌉️bridge/🦀️.rs"]
pub mod bridge;

#[path = "."]
pub mod extensions {
    #[path = "../🧩️extensions/🧬️schema/🦀️.rs"]
    pub mod schema;
}
