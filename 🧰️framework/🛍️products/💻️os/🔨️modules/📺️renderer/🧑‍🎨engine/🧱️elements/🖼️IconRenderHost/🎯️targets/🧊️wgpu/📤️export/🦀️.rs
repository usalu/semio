//! 📤️ Request-owned icon export operations.
#[path = "📷️png/🦀️.rs"]
mod png;
#[path = "🧊️gpu/🦀️.rs"]
mod gpu;
#[path = "🎬️scene/🦀️.rs"]
mod scene;
#[path = "📐️svg/🦀️.rs"]
mod svg;
pub use gpu::{IconGpuPngExport, IconGpuPngRejected};
pub use scene::{IconExportFormat, IconExportPreparedScene, IconExportScenePreparation, IconExportSceneRejected};
pub use scene::close_prepared;
pub use svg::{IconSvgExport, IconSvgRejected};
pub use semio_framework_os_renderer_wgpu_asset_authority::icon_export_asset as asset;

#[derive(Debug)]
pub enum IconExportFault {
    Message(String),
    World(infinite_world::world::WorldDynamicFault),
    Original(semio_framework_job::OriginalTurnFailure),
}

impl From<semio_framework_job::OriginalTurnFailure> for IconExportFault {
    fn from(value: semio_framework_job::OriginalTurnFailure) -> Self { Self::Original(value) }
}

impl From<String> for IconExportFault {
    fn from(value: String) -> Self { Self::Message(value) }
}

impl From<&str> for IconExportFault {
    fn from(value: &str) -> Self { Self::Message(value.to_owned()) }
}

impl std::fmt::Display for IconExportFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::Message(value) => formatter.write_str(value), Self::World(fault) => write!(formatter, "{fault:?}"), Self::Original(fault) => formatter.write_str(fault.primary.message()) }
    }
}
