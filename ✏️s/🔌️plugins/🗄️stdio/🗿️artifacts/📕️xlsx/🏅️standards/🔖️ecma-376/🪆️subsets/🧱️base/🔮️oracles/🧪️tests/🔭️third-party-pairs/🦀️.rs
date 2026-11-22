//! 🔭️ Third-party pairs: the oracle's own computation of every mutation, judged against the committed before/after packages a third-party tool wrote. See [`super`].

use super::*;
use semio_repo_test_host::parse_json;
use std::path::PathBuf;

const TABLE: &str = include_str!("../../../🧪️tests/🧫️fixtures/🔭️third-party-pairs/🔣️.json");
const REAL_WORKBOOK: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/📕️reuse-marketplaces.xlsx");

fn fixture_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures")
}

fn package(directory: &str, name: &str) -> Vec<u8> {
    let path = fixture_directory().join(directory).join(name);
    std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

pub(super) fn pairs() -> Vec<Json> {
    let table = parse_json(TABLE).expect("the third-party pair table is JSON");
    assert_eq!(table.str("schema"), "semio.xlsx.third-party-pairs/v1");
    table.array("pairs")
}

/// 🔭️ Every committed pair, reproduced: the oracle applies the wire mutation to the pair's `before`, and the result must read identically to the `after` a third-party
/// tool wrote — through `calamine` for the grid kinds, through the plumbing projection for the four OPC kinds — or, for a rejected pair, the oracle must refuse it.
pub(super) fn reproduce_pairs() -> usize {
    let mut reproduced = 0;
    for pair in pairs() {
        let (id, directory) = (pair.str("id"), pair.str("fixture"));
        let mutation = pair.get("mutation").expect("a pair carries its mutation").clone();
        let before = package(&directory, "⬅️before.xlsx");
        let produced = oracle_apply_mutation(&before, &mutation);
        if pair.str("outcome") == "rejected" {
            assert!(produced.is_err(), "{id}: the oracle accepted a mutation its pair records as rejected");
            reproduced += 1;
            continue;
        }
        let produced = produced.unwrap_or_else(|error| panic!("{id}: the oracle refused the mutation its pair records as applied: {error}"));
        let after = package(&directory, "➡️after.xlsx");
        match pair.str("compare").as_str() {
            "grid" => {
                assert_ne!(read_workbook_grid(&before).unwrap(), read_workbook_grid(&produced).unwrap(), "{id}: the mutation is not observable through calamine");
                assert_eq!(read_workbook_grid(&produced).unwrap(), read_workbook_grid(&after).unwrap(), "{id}: the oracle's result does not read like the third-party after through calamine");
            }
            "plumbing" => assert_eq!(project_xlsx_plumbing(&produced).unwrap(), project_xlsx_plumbing(&after).unwrap(), "{id}: the oracle's plumbing is not the third-party after's"),
            other => panic!("{id}: unknown comparison {other:?}"),
        }
        reproduced += 1;
    }
    reproduced
}

#[test]
fn the_oracle_reproduces_every_third_party_pair() {
    assert_eq!(reproduce_pairs(), pairs().len());
    assert!(pairs().len() >= 20, "the table must keep covering the grid kinds, insert-cell, set-shared-string and the four plumbing kinds");
}

#[test]
fn the_pair_table_covers_the_kinds_the_audit_named() {
    let kinds: Vec<String> = pairs().iter().map(|pair| pair.get("mutation").map(|mutation| mutation.str("kind")).unwrap_or_default()).collect();
    for kind in ["insert-cell", "set-shared-string", "set-relationship", "remove-relationship", "set-content-type", "remove-content-type"] {
        assert!(kinds.iter().any(|candidate| candidate == kind), "no third-party pair covers {kind}");
    }
}

#[test]
fn a_pair_is_not_satisfied_by_an_unmutated_before() {
    let pair = pairs().into_iter().find(|pair| pair.str("id") == "set-relationship-inserts-an-external-hyperlink-first").expect("the pair exists");
    let before = package(&pair.str("fixture"), "⬅️before.xlsx");
    let after = package(&pair.str("fixture"), "➡️after.xlsx");
    assert_ne!(project_xlsx_plumbing(&before).unwrap(), project_xlsx_plumbing(&after).unwrap(), "a pair whose before already equals its after would prove nothing");
}

fn vacancy(node_path: &[f64], revision: &str, row: f64, column: f64, value: Json) -> Json {
    let worksheet = Json::Object(vec![
        ("partPath".to_string(), Json::String("xl/worksheets/sheet1.xml".to_string())),
        ("nodePath".to_string(), Json::Array(node_path.iter().map(|index| Json::Number(*index)).collect())),
        ("namespaceUri".to_string(), Json::String("http://schemas.openxmlformats.org/spreadsheetml/2006/main".to_string())),
        ("localName".to_string(), Json::String("sheetData".to_string())),
        ("revision".to_string(), Json::String(revision.to_string())),
    ]);
    let address = Json::Object(vec![("worksheet".to_string(), worksheet), ("row".to_string(), Json::Number(row)), ("column".to_string(), Json::Number(column))]);
    Json::Object(vec![("kind".to_string(), Json::String("insert-cell".to_string())), ("params".to_string(), Json::Object(vec![("address".to_string(), address), ("value".to_string(), value)]))])
}

fn number_value(value: f64) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("number".to_string())), ("value".to_string(), Json::Number(value))])
}

fn grid_cell<'a>(sheets: &'a [GridSheet], sheet: &str, row: u32, col: u32) -> Option<&'a GridValue> {
    sheets.iter().find(|(name, _)| name == sheet)?.1.iter().find(|(r, c, _)| *r == row && *c == col).map(|(_, _, value)| value)
}

/// ➕️ `insert-cell` on the real 51-row workbook: into a row the sheet does not have, and into a row it has, each read back through `calamine`, and each undone by the
/// reference's own inverse.
#[test]
fn insert_cell_fills_a_vacancy_of_the_real_workbook_and_is_undone() {
    let sheet_data = "ecd99b57eb5d4205";
    let before = read_workbook_grid(REAL_WORKBOOK).unwrap();
    for (row, column, number, at) in [(52.0, 0.0, 42.0, (51, 0)), (2.0, 12.0, 7.5, (1, 12))] {
        let forward = vacancy(&[3.0], sheet_data, row, column, number_value(number));
        let inserted = oracle_apply_mutation(REAL_WORKBOOK, &forward).unwrap_or_else(|error| panic!("row {row} column {column}: {error}"));
        let grid = read_workbook_grid(&inserted).unwrap();
        assert_eq!(grid_cell(&grid, "Marktplätze", at.0, at.1), Some(&GridValue::Number(number)));
        assert_eq!(grid[0].1.len(), before[0].1.len() + 1, "exactly one cell arrived");
        assert_eq!(grid[1], before[1], "the other sheet is untouched");
        let restored = oracle_apply_inverse(REAL_WORKBOOK, &inserted, &forward).unwrap();
        assert_eq!(read_workbook_grid(&restored).unwrap(), before, "the inverse removes the inserted cell");
    }
}

/// 🚫️ An occupied coordinate, a stale part, a non-`sheetData` node and a value the pairing cannot write are errors, never a silent overwrite.
#[test]
fn insert_cell_refuses_what_is_not_a_vacancy() {
    let occupied = vacancy(&[3.0], "ecd99b57eb5d4205", 2.0, 0.0, number_value(1.0));
    assert!(oracle_apply_mutation(REAL_WORKBOOK, &occupied).unwrap_err().contains("not vacant"));
    let not_sheet_data = vacancy(&[0.0], "ecd99b57eb5d4205", 52.0, 0.0, number_value(1.0));
    assert!(oracle_apply_mutation(REAL_WORKBOOK, &not_sheet_data).is_err());
    let missing_row = Json::Object(vec![("kind".to_string(), Json::String("insert-cell".to_string())), ("params".to_string(), Json::Object(vec![("value".to_string(), number_value(1.0))]))]);
    assert!(oracle_apply_mutation(REAL_WORKBOOK, &missing_row).is_err());
}
