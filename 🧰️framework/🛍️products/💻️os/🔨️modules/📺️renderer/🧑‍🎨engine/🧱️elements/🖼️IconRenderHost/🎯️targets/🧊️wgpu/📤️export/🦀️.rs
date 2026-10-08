//! 📤️ Request-owned icon export operations.
#[path = "📷️png/🦀️.rs"]
mod png;
#[path = "🧊️gpu/🦀️.rs"]
mod gpu;
#[path = "🎬️scene/🦀️.rs"]
mod scene;
#[path = "📐️svg/🦀️.rs"]
mod svg;
pub(crate) use gpu::{IconGpuPngExport, IconGpuPngRejected};
pub(crate) use scene::{IconExportFormat, IconExportPreparedScene, IconExportScenePreparation, IconExportSceneRejected};
pub(crate) use svg::{IconSvgExport, IconSvgRejected};
#[path = "📥️asset/🦀️.rs"]
pub(crate) mod asset;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum IconExportFault {
    Message(String),
    World(infinite_world::world::WorldDynamicFault),
}

impl From<String> for IconExportFault {
    fn from(value: String) -> Self { Self::Message(value) }
}

impl From<&str> for IconExportFault {
    fn from(value: &str) -> Self { Self::Message(value.to_owned()) }
}

impl std::fmt::Display for IconExportFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::Message(value) => formatter.write_str(value), Self::World(fault) => write!(formatter, "{fault:?}") }
    }
}
