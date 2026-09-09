use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::document::schema::snapshot::DocRun;
use crate::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, SlidePictureImage, SlideTableCell, SlideTableRow};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

/// 🧪️ kinds_match_the_enum_and_the_catalog — the honesty check the test platform cannot make
/// for itself, because the framework reads a DECLARED list and never parses Rust. Two claims:
/// every enum variant reaches `KINDS` at its own [`variant_ordinal`] under exactly the keyword
/// its `print_op` grammar emits (`demo_mutation_cases` carries at least one instance of every
/// variant), and `KINDS` is character-for-character the `semio-v1-presentation` catalog the
/// platform reads.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let mut covered = vec![false; KINDS.len()];
    for case in demo_mutation_cases() {
        let ordinal = variant_ordinal(&case) as usize;
        let keyword = case.print_op().split(' ').next().expect("print_op is never empty").to_string();
        assert_eq!(KINDS[ordinal], keyword, "semio-presentation: KINDS[{ordinal}] must be the keyword print_op emits for {case:?}");
        covered[ordinal] = true;
    }
    let uncovered: Vec<&&str> = KINDS.iter().zip(&covered).filter(|(_, hit)| !**hit).map(|(kind, _)| kind).collect();
    assert!(uncovered.is_empty(), "semio-presentation: demo_mutation_cases carries no instance of {uncovered:?}, so those kinds are declared but never exercised");

    let manifest: serde_json::Value = serde_json::from_str(include_str!("../../../../🔮️oracle/🔣️.json")).expect("the subset's own oracle manifest decodes");
    let catalog = manifest["mutationCatalogs"].as_array().expect("the manifest declares mutationCatalogs").iter().find(|entry| entry["id"] == "semio-v1-presentation").expect("the manifest declares the semio-v1-presentation catalog");
    let declared: Vec<&str> = catalog["kinds"].as_array().expect("the catalog declares kinds").iter().map(|kind| kind.as_str().expect("every declared kind is a string")).collect();
    assert_eq!(declared, KINDS.to_vec(), "semio-presentation: the declared catalog and KINDS have drifted apart");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn frame(x: f64, y: f64, w: f64, h: f64) -> SlideFrame {
    SlideFrame { origin: SemioPoint2 { x, y }, width: w, height: h }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn text_block(text: &str) -> DocBlock {
    DocBlock::paragraph(text)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioPresentationSnapshot {
    SemioPresentationSnapshot {
        schema: "s.stdio.semio.presentation".into(),
        masters: vec![SlideMaster { id: "master1".into(), shapes: Vec::new() }],
        layouts: vec![SlideLayout { id: "layout1".into(), master_id: "master1".into(), shapes: Vec::new() }],
        slides: vec![
            Slide { id: "s1".into(), layout_id: Some("layout1".into()), shapes: vec![SlideShape::TextBox { frame: frame(0.0, 0.0, 10.0, 10.0), blocks: vec![text_block("first")] }], notes: Vec::new() },
            Slide { id: "s2".into(), layout_id: None, shapes: Vec::new(), notes: vec![text_block("note")] },
        ],
    }
}

//#region 🔖️Fixtures
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> SemioPresentationSnapshot {
    SemioPresentationSnapshot {
        schema: "s.stdio.semio.presentation".into(),
        masters: vec![
            SlideMaster { id: "keep".into(), shapes: vec![SlideShape::Placeholder { frame: frame(0.0, 0.0, 5.0, 5.0), kind: PlaceholderKind::Title }] },
            SlideMaster { id: "toModify".into(), shapes: Vec::new() },
            SlideMaster { id: "toRemove".into(), shapes: Vec::new() },
        ],
        layouts: vec![SlideLayout { id: "keepLayout".into(), master_id: "toRemove".into(), shapes: Vec::new() }, SlideLayout { id: "toRemoveLayout".into(), master_id: "keep".into(), shapes: Vec::new() }],
        slides: vec![
            Slide { id: "toModifySlide".into(), layout_id: None, shapes: vec![SlideShape::TextBox { frame: frame(0.0, 0.0, 1.0, 1.0), blocks: vec![text_block("old")] }], notes: vec![text_block("oldNote")] },
            Slide { id: "keepSlide".into(), layout_id: Some("keepLayout".into()), shapes: Vec::new(), notes: Vec::new() },
            Slide { id: "toDropSlide".into(), layout_id: None, shapes: Vec::new(), notes: Vec::new() },
        ],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> SemioPresentationSnapshot {
    SemioPresentationSnapshot {
        schema: "s.stdio.semio.presentation".into(),
        masters: vec![
            SlideMaster { id: "keep".into(), shapes: vec![SlideShape::Placeholder { frame: frame(0.0, 0.0, 5.0, 5.0), kind: PlaceholderKind::Title }] },
            SlideMaster { id: "toModify".into(), shapes: vec![SlideShape::Placeholder { frame: frame(1.0, 1.0, 2.0, 2.0), kind: PlaceholderKind::Body }] },
            SlideMaster { id: "addedMaster".into(), shapes: Vec::new() },
        ],
        layouts: vec![SlideLayout { id: "keepLayout".into(), master_id: "keep".into(), shapes: Vec::new() }, SlideLayout { id: "addedLayout".into(), master_id: "toModify".into(), shapes: Vec::new() }],
        // 🎯️ Length 2 vs `sweep_a`'s 3: per docx's own "known structural trap" precedent, a
        // single same-direction `between()` call on an INDEX-keyed collection can never show
        // BOTH a top-level `removed` AND a top-level `added` (only one tail flavor per
        // direction) -- `a -> b` exercises `slides.removed` (the dropped `toDropSlide`, index
        // 2) + `slides.modified[0]` (nested shapes modified+added, nested notes added);
        // `b -> a` (asserted separately in `field_sweep` below) exercises `slides.added` (the
        // very same dropped slide, carried whole as the added item's payload).
        slides: vec![
            Slide {
                id: "toModifySlide".into(),
                layout_id: Some("keepLayout".into()),
                shapes: vec![
                    SlideShape::TextBox { frame: frame(0.0, 0.0, 1.0, 1.0), blocks: vec![text_block("new")] },
                    SlideShape::Picture { frame: frame(2.0, 2.0, 3.0, 3.0), image: SlidePictureImage { asset_id: "a1".into(), mime: "image/png".into(), bytes: vec![1, 2] } },
                ],
                notes: vec![text_block("newNote"), text_block("secondNote")],
            },
            Slide { id: "keepSlide".into(), layout_id: Some("keepLayout".into()), shapes: Vec::new(), notes: Vec::new() },
        ],
    }
}
//#endregion 🔖️Fixtures

//#region 🔖️MutationDiffLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_mutations() -> Vec<SemioPresentationMutation> {
    vec![
        SemioPresentationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: 1, slide: Slide { id: "new".into(), layout_id: None, shapes: Vec::new(), notes: Vec::new() } }),
        SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }),
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: 0, layout_id: Some("layout1".into()) }),
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: 0, layout_id: None }),
        SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index: 1, notes: vec![text_block("updated")] }),
        SemioPresentationMutation::InsertShape(insert_shape::InsertShape {
            slide_index: 0,
            shape_index: 1,
            shape: SlideShape::Picture { frame: frame(0.0, 0.0, 1.0, 1.0), image: SlidePictureImage { asset_id: "x".into(), mime: "image/png".into(), bytes: vec![7] } },
        }),
        SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index: 0, shape_index: 0 }),
        SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index: 0, shape_index: 0, frame: frame(9.0, 9.0, 9.0, 9.0) }),
        SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index: 0, shape_index: 0, blocks: vec![text_block("changed")] }),
        SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: SlideMaster { id: "m2".into(), shapes: Vec::new() } }),
        SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id: "master1".into() }),
        SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout: SlideLayout { id: "l2".into(), master_id: "master1".into(), shapes: Vec::new() } }),
        SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id: "layout1".into() }),
        SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id: "layout1".into(), master_id: "master1".into() }),
    ]
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn apply_valid(diff: &SemioPresentationDiff, base: &SemioPresentationSnapshot) -> SemioPresentationSnapshot {
    MutationDiff::apply(diff, base).expect("valid Semio presentation diff fixture")
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in sample_mutations() {
        let base = fixture();
        let diff_direct = Mutation::diff(&mutation, &base);
        let applied_via_diff = apply_valid(diff_direct.diff(), &base);

        let mut via_apply = base.clone();
        let diff_from_apply = apply_semio_presentation_mutation(&mut via_apply, &mutation);

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
        apply_semio_presentation_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in <SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::inverse(&mutation, &base) {
            apply_semio_presentation_mutation(&mut round_tripped, &inverse_mutation);
        }
        assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

        let diff = Mutation::diff(&mutation, &base);
        let next = apply_valid(diff.diff(), &base);
        let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
        let restored = apply_valid(&inverse_diff, &next);
        assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️AbsorbLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_absorb_matches_sequential(base: &SemioPresentationSnapshot, d1: &SemioPresentationDiff, d2: &SemioPresentationDiff) -> SemioPresentationDiff {
    let sequential = apply_valid(d2, &apply_valid(d1, base));
    let mut absorbed = d1.clone();
    MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(apply_valid(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn slides_triple(diff: &SemioPresentationDiff) -> &crate::standards::v1::subsets::presentation::schema::diff::SlidesDiff {
    diff.slides.as_ref().expect("slides diff present")
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
    {
        let base = fixture();
        let new_slide = || Slide { id: "f".into(), layout_id: None, shapes: Vec::new(), notes: Vec::new() };
        let d1 = Mutation::diff(&SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: new_slide() }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = slides_triple(&absorbed);
        assert_eq!(triple.removed, vec![0]);
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].index, 1);
        assert_eq!(triple.added[0].item, new_slide());
    }

    // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
    {
        let base = fixture();
        let slide_f = Slide { id: "f".into(), layout_id: None, shapes: Vec::new(), notes: Vec::new() };
        let slide_g = Slide { id: "g".into(), layout_id: None, shapes: Vec::new(), notes: Vec::new() };
        let d1 = Mutation::diff(&SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: slide_f.clone() }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: slide_g.clone() }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = slides_triple(&absorbed);
        assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
        assert!(triple.added.iter().any(|a| a.item == slide_f));
        assert!(triple.added.iter().any(|a| a.item == slide_g));
    }

    // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
    {
        let base = fixture();
        let slide_f = Slide { id: "f".into(), layout_id: None, shapes: Vec::new(), notes: Vec::new() };
        let d1 = Mutation::diff(&SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: 1, slide: slide_f }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: 1, layout_id: Some("patched".into()) }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = slides_triple(&absorbed);
        assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].item.layout_id, Some("patched".to_string()));
    }

    // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
    {
        let base = fixture();
        let d1 = Mutation::diff(&SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: 1, layout_id: Some("x".into()) }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: 1 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = slides_triple(&absorbed);
        assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
        assert_eq!(triple.removed, vec![1]);
    }

    // Associativity over a triple.
    {
        let base = fixture();
        let slide_f = Slide { id: "f".into(), layout_id: None, shapes: Vec::new(), notes: Vec::new() };
        let slide_g = Slide { id: "g".into(), layout_id: None, shapes: Vec::new(), notes: Vec::new() };
        let d1 = Mutation::diff(&SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: slide_f }), &base);
        let mid1 = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: 2, slide: slide_g }), &mid1);
        let mid2 = apply_valid(d2.diff(), &mid1);
        let d3 = Mutation::diff(&SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }), &mid2);
        let sequential = apply_valid(d3.diff(), &mid2);

        let mut left = d1.diff().clone();
        MutationDiff::absorb(&mut left, d2.diff().clone());
        MutationDiff::absorb(&mut left, d3.diff().clone());

        let mut d2_then_d3 = d2.diff().clone();
        MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
        let mut right = d1.diff().clone();
        MutationDiff::absorb(&mut right, d2_then_d3);

        assert_eq!(apply_valid(&left, &base), sequential, "absorb associativity (left) failed");
        assert_eq!(apply_valid(&right, &base), sequential, "absorb associativity (right) failed");
    }
}
//#endregion 🔖️AbsorbLaw

//#region 🔖️BetweenRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(apply_valid(&<SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::between(&a, &b), &a), b);
    assert_eq!(apply_valid(&<SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::between(&b, &a), &b), a);

    let sample = fixture();
    assert_eq!(apply_valid(&<SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::between(&sample, &sample), &sample), sample);

    // "Real" fixture leg: a realistic small deck diffed against a mutated variant.
    let real = fixture();
    let mut mutated = real.clone();
    apply_semio_presentation_mutation(&mut mutated, &SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index: 0, shape_index: 0, blocks: vec![text_block("Chapter Two")] }));
    assert_ne!(real, mutated);
    assert_eq!(apply_valid(&<SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::between(&real, &mutated), &real), mutated);
    assert_eq!(apply_valid(&<SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::between(&mutated, &real), &mutated), real);
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️CodecRetentionLaw
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = fixture();
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
//#endregion 🔖️CodecRetentionLaw

//#region 🔖️FieldSweep
/// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field across
/// `masters`, `layouts`, and `slides` (incl. the nested shape tree, `document::DocBlock` reuse,
/// and the `layout_id` tri-state).
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::between(&a, &b);
    assert_eq!(apply_valid(&diff_ab, &a), b);
    let diff_ba = <SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::between(&b, &a);
    assert_eq!(apply_valid(&diff_ba, &b), a);
    assert!(<SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::between(&a, &a).is_empty());

    let masters = diff_ab.masters.as_ref().expect("masters diff present");
    assert!(!masters.removed.is_empty(), "masters: removed not exercised");
    assert!(!masters.added.is_empty(), "masters: added not exercised");
    let master_mod = masters.modified.iter().find(|m| m.key == "toModify").expect("toModify master modified");
    assert!(master_mod.diff.shapes.as_ref().expect("master shapes diff present").added.len() > 0);

    let layouts = diff_ab.layouts.as_ref().expect("layouts diff present");
    assert!(!layouts.removed.is_empty(), "layouts: removed not exercised");
    assert!(!layouts.added.is_empty(), "layouts: added not exercised");
    let layout_mod = layouts.modified.iter().find(|l| l.key == "keepLayout").expect("keepLayout modified");
    assert_eq!(layout_mod.diff.master_id, Some("keep".to_string()));

    // a -> b (sweep_a len 3, sweep_b len 2): exercises `removed` (the dropped `toDropSlide`,
    // index 2) + `modified[0]` (nested shapes modified+added, nested notes added, layout_id
    // tri-state Some(Some(_))) -- per the fixtures' own doc comment, a single same-direction
    // `between()` on an index-keyed collection can't show both `removed` AND `added` at once.
    let slides = diff_ab.slides.as_ref().expect("slides diff present");
    assert!(!slides.removed.is_empty(), "slides: removed not exercised");
    assert_eq!(slides.modified.len(), 1);
    let slide_mod = &slides.modified[0].diff;
    assert_eq!(slide_mod.layout_id, Some(Some("keepLayout".to_string())), "layout_id tri-state Some(Some(_)) not exercised");
    let shapes = slide_mod.shapes.as_ref().expect("shapes diff present");
    assert!(!shapes.modified.is_empty(), "shapes: modified not exercised");
    assert!(!shapes.added.is_empty(), "shapes: added (Picture) not exercised");
    let notes = slide_mod.notes.as_ref().expect("notes diff present");
    assert!(!notes.modified.is_empty() || !notes.added.is_empty(), "notes: not exercised");

    // b -> a: exercises the OTHER direction's `added` (the very same dropped `toDropSlide`,
    // carried whole as the added item's payload) + the layout_id tri-state's OTHER leg,
    // Some(None) (clearing `toModifySlide`'s layout_id back to what `sweep_a` has).
    let slides_ba = diff_ba.slides.as_ref().expect("slides diff (b->a) present");
    assert!(!slides_ba.added.is_empty(), "slides (b->a): added not exercised");
    assert_eq!(slides_ba.added[0].item, a.slides.iter().find(|s| s.id == "toDropSlide").unwrap().clone());
    let to_modify_index_in_b = b.slides.iter().position(|s| s.id == "toModifySlide").expect("present in b");
    let modified_entry = slides_ba.modified.iter().find(|m| m.index == to_modify_index_in_b).expect("toModifySlide modified b->a");
    assert_eq!(modified_entry.diff.layout_id, Some(None), "layout_id tri-state Some(None) not exercised on the reverse direction");
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let mutations = vec![
        SemioPresentationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide {
            index: 1,
            slide: Slide {
                id: "new".into(),
                layout_id: Some("layout1".into()),
                shapes: vec![SlideShape::Table { frame: frame(0.0, 0.0, 1.0, 1.0), rows: vec![SlideTableRow { cells: vec![SlideTableCell { blocks: vec![text_block("cell")] }] }] }],
                notes: Vec::new(),
            },
        }),
        SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }),
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: 0, layout_id: Some("other".into()) }),
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: 0, layout_id: None }),
        SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index: 1, notes: vec![text_block("hello world")] }),
        SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 0, shape: SlideShape::Placeholder { frame: frame(0.0, 0.0, 1.0, 1.0), kind: PlaceholderKind::Other { value: "custom".into() } } }),
        SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index: 0, shape_index: 0 }),
        SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index: 0, shape_index: 0, frame: frame(1.5, 2.5, 3.5, 4.5) }),
        SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks {
            slide_index: 0,
            shape_index: 0,
            blocks: vec![text_block("changed"), DocBlock::Heading { level: 1, style_id: Some("s".into()), runs: vec![DocRun { text: "h".into(), style: Default::default() }] }],
        }),
        SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: SlideMaster { id: "m2".into(), shapes: Vec::new() } }),
        SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id: "master1".into() }),
        SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout: SlideLayout { id: "l2".into(), master_id: "master1".into(), shapes: Vec::new() } }),
        SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id: "layout1".into() }),
        SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id: "layout1".into(), master_id: "master1".into() }),
    ];
    for mutation in mutations {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioPresentationMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = SemioPresentationMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw
