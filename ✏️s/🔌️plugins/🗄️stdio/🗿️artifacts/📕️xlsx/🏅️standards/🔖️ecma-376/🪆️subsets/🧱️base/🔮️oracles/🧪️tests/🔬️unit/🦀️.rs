
use super::*;

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}
fn cell(row: f64, col: f64, value: Json) -> Json {
    Json::Object(vec![("row".to_string(), Json::Number(row)), ("col".to_string(), Json::Number(col)), ("value".to_string(), value)])
}
fn sheet(name: &str, cells: Vec<Json>) -> Json {
    Json::Object(vec![("sheet".to_string(), Json::Object(vec![("name".to_string(), Json::String(name.to_string())), ("cells".to_string(), Json::Array(cells))]))])
}
fn inline(text: &str) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("inlineString".to_string())), ("value".to_string(), Json::String(text.to_string()))])
}
/// 🧭️ The subject's own lineage-bound address of `Marktplätze!C3`/`H6` in the real workbook — the same wire the case's
/// feature rows carry.
fn address(node_path: &[f64], revision: &str) -> Json {
    Json::Object(vec![
        ("partPath".to_string(), Json::String("xl/worksheets/sheet1.xml".to_string())),
        ("nodePath".to_string(), Json::Array(node_path.iter().map(|index| Json::Number(*index)).collect())),
        ("namespaceUri".to_string(), Json::String("http://schemas.openxmlformats.org/spreadsheetml/2006/main".to_string())),
        ("localName".to_string(), Json::String("c".to_string())),
        ("revision".to_string(), Json::String(revision.to_string())),
    ])
}

/// 🧫️ The REAL committed workbook this subset's case runs on — 11 parts, two worksheets and a
/// genuine 229-entry shared-string table. The synthetic [`fixture_bytes`] below is a
/// `rust_xlsxwriter` build whose pool holds one entry, which is the right input for the grid
/// kinds and the wrong one for anything that measures the pool.
fn real_fixture_bytes() -> Vec<u8> {
    include_bytes!("../../../🧫️fixtures/📕️reuse-marketplaces.xlsx").to_vec()
}

fn fixture_bytes() -> Vec<u8> {
    write_workbook_grid(&[("Sheet1".to_string(), vec![(0, 0, GridValue::Text("hello".to_string())), (0, 1, GridValue::Number(1.0)), (1, 0, GridValue::Bool(true))])]).unwrap()
}

#[test]
fn insert_and_remove_sheet_are_real_transformations() {
    let input = fixture_bytes();
    let inserted = oracle_apply_mutation(&input, &spec("insert-sheet", sheet("New", vec![cell(1.0, 0.0, inline("fresh"))]))).unwrap();
    let grid = read_workbook_grid(&inserted).unwrap();
    assert_eq!(grid.len(), 2);
    assert_eq!(grid[1].0, "New");

    let removed = oracle_apply_mutation(&inserted, &spec("remove-sheet", Json::Object(vec![("name".to_string(), Json::String("New".to_string()))]))).unwrap();
    assert_eq!(read_workbook_grid(&removed).unwrap().len(), 1);
}

#[test]
fn rename_sheet_changes_only_the_name() {
    let input = fixture_bytes();
    let renamed = oracle_apply_mutation(&input, &spec("rename-sheet", Json::Object(vec![("name".to_string(), Json::String("Sheet1".to_string())), ("newName".to_string(), Json::String("Renamed".to_string()))]))).unwrap();
    let grid = read_workbook_grid(&renamed).unwrap();
    assert_eq!(grid[0].0, "Renamed");
    assert_eq!(grid[0].1.len(), 3);
}

#[test]
fn a_cell_address_resolves_to_its_sheet_and_reference_independently() {
    let input = real_fixture_bytes();
    assert_eq!(cell_address::addressed_cell(&input, &address(&[3.0, 2.0, 2.0], "c10cb4fa36844692")).unwrap(), ("Marktplätze".to_string(), 2, 2));
    assert_eq!(cell_address::addressed_cell(&input, &address(&[3.0, 5.0, 7.0], "b2b766d8b96c5b6b")).unwrap(), ("Marktplätze".to_string(), 5, 7));
    assert!(cell_address::addressed_cell(&input, &address(&[3.0, 2.0], "0")).is_err(), "a path ending on a row is not a cell");
}

#[test]
fn set_and_remove_cell_are_real_transformations_undone_by_the_reference() {
    let input = real_fixture_bytes();
    let forward = spec("set-cell", Json::Object(vec![("address".to_string(), address(&[3.0, 2.0, 2.0], "c10cb4fa36844692")), ("value".to_string(), inline("changed"))]));
    let set = oracle_apply_mutation(&input, &forward).unwrap();
    let grid = read_workbook_grid(&set).unwrap();
    assert!(grid[0].1.contains(&(2, 2, GridValue::Text("changed".to_string()))));
    assert_eq!(read_workbook_grid(&oracle_apply_inverse(&input, &set, &forward).unwrap()).unwrap(), read_workbook_grid(&input).unwrap());

    let forward = spec("remove-cell", Json::Object(vec![("address".to_string(), address(&[3.0, 5.0, 7.0], "b2b766d8b96c5b6b"))]));
    let removed = oracle_apply_mutation(&input, &forward).unwrap();
    assert!(!read_workbook_grid(&removed).unwrap()[0].1.iter().any(|(r, c, _)| *r == 5 && *c == 7));
    assert_eq!(read_workbook_grid(&oracle_apply_inverse(&input, &removed, &forward).unwrap()).unwrap(), read_workbook_grid(&input).unwrap());
}

#[test]
/// 📑️ The three pool kinds really move the real 229-entry `xl/sharedStrings.xml`, and the pool
/// is really read back out of the bytes. This replaces a test that asserted the opposite — that
/// they were a byte identity — which was true only of the `calamine`/`rust_xlsxwriter` pairing
/// and never of the package.
fn shared_string_kinds_move_the_real_pool() {
    let input = real_fixture_bytes();
    let pool = shared_strings::read_pool(&input).expect("the real fixture carries a shared-string pool");
    assert_eq!(pool.len(), 229, "the committed fixture's own uniqueCount");

    let inserted = oracle_apply_mutation(&input, &spec("insert-shared-string", Json::Object(vec![("value".to_string(), Json::String("Ökobau Referenzquelle 2024".to_string()))]))).unwrap();
    let grown = shared_strings::read_pool(&inserted).unwrap();
    assert_eq!(grown.len(), 230);
    assert_eq!(grown.last().map(String::as_str), Some("Ökobau Referenzquelle 2024"));

    let removed = oracle_apply_mutation(&inserted, &spec("remove-shared-string", Json::Object(vec![("index".to_string(), Json::Number(229.0))]))).unwrap();
    assert_eq!(shared_strings::read_pool(&removed).unwrap(), pool, "removing the appended entry restores the pool exactly");

    let set = oracle_apply_mutation(&input, &spec("set-shared-string", Json::Object(vec![("index".to_string(), Json::Number(0.0)), ("value".to_string(), Json::String("Aktualisierter Quellwert".to_string()))]))).unwrap();
    assert_eq!(shared_strings::read_pool(&set).unwrap()[0], "Aktualisierter Quellwert");
}

/// ⚠️ An INTERIOR removal has no inverse in this vocabulary — `InsertSharedString` appends — and
/// the oracle refuses to invent one rather than returning an undo that does not undo.
#[test]
fn an_interior_shared_string_removal_is_refused_an_inverse() {
    let input = real_fixture_bytes();
    let forward = spec("remove-shared-string", Json::Object(vec![("index".to_string(), Json::Number(0.0))]));
    let error = shared_string_inverse_spec(&input, &forward).expect_err("index 0 of 229 is interior");
    assert!(error.contains("no inverse in this vocabulary"), "{error}");
    let last = spec("remove-shared-string", Json::Object(vec![("index".to_string(), Json::Number(228.0))]));
    assert_eq!(shared_string_inverse_spec(&input, &last).unwrap().str("kind"), "insert-shared-string");
}

/// 🕳️ A parameter set that would leave the pool exactly as it was is an error, not a pass.
#[test]
fn a_shared_string_write_that_changes_nothing_is_refused() {
    let input = real_fixture_bytes();
    let pool = shared_strings::read_pool(&input).unwrap();
    let unchanged = spec("set-shared-string", Json::Object(vec![("index".to_string(), Json::Number(0.0)), ("value".to_string(), Json::String(pool[0].clone()))]));
    assert!(oracle_apply_mutation(&input, &unchanged).is_err());
}

#[test]
fn project_xlsx_workbook_carries_the_caller_tracked_shared_string_count() {
    let bytes = fixture_bytes();
    let projection = project_xlsx_workbook(&bytes, 3).unwrap();
    assert_eq!(projection.str("format"), "xlsx");
    assert_eq!(projection.get("sharedStringCount"), Some(&Json::Number(3.0)));
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let input = fixture_bytes();
    let result = oracle_apply_mutation(&input, &spec("not-a-real-kind", Json::Object(vec![])));
    assert!(result.is_err(), "an unrecognised kind must fail loudly");
}

/// 🔗️ A pool entry a cell still references is refused, as the vocabulary refuses it; the arranged pre-state carries one
/// unreferenced entry, and removing THAT one restores the real pool.
#[test]
fn a_referenced_shared_string_is_refused_and_the_arranged_entry_is_removable() {
    let input = real_fixture_bytes();
    let referenced = spec("remove-shared-string", Json::Object(vec![("index".to_string(), Json::Number(228.0))]));
    assert!(oracle_apply_mutation(&input, &referenced).is_err(), "entry 228 is referenced by a cell");
    let forward = spec("remove-shared-string", Json::Object(vec![("index".to_string(), Json::Number(229.0))]));
    let arranged = oracle_arrange(&input, &forward).unwrap();
    assert_eq!(shared_strings::read_pool(&arranged).unwrap().len(), 230);
    let removed = oracle_apply_mutation(&arranged, &forward).unwrap();
    assert_eq!(shared_strings::read_pool(&removed).unwrap(), shared_strings::read_pool(&input).unwrap());
}
