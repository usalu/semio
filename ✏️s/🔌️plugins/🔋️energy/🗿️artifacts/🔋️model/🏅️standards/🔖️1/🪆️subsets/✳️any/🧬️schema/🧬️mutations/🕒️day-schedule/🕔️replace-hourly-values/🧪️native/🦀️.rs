use super::*;
#[path="../../../🧳️native/🔢️scalar/🧪️tests/🧰️law/🦀️.rs"]mod laws;
#[test]fn original_fixed_daily_profile_preserves_twenty_four_values_sparse_output_inverse_and_cancelled_sources(){let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for case in law["cases"].as_array().unwrap(){laws::run::<__ReplaceDailyScheduleHourlyValuesNativeRowPath>(case,law["cancelAt"].as_array().unwrap());}}
#[test]fn original_fixed_daily_profile_refuses_nonfinite_typed_values_without_losing_original_custody(){let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();laws::run_with_value::<__ReplaceDailyScheduleHourlyValuesNativeRowPath>(&law["nonfiniteCase"],law["cancelAt"].as_array().unwrap(),Some([f64::NAN;24]));}
