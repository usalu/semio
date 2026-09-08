use super::*;

#[test]
fn create_puzzle2d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_puzzle2d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, PUZZLE2D_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Puzzle2dViewer as ArtifactViewer>::DIALECT, PUZZLE2D_DIALECT);
}

#[test]
fn viewer_command_default_is_noop() {
    assert_eq!(Puzzle2dViewCommand::default(), Puzzle2dViewCommand::Noop);
}
