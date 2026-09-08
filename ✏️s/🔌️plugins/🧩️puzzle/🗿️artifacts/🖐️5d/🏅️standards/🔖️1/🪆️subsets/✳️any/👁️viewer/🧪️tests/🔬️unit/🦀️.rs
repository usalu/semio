
use super::*;

#[test]
fn create_puzzle5d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_puzzle5d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, PUZZLE5D_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Puzzle5dViewer as ArtifactViewer>::DIALECT, PUZZLE5D_DIALECT);
}
