
use super::*;
use crate::editor::remodeling::RemodelingCommand;
use crate::editor::remodeling::commands::add_gcp::AddGcp;
use crate::editor::remodeling::testkit::{app, dispatch, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_calibration_panel_lists_added_ground_control_points() {
    let mut app = app().await;
    dispatch(&mut app, RemodelingCommand::AddGcp(AddGcp { name: "Corner".into(), world_x: 1.0, world_y: 2.0, world_z: 3.0 })).await;
    assert!(render_body(&mut app, REMODELING_PLAY_BODY_CALIBRATION).await.contains("Corner"));
}
