//! 📦️ Portable paged carrier laws checked against independent Serde values.
use super::*;
use crate::{FromValue,ToValue};
const CORPUS: &str = include_str!("../🧫️fixtures/🔣️.json");

#[test]
fn paged_native_serde_and_sequence_ownership_match_neutral_shapes() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).unwrap();
    for row in corpus["vectors"].as_array().unwrap() {
        let input = row["input"].clone();
        match row["kind"].as_str().unwrap() {
            "text" => {
                let native = serde_json::from_value::<PagedUtf8<2048>>(input.clone());
                assert_eq!(native.is_ok(), row["accepted"].as_bool().unwrap());
                if let Ok(value) = native { assert_eq!(serde_json::to_value(value).unwrap(), input); }
            }
            "list" => {
                let native = serde_json::from_value::<PagedList<u64, 8>>(input.clone());
                assert_eq!(native.is_ok(), row["accepted"].as_bool().unwrap());
                if let Ok(value) = native {
                    let oracle: Vec<u64> = serde_json::from_value(input.clone()).unwrap();
                    assert_eq!(serde_json::to_value(&value).unwrap(), input);
                    assert_eq!(value.into_iter().collect::<Vec<_>>(), oracle);
                }
            }
            _ => {}
        }
    }
}

#[test]
fn portable_paged_carriers_match_independent_serde() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).unwrap();
    for row in corpus["vectors"].as_array().unwrap() {
        let input = row["input"].clone();
        let (actual, oracle) = match row["kind"].as_str().unwrap() {
            "text" => (
                PagedUtf8::<2048>::from_value(DslValue::from(&input)).map(|value| {
                    assert!(value.chunks().all(|chunk| chunk.len() <= PAGED_UTF8_CHUNK_BYTES));
                    value.to_value()
                }),
                serde_json::from_value::<String>(input.clone()).ok().filter(|value| value.len() <= 2048).map(serde_json::Value::String),
            ),
            "list" => (PagedList::<u64, 8>::from_value(DslValue::from(&input)).map(|value| value.to_value()), serde_json::from_value::<Vec<u64>>(input.clone()).ok().filter(|value| value.len() <= 8).map(|value| serde_json::to_value(value).unwrap())),
            "map" => (
                PagedMap::<u64, 4>::from_value(DslValue::from(&input)).map(|value| value.to_value()),
                serde_json::from_value::<std::collections::BTreeMap<String, u64>>(input.clone()).ok().filter(|value| value.len() <= 4).map(|value| serde_json::to_value(value).unwrap()),
            ),
            _ => unreachable!(),
        };
        assert_eq!(actual.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(actual.is_ok(), oracle.is_some(), "{}", row["id"]);
        if let (Ok(actual), Some(oracle)) = (actual, oracle) {
            assert_eq!(serde_json::Value::from(actual), oracle);
        }
    }
}

#[test]
fn paged_array_paths_and_structural_refusal_preserve_owner() {
    let mut value = PagedList::<u64, 8>::from_value(crate::dsl_value!([1u64, 2u64, 3u64])).unwrap();
    for index in ["", "01", "-1", "3"] {
        assert!(value.value_at_path(&[index]).is_err());
    }
    value.edit_value_at_path(&["1"], ValueEdit::Set(crate::dsl_value!(7u64))).unwrap();
    assert_eq!(value.value_at_path(&["1"]).unwrap(), crate::dsl_value!(7u64));
    let expected = value.to_value();
    assert!(value.edit_value_at_path(&["1"], ValueEdit::Remove).is_err());
    assert_eq!(value.to_value(), expected);
}

#[test]
fn paged_map_preserves_insertion_order_and_refuses_duplicate_owners() {
    let input = DslValue::Object(vec![("beta".into(), crate::dsl_value!(2u64)), ("alpha".into(), crate::dsl_value!(1u64))]);
    let mut value = PagedMap::<u64, 4>::from_value(input.clone()).unwrap();
    assert_eq!(value.keys().map(PagedUtf8::to_string_owner).collect::<Vec<_>>(), vec!["beta", "alpha"]);
    assert_eq!(value.value_key_at_path(&[], 0).unwrap(), "beta");
    assert!(value.edit_value_at_path(&["alpha"], ValueEdit::Remove).is_err());
    assert_eq!(value.to_value(), input);
    assert!(PagedMap::<u64, 4>::from_value(DslValue::Object(vec![("same".into(), crate::dsl_value!(1u64)), ("same".into(), crate::dsl_value!(2u64))])).is_err());
}

#[test]
fn paged_owners_construct_and_encode_under_one_admitted_control() {
    fn law<T: FromValue + ToValue>(input: &serde_json::Value, accepted: bool) {
        let source = DslValue::from(input);
        let mut open = |_| true;
        let mut admitted = crate::NativeDecodeControl::new(1 << 20, &mut open);
        let native = T::from_value_controlled(&source, &mut admitted);
        assert_eq!(native.is_ok(), accepted, "{input}");
        if let Ok(value) = native {
            let mut open = |_| true;
            let output = value.to_value_controlled(&mut crate::NativeEncodeControl::new(1 << 20, &mut open)).unwrap();
            assert_eq!(serde_json::Value::from(&output), *input);
        }
        let empty = input.as_str().is_some_and(str::is_empty) || input.as_array().is_some_and(Vec::is_empty) || input.as_object().is_some_and(serde_json::Map::is_empty);
        let mut open = |_| true;
        assert!(empty || T::from_value_controlled(&source, &mut crate::NativeDecodeControl::new(0, &mut open)).is_err());
        let mut closed = |_| false;
        assert!(T::from_value_controlled(&source, &mut crate::NativeDecodeControl::new(1 << 20, &mut closed)).is_err());
    }
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).unwrap();
    for row in corpus["vectors"].as_array().unwrap() {
        let (input, accepted) = (&row["input"], row["accepted"].as_bool().unwrap());
        match row["kind"].as_str().unwrap() {
            "text" => law::<PagedUtf8<2048>>(input, accepted),
            "list" => law::<PagedList<u64, 8>>(input, accepted),
            "map" => law::<PagedMap<u64, 4>>(input, accepted),
            _ => {}
        }
    }
}

#[test]
fn known_utf8_length_bounds_the_chunk_header_backing() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).unwrap();
    for case in corpus["textBacking"].as_array().unwrap() {
        let input = "a".repeat(case[0].as_u64().unwrap() as usize);
        let headers = case[1].as_u64().unwrap() as usize;
        let wire = serde_json::to_string(&input).unwrap();
        let value: PagedUtf8<{usize::MAX}> = serde_json::from_str(&wire).unwrap();
        let mut progress = |_| true;
        let mut control = crate::NativeDecodeControl::new(1 << 20, &mut progress);
        let controlled = PagedUtf8::try_from_str_controlled(&input, &mut control).unwrap();
        for text in [&value, &value.clone(), &controlled] {
            assert_eq!(text.chunks.capacity(), headers);
            assert!(text.eq_str(&input));
            assert_eq!(serde_json::to_string(text).unwrap(), wire);
        }
    }
}
