use super::*;

/// 🧾️ One expected row of `🧫️fixtures/🧫️number-facets`, read field by field (absent = none / default).
fn expected_facets(value: &serde_json::Value) -> Option<ActionArgNumberFacets> {
    let facets = value.as_object()?;
    let number = |key: &str| facets.get(key).and_then(serde_json::Value::as_f64);
    let text = |key: &str| facets.get(key).and_then(serde_json::Value::as_str).map(ToOwned::to_owned);
    Some(ActionArgNumberFacets {
        min: number("min"),
        max: number("max"),
        step: number("step"),
        appearance: serde_json::from_value(facets["appearance"].clone()).expect("fixture appearance"),
        scale: serde_json::from_value(facets["scale"].clone()).expect("fixture scale"),
        unit: text("unit"),
        display_unit: text("displayUnit"),
        display_factor: number("displayFactor"),
        precision: facets.get("precision").and_then(serde_json::Value::as_u64).map(|precision| u16::try_from(precision).expect("fixture precision")),
        snaps: facets["snaps"].as_array().expect("fixture snaps").iter().map(|snap| snap.as_f64().expect("fixture snap")).collect(),
        limits: serde_json::from_value(facets["limits"].clone()).expect("fixture limits"),
    })
}

#[test]
fn every_descriptor_derives_the_number_facets_of_the_shared_corpus() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️number-facets/🔣️.json")).expect("number-facets fixture");
    let cases = fixture["cases"].as_array().expect("cases");
    assert!(cases.len() >= 9, "the corpus covers dial, log slider, steppers, number field, vector, step/precision edges and non-numbers");
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let def: ActionArgDef = serde_json::from_value(case["def"].clone()).unwrap_or_else(|error| panic!("{name}: descriptor deserializes: {error}"));
        let locale = match case["locale"].as_str().expect("locale") {
            "en" => Locale::En,
            "de" => Locale::De,
            other => panic!("{name}: unknown locale {other}"),
        };
        assert_eq!(def.number_facets(locale), expected_facets(&case["facets"]), "{name}");
    }
}

#[test]
fn every_facet_row_keeps_the_contract_number_range_and_detent_laws() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️number-facets/🔣️.json")).expect("number-facets fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let Some(facets) = serde_json::from_value::<ActionArgDef>(case["def"].clone()).expect("descriptor").number_facets(Locale::En) else { continue };
        let travel = (facets.min.unwrap_or(f64::NEG_INFINITY), facets.max.unwrap_or(f64::INFINITY));
        assert!(semio_framework_ui_contract::snaps_are_valid(facets.snaps.iter().copied(), travel.0, travel.1), "{name}: detents obey the detent law");
        assert!(semio_framework_ui_contract::number_range_is_valid(facets.min, facets.max, facets.scale, facets.display_factor, Some(&facets.limits), facets.snaps.iter().copied()), "{name}: the limits admit the key range and every detent");
    }
}

#[test]
fn the_unit_symbols_read_as_shown() {
    assert_eq!(action_arg_unit_symbol("deg"), "°");
    assert_eq!(action_arg_unit_symbol("degrees"), "°");
    assert_eq!(action_arg_unit_symbol("percent"), "%");
    assert_eq!(action_arg_unit_symbol("mm"), "mm");
}
