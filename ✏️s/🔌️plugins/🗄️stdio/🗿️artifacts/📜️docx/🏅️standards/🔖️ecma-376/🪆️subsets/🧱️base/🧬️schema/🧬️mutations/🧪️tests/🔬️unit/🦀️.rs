use super::*;
use crate::schema::diff::{DocxBlockDiff, DocxOpcPartDiff};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_block_apply_and_inverse() {
    let base = fixture();
    let insert = DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 1 }, block: DocxBlock::paragraph("inserted") });
    let mut after = base.clone();
    apply_docx_mutation(&mut after, &insert);
    assert_eq!(after.document.body.len(), 3);
    assert_eq!(after.document.body[1], DocxBlock::paragraph("inserted"));

    let inverses = Mutation::inverse(&insert, &base);
    let mut restored = after.clone();
    for inv in &inverses {
        apply_docx_mutation(&mut restored, inv);
    }
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn remove_block_inverse_restores_removed_block() {
    let base = fixture();
    let remove = DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 0 } });
    let mut after = base.clone();
    apply_docx_mutation(&mut after, &remove);
    assert_eq!(after.document.body.len(), 1);
    for inv in Mutation::inverse(&remove, &base) {
        apply_docx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn set_run_text_and_formatting_apply_and_inverse() {
    let base = fixture();
    let mutation = DocxMutation::SetRunText(set_run_text::SetRunText { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, text: "changed".into() });
    let mut after = base.clone();
    apply_docx_mutation(&mut after, &mutation);
    let DocxBlock::Paragraph(p) = &after.document.body[0] else { panic!("paragraph") };
    assert_eq!(p.runs[0].text, "changed");
    for inv in Mutation::inverse(&mutation, &base) {
        apply_docx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let fmt = DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, bold: true, italic: true, underline: true });
    let mut after2 = base.clone();
    apply_docx_mutation(&mut after2, &fmt);
    let DocxBlock::Paragraph(p2) = &after2.document.body[0] else { panic!("paragraph") };
    assert!(p2.runs[0].bold && p2.runs[0].italic && p2.runs[0].underline);
    for inv in Mutation::inverse(&fmt, &base) {
        apply_docx_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);
}

#[semio_framework_async_macros::async_test]
async fn table_path_addressing_sets_nested_cell_content() {
    let mut base = fixture();
    base.document.body.push(DocxBlock::Table(DocxTable { rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("cell")], ..Default::default() }], ..Default::default() }], ..Default::default() }));
    let path = table_path(2, 0, 0, 0);
    let mutation = DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path: path.clone(), block: DocxBlock::paragraph("changed cell") });
    let mut after = base.clone();
    apply_docx_mutation(&mut after, &mutation);
    let DocxBlock::Table(t) = &after.document.body[2] else { panic!("table") };
    assert_eq!(t.rows[0].cells[0].blocks[0], DocxBlock::paragraph("changed cell"));
    for inv in Mutation::inverse(&mutation, &base) {
        apply_docx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
    let _ = &mut base; // silence unused-mut if the push above is later removed
}

#[semio_framework_async_macros::async_test]
async fn style_mutations_apply_and_inverse() {
    let base = fixture();
    let insert = DocxMutation::InsertStyle(insert_style::InsertStyle { style: DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) } });
    let mut after = base.clone();
    apply_docx_mutation(&mut after, &insert);
    assert_eq!(after.document.styles.len(), 2);
    for inv in Mutation::inverse(&insert, &base) {
        apply_docx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let rename = DocxMutation::SetStyleName(set_style_name::SetStyleName { id: "Normal".into(), name: "Body Text".into() });
    let mut after2 = base.clone();
    apply_docx_mutation(&mut after2, &rename);
    assert_eq!(after2.document.styles[0].name, "Body Text");
    for inv in Mutation::inverse(&rename, &base) {
        apply_docx_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);

    let based_on = DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Other".into()) });
    let mut after3 = base.clone();
    apply_docx_mutation(&mut after3, &based_on);
    assert_eq!(after3.document.styles[0].based_on, Some("Other".into()));
    for inv in Mutation::inverse(&based_on, &base) {
        apply_docx_mutation(&mut after3, &inv);
    }
    assert_eq!(after3, base);
}

#[semio_framework_async_macros::async_test]
async fn opc_part_mutations_apply_and_inverse() {
    let base = fixture();
    let set = DocxMutation::SetPart(set_part::SetPart { path: "word/numbering.xml".into(), content_type: "application/xml".into(), bytes: b"<w:numbering/>".to_vec() });
    let mut after = base.clone();
    apply_docx_mutation(&mut after, &set);
    assert_eq!(after.opc.part_bytes("word/numbering.xml"), Some(b"<w:numbering/>".as_slice()));
    for inv in Mutation::inverse(&set, &base) {
        apply_docx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let mut with_part = base.clone();
    apply_docx_mutation(&mut with_part, &set);
    let remove = DocxMutation::RemovePart(remove_part::RemovePart { path: "word/numbering.xml".into() });
    let mut after2 = with_part.clone();
    apply_docx_mutation(&mut after2, &remove);
    assert_eq!(after2.opc.part_bytes("word/numbering.xml"), None);
    for inv in Mutation::inverse(&remove, &with_part) {
        apply_docx_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, with_part);
}

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in demo_mutation_cases() {
        let base = fixture();
        let diff_direct = Mutation::diff(&mutation, &base);
        let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).unwrap();

        let mut via_apply = base.clone();
        let diff_from_apply = apply_docx_mutation(&mut via_apply, &mutation);

        assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
        assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    for mutation in demo_mutation_cases() {
        let base = fixture();

        let mut round_tripped = base.clone();
        apply_docx_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in <DocxMutation as Mutation<DocxSnapshot>>::inverse(&mutation, &base) {
            apply_docx_mutation(&mut round_tripped, &inverse_mutation);
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
fn assert_absorb_matches_sequential(base: &DocxSnapshot, d1: &DocxDiff, d2: &DocxDiff) -> DocxDiff {
    let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base).unwrap()).unwrap();
    let mut absorbed = d1.clone();
    MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(MutationDiff::apply(&absorbed, base).unwrap(), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn body_diff(diff: &DocxDiff) -> &crate::schema::diff::DocxBlocksDiff {
    diff.document.as_ref().expect("document diff present").body.as_ref().expect("body diff present")
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
    {
        let base = fixture();
        let d1 = Mutation::diff(&DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("f") }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 0 } }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = body_diff(&absorbed);
        assert_eq!(triple.removed, vec![0]);
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].index, 1);
        assert_eq!(triple.added[0].item, DocxBlock::paragraph("f"));
    }

    // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
    {
        let base = fixture();
        let d1 = Mutation::diff(&DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("f") }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("g") }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = body_diff(&absorbed);
        assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
        assert!(triple.added.iter().any(|a| a.item == DocxBlock::paragraph("f")));
        assert!(triple.added.iter().any(|a| a.item == DocxBlock::paragraph("g")));
    }

    // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
    {
        let base = fixture();
        let d1 = Mutation::diff(&DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 1 }, block: DocxBlock::paragraph("f") }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&DocxMutation::SetRunText(set_run_text::SetRunText { path: DocxBlockPath { segments: vec![], index: 1 }, run_index: 0, text: "patched".into() }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = body_diff(&absorbed);
        assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].item, DocxBlock::paragraph("patched"));
    }

    // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
    {
        let base = fixture();
        let d1 = Mutation::diff(&DocxMutation::SetRunText(set_run_text::SetRunText { path: DocxBlockPath { segments: vec![], index: 1 }, run_index: 0, text: "patched".into() }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 1 } }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = body_diff(&absorbed);
        assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
        assert_eq!(triple.removed, vec![1]);
    }

    // Associativity over a triple.
    {
        let base = fixture();
        let d1 = Mutation::diff(&DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("f") }), &base);
        let mid1 = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("g") }), &mid1);
        let mid2 = MutationDiff::apply(d2.diff(), &mid1).unwrap();
        let d3 = Mutation::diff(&DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 0 } }), &mid2);
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
    assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&a, &b), &a).unwrap(), b);
    assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&b, &a), &b).unwrap(), a);

    let sample = fixture();
    assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&sample, &sample), &sample).unwrap(), sample);

    // "Real" fixture leg: a realistic multi-paragraph document diffed against a mutated variant.
    let real = crate::engine::build_minimal_docx(DocxDocument { body: vec![DocxBlock::paragraph("Chapter One"), DocxBlock::paragraph("Body text goes here.")], styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }] });
    let mut mutated = real.clone();
    apply_docx_mutation(&mut mutated, &DocxMutation::SetRunText(set_run_text::SetRunText { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, text: "Chapter Two".into() }));
    assert_ne!(real, mutated);
    assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&real, &mutated), &real).unwrap(), mutated);
    assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&mutated, &real), &mutated).unwrap(), real);
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️CodecRetentionLaw
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = crate::engine::build_minimal_docx(DocxDocument {
        body: vec![DocxBlock::Paragraph(DocxParagraph {
            runs: vec![DocxRun { text: "Hello".into(), bold: true, italic: true, underline: true, extra_run_properties: Vec::new() }],
            style: Some("Normal".into()),
            extra_paragraph_properties: Vec::new(),
        })],
        styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }],
    });
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <DocxSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
//#endregion 🔖️CodecRetentionLaw

//#region 🔖️FieldSweep
/// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field across BOTH
/// `opc` and `document` (see the fixtures' doc comment for exactly how each collection flavor
/// -- removed/modified/added -- is exercised, and this ticket's "known structural trap" note
/// for why the two snapshots use different-length body lists rather than a single same-length
/// pairwise collection).
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&a, &b);
    assert_eq!(MutationDiff::apply(&diff_ab, &a).unwrap(), b);
    let diff_ba = <DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&b, &a);
    assert_eq!(MutationDiff::apply(&diff_ba, &b).unwrap(), a);
    assert!(<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&a, &a).is_empty());

    // opc: content_types (both defaults+overrides), parts, relationships all populated.
    let opc_diff = diff_ab.opc.as_ref().expect("opc diff present");
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
    assert!(matches!(&part_mod.diff, DocxOpcPartDiff { bytes: Some(_), .. }));
    let rels = opc_diff.relationships.as_ref().expect("relationships diff present");
    assert!(!rels.removed.is_empty(), "opc.relationships: removed (owner) not exercised");
    assert!(!rels.modified.is_empty(), "opc.relationships: modified (owner) not exercised");
    assert!(!rels.added.is_empty(), "opc.relationships: added (owner) not exercised");

    // document.body: `a -> b` exercises removed (top) + modified-with-nested-runs-added (per
    // the "known structural trap" note, one same-direction `between()` can't show BOTH a
    // top-level removed AND a top-level added -- see `sweep_b`'s doc comment).
    let doc_diff = diff_ab.document.as_ref().expect("document diff present");
    let body_diff = doc_diff.body.as_ref().expect("body diff present");
    assert!(!body_diff.removed.is_empty(), "body: removed not exercised");
    assert_eq!(body_diff.modified.len(), 1);
    let DocxBlockDiff::Paragraph(p_diff) = &body_diff.modified[0].diff else { panic!("expected paragraph diff") };
    let runs_diff = p_diff.runs.as_ref().expect("modified paragraph: runs not exercised");
    assert_eq!(p_diff.style, Some(Some("keep".to_string())), "modified paragraph: style tri-state Some(Some(_)) not exercised");
    assert!(!runs_diff.modified.is_empty(), "modified paragraph: runs.modified not exercised");
    let run_diff = &runs_diff.modified[0].diff;
    assert!(run_diff.text.is_some() && run_diff.bold.is_some(), "modified run: text/bold not exercised");
    assert!(!runs_diff.added.is_empty(), "modified paragraph: runs.added (nested) not exercised");

    // `b -> a` exercises the OTHER direction's top-level `added` (the very same dropped
    // `Table`, carried whole as the added item's payload, recursively structured).
    let body_diff_ba = diff_ba.document.as_ref().unwrap().body.as_ref().expect("body diff (b->a) present");
    assert!(!body_diff_ba.added.is_empty(), "body (b->a): added not exercised");
    let DocxBlock::Table(added_table) = &body_diff_ba.added[0].item else { panic!("expected added table") };
    assert!(!added_table.rows.is_empty());

    // document.styles: removed+modified(name+based_on tri-state)+added.
    let styles_diff = doc_diff.styles.as_ref().expect("styles diff present");
    assert!(!styles_diff.removed.is_empty(), "styles: removed not exercised");
    assert!(!styles_diff.added.is_empty(), "styles: added not exercised");
    let style_mod = styles_diff.modified.iter().find(|m| m.key == "toModify").expect("toModify style modified");
    assert!(style_mod.diff.name.is_some());
    assert_eq!(style_mod.diff.based_on, Some(Some("keep".to_string())), "style based_on tri-state Some(Some(_)) not exercised");

    // Some(None) tri-state coverage: style based_on cleared, going the OTHER direction (b -> a
    // clears "toModify"'s based_on since a's copy has based_on: None).
    let style_mod_ba = diff_ba.document.as_ref().unwrap().styles.as_ref().unwrap().modified.iter().find(|m| m.key == "toModify").expect("toModify present in b->a");
    assert_eq!(style_mod_ba.diff.based_on, Some(None), "style based_on tri-state Some(None) not exercised");
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `DocxMutation` grammar —
/// exercises every variant, incl. `InsertBlock`/`SetBlockContent`'s bare `DocxBlock` payload
/// (a `Table` carrying nested rows/cells/blocks), `SetSnapshot`'s whole `DocxSnapshot` (OPC
/// parts/content-types/relationships-by-owner plus the typed document/styles), and the
/// `Option<String>` tri-state on `SetStyleBasedOn`.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let table_block = DocxBlock::Table(DocxTable { rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("cell")], ..Default::default() }], ..Default::default() }], ..Default::default() });
    let mutations = vec![
        DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 1 }, block: table_block.clone() }),
        DocxMutation::InsertBlock(insert_block::InsertBlock { path: table_path(0, 0, 0, 0), block: DocxBlock::paragraph("nested") }),
        DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 0 } }),
        DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path: DocxBlockPath { segments: vec![], index: 0 }, block: table_block }),
        DocxMutation::SetRunText(set_run_text::SetRunText { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, text: "hello world".into() }),
        DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, bold: true, italic: false, underline: true }),
        DocxMutation::InsertStyle(insert_style::InsertStyle { style: DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) } }),
        DocxMutation::RemoveStyle(remove_style::RemoveStyle { id: "Normal".into() }),
        DocxMutation::SetStyleName(set_style_name::SetStyleName { id: "Normal".into(), name: "Body Text".into() }),
        DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Other".into()) }),
        DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: None }),
        DocxMutation::SetPart(set_part::SetPart { path: "word/numbering.xml".into(), content_type: "application/xml".into(), bytes: b"<w:numbering/>".to_vec() }),
        DocxMutation::RemovePart(remove_part::RemovePart { path: "word/styles.xml".into() }),
    ];
    for mutation in mutations {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = DocxMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = DocxMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw

//#region kinds_law
/// 🧪️ Keeps `KINDS` honest against the enum it claims to spell: every variant's
/// `print_docx_mutation` keyword, in the SAME declaration order `demo_mutation_cases()` already
/// carries (one instance per variant), must equal `KINDS` entry-for-entry -- the framework never
/// parses Rust to check this itself (see `KINDS`'s own doc comment), so this test is the one
/// thing that does. `KINDS` is also kept textually identical, by hand, to
/// `../🔣️oracle.json`'s own `kinds` array.
#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let cases = demo_mutation_cases();
    assert_eq!(cases.len(), KINDS.len(), "demo_mutation_cases() must cover every KINDS entry exactly once");
    for (mutation, kind) in cases.iter().zip(KINDS.iter()) {
        let printed = mutation.print_op();
        let keyword = printed.split(' ').next().unwrap_or(&printed);
        assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
    }
}
//#endregion kinds_law
