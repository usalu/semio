use protocol::{OpText, OpBinary};
use super::*;
use crate::schema::diff::XlsxOpcPartDiff;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn workbook(snapshot: &XlsxSnapshot) -> XlsxWorkbook {
    snapshot.project_workbook().expect("the canonical XML parts project a workbook")
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn address(snapshot: &XlsxSnapshot, sheet: &str, row: u32, col: u32) -> cell_address::XlsxCellAddress {
    cell_address::xlsx_cell_address(snapshot, sheet, row, col).unwrap_or_else(|error| panic!("{sheet}!({row},{col}) has an address: {error}"))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn set_cell_at(snapshot: &XlsxSnapshot, sheet: &str, row: u32, col: u32, value: XlsxCellValue) -> XlsxMutation {
    XlsxMutation::SetCell(set_cell::SetCell { address: address(snapshot, sheet, row, col), value })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn remove_cell_at(snapshot: &XlsxSnapshot, sheet: &str, row: u32, col: u32) -> XlsxMutation {
    XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: address(snapshot, sheet, row, col) })
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_sheet_apply_and_inverse() {
    let base = fixture();
    let insert = XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "New".into(), cells: vec![] } });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &insert);
    let sheets = workbook(&after).sheets;
    assert_eq!(sheets.len(), 3);
    assert!(sheets.iter().any(|s| s.name == "New"));

    for inv in Mutation::inverse(&insert, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn remove_sheet_inverse_restores_removed_sheet() {
    // 🎯️ Targets `"Sheet2"`, the LAST sheet in `fixture()`; the inverse is the canonical `SetSnapshot` of the
    // pre-state, so the removed worksheet part, its relationship and its workbook entry all come back exactly.
    let base = fixture();
    let remove = XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &remove);
    assert_eq!(workbook(&after).sheets.len(), 1);
    for inv in Mutation::inverse(&remove, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn rename_sheet_apply_and_inverse() {
    let base = fixture();
    let rename = XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &rename);
    let sheets = workbook(&after).sheets;
    assert!(!sheets.iter().any(|s| s.name == "Sheet2"));
    let renamed = sheets.iter().find(|s| s.name == "Renamed").expect("renamed sheet present");
    assert!(renamed.cells.is_empty());
    for inv in Mutation::inverse(&rename, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn set_and_remove_cell_apply_and_inverse() {
    let base = fixture();
    let set_existing = set_cell_at(&base, "Sheet1", 1, 0, XlsxCellValue::Boolean(true));
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &set_existing);
    assert_eq!(workbook(&after).sheets[0].cells[0].value, XlsxCellValue::Boolean(true));
    for inv in Mutation::inverse(&set_existing, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    assert!(cell_address::xlsx_cell_address(&base, "Sheet1", 2, 3).is_err(), "set-cell edits an existing SpreadsheetML cell: a cell the worksheet does not carry has no address");

    let remove = remove_cell_at(&base, "Sheet1", 1, 0);
    let mut after3 = base.clone();
    apply_xlsx_mutation(&mut after3, &remove);
    assert!(workbook(&after3).sheets[0].cells.is_empty());
    for inv in Mutation::inverse(&remove, &base).expect("valid retained mutation inverse fixture") {
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
    assert_eq!(workbook(&after).shared_strings, vec!["hello".to_string(), "world".to_string()]);
    for inv in Mutation::inverse(&insert, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let set = XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "changed".into() });
    let mut after2 = base.clone();
    apply_xlsx_mutation(&mut after2, &set);
    assert_eq!(workbook(&after2).shared_strings, vec!["changed".to_string()]);
    for inv in Mutation::inverse(&set, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);

    let remove = XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 });
    let mut after3 = base.clone();
    apply_xlsx_mutation(&mut after3, &remove);
    assert!(workbook(&after3).shared_strings.is_empty());
    for inv in Mutation::inverse(&remove, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after3, &inv);
    }
    assert_eq!(after3, base);
}

#[semio_framework_async_macros::async_test]
async fn removing_unreferenced_shared_string_reindexes_later_references() {
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(1) }] }],
        shared_strings: vec!["drop".into(), "keep".into()],
    });
    apply_xlsx_mutation(&mut snapshot, &XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }));
    let projected = workbook(&snapshot);
    assert_eq!(projected.shared_strings, vec!["keep".to_string()]);
    assert_eq!(projected.sheets[0].cells[0].value, XlsxCellValue::SharedString(0));
    let shared_strings = snapshot.part_text(crate::standards::v_ecma_376::subsets::base::schema::vocabulary::SHARED_STRINGS_PART).expect("shared strings XML");
    assert!(shared_strings.contains("count=\"1\""));
    assert!(shared_strings.contains("uniqueCount=\"1\""));
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
        for inverse_mutation in <XlsxMutation as Mutation<XlsxSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
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

/// 🧮️ One worksheet with two cells, `A1` and `B1` — cell edits address EXISTING SpreadsheetML cells.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn two_cell_base() -> XlsxSnapshot {
    crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }, XlsxCell { row: 1, col: 1, value: XlsxCellValue::Number(2.0) }] }],
        shared_strings: vec![],
    })
}

/// 🎯️ A cell-level diff touches exactly one authoritative XML part — the addressed worksheet — and never the OPC lane.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_only_part_modified(diff: &XlsxDiff, part: &str) {
    assert!(diff.opc.is_none(), "a cell edit never reaches the lossless OPC lane");
    let parts = diff.xml_parts.as_ref().expect("xml parts diff present");
    assert!(parts.removed.is_empty() && parts.added.is_empty(), "a cell edit neither adds nor removes a part");
    assert_eq!(parts.modified.iter().map(|modified| modified.key.as_str()).collect::<Vec<_>>(), vec![part], "a cell edit modifies only its worksheet part");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn cell_value(snapshot: &XlsxSnapshot, row: u32, col: u32) -> Option<XlsxCellValue> {
    workbook(snapshot).sheets[0].cells.iter().find(|cell| cell.row == row && cell.col == col).map(|cell| cell.value.clone())
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    // Modify + remove: the later remove wins; the absorbed delta is one worksheet-part edit.
    {
        let base = two_cell_base();
        let sheet = address(&base, "Sheet1", 1, 0).part_path;
        let d1 = Mutation::diff(&set_cell_at(&base, "Sheet1", 1, 0, XlsxCellValue::Boolean(true)), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&remove_cell_at(&mid, "Sheet1", 1, 0), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        assert_only_part_modified(&absorbed, &sheet);
        let result = MutationDiff::apply(&absorbed, &base).unwrap();
        assert_eq!(cell_value(&result, 1, 0), None, "the modify of a since-removed cell must not survive absorb");
        assert_eq!(cell_value(&result, 1, 1), Some(XlsxCellValue::Number(2.0)), "the untouched neighbour survives");
    }

    // Two edits on distinct cells: both survive (two independent edits never clobber each other).
    {
        let base = two_cell_base();
        let sheet = address(&base, "Sheet1", 1, 0).part_path;
        let d1 = Mutation::diff(&set_cell_at(&base, "Sheet1", 1, 0, XlsxCellValue::InlineString("f".into())), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&set_cell_at(&mid, "Sheet1", 1, 1, XlsxCellValue::InlineString("g".into())), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        assert_only_part_modified(&absorbed, &sheet);
        let result = MutationDiff::apply(&absorbed, &base).unwrap();
        assert_eq!((cell_value(&result, 1, 0), cell_value(&result, 1, 1)), (Some(XlsxCellValue::InlineString("f".into())), Some(XlsxCellValue::InlineString("g".into()))), "both cell edits must survive absorb");
    }

    // Two edits of the same cell: the later value lands, still as one worksheet-part edit.
    {
        let base = two_cell_base();
        let sheet = address(&base, "Sheet1", 1, 0).part_path;
        let d1 = Mutation::diff(&set_cell_at(&base, "Sheet1", 1, 0, XlsxCellValue::InlineString("f".into())), &base);
        let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&set_cell_at(&mid, "Sheet1", 1, 0, XlsxCellValue::InlineString("patched".into())), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        assert_only_part_modified(&absorbed, &sheet);
        assert_eq!(cell_value(&MutationDiff::apply(&absorbed, &base).unwrap(), 1, 0), Some(XlsxCellValue::InlineString("patched".into())));
    }

    // Associativity over a triple.
    {
        let base = two_cell_base();
        let d1 = Mutation::diff(&set_cell_at(&base, "Sheet1", 1, 0, XlsxCellValue::Number(10.0)), &base);
        let mid1 = MutationDiff::apply(d1.diff(), &base).unwrap();
        let d2 = Mutation::diff(&set_cell_at(&mid1, "Sheet1", 1, 1, XlsxCellValue::Number(20.0)), &mid1);
        let mid2 = MutationDiff::apply(d2.diff(), &mid1).unwrap();
        let d3 = Mutation::diff(&remove_cell_at(&mid2, "Sheet1", 1, 0), &mid2);
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
    let real = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
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
    let snap = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
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
/// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every lane the snapshot carries (see the fixtures'
/// doc comment) — the lossless OPC lane (binary parts, content types, part-owned relationships) and the
/// authoritative XML parts (workbook, worksheet, shared strings) — and the projected workbook follows exactly.
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    use crate::standards::v_ecma_376::subsets::base::schema::vocabulary::{SHARED_STRINGS_PART, WORKBOOK_PART};
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &b);
    assert_eq!(MutationDiff::apply(&diff_ab, &a).unwrap(), b);
    let diff_ba = <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&b, &a);
    assert_eq!(MutationDiff::apply(&diff_ba, &b).unwrap(), a);
    assert!(<XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(&a, &a).is_empty());

    let opc_diff = diff_ab.opc.as_ref().expect("opc diff present");
    let ct = opc_diff.content_types.as_ref().expect("content_types diff present");
    assert!(!ct.defaults.as_ref().expect("defaults diff present").added.is_empty(), "content_types.defaults: added not exercised");
    let overrides = ct.overrides.as_ref().expect("overrides diff present");
    assert!(!overrides.modified.is_empty(), "content_types.overrides: modified not exercised");
    let parts = opc_diff.parts.as_ref().expect("parts diff present");
    assert!(!parts.removed.is_empty(), "opc.parts: removed not exercised");
    assert!(!parts.modified.is_empty(), "opc.parts: modified not exercised");
    assert!(!parts.added.is_empty(), "opc.parts: added not exercised");
    assert!(matches!(&parts.modified[0].diff, XlsxOpcPartDiff { bytes: Some(_), content_type: Some(_) }), "the modified binary part changes bytes and content type");
    let rels = opc_diff.relationships.as_ref().expect("relationships diff present");
    assert!(!rels.removed.is_empty(), "opc.relationships: removed (owner) not exercised");
    assert!(!rels.modified.is_empty(), "opc.relationships: modified (owner) not exercised");
    assert!(!rels.added.is_empty(), "opc.relationships: added (owner) not exercised");

    let xml = diff_ab.xml_parts.as_ref().expect("xml parts diff present");
    let modified = xml.modified.iter().map(|part| part.key.as_str()).collect::<Vec<_>>();
    for part in [WORKBOOK_PART, SHARED_STRINGS_PART] {
        assert!(modified.contains(&part), "xml part {part} not modified: {modified:?}");
    }
    assert!(modified.iter().any(|part| part.starts_with("xl/worksheets/")), "no worksheet part modified: {modified:?}");

    let (before, after) = (workbook(&a), workbook(&b));
    assert_eq!(workbook(&MutationDiff::apply(&diff_ab, &a).unwrap()), after, "the projected workbook follows the diff");
    assert_eq!(before.sheets.iter().map(|sheet| sheet.name.as_str()).collect::<Vec<_>>(), ["toModify", "stay", "toDrop"]);
    assert_eq!(after.sheets.iter().map(|sheet| sheet.name.as_str()).collect::<Vec<_>>(), ["toModify", "stay", "added"]);
    let cells = |workbook: &XlsxWorkbook| workbook.sheets[0].cells.iter().map(|cell| (cell.row, cell.col, cell.value.clone())).collect::<Vec<_>>();
    assert_eq!(cells(&before), vec![(1, 0, XlsxCellValue::Number(1.0)), (2, 0, XlsxCellValue::Boolean(false))]);
    assert_eq!(
        cells(&after),
        vec![(1, 0, XlsxCellValue::Number(2.0)), (3, 0, XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(3.0))) })],
        "toModify: (2,0) removed, (1,0) modified, (3,0) added as a formula"
    );
    assert_eq!((before.shared_strings.len(), after.shared_strings.len()), (3, 2), "shared strings: one removed a -> b, re-added b -> a");
    assert_eq!(after.shared_strings[1], "toModify-changed");
    assert_eq!(workbook(&MutationDiff::apply(&diff_ba, &b).unwrap()), before, "the reverse diff restores the dropped sheet and the removed shared string");
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `XlsxMutation` grammar —
/// exercises every variant, incl. `SetSnapshot`'s full nested `XlsxSnapshot` (opc parts,
/// content-types, relationships incl. `OpcTargetMode::External`, the authoritative XML parts) and
/// `SetCell`'s lineage-bound cell address plus its direct `XlsxCellValue` payload (incl.
/// `Formula.cached` and raw `,`/`:`/`[`/`]` bytes-through-hex in a string value).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let snapshot_with_opc = sweep_b();
    assert!(snapshot_with_opc.opc.relationships.groups().map(|(_, relationships)| relationships).flatten().any(|relationship| relationship.target_mode == OpcTargetMode::External), "the codec sweep carries an External relationship");
    let base = fixture();
    let cell = || address(&base, "Sheet1", 1, 0);

    let mutations = vec![
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot_with_opc }),
        XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "New, odd: [name]".into(), cells: vec![XlsxCell { row: 1, col: 2, value: XlsxCellValue::Empty }] } }),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::Number(-2.5) }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::SharedString(3) }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::Boolean(false) }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::InlineString("has, weird: [chars]".into()) }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::Formula { expr: "SUM(A1:A4)".into(), cached: Some(Box::new(XlsxCellValue::Number(1.5))) } }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::Formula { expr: "NA()".into(), cached: None } }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: cell() }),
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
            XlsxMutation::PatchSnapshot(_) => "patch-snapshot",
            XlsxMutation::InsertSheet(insert_sheet::InsertSheet { .. }) => "insert-sheet",
            XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { .. }) => "remove-sheet",
            XlsxMutation::RenameSheet(rename_sheet::RenameSheet { .. }) => "rename-sheet",
            XlsxMutation::SetCell(set_cell::SetCell { .. }) => "set-cell",
            XlsxMutation::InsertCell(insert_cell::InsertCell { .. }) => "insert-cell",
            XlsxMutation::RemoveCell(remove_cell::RemoveCell { .. }) => "remove-cell",
            XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { .. }) => "insert-shared-string",
            XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { .. }) => "remove-shared-string",
            XlsxMutation::SetSharedString(set_shared_string::SetSharedString { .. }) => "set-shared-string",
        }
    }
    let samples = [
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: XlsxSnapshot::default() }),
        XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet::default() }),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: String::new() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: String::new(), new_name: String::new() }),
        XlsxMutation::SetCell(set_cell::SetCell { address: address(&fixture(), "Sheet1", 1, 0), value: XlsxCellValue::Empty }),
        XlsxMutation::InsertCell(insert_cell::InsertCell { address: cell_address::xlsx_cell_vacancy_address(&fixture(), "Sheet1", 2, 1).unwrap(), value: XlsxCellValue::Empty }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: address(&fixture(), "Sheet1", 1, 0) }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: String::new() }),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: String::new() }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every XlsxMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match XlsxMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw
