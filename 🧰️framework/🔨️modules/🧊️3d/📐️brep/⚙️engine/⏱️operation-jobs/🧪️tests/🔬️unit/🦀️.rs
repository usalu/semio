use super::super::*;
use super::*;
use std::collections::BTreeSet;

/// 📸️ Everything observable about a session: the whole body, the live handle set and the pending mutations. Two equal
/// snapshots mean the session is byte-identical.
fn snapshot(kernel: &Brep) -> String {
    format!("{:?}|{:?}|{}", kernel.body, kernel.live.keys().collect::<Vec<_>>(), kernel.pending_mutations)
}


/// 🔒️ `Body::extract` copies exactly what the roots reach with every label kept, and `Body::absorb` gives
/// each surviving copy a fresh label while leaving everything already in the body untouched.
#[test]
fn extract_and_absorb_isolate_a_working_copy() {
    let mut kernel = Brep::new();
    let kept = kernel.box_prim_sync(1.0, 2.0, 3.0).unwrap();
    let other = kernel.box_prim_sync(4.0, 4.0, 4.0).unwrap();
    let solid = kernel.solid_id(&kept).unwrap();
    let before = snapshot(&kernel);
    let base = kernel.body.labels.next();
    let (scratch, map) = kernel.body.extract(&[EntityRef::Solid(solid)]);
    assert_eq!(snapshot(&kernel), before, "extracting reads the body only");
    assert_eq!((scratch.solids.len(), scratch.faces.len(), scratch.edges.len(), scratch.vertices.len()), (1, 6, 12, 8), "only what the box reaches is copied");
    assert_eq!(scratch.labels.next(), base, "the copy's label counter continues at the body's");
    for (old, new) in &map.faces {
        assert_eq!(scratch.faces.get(*new).unwrap().label, kernel.body.faces.get(*old).unwrap().label, "labels are preserved in the copy");
    }
    let other_solid = kernel.solid_id(&other).unwrap();
    let other_label = kernel.body.solids.get(other_solid).unwrap().label;
    let absorbed = kernel.body.absorb(&scratch, &[EntityRef::Solid(map.solids[&solid])], base);
    let copy = absorbed.solids[&map.solids[&solid]];
    assert_ne!(kernel.body.solids.get(copy).unwrap().label, kernel.body.solids.get(solid).unwrap().label, "a surviving copy never shares its source's label");
    assert_eq!(kernel.body.solids.get(other_solid).unwrap().label, other_label, "existing entities are untouched");
    let body = &kernel.body;
    let labels: Vec<u64> = body.vertices.iter().map(|(_, v)| v.label.0).chain(body.edges.iter().map(|(_, e)| e.label.0)).chain(body.faces.iter().map(|(_, f)| f.label.0)).chain(body.shells.iter().map(|(_, s)| s.label.0)).chain(body.solids.iter().map(|(_, s)| s.label.0)).collect();
    assert_eq!(labels.iter().collect::<BTreeSet<_>>().len(), labels.len(), "no two live entities share a label");
}
