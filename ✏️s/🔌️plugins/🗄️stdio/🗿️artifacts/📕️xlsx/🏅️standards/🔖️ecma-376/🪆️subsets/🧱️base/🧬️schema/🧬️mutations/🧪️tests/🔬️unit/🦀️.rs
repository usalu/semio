use protocol::{OpText, OpBinary};
use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::*;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
#[cfg(test)] use semio_framework_replication_mutation::Mutation;
#[cfg(test)] use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCellValue;
#[cfg(test)] use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSheet;
#[cfg(test)] use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::snapshot::XmlNode;
#[cfg(test)] use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
#[cfg(test)] use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::diff::XlsxDiff;

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
    XlsxMutation::SetCell(set_cell::SetCell { address: address(snapshot, sheet, row, col), value, node: None })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn remove_cell_at(snapshot: &XlsxSnapshot, sheet: &str, row: u32, col: u32) -> XlsxMutation {
    XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: address(snapshot, sheet, row, col), keep_row: false })
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_sheet_apply_and_inverse() {
    let base = fixture();
    let insert = XlsxMutation::InsertSheet(insert_sheet::InsertSheet::minted(&base, XlsxSheet { name: "New".into(), cells: vec![] }, None).expect("the gesture mints the sheet slot"));
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &insert);
    let sheets = workbook(&after).sheets;
    assert_eq!(sheets.len(), 3);
    assert!(sheets.iter().any(|s| s.name == "New"));

    for inv in ::semio_os_kernel::ColdMutation::inverse(&insert, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn remove_sheet_inverse_restores_removed_sheet() {
    let base = fixture();
    let remove = XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &remove);
    assert_eq!(workbook(&after).sheets.len(), 1);
    for inv in ::semio_os_kernel::ColdMutation::inverse(&remove, &base).expect("valid retained mutation inverse fixture") {
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
    for inv in ::semio_os_kernel::ColdMutation::inverse(&rename, &base).expect("valid retained mutation inverse fixture") {
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
    for inv in ::semio_os_kernel::ColdMutation::inverse(&set_existing, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    assert!(cell_address::xlsx_cell_address(&base, "Sheet1", 2, 3).is_err(), "set-cell edits an existing SpreadsheetML cell: a cell the worksheet does not carry has no address");

    let remove = remove_cell_at(&base, "Sheet1", 1, 0);
    let mut after3 = base.clone();
    apply_xlsx_mutation(&mut after3, &remove);
    assert!(workbook(&after3).sheets[0].cells.is_empty());
    for inv in ::semio_os_kernel::ColdMutation::inverse(&remove, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after3, &inv);
    }
    assert_eq!(after3, base);
}

#[semio_framework_async_macros::async_test]
async fn shared_string_mutations_apply_and_inverse() {
    let base = fixture();
    let insert = XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "world".into(), index: None, node: None });
    let mut after = base.clone();
    apply_xlsx_mutation(&mut after, &insert);
    assert_eq!(workbook(&after).shared_strings, vec!["hello".to_string(), "world".to_string()]);
    for inv in ::semio_os_kernel::ColdMutation::inverse(&insert, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let set = XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "changed".into(), node: None });
    let mut after2 = base.clone();
    apply_xlsx_mutation(&mut after2, &set);
    assert_eq!(workbook(&after2).shared_strings, vec!["changed".to_string()]);
    for inv in ::semio_os_kernel::ColdMutation::inverse(&set, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);

    let remove = XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 });
    let mut after3 = base.clone();
    apply_xlsx_mutation(&mut after3, &remove);
    assert!(workbook(&after3).shared_strings.is_empty());
    for inv in ::semio_os_kernel::ColdMutation::inverse(&remove, &base).expect("valid retained mutation inverse fixture") {
        apply_xlsx_mutation(&mut after3, &inv);
    }
    assert_eq!(after3, base);
}

#[semio_framework_async_macros::async_test]
async fn removing_unreferenced_shared_string_reindexes_later_references() {
    let mut snapshot = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(1) }] }],
        shared_strings: vec!["drop".into(), "keep".into()],
    });
    apply_xlsx_mutation(&mut snapshot, &XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }));
    let projected = workbook(&snapshot);
    assert_eq!(projected.shared_strings, vec!["keep".to_string()]);
    assert_eq!(projected.sheets[0].cells[0].value, XlsxCellValue::SharedString(0));
    let shared_strings = snapshot.part_text(semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::SHARED_STRINGS_PART).expect("shared strings XML");
    assert!(shared_strings.contains("count=\"1\""));
    assert!(shared_strings.contains("uniqueCount=\"1\""));
}

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in demo_mutation_cases() {
        let base = fixture();
        let diff_direct = ::semio_os_kernel::ColdMutation::diff(&mutation, &base);
        let applied_via_diff = protocol::apply_diff_cold(diff_direct.diff(), &base).unwrap();

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
        for inverse_mutation in <XlsxMutation as ColdMutation<XlsxSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
            apply_xlsx_mutation(&mut round_tripped, &inverse_mutation);
        }
        assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

        let diff = ::semio_os_kernel::ColdMutation::diff(&mutation, &base);
        let next = protocol::apply_diff_cold(diff.diff(), &base).unwrap();
        let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
        let restored = protocol::apply_diff_cold(&inverse_diff, &next).unwrap();
        assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️AbsorbLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_absorb_matches_sequential(base: &XlsxSnapshot, d1: &XlsxDiff, d2: &XlsxDiff) -> XlsxDiff {
    let sequential = protocol::apply_diff_cold(d2, &protocol::apply_diff_cold(d1, base).unwrap()).unwrap();
    let mut absorbed = d1.clone();
    MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(protocol::apply_diff_cold(&absorbed, base).unwrap(), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

/// 🧮️ One worksheet with two cells, `A1` and `B1` — cell edits address EXISTING SpreadsheetML cells.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn two_cell_base() -> XlsxSnapshot {
    semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }, XlsxCell { row: 1, col: 1, value: XlsxCellValue::Number(2.0) }] }],
        shared_strings: vec![],
    })
}

/// 🎯️ A cell-level diff touches exactly one authoritative XML part — the addressed worksheet — and never the OPC lane.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_only_part_modified(diff: &XlsxDiff, part: &str) {
    assert!(diff.opc.is_none(), "a cell edit never reaches the lossless OPC lane");
    let parts = diff.xml_parts.as_ref().expect("xml parts diff present");
    assert!(parts.removed.is_empty() && parts.inserted.is_empty() && parts.moved.is_empty(), "a cell edit neither adds nor removes a part");
    assert_eq!(parts.modified.iter().map(|modified| modified.id.as_str()).collect::<Vec<_>>(), vec![part], "a cell edit modifies only its worksheet part");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn cell_value(snapshot: &XlsxSnapshot, row: u32, col: u32) -> Option<XlsxCellValue> {
    workbook(snapshot).sheets[0].cells.iter().find(|cell| cell.row == row && cell.col == col).map(|cell| cell.value.clone())
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    {
        let base = two_cell_base();
        let sheet = address(&base, "Sheet1", 1, 0).part_path;
        let d1 = ::semio_os_kernel::ColdMutation::diff(&set_cell_at(&base, "Sheet1", 1, 0, XlsxCellValue::Boolean(true)), &base);
        let mid = protocol::apply_diff_cold(d1.diff(), &base).unwrap();
        let d2 = ::semio_os_kernel::ColdMutation::diff(&remove_cell_at(&mid, "Sheet1", 1, 0), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        assert_only_part_modified(&absorbed, &sheet);
        let result = protocol::apply_diff_cold(&absorbed, &base).unwrap();
        assert_eq!(cell_value(&result, 1, 0), None, "the modify of a since-removed cell must not survive absorb");
        assert_eq!(cell_value(&result, 1, 1), Some(XlsxCellValue::Number(2.0)), "the untouched neighbour survives");
    }

    {
        let base = two_cell_base();
        let sheet = address(&base, "Sheet1", 1, 0).part_path;
        let d1 = ::semio_os_kernel::ColdMutation::diff(&set_cell_at(&base, "Sheet1", 1, 0, XlsxCellValue::InlineString("f".into())), &base);
        let mid = protocol::apply_diff_cold(d1.diff(), &base).unwrap();
        let d2 = ::semio_os_kernel::ColdMutation::diff(&set_cell_at(&mid, "Sheet1", 1, 1, XlsxCellValue::InlineString("g".into())), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        assert_only_part_modified(&absorbed, &sheet);
        let result = protocol::apply_diff_cold(&absorbed, &base).unwrap();
        assert_eq!((cell_value(&result, 1, 0), cell_value(&result, 1, 1)), (Some(XlsxCellValue::InlineString("f".into())), Some(XlsxCellValue::InlineString("g".into()))), "both cell edits must survive absorb");
    }

    {
        let base = two_cell_base();
        let sheet = address(&base, "Sheet1", 1, 0).part_path;
        let d1 = ::semio_os_kernel::ColdMutation::diff(&set_cell_at(&base, "Sheet1", 1, 0, XlsxCellValue::InlineString("f".into())), &base);
        let mid = protocol::apply_diff_cold(d1.diff(), &base).unwrap();
        let d2 = ::semio_os_kernel::ColdMutation::diff(&set_cell_at(&mid, "Sheet1", 1, 0, XlsxCellValue::InlineString("patched".into())), &mid);
        let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
        assert_only_part_modified(&absorbed, &sheet);
        assert_eq!(cell_value(&protocol::apply_diff_cold(&absorbed, &base).unwrap(), 1, 0), Some(XlsxCellValue::InlineString("patched".into())));
    }

    {
        let base = two_cell_base();
        let d1 = ::semio_os_kernel::ColdMutation::diff(&set_cell_at(&base, "Sheet1", 1, 0, XlsxCellValue::Number(10.0)), &base);
        let mid1 = protocol::apply_diff_cold(d1.diff(), &base).unwrap();
        let d2 = ::semio_os_kernel::ColdMutation::diff(&set_cell_at(&mid1, "Sheet1", 1, 1, XlsxCellValue::Number(20.0)), &mid1);
        let mid2 = protocol::apply_diff_cold(d2.diff(), &mid1).unwrap();
        let d3 = ::semio_os_kernel::ColdMutation::diff(&remove_cell_at(&mid2, "Sheet1", 1, 0), &mid2);
        let sequential = protocol::apply_diff_cold(d3.diff(), &mid2).unwrap();

        let mut left = d1.diff().clone();
        MutationDiff::absorb(&mut left, d2.diff().clone());
        MutationDiff::absorb(&mut left, d3.diff().clone());

        let mut d2_then_d3 = d2.diff().clone();
        MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
        let mut right = d1.diff().clone();
        MutationDiff::absorb(&mut right, d2_then_d3);

        assert_eq!(protocol::apply_diff_cold(&left, &base).unwrap(), sequential, "absorb associativity (left) failed");
        assert_eq!(protocol::apply_diff_cold(&right, &base).unwrap(), sequential, "absorb associativity (right) failed");
    }
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️CodecRetentionLaw
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
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
    let bytes = pack::snapshot::ArtifactPack::encode_pack(&snap);
    let decoded = <XlsxSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}
//#endregion 🔖️FieldSweep

//#region 🔖️OpTextBinaryRoundtripLaw
/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `XlsxMutation` grammar —
/// exercises every variant, incl. the optional insertion positions and
/// `SetCell`'s lineage-bound cell address plus its direct `XlsxCellValue` payload (incl.
/// `Formula.cached` and raw `,`/`:`/`[`/`]` bytes-through-hex in a string value).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = fixture();
    let cell = || address(&base, "Sheet1", 1, 0);

    let mutations = vec![
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "New, odd: [name]".into(), cells: vec![XlsxCell { row: 1, col: 2, value: XlsxCellValue::Empty }] }, index: None, slot: None }),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "Placed".into(), cells: vec![] }, index: Some(1), slot: None }),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::Number(-2.5), node: None }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::SharedString(3), node: None }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::Boolean(false), node: None }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::InlineString("has, weird: [chars]".into()), node: None }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::Formula { expr: "SUM(A1:A4)".into(), cached: Some(Box::new(XlsxCellValue::Number(1.5))) }, node: None }),
        XlsxMutation::SetCell(set_cell::SetCell { address: cell(), value: XlsxCellValue::Formula { expr: "NA()".into(), cached: None }, node: None }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: cell(), keep_row: false }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "z".into(), index: None, node: None }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "z".into(), index: Some(0), node: None }),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "y".into(), node: None }),
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
            XlsxMutation::InsertSheet(insert_sheet::InsertSheet { .. }) => "insert-sheet",
            XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { .. }) => "remove-sheet",
            XlsxMutation::RenameSheet(rename_sheet::RenameSheet { .. }) => "rename-sheet",
            XlsxMutation::SetCell(set_cell::SetCell { .. }) => "set-cell",
            XlsxMutation::InsertCell(insert_cell::InsertCell { .. }) => "insert-cell",
            XlsxMutation::RemoveCell(remove_cell::RemoveCell { .. }) => "remove-cell",
            XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { .. }) => "insert-shared-string",
            XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { .. }) => "remove-shared-string",
            XlsxMutation::SetSharedString(set_shared_string::SetSharedString { .. }) => "set-shared-string",
            XlsxMutation::SetRelationship(set_relationship::SetRelationship { .. }) => "set-relationship",
            XlsxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { .. }) => "remove-relationship",
            XlsxMutation::SetContentType(set_content_type::SetContentType { .. }) => "set-content-type",
            XlsxMutation::RemoveContentType(remove_content_type::RemoveContentType { .. }) => "remove-content-type",
        }
    }
    let samples = [
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet::default(), index: None, slot: None }),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: String::new() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: String::new(), new_name: String::new() }),
        XlsxMutation::SetCell(set_cell::SetCell { address: address(&fixture(), "Sheet1", 1, 0), value: XlsxCellValue::Empty, node: None }),
        XlsxMutation::InsertCell(insert_cell::InsertCell { address: cell_address::xlsx_cell_vacancy_address(&fixture(), "Sheet1", 2, 1).unwrap(), value: XlsxCellValue::Empty, node: None }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: address(&fixture(), "Sheet1", 1, 0), keep_row: false }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: String::new(), index: None, node: None }),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: String::new(), node: None }),
        XlsxMutation::SetRelationship(set_relationship::SetRelationship { owner: String::new(), id: String::new(), rel_type: String::new(), target: String::new(), external: false, index: None }),
        XlsxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: String::new(), id: String::new() }),
        XlsxMutation::SetContentType(set_content_type::SetContentType { is_override: false, name: String::new(), content_type: String::new(), index: None }),
        XlsxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override: false, name: String::new() }),
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

//#region 🔖️InverseSumLaw
/// 🧮️ Three sheets, three shared strings (the last one referenced) and a worksheet whose rows 1-3 and row 1's cells A-C give every ordered collection a MIDDLE member.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ordered_base() -> XlsxSnapshot {
    let cell = |row, col, value| XlsxCell { row, col, value };
    semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![
            XlsxSheet { name: "First".into(), cells: vec![cell(1, 0, XlsxCellValue::Number(1.0)), cell(1, 1, XlsxCellValue::Number(2.0)), cell(1, 2, XlsxCellValue::Number(3.0)), cell(2, 0, XlsxCellValue::Number(4.0)), cell(3, 0, XlsxCellValue::SharedString(2))] },
            XlsxSheet { name: "Middle".into(), cells: vec![cell(1, 0, XlsxCellValue::InlineString("kept".into()))] },
            XlsxSheet { name: "Last".into(), cells: vec![] },
        ],
        shared_strings: vec!["zero".into(), "one".into(), "two".into()],
    })
}

/// ⚖️ Every demo kind satisfies the inverse sum law on the demo fixture.
#[semio_framework_async_macros::async_test]
async fn every_demo_kind_satisfies_the_inverse_sum_law() {
    let base = fixture();
    for mutation in demo_mutation_cases() {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}

/// ⚖️ Removing or moving a MIDDLE member of every ordered collection is restored at its original position.
#[semio_framework_async_macros::async_test]
async fn removing_a_middle_member_is_restored_at_its_original_position() {
    let base = ordered_base();
    let laws = protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    laws(&XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Middle".into() }), &base).await;
    laws(&XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Middle".into(), new_name: "Renamed".into() }), &base).await;
    laws(&remove_cell_at(&base, "First", 1, 1), &base).await;
    laws(&remove_cell_at(&base, "First", 2, 0), &base).await;
    laws(&set_cell_at(&base, "First", 1, 1, XlsxCellValue::Boolean(true)), &base).await;
    laws(&XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 1 }), &base).await;
    laws(&XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 1, value: "changed".into(), node: None }), &base).await;
    laws(&XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "placed".into(), index: Some(1), node: None }), &base).await;
    laws(&XlsxMutation::InsertSheet(insert_sheet::InsertSheet::minted(&base, XlsxSheet { name: "Placed".into(), cells: vec![] }, Some(1)).expect("the gesture mints the sheet slot")), &base).await;
    let vacancy = cell_address::xlsx_cell_vacancy_address(&base, "First", 1, 5).expect("a vacancy after the last cell of row 1");
    laws(&XlsxMutation::InsertCell(insert_cell::InsertCell { address: vacancy, value: XlsxCellValue::Number(9.0), node: None }), &base).await;
    let between = cell_address::xlsx_cell_vacancy_address(&base, "Middle", 1, 3).expect("a vacancy in the middle sheet");
    laws(&XlsxMutation::InsertCell(insert_cell::InsertCell { address: between, value: XlsxCellValue::Number(9.0), node: None }), &base).await;
}

/// ⚖️ The reference renumbering of a removed shared string follows the diff and is undone by its inverse.
#[semio_framework_async_macros::async_test]
async fn removing_a_middle_shared_string_renumbers_the_references_behind_it() {
    let base = ordered_base();
    let mut after = base.clone();
    let remove = XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 1 });
    assert!(apply_xlsx_mutation(&mut after, &remove).messages().is_empty());
    assert_eq!(workbook(&after).shared_strings, vec!["zero".to_string(), "two".to_string()]);
    assert_eq!(workbook(&after).sheets[0].cells.last().map(|cell| cell.value.clone()), Some(XlsxCellValue::SharedString(1)));
    let referenced = XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 2 });
    assert!(!apply_xlsx_mutation(&mut base.clone(), &referenced).messages().is_empty(), "a referenced shared string cannot be removed");
}
//#endregion 🔖️InverseSumLaw

/// 🧮️ The ordered workbook with a styled cell (`s="3"`), a rich-text shared string (two runs) and a sheet entry that carries an extra attribute, so every verbatim carrier is exercised.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rich_base() -> XlsxSnapshot {
    let mut base = ordered_base();
    let sheet_path = cell_address::xlsx_cell_address(&base, "First", 1, 1).expect("cell address").part_path;
    {
        let root = base.xml_part_mut(&sheet_path).and_then(|part| part.document.root.as_mut()).expect("worksheet root");
        let XmlNode::Element { children, .. } = root else { panic!("worksheet element") };
        let XmlNode::Element { children: rows, .. } = &mut children[0] else { panic!("sheetData") };
        let XmlNode::Element { children: cells, .. } = &mut rows[0] else { panic!("row") };
        let XmlNode::Element { attrs, .. } = &mut cells[1] else { panic!("cell") };
        attrs.insert(1, XmlAttr { name: "s".into(), value: "3".into() });
    }
    {
        let shared_path = shared_strings_path_of(&base);
        let root = base.xml_part_mut(&shared_path).and_then(|part| part.document.root.as_mut()).expect("shared strings root");
        let XmlNode::Element { children, .. } = root else { panic!("sst element") };
        let run = |text: &str| XmlNode::Element { name: "r".into(), attrs: vec![], children: vec![XmlNode::Element { name: "t".into(), attrs: vec![], children: vec![XmlNode::Text { text: text.into() }] }] };
        children[1] = XmlNode::Element { name: "si".into(), attrs: vec![], children: vec![run("o"), run("ne")] };
    }
    {
        let workbook = base.workbook_part_path().expect("workbook path");
        let root = base.xml_part_mut(&workbook).and_then(|part| part.document.root.as_mut()).expect("workbook root");
        let XmlNode::Element { children, .. } = root else { panic!("workbook element") };
        let XmlNode::Element { children: sheets, .. } = &mut children[0] else { panic!("sheets") };
        let XmlNode::Element { attrs, .. } = &mut sheets[1] else { panic!("sheet") };
        attrs.push(XmlAttr { name: "state".into(), value: "hidden".into() });
    }
    base.validate_authority().expect("rich fixture authority");
    base
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shared_strings_path_of(base: &XlsxSnapshot) -> String {
    let workbook = base.workbook_part_path().expect("workbook path");
    let relationship = base.opc.relationships_for(&workbook).iter().find(|relationship| relationship.rel_type.ends_with("/sharedStrings")).expect("shared strings relationship");
    semio_s_artifact_stdio_zip::opc::resolve_relationship_target(&workbook, &relationship.target)
}

/// ⚖️ Cells, shared strings and sheets that rebuilding from their typed value would flatten return verbatim through their inverses.
#[semio_framework_async_macros::async_test]
async fn inverses_restore_what_the_typed_value_cannot_express() {
    let base = rich_base();
    let laws = protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    laws(&remove_cell_at(&base, "First", 1, 1), &base).await;
    laws(&set_cell_at(&base, "First", 1, 1, XlsxCellValue::Boolean(true)), &base).await;
    laws(&XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 1 }), &base).await;
    laws(&XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 1, value: "flat".into(), node: None }), &base).await;
    laws(&XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Middle".into() }), &base).await;
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn formulas_and_values(snapshot: &XlsxSnapshot) -> (std::collections::BTreeMap<String, String>, std::collections::BTreeMap<String, XlsxCellValue>) {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::calculation::recalculate_workbook;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::column_letter;
    let projected = workbook(snapshot);
    let (mut formulas, mut values) = (std::collections::BTreeMap::new(), std::collections::BTreeMap::new());
    for sheet in &projected.sheets {
        for cell in &sheet.cells {
            if let XlsxCellValue::Formula { expr, .. } = &cell.value {
                formulas.insert(format!("{}!{}{}", sheet.name, column_letter(cell.col), cell.row), expr.clone());
            }
        }
    }
    for cell in recalculate_workbook(&projected) {
        values.insert(format!("{}!{}{}", cell.sheet, column_letter(cell.col), cell.row), cell.cached);
    }
    (formulas, values)
}

#[semio_framework_async_macros::async_test]
async fn shared_formula_fixture_matches_openpyxl_and_formulas_through_every_edit_and_undo() {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::column_index;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/📋️shared-formulas/🔣️.json")).unwrap();
    let base = decode_xlsx(include_bytes!("../../../../🧪️tests/🧫️fixtures/📋️shared-formulas/📊️workbook.xlsx")).unwrap();
    let check = |snapshot: &XlsxSnapshot, case: &serde_json::Value| {
        let (formulas, values) = formulas_and_values(snapshot);
        let expected: std::collections::BTreeMap<String, String> = case["formulas"].as_object().unwrap().iter().map(|(key, expr)| (key.clone(), expr.as_str().unwrap().to_string())).collect();
        assert_eq!(formulas, expected, "{} formulas", case["id"]);
        for (key, expected) in case["values"].as_object().unwrap() {
            let actual = values.get(key);
            let expected = expected.as_f64().unwrap();
            match actual {
                Some(XlsxCellValue::Number(actual)) => assert!((actual - expected).abs() < 1e-9, "{} {key}: {actual} != {expected}", case["id"]),
                other => panic!("{} {key}: {other:?} != {expected}", case["id"]),
            }
        }
    };
    let cases = fixture["cases"].as_array().unwrap();
    check(&base, &cases[0]);
    for case in &cases[1..] {
        let edit = &case["edit"];
        let (sheet, reference) = edit["cell"].as_str().unwrap().split_once('!').unwrap();
        let letters = reference.trim_end_matches(|c: char| c.is_ascii_digit());
        let (row, col) = (reference[letters.len()..].parse::<u32>().unwrap(), column_index(letters).unwrap());
        let value = match edit["formula"].as_str() {
            Some(formula) => XlsxCellValue::Formula { expr: formula.trim_start_matches('=').into(), cached: None },
            None => XlsxCellValue::Number(edit["number"].as_f64().unwrap()),
        };
        let mutation = set_cell_at(&base, sheet, row, col, value);
        let mut after = base.clone();
        let outcome = apply_xlsx_mutation(&mut after, &mutation);
        assert!(outcome.diff().xml_parts.is_some(), "{} applies", case["id"]);
        check(&after, case);
        let mut undone = after.clone();
        for inverse in ::semio_os_kernel::ColdMutation::inverse(&mutation, &base).expect("the edit has an inverse") {
            apply_xlsx_mutation(&mut undone, &inverse);
        }
        assert_eq!(undone, base, "{} undo restores the shared group verbatim", case["id"]);
        check(&undone, &cases[0]);
    }
}

#[semio_framework_async_macros::async_test]
async fn a_cache_only_recalculation_keeps_the_shared_group_intact() {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx;
    let base = decode_xlsx(include_bytes!("../../../../🧪️tests/🧫️fixtures/📋️shared-formulas/📊️workbook.xlsx")).unwrap();
    let (formulas, _) = formulas_and_values(&base);
    let mut after = base.clone();
    let mutations = xlsx_with_recalculated_caches(&base, Vec::new());
    assert!(!mutations.is_empty(), "uncalculated formulas get caches");
    for mutation in &mutations {
        assert!(apply_xlsx_mutation(&mut after, mutation).diff().xml_parts.is_some());
    }
    assert_eq!(formulas_and_values(&after).0, formulas);
    let sheet_path = cell_address::xlsx_cell_address(&after, "Sheet1", 3, 1).unwrap().part_path;
    let xml = format!("{:?}", after.xml_part(&sheet_path).unwrap().document);
    assert_eq!(xml.matches("\"shared\"").count(), 5, "master and dependents stay a shared group");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn rewrite_case_base(case: &serde_json::Value) -> XlsxSnapshot {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::column_index;
    let sheets = case["sheets"].as_array().unwrap().iter().map(|sheet| {
        let cells = sheet["cells"].as_object().unwrap().iter().map(|(reference, raw)| {
            let letters = reference.trim_end_matches(|c: char| c.is_ascii_digit());
            let value = match (raw.as_f64(), raw.as_str()) {
                (Some(number), _) => XlsxCellValue::Number(number),
                (_, Some(text)) => XlsxCellValue::Formula { expr: text.trim_start_matches('=').into(), cached: None },
                _ => panic!("unsupported fixture cell {raw}"),
            };
            XlsxCell { row: reference[letters.len()..].parse().unwrap(), col: column_index(letters).unwrap(), value }
        });
        XlsxSheet { name: sheet["name"].as_str().unwrap().into(), cells: cells.collect() }
    });
    let sheets = sheets.collect::<Vec<_>>();
    let base = opc_layer::with_demo_entries(semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook { sheets, shared_strings: vec![] }));
    let mut calculated = base.clone();
    for mutation in xlsx_with_recalculated_caches(&base, Vec::new()) {
        assert!(apply_xlsx_mutation(&mut calculated, &mutation).messages().is_empty());
    }
    calculated
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn formula_cells(snapshot: &XlsxSnapshot) -> std::collections::BTreeMap<String, (String, Option<XlsxCellValue>)> {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::column_letter;
    let mut cells = std::collections::BTreeMap::new();
    for sheet in workbook(snapshot).sheets {
        for cell in sheet.cells {
            if let XlsxCellValue::Formula { expr, cached } = cell.value {
                cells.insert(format!("{}!{}{}", sheet.name, column_letter(cell.col), cell.row), (expr, cached.map(|cached| *cached)));
            }
        }
    }
    cells
}

#[test]
fn sheet_rename_and_removal_rewrite_every_formula_like_excel_and_undo_restores_them() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🏷️sheet-reference-rewrite/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let base = rewrite_case_base(case);
        let before = formula_cells(&base);
        let primary = match (case["op"]["rename"].as_array(), case["op"]["remove"].as_str()) {
            (Some(pair), _) => xlsx_rename_sheet_mutations(&base, pair[0].as_str().unwrap(), pair[1].as_str().unwrap()),
            (_, Some(name)) => xlsx_remove_sheet_mutations(&base, name),
            _ => panic!("{id}: unknown op"),
        };
        let mutations = xlsx_with_recalculated_caches(&base, primary.unwrap_or_else(|error| panic!("{id}: {error}")));
        let (mut after, mut inverses) = (base.clone(), Vec::new());
        for mutation in &mutations {
            inverses.push(::semio_os_kernel::ColdMutation::inverse(mutation, &after).expect("every step has an inverse"));
            assert!(apply_xlsx_mutation(&mut after, mutation).messages().is_empty(), "{id}: {mutation:?} was refused");
        }
        let formulas = formula_cells(&after);
        for (key, (expr, cached)) in &formulas {
            let expected_expr = case["expected_formulas"][key].as_str().map(|text| text[1..].to_string()).or_else(|| before.get(key).map(|(expr, _)| expr.clone()));
            assert_eq!(Some(expr.clone()), expected_expr, "{id}: {key} formula");
            let expected = &case["expected"][key];
            let actual = cached.clone().unwrap_or_else(|| panic!("{id}: {key} has no cache"));
            match (&actual, expected) {
                (XlsxCellValue::Number(left), serde_json::Value::Number(right)) => assert!((left - right.as_f64().unwrap()).abs() < 1e-9, "{id}: {key} {left}"),
                (XlsxCellValue::InlineString(left), serde_json::Value::String(right)) => assert_eq!(left, right, "{id}: {key}"),
                (XlsxCellValue::Error(left), serde_json::Value::Object(right)) => assert_eq!(left, right["error"].as_str().unwrap(), "{id}: {key}"),
                other => panic!("{id}: {key} cache {other:?}"),
            }
        }
        for inverse in inverses.iter().rev().flatten() {
            assert!(apply_xlsx_mutation(&mut after, inverse).messages().is_empty(), "{id}: undo refused");
        }
        assert_eq!(formula_cells(&after).into_iter().map(|(key, (expr, _))| (key, expr)).collect::<Vec<_>>(), before.iter().map(|(key, (expr, _))| (key.clone(), expr.clone())).collect::<Vec<_>>(), "{id}: undo restores the formula text");
    }
}

#[test]
fn sheet_reference_text_rewrites_follow_excel_quoting_rules() {
    use recalculation::rewrite_sheet_references as rewrite;
    assert_eq!(rewrite("'Old Name'!A1:B2+Sheet1!A1", "Old Name", Some("Plain")), "Plain!A1:B2+Sheet1!A1");
    assert_eq!(rewrite("Sheet2!A1", "Sheet2", Some("R1C1")), "'R1C1'!A1");
    assert_eq!(rewrite("Sheet2!A1", "Sheet2", Some("TRUE")), "'TRUE'!A1");
    assert_eq!(rewrite("Sheet2!$A$1:$C$3", "Sheet2", None), "#REF!");
    assert_eq!(rewrite("Sheet2!A:C+1", "Sheet2", None), "#REF!+1");
}
