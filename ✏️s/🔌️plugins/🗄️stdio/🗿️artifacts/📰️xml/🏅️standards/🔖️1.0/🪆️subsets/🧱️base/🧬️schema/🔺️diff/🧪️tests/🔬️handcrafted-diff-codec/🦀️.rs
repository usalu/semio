use super::*;
use protocol::DiffCodec;

/// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `XmlDiff` grammar — exercises the
/// recursive enum tree (`Element`/`Text`/`Replace` `XmlNodeDiff` variants), both top-level
/// tri-states, attribute add/remove/modify, and nested child add/remove/modify. Reuses
/// `demo_diff_cases()` (the single prolog of truth also consumed by
/// `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`).
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = XmlDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = XmlDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[test]
fn explicit_null_removes_declaration_and_doctype_without_erasing_unchanged_fields() {
    use semio_framework_value::ToValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏳️optional-clear/🔣️.json")).unwrap();
    let mut before = XmlSnapshot::default();
    before.doc.declaration = Some(XmlDeclaration { version: "1.0".into(), ..Default::default() });
    before.doc.doctype = Some(XmlDoctype { name: "root".into(), ..Default::default() });
    before.doc.root = Some(XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: Vec::new() });
    for case in fixture["cases"].as_array().unwrap() {
        let diff: XmlDiff = semio_framework_pack_json::from_json_str(&case["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&diff.to_value())).unwrap();
        assert_eq!(oracle, case["diff"]);
        let mut after = before.clone();
        if case["diff"].get("declaration").is_some() {
            after.doc.declaration = None;
        }
        if case["diff"].get("doctype").is_some() {
            after.doc.doctype = None;
        }
        for replay in [diff.clone(), XmlDiff::parse_diff(&diff.print_diff()).unwrap(), XmlDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap()] {
            assert_eq!(replay.apply(&before).unwrap(), after, "case={}", case["name"]);
            assert_eq!(replay.inverse(&before).apply(&after).unwrap(), before);
        }
    }
}

#[test]
fn attribute_reordering_and_composed_structural_edits_preserve_exact_identity_order() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️attribute-order/🔣️.json")).unwrap();
    let states: Vec<XmlSnapshot> = fixture["states"]
        .as_array()
        .unwrap()
        .iter()
        .map(|state| {
            let mut snapshot = XmlSnapshot::default();
            let attrs = state.as_array().unwrap().iter().map(|pair| XmlAttr { name: pair[0].as_str().unwrap().into(), value: pair[1].as_str().unwrap().into() }).collect();
            snapshot.doc.root = Some(XmlNode::Element { name: "root".into(), attrs, children: Vec::new() });
            snapshot
        })
        .collect();
    for start in 0..states.len() {
        let mut combined = XmlDiff::default();
        for end in start + 1..states.len() {
            let next = XmlDiff::between(&states[end - 1], &states[end]);
            assert!(!next.is_empty());
            combined.absorb(next);
            for replay in [combined.clone(), XmlDiff::parse_diff(&combined.print_diff()).unwrap(), XmlDiff::decode_diff(&combined.encode_diff().unwrap()).unwrap()] {
                assert_eq!(replay.apply(&states[start]).unwrap(), states[end], "range={start}..{end}");
                assert_eq!(replay.inverse(&states[start]).apply(&states[end]).unwrap(), states[start]);
                let XmlNode::Element { attrs, .. } = replay.apply(&states[start]).unwrap().doc.root.unwrap() else { unreachable!() };
                let oracle: Vec<Vec<String>> = serde_json::from_value(fixture["states"][end].clone()).unwrap();
                assert_eq!(attrs.into_iter().map(|attr| vec![attr.name, attr.value]).collect::<Vec<_>>(), oracle);
            }
        }
    }
    for order in fixture["invalidOrders"].as_array().unwrap() {
        let wire = serde_json::json!({ "root": { "kind": "element", "attributes": { "order": order } } });
        let diff: XmlDiff = semio_framework_pack_json::from_json_str(&wire.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert!(diff.apply(&states[0]).is_err(), "invalid order={order}");
    }
    for case in fixture["invalidChildDiffs"].as_array().unwrap() {
        let diff: XmlChildrenDiff = semio_framework_pack_json::from_json_str(&case["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let before: Vec<XmlNode> = semio_framework_pack_json::from_json_str(&case["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut binary = Vec::new();
        diff.encode_binary(&mut binary);
        for replay in [diff.clone(), XmlChildrenDiff::decode_text(&diff.encode_text()).unwrap(), XmlChildrenDiff::decode_binary(&mut store::ByteReader::new(&binary)).unwrap()] {
            let mut children = before.clone();
            assert!(replay.apply_to(&mut children).is_err(), "case={}", case["name"]);
            assert_eq!(children, before, "case={}", case["name"]);
        }
    }
}
