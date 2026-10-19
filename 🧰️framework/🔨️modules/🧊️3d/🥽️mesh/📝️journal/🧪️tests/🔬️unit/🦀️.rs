use super::*;
use semio_framework_3d_mesh_document::{MirrorAxis, WeldMode};
use semio_framework_3d::mesh::MeshModelingOps;

fn boxed() -> HalfedgeMesh {
    HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box primitive")
}

/// 🧊️ The unit box moved to x in 0..1, so it lies on one side of the YZ plane and touches it.
fn half_space_box() -> HalfedgeMesh {
    let base = boxed();
    let positions: Vec<[f32; 3]> = base.positions().into_iter().map(|position| [position[0] + 0.5, position[1], position[2]]).collect();
    HalfedgeMesh::from_faces(&positions, &base.polygons()).expect("shifted box")
}

/// ▦ Two coplanar quads sharing one edge.
fn two_quads() -> HalfedgeMesh {
    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [2.0, 1.0, 0.0]];
    HalfedgeMesh::from_faces(&positions, &[vec![0, 1, 4, 3], vec![1, 2, 5, 4]]).expect("two quads")
}

/// ▶️ The mesh the journal's rows write over `base`: arenas cut to their journaled length, journaled rows written, seams flipped.
fn replayed(base: &HalfedgeMesh, journal: &MeshJournal) -> HalfedgeMesh {
    fn written<T: Clone>(mut rows: Vec<T>, journal: &ArenaJournal<T>) -> Vec<T> {
        if let Some(len) = journal.len {
            rows.truncate(len);
        }
        for (index, row) in &journal.rows {
            let index = *index as usize;
            if index < rows.len() {
                rows[index] = row.clone();
            } else {
                assert_eq!(index, rows.len(), "added rows extend the arena without a gap");
                rows.push(row.clone());
            }
        }
        rows
    }
    let (vertices, halfedges, faces, seams, attributes, materials, textures) = base.clone().into_owned_parts();
    let mut marked: BTreeSet<u32> = seams.iter().copied().collect();
    for seam in &journal.seams_removed {
        assert!(marked.remove(seam), "a removed seam was marked");
    }
    marked.extend(journal.seams_added.iter().copied());
    let seams = marked.into_iter().collect::<Vec<u32>>().into_iter().fold(semio_framework_3d_mesh_document::HistoryFoldSet::new(), |mut set, seam| {
        set.insert(seam);
        set
    });
    let attributes: Vec<(String, MeshAttribute)> = written(attributes.into_iter().collect(), &journal.attributes);
    let materials: Vec<(String, DslValue)> = written(materials.into_iter().collect(), &journal.materials);
    let textures: Vec<(String, MeshTexture)> = written(textures.into_iter().collect(), &journal.textures);
    HalfedgeMesh::from_owned_parts(written(vertices, &journal.vertices), written(halfedges, &journal.halfedges), written(faces, &journal.faces), seams, attributes.into_iter().collect(), materials.into_iter().collect(), textures.into_iter().collect())
}

fn assert_same_mesh(left: &HalfedgeMesh, right: &HalfedgeMesh) {
    let (lv, lh, lf, ls, ..) = left.clone().into_owned_parts();
    let (rv, rh, rf, rs, ..) = right.clone().into_owned_parts();
    assert_eq!(lv.len(), rv.len());
    assert!(lv.iter().zip(&rv).all(|(a, b)| same_vertex(a, b)), "vertex rows");
    assert_eq!(lh.len(), rh.len());
    assert!(lh.iter().zip(&rh).all(|(a, b)| same_halfedge(a, b)), "halfedge rows");
    assert_eq!(lf.len(), rf.len());
    assert!(lf.iter().zip(&rf).all(|(a, b)| same_face(a, b)), "face rows");
    assert_eq!(ls.iter().copied().collect::<Vec<u32>>(), rs.iter().copied().collect::<Vec<u32>>(), "seams");
}

fn canonical(mesh: &HalfedgeMesh) -> Vec<Vec<[u32; 3]>> {
    mesh.surface_signature()
}

fn assert_valid(mesh: &HalfedgeMesh, label: &str) {
    assert_eq!(mesh.structure_violation(), None, "{label}: the rows hold together");
}

type Record = fn(&mut MeshOverlay<'_>) -> MeshResult<()>;
type Direct = fn(&mut HalfedgeMesh) -> MeshResult<()>;

/// 👁️ A fresh overlay answers reads from the base and has journaled nothing.
#[test]
fn a_fresh_overlay_reads_the_base_and_journals_nothing() {
    let base = boxed();
    let overlay = MeshOverlay::new(&base);
    assert!(overlay.journal().is_empty());
    assert!(overlay.vertex(VertexId(0)).is_some_and(|row| same_vertex(row, &base.vertices[0])));
    assert!(overlay.vertex(VertexId(base.vertices.len() as u32)).is_none());
}

/// ✍️ A written row shadows the base row and the last write of a key wins; the length cuts and extends.
#[test]
fn writes_shadow_the_base_and_the_last_write_of_a_key_wins() {
    let base = boxed();
    let mut overlay = MeshOverlay::new(&base);
    let mut moved = base.vertices[3].clone();
    moved.position[0] += 1.0;
    overlay.write_vertex(3, moved.clone());
    assert!(overlay.vertex(VertexId(3)).is_some_and(|row| same_vertex(row, &moved)));
    let mut again = moved.clone();
    again.position[1] += 2.0;
    overlay.write_vertex(3, again.clone());
    assert_eq!(overlay.journal().vertices.rows.len(), 1, "one key, last write wins");
    assert!(overlay.vertex(VertexId(3)).is_some_and(|row| same_vertex(row, &again)));
    overlay.write_vertex(base.vertices.len() as u32, base.vertices[0].clone());
    overlay.resize_vertices(base.vertices.len() + 1);
    assert_eq!(overlay.journal().vertices.len, Some(base.vertices.len() + 1));
    assert!(overlay.vertex(VertexId(base.vertices.len() as u32)).is_some());
    overlay.resize_vertices(base.vertices.len());
    assert_eq!(overlay.journal().vertices.rows.len(), 1, "cutting the added row away drops it");
    assert_eq!(overlay.journal().vertices.len, None);
}

/// 🪡️ Seam edits are local row writes: marked pairs are journaled once, clearing them again takes them back.
#[test]
fn seam_marks_are_journaled_as_pairs_and_clear_again() {
    let base = boxed();
    let mut overlay = MeshOverlay::new(&base);
    overlay.mark_uv_seam(&[EdgeId(0), EdgeId(4)], true);
    let journal = overlay.journal().clone();
    assert!(!journal.seams_added.is_empty() && journal.seams_removed.is_empty());
    assert!(journal.vertices.is_empty() && journal.halfedges.is_empty() && journal.faces.is_empty(), "a seam edit writes no arena row");
    let after = replayed(&base, &journal);
    let mut direct = base.clone();
    direct.mark_uv_seam(&[EdgeId(0), EdgeId(4)], true);
    assert_same_mesh(&after, &direct);
    overlay.mark_uv_seam(&[EdgeId(0), EdgeId(4)], false);
    assert!(overlay.journal().is_empty(), "clearing the pair again leaves no seam row");
}

/// 🏗️ Every topology gesture's journal replays to a valid half-edge structure that shows exactly the surface the kernel produces
/// (compared as numbering-free position loops), keeps the kernel's edge pairing, and leaves the base untouched.
#[test]
fn topology_gestures_journal_the_rows_that_give_the_kernels_surface() {
    let cases: Vec<(&str, fn() -> HalfedgeMesh, Record, Direct)> = vec![
        ("extrude box", boxed, |o| o.extrude_faces(&[FaceId(2)], 0.5), |m| m.extrude_faces(&[FaceId(2)], 0.5)),
        ("extrude two box faces", boxed, |o| o.extrude_faces(&[FaceId(1), FaceId(2)], 0.4), |m| m.extrude_faces(&[FaceId(1), FaceId(2)], 0.4)),
        ("extrude open plane", || HalfedgeMesh::plane_prim(2.0, 2.0).expect("plane"), |o| o.extrude_faces(&[FaceId(0)], 0.3), |m| m.extrude_faces(&[FaceId(0)], 0.3)),
        ("inset box", boxed, |o| o.inset_faces(&[FaceId(2)], 0.1), |m| m.inset_faces(&[FaceId(2)], 0.1)),
        ("inset open plane", || HalfedgeMesh::plane_prim(2.0, 2.0).expect("plane"), |o| o.inset_faces(&[FaceId(0)], 0.2), |m| m.inset_faces(&[FaceId(0)], 0.2)),
        ("subdivide box face", boxed, |o| o.subdivide_faces(&[FaceId(2)]), |m| m.subdivide_faces(&[FaceId(2)])),
        ("subdivide two box faces", boxed, |o| o.subdivide_faces(&[FaceId(0), FaceId(3)]), |m| m.subdivide_faces(&[FaceId(0), FaceId(3)])),
        ("triangulate box", boxed, |o| o.triangulate(), |m| m.triangulate()),
        ("flip box faces", boxed, |o| o.flip_faces(&[FaceId(1), FaceId(3)]), |m| m.flip_faces(&[FaceId(1), FaceId(3)])),
        ("loop cut box", boxed, |o| o.loop_cut(&[EdgeId(0)], 1), |m| m.loop_cut(&[EdgeId(0)], 1)),
        ("loop cut box twice", boxed, |o| o.loop_cut(&[EdgeId(4)], 2), |m| m.loop_cut(&[EdgeId(4)], 2)),
        ("merge first", boxed, |o| { let (a, b) = o.base().edge_endpoints(EdgeId(0))?; o.merge_vertices(&[a, b], WeldMode::First, 0.0) }, |m| { let (a, b) = m.edge_endpoints(EdgeId(0))?; m.merge_vertices(&[a, b], WeldMode::First, 0.0) }),
        ("merge centre", boxed, |o| { let (a, b) = o.base().edge_endpoints(EdgeId(0))?; o.merge_vertices(&[a, b], WeldMode::Center, 0.0) }, |m| { let (a, b) = m.edge_endpoints(EdgeId(0))?; m.merge_vertices(&[a, b], WeldMode::Center, 0.0) }),
        ("merge by distance", boxed, |o| { let (a, b) = o.base().edge_endpoints(EdgeId(0))?; o.merge_vertices(&[a, b], WeldMode::ByDistance, 1.1) }, |m| { let (a, b) = m.edge_endpoints(EdgeId(0))?; m.merge_vertices(&[a, b], WeldMode::ByDistance, 1.1) }),
        ("mirror half space box", half_space_box, |o| o.mirror(MirrorAxis::X, 0.0), |m| m.mirror(MirrorAxis::X, 0.0)),
        ("bevel box edge", boxed, |o| o.bevel_edges(&[EdgeId(0)], 0.1, 1), |m| m.bevel_edges(&[EdgeId(0)], 0.1, 1)),
        ("bevel box edge in steps", boxed, |o| o.bevel_edges(&[EdgeId(0)], 0.1, 3), |m| m.bevel_edges(&[EdgeId(0)], 0.1, 3)),
        ("decimate sphere", || HalfedgeMesh::ico_sphere_prim(1.0, 1).expect("sphere"), |o| o.decimate(0.5), |m| m.decimate(0.5)),
        ("dissolve box edge", boxed, |o| o.dissolve_edges(&[EdgeId(0)]), |m| m.dissolve_edges(&[EdgeId(0)])),
        ("dissolve shared edge", two_quads, |o| o.dissolve_edges(&[EdgeId(1)]), |m| m.dissolve_edges(&[EdgeId(1)])),
    ];
    let mut failures: Vec<String> = Vec::new();
    for (name, make, record, direct) in cases {
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let base = make();
            let before = base.clone();
            let mut overlay = MeshOverlay::new(&base);
            record(&mut overlay).unwrap_or_else(|error| panic!("{name}: {error:?}"));
            let journal = overlay.into_journal();
            assert!(!journal.is_empty(), "{name}: the gesture writes rows");
            let mut expected = base.clone();
            direct(&mut expected).unwrap_or_else(|error| panic!("{name} directly: {error:?}"));
            let after = replayed(&base, &journal);
            assert_valid(&after, name);
            assert_eq!(canonical(&after), canonical(&expected), "{name}: the surface is the kernel's");
            assert_eq!(after.edge_count(), expected.edge_count(), "{name}: the edge pairing is the kernel's");
            assert_eq!(after.vertex_count(), expected.vertex_count(), "{name}: the vertex count is the kernel's");
            assert_same_mesh(&base, &before);
        }));
        if outcome.is_err() {
            failures.push(name.to_string());
        }
    }
    assert!(failures.is_empty(), "gestures that differ from the kernel: {failures:?}");
}

/// 🧲️ Row-for-row gestures: snapping states the moved positions only, unwrapping states the kernel's uv of every corner.
#[test]
fn in_place_gestures_state_the_kernels_rows() {
    let base = boxed();
    let mut overlay = MeshOverlay::new(&base);
    overlay.snap_vertices_to_grid(&[VertexId(0), VertexId(1), VertexId(2)], 1.0).expect("snap");
    let journal = overlay.into_journal();
    assert_eq!(journal.vertices.rows.len(), 3, "three moved vertices, nothing else");
    assert!(journal.halfedges.is_empty() && journal.faces.is_empty());
    let mut expected = base.clone();
    expected.snap_vertices_to_grid(&[VertexId(0), VertexId(1), VertexId(2)], 1.0).expect("kernel snap");
    assert_eq!(replayed(&base, &journal).positions(), expected.positions());

    let mut overlay = MeshOverlay::new(&base);
    overlay.unwrap_uv().expect("unwrap");
    let journal = overlay.into_journal();
    assert!(journal.vertices.is_empty() && journal.faces.is_empty(), "a corner coordinate is a halfedge field");
    let mut expected = base.clone();
    expected.unwrap_uv().expect("kernel unwrap");
    assert_same_mesh(&replayed(&base, &journal), &expected);
}

/// ✂️ A gesture states only the rows it semantically writes: extruding one box face appends the four displaced vertices, the four side
/// faces and their halfedges, rewrites the cap face and takes the four released halfedges back, and touches no neighbour row.
#[test]
fn extruding_one_face_states_only_its_own_rows() {
    let base = boxed();
    let mut overlay = MeshOverlay::new(&base);
    overlay.extrude_faces(&[FaceId(2)], 0.5).expect("extrude");
    let journal = overlay.into_journal();
    let (vertices, halfedges, faces) = (base.vertices.len(), base.halfedges.len(), base.faces.len());
    assert_eq!(journal.vertices.len, Some(vertices + 4));
    assert!(journal.vertices.rows.keys().all(|index| *index as usize >= vertices), "no original vertex is rewritten");
    assert_eq!(journal.halfedges.len, Some(halfedges + 16));
    assert_eq!(journal.halfedges.rows.len(), 20, "16 appended halfedges and the 4 released ones taken back");
    assert_eq!(journal.faces.len, Some(faces + 4));
    assert_eq!(journal.faces.rows.len(), 5, "the cap face and the four side faces");
    let ring: BTreeSet<u32> = base.face_halfedge_ids(FaceId(2)).expect("ring").into_iter().collect();
    for (index, row) in &journal.halfedges.rows {
        if (*index as usize) < halfedges {
            assert!(ring.contains(index), "only the extruded face's own halfedges are rewritten, not {index}");
            assert!(!same_halfedge(row, &base.halfedges[*index as usize]), "a written row differs from the base row");
        }
    }
}

/// 🗑️ Deleting faces removes them, compacts the arenas by moving the tail rows into the holes and drops the orphaned vertices.
#[test]
fn deleting_faces_compacts_the_arenas() {
    let base = boxed();
    for doomed in [vec![FaceId(0)], vec![FaceId(2)], vec![FaceId(5)], vec![FaceId(1), FaceId(4)]] {
        let mut overlay = MeshOverlay::new(&base);
        overlay.delete_faces(&doomed).expect("delete");
        let after = replayed(&base, &overlay.into_journal());
        assert_valid(&after, "delete faces");
        let mut survivors: Vec<Vec<[u32; 3]>> = Vec::new();
        let positions = base.positions();
        for (face, corners) in base.polygons().into_iter().enumerate() {
            if !doomed.contains(&FaceId(face as u32)) {
                let points: Vec<[u32; 3]> = corners.iter().map(|vertex| positions[*vertex as usize].map(f32::to_bits)).collect();
                let start = (0..points.len()).min_by_key(|index| points[*index]).expect("corners");
                survivors.push((0..points.len()).map(|step| points[(start + step) % points.len()]).collect());
            }
        }
        survivors.sort();
        assert_eq!(canonical(&after), survivors);
    }
    let mut overlay = MeshOverlay::new(&base);
    overlay.delete_faces(&(0..6).map(FaceId).collect::<Vec<_>>()).expect("delete every face");
    let after = replayed(&base, &overlay.into_journal());
    assert_eq!((after.vertex_count(), after.halfedge_count(), after.face_count()), (0, 0, 0), "deleting every face leaves an empty mesh");
}

/// 🧱️ A second topology gesture on a touched overlay is refused: gestures state one edit each.
#[test]
fn a_topology_gesture_refuses_a_touched_overlay() {
    let base = boxed();
    let mut overlay = MeshOverlay::new(&base);
    overlay.mark_uv_seam(&[EdgeId(0)], true);
    assert!(overlay.extrude_faces(&[FaceId(2)], 0.5).is_err());
}

/// 🧱️ A gesture that renumbers corners refuses a mesh whose authored channels or uv seams address them.
#[test]
fn a_renumbering_gesture_refuses_channels_and_seams() {
    let mut seamed = boxed();
    seamed.mark_uv_seam(&[EdgeId(0)], true);
    assert!(MeshOverlay::new(&seamed).extrude_faces(&[FaceId(2)], 0.5).is_err());
    assert!(MeshOverlay::new(&seamed).unwrap_uv().is_ok(), "unwrapping reads the seams");
    let mut channelled = boxed();
    channelled
        .set_attribute("tag".into(), MeshAttribute { domain: semio_framework_3d_mesh_document::MeshAttributeDomain::Face, semantic: semio_framework_3d_mesh_document::MeshAttributeSemantic::Custom, interpolation: semio_framework_3d_mesh_document::MeshAttributeInterpolation::Constant, values: vec![DslValue::Null], indices: Some(vec![0; 6]) })
        .expect("a face channel");
    assert!(MeshOverlay::new(&channelled).triangulate().is_err());
}

/// 🛑️ Refusals match the kernel's: an empty selection, an unknown face and a duplicate face.
#[test]
fn gestures_refuse_what_the_kernel_refuses() {
    let base = boxed();
    assert!(matches!(MeshOverlay::new(&base).extrude_faces(&[], 0.5), Err(MeshKernelError::EmptySelection)));
    assert!(matches!(MeshOverlay::new(&base).extrude_faces(&[FaceId(99)], 0.5), Err(MeshKernelError::InvalidHandle)));
    assert!(MeshOverlay::new(&base).extrude_faces(&[FaceId(1), FaceId(1)], 0.5).is_err());
    assert!(MeshOverlay::new(&base).extrude_faces(&[FaceId(1)], f32::NAN).is_err());
    let open = HalfedgeMesh::plane_prim(1.0, 1.0).expect("plane");
    let boundary = (0..open.halfedge_count() as u32).find(|halfedge| open.halfedges[*halfedge as usize].twin.is_none()).expect("a boundary edge");
    assert!(MeshOverlay::new(&open).dissolve_edges(&[EdgeId(boundary)]).is_err(), "a boundary edge has no second face");
    assert!(MeshOverlay::new(&base).mirror(MirrorAxis::X, 0.0).is_err(), "a box centred on the plane has both sides");
}
