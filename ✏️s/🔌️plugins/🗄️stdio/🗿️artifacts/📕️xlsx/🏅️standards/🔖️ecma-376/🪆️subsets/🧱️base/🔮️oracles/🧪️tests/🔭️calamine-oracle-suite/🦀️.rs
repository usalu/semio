//! 🔭️ The `s.stdio.xlsx.conformance-suite.calamine-oracle` suite: every fixture the artifact definition declares for it, read through `calamine` and held to the
//! committed language-agnostic expectation. See [`super`].

use super::*;
use semio_repo_test_host::parse_json;
use semio_s_plugin_stdio_document_test_oracle::ooxml::write_parts;
use std::path::PathBuf;

const REAL_WORKBOOK: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/📕️reuse-marketplaces.xlsx");
const NATIVE_DOCUMENT: &str = include_str!("../../../🧪️tests/🧫️fixtures/📕️native-document/🔣️.json");
const CANONICAL_XML_SAVE: &str = include_str!("../../../🧪️tests/🧫️fixtures/🧬️canonical-xml-save/🔣️.json");
const VIEWER_CELL_WINDOW: &str = include_str!("../../../🧪️tests/🧫️fixtures/🪟️viewer-cell-window/🔣️.json");

const SUITE: &str = "s.stdio.xlsx.conformance-suite.calamine-oracle";
const COVERED: [&str; 5] = [
    "s.stdio.xlsx.fixture.native-document",
    "s.stdio.xlsx.fixture.canonical-xml-save",
    "s.stdio.xlsx.fixture.mutations",
    "s.stdio.xlsx.fixture.viewer-cell-window",
    "s.stdio.xlsx.fixture.reuse-marketplaces",
];

fn json(source: &str) -> Json {
    parse_json(source).expect("a committed fixture is JSON")
}

fn calamine_range(bytes: &[u8], sheet: &str) -> calamine::Range<calamine::Data> {
    use calamine::{Reader, Xlsx};
    let mut workbook: Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(bytes.to_vec())).expect("calamine opens the package");
    workbook.worksheet_range(sheet).unwrap_or_else(|error| panic!("calamine cannot read sheet {sheet:?}: {error}"))
}

/// 🪡️ A worksheet without its cell-level `extLst`, which `calamine` 0.36 refuses (`cells_reader.rs`: "v, f, or is") although ECMA-376 Part 1 §18.3.1.4 allows it. The values
/// calamine answers are unaffected, so the reader is held to the same worksheet minus that one extension.
fn without_cell_extensions(text: &str) -> String {
    let mut text = text.to_string();
    for prefix in ["", "s:"] {
        let (open, close, cell_end) = (format!("<{prefix}extLst>"), format!("</{prefix}extLst>"), format!("</{prefix}c>"));
        let mut from = 0;
        while let Some(found) = text[from..].find(&open) {
            let start = from + found;
            let Some(end) = text[start..].find(&close).map(|end| start + end + close.len()) else { break };
            if text[end..].starts_with(&cell_end) {
                text.replace_range(start..end, "");
                from = start;
            } else {
                from = end;
            }
        }
    }
    text
}

fn package_of(case: &Json, sheet_xml: Option<&str>) -> Vec<u8> {
    let sheet_path = case.str("sheetPath");
    let mut parts: Vec<(String, Vec<u8>)> = case.array("parts").iter().map(|part| (part.str("path"), if part.str("path") == sheet_path { without_cell_extensions(&part.str("text")) } else { part.str("text") }.into_bytes())).collect();
    for binary in case.array("binaryParts") {
        let bytes = binary.array("bytes").iter().map(|byte| if let Json::Number(n) = byte { *n as u8 } else { 0 }).collect();
        parts.push((binary.str("path"), bytes));
    }
    if let Some(xml) = sheet_xml {
        parts.iter_mut().find(|(name, _)| *name == sheet_path).expect("the case carries its sheet part").1 = without_cell_extensions(xml).into_bytes();
    }
    write_parts(&parts).expect("the case's parts assemble into a package")
}

fn display(value: &calamine::Data) -> String {
    use calamine::Data;
    match value {
        Data::Int(n) => n.to_string(),
        Data::Float(n) => n.to_string(),
        Data::String(text) => text.clone(),
        Data::Bool(flag) => flag.to_string(),
        Data::Error(error) => error.to_string(),
        other => format!("{other:?}"),
    }
}

fn kind_of(value: &calamine::Data) -> &'static str {
    use calamine::Data;
    match value {
        Data::Int(_) | Data::Float(_) => "number",
        Data::String(_) => "string",
        Data::Bool(_) => "boolean",
        Data::Error(_) => "error",
        _ => "other",
    }
}

fn a1(address: &str) -> (u32, u32) {
    let split = address.find(|c: char| c.is_ascii_digit()).expect("an A1 address has a row");
    let column = address[..split].bytes().fold(0u32, |acc, letter| acc * 26 + u32::from(letter - b'A' + 1));
    (address[split..].parse::<u32>().expect("a numeric row") - 1, column - 1)
}

/// 📕️ The real two-sheet survey workbook, as the reference reader sees it.
fn reuse_marketplaces() {
    use calamine::{Reader, Xlsx};
    let workbook: Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(REAL_WORKBOOK.to_vec())).unwrap();
    assert_eq!(workbook.sheet_names(), vec!["Marktplätze".to_string(), "Länderübersicht".to_string()]);
    let survey = calamine_range(REAL_WORKBOOK, "Marktplätze");
    assert_eq!(survey.used_cells().count(), 51 * 12, "50 survey rows plus the header, 12 columns");
    assert_eq!(survey.get_value((0, 0)), Some(&calamine::Data::String("ID".to_string())));
    assert_eq!(survey.get_value((0, 11)), Some(&calamine::Data::String("Uebergabe".to_string())));
    assert_eq!(calamine_range(REAL_WORKBOOK, "Länderübersicht").used_cells().count(), 9 * 2);
}

/// 🧬️ Every case the fixture marks `calamine`: the package assembled from its parts reads to the committed values before the edit, and the committed edited sheet reads to
/// the committed after value, formulas aside.
fn canonical_xml_save() {
    let mut checked = 0;
    for case in json(CANONICAL_XML_SAVE).array("cases") {
        if case.get("calamine") != Some(&Json::Bool(true)) {
            continue;
        }
        let sheet = case.get("edit").map(|edit| edit.str("sheetName")).unwrap();
        for (phase, bytes) in [("before", package_of(&case, None)), ("after", package_of(&case, Some(&case.str("expectedSheetXml"))))] {
            let range = calamine_range(&bytes, &sheet);
            for expected in case.array("expectedValues") {
                let at = a1(&expected.str("address"));
                let value = range.get_value(at).unwrap_or_else(|| panic!("{} {phase}: calamine reads nothing at {}", case.str("id"), expected.str("address")));
                assert_eq!(kind_of(value), expected.str("kind"), "{} {phase} {}", case.str("id"), expected.str("address"));
                let want = expected.str(phase);
                let matches = match value {
                    calamine::Data::Float(n) => want.parse::<f64>().is_ok_and(|w| (w - n).abs() < 1e-9),
                    calamine::Data::Int(n) => want.parse::<i64>().is_ok_and(|w| w == *n),
                    other => display(other) == want,
                };
                assert!(matches, "{} {phase} {}: calamine reads {:?}, the fixture says {want:?}", case.str("id"), expected.str("address"), value);
            }
        }
        checked += 1;
    }
    assert!(checked >= 4, "the fixture marks at least four cases for calamine");
}

/// 📥️ The implicit-coordinate worksheet of the native-document fixture, placed into the committed canonical package, reads to the committed cell list.
fn native_document() {
    let fixture = json(NATIVE_DOCUMENT);
    let implicit = fixture.get("implicitCoordinates").expect("the fixture carries implicit coordinates").clone();
    let template = json(CANONICAL_XML_SAVE).array("cases").into_iter().find(|case| case.str("id") == "transitional-default").expect("the canonical template case");
    let bytes = package_of(&template, Some(&implicit.str("worksheet")));
    let range = calamine_range(&bytes, "Data");
    let (origin_row, origin_col) = range.start().unwrap_or((0, 0));
    let read: Vec<(u32, u32, f64)> = range.used_cells().map(|(row, col, value)| (origin_row + row as u32, origin_col + col as u32, if let calamine::Data::Float(n) = value { *n } else if let calamine::Data::Int(n) = value { *n as f64 } else { f64::NAN })).collect();
    let expected: Vec<(u32, u32, f64)> = implicit
        .array("cells")
        .iter()
        .map(|cell| match cell {
            Json::Array(items) => match items.as_slice() {
                [Json::Number(row), Json::Number(col), Json::Number(value)] => (*row as u32 - 1, *col as u32, *value),
                other => panic!("a cell is [row, column, value], got {other:?}"),
            },
            other => panic!("a cell is an array, got {other:?}"),
        })
        .collect();
    assert_eq!(read, expected, "calamine resolves the implicit coordinates exactly as the fixture does");
}

/// 🪟️ The viewer fixture's sheets, written by the reference writer and read back by the reference reader, give the committed window.
fn viewer_cell_window() {
    let fixture = json(VIEWER_CELL_WINDOW);
    let sheets: Vec<GridSheet> = fixture
        .array("sheets")
        .iter()
        .map(|sheet| (sheet.str("name"), sheet.array("values").iter().enumerate().map(|(row, value)| (row as u32, 0, GridValue::Text(if let Json::String(text) = value { text.clone() } else { String::new() }))).collect()))
        .collect();
    let bytes = write_workbook_grid(&sheets).unwrap();
    let index = if let Some(Json::Number(n)) = fixture.get("sheetIndex") { *n as usize } else { 0 };
    let number = |key: &str| if let Some(Json::Number(n)) = fixture.get(key) { *n as usize } else { 0 };
    let range = calamine_range(&bytes, &sheets[index].0);
    let window: Vec<Vec<String>> = (0..number("rows")).map(|row| (0..number("columns")).map(|column| range.get_value(((number("rowOffset") + row) as u32, (number("columnOffset") + column) as u32)).map(display).unwrap_or_default()).collect()).collect();
    let expected: Vec<Vec<String>> = fixture.array("expectedCells").iter().map(|row| if let Json::Array(cells) = row { cells.iter().map(|cell| if let Json::String(text) = cell { text.clone() } else { String::new() }).collect() } else { Vec::new() }).collect();
    assert_eq!(window, expected, "the reference reader windows the fixture's sheet to the committed cells");
    for sheet in &sheets {
        let read = calamine_range(&bytes, &sheet.0).used_cells().count();
        assert_eq!(read, sheet.1.len(), "sheet {:?} reads back every value", sheet.0);
    }
}

fn declared_fixtures() -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../📜️artifact-definition.json");
    let definition = json(&std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display())));
    let suite = definition.array("conformance_suites").into_iter().find(|suite| suite.str("id") == SUITE).expect("the artifact definition declares the suite");
    suite.array("fixtures").iter().map(|fixture| if let Json::String(id) = fixture { id.clone() } else { String::new() }).collect()
}

/// 🔭️ The suite: one section per declared fixture, and the declaration must name exactly the fixtures the suite covers.
#[test]
fn the_calamine_oracle_suite_covers_every_declared_fixture() {
    let mut declared = declared_fixtures();
    declared.sort();
    let mut covered: Vec<String> = COVERED.iter().map(|id| id.to_string()).collect();
    covered.sort();
    assert_eq!(declared, covered, "the artifact definition and the suite must name the same fixtures");
    reuse_marketplaces();
    assert!(super::third_party_pairs::reproduce_pairs() >= 20, "the mutations fixture is the third-party pair table");
    canonical_xml_save();
    native_document();
    viewer_cell_window();
}
