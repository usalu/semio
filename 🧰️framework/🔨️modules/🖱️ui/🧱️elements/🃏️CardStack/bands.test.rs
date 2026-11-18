use super::*;
use serde_json::Value;

fn spans(value: &Value) -> Vec<CardBandSpan> {
    value.as_array().unwrap().iter().map(|span| CardBandSpan { left: span["left"].as_f64().unwrap(), right: span["right"].as_f64().unwrap() }).collect()
}

#[test]
fn card_band_language_neutral_vectors() {
    let fixture: Value = serde_json::from_str(include_str!("bands.cases.json")).unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert!(!cases.is_empty());
    for case in cases {
        let cards: Vec<_> = case["cards"].as_array().unwrap().iter().map(|card| CardBandCard { cap: card["cap"].as_f64().unwrap(), footer: card["footer"].as_f64().unwrap(), cap_chips: spans(&card["capChips"]), footer_chips: spans(&card["footerChips"]) }).collect();
        let expected: Vec<_> = case["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|band| CardBand {
                mode: match band["mode"].as_str().unwrap() {
                    "none" => CardBandMode::None,
                    "shared" => CardBandMode::Shared,
                    "stacked" => CardBandMode::Stacked,
                    other => panic!("unknown mode {other}"),
                },
                lift: band["lift"].as_f64().unwrap(),
                cap: band["cap"].as_f64().unwrap(),
                footer: band["footer"].as_f64().unwrap(),
            })
            .collect();
        assert_eq!(resolve_card_bands(&cards, case["inset"].as_f64().unwrap_or(0.0)), expected, "{}", case["name"]);
    }
}

#[test]
fn a_single_card_has_no_band() {
    assert!(resolve_card_bands(&[CardBandCard { cap: 28.0, footer: 28.0, cap_chips: vec![], footer_chips: vec![] }], 0.0).is_empty());
}

#[test]
fn card_height_snap_language_neutral_vectors() {
    let fixture: Value = serde_json::from_str(include_str!("bands.cases.json")).unwrap();
    for snap in fixture["snaps"].as_array().unwrap() {
        let got = snap_card_height(snap["height"].as_f64().unwrap(), snap["pixel"].as_f64().unwrap());
        assert!((got - snap["expected"].as_f64().unwrap()).abs() < 1e-9, "{snap}");
    }
}

#[test]
fn chip_gap_language_neutral_vectors() {
    let fixture: Value = serde_json::from_str(include_str!("bands.cases.json")).unwrap();
    for case in fixture["gaps"].as_array().unwrap() {
        assert_eq!(chip_gap_reads_as_sliver(case["gap"].as_f64().unwrap()), case["sliver"].as_bool().unwrap(), "{case}");
    }
}
