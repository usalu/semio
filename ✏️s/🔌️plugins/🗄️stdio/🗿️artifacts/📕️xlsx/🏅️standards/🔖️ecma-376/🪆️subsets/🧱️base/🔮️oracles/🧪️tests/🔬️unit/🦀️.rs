
use super::*;

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}
fn cell(row: f64, col: f64, value: Json) -> Json {
    Json::Object(vec![("row".to_string(), Json::Number(row)), ("col".to_string(), Json::Number(col)), ("value".to_string(), value)])
}
fn sheet(name: &str, cells: Vec<Json>) -> Json {
    Json::Object(vec![("name".to_string(), Json::String(name.to_string())), ("cells".to_string(), Json::Array(cells))])
}

/// 🧫️ The REAL committed workbook this subset's case runs on — 11 parts, two worksheets and a
/// genuine 229-entry shared-string table. The synthetic [`fixture_bytes`] below is a
/// `rust_xlsxwriter` build whose pool holds one entry, which is the right input for the grid
/// kinds and the wrong one for anything that measures the pool.
fn real_fixture_bytes() -> Vec<u8> {
    std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧫️fixtures/📕️reuse-marketplaces.xlsx")).expect("the committed reuse-marketplaces workbook")
}

fn fixture_bytes() -> Vec<u8> {
    write_workbook_grid(&[("Sheet1".to_string(), vec![(0, 0, GridValue::Text("hello".to_string())), (0, 1, GridValue::Number(1.0)), (1, 0, GridValue::Bool(true))])]).unwrap()
}

#[test]
fn no_mutation_is_a_true_byte_identity() {
    let input = fixture_bytes();
    let output = oracle_apply_mutation(&input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
    assert_eq!(output, input);
}

#[test]
fn insert_and_remove_sheet_are_real_transformations() {
    let input = fixture_bytes();
    let inserted = oracle_apply_mutation(&input, &spec("insert-sheet", sheet("New", vec![cell(1.0, 0.0, Json::String("fresh".to_string()))]))).unwrap();
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
fn set_and_remove_cell_are_real_transformations() {
    let input = fixture_bytes();
    let set = oracle_apply_mutation(
        &input,
        &spec("set-cell", Json::Object(vec![("sheetName".to_string(), Json::String("Sheet1".to_string())), ("row".to_string(), Json::Number(1.0)), ("col".to_string(), Json::Number(0.0)), ("value".to_string(), Json::String("changed".to_string()))])),
    )
    .unwrap();
    let grid = read_workbook_grid(&set).unwrap();
    assert!(grid[0].1.contains(&(0, 0, GridValue::Text("changed".to_string()))));

    let removed = oracle_apply_mutation(&set, &spec("remove-cell", Json::Object(vec![("sheetName".to_string(), Json::String("Sheet1".to_string())), ("row".to_string(), Json::Number(1.0)), ("col".to_string(), Json::Number(0.0))]))).unwrap();
    assert!(!read_workbook_grid(&removed).unwrap()[0].1.iter().any(|(r, c, _)| *r == 0 && *c == 0));
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
