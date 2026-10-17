use crate::NOTE_DIALECT;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::Viewer;
#[allow(unused_imports)]
use crate::op::NoteMutation;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::text::snapshot::empty_note_snapshot;
#[allow(unused_imports)]
use crate::viewer::note::modes::view;
#[allow(unused_imports)]
use crate::viewer::note::modes::view::windows::composite;
#[allow(unused_imports)]
use crate::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresence;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresenceMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransient;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_framework_plugin::ViewEmit;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
use crate::viewer::note::*;

#[semio_framework_async_macros::async_test]
async fn create_note_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_note_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, NOTE_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<NoteViewer as ArtifactViewer>::DIALECT, NOTE_DIALECT);
}
