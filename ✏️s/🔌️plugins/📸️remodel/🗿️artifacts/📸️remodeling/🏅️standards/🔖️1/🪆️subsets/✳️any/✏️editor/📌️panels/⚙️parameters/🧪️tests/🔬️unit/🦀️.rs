use super::*;
use crate::editor::remodeling::commands::set_ingest_params::SetIngestParams;
use crate::editor::remodeling::unit_tests::context::{app, dispatch, render as render_body};
use crate::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn the_parameters_panel_reflects_a_live_param_edit() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::SetIngestParams(SetIngestParams { frame_sample_stride: 9, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 })).await;
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_PARAMETERS).await.contains("stride 9"));
}
