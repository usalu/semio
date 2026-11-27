use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxSheet};

fn workbook_of(sheets: &[(&str, &[(&str, XlsxCellValue)])]) -> XlsxWorkbook {
    XlsxWorkbook {
        sheets: sheets
            .iter()
            .map(|(name, cells)| XlsxSheet {
                name: (*name).into(),
                cells: cells.iter().map(|(reference, value)| { let cell = cell_of(reference, None).unwrap(); XlsxCell { row: cell.row, col: cell.col, value: value.clone() } }).collect(),
            })
            .collect(),
        shared_strings: vec!["shared".into()],
    }
}

fn formula(expr: &str) -> XlsxCellValue {
    XlsxCellValue::Formula { expr: expr.into(), cached: None }
}

fn evaluate(expr: &str) -> XlsxCellValue {
    let workbook = workbook_of(&[("S", &[("A1", XlsxCellValue::Number(2.0)), ("A2", XlsxCellValue::InlineString("x".into())), ("A3", XlsxCellValue::Number(4.0)), ("Z9", formula(expr))])]);
    recalculate_workbook(&workbook).into_iter().find(|cell| cell.row == 9).map(|cell| cell.cached).unwrap_or(XlsxCellValue::Empty)
}

fn number(value: f64) -> XlsxCellValue {
    XlsxCellValue::Number(value)
}

#[test]
fn operators_follow_excel_precedence() {
    assert_eq!(evaluate("1+2*3"), number(7.0));
    assert_eq!(evaluate("-2^2"), number(4.0));
    assert_eq!(evaluate("2^3^2"), number(64.0));
    assert_eq!(evaluate("50%*2"), number(1.0));
    assert_eq!(evaluate("1+2&3+4"), XlsxCellValue::InlineString("37".into()));
    assert_eq!(evaluate("1+1=2"), XlsxCellValue::Boolean(true));
    assert_eq!(evaluate("TRUE+1"), number(2.0));
}

#[test]
fn references_ranges_and_sheet_qualifiers_resolve() {
    assert_eq!(evaluate("$A$1+A$3+$A1"), number(8.0));
    assert_eq!(evaluate("SUM(A1:A3)"), number(6.0));
    let workbook = workbook_of(&[("S", &[("A1", formula("'My Sheet'!B2+Other!B2"))]), ("My Sheet", &[("B2", number(5.0))]), ("Other", &[("B2", number(1.0))])]);
    assert_eq!(recalculate_workbook(&workbook)[0].cached, number(6.0));
}

#[test]
fn errors_and_unknown_names_surface_as_excel_errors() {
    assert_eq!(evaluate("1/0"), XlsxCellValue::Error("#DIV/0!".into()));
    assert_eq!(evaluate("A2+1"), XlsxCellValue::Error("#VALUE!".into()));
    assert_eq!(evaluate("NOPE(1)"), XlsxCellValue::Error("#NAME?".into()));
    assert_eq!(evaluate("missing+1"), XlsxCellValue::Error("#NAME?".into()));
    assert_eq!(evaluate("Gone!A1"), XlsxCellValue::Error("#REF!".into()));
    assert_eq!(evaluate("IFERROR(1/0,7)"), number(7.0));
    assert_eq!(evaluate("AVERAGE(B1:B2)"), XlsxCellValue::Error("#DIV/0!".into()));
}

#[test]
fn functions_ignore_text_in_ranges_and_round_half_away_from_zero() {
    assert_eq!(evaluate("COUNT(A1:A3)"), number(2.0));
    assert_eq!(evaluate("COUNTA(A1:A3)"), number(3.0));
    assert_eq!(evaluate("ROUND(2.5,0)"), number(3.0));
    assert_eq!(evaluate("ROUND(-2.5,0)"), number(-3.0));
    assert_eq!(evaluate("ROUND(1.005,2)"), number(1.01));
    assert_eq!(evaluate("ROUND(1234,-2)"), number(1200.0));
    assert_eq!(evaluate("CONCAT(A1:A3,\"!\")"), XlsxCellValue::InlineString("2x4!".into()));
    assert_eq!(evaluate("IF(A1>1,\"big\",\"small\")"), XlsxCellValue::InlineString("big".into()));
}

#[test]
fn circular_references_keep_their_cache_and_stale_values_are_reported() {
    let workbook = workbook_of(&[(
        "S",
        &[
            ("A1", formula("B1+1")),
            ("B1", formula("A1+1")),
            ("C1", formula("A1+1")),
            ("D1", XlsxCellValue::Number(3.0)),
            ("E1", XlsxCellValue::Formula { expr: "D1*2".into(), cached: Some(Box::new(number(6.0))) }),
            ("F1", XlsxCellValue::Formula { expr: "D1*3".into(), cached: Some(Box::new(number(1.0))) }),
        ],
    )]);
    let changed = recalculate_workbook(&workbook);
    assert_eq!(changed.iter().map(|cell| (cell.col, cell.cached.clone())).collect::<Vec<_>>(), vec![(5, number(9.0))]);
}

#[test]
fn shared_string_cache_equal_to_computed_text_is_not_stale() {
    let workbook = workbook_of(&[("S", &[("A1", XlsxCellValue::Formula { expr: "\"shared\"".into(), cached: Some(Box::new(XlsxCellValue::SharedString(0))) })])]);
    assert!(recalculate_workbook(&workbook).is_empty());
}

fn fixture_workbook(case: &serde_json::Value) -> XlsxWorkbook {
    let mut workbook = XlsxWorkbook::default();
    for sheet in case["sheets"].as_array().unwrap() {
        let cells = sheet["cells"].as_object().unwrap().iter().map(|(reference, raw)| {
            let cell = cell_of(reference, None).unwrap();
            let value = match raw {
                serde_json::Value::Number(number) => XlsxCellValue::Number(number.as_f64().unwrap()),
                serde_json::Value::Bool(flag) => XlsxCellValue::Boolean(*flag),
                serde_json::Value::String(text) => match text.strip_prefix('=') {
                    Some(expr) => XlsxCellValue::Formula { expr: expr.into(), cached: None },
                    None => XlsxCellValue::InlineString(text.clone()),
                },
                other => panic!("unsupported fixture cell {other}"),
            };
            XlsxCell { row: cell.row, col: cell.col, value }
        });
        workbook.sheets.push(XlsxSheet { name: sheet["name"].as_str().unwrap().into(), cells: cells.collect() });
    }
    workbook
}

fn expected_cache(expected: &serde_json::Value) -> XlsxCellValue {
    match expected {
        serde_json::Value::Number(number) => XlsxCellValue::Number(number.as_f64().unwrap()),
        serde_json::Value::Bool(flag) => XlsxCellValue::Boolean(*flag),
        serde_json::Value::String(text) => XlsxCellValue::InlineString(text.clone()),
        other => XlsxCellValue::Error(other["error"].as_str().unwrap().into()),
    }
}

#[test]
fn formula_recalculation_fixture_matches_the_independent_formulas_library() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🧮️formula-recalculation/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let computed = recalculate_workbook(&fixture_workbook(case));
        for (key, expected) in case["expected"].as_object().unwrap() {
            let (sheet, reference) = key.rsplit_once('!').unwrap();
            let target = cell_of(reference, None).unwrap();
            let actual = computed.iter().find(|cell| cell.sheet == sheet && cell.row == target.row && cell.col == target.col).unwrap_or_else(|| panic!("{}: {key} not computed", case["id"]));
            let expected = expected_cache(expected);
            match (&actual.cached, &expected) {
                (XlsxCellValue::Number(left), XlsxCellValue::Number(right)) => assert!((left - right).abs() <= 1e-9 * right.abs().max(1.0), "{}: {key} {left} != {right}", case["id"]),
                _ => assert_eq!(actual.cached, expected, "{}: {key}", case["id"]),
            }
        }
    }
}
