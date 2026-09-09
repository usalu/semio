
#[test]
fn borrowed_slices_match_neutral_values_and_serde() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔪️slices.json")).expect("slice fixture");
    for case in fixture["cases"].as_array().unwrap() {
        let values: Vec<i64> = serde_json::from_value(case.clone()).expect("integer sequence");
        let actual = crate::to_dsl_value(values.as_slice()).expect("slice encoding");
        assert_eq!(serde_json::to_string(&actual).unwrap(), serde_json::to_string(values.as_slice()).unwrap());
        assert_eq!(actual, case.clone());
    }
    eprintln!("[DEBUG] Borrowed slice encoding agrees with the neutral corpus and serde");
}
use super::*;

#[test]
fn integer_from_value_matches_serde_without_coercion() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral exact integer corpus");
    macro_rules! check {
            ($($ty:ty),+ $(,)?) => { $(
                for row in fixture["raw"].as_array().unwrap() {
                    let raw = row.as_str().unwrap();
                    let value: serde_json::Value = serde_json::from_str(raw).expect("valid JSON scalar");
                    let expected = raw.parse::<i128>().is_ok_and(|number| number >= <$ty>::MIN as i128 && number <= <$ty>::MAX as i128);
                    let reference = serde_json::from_str::<$ty>(raw);
                    assert_eq!(reference.is_ok(), expected, "oracle {} {raw}", stringify!($ty));
                    let ours = <$ty>::from_value(DslValue::from(&value));
                    assert_eq!(ours.is_ok(), expected, "production {} {raw}", stringify!($ty));
                    if let (Ok(ours), Ok(reference)) = (ours, reference) {
                        assert_eq!(ours, reference, "exact {} {raw}", stringify!($ty));
                        assert_eq!(<$ty>::from_value(ours.to_value()), Ok(reference));
                        assert_eq!(serde_json::to_string(&ours).unwrap(), serde_json::to_string(&reference).unwrap());
                    }
                }
                for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -0.0, 1.0] {
                    assert!(<$ty>::from_value(DslValue::float(number)).is_err(), "noninteger variant {} {number}", stringify!($ty));
                }
                assert_eq!(<$ty>::from_value(DslValue::int(1)), Ok(1));
                assert_eq!(<$ty>::from_value(DslValue::uint(1)), Ok(1));
            )+ };
        }
    check!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
}

#[test]
fn scalars_round_trip() {
    assert_eq!(42_i64.to_value(), DslValue::int(42));
    assert_eq!(i64::from_value(DslValue::int(42)), Ok(42_i64));
    assert_eq!(true.to_value(), DslValue::Bool(true));
    assert_eq!("hi".to_string().to_value(), DslValue::String("hi".to_string()));
}

#[test]
fn option_collapses_nested_none_like_naive_serde() {
    let outer_none: Option<Option<String>> = None;
    let inner_none: Option<Option<String>> = Some(None);
    assert_eq!(outer_none.to_value(), DslValue::Null);
    assert_eq!(inner_none.to_value(), DslValue::Null);
}

#[test]
fn vec_round_trips_and_reports_index_on_error() {
    let values = vec![1_i64, 2, 3];
    let encoded = values.to_value();
    assert_eq!(Vec::<i64>::from_value(encoded), Ok(values));
    let bad = DslValue::Array(vec![DslValue::int(1), DslValue::Bool(true)]);
    assert_eq!(Vec::<i64>::from_value(bad), Err(ValueError::new("1.expected a number, found Bool(true)")));
}

#[test]
fn btreemap_round_trips_in_key_order() {
    let mut map = std::collections::BTreeMap::new();
    map.insert("b".to_string(), 2_i64);
    map.insert("a".to_string(), 1_i64);
    let encoded = map.to_value();
    assert_eq!(encoded, DslValue::object([("a".to_string(), DslValue::int(1)), ("b".to_string(), DslValue::int(2))]));
    assert_eq!(std::collections::BTreeMap::<String, i64>::from_value(encoded), Ok(map));
}

#[test]
fn tuple_round_trips_as_two_element_array_like_serde_json() {
    let pair = ("a".to_string(), vec!["b".to_string(), "c".to_string()]);
    let encoded = pair.to_value();
    assert_eq!(encoded, DslValue::Array(vec![DslValue::String("a".to_string()), DslValue::Array(vec![DslValue::String("b".to_string()), DslValue::String("c".to_string())])]));
    assert_eq!(<(String, Vec<String>)>::from_value(encoded), Ok(pair));
    let bad = DslValue::Array(vec![DslValue::int(1)]);
    assert_eq!(<(i64, i64)>::from_value(bad), Err(ValueError::new("expected a 2-element array, found Array([Number(Int(1))])")));
}

#[test]
fn fixed_size_array_round_trips_and_rejects_wrong_length() {
    let values: [f64; 3] = [1.0, 2.0, 3.0];
    let encoded = values.to_value();
    assert_eq!(encoded, DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::float(3.0)]));
    assert_eq!(<[f64; 3]>::from_value(encoded), Ok(values));

    let too_short = DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0)]);
    assert_eq!(<[f64; 3]>::from_value(too_short), Err(ValueError::new("expected an array of length 3, found 2")));

    let too_long = DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::float(3.0), DslValue::float(4.0)]);
    assert_eq!(<[f64; 3]>::from_value(too_long), Err(ValueError::new("expected an array of length 3, found 4")));
}

#[test]
fn phantom_data_encodes_as_null_and_decodes_from_anything() {
    struct Marker;
    let phantom: std::marker::PhantomData<Marker> = std::marker::PhantomData;
    assert_eq!(phantom.to_value(), DslValue::Null);
    assert_eq!(std::marker::PhantomData::<Marker>::from_value(DslValue::Bool(true)), Ok(std::marker::PhantomData));
}

/// 🎯️ The regression this whole module exists for: a `u64` field round-trips as `Number::UInt`
/// (bare `3600` on the wire, never `3600.0`), and a genuine `f64` field still round-trips as
/// `Number::Float` (keeping its `.0`) — the two must stay distinguishable.
#[test]
fn u64_round_trips_as_uint_and_f64_round_trips_as_float() {
    let ttl_secs: u64 = 3600;
    let encoded = ttl_secs.to_value();
    assert_eq!(encoded, DslValue::uint(3600));
    assert!(matches!(encoded, DslValue::Number(Number::UInt(3600))));
    assert_eq!(u64::from_value(encoded), Ok(3600));

    let ratio: f64 = 3600.0;
    let encoded = ratio.to_value();
    assert_eq!(encoded, DslValue::float(3600.0));
    assert!(matches!(encoded, DslValue::Number(Number::Float(v)) if v == 3600.0));
    assert_eq!(f64::from_value(encoded), Ok(3600.0));
}

#[test]
fn i64_min_and_max_round_trip_exactly() {
    for value in [i64::MIN, i64::MAX, 0_i64, -1_i64] {
        let encoded = value.to_value();
        assert!(matches!(encoded, DslValue::Number(Number::Int(v)) if v == value));
        assert_eq!(i64::from_value(encoded), Ok(value));
    }
}

#[test]
fn u64_max_round_trips_exactly_beyond_f64_2_pow_53() {
    assert_eq!(u64::MAX as f64, (u64::MAX - 1) as f64, "test assumption: u64::MAX collides with u64::MAX - 1 once widened to f64");
    let encoded = u64::MAX.to_value();
    assert!(matches!(encoded, DslValue::Number(Number::UInt(v)) if v == u64::MAX));
    assert_eq!(u64::from_value(encoded), Ok(u64::MAX));

    let big: u64 = (1u64 << 53) + 1;
    assert!((big as f64) as u64 != big, "test assumption: 2^53+1 is not exactly representable in f64");
    let encoded = big.to_value();
    assert!(matches!(encoded, DslValue::Number(Number::UInt(v)) if v == big));
    assert_eq!(u64::from_value(encoded), Ok(big));
}

#[test]
fn negative_zero_float_round_trips_and_stays_a_float() {
    let encoded = (-0.0_f64).to_value();
    assert!(matches!(encoded, DslValue::Number(Number::Float(v)) if v.is_sign_negative() && v == 0.0));
    assert_eq!(f64::from_value(encoded), Ok(-0.0));
}

/// 🔬️ An `f32` widens by DECIMAL identity, not by bits: `0.42f32` is the number the writer
/// wrote, so `to_value` must carry the `f64` whose shortest lexeme is `0.42` — the byte
/// `serde_json::serialize_f32` writes — and not `0.42f32 as f64`, whose own shortest lexeme is
/// `0.41999998688697815`. `serde_json` is the third-party oracle for both widths here.
#[test]
fn f32_widens_through_its_own_shortest_lexeme_not_its_bits() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔢️f32-decimals.json")).expect("float decimal fixture");
    for case in fixture["cases"].as_array().unwrap() {
        let value = f32::from_bits(u32::from_str_radix(case["bits"].as_str().unwrap(), 16).unwrap());
        let decimal = case["decimal"].as_str().unwrap();
        assert_eq!(serde_json::to_string(&value).unwrap(), decimal);
        assert_eq!(f64::from_value(value.to_value()).unwrap().to_bits(), decimal.parse::<f64>().unwrap().to_bits());
    }
    for value in [0.42_f32, 1.1, 3.14159, 1e-7, 16777217.0, -0.02, 0.85, 0.0625, f32::MIN_POSITIVE, f32::MAX, f32::from_bits(1)] {
        let DslValue::Number(Number::Float(widened)) = value.to_value() else {
            panic!("f32 must encode as Number::Float, found {:?}", value.to_value());
        };
        assert_eq!(serde_json::to_string(&widened).expect("finite f64"), serde_json::to_string(&value).expect("finite f32"), "widened lexeme must equal serde_json's own f32 lexeme for {value:e}");
        assert_eq!(f32::from_value(value.to_value()), Ok(value), "narrowing back must be exact for {value:e}");
    }
    assert_eq!((0.42_f32).to_value(), DslValue::float(0.42));
}

/// 🔬️ Deterministic sweep over arbitrary `f32` bit patterns — every finite `f32` must survive
/// `to_value`/`from_value` bit-exactly (this is what keeps the binary record carriers, which
/// store `Number::Float`'s `f64` verbatim, lossless for `f32` fields) AND must widen to a
/// `f64` equal to serde's parsed decimal value. Serde uses different fixed/scientific
/// notation thresholds for f32 and f64, so spelling equality is not a numeric invariant.
#[test]
fn every_finite_f32_round_trips_bit_exactly_and_preserves_serde_decimal_value() {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut checked = 0usize;
    for _ in 0..200_000u32 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let value = f32::from_bits((state >> 32) as u32);
        if !value.is_finite() {
            continue;
        }
        let encoded = value.to_value();
        let DslValue::Number(Number::Float(widened)) = encoded else { panic!("f32 must encode as Number::Float") };
        let reference: f64 = serde_json::to_string(&value).expect("finite f32").parse().expect("decimal f64");
        assert_eq!(widened.to_bits(), reference.to_bits(), "decimal value mismatch for f32 bits {:#010x}", value.to_bits());
        assert_eq!(f32::from_value(encoded).expect("float").to_bits(), value.to_bits(), "bit mismatch for f32 bits {:#010x}", value.to_bits());
        checked += 1;
    }
    assert!(checked > 150_000, "sweep must reach a real sample count, got {checked}");
}

#[test]
fn whole_float_and_same_valued_integer_are_distinct_dsl_values() {
    let as_float = DslValue::float(3600.0);
    let as_uint = DslValue::uint(3600);
    assert_ne!(as_float, as_uint);
    assert!(matches!(as_float, DslValue::Number(Number::Float(_))));
    assert!(matches!(as_uint, DslValue::Number(Number::UInt(_))));
}
