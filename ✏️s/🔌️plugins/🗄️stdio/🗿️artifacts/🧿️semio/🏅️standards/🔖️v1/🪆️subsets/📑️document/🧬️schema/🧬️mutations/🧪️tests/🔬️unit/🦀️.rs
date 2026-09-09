use super::*;
use crate::standards::v1::subsets::document::schema::diff::DocBlockDiff as TestDocBlockDiff;
use crate::standards::v1::subsets::document::schema::snapshot::{DocListItem, DocTableCell, DocTableRow};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: "s.stdio.semio.document".into(),
        styles: vec![DocStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }],
        images: Vec::new(),
        blocks: vec![DocBlock::paragraph("first"), DocBlock::paragraph("second")],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn table_path(block_index: usize, row: usize, cell: usize, index: usize) -> DocBlockPath {
    DocBlockPath { segments: vec![DocPathSegment::TableCell { block_index, row, cell }], index }
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_block_apply_and_inverse() {
    let base = fixture();
    let insert = SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(1), block: DocBlock::paragraph("inserted") });
    let mut after = base.clone();
    apply_semio_document_mutation(&mut after, &insert);
    assert_eq!(after.blocks.len(), 3);
    assert_eq!(after.blocks[1], DocBlock::paragraph("inserted"));

    let inverses = Mutation::inverse(&insert, &base);
    let mut restored = after.clone();
    for inv in &inverses {
        apply_semio_document_mutation(&mut restored, inv);
    }
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn nested_quote_and_list_path_addressing_apply_and_inverse() {
    let mut base = fixture();
    base.blocks.push(DocBlock::Quote { blocks: vec![DocBlock::paragraph("quoted")] });
    base.blocks.push(DocBlock::List { ordered: false, items: vec![DocListItem { blocks: vec![DocBlock::paragraph("item")] }] });

    let quote_path = DocBlockPath { segments: vec![DocPathSegment::Quote { block_index: 2 }], index: 0 };
    let mutation = SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: quote_path.clone(), run_index: 0, text: "changed quote".into() });
    let mut after = base.clone();
    apply_semio_document_mutation(&mut after, &mutation);
    let DocBlock::Quote { blocks } = &after.blocks[2] else { panic!("quote") };
    let DocBlock::Paragraph { runs, .. } = &blocks[0] else { panic!("paragraph") };
    assert_eq!(runs[0].text, "changed quote");
    for inv in Mutation::inverse(&mutation, &base) {
        apply_semio_document_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let list_path = DocBlockPath { segments: vec![DocPathSegment::ListItem { block_index: 3, item: 0 }], index: 0 };
    let list_mutation = SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: list_path, run_index: 0, text: "changed item".into() });
    let mut after2 = base.clone();
    apply_semio_document_mutation(&mut after2, &list_mutation);
    let DocBlock::List { items, .. } = &after2.blocks[3] else { panic!("list") };
    let DocBlock::Paragraph { runs, .. } = &items[0].blocks[0] else { panic!("paragraph") };
    assert_eq!(runs[0].text, "changed item");
    for inv in Mutation::inverse(&list_mutation, &base) {
        apply_semio_document_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);
}

#[semio_framework_async_macros::async_test]
async fn table_path_addressing_sets_nested_cell_content() {
    let mut base = fixture();
    base.blocks.push(DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] });
    let path = table_path(2, 0, 0, 0);
    let mutation = SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path: path.clone(), block: DocBlock::paragraph("changed cell") });
    let mut after = base.clone();
    apply_semio_document_mutation(&mut after, &mutation);
    let DocBlock::Table { rows } = &after.blocks[2] else { panic!("table") };
    assert_eq!(rows[0].cells[0].blocks[0], DocBlock::paragraph("changed cell"));
    for inv in Mutation::inverse(&mutation, &base) {
        apply_semio_document_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn style_and_image_mutations_apply_and_inverse() {
    let base = fixture();
    let insert = SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: DocStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) } });
    let mut after = base.clone();
    apply_semio_document_mutation(&mut after, &insert);
    assert_eq!(after.styles.len(), 2);
    for inv in Mutation::inverse(&insert, &base) {
        apply_semio_document_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let insert_img = SemioDocumentMutation::InsertImage(insert_image::InsertImage { image: DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2] } });
    let mut with_img = base.clone();
    apply_semio_document_mutation(&mut with_img, &insert_img);
    assert_eq!(with_img.images.len(), 1);
    let set_bytes = SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id: "img1".into(), mime: "image/jpeg".into(), bytes: vec![9] });
    let mut after2 = with_img.clone();
    apply_semio_document_mutation(&mut after2, &set_bytes);
    assert_eq!(after2.images[0].mime, "image/jpeg");
    for inv in Mutation::inverse(&set_bytes, &with_img) {
        apply_semio_document_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, with_img);
}

//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling the binary op
/// frame's `tag` ordinal and the text grammar's keyword both use, and every one of those
/// spellings must also appear in the committed oracle manifest's catalog. The framework never
/// parses Rust, so this is what makes the declaration honest.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    assert_eq!(KINDS.len(), 17, "KINDS must name exactly one entry per declared SemioDocumentMutation variant");
    let mut seen = vec![false; KINDS.len()];
    for m in demo_mutation_cases() {
        let keyword = print_document_mutation(&m).split(' ').next().expect("printed op is never empty").to_string();
        let ordinal = variant_ordinal(&m) as usize;
        assert_eq!(KINDS[ordinal], keyword, "KINDS must match the declaration order and spelling for {m:?}");
        seen[ordinal] = true;
    }
    assert!(seen.iter().all(|hit| *hit), "demo_mutation_cases must reach every KINDS entry, missing {:?}", KINDS.iter().zip(seen.iter()).filter(|(_, hit)| !**hit).map(|(kind, _)| *kind).collect::<Vec<_>>());
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog

//#region 🔖️Fixtures
/// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field. `blocks` uses different-length lists
/// so a single same-direction `between()` shows removed+modified+added simultaneously; the
/// modified paragraph exercises `style_id`'s tri-state AND nested `runs` modified+added; the
/// reverse direction (asserted in `field_sweep`) exercises `blocks.added` carrying a whole
/// recursively-structured `Table`. `styles`/`images` each get one removed, one
/// modified-in-every-field, one added.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: "s.stdio.semio.document".into(),
        styles: vec![DocStyle { id: "keep".into(), name: "Keep".into(), based_on: None }, DocStyle { id: "toModify".into(), name: "old".into(), based_on: None }, DocStyle { id: "toRemove".into(), name: "Gone".into(), based_on: None }],
        images: vec![DocImage { id: "toModify".into(), mime: "image/png".into(), bytes: vec![1] }, DocImage { id: "toRemove".into(), mime: "image/gif".into(), bytes: vec![2] }],
        blocks: vec![
            DocBlock::Paragraph { style_id: None, runs: vec![DocRun { text: "old".into(), style: RunStyle { bold: false, ..Default::default() } }] },
            DocBlock::paragraph("stay"),
            DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("toDrop cell")] }] }] },
        ],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: "s.stdio.semio.document".into(),
        styles: vec![DocStyle { id: "keep".into(), name: "Keep".into(), based_on: None }, DocStyle { id: "toModify".into(), name: "new".into(), based_on: Some("keep".into()) }, DocStyle { id: "added".into(), name: "Added".into(), based_on: None }],
        images: vec![DocImage { id: "toModify".into(), mime: "image/jpeg".into(), bytes: vec![9, 9] }, DocImage { id: "added".into(), mime: "image/webp".into(), bytes: vec![3] }],
        blocks: vec![DocBlock::Paragraph { style_id: Some("keep".into()), runs: vec![DocRun { text: "new".into(), style: RunStyle { bold: true, ..Default::default() } }, DocRun::plain("second run")] }, DocBlock::paragraph("stay")],
    }
}
//#endregion 🔖️Fixtures

//#region 🔖️MutationDiffLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_mutations() -> Vec<SemioDocumentMutation> {
    vec![
        SemioDocumentMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(1), block: DocBlock::paragraph("x") }),
        SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: DocBlockPath::top(0) }),
        SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path: DocBlockPath::top(0), block: DocBlock::paragraph("y") }),
        SemioDocumentMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { path: DocBlockPath::top(0), style_id: Some("Normal".into()) }),
        SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: DocBlockPath::top(0), run_index: 0, text: "z".into() }),
        SemioDocumentMutation::SetRunStyle(set_run_style::SetRunStyle { path: DocBlockPath::top(0), run_index: 0, style: RunStyle { bold: true, italic: false, underline: true, ..Default::default() } }),
        SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: DocStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: None } }),
        SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id: "Normal".into() }),
        SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id: "Normal".into(), name: "Body".into() }),
        SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Heading1".into()) }),
        SemioDocumentMutation::InsertImage(insert_image::InsertImage { image: DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1] } }),
        SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id: "img1".into() }),
    ]
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn apply_valid(diff: &SemioDocumentDiff, base: &SemioDocumentSnapshot) -> SemioDocumentSnapshot {
    MutationDiff::apply(diff, base).expect("valid Semio document diff fixture")
}

/// 🖼️ Supplies the existing image required by the removal law's starting document.
fn law_fixture(mutation: &SemioDocumentMutation) -> SemioDocumentSnapshot {
    let mut base = fixture();
    if matches!(mutation, SemioDocumentMutation::RemoveImage(_)) {
        base.images.push(DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1] });
    }
    base
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in sample_mutations() {
        let base = law_fixture(&mutation);
        let diff_direct = Mutation::diff(&mutation, &base);
        let applied_via_diff = apply_valid(diff_direct.diff(), &base);

        let mut via_apply = base.clone();
        let diff_from_apply = apply_semio_document_mutation(&mut via_apply, &mutation);

        assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
        assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    for mutation in sample_mutations() {
        let base = law_fixture(&mutation);

        let mut round_tripped = base.clone();
        apply_semio_document_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::inverse(&mutation, &base) {
            apply_semio_document_mutation(&mut round_tripped, &inverse_mutation);
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
fn assert_absorb_matches_sequential(base: &SemioDocumentSnapshot, d1: &SemioDocumentDiff, d2: &SemioDocumentDiff) -> SemioDocumentDiff {
    let sequential = apply_valid(d2, &apply_valid(d1, base));
    let mut absorbed = d1.clone();
    MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(apply_valid(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn blocks_diff(diff: &SemioDocumentDiff) -> &BlocksDiff {
    diff.blocks.as_ref().expect("blocks diff present")
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
    {
        let base = fixture();
        let d1 = Mutation::diff(&SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("f") }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: DocBlockPath::top(0) }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = blocks_diff(&absorbed);
        assert_eq!(triple.removed, vec![0]);
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].index, 1);
        assert_eq!(triple.added[0].item, DocBlock::paragraph("f"));
    }

    // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
    {
        let base = fixture();
        let d1 = Mutation::diff(&SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("f") }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("g") }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = blocks_diff(&absorbed);
        assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
        assert!(triple.added.iter().any(|a| a.item == DocBlock::paragraph("f")));
        assert!(triple.added.iter().any(|a| a.item == DocBlock::paragraph("g")));
    }

    // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
    {
        let base = fixture();
        let d1 = Mutation::diff(&SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(1), block: DocBlock::paragraph("f") }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: DocBlockPath::top(1), run_index: 0, text: "patched".into() }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = blocks_diff(&absorbed);
        assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].item, DocBlock::paragraph("patched"));
    }

    // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
    {
        let base = fixture();
        let d1 = Mutation::diff(&SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: DocBlockPath::top(1), run_index: 0, text: "patched".into() }), &base);
        let mid = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: DocBlockPath::top(1) }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = blocks_diff(&absorbed);
        assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
        assert_eq!(triple.removed, vec![1]);
    }

    // Associativity over a triple.
    {
        let base = fixture();
        let d1 = Mutation::diff(&SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("f") }), &base);
        let mid1 = apply_valid(d1.diff(), &base);
        let d2 = Mutation::diff(&SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(2), block: DocBlock::paragraph("g") }), &mid1);
        let mid2 = apply_valid(d2.diff(), &mid1);
        let d3 = Mutation::diff(&SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: DocBlockPath::top(0) }), &mid2);
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
    assert_eq!(apply_valid(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&a, &b), &a), b);
    assert_eq!(apply_valid(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&b, &a), &b), a);

    let sample = fixture();
    assert_eq!(apply_valid(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&sample, &sample), &sample), sample);

    let mut mutated = sample.clone();
    apply_semio_document_mutation(&mut mutated, &SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: DocBlockPath::top(0), run_index: 0, text: "Chapter Two".into() }));
    assert_ne!(sample, mutated);
    assert_eq!(apply_valid(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&sample, &mutated), &sample), mutated);
    assert_eq!(apply_valid(&<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&mutated, &sample), &mutated), sample);
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️CodecRetentionLaw
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = sweep_b();
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
//#endregion 🔖️CodecRetentionLaw

//#region 🔖️FieldSweep
/// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field (see the
/// fixtures' doc comment for exactly how each collection flavor -- removed/modified/added --
/// is exercised).
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&a, &b);
    assert_eq!(apply_valid(&diff_ab, &a), b);
    let diff_ba = <SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&b, &a);
    assert_eq!(apply_valid(&diff_ba, &b), a);
    assert!(<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(&a, &a).is_empty());

    let styles_diff = diff_ab.styles.as_ref().expect("styles diff present");
    assert!(!styles_diff.removed.is_empty(), "styles: removed not exercised");
    assert!(!styles_diff.added.is_empty(), "styles: added not exercised");
    let style_mod = styles_diff.modified.iter().find(|m| m.key == "toModify").expect("toModify style modified");
    assert!(style_mod.diff.name.is_some());
    assert_eq!(style_mod.diff.based_on, Some(Some("keep".to_string())), "style based_on tri-state Some(Some(_)) not exercised");

    let images_diff = diff_ab.images.as_ref().expect("images diff present");
    assert!(!images_diff.removed.is_empty(), "images: removed not exercised");
    assert!(!images_diff.added.is_empty(), "images: added not exercised");
    let image_mod = images_diff.modified.iter().find(|m| m.key == "toModify").expect("toModify image modified");
    assert!(image_mod.diff.mime.is_some() && image_mod.diff.bytes.is_some());

    let body_diff = diff_ab.blocks.as_ref().expect("blocks diff present");
    assert!(!body_diff.removed.is_empty(), "blocks: removed not exercised");
    assert_eq!(body_diff.modified.len(), 1);
    let TestDocBlockDiff::Paragraph(p_diff) = &body_diff.modified[0].diff else { panic!("expected paragraph diff") };
    let runs_diff = p_diff.runs.as_ref().expect("modified paragraph: runs not exercised");
    assert_eq!(p_diff.style_id, Some(Some("keep".to_string())), "modified paragraph: style_id tri-state Some(Some(_)) not exercised");
    assert!(!runs_diff.modified.is_empty(), "modified paragraph: runs.modified not exercised");
    let run_diff = &runs_diff.modified[0].diff;
    assert!(run_diff.text.is_some(), "modified run: text not exercised");
    let style_diff = run_diff.style.as_ref().expect("modified run: style not exercised");
    assert!(style_diff.bold.is_some(), "modified run style: bold not exercised");
    assert!(!runs_diff.added.is_empty(), "modified paragraph: runs.added (nested) not exercised");

    let body_diff_ba = diff_ba.blocks.as_ref().expect("blocks diff (b->a) present");
    assert!(!body_diff_ba.added.is_empty(), "blocks (b->a): added not exercised");
    let DocBlock::Table { rows } = &body_diff_ba.added[0].item else { panic!("expected added table") };
    assert!(!rows.is_empty());

    // Some(None) tri-state coverage: style based_on cleared going the OTHER direction.
    let style_mod_ba = diff_ba.styles.as_ref().unwrap().modified.iter().find(|m| m.key == "toModify").expect("toModify present in b->a");
    assert_eq!(style_mod_ba.diff.based_on, Some(None), "style based_on tri-state Some(None) not exercised");
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ `OpText`/`OpBinary` round-trip laws for the hand-rolled `SemioDocumentMutation`
/// grammar -- exercises every variant, incl. `InsertBlock`'s bare `DocBlock` payload (a
/// `Table` carrying nested rows/cells/blocks), `SetSnapshot`'s whole snapshot, and every
/// `Option`/tri-state field.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let table_block = DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] };
    let mutations = vec![
        SemioDocumentMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(1), block: table_block.clone() }),
        SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: table_path(0, 0, 0, 0), block: DocBlock::paragraph("nested") }),
        SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: DocBlockPath::top(0) }),
        SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path: DocBlockPath::top(0), block: table_block }),
        SemioDocumentMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { path: DocBlockPath::top(0), style_id: None }),
        SemioDocumentMutation::SetHeadingLevel(set_heading_level::SetHeadingLevel { path: DocBlockPath::top(0), level: 2 }),
        SemioDocumentMutation::SetListOrdered(set_list_ordered::SetListOrdered { path: DocBlockPath::top(0), ordered: true }),
        SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: DocBlockPath::top(0), run_index: 0, text: "hello world".into() }),
        SemioDocumentMutation::SetRunStyle(set_run_style::SetRunStyle { path: DocBlockPath::top(0), run_index: 0, style: RunStyle { bold: true, size: Some(12.0), font: Some("Arial".into()), ..Default::default() } }),
        SemioDocumentMutation::SetImageBlock(set_image_block::SetImageBlock { path: DocBlockPath::top(0), image_id: "img1".into(), alt: "alt".into(), width: Some(10.0), height: None }),
        SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: DocStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) } }),
        SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id: "Normal".into() }),
        SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id: "Normal".into(), name: "Body Text".into() }),
        SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Other".into()) }),
        SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: None }),
        SemioDocumentMutation::InsertImage(insert_image::InsertImage { image: DocImage { id: "img2".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] } }),
        SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id: "img2".into() }),
        SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id: "img1".into(), mime: "image/gif".into(), bytes: vec![7] }),
    ];
    for mutation in mutations {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioDocumentMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = SemioDocumentMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw
