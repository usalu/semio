
use super::*;
use crate::standards::v1::subsets::brep::schema::diff::euler::make_vertex;
use crate::standards::v1::subsets::brep::schema::diff::primitives::make_box;

/// ♻️ A closed box plus one orphan vertex: `compact` must free exactly the orphan and nothing
/// the box's solid transitively reaches.
#[semio_framework_async_macros::async_test]
async fn compact_frees_exactly_the_unreachable_orphan() {
    let mut body = Body::new();
    let mut rec = history::OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let orphan = make_vertex(&mut body, Pnt3::new(9.0, 9.0, 9.0), Tol::DEFAULT, &mut rec);
    let before = body.entity_counts();
    assert_eq!(before.vertices, 9, "8 box corners + 1 orphan");

    let keep = body.reachable_from(&[EntityRef::Solid(solid)]);
    assert!(!keep.vertices.contains(&orphan), "the orphan is not reachable from the solid");
    let freed = body.compact(&keep);
    assert_eq!(freed.freed_vertices, 1);
    assert_eq!(freed.freed_edges, 0);
    assert_eq!(freed.freed_faces, 0);

    let after = body.entity_counts();
    assert_eq!(after.vertices, 8);
    assert!(!body.vertices.is_live(orphan));
    assert!(body.solids.is_live(solid), "the kept solid's id must stay valid — no index remap");
}

/// ♻️ A stale id from before a `compact` must be rejected by the generation check, never alias
/// whatever entity ends up reusing the freed slot.
#[semio_framework_async_macros::async_test]
async fn compact_leaves_stale_ids_rejected_by_generation() {
    let mut body = Body::new();
    let mut rec = history::OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let orphan = make_vertex(&mut body, Pnt3::new(9.0, 9.0, 9.0), Tol::DEFAULT, &mut rec);

    let keep = body.reachable_from(&[EntityRef::Solid(solid)]);
    body.compact(&keep);
    assert_eq!(body.vertices.get(orphan), None, "stale id must not resolve after compaction");

    let reused = make_vertex(&mut body, Pnt3::new(1.0, 2.0, 3.0), Tol::DEFAULT, &mut rec);
    assert_eq!(reused.raw_index(), orphan.raw_index(), "LIFO free list reuses the freed slot");
    assert_ne!(reused.raw_generation(), orphan.raw_generation());
    assert_eq!(body.vertices.get(orphan), None, "the old id still must not alias the new vertex");
}

/// ♻️ `merge` must leave `self`'s own ids/labels untouched (existing handles stay resolvable)
/// while grafting `other`'s entities in with non-colliding, offset labels.
#[semio_framework_async_macros::async_test]
async fn merge_preserves_self_and_offsets_others_labels() {
    let mut a = Body::new();
    let mut rec = history::OpRecorder::new();
    let a_solid = make_box(&mut a, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let a_label_before = a.solids.get(a_solid).unwrap().label;
    let a_next_before = a.labels.next();

    let mut b = Body::new();
    let mut rec_b = history::OpRecorder::new();
    let b_solid = make_box(&mut b, 2.0, 2.0, 2.0, &mut rec_b).unwrap();
    let b_label = b.solids.get(b_solid).unwrap().label;

    let map = a.merge(&b);

    assert!(a.solids.is_live(a_solid), "self's own solid id must stay valid after merge");
    assert_eq!(a.solids.get(a_solid).unwrap().label, a_label_before, "self's own labels must not shift");

    let merged_solid = map.solids[&b_solid];
    assert!(a.solids.is_live(merged_solid));
    let merged_label = a.solids.get(merged_solid).unwrap().label;
    assert_eq!(merged_label.0, b_label.0 + a_next_before, "other's labels are offset above self's high-water mark");
    assert!(a.labels.next() > merged_label.0, "self's label source now carries the merged high-water mark forward");

    let keep = a.reachable_from(&[EntityRef::Solid(merged_solid)]);
    let mesh_faces = a.solid_faces(merged_solid);
    assert_eq!(mesh_faces.len(), 6, "the merged box keeps all 6 faces");
    assert_eq!(keep.faces.len(), 6);
}
