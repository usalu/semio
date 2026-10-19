use super::*;
use protocol::{Mutation, MutationDiff, MutationLeaf, OpBinary, OpText};

#[test]
fn local_interaction_mutation_leaf_descriptor_and_exact_codecs_are_owned() {
    use std::{io::Write,process::{Command,Stdio}};
    let source: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let descriptor: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
    assert_eq!(descriptor.as_object().unwrap().len(),14);
    assert_eq!(descriptor["textOpcode"],"set-interaction-state");
    assert_eq!(descriptor["binaryTag"],serde_json::Value::Null);
    assert!(descriptor["owner"].as_str().unwrap().ends_with("/🕹️interaction/🧬️mutations/🔁️set-state"));
    let mut child=Command::new("bun").args(["-e","import{produce}from'immer';const source=JSON.parse(await Bun.stdin.text());await Bun.write(Bun.stdout,JSON.stringify(produce(source,()=>{})));"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let bytes=serde_json::to_vec(&source).unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap());
    let state: InteractionState = serde_json::from_value(source.clone()).unwrap();
    let mutation = InteractionConfigMutation::set_state(state.clone());
    assert_eq!(::store::os_store::test_support::assert_wire_witness::<InteractionConfigMutation>(include_str!("../🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json")), mutation);
    assert_eq!(InteractionConfigMutation::DESCRIPTORS.len(), 1);
    assert_eq!(serde_json::Value::from(semio_framework_value::ToValue::to_value(mutation.descriptor())), descriptor);
    assert_eq!(mutation.descriptor(), &SetInteractionState::DESCRIPTOR);
    assert!(SetInteractionState::PROVENANCE.source_path.ends_with("/🔁️set-state/🦀️.rs"));
    assert_eq!(SetInteractionState::PROVENANCE.owner, mutation.descriptor().owner);
    let text = mutation.print_op();
    assert_eq!(serde_json::from_str::<serde_json::Value>(text.strip_prefix("set-interaction-state ").unwrap()).unwrap(), source);
    assert_eq!(InteractionConfigMutation::parse_op(&text).unwrap(), mutation);
    let binary = mutation.encode_op().unwrap();
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&binary).unwrap(), source);
    assert_eq!(InteractionConfigMutation::decode_op(&binary).unwrap(), mutation);
    assert_eq!(protocol::apply_diff_cold(mutation.diff(&InteractionState::default()).diff(), &InteractionState::default()).unwrap(), state);
    let inverse = mutation.inverse(&InteractionState::default()).expect("valid retained mutation inverse fixture");
    assert_eq!(protocol::apply_diff_cold(inverse[0].diff(&state).diff(), &state).unwrap(), InteractionState::default());
}

#[test]
fn the_interaction_mutation_round_trips_its_serde_json_document_through_the_plain_and_the_controlled_decoder() {
    use semio_framework_value::{FromValue, ToValue};
    let source: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let state: InteractionState = serde_json::from_value(source).unwrap();
    for mutation in [InteractionConfigMutation::set_state(InteractionState::default()), InteractionConfigMutation::set_state(state)] {
        let document = serde_json::to_string(&mutation.to_value()).expect("independent serde_json document");
        let parsed: semio_framework_value::DslValue = serde_json::from_str(&document).expect("the document parses back through an independent parser");
        assert_eq!(InteractionConfigMutation::from_value(parsed.clone()).expect("plain decode"), mutation);
        let mut allow = |_: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| true;
        let mut control = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1 << 20, &mut allow);
        let decoded = InteractionConfigMutation::from_value_controlled(&parsed, &mut control).expect("controlled decode");
        assert_eq!(decoded, mutation);
        decoded.retire_decoded();
    }
}

/// ➕️ L3: the replacement state's inverse diffs sum to the negative diff, from the default and from a populated state.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let source: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let populated: InteractionState = serde_json::from_value(source).unwrap();
    for (base, next) in [(InteractionState::default(), populated.clone()), (populated, InteractionState::default())] {
        semio_framework_os_kernel::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&InteractionConfigMutation::set_state(next), &base).await;
    }
}
