
use super::*;

#[test]
fn create_puzzle3d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_puzzle3d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, PUZZLE3D_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Puzzle3dViewer as ArtifactViewer>::DIALECT, PUZZLE3D_DIALECT);
}
