use super::*;
use crate::editor::note::commands::set_grid_opacity;
use crate::editor::note::unit_tests::context::{dispatch, note_app};
use crate::editor::note::NoteCommand;

#[semio_framework_async_macros::async_test]
async fn set_grid_subdivisions_and_opacity_clamp() {
    let mut app = note_app().await;
    dispatch(&mut app, NoteCommand::SetGridSubdivisions(SetGridSubdivisions { value: 40.0 })).await;
    assert_eq!(app.snapshot().expect("snapshot").grid_subdivisions, Some(16.0));

    dispatch(&mut app, NoteCommand::SetGridOpacity(set_grid_opacity::SetGridOpacity { value: 5.0 })).await;
    assert_eq!(app.snapshot().expect("snapshot").grid_opacity, Some(1.0));
}
