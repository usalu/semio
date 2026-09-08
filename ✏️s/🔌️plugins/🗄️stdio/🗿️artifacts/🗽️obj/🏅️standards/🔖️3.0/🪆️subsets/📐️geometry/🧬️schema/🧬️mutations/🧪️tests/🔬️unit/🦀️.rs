
use super::*;
use protocol::command::DiffAlgebra;

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = base_snapshot();
    for m in demo_mutation_cases() {
        let diff = m.diff(&base);
        let expected = diff.diff().apply(&base).expect("valid mutation diff");

        let mut via_apply = base.clone();
        let returned_diff = apply_obj_mutation(&mut via_apply, &m);

        assert_eq!(via_apply, expected, "apply_obj_mutation mismatch for {m:?}");
        assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = base_snapshot();
    for m in demo_mutation_cases() {
        let mut forward = base.clone();
        apply_obj_mutation(&mut forward, &m);
        for inv in m.inverse(&base) {
            apply_obj_mutation(&mut forward, &inv);
        }
        assert_eq!(forward, base, "mutation-level inverse round trip failed for {m:?}");

        let d = m.diff(&base);
        let mid = d.diff().apply(&base).expect("valid forward diff");
        let back = d.diff().inverse(&base).apply(&mid).expect("valid inverse diff");
        assert_eq!(back, base, "diff-level inverse round trip failed for {m:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️AbsorbLaw
#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    let base = base_snapshot();

    // 🧩 Insert(2) + Remove(0): the two-op sequence base → mid → after.
    let d1 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 2, vertex: ObjVertex { x: 8.0, y: 8.0, z: 8.0, w: None } }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let d2 = ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index: 0 }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Remove-before absorb mismatch");

    // 🧩 Insert(2,f) + Insert(2,g): both must survive.
    let d1 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 2, vertex: ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None } }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let d2 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 2, vertex: ObjVertex { x: 2.0, y: 0.0, z: 0.0, w: None } }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Insert-same-index absorb mismatch");
    assert_eq!(after.vertices.len(), base.vertices.len() + 2, "both inserts must survive");

    // 🧩 Add + SetField (SetVertex): patch into the added payload.
    let d1 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 1, vertex: ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None } }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let d2 = ObjMutation::SetVertex(set_vertex::SetVertex { index: 1, vertex: ObjVertex { x: 42.0, y: 0.0, z: 0.0, w: Some(1.0) } }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+SetField absorb mismatch");
    assert_eq!(after.vertices[1].x, 42.0);

    // 🧩 Modify + Remove: modifying then removing the same vertex collapses to a removal.
    let d1 = ObjMutation::SetVertex(set_vertex::SetVertex { index: 1, vertex: ObjVertex { x: 7.0, y: 0.0, z: 0.0, w: None } }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let d2 = ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index: 1 }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Modify+Remove absorb mismatch");

    // 🧩 Name-keyed: Add group + Rename-shaped remove-of-added annihilates the add.
    let d1 = ObjMutation::SetGroup(set_group::SetGroup { name: "Fresh".into(), faces: vec![0] }).diff(&base);
    let mid = d1.diff().apply(&base).expect("valid first diff");
    let d2 = ObjMutation::RemoveGroup(remove_group::RemoveGroup { name: "Fresh".into() }).diff(&mid);
    let after = d2.diff().apply(&mid).expect("valid second diff");
    let mut composed = d1.diff().clone();
    composed.absorb(d2.diff().clone());
    assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+Remove(name-keyed) absorb mismatch");
    assert_eq!(after.groups, base.groups, "add-then-remove of the same name must be a full no-op");

    // 🧩 Associativity over a triple.
    let base = base_snapshot();
    let d1 = ObjMutation::InsertVertex(insert_vertex::InsertVertex { index: 0, vertex: ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None } }).diff(&base);
    let s1 = d1.diff().apply(&base).expect("valid first diff");
    let d2 = ObjMutation::SetVertex(set_vertex::SetVertex { index: 0, vertex: ObjVertex { x: 2.0, y: 0.0, z: 0.0, w: Some(1.0) } }).diff(&s1);
    let s2 = d2.diff().apply(&s1).expect("valid second diff");
    let d3 = ObjMutation::RemoveVertex(remove_vertex::RemoveVertex { index: 2 }).diff(&s2);
    let s3 = d3.diff().apply(&s2).expect("valid third diff");

    let mut left = d1.diff().clone();
    left.absorb(d2.diff().clone());
    left.absorb(d3.diff().clone());

    let mut d23 = d2.diff().clone();
    d23.absorb(d3.diff().clone());
    let mut right = d1.diff().clone();
    right.absorb(d23);

    assert_eq!(left.apply(&base).expect("valid left diff"), s3);
    assert_eq!(right.apply(&base).expect("valid right diff"), s3);
    assert_eq!(left.apply(&base).expect("valid left diff"), right.apply(&base).expect("valid right diff"), "absorb must be associative");
}
//#endregion 🔖️AbsorbLaw

//#region 🔖️BetweenRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(ObjDiff::between(&a, &b).apply(&a).expect("valid forward diff"), b);
    assert_eq!(ObjDiff::between(&b, &a).apply(&b).expect("valid backward diff"), a);
    assert!(ObjDiff::between(&a, &a).is_empty());
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️FieldSweep
#[semio_framework_async_macros::async_test]
async fn field_sweep_every_mutable_field_changes() {
    let a = sweep_a();
    let b = sweep_b();

    let d_ab = ObjDiff::between(&a, &b);
    assert_eq!(d_ab.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) == b");
    let d_ba = ObjDiff::between(&b, &a);
    assert_eq!(d_ba.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) == a");
    assert!(ObjDiff::between(&a, &a).is_empty());

    // 🔍 Index-keyed collections: `between(a,b)` (b longer) proves modified+added;
    // `between(b,a)` (b longer, now the base) proves modified+removed. Combined, every
    // triple kind is exercised for every one of the four index-keyed collections.
    let vd_ab = d_ab.vertices.as_ref().expect("vertices diff populated (a->b)");
    assert!(vd_ab.removed.is_empty() && !vd_ab.modified.is_empty() && !vd_ab.added.is_empty());
    let vm = &vd_ab.modified[0].diff;
    assert!(vm.x.is_some() && vm.y.is_some() && vm.z.is_some() && vm.w.is_some(), "every ObjVertexDiff field must be patched");
    let vd_ba = d_ba.vertices.as_ref().expect("vertices diff populated (b->a)");
    assert!(!vd_ba.removed.is_empty() && !vd_ba.modified.is_empty() && vd_ba.added.is_empty());

    let td_ab = d_ab.texcoords.as_ref().expect("texcoords diff populated");
    assert!(!td_ab.modified.is_empty() && !td_ab.added.is_empty());
    let tm = &td_ab.modified[0].diff;
    assert!(tm.u.is_some() && tm.v.is_some(), "u/v must be patched");
    assert_eq!(tm.w, Some(None), "w tri-state must exercise Some(None) (source had w, target doesn't)");

    let nd_ab = d_ab.normals.as_ref().expect("normals diff populated");
    assert!(!nd_ab.modified.is_empty() && !nd_ab.added.is_empty());
    let nm = &nd_ab.modified[0].diff;
    assert!(nm.x.is_some() && nm.y.is_some() && nm.z.is_some());

    let fd_ab = d_ab.faces.as_ref().expect("faces diff populated");
    assert!(!fd_ab.modified.is_empty() && !fd_ab.added.is_empty());
    assert!(fd_ab.modified[0].diff.vertices.is_some());

    // 🔍 Name-keyed collections: all three kinds from ONE `between(a,b)` call.
    let gd = d_ab.groups.as_ref().expect("groups diff populated");
    assert!(!gd.removed.is_empty(), "removed must be non-empty (G1 dropped)");
    assert!(!gd.modified.is_empty(), "modified must be non-empty (G2's faces changed)");
    assert!(!gd.added.is_empty(), "added must be non-empty (G3 is new)");
    assert!(gd.modified[0].diff.faces.is_some());

    let od = d_ab.objects.as_ref().expect("objects diff populated");
    assert!(!od.removed.is_empty() && !od.modified.is_empty() && !od.added.is_empty());

    // 🔍 Scalars.
    assert_eq!(d_ab.mtllib, Some(None), "mtllib tri-state must exercise Some(None)");
    assert!(d_ab.usemtl.is_some());
    assert!(d_ab.smoothing_groups.is_some());
    assert!(d_ab.unknown_statements.is_some());
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws over every `ObjMutation` variant (handcrafted
/// impls over the `dsl::DslOps`-derived `DslVariants` — ticket `f6-recon-report.md` §2/§3;
/// `demo_mutation_cases()` already covers every variant, incl. `SetSnapshot`'s whole nested
/// `ObjSnapshot` tree and every index-/name-keyed leaf payload type).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for m in demo_mutation_cases() {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = ObjMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = ObjMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw

//#region 🔖️KindsCoverageLaw
/// 🏷️ `KINDS` must name exactly the enum's variants (kebab-case), one entry each — an
/// exhaustive `match` so the compiler itself fails the moment a variant is added, renamed or
/// removed without this list being updated alongside it. The manifest side of the same claim
/// (`../../🔣️oracle.json`'s `obj-3-0-any` catalog `kinds`) is checked by the
/// mutate/inverse test case's own contract gate, which fails if the two lists ever diverge.
#[semio_framework_async_macros::async_test]
async fn kinds_cover_every_variant() {
    fn kind_of(mutation: &ObjMutation) -> &'static str {
        match mutation {
            ObjMutation::SetSnapshot(_) => "set-snapshot",
            ObjMutation::InsertVertex(_) => "insert-vertex",
            ObjMutation::RemoveVertex(_) => "remove-vertex",
            ObjMutation::SetVertex(_) => "set-vertex",
            ObjMutation::InsertTexcoord(_) => "insert-texcoord",
            ObjMutation::RemoveTexcoord(_) => "remove-texcoord",
            ObjMutation::SetTexcoord(_) => "set-texcoord",
            ObjMutation::InsertNormal(_) => "insert-normal",
            ObjMutation::RemoveNormal(_) => "remove-normal",
            ObjMutation::SetNormal(_) => "set-normal",
            ObjMutation::InsertFace(_) => "insert-face",
            ObjMutation::RemoveFace(_) => "remove-face",
            ObjMutation::SetFace(_) => "set-face",
            ObjMutation::SetGroup(_) => "set-group",
            ObjMutation::RemoveGroup(_) => "remove-group",
            ObjMutation::SetObject(_) => "set-object",
            ObjMutation::RemoveObject(_) => "remove-object",
            ObjMutation::SetMtllib(_) => "set-mtllib",
            ObjMutation::SetUsemtl(_) => "set-usemtl",
            ObjMutation::SetSmoothingGroups(_) => "set-smoothing-groups",
            ObjMutation::SetUnknownStatements(_) => "set-unknown-statements",
        }
    }
    let mut exercised: Vec<&str> = demo_mutation_cases().iter().map(kind_of).collect();
    exercised.sort_unstable();
    exercised.dedup();
    let mut declared: Vec<&str> = KINDS.to_vec();
    declared.sort_unstable();
    assert_eq!(exercised, declared, "KINDS must name exactly the variants demo_mutation_cases() exercises");
    assert_eq!(KINDS.len(), 21, "obj-3-0-any declares 21 ObjMutation variants");
}
//#endregion 🔖️KindsCoverageLaw

//#region 🔖️IndexSpaceInverseLaw
/// 🧊️ Three faces, two bands and one object over them — the smallest mesh on which removing a
/// face disturbs a membership list that another entry sits after.
// 🚫️async: E1 pure test-fixture builder, no I/O — see R9
fn banded_snapshot() -> ObjSnapshot {
    let corner = |vertex: u32| ObjFaceVertex { vertex, texcoord: None, normal: None };
    let face = |a: u32, b: u32, c: u32| ObjFace { vertices: vec![corner(a), corner(b), corner(c)] };
    ObjSnapshot {
        schema: "stdio.obj".into(),
        vertices: vec![ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 0.0, y: 1.0, z: 0.0, w: None }],
        texcoords: vec![],
        normals: vec![],
        faces: vec![face(0, 1, 2), face(1, 2, 0), face(2, 0, 1)],
        groups: vec![ObjGroup { name: "front".into(), faces: vec![0, 1] }, ObjGroup { name: "back".into(), faces: vec![2] }],
        objects: vec![ObjObject { name: "shell".into(), faces: vec![0, 1, 2] }],
        mtllib: None,
        usemtl: vec![],
        smoothing_groups: vec![],
        unknown_statements: vec![],
    }
}

/// ↩️ Removing a face that BELONGS to a band must invert through the band, not through the row
/// alone: the undo names every membership list the positional removal disturbed and puts each
/// back to the exact list the pre-mutation document declared.
#[test]
fn remove_face_inverts_through_the_membership_it_disturbed() {
    let base = banded_snapshot();
    let removal = ObjMutation::RemoveFace(remove_face::RemoveFace { index: 1 });
    let undo = removal.inverse(&base);
    assert!(matches!(undo.first(), Some(ObjMutation::InsertFace(insert_face::InsertFace { index: 1, .. }))), "the row itself comes back first, at its own position: {undo:?}");
    assert!(undo.iter().any(|step| matches!(step, ObjMutation::SetGroup(set_group::SetGroup { name, faces }) if name == "front" && faces == &vec![0, 1])), "the removed face's own band must be re-declared: {undo:?}");
    assert!(undo.iter().any(|step| matches!(step, ObjMutation::SetGroup(set_group::SetGroup { name, faces }) if name == "back" && faces == &vec![2])), "so must the band the removal shifted: {undo:?}");
    assert!(undo.iter().any(|step| matches!(step, ObjMutation::SetObject(set_object::SetObject { name, faces }) if name == "shell" && faces == &vec![0, 1, 2])), "and the object over all three: {undo:?}");

    let mut restored = base.clone();
    apply_obj_mutation(&mut restored, &removal);
    assert_ne!(restored.faces, base.faces, "the removal has to move the mesh, or the undo proves nothing");
    for step in &undo {
        apply_obj_mutation(&mut restored, step);
    }
    assert_eq!(restored, base, "forward then inverse must return the whole snapshot, membership included");
}

/// ↩️ Removing the FIRST of several bands must invert back to that band's own position. A single
/// `SetGroup` appends instead, which the second half of this test states outright rather than
/// leaving as folklore.
#[test]
fn remove_group_inverts_back_to_its_own_position() {
    let base = banded_snapshot();
    let removal = ObjMutation::RemoveGroup(remove_group::RemoveGroup { name: "front".into() });
    let mut restored = base.clone();
    apply_obj_mutation(&mut restored, &removal);
    assert_eq!(restored.groups.len(), 1, "the removal has to move the document");

    let mut naive = restored.clone();
    apply_obj_mutation(&mut naive, &ObjMutation::SetGroup(set_group::SetGroup { name: "front".into(), faces: vec![0, 1] }));
    assert_eq!(naive.groups.iter().map(|group| group.name.as_str()).collect::<Vec<_>>(), vec!["back", "front"], "a lone SetGroup appends — this is the position loss the sequenced inverse repairs");

    for step in removal.inverse(&base) {
        apply_obj_mutation(&mut restored, &step);
    }
    assert_eq!(restored, base, "the sequenced inverse must restore both the membership and the order");
}

/// ↩️ The `o` mirror, kept separate because `groups` and `objects` are distinct name spaces.
#[test]
fn remove_object_inverts_back_to_its_own_position() {
    let mut base = banded_snapshot();
    base.objects = vec![ObjObject { name: "shell".into(), faces: vec![0, 1] }, ObjObject { name: "cap".into(), faces: vec![2] }];
    let removal = ObjMutation::RemoveObject(remove_object::RemoveObject { name: "shell".into() });
    let mut restored = base.clone();
    apply_obj_mutation(&mut restored, &removal);
    assert_eq!(restored.objects.len(), 1, "the removal has to move the document");
    for step in removal.inverse(&base) {
        apply_obj_mutation(&mut restored, &step);
    }
    assert_eq!(restored, base, "the sequenced inverse must restore both the membership and the order");
}
//#endregion 🔖️IndexSpaceInverseLaw
