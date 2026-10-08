use super::*;
use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
use protocol::Mutation;

#[test]
fn language_neutral_forward_and_concrete_inverse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📝️set-page-text/🔄️round-trips-the-concrete-inverse/🔣️.json")).unwrap();
    fn assert_json_shape(actual: &serde_json::Value, expected: &serde_json::Value) {
        match (actual, expected) {
            (serde_json::Value::Object(actual), serde_json::Value::Object(expected)) => {
                assert_eq!(actual.len(), expected.len());
                for (key, expected) in expected {
                    assert_json_shape(actual.get(key).unwrap_or_else(|| panic!("missing JSON field {key:?}")), expected);
                }
            }
            (serde_json::Value::Array(actual), serde_json::Value::Array(expected)) => {
                assert_eq!(actual.len(), expected.len());
                for (actual, expected) in actual.iter().zip(expected) {
                    assert_json_shape(actual, expected);
                }
            }
            (serde_json::Value::Number(_), serde_json::Value::Number(_)) => {}
            _ => assert_eq!(actual, expected),
        }
    }
    let base: PdfSnapshot = semio_framework_value::FromValue::from_value((fixture["base"].clone()).into()).unwrap();
    let mutation: PdfA1Mutation = semio_framework_value::FromValue::from_value((fixture["mutation"].clone()).into()).unwrap();
    assert_json_shape(&serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation)).unwrap(), &fixture["mutation"]);
    let mut state = base.clone();
    let outcome = crate::standards::v1_4::subsets::base::schema::mutations::apply_outcome(mutation.diff(&state), &mut state);
    assert!(outcome.messages().is_empty());
    let expected: PdfSnapshot = semio_framework_value::FromValue::from_value((fixture["expected"].clone()).into()).unwrap();
    assert_eq!(state, expected);
    assert_json_shape(&serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&state)).unwrap(), &fixture["expected"]);
    let inverse = mutation.inverse(&base).expect("valid retained mutation inverse fixture");
    let expected_inverse: Vec<PdfA1Mutation> = semio_framework_value::FromValue::from_value((fixture["inverse"].clone()).into()).unwrap();
    assert_eq!(inverse, expected_inverse);
    assert_json_shape(&serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&inverse)).unwrap(), &fixture["inverse"]);
    for step in std::iter::once(mutation.clone()).chain(inverse.iter().cloned()) {
        assert_eq!(<PdfA1Mutation as semio_framework_value::FromValue>::from_value((serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&step)).unwrap()).into()).unwrap(), step);
    }
    for step in inverse {
        assert!(crate::standards::v1_4::subsets::base::schema::mutations::apply_outcome(step.diff(&state), &mut state).messages().is_empty());
    }
    assert_eq!(state, base);
}

#[test]
fn missing_page_refuses_without_inverse_or_state_change() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📝️set-page-text/🔄️round-trips-the-concrete-inverse/🔣️.json")).unwrap();
    let mutation: PdfA1Mutation = semio_framework_value::FromValue::from_value((fixture["mutation"].clone()).into()).unwrap();
    let base = PdfSnapshot { pages: Vec::new(), ..Default::default() };
    let mut state = base.clone();
    assert!(crate::standards::v1_4::subsets::base::schema::mutations::apply_outcome(!mutation.diff(&state), &mut state).messages().is_empty());
    assert_eq!(state, base);
    assert!(mutation.inverse(&base).expect("valid retained mutation inverse fixture").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = PdfSnapshot { pages: vec![PageDoc { width: 612.0, height: 792.0, text: "first".to_string() }, PageDoc { width: 100.0, height: 200.0, text: "second".to_string() }], ..Default::default() };
    assert_mutation_inverse_sum_law(&PdfA1Mutation::SetPageText(SetPageText { text: "replaced".to_string() }), &base).await;
}
