//! 🧪️ Real Flow payload, codec, structural-diff, and Store inverse laws.
use super::*;
use crate::os_spr::{MutationLeaf, OpBinary, OpText};
use crate::{DslValue, FromValue, ToValue};

//#region 🧪️FixtureOwnership
fn cases() -> serde_json::Value { serde_json::from_str(include_str!("🔣️cases.json")).expect("neutral Flow cases") }
fn base() -> FlowFixture { serde_json::from_value(cases()["fixture"].clone()).expect("Flow fixture") }
fn operation(index: usize) -> FlowMutation {
    let cases = cases();
    let mut value = cases["positives"][index].clone();
    value["operation"] = cases["roster"][index]["operation"].clone();
    serde_json::from_value(value).expect("direct Flow operation")
}
fn retire_diff(diff: FlowDiff) {
    for delta in diff.deltas {
        match delta {
            FlowDelta::Widgets(delta) => {
                for (_, widget) in delta.inserted { widget.retire_cold(); }
                for (_, widget) in delta.replaced { widget.retire_cold(); }
            }
            FlowDelta::Fixture(fixture) => fixture.retire_cold(),
            _ => {}
        }
    }
}
fn apply(base: &FlowFixture, mutation: &FlowMutation) -> FlowFixture {
    let (diff, _) = mutation.diff(base).into_parts();
    let next = diff.apply(base).expect("valid Flow delta");
    retire_diff(diff);
    next
}
fn retire_mutation(mutation: FlowMutation) {
    match mutation {
        FlowMutation::AddWidget(leaf) => leaf.widget.retire_cold(),
        FlowMutation::ChangeWidget(leaf) => leaf.widget.retire_cold(),
        FlowMutation::ReplaceFlowFixture(leaf) => leaf.fixture.retire_cold(),
        _ => {}
    }
}
fn assert_codecs(mutation: &FlowMutation) {
    let decoded = FlowMutation::parse_op(&mutation.print_op()).expect("Flow text");
    assert_eq!(decoded, *mutation);
    retire_mutation(decoded);
    let bytes = mutation.encode_op().expect("Flow binary");
    assert_eq!(bytes[0], crate::os_dsl::variants_binary::OP_BINARY_FORMAT);
    assert_eq!(u32::from(bytes[1]), mutation.descriptor().binary_tag.expect("binary tag"));
    let decoded = FlowMutation::decode_op(&bytes).expect("Flow binary decode");
    assert_eq!(decoded, *mutation);
    retire_mutation(decoded);
    let decoded = serde_json::from_value::<FlowMutation>(serde_json::to_value(mutation).expect("serialize")).expect("deserialize");
    assert_eq!(decoded, *mutation);
    retire_mutation(decoded);
    // 🛡️ First-party round-trip — proves `FlowMutation::from_value` (internally-tagged, no
    // `content`, single-unnamed-payload variants) actually decodes every real leaf correctly now
    // that the tag key is stripped before reaching the leaf's own `#[value(deny_unknown_fields)]`
    // check (see `🦀️component.rs`'s tag-stripping fix). A silently-always-erroring `from_value`
    // would make the unknown-field `is_err()` assertions in `assert_leaf_contract` pass for the
    // wrong reason — this is what rules that out.
    let decoded = FlowMutation::from_value(mutation.to_value()).expect("first-party Flow decode");
    assert_eq!(decoded, *mutation);
    retire_mutation(decoded);
}
pub(crate) fn assert_leaf_contract<T>(index: usize, wrap: fn(T) -> FlowMutation, descriptor: &str)
where T: MutationLeaf + serde::Serialize + serde::de::DeserializeOwned + FromValue {
    let cases = cases();
    let payload = cases["positives"][index].clone();
    let leaf: T = serde_json::from_value(payload.clone()).expect("actual leaf payload");
    let mutation = wrap(leaf);
    assert_eq!(serde_json::to_value(T::DESCRIPTOR).expect("descriptor"), serde_json::from_str::<serde_json::Value>(descriptor).expect("owned descriptor"));
    assert!(T::DESCRIPTOR.validate().is_ok());
    assert_eq!(mutation.descriptor(), &T::DESCRIPTOR);
    assert_eq!(mutation.descriptor().binary_tag, Some(u32::try_from(index).expect("bounded roster")));
    assert_codecs(&mutation);
    // 🛡️ First-party path (NOT serde_json) — the real acceptance criterion for
    // `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`'s
    // enum `deny_unknown_fields` fix: `serde_json::to_value`/`json!` still build the malformed
    // JSON tree (cheap, unrelated to the codec under test), but decoding it now goes through
    // `FromValue::from_value(DslValue::from(&value))`, exercising the derive's own enforcement.
    let mut unknown = payload.clone();
    unknown["unknown"] = serde_json::json!(true);
    assert!(T::from_value(DslValue::from(&unknown)).is_err());
    let mut unknown = serde_json::to_value(&mutation).expect("mutation");
    unknown["unknown"] = serde_json::json!(true);
    assert!(FlowMutation::from_value(DslValue::from(&unknown)).is_err());
    for field in cases["roster"][index]["required"].as_array().expect("required fields") {
        let mut missing = payload.clone();
        missing.as_object_mut().expect("payload").remove(field.as_str().expect("field"));
        assert!(serde_json::from_value::<T>(missing).is_err());
    }
    let before = base();
    let mut restored = apply(&before, &mutation);
    let inverse = mutation.inverse(&before);
    assert!(!inverse.is_empty());
    for inverse in inverse.into_iter().rev() {
        let next = apply(&restored, &inverse);
        retire_mutation(inverse);
        restored.retire_cold();
        restored = next;
    }
    assert_eq!(restored, before);
    restored.retire_cold();
    before.retire_cold();
    retire_mutation(mutation);
}

//#endregion 🧪️FixtureOwnership

//#region 🧪️Laws
#[test]
fn all_ten_codecs_and_descriptors() {
    assert_eq!(<FlowMutation as Mutation<FlowFixture>>::DESCRIPTORS.len(), 10);
    for index in 0..10 { let mutation = operation(index); assert_codecs(&mutation); retire_mutation(mutation); }
    for index in [0, 2, 4, 6] {
        for value in ["-1", "0.5", "4294967296", "1e21"] {
            let cases = cases();
            let mut payload = cases["positives"][index].clone();
            let field = if index == 0 || index == 4 { "index" } else { "toIndex" };
            payload[field] = serde_json::from_str(value).expect("JSON number");
            payload["operation"] = cases["roster"][index]["operation"].clone();
            assert!(serde_json::from_value::<FlowMutation>(payload).is_err());
        }
    }
    assert!(FlowMutation::decode_op(&[1, 10]).is_err());
    assert!(FlowMutation::decode_op(&[2, 0]).is_err());
}


/// 📏️ Exercises actual derived record, text, and binary conversion before an index can truncate.
#[test]
fn index_codecs_reject_overflow() {
    for index in [0, 2, 4, 6] {
        let mutation = operation(index);
        let (keyword, mut record) = <FlowMutation as crate::os_dsl::DslVariants>::to_named_record(&mutation);
        let value = record.fields.values_mut().find(|value| matches!(value, crate::os_dsl::FieldValue::UInt(_))).expect("direct index field");
        *value = crate::os_dsl::FieldValue::UInt(u64::from(u32::MAX) + 1);
        assert!(<FlowMutation as crate::os_dsl::DslVariants>::from_named_record(&keyword, &record).is_err());
        let spec = (<FlowMutation as crate::os_dsl::DslVariants>::variants()[index].1)();
        let text = crate::os_dsl::print(&record, &spec, crate::os_dsl::JoinMode::Inline);
        assert!(FlowMutation::parse_op(&text).is_err());
        let mut bytes = vec![1, u8::try_from(index).expect("ten leaves")];
        bytes.extend(crate::os_pack::encode_record_body(&spec, &record, &crate::os_pack::EncodeOptions::default()).expect("wide UInt body"));
        assert!(FlowMutation::decode_op(&bytes).is_err());
        retire_mutation(mutation);
    }
    let mut invalid = vec![1];
    crate::os_pack::write_varint_u64(&mut invalid, u64::MAX);
    assert!(FlowMutation::decode_op(&invalid).is_err());
}

#[test]
fn ordered_collection_and_inverse_laws() {
    for (index, expected) in [(0, vec!["a","x","b","c"]), (1,vec!["a","c"]), (2,vec!["c","a","b"]), (3,vec!["renamed","b","c"])] {
        let before = base();
        let mutation = operation(index);
        let after = apply(&before, &mutation);
        assert_eq!(after.widgets.iter().map(|widget| widget.id().as_str()).collect::<Vec<_>>(), expected);
        let mut restored = after;
        for inverse in mutation.inverse(&before).into_iter().rev() {
            let next = apply(&restored, &inverse);
            retire_mutation(inverse);
            restored.retire_cold();
            restored = next;
        }
        assert_eq!(restored, before);
        before.retire_cold();
        restored.retire_cold();
        retire_mutation(mutation);
    }
    for (index, expected) in [(4, vec!["s1","s3","s2"]), (5,vec!["s2"]), (6,vec!["s2","s1"]), (7,vec!["s1-new","s2"])] {
        let before = base();
        let mutation = operation(index);
        let after = apply(&before, &mutation);
        assert_eq!(after.synapses.iter().map(|synapse| synapse.id.as_str()).collect::<Vec<_>>(), expected);
        let mut restored = after;
        for inverse in mutation.inverse(&before).into_iter().rev() {
            let next = apply(&restored, &inverse);
            retire_mutation(inverse);
            restored.retire_cold();
            restored = next;
        }
        assert_eq!(restored, before);
        before.retire_cold();
        restored.retire_cold();
        retire_mutation(mutation);
    }
}

#[test]
fn structural_composition_is_ordered() {
    let before = base();
    let mut current = before.clone();
    let mut combined = FlowDiff::default();
    for index in [0, 2, 4, 6, 9, 8] {
        let mutation = operation(index);
        let (diff, _) = mutation.diff(&current).into_parts();
        retire_mutation(mutation);
        let next = diff.apply(&current).expect("sequential diff");
        combined.absorb(diff);
        current.retire_cold();
        current = next;
    }
    let result = combined.apply(&before).expect("composed diff");
    assert_eq!(result, current);
    assert_eq!(combined.deltas.len(), 6);
    assert!(serde_json::to_value(&combined).expect("structural JSON").get("operations").is_none());
    result.retire_cold();
    current.retire_cold();
    before.retire_cold();
    retire_diff(combined);
}

#[test]
fn repeated_layout_inverse_uses_store_order() {
    let before = base();
    let mutation = operation(8);
    let inverse = mutation.inverse(&before);
    assert_eq!(inverse.len(), 3);
    assert_eq!(serde_json::to_value(&inverse).expect("inverse"), serde_json::json!([
        {"operation":"changeLayout","entries":[{"id":"a","layout":{"x":1.0,"y":2.0}}]},
        {"operation":"changeLayout","entries":[{"id":"a","layout":{"x":3.0,"y":4.0}}]},
        {"operation":"changeLayout","entries":[{"id":"a","layout":null}]}
    ]));
    let mut restored = apply(&before, &mutation);
    for inverse in inverse.iter().rev() {
        let next = apply(&restored, inverse);
        restored.retire_cold();
        restored = next;
    }
    assert_eq!(restored, before);
    restored.retire_cold();
    before.retire_cold();
}

#[test]
fn typed_rejection_is_atomic() {
    let before = base();
    let original = serde_json::to_value(&before).expect("base");
    let invalid = [
        FlowMutation::AddWidget(AddWidget { index: u32::MAX, widget: Widget::InputNote { id:"x".into(),text:String::new() } }),
        FlowMutation::AddWidget(AddWidget { index: 0, widget: before.widgets[0].clone() }),
        FlowMutation::MoveWidget(MoveWidget { id:"c".into(),to_index:u32::MAX }),
        FlowMutation::RemoveWidget(RemoveWidget { id:"missing".into() }),
        FlowMutation::ChangeWidget(ChangeWidget { id:"a".into(),widget:before.widgets[1].clone() }),
        FlowMutation::AddSynapse(AddSynapse { index:u32::MAX,synapse:before.synapses[0].clone() }),
        FlowMutation::MoveSynapse(MoveSynapse { id:"s2".into(),to_index:u32::MAX }),
        FlowMutation::RemoveSynapse(RemoveSynapse { id:"missing".into() }),
        FlowMutation::ChangeLayout(ChangeLayout { entries:vec![FlowLayoutEntry {id:"a".into(),layout:Some(WidgetLayout{x:9.0,y:9.0})},FlowLayoutEntry{id:"missing".into(),layout:None}] }),
    ];
    for mutation in invalid {
        let (diff, _) = mutation.diff(&before).into_parts();
        assert!(diff.apply(&before).is_err());
        retire_diff(diff);
        retire_mutation(mutation);
        assert_eq!(serde_json::to_value(&before).expect("unchanged base"), original);
    }
    assert!(FlowMutation::RemoveWidget(RemoveWidget{id:"missing".into()}).inverse(&before).is_empty());
    assert!(FlowMutation::MoveSynapse(MoveSynapse{id:"missing".into(),to_index:0}).inverse(&before).is_empty());
    before.retire_cold();
}

#[test]
fn actual_nested_serde_shapes() {
    for value in cases()["widgets"].as_array().expect("widget cases") {
        let widget: Widget = serde_json::from_value(value.clone()).expect("actual widget");
        assert_eq!(serde_json::to_value(&widget).expect("canonical widget"), *value);
        let mutation = FlowMutation::AddWidget(AddWidget{index:0,widget});
        assert_codecs(&mutation);
        let FlowMutation::AddWidget(AddWidget{widget,..}) = mutation else { unreachable!() };
        FlowFixture{schema:String::new(),camera:CameraJson::default(),widgets:vec![widget],synapses:vec![],layout:crate::OrderedMap::new()}.retire_cold();
    }
    for text in [r#"{"index":0,"widget":{}}"#, r#"{"index":0,"widget":{"kind":"neuron","id":"n"}}"#, r#"{"index":0,"widget":{"kind":"neuron","id":"n","neuronKind":"x","params":{"bad":[]}}}"#] {
        assert!(serde_json::from_str::<AddWidget>(text).is_err());
    }
    let option: ChangeLayout = serde_json::from_str(r#"{"entries":[{"id":"a"},{"id":"a","layout":null}]}"#).expect("nullable omittable Option");
    assert!(option.entries.iter().all(|entry|entry.layout.is_none()));
    assert!(serde_json::from_str::<ChangeLayout>(r#"{"entries":[{"id":"a","layout":{} }]}"#).is_err());
    assert!(serde_json::from_str::<ChangeLayout>(r#"{"entries":[{"id":"a","unknown":1}]}"#).is_err());
}

#[test]
fn diff_json_contract_matches_native_serde() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../🧬️schema/🔺️diff/🧪️tests/🔣️vectors.json")).unwrap();
    for row in vectors["valid"].as_array().unwrap() {
        let diff: FlowDiff = serde_json::from_value(row["value"].clone()).unwrap_or_else(|error| panic!("{}: {error}", row["name"]));
        let decoded: FlowDiff = serde_json::from_value(serde_json::to_value(&diff).unwrap()).unwrap();
        assert_eq!(decoded, diff);
        retire_diff(decoded);
        // 🛡️ First-party round-trip — proves `FlowDelta::from_value` (adjacently-tagged,
        // `tag = "delta", content = "value"`) decodes every real fragment shape correctly.
        let first_party = FlowDiff::from_value(DslValue::from(&row["value"])).unwrap_or_else(|error| panic!("{} (first-party): {error:?}", row["name"]));
        assert_eq!(first_party, diff);
        retire_diff(first_party);
        retire_diff(diff);
    }
    for row in vectors["invalid"].as_array().unwrap() {
        assert!(serde_json::from_value::<FlowDiff>(row["value"].clone()).is_err(), "{}", row["name"]);
        // 🛡️ First-party path — the real acceptance criterion: "unknown-delta-envelope-field"
        // exercises FlowDelta's adjacently-tagged OUTER `{tag, content}` enforcement, and
        // "unknown-fragment-field" exercises the CONTENT payload's own (struct-level, pre-existing)
        // `deny_unknown_fields`. Both must still fail through `FromValue`, not just `serde_json`.
        assert!(FlowDiff::from_value(DslValue::from(&row["value"])).is_err(), "{} (first-party)", row["name"]);
    }
}
//#endregion 🧪️Laws
