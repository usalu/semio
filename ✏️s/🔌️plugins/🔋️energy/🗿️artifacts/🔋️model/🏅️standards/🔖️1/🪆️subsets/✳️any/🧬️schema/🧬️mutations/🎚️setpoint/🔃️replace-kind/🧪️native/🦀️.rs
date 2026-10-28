use super::*;
#[path="../../../🧳️native/🔢️scalar/🧩️transform/🧪️tests/🧰️leaf/🦀️.rs"]mod laws;
#[test]fn original_setpoint_kind_preserves_typed_tag_four_limits_exact_inverse_and_every_cancelled_source(){laws::run::<__ReplaceSetpointManagerKindNativeFacetPath>(include_str!("🧫️fixtures/🔣️.json"));}
#[test]fn typed_setpoint_limits_refuse_each_nonfinite_original_axis(){for i in 0..4{for value in [f64::NAN,f64::INFINITY,f64::NEG_INFINITY]{let mut limits=[-10.0,20.0,18.0,12.0];limits[i]=value;assert!(crate::model::SetpointManagerKind::from_parts(crate::model::SetpointManagerKindTag::OutdoorAirReset,limits).is_none());}}}
