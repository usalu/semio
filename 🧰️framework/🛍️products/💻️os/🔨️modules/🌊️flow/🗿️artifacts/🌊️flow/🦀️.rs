//! 🌊️ Independently composable Flow document, mutation, and ownership contracts.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
pub use neural_engine as neural;
pub use protocol::value::ordered::{OrderedMap, OrderedSet};
pub use semio_framework_os_kernel::{os_dsl, os_pack, os_spr, os_store};
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

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod package_tests;
