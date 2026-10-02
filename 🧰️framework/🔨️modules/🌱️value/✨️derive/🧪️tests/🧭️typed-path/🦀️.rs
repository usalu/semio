//! 🧭️ Language-neutral typed-path fixtures exercised through generated Rust implementations.

use semio_framework_value::{DslValue, FromValue, ToValue, ValueEdit, ValueError, ValueShape};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static PAYLOAD_CONVERSIONS: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, PartialEq)]
struct CountedBytes(Vec<u8>);

impl ToValue for CountedBytes {
    fn to_value(&self) -> DslValue {
        PAYLOAD_CONVERSIONS.fetch_add(1, Ordering::SeqCst);
        self.0.to_value()
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        self.0.value_at_path(path)
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        self.0.value_shape_at_path(path)
    }
}

impl FromValue for CountedBytes {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Vec::from_value(value).map(Self)
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        self.0.edit_value_at_path(path, edit)
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
struct Document {
    sample_rate: u32,
    metadata: BTreeMap<String, String>,
    bytes: CountedBytes,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", tag = "kind", content = "data")]
enum Adjacent {
    Samples(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
enum Internal {
    Track { sample_rate: u32 },
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
enum ReservedFieldNames {
    Entry { path: std::path::PathBuf, index: bool, edit: String },
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
enum EmptyExternal {}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind")]
enum EmptyTagged {}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
enum External {
    Samples(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(transparent)]
struct Transparent(Vec<u8>);

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
struct Flattened {
    name: String,
    #[value(flatten)]
    extra: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
struct Payload(Vec<u8>);

impl FromValue for Payload {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        payload_from_value(value)
    }
}

fn payload_to_value(payload: &Payload) -> DslValue {
    let hex = payload.0.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    DslValue::object([("hex".to_owned(), DslValue::String(hex))])
}

fn payload_from_value(value: DslValue) -> Result<Payload, ValueError> {
    let mut entries = DslValue::into_object(value)?;
    if entries.len() != 1 || entries[0].0 != "hex" {
        return Err(ValueError::new("expected one hex field"));
    }
    let DslValue::String(hex) = entries.remove(0).1 else {
        return Err(ValueError::new("expected a hex string"));
    };
    if hex.len() % 2 != 0 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ValueError::new("invalid hexadecimal payload"));
    }
    let bytes = (0..hex.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&hex[index..index + 2], 16).map_err(|error| ValueError::new(error.to_string())))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Payload(bytes))
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
struct Custom {
    #[value(serialize_with = "payload_to_value", deserialize_with = "payload_from_value")]
    payload: Payload,
    name: String,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
struct OneWayCustom {
    #[value(serialize_with = "payload_to_value")]
    payload: Payload,
}

#[derive(Deserialize)]
struct Corpus {
    accepted: Vec<Case>,
    rejected: Vec<Case>,
    #[serde(rename = "largeDocument")]
    large_document: LargeDocument,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    model: String,
    before: serde_json::Value,
    path: Vec<String>,
    edit: Edit,
    expected: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct Edit {
    operation: String,
    value: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct LargeDocument {
    #[serde(rename = "byteCount")]
    byte_count: usize,
    fill: u8,
    path: Vec<String>,
    edit: Edit,
    #[serde(rename = "unchangedPayloadConversions")]
    unchanged_payload_conversions: usize,
}

fn corpus() -> Corpus {
    serde_json::from_str(include_str!("../../../🔁️codec/🧭️path/🧫️fixtures/🔣️.json")).expect("neutral typed-path fixture parses")
}

fn value_edit(edit: &Edit) -> ValueEdit {
    match edit.operation.as_str() {
        "set" => ValueEdit::Set(DslValue::from(edit.value.as_ref().expect("set value"))),
        "insert" => ValueEdit::Insert(DslValue::from(edit.value.as_ref().expect("insert value"))),
        "remove" => ValueEdit::Remove,
        operation => panic!("unknown fixture operation {operation}"),
    }
}

fn apply<T>(before: &serde_json::Value, path: &[&str], edit: ValueEdit) -> Result<DslValue, ValueError>
where
    T: FromValue + ToValue,
{
    let mut value = T::from_value(DslValue::from(before))?;
    value.edit_value_at_path(path, edit)?;
    Ok(value.to_value())
}

fn apply_case(case: &Case) -> Result<DslValue, ValueError> {
    let path = case.path.iter().map(String::as_str).collect::<Vec<_>>();
    let edit = value_edit(&case.edit);
    match case.model.as_str() {
        "document" => apply::<Document>(&case.before, &path, edit),
        "adjacent" => apply::<Adjacent>(&case.before, &path, edit),
        "internal" => apply::<Internal>(&case.before, &path, edit),
        "external" => apply::<External>(&case.before, &path, edit),
        "transparent" => apply::<Transparent>(&case.before, &path, edit),
        "flattened" => apply::<Flattened>(&case.before, &path, edit),
        "custom" => apply::<Custom>(&case.before, &path, edit),
        "one-way-custom" => apply::<OneWayCustom>(&case.before, &path, edit),
        model => panic!("unknown fixture model {model}"),
    }
}

#[test]
fn neutral_accepted_edits_match_the_serde_json_oracle() {
    for case in corpus().accepted {
        let actual = apply_case(&case).unwrap_or_else(|error| panic!("{}: {error}", case.id));
        let actual = serde_json::Value::from(&actual);
        assert_eq!(actual, case.expected.expect("accepted expected value"), "{}", case.id);
    }
}

#[test]
fn neutral_rejected_edits_fail_without_returning_a_modified_value() {
    for case in corpus().rejected {
        assert!(apply_case(&case).is_err(), "{}", case.id);
    }
}

#[test]
fn empty_enums_generate_exhaustive_value_and_path_implementations() {
    assert!(<EmptyExternal as FromValue>::from_value(DslValue::Null).is_err());
    assert!(<EmptyTagged as FromValue>::from_value(DslValue::object([(
        "kind".to_owned(),
        DslValue::String("impossible".to_owned()),
    )]))
    .is_err());
}

#[test]
fn refused_custom_decode_keeps_the_typed_document_atomic() {
    let mut value = Custom { payload: Payload(vec![10, 11]), name: "Keep".to_owned() };
    let before = value.clone();
    let result = value.edit_value_at_path(&["payload", "hex"], ValueEdit::Set(DslValue::String("not-hex".to_owned())));
    assert!(result.is_err());
    assert_eq!(value, before);
}

#[test]
fn direct_field_edit_and_shape_do_not_convert_a_two_megabyte_sibling() {
    let fixture = corpus().large_document;
    let mut document = Document {
        sample_rate: 44_100,
        metadata: BTreeMap::new(),
        bytes: CountedBytes(vec![fixture.fill; fixture.byte_count]),
    };
    let path = fixture.path.iter().map(String::as_str).collect::<Vec<_>>();
    PAYLOAD_CONVERSIONS.store(0, Ordering::SeqCst);
    document.edit_value_at_path(&path, value_edit(&fixture.edit)).expect("direct edit");
    assert_eq!(document.sample_rate, 48_000);
    assert_eq!(document.value_shape_at_path(&[]), Ok(ValueShape::Object { len: 3 }));
    assert_eq!(document.value_shape_at_path(&["bytes"]), Ok(ValueShape::Array { len: fixture.byte_count }));
    assert_eq!(document.value_key_at_path(&[], 2), Ok("bytes".to_owned()));
    assert_eq!(PAYLOAD_CONVERSIONS.load(Ordering::SeqCst), fixture.unchanged_payload_conversions);
}

#[test]
fn enum_shape_and_key_queries_do_not_encode_the_payload() {
    let adjacent = Adjacent::Samples(vec![4, 5]);
    assert_eq!(adjacent.value_shape_at_path(&[]), Ok(ValueShape::Object { len: 2 }));
    assert_eq!(adjacent.value_key_at_path(&[], 0), Ok("kind".to_owned()));
    assert_eq!(adjacent.value_key_at_path(&[], 1), Ok("data".to_owned()));
    assert_eq!(adjacent.value_shape_at_path(&["data"]), Ok(ValueShape::Array { len: 2 }));

    let internal = Internal::Track { sample_rate: 44_100 };
    assert_eq!(internal.value_shape_at_path(&[]), Ok(ValueShape::Object { len: 2 }));
    assert_eq!(internal.value_key_at_path(&[], 0), Ok("kind".to_owned()));
    assert_eq!(internal.value_key_at_path(&[], 1), Ok("sampleRate".to_owned()));
}

#[test]
fn enum_fields_cannot_capture_typed_path_arguments() {
    let mut value = ReservedFieldNames::Entry { path: "before".into(), index: false, edit: "old".to_owned() };
    assert_eq!(value.value_shape_at_path(&[]), Ok(ValueShape::Object { len: 4 }));
    assert_eq!(value.value_key_at_path(&[], 1), Ok("path".to_owned()));
    assert_eq!(value.value_at_path(&["index"]), Ok(DslValue::Bool(false)));
    value.edit_value_at_path(&["edit"], ValueEdit::Set(DslValue::String("new".to_owned()))).expect("reserved-name field edit");
    assert_eq!(value.value_at_path(&["edit"]), Ok(DslValue::String("new".to_owned())));
}

mod decoding_trait_collision {
    use super::*;
    trait OtherDecode {fn from_value(value:&str)->Self;}
    #[derive(Clone,Debug,PartialEq,ToValue,FromValue)]
    struct Record {value:u32}
    impl OtherDecode for Record {fn from_value(_value:&str)->Self {panic!("wrong decode trait")}}
    #[derive(Clone,Debug,PartialEq,ToValue,FromValue)]
    enum Choice {Value(u32)}
    impl OtherDecode for Choice {fn from_value(_value:&str)->Self {panic!("wrong decode trait")}}
    #[test]
    fn root_edit_uses_the_value_decoder_when_another_trait_has_the_same_method() {
        let mut record=Record {value:1};let replacement=Record {value:2};
        FromValue::edit_value_at_path(&mut record,&[],ValueEdit::Set(replacement.to_value())).unwrap();assert_eq!(record,replacement);
        let mut choice=Choice::Value(1);let replacement=Choice::Value(2);
        FromValue::edit_value_at_path(&mut choice,&[],ValueEdit::Set(replacement.to_value())).unwrap();assert_eq!(choice,replacement);
    }
}

#[test]
fn controlled_empty_enum_corpus_refuses_every_uninhabited_wire(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🔁️codec/🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();
 for row in fixture["emptyEnums"].as_array().unwrap(){assert_eq!(row["inhabited"],false);for input in row["inputs"].as_array().unwrap(){let mut callback=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(10000,&mut callback);let value=DslValue::from(input);match row["id"].as_str().unwrap(){"external-empty"=>assert!(EmptyExternal::from_value_controlled(&value,&mut control).is_err()),"tagged-empty"=>assert!(EmptyTagged::from_value_controlled(&value,&mut control).is_err()),_=>panic!("unknown empty enum identity")};}}
}
