//#region 🧬️Fixture
use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct IntegerCase {
    #[serde(rename = "type")]
    integer_type: String,
    decimal: String,
    accepted32: bool,
    accepted64: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UnsignedCase {
    decimal: String,
    accepted: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Vectors {
    version: u32,
    integers: Vec<IntegerCase>,
    indices: Vec<UnsignedCase>,
    ordinals: Vec<UnsignedCase>,
}

fn vectors() -> Vectors {
    let vectors: Vectors = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    assert_eq!(vectors.version, 1);
    vectors
}

#[derive(Clone, Debug, PartialEq, DslRecord)]
#[dsl(keyword = "set-index")]
struct IndexRecord {
    index: u32,
}

#[derive(Clone, Debug, PartialEq, DslOps)]
enum IndexOperation {
    SetIndex(IndexRecord),
}

fn index_record(value: u64) -> RecordValue {
    let mut record = IndexRecord { index: 0 }.__dsl_to_record();
    *record.fields.values_mut().next().unwrap() = FieldValue::UInt(value);
    record
}

fn binary_record(ordinal: u64, record: &RecordValue) -> Vec<u8> {
    let mut bytes = vec![variants_binary::OP_BINARY_FORMAT];
    crate::os_pack::write_varint_u64(&mut bytes, ordinal);
    bytes.extend(crate::os_pack::encode_record_body(&IndexRecord::__dsl_spec(), record, &crate::os_pack::EncodeOptions::default()).unwrap());
    bytes
}
//#endregion 🧬️Fixture

//#region 🧪️Fields
#[semio_framework_async_macros::async_test]
async fn fields_match_neutral_boundaries_and_serde() {
    let mut checked = 0;
    for row in vectors().integers {
        let expected = match usize::BITS { 32 => row.accepted32, 64 => row.accepted64, bits => panic!("unsupported pointer width {bits}") };
        macro_rules! check_type {
            ($ty:ty, $wide:ty, $variant:ident) => {{
                let actual = row.decimal.parse::<$wide>().map_err(|error| error.to_string()).and_then(|value| <$ty>::from_value(&FieldValue::$variant(value)));
                let reference = serde_json::from_str::<$ty>(&row.decimal);
                assert_eq!(actual.is_ok(), expected, "{} {}", row.integer_type, row.decimal);
                assert_eq!(reference.is_ok(), expected, "serde {} {}", row.integer_type, row.decimal);
                assert_eq!(actual.as_ref().ok(), reference.as_ref().ok(), "{} {}", row.integer_type, row.decimal);
                if let Ok(value) = actual {
                    assert_eq!(<$ty>::from_value(&value.to_value()), Ok(value));
                    assert_eq!(value.to_string(), row.decimal);
                }
            }};
        }
        match row.integer_type.as_str() {
            "i8" => check_type!(i8, i64, Int),
            "i16" => check_type!(i16, i64, Int),
            "i32" => check_type!(i32, i64, Int),
            "i64" => check_type!(i64, i64, Int),
            "isize" => check_type!(isize, i64, Int),
            "u8" => check_type!(u8, u64, UInt),
            "u16" => check_type!(u16, u64, UInt),
            "u32" => check_type!(u32, u64, UInt),
            "u64" => check_type!(u64, u64, UInt),
            "usize" => check_type!(usize, u64, UInt),
            other => panic!("unrecognized integer type {other}"),
        }
        checked += 1;
    }
    assert_eq!(checked, 51);
    eprintln!("[DEBUG] checked integer vectors={checked} pointerBits={}", usize::BITS);
}

#[semio_framework_async_macros::async_test]
async fn mismatched_variants_rejected_all_types() {
    macro_rules! check_type {
        ($ty:ty, $other:expr) => {
            for value in [$other, FieldValue::Bool(false), FieldValue::Float(1.0), FieldValue::Text("1".into()), FieldValue::Absent] {
                assert!(<$ty>::from_value(&value).is_err(), "{} {value:?}", stringify!($ty));
            }
        };
    }
    check_type!(i8, FieldValue::UInt(1));
    check_type!(i16, FieldValue::UInt(1));
    check_type!(i32, FieldValue::UInt(1));
    check_type!(i64, FieldValue::UInt(1));
    check_type!(isize, FieldValue::UInt(1));
    check_type!(u8, FieldValue::Int(1));
    check_type!(u16, FieldValue::Int(1));
    check_type!(u32, FieldValue::Int(1));
    check_type!(u64, FieldValue::Int(1));
    check_type!(usize, FieldValue::Int(1));
}

#[semio_framework_async_macros::async_test]
async fn nested_collections_propagate_integer_overflow() {
    assert_eq!(Vec::<u8>::from_value(&FieldValue::List(vec![FieldValue::UInt(255)])), Ok(vec![255]));
    assert!(Vec::<u8>::from_value(&FieldValue::List(vec![FieldValue::UInt(256)])).is_err());
    assert_eq!(<[i8; 1]>::from_value(&FieldValue::Tuple(vec![FieldValue::Int(-128)])), Ok([-128]));
    assert!(<[i8; 1]>::from_value(&FieldValue::Tuple(vec![FieldValue::Int(-129)])).is_err());
    let value = FieldValue::Map(vec![("index".into(), FieldValue::UInt(4294967296))]);
    assert!(std::collections::BTreeMap::<String, u32>::from_value(&value).is_err());
}
//#endregion 🧪️Fields

//#region 🧪️Codecs
#[semio_framework_async_macros::async_test]
async fn derived_records_reject_text_integer_overflow() {
    for row in vectors().indices {
        let wide = row.decimal.parse::<u64>().unwrap();
        let text = print(&index_record(wide), &IndexRecord::__dsl_spec(), JoinMode::Inline);
        assert!(text.contains(&row.decimal), "printer lost integer precision: {text}");
        let record = parse(&text, &IndexRecord::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline }).unwrap();
        let decoded = IndexRecord::__dsl_from_record(&record);
        assert_eq!(decoded.is_ok(), row.accepted, "{text}");
        assert_eq!(IndexOperation::from_named_record("set-index", &record).is_ok(), row.accepted, "{text}");
        if let Ok(value) = decoded { assert_eq!(u64::from(value.index), wide); }
    }
}

#[semio_framework_async_macros::async_test]
async fn derived_variants_reject_binary_integer_overflow() {
    for row in vectors().indices {
        let wide = row.decimal.parse::<u64>().unwrap();
        let bytes = binary_record(0, &index_record(wide));
        let decoded = variants_binary::decode_op::<IndexOperation>(&bytes);
        assert_eq!(decoded.is_ok(), row.accepted, "binary index {}", row.decimal);
        if let Ok(value) = decoded {
            let IndexOperation::SetIndex(record) = &value;
            assert_eq!(u64::from(record.index), wide);
            assert_eq!(variants_binary::encode_op(&value).unwrap(), bytes);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn binary_ordinals_reject_overflow_and_truncation() {
    for row in vectors().ordinals {
        let bytes = binary_record(row.decimal.parse().unwrap(), &index_record(7));
        let decoded = variants_binary::decode_op::<IndexOperation>(&bytes);
        assert_eq!(decoded.is_ok(), row.accepted, "ordinal {}", row.decimal);
        match decoded {
            Ok(value) => assert_eq!(value, IndexOperation::SetIndex(IndexRecord { index: 7 })),
            Err(error) => assert!(matches!(error, crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, .. })),
        }
    }
    for bytes in [vec![], vec![1], vec![1, 128], vec![1, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128], vec![2, 0]] {
        assert!(variants_binary::decode_op::<IndexOperation>(&bytes).is_err());
    }
}
//#endregion 🧪️Codecs
