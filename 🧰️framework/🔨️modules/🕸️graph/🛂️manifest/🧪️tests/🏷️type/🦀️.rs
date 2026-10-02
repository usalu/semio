//! 🎯️ Original Graph property admission against the lower language-neutral corpus.

use super::{dsl_value_to_property_value, property_value_matches_type};
use semio_framework_value::{FromValue, ValueType};

#[test]
fn graph_properties_preserve_all_original_type_classifications() {
    let fixture = dsl_core::json::to_dsl_value(&dsl_core::json::parse(include_str!("../../../../🌱️value/🏷️type/🧫️fixtures/🔣️.json")).unwrap());
    let cases = fixture.get("graphCases").and_then(dsl_core::DslValue::as_array).unwrap();
    let types = fixture.get("types").and_then(dsl_core::DslValue::as_array).unwrap();
    let values = fixture.get("graphValues").and_then(dsl_core::DslValue::as_array).unwrap();
    for row in cases {
        let type_index = u64::from_value(row.get("type").unwrap().clone()).unwrap() as usize;
        let value_index = u64::from_value(row.get("value").unwrap().clone()).unwrap() as usize;
        let value_type = ValueType::from_value(types[type_index].get("type").unwrap().clone()).unwrap();
        let property = dsl_value_to_property_value(&values[value_index]);
        let expected = bool::from_value(row.get("accepted").unwrap().clone()).unwrap();
        assert_eq!(property_value_matches_type(&property, &value_type), expected, "{}", String::from_value(row.get("name").unwrap().clone()).unwrap());
    }
}
