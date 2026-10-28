//! 🧪️ Exact scalar and intrinsic word identities share the portable byte oracle.
use super::*;

#[test]
fn ieee_scalar_and_intrinsic_number_equality_preserve_all_authored_bits() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let expected = case["equal"].as_bool().unwrap();
        if case["width"] == 64 {
            let left = f64::from_bits(u64::from_str_radix(case["left"].as_str().unwrap(), 16).unwrap());
            let right = f64::from_bits(u64::from_str_radix(case["right"].as_str().unwrap(), 16).unwrap());
            assert_eq!(equal_f64(left, right), expected, "{name}: scalar owner");
            assert_eq!(Number::Float(left) == Number::Float(right), expected, "{name}: intrinsic number");
            assert_eq!(DslValue::Number(Number::Float(left)) == DslValue::Number(Number::Float(right)), expected, "{name}: opaque intrinsic payload");
        } else {
            let left = f32::from_bits(u32::from_str_radix(case["left"].as_str().unwrap(), 16).unwrap());
            let right = f32::from_bits(u32::from_str_radix(case["right"].as_str().unwrap(), 16).unwrap());
            assert_eq!(equal_f32(left, right), expected, "{name}: scalar owner");
        }
    }
}
