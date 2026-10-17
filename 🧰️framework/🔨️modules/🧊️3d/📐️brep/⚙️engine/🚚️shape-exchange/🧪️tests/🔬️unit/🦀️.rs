//! 🔬️ Shape exchange: 1 test covering shape value import refuses dangling references without mutation.
use super::*;
use semio_framework_3d_brep_representation::arena::{ArenaId, Curve3Id, SolidId};


/// 🛡️ A decoded value with dangling references is refused instead of panicking, and the session stays untouched.
#[test]
fn shape_value_import_refuses_dangling_references_without_mutation() {
    let mut source = Brep::new();
    let solid = source.box_prim_sync(1.0, 2.0, 3.0).unwrap();
    let shape = source.export_shape(&solid).unwrap();
    let mut missing_root = shape.clone();
    missing_root.root = ShapeRoot::Solid(SolidId::from_raw(99, 0));
    let mut missing_curve = shape.clone();
    let edge = missing_curve.body.edges.ids().next().unwrap();
    missing_curve.body.edges.get_mut(edge).unwrap().curve = Curve3Id::from_raw(77, 0);
    let mut stale_labels = shape.clone();
    stale_labels.body.labels = LabelSource::from_next(0);
    let mut target = Brep::new();
    let before = (target.body.clone(), target.live.len());
    for (name, broken) in [("root", missing_root), ("curve", missing_curve), ("labels", stale_labels)] {
        assert!(matches!(target.import_shape(&broken), Err(BrepError::InvalidInput(_))), "{name}");
        assert!(matches!(broken.tessellate_job(0.1), Err(BrepError::InvalidInput(_))), "{name}");
    }
    assert_eq!((target.body.clone(), target.live.len()), before);
}
