#[allow(unused_imports)]
use crate::op::NoteMutation;
#[allow(unused_imports)]
use semio_s_artifact_note_note_schema::NoteSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::note::commands::set_grid_subdivisions::*;
use crate::editor::note::commands::set_grid_opacity;
use crate::editor::note::unit_tests::context::{dispatch, note_app};
use crate::editor::note::command_enum::NoteCommand;

#[semio_framework_async_macros::async_test]
async fn set_grid_subdivisions_and_opacity_clamp() {
    let mut app = note_app().await;
    dispatch(&mut app, NoteCommand::SetGridSubdivisions(SetGridSubdivisions { value: 40.0 })).await;
    assert_eq!(app.snapshot().expect("snapshot").grid_subdivisions, Some(16.0));

    dispatch(&mut app, NoteCommand::SetGridOpacity(set_grid_opacity::SetGridOpacity { value: 5.0 })).await;
    assert_eq!(app.snapshot().expect("snapshot").grid_opacity, Some(1.0));
}
