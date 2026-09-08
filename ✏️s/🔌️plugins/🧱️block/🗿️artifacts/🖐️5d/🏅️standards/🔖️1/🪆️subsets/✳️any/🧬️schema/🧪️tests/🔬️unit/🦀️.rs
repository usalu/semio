
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_definition_matches_default() {
    assert_eq!(empty_block5d_snapshot(), Block5dSnapshot::default());
}

/// 📄️ The boot document is a real part kind with a mesh, so the board window has a label to show
/// and the World3d window has a `mesh_url` to render before the first command.
#[semio_framework_async_macros::async_test]
async fn default_definition_boots_on_the_forest_left_example() {
    let booted = default_block5d_snapshot();
    assert_ne!(booted, empty_block5d_snapshot());
    assert_eq!(booted.part_kind.id, "Hexagonal Cut Concrete Forest Left");
    assert_eq!(booted.part_kind.label, "Hexagonal Cut Concrete Forest Left");
    assert_eq!(booted.representations.first().and_then(|representation| representation.mesh_url.as_deref()), Some("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb"));
    assert_eq!(booted.grip_kinds.len(), 1);
    assert_eq!(booted.grips.len(), 1);
}
