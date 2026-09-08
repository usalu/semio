
use super::*;
use crate::schema::diff::{XlsxCellDiff, XlsxOpcPartDiff};
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_sheet_apply_and_inverse() {
    let base = fixture();
    let insert = XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "New".into(), cells: vec![] } });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &insert);
    assert_eq!(after.workbook.sheets.len(), 3);
    assert!(after.workbook.sheets.iter().any(|s| s.name == "New"));

    for inv in Mutation::inverse(&insert, &base) {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn remove_sheet_inverse_restores_removed_sheet() {
    // 🎯️ Targets `"Sheet2"`, the LAST sheet in `fixture()` — like docx's own `RemovePart`
    // precedent (see that artifact's `sample_mutations` doc comment), `sheets` is a
    // NAME-keyed collection (position carries no spec meaning), so `RemoveSheet`'s
    // mutation-level inverse (`InsertSheet`, which always APPENDS) only restores the exact
    // original Vec position when the removed sheet was already last — exact positional
    // restoration in the general case is only guaranteed at the diff level.
    let base = fixture();
    let remove = XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &remove);
    assert_eq!(after.workbook.sheets.len(), 1);
    for inv in Mutation::inverse(&remove, &base) {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn rename_sheet_apply_and_inverse() {
    // 🎯️ Targets `"Sheet2"` (the last sheet, empty) — same last-position caveat as
    // `remove_sheet_inverse_restores_removed_sheet` above: `RenameSheet`'s diff is a
    // remove-old-name + add-new-name (name IS the sheet's identity), so its mutation-level
    // inverse only reproduces the EXACT original Vec position when the renamed sheet was
    // already last.
    let base = fixture();
    let rename = XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &rename);
    assert!(!after.workbook.sheets.iter().any(|s| s.name == "Sheet2"));
    let renamed = after.workbook.sheets.iter().find(|s| s.name == "Renamed").expect("renamed sheet present");
    assert!(renamed.cells.is_empty());
    for inv in Mutation::inverse(&rename, &base) {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn set_and_remove_cell_apply_and_inverse() {
    let base = fixture();
    let set_existing = XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Boolean(true) });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &set_existing);
    assert_eq!(after.workbook.sheets[0].cells[0].value, XlsxCellValue::Boolean(true));
    for inv in Mutation::inverse(&set_existing, &base) {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let set_new = XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 2, col: 3, value: XlsxCellValue::InlineString("fresh".into()) });
    let mut after2 = base.clone();
    apply_xlsx_mutation(&mut after2, &set_new);
    assert!(after2.workbook.sheets[0].cells.iter().any(|c| c.row == 2 && c.col == 3 && c.value == XlsxCellValue::InlineString("fresh".into())));
    for inv in Mutation::inverse(&set_new, &base) {
        apply_xlsx_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);

    let remove = XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 });
    let mut after3 = base.clone();
    apply_xlsx_mutation(&mut after3, &remove);
    assert!(after3.workbook.sheets[0].cells.is_empty());
    for inv in Mutation::inverse(&remove, &base) {
        apply_xlsx_mutation(&mut after3, &inv);
    }
    assert_eq!(after3, base);
}

#[semio_framework_async_macros::async_test]
async fn shared_string_mutations_apply_and_inverse() {
    let base = fixture();
    let insert = XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "world".into() });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &insert);
    assert_eq!(after.workbook.shared_strings, vec!["hello".to_string(), "world".to_string()]);
    for inv in Mutation::inverse(&insert, &base) {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let set = XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "changed".into() });
    let mut after2 = base.clone();
    apply_xlsx_mutation(&mut after2, &set);
    assert_eq!(after2.workbook.shared_strings, vec!["changed".to_string()]);
    for inv in Mutation::inverse(&set, &base) {
        apply_xlsx_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);

    let remove = XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 });
    let mut after3 = base.clone();
    apply_xlsx_mutation(&mut after3, &remove);
    assert!(after3.workbook.shared_strings.is_empty());
    for inv in Mutation::inverse(&remove, &base) {
        apply_xlsx_mutation(&mut after3, &inv);
    }
    assert_eq!(after3, base);
}

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in demo_mutation_cases() {
        let base = fixture();
        let diff_direct = Mutation::diff(&mutation, &base);
        let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).unwrap();

        let mut via_apply = base.clone();
        let diff_from_apply = apply_xlsx_mutation(&mut via_apply, &mutation);

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
        apply_xlsx_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in <XlsxMutation as Mutation<XlsxSnapshot>>::inverse(&mutation, &base) {
            apply_xlsx_mutation(&mut round_tripped, &inverse_mutation);
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
fn assert_absorb_matches_sequential(base: &XlsxSnapshot, d1: &XlsxDiff, d2: &XlsxDiff) -> XlsxDiff {
    let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base).unwrap()).unwrap();
    let mut absorbed = d1.clone();
    MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(MutationDiff::apply(&absorbed, base).unwrap(), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn cells_diff<'a>(diff: &'a XlsxDiff, sheet_name: &str) -> &'a crate::schema::diff::XlsxCellsDiff {
    let sheets = diff.workbook.as_ref().expect("workbook diff present").sheets.as_ref().expect("sheets diff present");
    sheets.modified.iter().find(|m| m.key == sheet_name).expect("sheet modified").diff.cells.as_ref().expect("cells diff present")
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    // Canonical: SetCell(1,0)+RemoveCell(1,0) on a fresh row -> annihilated add (mirrors
    // Insert+Remove-before): net effect is "never existed".
    {
        let base = fixture();
        let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::Number(1.0) }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet2".into(), row: 5, col: 5 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = cells_diff(&absorbed, "Sheet2");
        assert!(triple.added.is_empty(), "the add must be annihilated by the later remove");
        assert!(triple.removed.is_empty(), "a never-based cell must not appear as a base removal either");
    }

    // Canonical: SetCell(5,5,f)+SetCell(5,6,g) on distinct fresh cells -> both survive
    // (mirrors Insert+Insert-same-index-both-survive: two independent adds never LWW-clobber
    // each other).
    {
        let base = fixture();
        let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::InlineString("f".into()) }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 6, value: XlsxCellValue::InlineString("g".into()) }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = cells_diff(&absorbed, "Sheet2");
        assert_eq!(triple.added.len(), 2, "both cell adds must survive absorb");
    }

    // Canonical: SetCell(insert)+SetCell(same coord, patch) -> patch into the added payload.
    {
        let base = fixture();
        let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::InlineString("f".into()) }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::InlineString("patched".into()) }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = cells_diff(&absorbed, "Sheet2");
        assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].value, XlsxCellValue::InlineString("patched".into()));
    }

    // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
    {
        let base = fixture();
        let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Boolean(true) }), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 }), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        let triple = cells_diff(&absorbed, "Sheet1");
        assert!(triple.modified.is_empty(), "modify of a since-removed cell must not survive absorb");
        assert_eq!(triple.removed, vec![(1u32, 0u32)]);
    }

    // Associativity over a triple.
    {
        let base = fixture();
        let d1 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 5, col: 5, value: XlsxCellValue::Number(1.0) }), &base);
        let mid1 = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet2".into(), row: 6, col: 6, value: XlsxCellValue::Number(2.0) }), &mid1);
        let mid2 = MutationDiff::apply(d2.diff(), &mid1).unwrap();
        let d3 = Mutation::diff(&XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet2".into(), row: 5, col: 5 }), &mid2);
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
    assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &b), &a).unwrap(), b);
    assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&b, &a), &b).unwrap(), a);

    let sample = fixture();
    assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&sample, &sample), &sample).unwrap(), sample);

    // "Real" fixture leg: a realistic multi-sheet workbook diffed against a mutated variant.
    let real = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Data".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) }] }],
        shared_strings: vec!["Chapter One".into()],
    });
    let mut mutated = real.clone();
    apply_xlsx_mutation(&mut mutated, &XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "Chapter Two".into() }));
    assert_ne!(real, mutated);
    assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&real, &mutated), &real).unwrap(), mutated);
    assert_eq!(MutationDiff::apply(&<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&mutated, &real), &mutated).unwrap(), real);
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️CodecRetentionLaw
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet {
            name: "Sheet1".into(),
            cells: vec![
                XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) },
                XlsxCell { row: 1, col: 1, value: XlsxCellValue::Number(9.5) },
                XlsxCell { row: 2, col: 0, value: XlsxCellValue::Boolean(true) },
                XlsxCell { row: 2, col: 1, value: XlsxCellValue::InlineString("literal".into()) },
                XlsxCell { row: 3, col: 0, value: XlsxCellValue::Formula { expr: "SUM(B1:B2)".into(), cached: Some(Box::new(XlsxCellValue::Number(9.5))) } },
            ],
        }],
        shared_strings: vec!["Hello".into()],
    });
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <XlsxSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
//#endregion 🔖️CodecRetentionLaw

//#region 🔖️FieldSweep
/// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field across BOTH
/// `opc` and `workbook` (see the fixtures' doc comment for exactly how each collection flavor
/// — removed/modified/added — is exercised).
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &b);
    assert_eq!(MutationDiff::apply(&diff_ab, &a).unwrap(), b);
    let diff_ba = <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&b, &a);
    assert_eq!(MutationDiff::apply(&diff_ba, &b).unwrap(), a);
    assert!(<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &a).is_empty());

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
    assert!(matches!(&part_mod.diff, XlsxOpcPartDiff { bytes: Some(_), .. }));
    let rels = opc_diff.relationships.as_ref().expect("relationships diff present");
    assert!(!rels.removed.is_empty(), "opc.relationships: removed (owner) not exercised");
    assert!(!rels.modified.is_empty(), "opc.relationships: modified (owner) not exercised");
    assert!(!rels.added.is_empty(), "opc.relationships: added (owner) not exercised");

    // workbook.sheets: removed ("toDrop") + modified ("toModify", whose OWN cells diff
    // exercises removed+modified+added together) + added ("added", carried whole).
    let wb_diff = diff_ab.workbook.as_ref().expect("workbook diff present");
    let sheets_diff = wb_diff.sheets.as_ref().expect("sheets diff present");
    assert!(sheets_diff.removed.contains(&"toDrop".to_string()), "sheets: removed not exercised");
    assert!(sheets_diff.added.iter().any(|s| s.name == "added"), "sheets: added not exercised");
    let sheet_mod = sheets_diff.modified.iter().find(|m| m.key == "toModify").expect("toModify sheet modified");
    let cells_diff = sheet_mod.diff.cells.as_ref().expect("toModify cells diff present");
    assert!(!cells_diff.removed.is_empty(), "toModify.cells: removed not exercised");
    assert!(!cells_diff.modified.is_empty(), "toModify.cells: modified not exercised");
    assert!(!cells_diff.added.is_empty(), "toModify.cells: added not exercised");
    let cell_mod = &cells_diff.modified[0];
    assert!(matches!(&cell_mod.diff, XlsxCellDiff { value: Some(_) }));
    // The added cell in `toModify` carries a `Formula` value — exercises that variant too.
    assert!(matches!(&cells_diff.added[0].value, XlsxCellValue::Formula { .. }), "added cell should carry a Formula value");

    // The dropped sheet's full payload recurs as an `added` item in the OTHER direction.
    let sheets_diff_ba = diff_ba.workbook.as_ref().unwrap().sheets.as_ref().expect("sheets diff (b->a) present");
    let added_back = sheets_diff_ba.added.iter().find(|s| s.name == "toDrop").expect("toDrop sheet re-added in b->a");
    assert!(!added_back.cells.is_empty());

    // workbook.shared_strings (index-keyed, pairwise-position-matched): per the "known
    // structural trap" note, `a -> b` (shorter) exercises removed+modified; `b -> a`
    // (asserted separately) exercises added+modified.
    let ss_diff = wb_diff.shared_strings.as_ref().expect("shared_strings diff present");
    assert!(!ss_diff.removed.is_empty(), "shared_strings: removed not exercised");
    assert!(!ss_diff.modified.is_empty(), "shared_strings: modified not exercised");
    let ss_diff_ba = diff_ba.workbook.as_ref().unwrap().shared_strings.as_ref().expect("shared_strings diff (b->a) present");
    assert!(!ss_diff_ba.added.is_empty(), "shared_strings (b->a): added not exercised");
    assert!(!ss_diff_ba.modified.is_empty(), "shared_strings (b->a): modified not exercised");
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `XlsxMutation` grammar —
/// exercises every variant, incl. `SetSnapshot`'s full nested `XlsxSnapshot` (opc parts,
/// content-types, relationships incl. `OpcTargetMode::External`, workbook sheets/cells/shared
/// strings) and `SetCell`'s direct `XlsxCellValue` payload (incl. `Formula.cached` and raw
/// `,`/`:`/`[`/`]` bytes-through-hex in a string value).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let mut snapshot_with_opc = sweep_b();
    snapshot_with_opc.opc.relationships.get_mut("xl/toModify.xml").unwrap()[0].target_mode = OpcTargetMode::External;

    let mutations = vec![
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot_with_opc }),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "New, odd: [name]".into(), cells: vec![XlsxCell { row: 1, col: 2, value: XlsxCellValue::Empty }] } }),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() }),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 1, col: 0, value: XlsxCellValue::Number(-2.5) }),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 2, col: 0, value: XlsxCellValue::SharedString(3) }),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 3, col: 0, value: XlsxCellValue::Boolean(false) }),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 4, col: 0, value: XlsxCellValue::InlineString("has, weird: [chars]".into()) }),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 5, col: 0, value: XlsxCellValue::Formula { expr: "SUM(A1:A4)".into(), cached: Some(Box::new(XlsxCellValue::Number(1.5))) } }),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name: "Sheet1".into(), row: 6, col: 0, value: XlsxCellValue::Formula { expr: "NA()".into(), cached: None } }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: "Sheet1".into(), row: 1, col: 0 }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "z".into() }),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "y".into() }),
    ];
    for mutation in mutations {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = XlsxMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = XlsxMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw

//#region 🔖️KindsConformanceLaw
/// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
/// variant is added to `XlsxMutation` without a matching kebab-case spelling here, which is what
/// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
/// `kinds` array as text (the framework never parses Rust, so this is the only side that can
/// prove the manifest matches) and asserts the same list, in the same order.
#[semio_framework_async_macros::async_test]
async fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &XlsxMutation) -> &'static str {
        match mutation {
            XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { .. }) => "set-snapshot",
            XlsxMutation::InsertSheet(insert_sheet::InsertSheet { .. }) => "insert-sheet",
            XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { .. }) => "remove-sheet",
            XlsxMutation::RenameSheet(rename_sheet::RenameSheet { .. }) => "rename-sheet",
            XlsxMutation::SetCell(set_cell::SetCell { .. }) => "set-cell",
            XlsxMutation::RemoveCell(remove_cell::RemoveCell { .. }) => "remove-cell",
            XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { .. }) => "insert-shared-string",
            XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { .. }) => "remove-shared-string",
            XlsxMutation::SetSharedString(set_shared_string::SetSharedString { .. }) => "set-shared-string",
        }
    }
    let samples = [
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: XlsxSnapshot::default() }),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet::default() }),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: String::new() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: String::new(), new_name: String::new() }),
        XlsxMutation::SetCell(set_cell::SetCell { sheet_name: String::new(), row: 0, col: 0, value: XlsxCellValue::Empty }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { sheet_name: String::new(), row: 0, col: 0 }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: String::new() }),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: String::new() }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every XlsxMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match XlsxMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw
