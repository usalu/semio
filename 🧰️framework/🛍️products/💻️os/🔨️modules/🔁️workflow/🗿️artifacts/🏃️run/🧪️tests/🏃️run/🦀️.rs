//! 🧪️ Run package serialization, admission, and replay laws.
use crate::*;
use semio_framework_dsl_record::DslField as _;
fn assert_run_manual_binding<T:semio_framework_dsl_record::DslField+PartialEq+std::fmt::Debug>(value:&T,maximum:usize,tiny:usize){
 let expected=T::to_value(value);let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(maximum,&mut admitted);let encoded=value.to_value_controlled(&mut control).expect("owned run field projection");let exact=control.owned_bytes();assert_eq!(encoded,expected);assert!(exact>0);assert!(value.to_value_controlled(&mut semio_framework_value::NativeEncodeControl::new(exact,&mut |_|true)).is_ok());assert!(value.to_value_controlled(&mut semio_framework_value::NativeEncodeControl::new(exact-1,&mut |_|true)).is_err());
 let mut admitted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(maximum,&mut admitted);assert_eq!(&T::from_value_controlled(&encoded,&mut control).expect("owned run field construction"),value);let decoded_exact=control.owned_bytes();assert!(T::from_value_controlled(&encoded,&mut semio_framework_value::NativeDecodeControl::new(decoded_exact,&mut |_|true)).is_ok());if decoded_exact>0{assert!(T::from_value_controlled(&encoded,&mut semio_framework_value::NativeDecodeControl::new(decoded_exact-1,&mut |_|true)).is_err());}
 let semio_framework_dsl_record::FieldValue::Record(record)=&encoded else{panic!("explicit run record")};assert_eq!(value.to_record_controlled(&mut semio_framework_value::NativeEncodeControl::new(maximum,&mut |_|true)).unwrap(),*record);assert_eq!(&T::from_record_controlled(record,&mut semio_framework_value::NativeDecodeControl::new(maximum,&mut |_|true)).unwrap(),value);
 let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(tiny,&mut admitted);assert!(value.to_value_controlled(&mut control).is_err());assert_eq!(control.owned_bytes(),0);if decoded_exact>tiny{assert!(T::from_value_controlled(&encoded,&mut semio_framework_value::NativeDecodeControl::new(tiny,&mut |_|true)).is_err());}assert!(value.to_value_controlled(&mut semio_framework_value::NativeEncodeControl::new(maximum,&mut |_|false)).is_err());assert!(T::from_value_controlled(&encoded,&mut semio_framework_value::NativeDecodeControl::new(maximum,&mut |_|false)).is_err());
}
#[test]
fn sqlite_snapshot_run_trigger_binds_literal_values_and_cancels_owned_utf8(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;let tiny=fixture["tinyBytes"].as_u64().unwrap()as usize;
 for expected in fixture["triggers"].as_array().unwrap(){let value:RunTrigger=semio_framework_pack_json::from_json_str(&serde_json::to_string(expected).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&value)).unwrap(),*expected);assert_run_manual_binding(&value,maximum,tiny);}
 let text="😀".repeat(fixture["largeCharacters"].as_u64().unwrap()as usize);let value=RunTrigger::Automation{automation_ref:text.clone(),event_fingerprint:text.clone()};let raw=<RunTrigger as semio_framework_dsl_record::DslField>::to_value(&value);let threshold=fixture["cancelAfterBytes"].as_u64().unwrap()as usize;
 let mut observed=false;let mut cancel=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{if event.total==text.len()&&event.completed>=threshold{observed=true;false}else{true}};assert!(value.to_value_controlled(&mut semio_framework_value::NativeEncodeControl::new(maximum,&mut cancel)).is_err());assert!(observed);
 let mut observed=false;let mut cancel=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if event.total==text.len()&&event.completed>=threshold{observed=true;false}else{true}};assert!(<RunTrigger as semio_framework_dsl_record::DslField>::from_value_controlled(&raw,&mut semio_framework_value::NativeDecodeControl::new(maximum,&mut cancel)).is_err());assert!(observed);
}
use protocol::MutationDiff;

fn apply_run_operation(document: &RunArtifact, operation: &RunMutation) -> RunArtifact {
    protocol::apply_diff(protocol::Mutation::diff(operation, document).diff(), document).expect("a valid run operation applies")
}
fn assert_run_native_status<T:semio_framework_dsl_record::DslField+PartialEq+std::fmt::Debug>(value:&T,index:u32,invalid:u32){
 let mut admitted=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(0,&mut admitted);let native=value.to_value_controlled(&mut encoding).expect("owned declared status ordinal");assert_eq!(native,semio_framework_dsl_record::FieldValue::Enum(index));assert_eq!(encoding.owned_bytes(),0);
 let mut admitted=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(0,&mut admitted);assert_eq!(&T::from_value_controlled(&native,&mut decoding).unwrap(),value);assert_eq!(decoding.owned_bytes(),0);assert!(T::from_value_controlled(&semio_framework_dsl_record::FieldValue::Enum(invalid),&mut semio_framework_value::NativeDecodeControl::new(0,&mut |_|true)).is_err());assert!(value.to_value_controlled(&mut semio_framework_value::NativeEncodeControl::new(0,&mut |_|false)).is_err());assert!(T::from_value_controlled(&native,&mut semio_framework_value::NativeDecodeControl::new(0,&mut |_|false)).is_err());
}
#[test]
fn sqlite_snapshot_run_status_binds_every_declared_ordinal_without_allocation(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();let invalid=fixture["invalidOrdinal"].as_u64().unwrap()as u32;for(index,word)in fixture["statuses"].as_array().unwrap().iter().enumerate(){let value:RunStatus=semio_framework_pack_json::from_json_str(&serde_json::to_string(word).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&value)).unwrap(),*word);assert_run_native_status(&value,index as u32,invalid);}
}
#[test]
fn sqlite_snapshot_run_node_status_binds_every_declared_ordinal_without_allocation(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();let invalid=fixture["invalidOrdinal"].as_u64().unwrap()as u32;for(index,word)in fixture["nodeStatuses"].as_array().unwrap().iter().enumerate(){let value:RunNodeStatus=semio_framework_pack_json::from_json_str(&serde_json::to_string(word).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&value)).unwrap(),*word);assert_run_native_status(&value,index as u32,invalid);}
}
#[test]
fn sqlite_snapshot_run_manual_metadata_preserves_neutral_trigger_fields_and_statuses(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏭️schema/🔣️.json")).unwrap();let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;let tiny=fixture["tinyBytes"].as_u64().unwrap()as usize;let semio_framework_dsl_record::Shape::Record(producer)=<RunTrigger as semio_framework_dsl_record::DslField>::shape()else{panic!("declared trigger record")};
 let mut admitted=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(maximum,&mut admitted);let encoded=producer.encode(&mut encoding).unwrap();let exact=encoding.owned_bytes();let mut admitted=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(maximum,&mut admitted);let decoded=producer.decode(&mut decoding).unwrap();for record in[encoded,decoded]{let fields:Vec<_>=record.fields.iter().map(|field|serde_json::json!([field.id,field.key,field.optional])).collect();assert_eq!(serde_json::json!(fields),fixture["fields"]);}
 assert!(exact>0);assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(exact,&mut |_|true)).is_ok());assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(exact-1,&mut |_|true)).is_err());let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(tiny,&mut admitted);assert!(producer.encode(&mut control).is_err());assert_eq!(control.owned_bytes(),0);assert!(producer.decode(&mut semio_framework_value::NativeDecodeControl::new(tiny,&mut |_|true)).is_err());assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(maximum,&mut |_|false)).is_err());assert!(producer.decode(&mut semio_framework_value::NativeDecodeControl::new(maximum,&mut |_|false)).is_err());
 for index in 0..2{
  let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(maximum,&mut admitted);let shape=if index==0{<RunStatus as semio_framework_dsl_record::DslField>::shape_controlled(&mut control)}else{<RunNodeStatus as semio_framework_dsl_record::DslField>::shape_controlled(&mut control)}.unwrap();let semio_framework_dsl_record::Shape::Enum(labels)=shape else{panic!("declared status labels")};assert_eq!(serde_json::json!(labels),fixture["statuses"][index]);
  let mut admitted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(maximum,&mut admitted);let shape=if index==0{<RunStatus as semio_framework_dsl_record::DslField>::shape_controlled(&mut control)}else{<RunNodeStatus as semio_framework_dsl_record::DslField>::shape_controlled(&mut control)}.unwrap();let semio_framework_dsl_record::Shape::Enum(labels)=shape else{panic!("declared status labels")};assert_eq!(serde_json::json!(labels),fixture["statuses"][index]);let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(tiny,&mut admitted);assert!(if index==0{<RunStatus as semio_framework_dsl_record::DslField>::shape_controlled(&mut control)}else{<RunNodeStatus as semio_framework_dsl_record::DslField>::shape_controlled(&mut control)}.is_err());assert_eq!(control.owned_bytes(),0);
 }
}

/// 🧾️ Every committed wire witness decodes through `RunMutation`'s `FromValue` and re-encodes to exactly the committed JSON.
#[test]
fn committed_wire_witnesses_are_the_canonical_wire() {
    for witness in [
        include_str!("../../🧫️fixtures/🧬️mutations/🚀️start-run/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../🧫️fixtures/🧬️mutations/▶️start-run-node/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../🧫️fixtures/🧬️mutations/✅️finish-run-node/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../🧫️fixtures/🧬️mutations/🪵️append-run-log/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../🧫️fixtures/🧬️mutations/🔏️seal-run/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../🧫️fixtures/🧬️mutations/🧷️set-run-header/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../🧫️fixtures/🧬️mutations/🔓️set-run-seal/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../🧫️fixtures/🧬️mutations/🧽️remove-run-log/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../🧫️fixtures/🧬️mutations/🫥️remove-run-node/🧾️wire-witness/🦠️mutation/🔣️.json"),
    ] {
        store::os_store::test_support::assert_wire_witness::<RunMutation>(witness);
    }
}

async fn sample_run_node_record(node_id: &str, status: RunNodeStatus) -> RunNodeRecord {
    RunNodeRecord {
        node_id: node_id.into(),
        status,
        document_fingerprint: "doc-fp".into(),
        config_fingerprint: "cfg-fp".into(),
        input_fingerprints: vec![PortFingerprint { port_id: format!("{node_id}:in:in"), fingerprint: "in-fp".into() }],
        output_fingerprints: vec![PortFingerprint { port_id: format!("{node_id}:out:out"), fingerprint: "out-fp".into() }],
        outputs: vec![RunOutputArtifact { port_id: format!("{node_id}:out:out"), artifact_id: format!("artifacts/{node_id}"), path: format!("out/{node_id}.out") }],
        duration_ms: 12.5,
    }
}

async fn sample_run_document() -> RunArtifact {
    let mut document = empty_run_document().await;
    document = apply_run_operation(
        &document,
        &RunMutation::StartRun(StartRun {
            workflow_ref: "space.space".into(),
            workflow_checkpoint_id: "ck-1".into(),
            input_collection_ref: "collections/in".into(),
            input_snapshot_id: "snap-1".into(),
            parameter_values: vec![RunParameterValue { parameter_id: "p1".into(), value: "10".into() }],
            output_collection_ref: "collections/out".into(),
            trigger: RunTrigger::Manual { actor: "dev".into() },
        }),
    );
    document = apply_run_operation(&document, &RunMutation::StartRunNode(StartRunNode { node_id: "a".into() }));
    document = apply_run_operation(&document, &RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("a", RunNodeStatus::Computed).await }));
    document
}

#[semio_framework_async_macros::async_test]
async fn empty_run_document_matches_schema() {
    let document = empty_run_document().await;
    assert_eq!(document.schema, S_RUN_SCHEMA);
    assert_eq!(document.status, RunStatus::Pending);
    assert!(!document.sealed);
    assert!(document.node_records.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn run_document_dsl_pack_round_trips() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&sample_run_document().await);
    store::os_store::test_support::assert_dsl_pack_equivalence(&empty_run_document().await);
}

#[semio_framework_async_macros::async_test]
async fn run_operation_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::StartRun(StartRun {
        workflow_ref: "space.space".into(),
        workflow_checkpoint_id: "ck-1".into(),
        input_collection_ref: "collections/in".into(),
        input_snapshot_id: "snap-1".into(),
        parameter_values: vec![RunParameterValue { parameter_id: "p1".into(), value: "10".into() }],
        output_collection_ref: "collections/out".into(),
        trigger: RunTrigger::Manual { actor: "dev".into() },
    }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::StartRun(StartRun {
        workflow_ref: "space.space".into(),
        workflow_checkpoint_id: "ck-1".into(),
        input_collection_ref: "collections/in".into(),
        input_snapshot_id: "snap-1".into(),
        parameter_values: Vec::new(),
        output_collection_ref: "collections/out".into(),
        trigger: RunTrigger::Automation { automation_ref: "os.automation/a1".into(), event_fingerprint: "evt-1".into() },
    }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::StartRunNode(StartRunNode { node_id: "a".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("a", RunNodeStatus::CacheHit).await }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::AppendRunLog(AppendRunLog { node_id: "a".into(), level: "info".into(), message: "computed".into(), at: "123".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::SealRun(SealRun { status: RunStatus::Succeeded }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::SetRunHeader(SetRunHeader {
        workflow_ref: "space.space".into(),
        workflow_checkpoint_id: "ck-1".into(),
        input_collection_ref: "collections/in".into(),
        input_snapshot_id: "snap-1".into(),
        parameter_values: Vec::new(),
        output_collection_ref: "collections/out".into(),
        trigger: RunTrigger::Manual { actor: "dev".into() },
        status: RunStatus::Pending,
        started_at: String::new(),
    }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::SetRunSeal(SetRunSeal { sealed: true, status: RunStatus::Succeeded, finished_at: Some("2026-09-30T12:00:00Z".into()) }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::RemoveRunLog(RemoveRunLog { count: 2 }));
    store::os_store::test_support::assert_op_line_round_trip(&RunMutation::RemoveRunNode(RemoveRunNode { node_id: "a".into() }));
}

#[test]
fn run_payload_json_uses_exact_camel_case_and_rejects_unknown_fields() {
    let automation = RunTrigger::Automation { automation_ref: "automation".into(), event_fingerprint: "event".into() };
    let encoded = semio_framework_pack_json::parse(&semio_framework_pack_json::to_json_string(&automation), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("RunTrigger JSON");
    let expected = semio_framework_pack_json::parse(r#"{"kind":"automation","automationRef":"automation","eventFingerprint":"event"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("expected JSON");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&encoded, &expected));
    assert!(semio_framework_pack_json::from_json_str::<RunTrigger>(r#"{"kind":"automation","automation_ref":"automation","event_fingerprint":"event"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    assert!(semio_framework_pack_json::from_json_str::<RunTrigger>(r#"{"kind":"manual","actor":"operator","extra":true}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    assert!(semio_framework_pack_json::from_json_str::<RunNodeRecord>(
        r#"{"nodeId":"node","status":"computed","documentFingerprint":"document","configFingerprint":"config","inputFingerprints":[],"outputFingerprints":[],"outputs":[],"durationMs":1.0,"extra":true}"#
    , semio_framework_pack_json::JsonMemberPolicy::Reject)
    .is_err());
}

#[semio_framework_async_macros::async_test]
async fn checked_run_admission_matches_the_typed_diff_rejection() {
    let start = RunMutation::StartRun(StartRun {
        workflow_ref: "space.space".into(),
        workflow_checkpoint_id: "checkpoint".into(),
        input_collection_ref: "collections/in".into(),
        input_snapshot_id: "snapshot".into(),
        parameter_values: Vec::new(),
        output_collection_ref: "collections/out".into(),
        trigger: RunTrigger::Manual { actor: "operator".into() },
    });
    let started = apply_run_operation_checked(&empty_run_document().await, start.clone()).await.expect("first start applies");
    let outcome = protocol::Mutation::diff(&start, &started);
    let rejection = protocol::apply_diff(outcome.diff(), &started).expect_err("the direct diff rejects a second start");
    let mut expected = outcome.messages().to_vec();
    expected.push(protocol::MutationMessage::fatal(rejection.code, rejection.message).at(rejection.target));
    let actual = apply_run_operation_checked(&started, start).await.expect_err("checked admission rejects the same second start");
    assert_eq!(actual, expected, "the checked seam reports exactly what the store would persist");
    assert_eq!(actual.iter().map(|message| (message.code.0.as_str(), message.level)).collect::<Vec<_>>(), [("mutation.apply.conflicting-target", semio_framework_diagnostic::Severity::Fatal)]);
    assert_eq!(actual[0].target, vec!["status"]);
    assert!(actual.iter().all(|message| protocol::outcome_code_level(&message.code.0) == Some(message.level)), "{actual:?}");
}

/// 🔒️ The load-bearing law this wave exists to prove: once `Seal` has been applied, every further
/// operation is rejected by `apply_run_operation_checked` (not silently accepted, not a panic) —
/// this is the real write seam `run::SpaceRunner` goes through for every `RunMutation` it emits.
#[semio_framework_async_macros::async_test]
async fn apply_run_operation_checked_rejects_everything_after_seal() {
    let document = sample_run_document().await;
    assert!(!document.sealed);

    let sealed = apply_run_operation_checked(&document, RunMutation::SealRun(SealRun { status: RunStatus::Succeeded })).await.expect("sealing an unsealed run must succeed");
    assert!(sealed.sealed);
    assert_eq!(sealed.status, RunStatus::Succeeded);
    assert!(sealed.finished_at.is_some());

    let rejected_log = apply_run_operation_checked(&sealed, RunMutation::AppendRunLog(AppendRunLog { node_id: "a".into(), level: "info".into(), message: "too late".into(), at: "999".into() }));
    assert!(rejected_log.await.is_err(), "a Log after Seal must be rejected, not silently applied");

    let rejected_node_finished = apply_run_operation_checked(&sealed, RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("b", RunNodeStatus::Computed).await }));
    assert!(rejected_node_finished.await.is_err(), "a NodeFinished after Seal must be rejected");

    let rejected_reseal = apply_run_operation_checked(&sealed, RunMutation::SealRun(SealRun { status: RunStatus::Failed }));
    assert!(rejected_reseal.await.is_err(), "re-sealing an already-sealed run must be rejected");

    assert_eq!(sealed.node_records.len(), 1, "the rejected NodeFinished must not have been applied");
}

#[semio_framework_async_macros::async_test]
async fn run_diff_absorb_preserves_each_append_in_order() {
    let document = empty_run_document().await;
    let first = RunMutation::AppendRunLog(AppendRunLog { node_id: String::new(), level: "info".into(), message: "first".into(), at: "1".into() });
    let first_diff = protocol::Mutation::diff(&first, &document).diff().clone();
    let middle = protocol::apply_diff(&first_diff, &document).expect("first append applies");
    let second = RunMutation::AppendRunLog(AppendRunLog { node_id: String::new(), level: "info".into(), message: "second".into(), at: "2".into() });
    let mut combined = first_diff;
    combined.absorb(protocol::Mutation::diff(&second, &middle).diff().clone());
    let after = protocol::apply_diff(&combined, &document).expect("combined appends apply");
    assert_eq!(after.logs.iter().map(|line| line.message.as_str()).collect::<Vec<_>>(), vec!["first", "second"]);
}

#[semio_framework_async_macros::async_test]
async fn run_diff_absorb_preserves_start_before_later_log() {
    let document = empty_run_document().await;
    let start = RunMutation::StartRun(StartRun {
        workflow_ref: "workflow-selected".into(),
        workflow_checkpoint_id: "checkpoint".into(),
        input_collection_ref: "inputs".into(),
        input_snapshot_id: "snapshot".into(),
        parameter_values: Vec::new(),
        output_collection_ref: "outputs".into(),
        trigger: RunTrigger::Manual { actor: "operator".into() },
    });
    let start_diff = protocol::Mutation::diff(&start, &document).diff().clone();
    let middle = protocol::apply_diff(&start_diff, &document).expect("start applies");
    let append = RunMutation::AppendRunLog(AppendRunLog { node_id: String::new(), level: "info".into(), message: "started".into(), at: "1".into() });
    let mut combined = start_diff;
    combined.absorb(protocol::Mutation::diff(&append, &middle).diff().clone());
    let after = protocol::apply_diff(&combined, &document).expect("combined start and append apply");
    assert_eq!(after.workflow_ref, "workflow-selected");
    assert_eq!(after.status, RunStatus::Running);
    assert_eq!(after.logs.iter().map(|line| line.message.as_str()).collect::<Vec<_>>(), vec!["started"]);
}

#[semio_framework_async_macros::async_test]
async fn run_diff_absorb_is_associative_with_empty_identity() {
    let document = empty_run_document().await;
    let log = |message: &str| RunDiff::step(RunStep::LogAppend(RunLogLine { node_id: String::new(), level: "info".into(), message: message.into(), at: message.into() }));
    let first = log("first");
    let second = log("second");
    let third = log("third");
    let mut left = first.clone();
    left.absorb(second.clone());
    left.absorb(third.clone());
    let mut suffix = second;
    suffix.absorb(third);
    let mut right = first.clone();
    right.absorb(suffix);
    let mut leading_identity = RunDiff::default();
    leading_identity.absorb(first.clone());
    let mut trailing_identity = first.clone();
    trailing_identity.absorb(RunDiff::default());
    assert_eq!(left, right);
    assert_eq!(leading_identity, first);
    assert_eq!(trailing_identity, first);
    assert_eq!(protocol::apply_diff(&left, &document), protocol::apply_diff(&right, &document));
}

#[semio_framework_async_macros::async_test]
async fn run_diff_sequence_rejects_later_steps_without_mutating_the_base() {
    let document = empty_run_document().await;
    let mut seal_then_log = RunDiff::step(RunStep::Seal(RunSealEdit { sealed: true, status: RunStatus::Succeeded, finished_at: Some("1".into()) }));
    seal_then_log.absorb(RunDiff::step(RunStep::LogAppend(RunLogLine { node_id: String::new(), level: "info".into(), message: "late".into(), at: "1".into() })));
    let sealed_error = protocol::apply_diff(&seal_then_log, &document).expect_err("log after a composed seal rejects");
    assert_eq!(sealed_error.code, "mutation.apply.sealed");
    assert_eq!(sealed_error.target, vec!["sealed"]);
    assert_eq!(document, empty_run_document().await);

    let first_start = RunDiff::step(RunStep::Header(RunHeaderEdit {
        workflow_ref: "workflow".into(),
        workflow_checkpoint_id: "checkpoint".into(),
        input_collection_ref: "inputs".into(),
        input_snapshot_id: "snapshot".into(),
        parameter_values: Vec::new(),
        output_collection_ref: "outputs".into(),
        trigger: RunTrigger::Manual { actor: "operator".into() },
        status: RunStatus::Running,
        started_at: "1".into(),
    }));
    let mut double_start = first_start.clone();
    double_start.absorb(first_start);
    let start_error = protocol::apply_diff(&double_start, &document).expect_err("second composed start rejects");
    assert_eq!(start_error.code, "mutation.apply.conflicting-target");
    assert_eq!(start_error.target, vec!["status"]);
    assert_eq!(document, empty_run_document().await);
}

#[semio_framework_async_macros::async_test]
async fn finish_run_node_replacement_inverse_restores_the_original_node_order() {
    let mut document = empty_run_document().await;
    document.node_records = vec![sample_run_node_record("a", RunNodeStatus::CacheHit).await, sample_run_node_record("b", RunNodeStatus::CacheHit).await, sample_run_node_record("c", RunNodeStatus::CacheHit).await];
    let operation = RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("b", RunNodeStatus::Computed).await });
    let inverse = protocol::Mutation::inverse(&operation, &document).expect("valid retained mutation inverse fixture");
    let mut restored = apply_run_operation(&document, &operation);
    assert_eq!(restored.node_records.iter().map(|record| record.node_id.as_str()).collect::<Vec<_>>(), vec!["a", "b", "c"]);
    for step in inverse.iter().rev() {
        restored = apply_run_operation(&restored, step);
    }
    assert_eq!(restored, document);
}

/// ➕️ L3 for every run kind: the concrete inverse's diffs sum to the negative of the forward diff.
#[semio_framework_async_macros::async_test]
async fn every_run_kind_inverse_diffs_sum_to_the_negative_diff() {
    use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law as law;
    let fresh = empty_run_document().await;
    law(
        &RunMutation::StartRun(StartRun {
            workflow_ref: "space.space".into(),
            workflow_checkpoint_id: "ck-1".into(),
            input_collection_ref: "collections/in".into(),
            input_snapshot_id: "snap-1".into(),
            parameter_values: vec![RunParameterValue { parameter_id: "p1".into(), value: "10".into() }],
            output_collection_ref: "collections/out".into(),
            trigger: RunTrigger::Manual { actor: "dev".into() },
        }),
        &fresh,
    )
    .await;
    let started = sample_run_document().await;
    law(&RunMutation::StartRunNode(StartRunNode { node_id: "b".into() }), &started).await;
    law(&RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("b", RunNodeStatus::Computed).await }), &started).await;
    law(&RunMutation::FinishRunNode(FinishRunNode { node_record: sample_run_node_record("a", RunNodeStatus::CacheHit).await }), &started).await;
    law(&RunMutation::AppendRunLog(AppendRunLog { node_id: "a".into(), level: "info".into(), message: "computed".into(), at: "2".into() }), &started).await;
    law(&RunMutation::SealRun(SealRun { status: RunStatus::Succeeded }), &started).await;
    let sealed = apply_run_operation(&started, &RunMutation::SealRun(SealRun { status: RunStatus::Succeeded }));
    law(&RunMutation::SetRunSeal(SetRunSeal { sealed: false, status: RunStatus::Running, finished_at: None }), &sealed).await;
    law(&RunMutation::SetRunHeader(SetRunHeader { status: RunStatus::Pending, started_at: String::new(), ..set_run_header_of(&started) }), &started).await;
    law(&RunMutation::RemoveRunLog(RemoveRunLog { count: 1 }), &started).await;
    law(&RunMutation::RemoveRunNode(RemoveRunNode { node_id: "a".into() }), &started).await;
}

fn set_run_header_of(document: &RunArtifact) -> SetRunHeader {
    SetRunHeader {
        workflow_ref: document.workflow_ref.clone(),
        workflow_checkpoint_id: document.workflow_checkpoint_id.clone(),
        input_collection_ref: document.input_collection_ref.clone(),
        input_snapshot_id: document.input_snapshot_id.clone(),
        parameter_values: document.parameter_values.clone(),
        output_collection_ref: document.output_collection_ref.clone(),
        trigger: document.trigger.clone(),
        status: document.status,
        started_at: document.started_at.clone(),
    }
}

#[semio_framework_async_macros::async_test]
async fn run_node_record_dsl_pack_round_trips_nested_tables() {
    let record = sample_run_node_record("a", RunNodeStatus::Failed).await;
    let mut document = empty_run_document().await;
    document.node_records.push(record);
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
}

#[semio_framework_async_macros::async_test]
async fn language_neutral_package_cases_match_serde_json() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️package-contract/📜️cases.json")).expect("language-neutral fixture");
    assert_eq!(fixture["package"], env!("CARGO_PKG_NAME"));
    for case in fixture["cases"].as_array().expect("cases") {
        let mut document = empty_run_document().await;
        let mut rejections = Vec::new();
        for value in case["operations"].as_array().expect("operations") {
            let encoded = serde_json::to_string(value).expect("third-party JSON oracle");
            let operation: RunMutation = semio_framework_pack_json::from_json_str(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("domain operation JSON");
            let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&operation)).expect("domain JSON output");
            assert_eq!(&actual, value);
            match apply_run_operation_checked(&document, operation).await {
                Ok(next) => document = next,
                Err(messages) => rejections.extend(messages.into_iter().map(|message| message.code.0)),
            }
        }
        let status: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&document.status)).expect("status JSON");
        let actual = serde_json::json!({"schema":document.schema,"status":status,"sealed":document.sealed,"logs":document.logs.iter().map(|line| &line.message).collect::<Vec<_>>(),"rejections":rejections});
        assert_eq!(actual, case["expected"], "{}", case["name"]);
    }
}

/// ↩️ A composed diff's inverse reads each slot's prior row from the base or from an earlier row of the same diff, never from
/// an applied copy: undoing the sealed result returns to the base.
#[semio_framework_async_macros::async_test]
async fn composed_run_diff_inverse_reads_rows_and_restores_the_base() {
    use protocol::DiffAlgebra as _;
    let mut base = sample_run_document().await;
    let line = |message: &str| RunLogLine { node_id: "a".into(), level: "info".into(), message: message.into(), at: "3".into() };
    base.logs = vec![line("one"), line("two")];
    let diff = RunDiff {
        steps: vec![
            RunStep::LogAppend(line("three")),
            RunStep::LogRetract { count: 2 },
            RunStep::Node { node_id: "b".into(), record: Some(sample_run_node_record("b", RunNodeStatus::Computed).await) },
            RunStep::Node { node_id: "b".into(), record: None },
            RunStep::Seal(RunSealEdit { sealed: true, status: RunStatus::Succeeded, finished_at: Some("9".into()) }),
        ],
    };
    let after = protocol::apply_diff(&diff, &base).expect("composed run diff applies");
    assert!(after.sealed);
    let inverse = protocol::DiffAlgebra::<RunArtifact>::inverse(&diff, &base);
    let restored = protocol::apply_diff(&inverse, &after).expect("the row-read inverse applies");
    assert_eq!(restored, base);
}
