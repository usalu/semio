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
        assert_eq!(resolve_card_bands(&cards), expected, "{}", case["name"]);
    }
}

#[test]
fn a_single_card_has_no_band() {
    assert!(resolve_card_bands(&[CardBandCard { cap: 28.0, footer: 28.0, cap_chips: vec![], footer_chips: vec![] }]).is_empty());
}
