//! 🎛️ Raster viewer command and manifest: the one view command and the viewer app definition.

use super::*;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*` carries
/// them would be pure ceremony for a surface that never dispatches anything through `handle`. Derives
/// `Default` (`#[default]` on the sole variant) — required by `artifact_app_laws::assert_viewer_never_mutates`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, semio_framework_value::RetireOwned)]
pub enum RasterViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for RasterViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(RasterViewCommand::Noop)
    }
}

//#region 🔖️Manifest
pub fn create_raster_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(RASTER_DIALECT)
        .document(["semio", "raster"])
        .icon_id("raster")
        .mode_def(view::definition())
        .default_mode_id(view::RASTER_VIEW_MODE_VIEW)
        .window_kind_def(composite::definition())
        .window_kind_def(navigator::definition())
        .default_layout(view::layout())
        .build_definition()
}
