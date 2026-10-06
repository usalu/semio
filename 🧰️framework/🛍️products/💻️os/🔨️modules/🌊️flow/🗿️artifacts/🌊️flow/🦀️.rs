//! 🌊️ Independently composable Flow document, mutation, and ownership contracts.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
pub use neural_engine as neural;
pub use protocol::value::ordered::{OrderedMap, OrderedSet};
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_pack;
pub use semio_framework_os_kernel::os_spr;
pub use semio_framework_os_kernel::os_store;
use semio_framework_ui_locale::LocalizedLabel;
#[path = "🧬️schema/📸️snapshot/🦀️.rs"]
pub mod artifact;
pub use artifact::*;
#[path = "🎚️parameter/📨️intent/🦀️.rs"]
pub mod graph_parameter;
#[path = "🧵️retained/🦀️.rs"]
pub mod retained;
#[path = "🌿️vcs/🦀️.rs"]
pub mod vcs;
pub use vcs::*;
pub fn widget_id_for(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

#[path = "🧩️extensions/🦀️.rs"]
pub mod extensions;
#[path = "📔️registry/🦀️.rs"]
pub mod registry;
pub use registry::*;
#[path = "🗂️catalogue/🦀️.rs"]
pub mod catalogue;
pub use catalogue::*;

#[cfg(test)]
#[path = "🧪️tests/🌊️flow/🦀️.rs"]
mod package_tests;

#[cfg(test)]
#[path = "./🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
