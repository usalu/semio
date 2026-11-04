//! 🧪️ The scene frames the real bounds, drops invalid faces and caps markers.

use super::*;

#[test]
fn the_frame_is_the_bounding_box_centre_and_diagonal() {
    let (centre, diagonal) = mesh_scene_frame(&[[0.0, 0.0, 0.0], [2.0, 4.0, 4.0]], &[[1.0, 1.0, 1.0]]);
    assert_eq!(centre, [1.0, 2.0, 2.0]);
    assert!((diagonal - 6.0).abs() < 1e-12);
    assert_eq!(mesh_scene_frame(&[], &[]), ([0.0; 3], 1.0));
    assert_eq!(mesh_scene_frame(&[[3.0, 3.0, 3.0]], &[]).1, 1.0);
}

#[test]
fn a_scene_renders_for_geometry_points_and_nothing() {
    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    assert!(mesh_scene("tri", &positions, &[0, 1, 2, 0, 1, 9], &[]).is_ok());
    assert!(mesh_scene("cloud", &[], &[], &vec![[0.5, 0.5, 0.5]; MESH_SCENE_MAXIMUM_MARKERS * 3]).is_ok());
    assert!(mesh_scene("empty", &[], &[], &[]).is_ok());
}
