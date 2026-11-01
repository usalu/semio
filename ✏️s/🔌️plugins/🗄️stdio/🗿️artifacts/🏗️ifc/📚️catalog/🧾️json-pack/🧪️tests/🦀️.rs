//! 🧪️ Laws of the IFCJ reader: the committed fixture decodes to its neutral JSON under third-party `serde_json`, member order survives, and hostile payloads are refused with closed causes.
use super::*;
use serde_json::Value;

const FIXTURE: &[u8] = include_bytes!("🧫️fixtures/minimal.ifcj");
const NEUTRAL: &str = include_str!("🧫️fixtures/minimal.json");

fn value(json: &Json<'_>) -> Value {
    match json {
        Json::Null => Value::Null,
        Json::Bool(flag) => Value::Bool(*flag),
        Json::Int(number) => Value::from(*number),
        Json::Str(text) => Value::String((*text).to_owned()),
        Json::Arr(items) => Value::Array(items.iter().map(value).collect()),
        Json::Obj(members) => Value::Object(members.iter().map(|(key, item)| ((*key).to_owned(), value(item))).collect()),
    }
}

fn stored(payload: &[u8]) -> Vec<u8> {
    let length = u16::try_from(payload.len()).unwrap();
    let mut block = vec![1, length as u8, (length >> 8) as u8, !length as u8, (!length >> 8) as u8];
    block.extend_from_slice(payload);
    block
}

fn payload() -> Vec<u8> {
    inflate(FIXTURE, MAXIMUM_PAYLOAD_BYTES).unwrap()
}

fn decode(pack: &[u8]) -> Result<Value, JsonPackError> {
    with_json_pack(pack, |root| Ok(value(root)))
}

#[test]
fn fixture_decodes_to_its_neutral_json_with_member_order() {
    let expected: Value = serde_json::from_str(NEUTRAL).unwrap();
    assert_eq!(decode(FIXTURE).unwrap(), expected);
    with_json_pack(FIXTURE, |root| {
        let Json::Obj(members) = root else { return Err(JsonPackError::Shape("object")) };
        let keys: Vec<_> = members.iter().map(|(key, _)| *key).collect();
        assert_eq!(keys, ["null", "yes", "no", "zero", "small", "wide", "large", "safe", "negative", "deep", "text", "empty", "list", "again", "text2"]);
        Ok(())
    })
    .unwrap();
}

#[test]
fn accessors_refuse_unlisted_duplicate_and_mistyped_members() {
    with_json_pack(FIXTURE, |root| {
        assert!(root.object(&["null"]).is_err());
        let all: Vec<_> = match root {
            Json::Obj(members) => members.iter().map(|(key, _)| *key).collect(),
            _ => return Err(JsonPackError::Shape("object")),
        };
        let fields = root.object(&all)?;
        assert_eq!(fields.require("wide")?.index()?, 128);
        assert_eq!(fields.require("negative")?.integer()?, -1);
        assert!(fields.require("text")?.integer().is_err());
        assert!(fields.get("absent").is_none());
        assert!(fields.require("absent").is_err());
        Ok(())
    })
    .unwrap();
    let duplicate = Json::Obj(vec![("a", Json::Null), ("a", Json::Null)]);
    assert_eq!(duplicate.object(&["a"]).unwrap_err(), JsonPackError::Shape("member"));
}

#[test]
fn hostile_payloads_are_refused_with_closed_causes() {
    let good = payload();
    assert_eq!(decode(&stored(&good)).unwrap(), decode(FIXTURE).unwrap());
    assert_eq!(decode(&stored(&good[..good.len() - 1])).unwrap_err(), JsonPackError::Truncated);
    let mut trailing = good.clone();
    trailing.push(0);
    assert_eq!(decode(&stored(&trailing)).unwrap_err(), JsonPackError::Trailing);
    let mut magic = good.clone();
    magic[0] = 0;
    assert_eq!(decode(&stored(&magic)).unwrap_err(), JsonPackError::Magic);
    assert_eq!(decode(&[0xff, 0xff]).unwrap_err().clone(), decode(&[0xff, 0xff]).unwrap_err());
    assert!(matches!(decode(&[0xff, 0xff]).unwrap_err(), JsonPackError::Inflate(_)));
    let unknown_tag = [&MAGIC[..], &[0, 9]].concat();
    assert_eq!(decode(&stored(&unknown_tag)).unwrap_err(), JsonPackError::Tag);
    let missing_string = [&MAGIC[..], &[0, STRING, 0]].concat();
    assert_eq!(decode(&stored(&missing_string)).unwrap_err(), JsonPackError::Index);
    let huge_count = [&MAGIC[..], &[0, ARRAY, 0xff, 0xff, 0xff, 0xff, 0x0f]].concat();
    assert_eq!(decode(&stored(&huge_count)).unwrap_err(), JsonPackError::Truncated);
    let overflow = [&MAGIC[..], &[0, UNSIGNED, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]].concat();
    assert_eq!(decode(&stored(&overflow)).unwrap_err(), JsonPackError::Overflow);
    let mut deep = [&MAGIC[..], &[0]].concat();
    for _ in 0..=MAXIMUM_DEPTH + 1 {
        deep.extend_from_slice(&[ARRAY, 1]);
    }
    deep.push(NULL);
    assert_eq!(decode(&stored(&deep)).unwrap_err(), JsonPackError::Depth);
    let invalid_utf8 = [&MAGIC[..], &[1, 1, 0xff, NULL]].concat();
    assert_eq!(decode(&stored(&invalid_utf8)).unwrap_err(), JsonPackError::Utf8);
}

#[test]
fn leaker_shares_equal_text_and_leaves_empty_slices_static() {
    let mut leak = Leaker::default();
    let first = leak.text("IfcWall");
    let second = leak.text(&String::from("IfcWall"));
    assert!(std::ptr::eq(first, second));
    assert_eq!(leak.text(""), "");
    assert!(Leaker::slice::<u8>(Vec::new()).is_empty());
    assert_eq!(Leaker::slice(vec![1, 2, 3]), &[1, 2, 3]);
    assert_eq!(leak.optional(Some(&Json::Null)).unwrap(), None);
    assert_eq!(leak.optional(Some(&Json::Str("x"))).unwrap(), Some("x"));
    assert_eq!(leak.texts(Some(&Json::Arr(vec![Json::Str("a"), Json::Str("b")]))).unwrap(), &["a", "b"]);
}
