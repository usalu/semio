//! 🧱️ Flow document model: widgets, synapses, GUI projections, typed retirement and the component text codecs every Flow document representation shares.
pub use neural_engine as neural;
pub use protocol::value::ordered::{OrderedMap, OrderedSet};
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_pack;
pub use semio_framework_os_kernel::os_spr;
pub use semio_framework_os_kernel::os_store;
#[path = "../🧬️schema/📸️snapshot/🦀️.rs"]
pub mod artifact;
pub use artifact::*;
#[path = "../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
pub mod component_dsl;
#[path = "../🎚️parameter/📨️intent/🦀️.rs"]
pub mod graph_parameter;
#[path = "../🧵️retained/🦀️.rs"]
pub mod retained;

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
