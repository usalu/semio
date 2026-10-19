//! 🧪️ Real Flow payload, codec, structural-diff, and Store inverse laws.
use crate::os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use super::*;
use crate::os_spr::{Mutation, MutationLeaf, OpBinary, OpText};
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;

//#region 🧪️FixtureOwnership
fn cases() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("neutral Flow cases") }
/// 🧾️ The committed wire witness of each roster row: the Rust `ToValue` of one aggregate operation, in roster order.
const WITNESSES: [&str; 9] = [
    include_str!("../🧫️fixtures/🧬️mutations/➕️add-widget/🧾️wire-witness/🦠️mutation/🔣️.json"),
    include_str!("../🧫️fixtures/🧬️mutations/🗑️remove-widget/🧾️wire-witness/🦠️mutation/🔣️.json"),
    include_str!("../🧫️fixtures/🧬️mutations/↔️move-widget/🧾️wire-witness/🦠️mutation/🔣️.json"),
    include_str!("../🧫️fixtures/🧬️mutations/🩹change-widget/🧾️wire-witness/🦠️mutation/🔣️.json"),
    include_str!("../🧫️fixtures/🧬️mutations/🔗️add-synapse/🧾️wire-witness/🦠️mutation/🔣️.json"),
    include_str!("../🧫️fixtures/🧬️mutations/✂️remove-synapse/🧾️wire-witness/🦠️mutation/🔣️.json"),
    include_str!("../🧫️fixtures/🧬️mutations/🔀️move-synapse/🧾️wire-witness/🦠️mutation/🔣️.json"),
    include_str!("../🧫️fixtures/🧬️mutations/🔄change-synapse/🧾️wire-witness/🦠️mutation/🔣️.json"),
    include_str!("../🧫️fixtures/🧬️mutations/📐️change-layout/🧾️wire-witness/🦠️mutation/🔣️.json"),
];
fn witness(index: usize) -> serde_json::Value { serde_json::from_str(WITNESSES[index]).expect("committed Flow wire witness") }
fn positive(index: usize) -> serde_json::Value {
    let mut payload = witness(index);
    payload.as_object_mut().expect("wire object").remove("operation");
    payload
}
fn third_party_json<T: ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("first-party JSON must remain valid RFC 8259")
}
pub(crate) fn base() -> FlowHostSnapshot { FlowHostSnapshot::from_value(semio_framework_value::DslValue::from(&cases()["hostSnapshot"])).expect("Flow host document") }
pub(crate) fn operation(index: usize) -> FlowMutation {
    FlowMutation::from_value(semio_framework_value::DslValue::from(&witness(index))).expect("direct Flow operation")
}
fn retire_diff(diff: FlowDiff) {
    for delta in diff.deltas {
        match delta {
            FlowDelta::Widgets(delta) => {
                for (_, widget) in delta.inserted { widget.retire_cold(); }
                for (_, widget) in delta.replaced { widget.retire_cold(); }
            }
            _ => {}
        }
    }
}
fn apply(base: &FlowHostSnapshot, mutation: &FlowMutation) -> FlowHostSnapshot {
    let (diff, _) = mutation.diff(base).into_parts();
    let next = protocol::apply_diff_cold(&diff, base).expect("valid Flow delta");
    retire_diff(diff);
    next
}
pub(crate) fn retire_mutation(mutation: FlowMutation) {
    match mutation {
        FlowMutation::AddWidget(leaf) => leaf.widget.retire_cold(),
        FlowMutation::ChangeWidget(leaf) => leaf.widget.retire_cold(),
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
    let decoded = FlowMutation::from_value(mutation.to_value()).expect("first-party Flow decode");
    assert_eq!(decoded, *mutation);
    retire_mutation(decoded);
}
pub(crate) fn assert_leaf_contract<T>(index: usize, wrap: fn(T) -> FlowMutation, descriptor: &str)
where T: MutationLeaf + ToValue + FromValue {
    let cases = cases();
    let payload = positive(index);
    let leaf = T::from_value(semio_framework_value::DslValue::from(&payload)).expect("actual leaf payload");
    let mutation = wrap(leaf);
    assert_eq!(third_party_json(&T::DESCRIPTOR), serde_json::from_str::<serde_json::Value>(descriptor).expect("owned descriptor"));
    assert!(T::DESCRIPTOR.validate().is_ok());
    assert_eq!(mutation.descriptor(), &T::DESCRIPTOR);
    assert_eq!(mutation.descriptor().binary_tag, Some(u32::try_from(index).expect("bounded roster")));
    assert_codecs(&mutation);
    let mut unknown = payload.clone();
    unknown["unknown"] = serde_json::json!(true);
    assert!(T::from_value(semio_framework_value::DslValue::from(&unknown)).is_err());
    let mut unknown = third_party_json(&mutation);
    unknown["unknown"] = serde_json::json!(true);
    assert!(FlowMutation::from_value(semio_framework_value::DslValue::from(&unknown)).is_err());
    for field in cases["roster"][index]["required"].as_array().expect("required fields") {
        let mut missing = payload.clone();
        missing.as_object_mut().expect("payload").remove(field.as_str().expect("field"));
        assert!(T::from_value(semio_framework_value::DslValue::from(&missing)).is_err());
    }
    let before = base();
    let mut restored = apply(&before, &mutation);
    let inverse = mutation.inverse(&before).expect("valid retained mutation inverse fixture");
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


/// 🧱️ The base with a third synapse so every synapse kind has a first, a middle and a last row to hit.
pub(crate) fn base_with_three_synapses() -> FlowHostSnapshot {
    let mut document = base();
    document.synapses.push(synapse("s3", "a", "c"));
    document
}
pub(crate) fn synapse(id: &str, from: &str, to: &str) -> SynapseSpec {
    SynapseSpec { id: id.into(), from: from.into(), to: to.into(), from_port: "out".into(), to_port: "in".into() }
}
pub(crate) fn note(id: &str) -> Widget { Widget::InputNote { id: id.into(), text: id.to_uppercase() } }

//#endregion 🧪️FixtureOwnership

//#region 🧪️Laws
#[test]
fn all_nine_codecs_and_descriptors() {
    assert_eq!(<FlowMutation as Mutation<FlowHostSnapshot>>::DESCRIPTORS.len(), 9);
    for index in 0..9 {
        let mutation = operation(index);
        assert_eq!(third_party_json(&mutation), witness(index), "the committed wire witness is the canonical Rust wire");
        assert_codecs(&mutation);
        retire_mutation(mutation);
    }
    for index in [0, 2, 4, 6] {
        for value in ["-1", "0.5", "4294967296", "1e21"] {
            let cases = cases();
            let mut payload = positive(index);
            let field = if index == 0 || index == 4 { "index" } else { "toIndex" };
            payload[field] = serde_json::from_str(value).expect("JSON number");
            payload["operation"] = cases["roster"][index]["operation"].clone();
            assert!(FlowMutation::from_value(semio_framework_value::DslValue::from(&payload)).is_err());
        }
    }
    assert!(FlowMutation::decode_op(&[1, 9]).is_err());
    assert!(FlowMutation::decode_op(&[2, 0]).is_err());
}


/// 📏️ Exercises actual derived record, text, and binary conversion before an index can truncate.
#[test]
fn index_codecs_reject_overflow() {
    for index in [0, 2, 4, 6] {
        let mutation = operation(index);
        let (keyword, mut record) = <FlowMutation as semio_framework_dsl_record::DslVariants>::to_named_record(&mutation);
        let value = record.fields.values_mut().find(|value| matches!(value, semio_framework_dsl_record::FieldValue::UInt(_))).expect("direct index field");
        *value = semio_framework_dsl_record::FieldValue::UInt(u64::from(u32::MAX) + 1);
        assert!(<FlowMutation as semio_framework_dsl_record::DslVariants>::from_named_record(&keyword, &record).is_err());
        let spec = (<FlowMutation as semio_framework_dsl_record::DslVariants>::variants()[index].1.ordinary)();
        let text = semio_framework_dsl_record::print(&record, &spec, semio_framework_dsl_record::JoinMode::Inline);
        assert!(FlowMutation::parse_op(&text).is_err());
        let mut bytes = vec![1, u8::try_from(index).expect("nine leaves")];
        bytes.extend(crate::os_pack::record::encode_record_body(&spec, &record, &crate::os_pack::record::EncodeOptions::default()).expect("wide UInt body"));
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
        for inverse in mutation.inverse(&before).expect("valid retained mutation inverse fixture").into_iter().rev() {
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
        for inverse in mutation.inverse(&before).expect("valid retained mutation inverse fixture").into_iter().rev() {
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

/// 🔀️ L1/L2: a move is ONE `{id, from, to}` row read from the base position — no widget record travels — and its inverse is the
/// reverse move; the negative diff is the same reverse row, read without applying anything.
#[test]
fn a_move_is_one_positional_row_and_inverts_to_the_reverse_move() {
    let before = base();
    let mutation = operation(2);
    let (diff, _) = mutation.diff(&before).into_parts();
    assert_eq!(third_party_json(&diff), serde_json::json!({"deltas":[{"delta":"widgets","value":{"removed":[],"inserted":[],"replaced":[],"moved":[{"id":"c","from":2,"to":0}]}}]}));
    assert_eq!(third_party_json(&mutation.inverse(&before).expect("valid retained mutation inverse fixture")), serde_json::json!([{"operation":"moveWidget","id":"c","toIndex":2}]));
    let negative = crate::os_spr::DiffAlgebra::<FlowHostSnapshot>::inverse(&diff, &before);
    assert_eq!(third_party_json(&negative), serde_json::json!({"deltas":[{"delta":"widgets","value":{"removed":[],"inserted":[],"replaced":[],"moved":[{"id":"c","from":0,"to":2}]}}]}));
    let moved = protocol::apply_diff_cold(&diff, &before).expect("move applies");
    let restored = protocol::apply_diff_cold(&negative, &moved).expect("negative applies");
    assert_eq!(restored, before);
    for snapshot in [moved, restored, before] { snapshot.retire_cold(); }
    retire_diff(diff);
    retire_diff(negative);
    retire_mutation(mutation);
}

/// 🔀️ A delta that removes AND moves inverts into two ordered deltas (un-move first, then the base rows come back) because the
/// moves apply last.
#[test]
fn a_mixed_remove_and_move_delta_inverts_in_two_ordered_deltas() {
    let before = base();
    let diff = FlowDiff::from(FlowDelta::Widgets(FlowCollectionDelta { removed: vec!["b".into()], moved: vec![FlowMove { id: "c".into(), from: 1, to: 0 }], ..FlowCollectionDelta::default() }));
    let negative = crate::os_spr::DiffAlgebra::<FlowHostSnapshot>::inverse(&diff, &before);
    assert_eq!(negative.deltas.len(), 2);
    let after = protocol::apply_diff_cold(&diff, &before).expect("mixed delta applies");
    assert_eq!(after.widgets.iter().map(|widget| widget.id().as_str()).collect::<Vec<_>>(), ["c", "a"]);
    let restored = protocol::apply_diff_cold(&negative, &after).expect("two-delta inverse applies");
    assert_eq!(restored, before);
    for snapshot in [after, restored, before] { snapshot.retire_cold(); }
    retire_diff(diff);
    retire_diff(negative);
}

/// ➕️ Adjacent layout deltas coalesce on absorb, so the summed single-entry inverses equal the one negative layout delta.
#[test]
fn absorbing_layout_deltas_concatenates_their_entries() {
    use crate::os_spr::MutationDiff;
    let entry = |id: &str, x: f64| FlowLayoutEntry { id: id.into(), layout: Some(WidgetLayout { x, y: x }) };
    let mut left = FlowDiff::from(FlowDelta::Layout(vec![entry("a", 1.0)]));
    left.absorb(FlowDiff::from(FlowDelta::Layout(vec![entry("b", 2.0), entry("a", 3.0)])));
    left.absorb(FlowDiff::from(FlowDelta::Synapses(FlowCollectionDelta::default())));
    left.absorb(FlowDiff::from(FlowDelta::Layout(vec![entry("c", 4.0)])));
    assert_eq!(left.deltas.len(), 3);
    assert_eq!(left.deltas[0], FlowDelta::Layout(vec![entry("a", 1.0), entry("b", 2.0), entry("a", 3.0)]));
    retire_diff(left);
}

#[test]
fn structural_composition_is_ordered() {
    let before = base();
    let mut current = before.clone();
    let mut combined = FlowDiff::default();
    for index in [0, 2, 4, 6, 8] {
        let mutation = operation(index);
        let (diff, _) = mutation.diff(&current).into_parts();
        retire_mutation(mutation);
        let next = protocol::apply_diff_cold(&diff, &current).expect("sequential diff");
        combined.absorb(diff);
        current.retire_cold();
        current = next;
    }
    let result = protocol::apply_diff_cold(&combined, &before).expect("composed diff");
    assert_eq!(result, current);
    assert_eq!(combined.deltas.len(), 5);
    assert!(third_party_json(&combined).get("operations").is_none());
    result.retire_cold();
    current.retire_cold();
    before.retire_cold();
    retire_diff(combined);
}

#[test]
fn repeated_layout_inverse_uses_store_order() {
    let before = base();
    let mutation = operation(8);
    let inverse = mutation.inverse(&before).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 3);
    assert_eq!(third_party_json(&inverse), serde_json::json!([
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
    let original = third_party_json(&before);
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
        assert!(protocol::apply_diff_cold(&diff, &before).is_err());
        retire_diff(diff);
        retire_mutation(mutation);
        assert_eq!(third_party_json(&before), original);
    }
    assert!(FlowMutation::RemoveWidget(RemoveWidget{id:"missing".into()}).inverse(&before).expect("valid retained mutation inverse fixture").is_empty());
    assert!(FlowMutation::MoveSynapse(MoveSynapse{id:"missing".into(),to_index:0}).inverse(&before).expect("valid retained mutation inverse fixture").is_empty());
    before.retire_cold();
}

#[test]
fn actual_nested_first_party_shapes() {
    for value in cases()["widgets"].as_array().expect("widget cases") {
        let widget = Widget::from_value(semio_framework_value::DslValue::from(value)).expect("actual widget");
        assert_eq!(third_party_json(&widget), *value);
        let mutation = FlowMutation::AddWidget(AddWidget{index:0,widget});
        assert_codecs(&mutation);
        let FlowMutation::AddWidget(AddWidget{widget,..}) = mutation else { unreachable!() };
        FlowHostSnapshot{schema:String::new(),camera:CameraJson::default(),widgets:vec![widget],synapses:vec![],layout:crate::OrderedMap::new()}.retire_cold();
    }
    for text in [r#"{"index":0,"widget":{}}"#, r#"{"index":0,"widget":{"kind":"neuron","id":"n"}}"#, r#"{"index":0,"widget":{"kind":"neuron","id":"n","neuronKind":"x","params":{"bad":[]}}}"#] {
        let value: serde_json::Value = serde_json::from_str(text).expect("JSON syntax");
        assert!(AddWidget::from_value(semio_framework_value::DslValue::from(&value)).is_err());
    }
    let option = ChangeLayout::from_value(semio_framework_value::DslValue::from(&serde_json::from_str::<serde_json::Value>(r#"{"entries":[{"id":"a"},{"id":"a","layout":null}]}"#).unwrap())).expect("nullable omittable Option");
    assert!(option.entries.iter().all(|entry|entry.layout.is_none()));
    for text in [r#"{"entries":[{"id":"a","layout":{} }]}"#, r#"{"entries":[{"id":"a","unknown":1}]}"#] {
        let value: serde_json::Value = serde_json::from_str(text).unwrap();
        assert!(ChangeLayout::from_value(semio_framework_value::DslValue::from(&value)).is_err());
    }
}

#[test]
fn diff_json_contract_matches_third_party_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔺️diff/🔣️.json")).unwrap();
    for row in vectors["valid"].as_array().unwrap() {
        let diff = FlowDiff::from_value(semio_framework_value::DslValue::from(&row["value"])).unwrap_or_else(|error| panic!("{}: {error}", row["name"]));
        assert_eq!(third_party_json(&diff), row["value"]);
        let decoded = FlowDiff::from_value(diff.to_value()).unwrap();
        assert_eq!(decoded, diff);
        retire_diff(decoded);
        retire_diff(diff);
    }
    for row in vectors["invalid"].as_array().unwrap() {
        assert!(FlowDiff::from_value(semio_framework_value::DslValue::from(&row["value"])).is_err(), "{} (first-party)", row["name"]);
        if matches!(row["name"].as_str(), Some("missing-fragment-fields" | "unknown-fragment-field")) {
            assert!(serde_json::from_value::<FlowCollectionDelta<SynapseSpec>>(row["value"]["deltas"][0]["value"].clone()).is_err(), "{} (serde oracle)", row["name"]);
        }
    }
}
/// 📦️ A real flow document must be OPENABLE as an owned member of a composed document. Until
/// 2026-09-21 this declared `UnsupportedMemberSnapshotOpen`, whose `step` has exactly one answer —
/// `Rejected(MemberOpenDiagnostic::Decode)` — so a composed replacement or document archive carrying a
/// flow member was refused at member-open step 0, always. The declared opener is named here so a
/// regression back to the rejecting one fails loudly instead of silently refusing every flow member.
#[test]
fn flow_opens_as_an_owned_member_through_its_own_pack_codec() {
    assert_eq!(
        std::any::type_name::<<crate::io::binary::snapshot::FlowSnapshotPackProducer as crate::os_store::document_store_owners::MemberStoreProducer<FlowHostSnapshot,FlowMutation>>::SnapshotOpen>(),
        std::any::type_name::<crate::os_store::ProducedMemberSnapshotOpen<FlowHostSnapshot,crate::io::binary::snapshot::FlowSnapshotPackProducer>>(),
        "a flow member must open through ProducedMemberSnapshotOpen"
    );
    let snapshot = base();
    let encoded = pack::snapshot::ArtifactPack::encode_pack(&snapshot);
    let decoded = <crate::io::binary::snapshot::FlowSnapshotPackProducer as crate::os_store::MemberSnapshotDecoder<FlowHostSnapshot>>::decode_snapshot(&encoded).expect("the member opener's whole-pack decode");
    assert_eq!(decoded, snapshot, "the opener's decode round-trips the exact member snapshot");
    assert_eq!(third_party_json(&decoded),cases()["hostSnapshot"],"selected actual child decoder matches independent portable JSON");
    let mut cursor = semio_framework_value::retirement::RetireOwned::retirement(decoded);
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 0, maximum_release_bytes: 4_096, maximum_depth: 4_096 };
    assert!(matches!(cursor.close_step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, ..grant }), semio_framework_value::retirement::RetirementStep::BudgetExhausted), "zero items refuse before moving any owner");
    for turn in 0..1_000_000 {
        if owner.terminal_is_empty() { break; }
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: owner.next_capacity_byte_demand(4_096).expect("exact Flow frontier capacity demand"), maximum_release_bytes: owner.next_release_byte_demand().expect("exact Flow frontier release demand"), maximum_depth: owner.next_depth_demand().expect("exact Flow frontier depth demand") };
        assert_eq!(owner.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, ..grant }).expect("zero items refuse before moving any owner").progress().copied_items, 0);
        owner.step(grant).expect("the member opener's owner cursor accepts its exact quote");
        assert!(turn < 999_999, "the member opener's owner cursor never reached terminal-empty");
    }
    assert!(owner.terminal_is_empty(), "the member opener's owner cursor reached terminal-empty");
    snapshot.retire_cold();
}
//#endregion 🧪️Laws


#[test]
fn actual_flow_genesis_producer_preserves_original_stored_pack_and_owner_identity(){
 use ::semio_framework_os_kernel::os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
 let producer=crate::io::binary::snapshot::FlowSnapshotPackProducer;
 let born=AdmittedArtifactGenesis::born(base(),&producer).unwrap();
 assert_eq!(third_party_json(born.facts().snapshot()),cases()["hostSnapshot"]);
 let original=born.pack().to_vec();let pointer=original.as_ptr();let digest=born.facts().digest();
 let reloaded=AdmittedArtifactGenesis::from_stored_pack(original,&producer).unwrap();
 assert_eq!(reloaded.pack().as_ptr(),pointer);assert_eq!(reloaded.pack(),born.pack());assert_eq!(reloaded.facts().digest(),digest);
 assert_eq!(third_party_json(reloaded.facts().snapshot()),cases()["hostSnapshot"]);
 let alias=reloaded.clone();assert!(Arc::ptr_eq(&reloaded.facts().share_snapshot(),&alias.facts().share_snapshot()));assert!(Arc::ptr_eq(&reloaded.share_pack(),&alias.share_pack()));
 drop(alias);
 for genesis in [born,reloaded]{let(snapshot,pack)=genesis.into_owners();drop(pack);Arc::try_unwrap(snapshot).unwrap_or_else(|_|panic!("original genesis snapshot alias retained")).retire_cold();}
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn move_widget_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![(self::operation(2), self::base())];
    cases.extend([
        (FlowMutation::MoveWidget(MoveWidget { id: "a".into(), to_index: 2 }), self::base()),
        (FlowMutation::MoveWidget(MoveWidget { id: "b".into(), to_index: 0 }), self::base()),
        (FlowMutation::MoveWidget(MoveWidget { id: "b".into(), to_index: 2 }), self::base()),
        (FlowMutation::MoveWidget(MoveWidget { id: "a".into(), to_index: 1 }), self::base()),
        (FlowMutation::MoveWidget(MoveWidget { id: "c".into(), to_index: 1 }), self::base()),
    ]);
    for (mutation, before) in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        self::retire_mutation(mutation);
        before.retire_cold();
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn remove_synapse_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![(self::operation(5), self::base())];
    cases.extend([
        (FlowMutation::RemoveSynapse(RemoveSynapse { id: "s2".into() }), self::base_with_three_synapses()),
        (FlowMutation::RemoveSynapse(RemoveSynapse { id: "s3".into() }), self::base_with_three_synapses()),
    ]);
    for (mutation, before) in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        self::retire_mutation(mutation);
        before.retire_cold();
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn add_widget_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![(self::operation(0), self::base())];
    cases.extend([
        (FlowMutation::AddWidget(AddWidget { index: 0, widget: self::note("x") }), self::base()),
        (FlowMutation::AddWidget(AddWidget { index: 3, widget: self::note("x") }), self::base()),
    ]);
    for (mutation, before) in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        self::retire_mutation(mutation);
        before.retire_cold();
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn change_layout_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![(self::operation(8), self::base())];
    cases.extend([
        (FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "b".into(), layout: Some(crate::WidgetLayout { x: 5.0, y: 6.0 }) }, FlowLayoutEntry { id: "a".into(), layout: None }, FlowLayoutEntry { id: "c".into(), layout: Some(crate::WidgetLayout { x: 7.0, y: 8.0 }) }] }), self::base()),
        (FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "b".into(), layout: Some(crate::WidgetLayout { x: 1.0, y: 1.0 }) }, FlowLayoutEntry { id: "b".into(), layout: Some(crate::WidgetLayout { x: 2.0, y: 2.0 }) }] }), self::base()),
    ]);
    for (mutation, before) in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        self::retire_mutation(mutation);
        before.retire_cold();
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn move_synapse_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![(self::operation(6), self::base())];
    cases.extend([
        (FlowMutation::MoveSynapse(MoveSynapse { id: "s2".into(), to_index: 0 }), self::base_with_three_synapses()),
        (FlowMutation::MoveSynapse(MoveSynapse { id: "s2".into(), to_index: 2 }), self::base_with_three_synapses()),
        (FlowMutation::MoveSynapse(MoveSynapse { id: "s1".into(), to_index: 1 }), self::base_with_three_synapses()),
        (FlowMutation::MoveSynapse(MoveSynapse { id: "s3".into(), to_index: 0 }), self::base_with_three_synapses()),
        (FlowMutation::MoveSynapse(MoveSynapse { id: "s1".into(), to_index: 2 }), self::base_with_three_synapses()),
    ]);
    for (mutation, before) in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        self::retire_mutation(mutation);
        before.retire_cold();
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn change_synapse_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![(self::operation(7), self::base())];
    cases.extend([
        (FlowMutation::ChangeSynapse(ChangeSynapse { id: "s2".into(), synapse: self::synapse("s2", "c", "a") }), self::base_with_three_synapses()),
        (FlowMutation::ChangeSynapse(ChangeSynapse { id: "s3".into(), synapse: self::synapse("s3b", "c", "b") }), self::base_with_three_synapses()),
    ]);
    for (mutation, before) in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        self::retire_mutation(mutation);
        before.retire_cold();
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn add_synapse_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![(self::operation(4), self::base())];
    cases.extend([
        (FlowMutation::AddSynapse(AddSynapse { index: 0, synapse: self::synapse("s9", "b", "a") }), self::base_with_three_synapses()),
        (FlowMutation::AddSynapse(AddSynapse { index: 2, synapse: self::synapse("s9", "b", "a") }), self::base_with_three_synapses()),
        (FlowMutation::AddSynapse(AddSynapse { index: 3, synapse: self::synapse("s9", "b", "a") }), self::base_with_three_synapses()),
    ]);
    for (mutation, before) in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        self::retire_mutation(mutation);
        before.retire_cold();
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn remove_widget_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![(self::operation(1), self::base())];
    cases.extend([
        (FlowMutation::RemoveWidget(RemoveWidget { id: "a".into() }), self::base()),
        (FlowMutation::RemoveWidget(RemoveWidget { id: "c".into() }), self::base()),
    ]);
    for (mutation, before) in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        self::retire_mutation(mutation);
        before.retire_cold();
    }
}

/// ➕️ L3 on the committed wire witness and on first, middle and last row cases: the inverse's diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn change_widget_inverse_diffs_sum_to_the_negative_diff() {
    let mut cases = vec![(self::operation(3), self::base())];
    cases.extend([
        (FlowMutation::ChangeWidget(ChangeWidget { id: "b".into(), widget: self::note("b2") }), self::base()),
        (FlowMutation::ChangeWidget(ChangeWidget { id: "c".into(), widget: self::note("c2") }), self::base()),
    ]);
    for (mutation, before) in cases {
        crate::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        self::retire_mutation(mutation);
        before.retire_cold();
    }
}
