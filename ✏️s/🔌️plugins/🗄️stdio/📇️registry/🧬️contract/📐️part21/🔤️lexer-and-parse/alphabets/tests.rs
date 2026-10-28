use super::*;
use crate::{parse_part21, Part21Value};

fn envelope(literal: &str, extra: &str) -> String {
    format!("ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;#1=IFCLABEL('{literal}'{extra});ENDSEC;END-ISO-10303-21;")
}

#[test]
fn complete_neutral_alphabet_cases_use_the_real_reader() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    let rows = cases["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 855);
    for row in rows {
        let literal = row["source"].as_str().unwrap();
        let result = parse_part21(&envelope(literal, ""));
        if let Some(expected) = row["expected"].as_str() {
            let document = result.unwrap_or_else(|error| panic!("{}: {error}", row["name"]));
            assert_eq!(document.instances[0].entities[0].1[0], Part21Value::Str(expected.into()), "{}", row["name"]);
        } else { assert!(result.is_err(), "{}", row["name"]); }
    }
}

#[test]
fn page_selection_resets_and_out_of_alphabet_payloads_refuse() {
    let document = parse_part21(&envelope(r"\PB\\S\!", r",'\S\!'")).unwrap();
    assert_eq!(document.instances[0].entities[0].1, vec![Part21Value::Str("Ą".into()), Part21Value::Str("¡".into())]);
    for payload in ['\u{7f}', '\u{80}', 'é'] { assert!(parse_part21(&envelope(&format!("\\S\\{payload}"), "")).is_err()); }
    assert_eq!(part21_alphabet_character('J', '!'), None);
}

#[test]
fn neutral_print_and_ignored_character_cases_use_the_real_reader() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../../🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/control-cases.json")).unwrap();
    for row in cases["lexical"].as_array().unwrap() {
        let data = row["data"].as_str().unwrap();
        let source = envelope("", "").replace("#1=IFCLABEL('');", data);
        let document = parse_part21(&source).unwrap_or_else(|error| panic!("{}: {error}", row["name"]));
        assert_eq!(document.instances[0].entities[0].0, row["nameAfter"].as_str().unwrap());
        let expected = if let Some(value) = row["binary"].as_str() { Part21Value::Binary(value.into()) } else { Part21Value::Str(row["value"].as_str().unwrap().into()) };
        assert_eq!(document.instances[0].entities[0].1[0], expected, "{}", row["name"]);
    }
}

#[test]
fn lexical_errors_use_original_utf8_bytes_and_reals_require_decimal_mantissa() {
    let source = envelope("ä", ",\r\n?\t");
    let error = parse_part21(&source).unwrap_err();
    let crate::Part21Error::UnexpectedChar { at, .. } = error else { panic!("unexpected refusal {error}") };
    assert_eq!(at, source.find('?').unwrap());
    for literal in ["1E5", "+1E-2", "-.5", ".5"] { assert!(parse_part21(&envelope("", &format!(",{literal}"))).is_err(), "{literal}"); }
}
