use super::*;
use crate::schema::diff::{PptxOpcPartDiff, PptxShapeDiff};
use crate::schema::snapshot::{PptxPresentation, PptxRun, PptxXmlPart};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn other(name: &str) -> PptxShape {
    PptxShape::Other { node: XmlNode::Element { name: name.into(), attrs: Vec::new(), children: Vec::new() } }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(PptxPresentation {
        slides: vec![
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("first")], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 100 } }] },
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("second")], position: PptxTransform::default() }] },
        ],
    })
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_slide_apply_and_inverse() {
    let base = fixture();
    let insert = PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 1, slide: PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("inserted")], position: PptxTransform::default() }] } });
    let mut after = base.clone();
    apply_pptx_mutation(&mut after, &insert);
    assert_eq!(after.presentation.slides.len(), 3);

    let inverses = Mutation::inverse(&insert, &base);
    let mut restored = after.clone();
    for inv in &inverses {
        apply_pptx_mutation(&mut restored, inv);
    }
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn remove_slide_inverse_restores_removed_slide() {
    let base = fixture();
    let remove = PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 });
    let mut after = base.clone();
    apply_pptx_mutation(&mut after, &remove);
    assert_eq!(after.presentation.slides.len(), 1);
    for inv in Mutation::inverse(&remove, &base) {
        apply_pptx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn move_slide_apply_and_inverse() {
    let base = fixture();
    let mv = PptxMutation::MoveSlide(move_slide::MoveSlide { from: 0, to: 1 });
    let mut after = base.clone();
    apply_pptx_mutation(&mut after, &mv);
    assert_eq!(after.presentation.slides[1], base.presentation.slides[0]);
    assert_eq!(after.presentation.slides[0], base.presentation.slides[1]);
    for inv in Mutation::inverse(&mv, &base) {
        apply_pptx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_shape_apply_and_inverse() {
    let base = fixture();
    let shape = PptxShape::Picture { blip_rel_id: "rId9".into(), position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } };
    let insert = PptxMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 1, shape: shape.clone() });
    let mut after = base.clone();
    apply_pptx_mutation(&mut after, &insert);
    assert_eq!(after.presentation.slides[0].shapes.len(), 2);
    assert_eq!(after.presentation.slides[0].shapes[1], shape);
    for inv in Mutation::inverse(&insert, &base) {
        apply_pptx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let remove = PptxMutation::RemoveShape(remove_shape::RemoveShape { slide_index: 0, shape_index: 0 });
    let mut after2 = base.clone();
    apply_pptx_mutation(&mut after2, &remove);
    assert!(after2.presentation.slides[0].shapes.is_empty());
    for inv in Mutation::inverse(&remove, &base) {
        apply_pptx_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);
}

#[semio_framework_async_macros::async_test]
async fn set_shape_text_and_position_apply_and_inverse() {
    let base = fixture();
    let mutation = PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: 0, shape_index: 0, text_frame: vec![PptxParagraph::text("changed")] });
    let mut after = base.clone();
    apply_pptx_mutation(&mut after, &mutation);
    let PptxShape::TextBox { text_frame, .. } = &after.presentation.slides[0].shapes[0] else { panic!("text box") };
    assert_eq!(text_frame[0].runs[0].text, "changed");
    for inv in Mutation::inverse(&mutation, &base) {
        apply_pptx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let pos = PptxTransform { x: 10, y: 20, cx: 30, cy: 40 };
    let set_pos = PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { slide_index: 0, shape_index: 0, position: pos });
    let mut after2 = base.clone();
    apply_pptx_mutation(&mut after2, &set_pos);
    let PptxShape::TextBox { position, .. } = &after2.presentation.slides[0].shapes[0] else { panic!("text box") };
    assert_eq!(*position, pos);
    for inv in Mutation::inverse(&set_pos, &base) {
        apply_pptx_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);
}

#[semio_framework_async_macros::async_test]
async fn set_shape_text_on_picture_is_a_no_op() {
    let mut base = fixture();
    base.presentation.slides[0].shapes.push(PptxShape::Picture { blip_rel_id: "rId5".into(), position: PptxTransform::default() });
    let mutation = PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: 0, shape_index: 1, text_frame: vec![PptxParagraph::text("nope")] });
    let diff = Mutation::diff(&mutation, &base);
    assert!(<PptxDiff as DiffAlgebra<PptxSnapshot>>::is_empty(diff.diff()));
}

//#region 🔖️Fixtures
/// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field, both `presentation` and `opc`.
///
/// The recipe's "known structural trap" applies RECURSIVELY: within ANY single `between(a,b)`
/// call, no nesting level can show both `removed` AND `added` at once (a naive positional
/// diff has only one tail flavor per direction). So EVERY level here (top-level `slides`,
/// AND `slide0`'s own nested `shapes`) uses different-length lists and splits its
/// removed/added coverage across the two `between()` directions, same technique repeated one
/// level deeper:
/// - `slide0`: `sweep_a` has `[TextBox_old, Picture_toDrop]` (len 2), `sweep_b` has
///   `[TextBox_new]` (len 1) -- `a -> b` shows `shapes.removed` (Picture) +
///   `shapes.modified` (TextBox, every field incl. the `font_size` tri-state and a nested
///   `text_frame` paragraph ADDED); `b -> a` shows `shapes.added` (the same Picture, whole).
/// - `slide1`: `[Placeholder_old]` -> `[Placeholder_new]`, same length both sides -- pure
///   `shapes.modified`, exercising `kind`/`text_frame`/`position` all three in one diff.
/// - top level: `sweep_a` has 3 slides, `sweep_b` has 2 -- `a -> b` shows `slides.removed`
///   (the dropped `Other`-shaped slide2) + `slides.modified` (both slide0 and slide1); `b ->
///   a` shows `slides.added` (slide2, whole, carrying its logical `Other` XML node).
///
/// `opc` content_types/parts/relationships each get one removed, one modified, one added,
/// same convention as docx's own sweep fixtures (name-keyed collections have no such trap).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> PptxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_default("toRemove", "application/octet-stream");
    opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", b"<p:presentation/>".to_vec());
    opc.set_part("ppt/toModify.xml", "application/xml", b"old".to_vec());
    opc.set_part("ppt/toRemove.xml", "application/xml", b"gone".to_vec());
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
    opc.add_relationship("", "rId9", "http://example/toRemove", "ppt/toRemove.xml");
    opc.relationships.insert("ppt/presentation.xml".into(), vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "slides/old.xml".into(), target_mode: OpcTargetMode::Internal }]);
    opc.relationships.insert("ppt/toRemove.xml".into(), vec![OpcRelationship { id: "rId8".into(), rel_type: "http://example/ownerToRemove".into(), target: "media/gone.png".into(), target_mode: OpcTargetMode::Internal }]);

    PptxSnapshot::from_parts(
        opc,
        vec![PptxXmlPart { path: "docProps/core.xml".into(), content_type: "application/xml".into(), document: XmlDocument::default() }],
        PptxPresentation {
            slides: vec![
                PptxSlide {
                    shapes: vec![
                        PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "old".into(), bold: false, italic: false, font_size: Some(10) }] }], position: PptxTransform { x: 1, y: 1, cx: 1, cy: 1 } },
                        PptxShape::Picture { blip_rel_id: "rIdToDrop".into(), position: PptxTransform::default() },
                    ],
                },
                PptxSlide { shapes: vec![PptxShape::Placeholder { kind: "body".into(), text_frame: vec![PptxParagraph::text("stay old")], position: PptxTransform::default() }] },
                PptxSlide { shapes: vec![other("p:graphicFrame")] },
            ],
        },
    )
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> PptxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_default("added", "application/octet-stream");
    opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", b"<p:presentation/>changed".to_vec());
    opc.set_part("ppt/toModify.xml", "application/xml", b"new".to_vec());
    opc.set_part("ppt/added.xml", "application/xml", b"fresh".to_vec());
    opc.content_types.set_override("ppt/toModify.xml", "application/xml-modified");
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
    opc.relationships.insert("ppt/presentation.xml".into(), vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "slides/new.xml".into(), target_mode: OpcTargetMode::Internal }]);
    opc.relationships.insert("ppt/added.xml".into(), vec![OpcRelationship { id: "rId3".into(), rel_type: "http://example/added".into(), target: "media/added.png".into(), target_mode: OpcTargetMode::Internal }]);

    PptxSnapshot::from_parts(
        opc,
        vec![PptxXmlPart { path: "docProps/app.xml".into(), content_type: "application/xml".into(), document: XmlDocument::default() }],
        PptxPresentation {
            slides: vec![
                PptxSlide {
                    shapes: vec![PptxShape::TextBox {
                        text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "new".into(), bold: true, italic: true, font_size: None }] }, PptxParagraph::text("second para")],
                        position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 },
                    }],
                },
                PptxSlide { shapes: vec![PptxShape::Placeholder { kind: "subTitle".into(), text_frame: vec![PptxParagraph::text("stay new")], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } }] },
            ],
        },
    )
}
//#endregion 🔖️Fixtures

//#region 🔖️MutationDiffLaw
/// 🎯️ A `SetSnapshot` target for `sample_mutations()`, built through `build_minimal_pptx`
/// with the SAME slide COUNT as `fixture()` (2), only DIFFERENT content -- unlike `sweep_b()`
/// (a deliberately minimal, unrelated hand-built OPC used only by `field_sweep`/
/// `between_roundtrip_law`, which document/tolerate the name-keyed collection's known
/// order-on-append caveat), this shares `fixture()`'s EXACT OPC key set (same six parts:
/// slideMaster/slideLayout/theme/slide1/slide2/presentation.xml), so `between()` produces
/// ONLY `modified` entries (zero removed/added) at the OPC level -- a clean bijection that
/// round-trips losslessly through TWO independent `between()` calls (base->next->base),
/// which `inverse_law` requires. Mixing OPC key SETS (as `sweep_b()` does) is fine for a
/// SINGLE `between(a,b).apply(a)==b` check (that's what `field_sweep` proves) but is NOT
/// guaranteed to survive a round trip THROUGH an intermediate state whose key set differs
/// again on the way back -- a structural property of the append-new-at-end convention this
/// engine shares with docx's, not a regression.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn mutated_fixture() -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(PptxPresentation {
        slides: vec![
            PptxSlide { shapes: vec![PptxShape::Placeholder { kind: "title".into(), text_frame: vec![PptxParagraph::text("changed first")], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } }] },
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("changed second")], position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } }] },
        ],
    })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_mutations() -> Vec<PptxMutation> {
    vec![
        PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: mutated_fixture() }),
        PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 1, slide: PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("x")], position: PptxTransform::default() }] } }),
        PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }),
        PptxMutation::MoveSlide(move_slide::MoveSlide { from: 0, to: 1 }),
        PptxMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::Picture { blip_rel_id: "rId7".into(), position: PptxTransform::default() } }),
        PptxMutation::RemoveShape(remove_shape::RemoveShape { slide_index: 0, shape_index: 0 }),
        PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: 0, shape_index: 0, text_frame: vec![PptxParagraph::text("z")] }),
        PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { slide_index: 0, shape_index: 0, position: PptxTransform { x: 5, y: 6, cx: 7, cy: 8 } }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in sample_mutations() {
        let base = fixture();
        let diff_direct = Mutation::diff(&mutation, &base);
        let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).unwrap();

        let mut via_apply = base.clone();
        let diff_from_apply = apply_pptx_mutation(&mut via_apply, &mutation);

        assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
        assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    for mutation in sample_mutations() {
        let base = fixture();

        let mut round_tripped = base.clone();
        apply_pptx_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in <PptxMutation as Mutation<PptxSnapshot>>::inverse(&mutation, &base) {
            apply_pptx_mutation(&mut round_tripped, &inverse_mutation);
        }
        assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

        let diff = Mutation::diff(&mutation, &base);
        let next = MutationDiff::apply(diff.diff(), &base).unwrap();
        let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
        let restored = MutationDiff::apply(&inverse_diff, &next).unwrap();
        assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️AbsorbLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_absorb_matches_sequential(base: &PptxSnapshot, d1: &PptxDiff, d2: &PptxDiff) -> PptxDiff {
    let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base).unwrap()).unwrap();
    let mut absorbed = d1.clone();
    MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(MutationDiff::apply(&absorbed, base).unwrap(), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn slides_diff(diff: &PptxDiff) -> &crate::schema::diff::PptxSlidesDiff {
    diff.presentation.as_ref().expect("presentation diff present").slides.as_ref().expect("slides diff present")
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
    {
        let base = fixture();
        let slide = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("f")], position: PptxTransform::default() }] };
        let d1 = Mutation::diff(&PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: slide.clone() }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = slides_diff(&absorbed);
        assert_eq!(triple.removed, vec![0]);
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].index, 1);
        assert_eq!(triple.added[0].item, slide);
    }

    // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
    {
        let base = fixture();
        let f = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("f")], position: PptxTransform::default() }] };
        let g = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("g")], position: PptxTransform::default() }] };
        let d1 = Mutation::diff(&PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: f.clone() }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: g.clone() }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = slides_diff(&absorbed);
        assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
        assert!(triple.added.iter().any(|a| a.item == f));
        assert!(triple.added.iter().any(|a| a.item == g));
    }

    // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
    {
        let base = fixture();
        let f = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("f")], position: PptxTransform::default() }] };
        let d1 = Mutation::diff(&PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 1, slide: f }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: 1, shape_index: 0, text_frame: vec![PptxParagraph::text("patched")] }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = slides_diff(&absorbed);
        assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
        assert_eq!(triple.added.len(), 1);
        let PptxShape::TextBox { text_frame, .. } = &triple.added[0].item.shapes[0] else { panic!("text box") };
        assert_eq!(text_frame[0].runs[0].text, "patched");
    }

    // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
    {
        let base = fixture();
        let d1 = Mutation::diff(&PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: 1, shape_index: 0, text_frame: vec![PptxParagraph::text("patched")] }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index: 1 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = slides_diff(&absorbed);
        assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
        assert_eq!(triple.removed, vec![1]);
    }

    // Associativity over a triple.
    {
        let base = fixture();
        let f = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("f")], position: PptxTransform::default() }] };
        let g = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("g")], position: PptxTransform::default() }] };
        let d1 = Mutation::diff(&PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: f }), &base);
        let mid1 = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: g }), &mid1);
        let mid2 = MutationDiff::apply(d2.diff(), &mid1).unwrap();
        let d3 = Mutation::diff(&PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }), &mid2);
        let sequential = MutationDiff::apply(d3.diff(), &mid2).unwrap();

        let mut left = d1.diff().clone();
        MutationDiff::absorb(&mut left, d2.diff().clone());
        MutationDiff::absorb(&mut left, d3.diff().clone());

        let mut d2_then_d3 = d2.diff().clone();
        MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
        let mut right = d1.diff().clone();
        MutationDiff::absorb(&mut right, d2_then_d3);

        assert_eq!(MutationDiff::apply(&left, &base).unwrap(), sequential, "absorb associativity (left) failed");
        assert_eq!(MutationDiff::apply(&right, &base).unwrap(), sequential, "absorb associativity (right) failed");
    }
}
//#endregion 🔖️AbsorbLaw

//#region 🔖️BetweenRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&a, &b), &a).unwrap(), b);
    assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&b, &a), &b).unwrap(), a);

    let sample = fixture();
    assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&sample, &sample), &sample).unwrap(), sample);

    // "Real" fixture leg: a realistic multi-slide presentation diffed against a mutated variant.
    let real = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(PptxPresentation {
        slides: vec![
            PptxSlide { shapes: vec![PptxShape::Placeholder { kind: "title".into(), text_frame: vec![PptxParagraph::text("Chapter One")], position: PptxTransform::default() }] },
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("Body text goes here.")], position: PptxTransform::default() }] },
        ],
    });
    let mut mutated = real.clone();
    apply_pptx_mutation(&mut mutated, &PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: 0, shape_index: 0, text_frame: vec![PptxParagraph::text("Chapter Two")] }));
    assert_ne!(real, mutated);
    assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&real, &mutated), &real).unwrap(), mutated);
    assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&mutated, &real), &mutated).unwrap(), real);
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️CodecRetentionLaw
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let authored = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(PptxPresentation {
        slides: vec![PptxSlide {
            shapes: vec![
                PptxShape::Placeholder {
                    kind: "ctrTitle".into(),
                    text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Hello".into(), bold: true, italic: true, font_size: Some(44) }] }],
                    position: PptxTransform { x: 100, y: 200, cx: 300, cy: 400 },
                },
                PptxShape::Picture { blip_rel_id: "rId2".into(), position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } },
                other("p:graphicFrame"),
            ],
        }],
    });
    let native = crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx(&authored).expect("encode authored");
    let snap = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&native).expect("decode authored");
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <PptxSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
//#endregion 🔖️CodecRetentionLaw

//#region 🔖️FieldSweep
/// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field across BOTH
/// `opc` and `presentation`, at every nesting level (`slides` AND, within a modified slide,
/// `shapes` AND, within a modified `TextBox`/`Placeholder`, `text_frame`/`runs`) -- see the
/// fixtures' doc comment for exactly how each collection flavor (removed/modified/added) is
/// exercised and split across the two `between()` directions per this ticket's "known
/// structural trap" note, which this test found applies recursively at every level, not just
/// the top one.
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&a, &b);
    assert_eq!(MutationDiff::apply(&diff_ab, &a).unwrap(), b);
    let diff_ba = <PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&b, &a);
    assert_eq!(MutationDiff::apply(&diff_ba, &b).unwrap(), a);
    assert!(<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&a, &a).is_empty());

    // opc: content_types (both defaults+overrides), parts, relationships all populated.
    let opc_diff = diff_ab.opc.as_ref().expect("opc diff present");
    assert_eq!(diff_ab.xml_parts, Some(b.xml_parts.clone()), "logical XML parts not exercised");
    let ct = opc_diff.content_types.as_ref().expect("content_types diff present");
    let defaults = ct.defaults.as_ref().expect("defaults diff present");
    assert!(!defaults.added.is_empty(), "content_types.defaults: added not exercised");
    let overrides = ct.overrides.as_ref().expect("overrides diff present");
    assert!(!overrides.modified.is_empty(), "content_types.overrides: modified not exercised");
    let parts = opc_diff.parts.as_ref().expect("parts diff present");
    assert!(!parts.removed.is_empty(), "opc.parts: removed not exercised");
    assert!(!parts.modified.is_empty(), "opc.parts: modified not exercised");
    assert!(!parts.added.is_empty(), "opc.parts: added not exercised");
    let part_mod = &parts.modified[0];
    assert!(matches!(&part_mod.diff, PptxOpcPartDiff { bytes: Some(_), .. }));
    let rels = opc_diff.relationships.as_ref().expect("relationships diff present");
    assert!(!rels.removed.is_empty(), "opc.relationships: removed (owner) not exercised");
    assert!(!rels.modified.is_empty(), "opc.relationships: modified (owner) not exercised");
    assert!(!rels.added.is_empty(), "opc.relationships: added (owner) not exercised");

    // presentation.slides, `a -> b` direction: top-level `removed` (the dropped `Other`
    // slide, index 2) + `modified` (BOTH slide0 and slide1 differ). The recipe's "known
    // structural trap" applies RECURSIVELY -- within this ONE `between(a,b)` call, no level
    // (top OR nested) can show both `removed` and `added` at once, so slide0's own nested
    // `shapes` diff (below) shows removed+modified here, and `added` only in the `b -> a` leg.
    let pres_diff = diff_ab.presentation.as_ref().expect("presentation diff present");
    let slides = pres_diff.slides.as_ref().expect("slides diff present");
    assert_eq!(slides.removed, vec![2], "slides: removed (top) not exercised");
    assert_eq!(slides.modified.len(), 2, "both slide0 and slide1 must be modified");

    // slide0 (modified.index == 0): shapes.removed (Picture dropped) + shapes.modified
    // (TextBox, every field incl. the font_size tri-state Some(None) + a nested text_frame
    // paragraph ADDED).
    let slide0_diff = &slides.modified.iter().find(|m| m.index == 0).expect("slide0 modified").diff;
    let shapes_diff = slide0_diff.shapes.as_ref().expect("slide0: shapes not exercised");
    assert!(!shapes_diff.removed.is_empty(), "shapes: removed (Picture dropped) not exercised");
    assert!(!shapes_diff.modified.is_empty(), "shapes: modified (TextBox) not exercised");
    let PptxShapeDiff::TextBox(tb_diff) = &shapes_diff.modified[0].diff else { panic!("expected TextBox diff") };
    assert!(tb_diff.position.is_some(), "modified TextBox: position not exercised");
    let tf_diff = tb_diff.text_frame.as_ref().expect("modified TextBox: text_frame not exercised");
    assert!(!tf_diff.modified.is_empty(), "text_frame: modified not exercised");
    assert!(!tf_diff.added.is_empty(), "text_frame: added (second paragraph) not exercised");
    let run_diff = &tf_diff.modified[0].diff.runs.as_ref().expect("paragraph: runs not exercised").modified[0].diff;
    assert!(run_diff.text.is_some() && run_diff.bold.is_some() && run_diff.italic.is_some(), "modified run: text/bold/italic not exercised");
    assert_eq!(run_diff.font_size, Some(None), "run font_size tri-state Some(None) not exercised");

    // slide1 (modified.index == 1): Placeholder modified in ALL THREE of its own fields
    // (kind/text_frame/position) in one diff -- same length both sides, pure `modified`.
    let slide1_diff = &slides.modified.iter().find(|m| m.index == 1).expect("slide1 modified").diff;
    let shapes_diff_1 = slide1_diff.shapes.as_ref().expect("slide1: shapes not exercised");
    let PptxShapeDiff::Placeholder(ph_diff) = &shapes_diff_1.modified[0].diff else { panic!("expected Placeholder diff") };
    assert_eq!(ph_diff.kind.as_deref(), Some("subTitle"), "placeholder kind not exercised");
    assert!(ph_diff.text_frame.is_some(), "placeholder text_frame not exercised");
    assert!(ph_diff.position.is_some(), "placeholder position not exercised");

    // `b -> a` exercises: top-level `added` (the dropped `Other` slide, carried whole) +
    // slide0's `shapes.added` (the Picture, whole) + the font_size tri-state's OTHER state
    // (`Some(Some(10))`, restoring the value `a` had).
    let slides_ba = diff_ba.presentation.as_ref().unwrap().slides.as_ref().expect("slides diff (b->a) present");
    assert!(!slides_ba.added.is_empty(), "slides (b->a): added (top) not exercised");
    let PptxShape::Other { node } = &slides_ba.added[0].item.shapes[0] else { panic!("expected added Other shape") };
    assert!(matches!(node, XmlNode::Element { .. }));

    let slide0_diff_ba = &slides_ba.modified.iter().find(|m| m.index == 0).expect("slide0 modified (b->a)").diff;
    let shapes_diff_ba = slide0_diff_ba.shapes.as_ref().expect("shapes diff (b->a) present");
    assert!(!shapes_diff_ba.added.is_empty(), "shapes (b->a): added (Picture) not exercised");
    let PptxShape::Picture { blip_rel_id, .. } = &shapes_diff_ba.added[0].item else { panic!("expected added Picture") };
    assert_eq!(blip_rel_id, "rIdToDrop");
    let PptxShapeDiff::TextBox(tb_ba) = &shapes_diff_ba.modified[0].diff else { panic!("expected TextBox diff (b->a)") };
    let tf_ba = tb_ba.text_frame.as_ref().expect("text_frame diff (b->a) present");
    let run_diff_ba = &tf_ba.modified[0].diff.runs.as_ref().expect("runs diff (b->a) present").modified[0].diff;
    assert_eq!(run_diff_ba.font_size, Some(Some(10)), "run font_size tri-state Some(Some(_)) not exercised");
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `PptxMutation` grammar --
/// exercises every variant incl. `SetSnapshot`'s full `PptxSnapshot` (OPC package + typed
/// slides), `InsertShape`'s bare `PptxShape` enum payload (every `TextBox`/`Picture`/
/// `Placeholder`/`Other` variant), and `SetShapeText`'s `Vec<PptxParagraph>`.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = fixture();
    let mutations = vec![
        PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 1, slide: PptxSlide { shapes: vec![other("p:graphicFrame")] } }),
        PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }),
        PptxMutation::MoveSlide(move_slide::MoveSlide { from: 0, to: 1 }),
        PptxMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::TextBox { text_frame: vec![PptxParagraph::text("x")], position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } } }),
        PptxMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::Picture { blip_rel_id: "rId7".into(), position: PptxTransform::default() } }),
        PptxMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::Placeholder { kind: "body".into(), text_frame: vec![PptxParagraph::text("ph")], position: PptxTransform::default() } }),
        PptxMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 1, shape: other("p:cxnSp") }),
        PptxMutation::RemoveShape(remove_shape::RemoveShape { slide_index: 0, shape_index: 0 }),
        PptxMutation::SetShapeText(set_shape_text::SetShapeText {
            slide_index: 0,
            shape_index: 0,
            text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "bold".into(), bold: true, italic: false, font_size: Some(24) }] }, PptxParagraph::text("second")],
        }),
        PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { slide_index: 0, shape_index: 0, position: PptxTransform { x: 5, y: 6, cx: 7, cy: 8 } }),
    ];
    for mutation in mutations {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = PptxMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = PptxMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw

//#region 🔖️KindsLaw
/// 🧪️ Wave 7 (ticket 26/08/23/END-TO-END-TESTING-REFACTOR): `KINDS` must list exactly the
/// kebab-case spelling of every `PptxMutation` variant, exactly once, and match the mutation
/// oracle manifest's own declared `kinds` — the manifest is the only place the test platform
/// reads this vocabulary from, and it never parses this enum.
#[semio_framework_async_macros::async_test]
async fn kinds_matches_enum_variants_and_manifest() {
    let observed: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(kind_of).collect();
    let declared: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
    assert_eq!(observed, declared, "KINDS must list exactly the kebab-case spelling of every PptxMutation variant");
    assert_eq!(KINDS.len(), demo_mutation_cases().len(), "KINDS must cover every variant exactly once, with no duplicates");

    let manifest: pack::JsonValue = pack::parse_json(include_str!("../../../../🔮️oracles/🔣️.json")).expect("valid oracle manifest JSON");
    let catalog_kinds: std::collections::BTreeSet<String> = manifest["mutationCatalogs"][0]["kinds"].as_array().expect("mutationCatalogs[0].kinds array").iter().map(|value| value.as_str().expect("kind is a string").to_string()).collect();
    let declared_owned: std::collections::BTreeSet<String> = KINDS.iter().map(|kind| kind.to_string()).collect();
    assert_eq!(catalog_kinds, declared_owned, "the oracle manifest's mutationCatalogs[0].kinds must match KINDS exactly");
}
//#endregion 🔖️KindsLaw
