//! 📏️ Every JSON writer step quotes exactly what its receipt charges.
use semio_framework_pack_json::{JsonBorrowedWriteCursor, JsonWriteNode, JsonWriteSource};
use semio_framework_value::{DslValue, RetainedCloneGrant, ValueError, ValueRefusalKind};

fn dsl_of(value: &serde_json::Value) -> DslValue {
    match value {
        serde_json::Value::Null => DslValue::Null,
        serde_json::Value::Bool(value) => DslValue::Bool(*value),
        serde_json::Value::Number(number) => number.as_u64().map(DslValue::uint).or_else(|| number.as_i64().map(DslValue::int)).unwrap_or_else(|| DslValue::float(number.as_f64().unwrap())),
        serde_json::Value::String(text) => DslValue::String(text.clone()),
        serde_json::Value::Array(items) => DslValue::Array(items.iter().map(dsl_of).collect()),
        serde_json::Value::Object(entries) => DslValue::Object(entries.iter().map(|(key, value)| (key.clone(), dsl_of(value))).collect()),
    }
}
/// 📏️ Every writer step quotes exactly the copy and capacity its receipt charges, and a quote does not change when it is asked again with the grant it implies.
fn assert_quotes_equal_receipts<S: JsonWriteSource + ?Sized>(name: &str, source: &S, copy: usize) -> Option<String> {
    let mut writer = JsonBorrowedWriteCursor::new();
    let mut callback = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new_retained(&mut callback);
    let mut output = None;
    for turn in 0..100000 {
        let first = writer.normal_step_demands_with_copy(source, copy).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy.max(first.copy_bytes), maximum_capacity_bytes: first.capacity_bytes, maximum_release_bytes: first.release_bytes, maximum_depth: first.depth };
        let again = writer.normal_step_demands_with_copy(source, grant.maximum_copy_bytes).unwrap();
        assert_eq!(first, again, "{name} copy={copy} turn={turn}: the quote changed when asked again with its own grant");
        let result = writer.step(source, 1, &mut control, grant).unwrap_or_else(|error| panic!("{name} copy={copy} turn={turn}: step refused {error:?} under quote {first:?}"));
        let receipt = writer.normal_step_progress();
        assert_eq!((receipt.copied_bytes, receipt.retained_capacity_bytes, receipt.released_bytes), (first.copy_bytes, first.capacity_bytes, first.release_bytes), "{name} copy={copy} turn={turn}: quote differs from receipt");
        if result.is_some() {
            output = result;
            break;
        }
    }
    drop(writer);
    output
}
#[test]
fn json_writer_quotes_equal_receipts_for_every_corpus_case_and_step() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️quote-equals-receipt.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let independent = serde_json::to_string(&case["value"]).unwrap();
        for copy in fixture["copies"].as_array().unwrap() {
            let source = dsl_of(&case["value"]);
            let output = assert_quotes_equal_receipts(name, &source, copy.as_u64().unwrap() as usize);
            assert_eq!(output.as_deref(), Some(independent.as_str()), "{name}: writer output matches independent serde JSON");
        }
    }
}
struct ScalarJsonSource(bool);
impl JsonWriteSource for ScalarJsonSource {
    fn node_at_path(&self, _: &[usize]) -> Result<JsonWriteNode<'_>, ValueError> {
        Ok(if self.0 { JsonWriteNode::Float32(1.5) } else { JsonWriteNode::QuotedUnsigned(u128::MAX) })
    }
    fn object_key_at_path(&self, _: &[usize], _: usize) -> Result<&str, ValueError> {
        Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "scalar source has no keys"))
    }
}
#[test]
fn json_writer_quotes_equal_receipts_for_binary32_and_quoted_unsigned_scalars() {
    for (float, expected) in [(true, "1.5".to_owned()), (false, format!("\"{}\"", u128::MAX))] {
        for copy in [64, 4096] {
            assert_eq!(assert_quotes_equal_receipts("scalar", &ScalarJsonSource(float), copy), Some(expected.clone()));
        }
    }
}
