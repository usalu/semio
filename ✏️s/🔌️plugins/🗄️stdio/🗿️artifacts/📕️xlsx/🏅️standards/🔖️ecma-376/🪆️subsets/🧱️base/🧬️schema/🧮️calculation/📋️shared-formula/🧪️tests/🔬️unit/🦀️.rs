use super::*;

#[test]
fn translation_fixture_matches_the_openpyxl_translator() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/📋️shared-formulas/🔣️.json")).unwrap();
    for case in fixture["translations"].as_array().unwrap() {
        let at = |reference: &str| {
            let letters = reference.trim_end_matches(|c: char| c.is_ascii_digit());
            (reference[letters.len()..].parse::<u32>().unwrap(), column_index(letters).unwrap())
        };
        let ((from_row, from_col), (to_row, to_col)) = (at(case["from"].as_str().unwrap()), at(case["to"].as_str().unwrap()));
        let master = SharedMaster { expr: case["master"].as_str().unwrap().into(), row: from_row, col: from_col };
        assert_eq!(expand_shared_formula(&master, to_row, to_col), case["expected"].as_str().unwrap(), "{} from {} to {}", case["master"], case["from"], case["to"]);
    }
}

#[test]
fn references_pushed_off_the_grid_become_ref_errors() {
    assert_eq!(translate_formula("A1+B2", -1, 0), "#REF!+B1");
    assert_eq!(translate_formula("A1+$B$2", 0, -1), "#REF!+$B$2");
    assert_eq!(translate_formula("A1", 1_048_576, 0), "#REF!");
    assert_eq!(translate_formula("XFD1", 0, 1), "#REF!");
}

#[test]
fn names_functions_strings_and_sheet_qualifiers_are_not_references() {
    assert_eq!(translate_formula("LOG10(1)+Sheet1!A1+SUM(1.5E3)+TAX_A1+\"A1\"", 1, 1), "LOG10(1)+Sheet1!B2+SUM(1.5E3)+TAX_A1+\"A1\"");
    assert_eq!(translate_formula("'It''s A1'!A1", 1, 0), "'It''s A1'!A2");
}
