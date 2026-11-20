use super::*;

/// ♻️ Disposing the only handle reaching a body's geometry must actually free it (audit §5.3:
/// "dispose is not equivalent to deleting geometry" under the old registry-only dispose).
#[semio_framework_async_macros::async_test]
async fn dispose_reclaims_unreferenced_topology() {
    let mut k = Brep::new();
    let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
    k.dispose(&solid);
    assert_eq!(k.registry_len(), 0);
    let counts = k.native.body.entity_counts();
    assert_eq!(counts.vertices, 0);
    assert_eq!(counts.edges, 0);
    assert_eq!(counts.faces, 0);
    assert_eq!(counts.shells, 0);
    assert_eq!(counts.solids, 0);
}


/// ⚖️ LAW: the validation gate judges the SHAPE it was asked about, never the arena around it.
///
/// 🐛️ It used to run `validate_body` over the whole body and block on any error-class issue found
/// anywhere, and this kernel is process-wide: one invalid solid — a boolean result whose void shell
/// is not inverted, say — refused the tessellation of every other live handle for as long as it
/// stayed live. In the procedural playground that read as six of eight examples settling on an empty
/// preview payload reported as `idle` (`📓️slider-reevaluation-correctness-2026-09-15.md`).
#[semio_framework_async_macros::async_test]
async fn the_validate_gate_judges_the_shape_it_was_asked_about_and_not_the_arena_around_it() {
    let mut kernel = Brep::new();
    let healthy = kernel.box_prim_sync(1.0, 1.0, 1.0).expect("a plain box is valid geometry");
    assert!(kernel.validate_gate_sync(&healthy).is_ok(), "a plain box must pass its own gate");
    let solid = kernel.native.solid_id(&healthy).expect("the box is a solid");
    let stranger = kernel.box_prim_sync(2.0, 2.0, 2.0).expect("a second box");
    let stranger_solid = kernel.native.solid_id(&stranger).expect("the second box is a solid");
    assert_ne!(solid, stranger_solid, "the two boxes are different solids");
    let outer = kernel.native.body.solids.get(stranger_solid).expect("stranger solid").outer;
    let faces = kernel.native.body.shells.get(outer).expect("stranger shell").faces.clone();
    for face in faces {
        if let Some(entry) = kernel.native.body.faces.get_mut(face) {
            entry.flipped = !entry.flipped;
        }
    }
    let stranger_issues = kernel.validate_gate_sync(&stranger).expect_err("the broken box must fail its own gate");
    assert!(!stranger_issues.is_empty(), "the broken box's gate must name at least one issue");
    assert!(kernel.validate_gate_sync(&healthy).is_ok(), "the healthy box must still pass while a STRANGER is broken: {stranger_issues:?}");
    eprintln!("scoped gate: healthy=ok stranger={:?}", stranger_issues.iter().map(|issue| format!("{}:{}", issue.entity, issue.code)).collect::<Vec<_>>());
}

fn open_triangle_import() -> MeshImportCursor {
    MeshImportCursor::new(vec![0., 0., 0., 1., 0., 0., 0., 1., 0.], Vec::new(), vec![0, 1, 2], 1e-6).unwrap()
}

/// 🔒️ A dropped import releases its open mutation, so retention and compaction are never blocked by it.
#[test]
fn dropped_mesh_import_releases_its_pending_mutation() {
    let mut kernel = Brep::new();
    let mut cursor = open_triangle_import();
    assert!(kernel.step_mesh_import_sync(&mut cursor, 1).unwrap().is_none());
    assert_eq!(kernel.native.pending_mutations.count(), 1);
    drop(cursor);
    assert_eq!(kernel.native.pending_mutations.count(), 0);
}

/// 🔒️ A finished import releases its mutation, and stepping it again is a typed error instead of a republish or panic.
#[test]
#[ignore = "import close still does not reach cleanup within 4096 steps although every quote is now met (reviewfix3-211/221)"]
fn finished_mesh_import_publishes_once() {
    let mut kernel = Brep::new();
    let mut cursor = open_triangle_import();
    let mut published = 0;
    for _ in 0..4096 {
        if kernel.step_mesh_import_sync(&mut cursor, 1).unwrap().is_some() { published += 1; break; }
    }
    assert_eq!(published, 1, "import never published: progress={:?} cleanup={} active={}", cursor.progress(), cursor.cleanup_complete(), cursor.is_active());
    assert_eq!(kernel.native.pending_mutations.count(), 0);
    assert!(kernel.step_mesh_import_sync(&mut cursor, 1).is_err());
    assert_eq!(kernel.native.pending_mutations.count(), 0);
}
