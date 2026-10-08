//! 🧪️ Exact serde operation wire and same-thread allocator witnesses across admitted copy and cancellation turns.
use super::*;
use super::super::{ArtifactCanonicalJsonNode, ArtifactCanonicalJson};

struct OracleSource(serde_json::Value);
impl OracleSource {
    fn value(&self, path: &[usize]) -> Result<&serde_json::Value, String> {
        let mut value = &self.0;
        for index in path {
            value = match value {
                serde_json::Value::Array(values) => values.get(*index),
                serde_json::Value::Object(values) => values.values().nth(*index),
                _ => None,
            }.ok_or_else(|| "operation-wire.invalid-fixture-path".to_string())?;
        }
        Ok(value)
    }
}

struct TextOracle(serde_json::Value);
impl TextOracle {
    fn node(&self, path: &[usize]) -> &serde_json::Value {
        let mut node = &self.0;
        for index in path { node = &node["children"][*index]; }
        node
    }
    fn independent(node: &serde_json::Value, output: &mut Vec<u8>) {
        match node["kind"].as_str().unwrap() {
            "sequence" => {
                output.extend_from_slice(node["open"].as_str().unwrap().as_bytes());
                for (index, child) in node["children"].as_array().unwrap().iter().enumerate() {
                    if index > 0 { output.extend_from_slice(node["separator"].as_str().unwrap().as_bytes()); }
                    Self::independent(child, output);
                }
                output.extend_from_slice(node["close"].as_str().unwrap().as_bytes());
            }
            "hex" => for byte in node["text"].as_str().unwrap().as_bytes() { output.extend_from_slice(format!("{byte:02x}").as_bytes()); },
            _ => output.extend_from_slice(node["text"].as_str().unwrap().as_bytes()),
        }
    }
}
impl ArtifactOperationText for TextOracle {
    fn operation_text_node(&self, path: &[usize]) -> Result<ArtifactOperationTextNode<'_>, String> {
        let node = self.node(path);
        Ok(match node["kind"].as_str().unwrap() {
            "sequence" => ArtifactOperationTextNode::Sequence { length: node["children"].as_array().unwrap().len(), open: node["open"].as_str().unwrap().as_bytes(), separator: node["separator"].as_str().unwrap().as_bytes(), close: node["close"].as_str().unwrap().as_bytes() },
            "hex" => ArtifactOperationTextNode::Hex(node["text"].as_str().unwrap().as_bytes()),
            "scalar" => ArtifactOperationTextNode::Scalar,
            _ => ArtifactOperationTextNode::Bytes(node["text"].as_str().unwrap().as_bytes()),
        })
    }
    fn operation_text_scalar(&self, path: &[usize], offset: usize, output: &mut [u8]) -> Result<(usize, bool), String> {
        let bytes = self.node(path)["text"].as_str().unwrap().as_bytes();
        let count = output.len().min(bytes.len() - offset);
        output[..count].copy_from_slice(&bytes[offset..offset + count]);
        Ok((count, offset + count == bytes.len()))
    }
}

#[test]
fn prepared_operation_wire_borrowed_text_matches_neutral_ordered_tuple_and_hex() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for (index, row) in fixture["textCases"].as_array().unwrap().iter().enumerate() {
        let header: &'static [u8] = if index == 0 { &[1, 5] } else { &[1, 6] };
        let source = TextOracle(row["node"].clone());
        let mut expected = header.to_vec();
        TextOracle::independent(&source.0, &mut expected);
        for maximum in [1, 3, 7, 64, 256] {
            let (mut cursor, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(ArtifactPreparedOperationCursor::default);
            assert_eq!((birth.requested_bytes, birth.released_bytes), (0, 0));
            let mut actual = Vec::with_capacity(expected.len());
            for _ in 0..expected.len() * 4 + 64 {
                let mut output = [0; 256];
                let wire = || ArtifactPreparedOperationSource::Text { header, body: &source };
                let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(wire(), &mut output, RetainedCloneGrant::default()).unwrap());
                assert_eq!(denied, ArtifactPreparedOperationProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(wire(), &mut output, RetainedCloneGrant::one_payload_turn(maximum, 64)).unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(progress.processed_items, 1);
                assert!(progress.written_bytes <= maximum.min(64));
                actual.extend_from_slice(&output[..progress.written_bytes]);
                if progress.complete { break; }
            }
            assert_eq!(actual, expected);
        }
        for stop in fixture["cancelStops"].as_array().unwrap() {
            let mut cursor = ArtifactPreparedOperationCursor::default();
            for _ in 0..stop.as_u64().unwrap() { cursor.advance(ArtifactPreparedOperationSource::Text { header, body: &source }, &mut [0; 64], RetainedCloneGrant::one_payload_turn(64, 64)).unwrap(); }
            let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap());
            assert!(progress.complete);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        println!("[DEBUG] prepared operation text row={index} exact-independent-bytes={} one structural event or copy<=64 heap0 five cancellation stops", expected.len());
    }
}
impl ArtifactCanonicalJson for OracleSource {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
        use ArtifactCanonicalJsonNode as N;
        Ok(match self.value(path)? {
            serde_json::Value::Null => N::Null,
            serde_json::Value::Bool(value) => N::Bool(*value),
            serde_json::Value::Number(value) => if let Some(value) = value.as_i64() { N::I64(value) } else if let Some(value) = value.as_u64() { N::U64(value) } else { N::F64(value.as_f64().unwrap()) },
            serde_json::Value::String(value) => N::String(value),
            serde_json::Value::Array(values) => N::Array(values.len()),
            serde_json::Value::Object(values) => N::Object(values.len()),
        })
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<crate::os_store::ArtifactCanonicalJsonText<'_>, String> {
        self.value(path)?.as_object().and_then(|values| values.keys().nth(index)).map(|key| key.as_str().into()).ok_or_else(|| "operation-wire.invalid-fixture-key".to_string())
    }
}

#[test]
fn prepared_operation_wire_borrowed_json_matches_serde_without_birth_or_owner_copy() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for (row_index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let header: &'static [u8] = if row_index == 2 { &[1, 128, 1] } else { &[1, 6] };
        let mut body = row["body"].clone();
        if row_index == 0 { body["value"] = serde_json::Value::String("δ😀\0\n\"\\".repeat(512)); }
        let source = OracleSource(body);
        for hex in [false, true] {
        let mut expected = header.to_vec();
        let raw = serde_json::to_vec(&source.0).unwrap();
        if hex { expected.extend_from_slice(b"snapshot="); for byte in raw { expected.push(b"0123456789abcdef"[usize::from(byte >> 4)]); expected.push(b"0123456789abcdef"[usize::from(byte & 15)]); } } else { expected.extend(raw); }
        let wire_source = || if hex { ArtifactPreparedOperationSource::HexJson { header, prefix: b"snapshot=", body: &source } } else { ArtifactPreparedOperationSource::CanonicalJson { header, body: &source } };
        for maximum in [1, 3, 7, 64, 256] {
            let (mut cursor, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(ArtifactPreparedOperationCursor::default);
            assert_eq!((birth.requested_bytes, birth.released_bytes), (0, 0));
            let mut actual = Vec::with_capacity(expected.len());
            for _ in 0..expected.len() * 2 + 8 {
                let mut output = [0; 256];
                for grant in [RetainedCloneGrant::default(), RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: maximum, ..Default::default() }] {
                    let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(wire_source(), &mut output, grant).unwrap());
                    assert_eq!(progress, ArtifactPreparedOperationProgress::default());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                }
                let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(wire_source(), &mut output, RetainedCloneGrant::one_payload_turn(maximum, 64)).unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(progress.processed_items, 1);
                assert!(progress.written_bytes <= maximum.min(64));
                actual.extend_from_slice(&output[..progress.written_bytes]);
                if progress.complete { break; }
            }
            assert_eq!(actual, expected);
            if !hex { assert_eq!(serde_json::from_slice::<serde_json::Value>(&actual[header.len()..]).unwrap(), source.0); }
            assert_eq!(cursor.next_minimum_copy_bytes(), 0);
            assert_eq!((cursor.next_capacity_byte_demand().unwrap(), cursor.next_close_byte_demand().unwrap()), (0, 0));
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        for stop in fixture["cancelStops"].as_array().unwrap() {
            let mut cursor = ArtifactPreparedOperationCursor::default();
            for _ in 0..stop.as_u64().unwrap() { cursor.advance(wire_source(), &mut [0; 64], RetainedCloneGrant::one_payload_turn(64, 64)).unwrap(); }
            let before = &source.0 as *const serde_json::Value;
            let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap());
            assert!(progress.complete);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(&source.0 as *const serde_json::Value, before);
        }
        println!("[DEBUG] prepared operation wire row={row_index} hex={hex} exact-serde-bytes={} copy<=64 no births/frees all five cancellation stops preserve original typed owner", expected.len());
        }
    }
}
