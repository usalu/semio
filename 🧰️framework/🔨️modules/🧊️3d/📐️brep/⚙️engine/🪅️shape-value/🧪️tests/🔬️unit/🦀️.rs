use super::*;
use semio_framework_3d_brep_representation::arena::{ArenaId, SolidId};

/// 🩺️ A value whose root does not resolve in its own body is refused with the inconsistent-root contract, before any session sees it.
#[test]
fn a_value_whose_root_does_not_resolve_in_its_own_body_is_refused() {
    let shape = ShapeValue { root: ShapeRoot::Solid(SolidId::from_raw(3, 0)), body: Body::new() };
    assert_eq!(shape.check(), Err(BrepError::InvalidInput("shape value is inconsistent: root".into())));
}
