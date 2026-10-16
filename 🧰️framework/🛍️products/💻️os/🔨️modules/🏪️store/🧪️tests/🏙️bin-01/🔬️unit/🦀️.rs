//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 1 of the cut by subject).

use super::*;

#[test]
fn original_catalog_constructor_stages_retain_original_sources_and_conserve_native_heap(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🔌️plugin/⏪️time-travel/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let policy=&corpus["constructorGrant"];let n=|key:&str|policy[key].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 let (admitted,source_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bounded_artifact_store_owners::<DemoSnapshot,DemoMutation>(grant));let(mut owners,source_receipt)=admitted.unwrap_or_else(|_|panic!("independent constructor authority admits original sources"));assert!(source_receipt.fits(grant));assert_eq!(source_heap.requested_bytes,source_receipt.retained_capacity_bytes);assert_eq!(source_heap.released_bytes,source_receipt.released_bytes);let mut births=source_heap.requested_bytes;let mut releases=source_heap.released_bytes;let mut turns=0;
 while !owners.constructor_is_complete(){assert!(turns<512,"original constructor must progress under unchanged independent authority");let pointers=||[owners.snapshot_retirement.0.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize),owners.initial_snapshot_retirement.0.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize),owners.mutation_retirement.0.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize)];let original=pointers();
  for axis in 0..5{let denied=match axis{0=>RetainedCloneGrant{maximum_items:0,..grant},1=>RetainedCloneGrant{maximum_copy_bytes:0,..grant},2=>RetainedCloneGrant{maximum_capacity_bytes:0,..grant},3=>RetainedCloneGrant{maximum_release_bytes:0,..grant},_=>RetainedCloneGrant{maximum_depth:0,..grant}};let demand=owners.constructor_demands(grant.maximum_copy_bytes).unwrap();if axis==1&&demand.copy_bytes==0||axis==2&&demand.capacity_bytes==0||axis==3&&demand.release_bytes==0{continue}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.admit_constructor(denied));assert_eq!(heap.requested_bytes,0,"denied original constructor axis {axis}");assert_eq!(heap.released_bytes,0,"denied original constructor axis {axis}");let receipt=match result{Ok(receipt)=>receipt,Err((_,receipt))=>receipt};assert_eq!(receipt,RetainedCloneProgress::default());let current=[owners.snapshot_retirement.0.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize),owners.initial_snapshot_retirement.0.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize),owners.mutation_retirement.0.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize)];assert_eq!(current,original);}
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.admit_constructor(grant));let receipt=result.unwrap_or_else(|_|panic!("unchanged independent constructor policy must admit next original stage"));assert!(receipt.fits(grant));assert!(receipt.copied_items>0);assert_eq!(heap.requested_bytes,receipt.retained_capacity_bytes);assert_eq!(heap.released_bytes,receipt.released_bytes);births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;
 }
 while !owners.uninstalled_owners_terminal_is_empty(){assert!(turns<4096,"original uninstalled catalog must close under independent authority");let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.close_uninstalled_owners_step(grant));let step=step.unwrap();let receipt=step.progress();assert!(receipt.fits(grant));assert!(receipt.copied_items>0||matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(heap.requested_bytes,receipt.retained_capacity_bytes);assert_eq!(heap.released_bytes,receipt.released_bytes);births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;}
 let(_,terminal)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owners));assert_eq!(terminal.requested_bytes,0);assert_eq!(terminal.released_bytes,0);assert_eq!(births,releases);println!("[DEBUG] Original catalog source/ticket stages turns={turns} births={births} releases={releases} terminalDrop=0 fixedGrant={grant:?}");
}

#[semio_framework_async_macros::async_test]
async fn original_member_group_stage_and_adoption_preserve_physical_receipts_and_visibility(){
 fn original_heap<T>(action:impl FnOnce()->T)->(T,(usize,usize)){let(value,requested,released)=semio_framework_trace_memory_testing::observe_backing(action);(value,(requested,released))}
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧩️composition/📬️publication/🤝️group/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let policy=&fixture["grant"];let n=|axis:&str|policy[axis].as_u64().unwrap()as usize;
 let grant=ArtifactStoreOneItemGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 let before=fixture["before"].as_i64().unwrap()as i32;let after=fixture["after"].as_i64().unwrap()as i32;
 let mut store=ArtifactStore::new(create_document_envelope::<DemoSnapshot,DemoMutation>("demo/v1","original-group-receipts",DemoSnapshot{n:Some(before)},None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
 let mutation=DemoMutation::SetN(SetN{n:after});
 let mut publication=store.begin_member_apply_batch(semio_framework_job::OperationId(919),store.generation(),store.content_revision_now(),fixture["actor"].as_str().unwrap().into(),vec![mutation.clone()]).unwrap_or_else(|_|panic!("original installed member factory admits its typed fixture"));
 let authority=publication.authority.as_ref().unwrap();
 let mut stage=semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot,DemoMutation>::empty_batch_stage(authority);stage.edit=Some(Box::new(semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot,DemoMutation>::empty_batch_edit(authority)));
 let id=fixture["edit"].as_str().unwrap();stage.applied_edit_id=id.into();stage.tail_edit_id=id.into();stage.post=Some(Arc::new(DemoSnapshot{n:Some(after)}));stage.local_actor=Some(authority.actor().into());stage.folded=1;stage.completed_items=1;stage.digest=[7;32];
 stage.mutation_retirement=(*store.owners.mutation_retirement_factory).clone();stage.snapshot_retirement=(*store.owners.snapshot_retirement_factory).clone();stage.unit_flags.reserve_exact(2);
 let edit=stage.edit.as_mut().unwrap();edit.id=id.into();edit.actor=Some(authority.actor_utf8().clone());edit.forwards.push(mutation);edit.inverse.push(DemoMutation::SetN(SetN{n:before}));edit.mutation_meta.push(MutationMeta{mutation_id:Some(MutationId(fixture["mutation"].as_str().unwrap().into())),dependencies:Vec::new(),base_version:0,author_id:Some(ActorId(authority.actor_utf8().clone().into())),timestamp:authority.next_clock(),undo_policy:UndoPolicy::ExactBaseOnly,payload_hash:None,semantic_kind:None,label:None,group_id:None,origin:protocol::MutationOrigin::Owner,transaction:None});
 publication.stage=Some(Box::new(stage));publication.phase=ArtifactStoreOneItemPublicationPhase::Publishing;
 let original_stage=std::ptr::from_ref(publication.stage.as_ref().unwrap().as_ref());let original_flags=publication.stage.as_ref().unwrap().unit_flags.as_ptr();
 let mut publisher=semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();let visibility=publisher.view();
 publication.group_preparation=Some(member_group::MemberGroupPreparation::new(&visibility,None,None));
 let mut prepared=false;let mut births=0;let mut releases=0;
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){
  let(step,heap)=original_heap(||store.stage_apply_batch_group(&mut publication,&visibility,ArtifactStoreOneItemGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step,ArtifactStoreOneItemPreparationStep::Blocked);assert_eq!((heap.0,heap.1),(0,0));
  let(step,heap)=original_heap(||store.stage_apply_batch_group(&mut publication,&visibility,grant).unwrap());let receipt=step.ownership_progress();assert!(receipt.fits(grant.retained_grant()));assert_eq!((heap.0,heap.1),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.0;releases+=heap.1;
  assert_eq!(store.snapshot().unwrap().n,Some(before));assert_eq!(std::ptr::from_ref(publication.stage.as_ref().unwrap().as_ref()),original_stage);assert_eq!(publication.stage.as_ref().unwrap().unit_flags.as_ptr(),original_flags);
  if matches!(step,ArtifactStoreOneItemPreparationStep::Prepared(..)){prepared=true;break;}
 }
 assert!(prepared);assert!(publication.stage.as_ref().unwrap().edit.is_none());assert!(publication.stage.as_ref().unwrap().applied_edit_id.capacity()>0);assert!(publication.stage.as_ref().unwrap().unit_flags.capacity()>0);
 let((),heap)=original_heap(||{assert!(publisher.commit());});assert_eq!((heap.0,heap.1),(0,0));
 let(step,heap)=original_heap(||store.adopt_apply_batch_group(&mut publication,&visibility,grant).unwrap());let receipt=step.ownership_progress();assert!(matches!(step,ArtifactStoreOneItemPreparationStep::Prepared(..)));assert!(receipt.fits(grant.retained_grant()));assert_eq!((heap.0,heap.1),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(store.snapshot().unwrap().n,Some(after));
 assert!(publication.acknowledge());
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if publication.terminal_is_empty(){break;}let(step,heap)=original_heap(||publication.close_step(grant.retained_grant()).unwrap());let receipt=step.progress();assert!(receipt.fits(grant.retained_grant()));assert_eq!((heap.0,heap.1),(receipt.retained_capacity_bytes,receipt.released_bytes));}
 assert!(publication.terminal_is_empty());let((),heap)=original_heap(||drop(publication));assert_eq!((heap.0,heap.1),(0,fixture["law"]["terminalDropBytes"].as_u64().unwrap()as usize));
 eprintln!("[DEBUG] original member group staged born={births} released={releases}, private before={before} committed after={after}, original stage+flags retained until quoted closure and terminalDrop0");
}

#[test]
fn artifact_store_one_item_grant_preserves_neutral_independent_currencies() {
    let fixture: serde_json::Value = serde_json::from_str(ONE_ITEM_PUBLICATION_FIXTURE).unwrap();
    for row in fixture["independentGrants"].as_array().unwrap() {
        let mut payload = row.clone();
        let permits = payload.as_object_mut().unwrap().remove("permitsOne").unwrap().as_bool().unwrap();
        let grant = ArtifactStoreOneItemGrant::from_value(payload.clone().into()).unwrap();
        assert_eq!(serde_json::Value::from(grant.to_value()), payload);
        assert_eq!(grant.permits_one(), permits);
        let actual = grant.retained_grant();
        assert_eq!([actual.maximum_items, actual.maximum_copy_bytes, actual.maximum_capacity_bytes, actual.maximum_release_bytes, actual.maximum_depth], [row["maximumItems"].as_u64().unwrap() as usize, row["maximumCopyBytes"].as_u64().unwrap() as usize, row["maximumCapacityBytes"].as_u64().unwrap() as usize, row["maximumReleaseBytes"].as_u64().unwrap() as usize, row["maximumDepth"].as_u64().unwrap() as usize]);
    }
    eprintln!("[DEBUG] original Store one-item schema preserves independent items/copy/capacity/whole-release/depth through actual neutral grant and serde_json oracle");
}

/// 🪪️ Reads the complete neutral error corpus's authored kind without message classification.
fn command_canonical_refusal_kind(value:&str)->semio_framework_value::ValueRefusalKind{
 use semio_framework_value::ValueRefusalKind as K;
 match value{"InvalidValue"=>K::InvalidValue,"Canceled"=>K::Canceled,"OwnershipLimit"=>K::OwnershipLimit,"AllocationFailed"=>K::AllocationFailed,"WorkLimit"=>K::WorkLimit,"DepthLimit"=>K::DepthLimit,"UnsupportedOwner"=>K::UnsupportedOwner,"InvariantViolated"=>K::InvariantViolated,_=>panic!("unknown authored refusal kind")}
}

#[test]
fn command_decode_canonical_pack_refusal_preserves_all_value_causes(){
 use semio_framework_value::ValueError;
 let fixture:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../../../../../../🔨️modules/🎒️pack/⚠️error/🧪️tests/🧫️fixtures/⚠️refusal/🔣️.json"))).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let kind=command_canonical_refusal_kind(row["kind"].as_str().unwrap());let mut cause=ValueError::new(kind,row["message"].as_str().unwrap());
  for part in row["path"].as_array().unwrap(){cause=cause.under(part.as_str().unwrap());}
  let allocation=cause.message.as_ptr();let refusal=pack::PackRefusal::from(cause);
  let(command,requested,released)=semio_framework_trace_memory_testing::observe_backing(||CommandDecodeError::from(refusal));
  assert_eq!((requested,released),(0,0),"typed joins do not allocate or retire copied diagnostics");
  let CommandDecodeError::Layout(semio_framework_os_spr::ProtocolError::Pack(pack::PackError::Refusal(pack::PackRefusal::ValueRefusal(value))))=&command else{panic!("original typed layout refusal");};
  assert_eq!(value.kind,kind);assert_eq!(value.message.as_ptr(),allocation);
  assert_eq!(command.to_string(),format!("command layout: {}",row["display"].as_str().unwrap()));
  let protocol=std::error::Error::source(&command).unwrap().downcast_ref::<semio_framework_os_spr::ProtocolError>().unwrap();
  let wrapped=std::error::Error::source(protocol).unwrap().downcast_ref::<pack::PackError>().unwrap();
  let original=std::error::Error::source(wrapped).unwrap().downcast_ref::<ValueError>().unwrap();assert!(std::ptr::eq(value,original));
  let reference=serde_json::json!({"kind":row["kind"],"message":row["display"].as_str().unwrap().strip_prefix("schema error: ").unwrap()});
  assert_eq!(serde_json::json!({"kind":format!("{:?}",value.kind),"message":value.message}),reference);
 }
 eprintln!("[DEBUG] Command layout preserves all eight original canonical Pack value causes without allocation");
}

#[test]
fn command_decode_canonical_pack_refusal_retains_actual_reader_and_operation_sources(){
 let mut reader=pack::ByteReader::new(&[0x80]);let refusal=reader.read_varint_u64().unwrap_err();let kind=refusal.kind();
 let command=CommandDecodeError::from(refusal);let CommandDecodeError::Layout(semio_framework_os_spr::ProtocolError::Pack(error))=&command else{panic!("actual primitive layout cause");};assert_eq!(error.refusal_kind(),Some(kind));
 let source=std::error::Error::source(&command).unwrap().downcast_ref::<semio_framework_os_spr::ProtocolError>().unwrap();assert!(std::ptr::eq(source,match &command{CommandDecodeError::Layout(error)=>error,_=>unreachable!()}));
 let mut reader=pack::ByteReader::new(&[]);let refusal=reader.read_u8().unwrap_err();let operation=CommandDecodeError::Operation{index:7,error:refusal.into()};
 let CommandDecodeError::Operation{index,error}=&operation else{unreachable!()};assert_eq!(*index,7);assert!(std::ptr::eq(error,std::error::Error::source(&operation).unwrap().downcast_ref::<semio_framework_os_spr::ProtocolError>().unwrap()));
 eprintln!("[DEBUG] Actual canonical primitive read refusal and indexed operation retain their original command causes");
}

#[test]
fn command_decode_canonical_pack_refusal_preserves_all_positioned_text_causes(){
 use semio_framework_diagnostic::{TextError,TextSpan};
 let fixture:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../../../../../../🔨️modules/🎒️pack/⚠️error/🧪️tests/🧫️fixtures/📍️text-refusal/🔣️.json"))).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let kind=command_canonical_refusal_kind(row["kind"].as_str().unwrap());let span=TextSpan{line:u32::try_from(row["span"]["line"].as_u64().unwrap()).unwrap(),column:u32::try_from(row["span"]["column"].as_u64().unwrap()).unwrap(),length:u32::try_from(row["span"]["length"].as_u64().unwrap()).unwrap()};
  let cause=match row["expected"].as_str(){Some(expected)=>TextError::expected(kind,row["message"].as_str().unwrap(),span,expected),None=>TextError::new(kind,row["message"].as_str().unwrap(),span)};
  let allocation=cause.message.as_ptr();let expected_allocation=cause.expected.as_ref().map(|text|text.as_ptr());
  let(wrapped,requested,released)=semio_framework_trace_memory_testing::observe_backing(||text_error_to_pack_error(cause));assert_eq!((requested,released),(0,0));
  let pack::PackError::Refusal(pack::PackRefusal::TextRefusal(value))=&wrapped else{panic!("original positioned text refusal");};
  assert_eq!(value.kind,kind);assert_eq!(value.span,span);assert_eq!(value.expected.as_deref(),row["expected"].as_str());assert_eq!(value.message.as_ptr(),allocation);assert_eq!(value.expected.as_ref().map(|text|text.as_ptr()),expected_allocation);assert_eq!(wrapped.to_string(),row["display"].as_str().unwrap());
  assert!(std::ptr::eq(value,std::error::Error::source(&wrapped).unwrap().downcast_ref::<TextError>().unwrap()));
 }
 eprintln!("[DEBUG] Store positioned Text-to-Pack join preserves eight kinds, span, expectation and exact original source");
}


fn direct_fixture_cases() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

fn assert_fixture_case<Op>(row: &serde_json::Value)
where
    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + OpText + OpBinary + PartialEq + std::fmt::Debug,
{
    let mut value = row["payload"].clone();
    value["operation"] = row["operation"].clone();
    let mutation = Op::from_value(value.into()).unwrap();
    assert!(mutation.descriptor().validate().is_ok());
    let mut unknown = serde_json::Value::from(mutation.to_value());
    unknown["unknown"] = serde_json::json!(true);
    assert!(Op::from_value(unknown.into()).is_err());
    for key in row["payload"].as_object().unwrap().keys() {
        let mut invalid = serde_json::Value::from(mutation.to_value());
        invalid[key] = serde_json::json!(true);
        assert!(Op::from_value(invalid.clone().into()).is_err());
        invalid[key] = if key == "physicalMs" { serde_json::json!(1e21) } else { serde_json::json!(2147483648i64) };
        assert!(Op::from_value(invalid.into()).is_err());
        let mut missing = serde_json::Value::from(mutation.to_value());
        missing.as_object_mut().unwrap().remove(key);
        assert_eq!(Op::from_value(missing.into()).is_ok(), row["operation"] == "assignN" && key == "n");
    }
    let before = DemoSnapshot { n: serde_json::from_value(row["before"].clone()).unwrap() };
    let outcome = mutation.diff(&before);
    let (diff, messages) = outcome.into_parts();
    let actual = semio_framework_os_spr::apply_diff(&diff, &before).unwrap();
    let decoded_diff = serde_json::from_value::<DemoDiff>(serde_json::to_value(&diff).unwrap()).unwrap();
    assert_eq!(semio_framework_os_spr::apply_diff(&decoded_diff, &before).unwrap(), actual);
    assert_eq!(actual.n, serde_json::from_value::<Option<i32>>(row["after"].clone()).unwrap());
    let level = messages.first().map(|message| format!("{:?}", message.level).to_lowercase());
    assert_eq!(level.as_deref(), row["level"].as_str());
    let mut restored = actual;
    for inverse in mutation.inverse(&before).expect("valid retained mutation inverse fixture").into_iter().rev() {
        restored = semio_framework_os_spr::apply_diff(inverse.diff(&restored).diff(), &restored).unwrap();
    }
    assert_eq!(restored, before);
    assert_eq!(Op::from_value(serde_json::from_slice::<serde_json::Value>(&serde_json::to_vec(&serde_json::Value::from(mutation.to_value())).unwrap()).unwrap().into()).unwrap(), mutation);
    let text = mutation.print_op();
    assert_eq!(text.split(' ').next(), mutation.descriptor().text_opcode);
    assert_eq!(Op::parse_op(&text).unwrap(), mutation);
    let binary = mutation.encode_op().unwrap();
    assert_eq!(binary.first(), Some(&1));
    assert_eq!(binary.get(1).copied().map(u32::from), mutation.descriptor().binary_tag);
    assert_eq!(Op::decode_op(&binary).unwrap(), mutation);
    assert_eq!(mutation.timestamp(), row.get("timestamp").map(|time| HybridLogicalTimestamp::new(0, time.as_u64().unwrap())));
}

fn assert_fixture_text_codecs<Op>(fixture: &serde_json::Value, family: &str, canonical: impl Fn(&serde_json::Value) -> Op)
where
    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + OpText + OpBinary + PartialEq + std::fmt::Debug,
{
    for row in fixture["textCodecs"].as_array().unwrap().iter().filter(|row| row["family"] == family) {
        let mutation = canonical(row);
        assert_eq!(mutation.print_op(), row["canonical"].as_str().unwrap());
        assert_eq!(Op::parse_op(row["canonical"].as_str().unwrap()).unwrap(), mutation);
        assert!(Op::parse_op(row["rejected"].as_str().unwrap()).is_err());
        assert_eq!(mutation.descriptor().text_opcode, Some(row["canonical"].as_str().unwrap().split(' ').next().unwrap()));
        assert_eq!(mutation.descriptor().binary_tag, row["tag"].as_u64().map(|tag| u32::try_from(tag).unwrap()));
    }
}


#[test]
fn direct_store_fixture_absence_inverse_boundary() {
    let fixture = direct_fixture_cases();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["family"] == "demo") {
        assert_fixture_case::<DemoMutation>(row);
    }
    assert_fixture_text_codecs(&fixture, "demo", |row| DemoMutation::AddN(AddN { delta: row["payload"]["delta"].as_i64().unwrap() as i32 }));
    assert_fixture_text_codecs(&fixture, "severity", |row| match row["variant"].as_str().unwrap() {
        "SetN" => SeverityMutation::SetN(SeveritySetN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        "SetWarningN" => SeverityMutation::SetWarningN(SetWarningN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        "SetErrorN" => SeverityMutation::SetErrorN(SetErrorN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        "SetFatalN" => SeverityMutation::SetFatalN(SetFatalN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        variant => panic!("unexpected severity text codec variant {variant}"),
    });
    assert_fixture_text_codecs(&fixture, "validated", |row| match row["variant"].as_str().unwrap() {
        "SetN" => ValidatedMutation::SetN(ValidatedSetN { n: row["payload"]["n"].as_i64().unwrap() as i32 }),
        "AssignN" => ValidatedMutation::AssignN(ValidatedAssignN { n: Some(row["payload"]["n"].as_i64().unwrap() as i32) }),
        variant => panic!("unexpected validated text codec variant {variant}"),
    });
    for json in ["{}", "{\"n\":null}"] {
        assert_eq!(serde_json::from_str::<DemoSnapshot>(json).unwrap().n, None);
    }
    for json in ["{\"n\":2147483648}", "{\"n\":-2147483649}", "{\"n\":0.5}", "{\"n\":\"1\"}"] {
        assert!(serde_json::from_str::<DemoSnapshot>(json).is_err());
    }
    let before = DemoSnapshot { n: Some(i32::MIN) };
    let mutation = DemoMutation::DeleteN(DeleteN {});
    let deleted = semio_framework_os_spr::apply_diff(mutation.diff(&before).diff(), &before).unwrap();
    assert_eq!(deleted.n, None);
    assert_eq!(semio_framework_os_spr::apply_diff(mutation.inverse(&before).expect("valid retained mutation inverse fixture")[0].diff(&deleted).diff(), &deleted).unwrap(), before);
}

#[test]
fn direct_store_fixture_structural_diff_composition() {
    let fixture = direct_fixture_cases();
    for row in fixture["diffCases"].as_array().unwrap() {
        let before = DemoSnapshot { n: serde_json::from_value(row["before"].clone()).unwrap() };
        let mut actual = before.clone();
        let mut combined = DemoDiff::default();
        for value in row["diffs"].as_array().unwrap() {
            let diff = serde_json::from_value::<DemoDiff>(value.clone()).unwrap();
            actual = semio_framework_os_spr::apply_diff(&diff, &actual).unwrap();
            combined.absorb(diff);
        }
        let expected = DemoSnapshot { n: serde_json::from_value(row["after"].clone()).unwrap() };
        assert_eq!(actual, expected);
        assert_eq!(semio_framework_os_spr::apply_diff(&combined, &before).unwrap(), expected);
        let round_trip = serde_json::from_value::<DemoDiff>(serde_json::to_value(&combined).unwrap()).unwrap();
        assert_eq!(semio_framework_os_spr::apply_diff(&round_trip, &before).unwrap(), expected);
        let restore = DemoMutation::AssignN(AssignN { n: before.n });
        assert_eq!(semio_framework_os_spr::apply_diff(restore.diff(&actual).diff(), &actual).unwrap(), before);
    }
}

#[test]
fn direct_store_fixture_timestamp() {
    let fixture = direct_fixture_cases();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["family"] == "timestamped") {
        assert_fixture_case::<TimestampedMutation>(row);
    }
    let op = TimestampedMutation::SetN(TimestampedSetN { n: 7, physical_ms: u64::MAX });
    assert_eq!(op.timestamp(), Some(HybridLogicalTimestamp::new(0, u64::MAX)));
    assert_eq!(TimestampedMutation::decode_op(&op.encode_op().unwrap()).unwrap(), op);
    assert_eq!(TimestampedMutation::parse_op(&op.print_op()).unwrap(), op);
    let json = serde_json::to_string(&op).unwrap();
    for invalid in ["18446744073709551616", "-1", "0.5", "1e21", "null", "\"1\""] {
        assert!(serde_json::from_str::<TimestampedMutation>(&json.replace("18446744073709551615", invalid)).is_err());
    }
}

#[test]
fn direct_store_fixture_severity_and_rejection() {
    let fixture = direct_fixture_cases();
    for row in fixture["cases"].as_array().unwrap() {
        match row["family"].as_str().unwrap() {
            "severity" => assert_fixture_case::<SeverityMutation>(row),
            "validated" => assert_fixture_case::<ValidatedMutation>(row),
            _ => {}
        }
    }
}

#[test]
fn direct_store_fixture_lossy_oracle() {
    let op = LossyMutation::SetN(LossySetN { n: 7 });
    let json = serde_json::to_value(&op).unwrap();
    assert_eq!(json, serde_json::json!({"SetN": 7}));
    let decoded = serde_json::from_value::<LossyMutation>(json).unwrap();
    assert_eq!(decoded, LossyMutation::SetN(LossySetN { n: 0 }));
    assert_ne!(decoded, op);
    assert_eq!(LossyMutation::decode_op(&op.encode_op().unwrap()).unwrap(), decoded);
    for json in ["{\"SetN\":null}", "{\"SetN\":\"7\"}", "{\"SetN\":true}", "{\"SetN\":{}}", "{\"SetN\":2147483648}", "{\"SetN\":7,\"unknown\":true}"] {
        assert!(serde_json::from_str::<LossyMutation>(json).is_err());
    }
}

const ONE_ITEM_PUBLICATION_FIXTURE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧫️fixtures/artifact-store-one-item-publication-v1/🔣️.json"));

const EPHEMERAL_ONE_ITEM_PUBLICATION_FIXTURE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧫️fixtures/artifact-ephemeral-one-item-publication-v1/🔣️.json"));

struct SerdeOneItemPublicationOracle {
    maximum_items: usize,
    maximum_bytes: usize,
    maximum_retries: u8,
}

impl SerdeOneItemPublicationOracle {
    fn from_host_snapshot(fixture: &serde_json::Value) -> Self {
        Self {
            maximum_items: fixture["capacities"]["maximumWorkItems"].as_u64().expect("maximum work items") as usize,
            maximum_bytes: fixture["capacities"]["maximumRetainedBytes"].as_u64().expect("maximum retained bytes") as usize,
            maximum_retries: fixture["capacities"]["maximumRetries"].as_u64().expect("maximum retries") as u8,
        }
    }

    fn admits(&self, items: usize, bytes: usize) -> bool {
        items != 0 && items <= self.maximum_items && bytes <= self.maximum_bytes
    }

    fn freshness(base_generation: u64, live_generation: u64, base_revision: u64, live_revision: u64) -> &'static str {
        if base_generation == live_generation && base_revision == live_revision { "accepted" } else { "fault" }
    }

    fn retry(&self, attempts: u8, acknowledged: bool) -> &'static str {
        if attempts > self.maximum_retries {
            "retry-exhausted"
        } else if acknowledged {
            "complete-empty"
        } else {
            "awaiting-ack"
        }
    }
}

#[test]
fn one_item_publication_fixture_matches_the_third_party_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(ONE_ITEM_PUBLICATION_FIXTURE).expect("language-neutral one-item publication fixture");
    let oracle = SerdeOneItemPublicationOracle::from_host_snapshot(&fixture);
    assert_eq!(fixture["capacities"]["historyItems"], semio_framework_os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY);
    for case in fixture["admissionCases"].as_array().expect("admission cases") {
        let actual = oracle.admits(case["workItems"].as_u64().expect("work items") as usize, case["retainedBytes"].as_u64().expect("retained bytes") as usize);
        assert_eq!(actual, case["accepted"].as_bool().expect("accepted"), "{}", case["id"]);
        assert_eq!(case["fuelAfter"], 0, "rejected preflight consumes no fuel");
    }
    for case in fixture["authorityCases"].as_array().expect("authority cases") {
        let accepted = case["authorityMatches"] == true && case["digestOwner"] == "store" && case["cursorFieldsExposed"] == false;
        assert_eq!(accepted, case["accepted"].as_bool().expect("authority acceptance"), "{}", case["id"]);
    }
    for case in fixture["lifecycleCases"].as_array().expect("lifecycle cases") {
        match case["id"].as_str().expect("case id") {
            "stale-generation" | "stale-revision" => assert_eq!(
                SerdeOneItemPublicationOracle::freshness(
                    case["baseGeneration"].as_u64().expect("base generation"),
                    case["liveGeneration"].as_u64().expect("live generation"),
                    case["baseRevision"].as_u64().expect("base revision"),
                    case["liveRevision"].as_u64().expect("live revision"),
                ),
                case["expected"].as_str().expect("expected freshness")
            ),
            "retry-then-ack" | "retry-plus-one" => {
                assert_eq!(oracle.retry(case["retryAttempts"].as_u64().expect("retry attempts") as u8, case["acknowledged"].as_bool().expect("acknowledged")), case["expected"].as_str().expect("expected retry outcome"));
            }
            "cancel-before-publish" | "interrupted-close" => assert_eq!(case["expected"], "complete-empty"),
            unexpected => panic!("unknown one-item lifecycle fixture {unexpected}"),
        }
    }
}

#[test]
fn ephemeral_one_item_publication_fixture_matches_the_third_party_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(EPHEMERAL_ONE_ITEM_PUBLICATION_FIXTURE).expect("language-neutral ephemeral publication fixture");
    assert_eq!(fixture["lanes"], serde_json::json!(["presence", "transient"]));
    for case in fixture["admissionCases"].as_array().expect("ephemeral admission cases") {
        let work_items = case["workItems"].as_u64().expect("work items") as usize;
        let retained_bytes = case["retainedBytes"].as_u64().expect("retained bytes") as usize;
        let factories = case["preparationFactory"].as_bool().expect("preparation factory") && case["retirementFactory"].as_bool().expect("retirement factory");
        let admitted = factories && ArtifactStoreOneItemFootprint { work_items, retained_bytes }.is_admissible();
        assert_eq!(admitted, case["accepted"].as_bool().expect("accepted"), "{}", case["id"]);
    }
    for case in fixture["lifecycleCases"].as_array().expect("ephemeral lifecycle cases") {
        let expected = case["expected"].as_str().expect("expected");
        match case["id"].as_str().expect("case id") {
            "stale-generation" => assert_eq!(expected, "fault"),
            "single-publication" => assert_eq!(case["generationDelta"], 1),
            "cancel" | "retry-ack" | "interrupted-close" => assert_eq!(expected, "complete-empty"),
            unexpected => panic!("unknown ephemeral lifecycle fixture {unexpected}"),
        }
    }
}

#[test]
fn one_item_publication_source_denies_generic_or_unbounded_shortcuts() {
    let source = ["../../../../⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs", "../../../../⚙️engine/📥️lifted/🏪️store/🦀️.rs", "../../../../🎮️one-item/📬️publication/🦀️.rs"].iter().map(|file| std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file)).unwrap_or_else(|error| panic!("read {file}: {error}"))).collect::<String>();
    let authority_start = source.find("pub struct ArtifactStoreOneItemLiveAuthority").expect("private live authority");
    let authority_end = source[authority_start..].find("impl ArtifactStoreOneItemLiveAuthority").map(|offset| authority_start + offset).expect("authority helper");
    let authority_shape = &source[authority_start..authority_end];
    for forbidden in ["pub operation:", "pub generation:", "pub base_revision:", "pub base_applied_edit_count:", "pub next_sequence_number:", "pub next_clock:", "pub actor:"] {
        assert!(!authority_shape.contains(forbidden), "live authority exposes raw fabrication field {forbidden}");
    }
    let prepared_start = source.find("pub struct ArtifactStoreOneItemPrepared").expect("sealed prepared envelope");
    let prepared_end = source[prepared_start..].find("impl<P, Mutation> ArtifactStoreOneItemPrepared").map(|offset| prepared_start + offset).expect("prepared accessor");
    let prepared_shape = &source[prepared_start..prepared_end];
    assert!(!prepared_shape.contains("pub edit:"));
    assert!(!prepared_shape.contains("pub edit_digest:"));
    assert!(!prepared_shape.contains("cursor"), "domain-prepared envelope cannot carry cursor or history fabrication fields");
    let start = source.find("pub fn advance_apply_batch(").expect("retained batched advance");
    let end = source[start..].find("pub fn cancel_apply_batch(").map(|offset| start + offset).expect("retained batched cancel");
    let advance = &source[start..end];
    for forbidden in [".diff(", ".inverse(", "apply_mutation(", "replay_mutations(", "apply_command(", "flush_outbound(", "try_reserve_exact(", "post_snapshot.clone()", "current.as_ref().clone()", ".to_vec()"] {
        assert!(!advance.contains(forbidden), "batched advance contains forbidden shortcut {forbidden}");
    }
    let close_start = source.find("pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant)").expect("one-item close");
    let close_end = source[close_start..].find("pub fn terminal_is_empty").map(|offset| close_start + offset).expect("one-item terminal witness");
    let close = &source[close_start..close_end];
    assert!(close.contains("maximum_items: 1"), "a close turn hands each nested owner exactly one item");
    assert!(close.contains("maximum_release_bytes") && close.contains("maximum_copy_bytes") && close.contains("maximum_capacity_bytes"), "a close turn gates on every independent byte axis");
    assert!(close.contains("terminal_is_empty"));
    assert!(!close.contains(".clear()"));
    assert!(!advance.contains("prepared_edit_digest("), "Store commit validation must not reserialize retained edits");
    let fold_start = source.find("fn fold_batch_item(").expect("retained batched fold");
    let fold_end = source[fold_start..].find("fn empty_batch_stage(").map(|offset| fold_start + offset).expect("retained batched stage shell");
    let fold = &source[fold_start..fold_end];
    assert!(fold.contains("authority.validate_prepared(candidate)"), "Store validation must verify its private exact-owner seal before folding an item");
    for forbidden in [".diff(", "apply_mutation(", "replay_mutations(", "apply_command(", "flush_outbound(", "post_snapshot.clone()", "current.as_ref().clone()", ".to_vec()"] {
        assert!(!fold.contains(forbidden), "batched fold contains forbidden shortcut {forbidden}");
    }
    assert_eq!(fold.matches("try_reserve_exact(").count(), 1, "the batched stage reserves every owner's whole admitted capacity through one exact reservation");
    assert_eq!(fold.matches("reserve_original!(&mut").count(), 3, "the batched stage reserves its unit flags, forwards and metadata capacity exactly once each; the inverse pages reserve through their own exact page stack");
    assert!(fold.contains("original.inverse.pop()") && fold.contains("target.inverse.push_reserved(item)"), "a staged item's inverse block is popped from the original tail and pushed in order, which reverses it exactly as replay_mutations reverses one operation's inverse");
}

/// 🧹️ Takes one returned owner through the grant-admitting probe, handing back the exact registered `Arc`.
fn take_one_returned<T: Send + Sync + 'static>(registry: &semio_framework_value_snapshot_read::SnapshotReadRegistryHandle) -> Result<Option<Arc<T>>, SnapshotReadLeaseRefusal> {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 };
    registry.try_admit_one_returned::<T, Arc<T>>(grant, |owner, _| Ok((owner, RetainedCloneProgress::default()))).map(|(owner, _)| owner)
}


#[test]
fn snapshot_read_lease_capacity_plus_one_returns_the_exact_owner_and_every_registered_owner_retires() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let mut reads = Vec::with_capacity(SNAPSHOT_READ_LEASE_CAPACITY);
    for index in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        let owner = Arc::new(index);
        let lease = registry.try_issue(owner.clone()).expect("fixed snapshot read lease admits its exact capacity");
        reads.push(ErasedSnapshotRead::new(owner, lease));
    }
    let rejected = Arc::new(usize::MAX);
    let returned = match registry.try_issue(rejected.clone()) {
        Ok(_) => panic!("fixed snapshot read lease rejects capacity plus one"),
        Err(returned) => returned,
    };
    assert!(Arc::ptr_eq(&rejected, &returned));
    for read in reads {
        let owner = match read.into_typed::<usize>(&registry) {
            Ok(owner) => owner,
            Err(_) => panic!("exact registered snapshot lease retires"),
        };
        drop(owner);
    }
    assert!(registry.terminal_is_empty());
}

#[test]
fn snapshot_commit_authority_rejects_stale_generation_and_revision() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let first = [7; 32];
    let second = [9; 32];
    assert!(registry.publish_authority(4, first));
    assert!(registry.authority_matches(4, first));
    assert!(!registry.authority_matches(5, first));
    assert!(!registry.authority_matches(4, second));
    assert!(registry.publish_authority(5, second));
    assert!(!registry.authority_matches(4, first));
    assert!(registry.authority_matches(5, second));
}

#[test]
fn dropped_snapshot_read_remains_observable_until_one_slot_per_step_cleanup_takes_its_guard() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let owner = Arc::new(String::from("retained"));
    let lease = registry.try_issue(owner.clone()).expect("snapshot read lease admission");
    let index = lease.index;
    let generation = lease.generation;
    let read = ErasedSnapshotRead::new(owner, lease);
    assert!(registry.contains(index, generation));
    assert!(!registry.terminal_is_empty());
    drop(read);
    assert!(registry.contains(index, generation));
    let mut retired = None;
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        if let Some(owner) = registry.try_admit_one_returned::<String,_>(physical_test_close_grant(),|original,_|Ok((original,RetainedCloneProgress{copied_items:1,..Default::default()}))).map(|(original,progress)|{assert!(progress.fits(physical_test_close_grant()));original}).expect("one fixed cleanup probe") {
            retired = Some(owner);
            break;
        }
    }
    assert_eq!(retired.as_deref().map(String::as_str), Some("retained"));
    drop(retired);
    assert!(registry.terminal_is_empty());
}

#[test]
fn stale_snapshot_read_generation_never_removes_a_reused_slot() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let owner = Arc::new(7u32);
    let lease = registry.try_issue(owner.clone()).expect("first exact lease");
    let index = lease.index;
    let generation = lease.generation;
    let read = ErasedSnapshotRead::new(owner, lease);
    drop(match read.into_typed::<u32>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("first exact retirement"),
    });
    assert!(registry.try_take(index, generation).is_err());
    assert!(registry.terminal_is_empty());
}

#[test]
fn snapshot_read_lease_contention_returns_the_exact_untouched_owner() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let owner = Arc::new(String::from("contention-owner"));
    let guard = registry.state.try_lock().expect("hold exact registry contention fixture");
    let rejected = match registry.try_issue(owner.clone()) {
        Ok(_) => panic!("nonblocking admission rejects while the fixed registry is contended"),
        Err(rejected) => rejected,
    };
    assert!(Arc::ptr_eq(&owner, &rejected));
    assert_eq!(Arc::strong_count(&owner), 2, "contention creates no hidden owner clone");
    drop(guard);
    let lease = registry.try_issue(rejected.clone()).expect("the same owner retries after contention");
    let read = ErasedSnapshotRead::new(rejected, lease);
    drop(match read.into_typed::<String>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("retried exact owner handback succeeds"),
    });
    assert!(registry.terminal_is_empty());
}

#[test]
fn artifact_store_edit_retirement_is_interruptible_and_terminal_empty_after_deep_strings() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["editRetirement"];
    let strings = law["strings"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().repeat(law["repeat"].as_u64().unwrap() as usize)).collect::<Vec<_>>();
    let mut inverse = semio_framework_value::list::PagedList::new(); inverse.try_push(DemoMutation::SetN(SetN { n: 7 })).unwrap(); drop(inverse.pop());
    assert!(!inverse.terminal_is_empty());
    let forwards = Vec::with_capacity(law["forwardBackingSlots"].as_u64().unwrap() as usize); let forward_pointer = forwards.as_ptr();
    let edit = Edit::<DemoMutation> { line: None, id: strings[0].clone(), actor: Some(strings[1].clone().into()), forwards, inverse, mutation_meta: Vec::with_capacity(law["metadataBackingSlots"].as_u64().unwrap() as usize), verb: None, sequence_number: 1, started_at: strings[2].clone(), finished_at: Some(strings[3].clone()) };
    let payload_bytes = strings.iter().map(String::len).sum::<usize>();
    let (result, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| ArtifactStoreEditRetirement::new(edit));
    let mut retirement = result.unwrap_or_else(|_| panic!("empty original mutation lanes admitted"));
    assert_eq!((allocated, released), (0, law["constructorDropBytes"].as_u64().unwrap() as usize));
    assert_eq!(retirement.forwards.as_ref().unwrap().as_ptr(), forward_pointer);
    let copy = law["maximumCopyBytes"].as_u64().unwrap() as usize; let mut physical_release = 0;
    for _ in 0..10_000 {
        if retirement.terminal_is_empty() {
            assert!(physical_release >= payload_bytes);
            let (_, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| drop(retirement));
            assert_eq!((allocated, released), (0, law["terminalDropBytes"].as_u64().unwrap() as usize));
            eprintln!("[DEBUG] original edit retains forward/inverse/metadata backings at birth; bounded copy and exact whole physical releases match System through terminal-empty"); return;
        }
        let demand = retirement.demands(copy).unwrap();
        let grant = RetainedCloneGrant { maximum_copy_bytes: copy.max(demand.copy_bytes), ..physical_test_close_grant() };
        assert!(demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth);
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| retirement.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
        assert_eq!((allocated, released), (0, 0)); assert_eq!(step.progress(), RetainedCloneProgress::default()); assert_eq!(retirement.demands(copy).unwrap(), demand);
        if demand.release_bytes != 0 {
            let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| retirement.close_step(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }).unwrap());
            assert_eq!((allocated, released), (0, 0)); assert_eq!(step.progress(), RetainedCloneProgress::default()); assert_eq!(retirement.demands(copy).unwrap(), demand);
        }
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| retirement.close_step(grant).unwrap());
        assert!(step.progress().fits(grant)); assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes)); physical_release += released;
    }
    panic!("original edit retirement must make finite progress across interruptions");
}

#[test]
fn artifact_store_edit_retirement_rejects_and_returns_the_exact_nonempty_mutation_owner() {
    let edit = Edit::<DemoMutation> {
        line: None,
        id: "edit-owner".into(),
        actor: None,
        forwards: vec![DemoMutation::SetN(SetN { n: 7 })],
        inverse: Vec::new().into(),
        mutation_meta: Vec::new(),
        verb: None,
        sequence_number: 1,
        started_at: "now".into(),
        finished_at: None,
    };
    let rejected = match ArtifactStoreEditRetirement::new(edit) {
        Err(rejected) => rejected,
        Ok(_) => panic!("a nonempty mutation vector must remain with its exact domain owner"),
    };
    assert_eq!(rejected.forwards, vec![DemoMutation::SetN(SetN { n: 7 })]);
}

#[test]
fn artifact_store_history_metadata_retirement_cursors_authors_pins_and_ids_under_each_grant() {
    let checkpoint = Checkpoint {
        id: "checkpoint".repeat(64),
        change_ids: vec!["change-a".repeat(64), "change-b".repeat(64)],
        parent_id: Some("parent".repeat(64)),
        authors: vec![Author { id: "author".repeat(64), name: "name".repeat(64), avatar: Some("avatar".repeat(64)) }],
        message: Some("message".repeat(64)),
        timestamp: "timestamp".repeat(64),
        composition_pins: vec![semio_framework_os_vcs::CompositionPin {
            child_ref: semio_framework_artifact_reference::ArtifactRef { artifact_id: "child".repeat(64), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "kind".repeat(64), standard: "standard".repeat(64), subset: "subset".repeat(64) } },
            checkpoint_id: "child-checkpoint".repeat(64),
        }],
    };
    let mut retirement = ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint);
    let grant=RetainedCloneGrant { maximum_copy_bytes:11,..physical_test_close_grant() };
    for _ in 0..10_000 {
        if retirement.terminal_is_empty() { return; }
        let demand=retirement.next_release_byte_demand().unwrap();
        if demand>11 {
            let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirement.close_step(RetainedCloneGrant { maximum_release_bytes:11,..grant }).unwrap());
            assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(retirement.next_release_byte_demand().unwrap(),demand);
        }
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirement.close_step(grant).unwrap());
        assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        assert!(retirement.terminal_is_empty() || step.progress()!=RetainedCloneProgress::default(),"funded owned metadata makes genuine progress without an external wait");
    }
    panic!("history metadata must terminate across interrupted supplied grants");
}

#[test]
fn fixed_edit_message_ledger_resolves_probe_collisions_without_a_parallel_runtime_index() {
    let mut first_by_slot = BTreeMap::new();
    let mut collision = None;
    for index in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY * 2 {
        let id = format!("edit-collision-{index}");
        let slot = ArtifactEditMessageLedger::hash(&id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
        if let Some(first) = first_by_slot.insert(slot, id.clone()) {
            collision = Some((first, id));
            break;
        }
    }
    let (first, second) = collision.expect("pigeonhole probe collision exists");
    let entries = vec![
        semio_framework_os_spr::EditMessages { edit_id: first.clone(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "first")] },
        semio_framework_os_spr::EditMessages { edit_id: second.clone(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "second")] },
    ];
    let mut ledger = ArtifactEditMessageLedger::try_from_entries(entries).unwrap_or_else(|_| panic!("colliding fixed entries probe deterministically"));
    assert_eq!(ledger.get_by_id(&first).and_then(|entry| entry.messages.first()).map(|message| message.message.as_str()), Some("first"));
    assert_eq!(ledger.get_by_id(&second).and_then(|entry| entry.messages.first()).map(|message| message.message.as_str()), Some("second"));
    while let Some(entry) = ledger.pop() {
        let mut retirement = ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages);
        while !retirement.terminal_is_empty() {
            let grant=physical_test_close_grant();
            let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirement.close_step(grant).expect("colliding owner declared close"));
            assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        }
    }
}

fn drain_rejected_edit_message_ledger(mut rejected: ArtifactEditMessageLedgerRejected) {
    let grant=physical_test_close_grant();
    for _ in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY*32 {
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||rejected.close_step(grant).expect("rejected ledger declared close"));
        assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        match step {
            RetainedCloneStep::Complete(_) => { assert!(rejected.terminal_is_empty());return; }
            RetainedCloneStep::Progress(progress) => assert!(progress!=RetainedCloneProgress::default(),"original rejected ledger has no external wait; declared policy must advance its original owner"),
        }
    }
    panic!("rejected ledger close must terminate under repeated supplied grants");
}

#[test]
fn fixed_edit_message_ledger_rejects_duplicate_and_capacity_plus_one_with_exact_retained_owners() {
    let duplicate = vec![
        semio_framework_os_spr::EditMessages { edit_id: "duplicate".into(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "first")] },
        semio_framework_os_spr::EditMessages { edit_id: "duplicate".into(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "second")] },
    ];
    let duplicate = match ArtifactEditMessageLedger::try_from_entries(duplicate) {
        Err(rejected) => rejected,
        Ok(mut ledger) => {
            while ledger.pop().is_some() {}
            panic!("duplicate identities must fail before payload adoption")
        }
    };
    assert_eq!(duplicate.retained_entries().len(), 2, "duplicate rejection retains the exact owner batch");
    drain_rejected_edit_message_ledger(duplicate);

    let plus_one = (0..=ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).map(|index| semio_framework_os_spr::EditMessages { edit_id: format!("edit-{index}"), messages: Vec::new() }).collect();
    let plus_one = match ArtifactEditMessageLedger::try_from_entries(plus_one) {
        Err(rejected) => rejected,
        Ok(mut ledger) => {
            while ledger.pop().is_some() {}
            panic!("capacity plus one must fail before payload adoption")
        }
    };
    assert_eq!(plus_one.retained_entries().len(), ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY + 1, "capacity rejection retains every exact owner");
    drain_rejected_edit_message_ledger(plus_one);
}

#[test]
fn fixed_edit_message_ledger_preserves_order_and_rejects_stale_generation_after_slot_reuse() {
    let entries =
        vec![semio_framework_os_spr::EditMessages { edit_id: "first".into(), messages: Vec::new() }, semio_framework_os_spr::EditMessages { edit_id: "middle".into(), messages: Vec::new() }, semio_framework_os_spr::EditMessages { edit_id: "last".into(), messages: Vec::new() }];
    let mut ledger = match ArtifactEditMessageLedger::try_from_entries(entries) {
        Ok(ledger) => ledger,
        Err(rejected) => {
            drain_rejected_edit_message_ledger(rejected);
            panic!("valid fixed ledger must be admitted")
        }
    };
    let middle = ledger.find_ticket("middle").expect("middle ticket");
    let removed = ledger.remove(middle).expect("middle owner detaches in O(1)");
    assert_eq!(removed.edit_id, "middle");
    let reused = ledger.try_push(semio_framework_os_spr::EditMessages { edit_id: "replacement".into(), messages: Vec::new() }).unwrap_or_else(|_| panic!("released slot is reusable"));
    assert_eq!(reused.slot, middle.slot);
    assert_ne!(reused.generation, middle.generation);
    assert!(ledger.get(middle).is_none(), "stale generation cannot resolve the reused slot");
    assert_eq!(ledger.iter().map(|entry| entry.edit_id.as_str()).collect::<Vec<_>>(), vec!["first", "last", "replacement"]);
    let removed = ArtifactStoreMessageLedgerRetirement::new(removed.edit_id, removed.messages);
    let mut removed = removed;
    while !removed.terminal_is_empty() {
        let grant=RetainedCloneGrant {maximum_copy_bytes:ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES,..physical_test_close_grant()};let step=removed.close_step(grant).expect("original message owner full-policy close");assert!(step.progress().fits(grant));
    }
    while let Some(entry) = ledger.pop() {
        let mut retirement = ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages);
        while !retirement.terminal_is_empty() {
                let grant=RetainedCloneGrant {maximum_copy_bytes:ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES,..physical_test_close_grant()};let step=retirement.close_step(grant).expect("original message owner full-policy close");assert!(step.progress().fits(grant));
        }
    }
    assert!(ledger.terminal_is_empty());
}

struct UnitOwnedRetirement {
    value: Option<()>,
}

impl ErasedSnapshotRetirement for UnitOwnedRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original unit payload requires supplied depth")); }
        self.value.take();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1,..Default::default() }))
    }
    fn terminal_is_empty(&self) -> bool { self.value.is_none() }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.value.is_some()))}
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct UnitOwnedRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<()> for UnitOwnedRetirementFactory {
    fn retirement_birth_bytes(&self,_:&())->usize { std::mem::size_of::<UnitOwnedRetirement>() }
    fn retire_owned(&self, value: (), grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,())> {
        admit_artifact_retirement(value,grant,|original|UnitOwnedRetirement { value: Some(original) })
    }
}

#[test]
fn displaced_owner_reservations_preserve_capacity_generation_and_interrupted_close() {
    let mut retirements = ArtifactStoreDisplacedRetirements::new();
    let large = retirements.reserve_owner_slots(ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY - 1).expect("exact capacity minus one is reserved");
    let mut final_slot = retirements.reserve_owner_slots(1).expect("the final exact retirement slot is independently reserved");
    assert!(retirements.reserve_owner_slots(1).is_err(), "capacity +1 cannot consume an owner");
    retirements.release_owner_slots(large).expect("unused large reservation returns atomically");
    let grant=physical_test_close_grant();
    let(owner,birth)=UnitOwnedRetirementFactory.retire_owned((),grant).expect("original unit cursor admission is funded");
    assert_eq!(birth.retained_capacity_bytes,std::mem::size_of::<UnitOwnedRetirement>());
    retirements.push_owner_reserved(&mut final_slot, owner).unwrap_or_else(|_| panic!("the exact final-slot reservation retains its owner"));
    retirements.release_owner_slots(final_slot).expect("consumed reservation has no live credit");
    assert_eq!(retirements.close_step(RetainedCloneGrant { maximum_items: 0,..grant }).unwrap().progress(),RetainedCloneProgress::default());
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirements.close_step(grant).unwrap());
    assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress { copied_items: 1,..Default::default() });
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirements.close_step(RetainedCloneGrant { maximum_release_bytes: 0,..grant }).unwrap());
    assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirements.close_step(grant).unwrap());
    assert_eq!((allocated,released),(0,std::mem::size_of::<UnitOwnedRetirement>()));assert_eq!(step.progress(),RetainedCloneProgress { copied_items: 1,released_bytes: released,..Default::default() });
    assert_eq!(retirements.close_step(grant).unwrap(),RetainedCloneStep::Complete(Default::default()));
    assert!(retirements.terminal_is_empty());
    let expected=retirements.backing_release_byte_demand().unwrap();
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||retirements.release_empty_backing_step(grant).unwrap());
    assert_eq!((allocated,released),(0,expected));assert_eq!(step.progress().released_bytes,expected);

    let mut reservations = Vec::new();
    for _ in 0..ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY {
        reservations.push(retirements.reserve_owner_slots(1).expect("fixed reservation registry admits its exact capacity"));
    }
    assert!(retirements.reserve_owner_slots(1).is_err(), "reservation-registry +1 fails before owner construction");
    for reservation in reservations {
        retirements.release_owner_slots(reservation).expect("each exact generation-qualified reservation returns once");
    }
    assert!(retirements.terminal_is_empty());
}

fn completed_record_owner(id: &str) -> Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>> {
    Box::new(ArtifactEnvelopeCompletedRecordOwner::new(create_document_envelope_from_genesis("test.completed-record/v1", id, AdmittedArtifactGenesis::from_decoded_pack((), vec![0]), None), Arc::new(UnitOwnedRetirementFactory), Arc::new(UnitOwnedRetirementFactory)))
}

fn drain_completed_record(mut owner: Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>>) {
    let mut born = 0;
    let mut released = 0;
    for _ in 0..10_000 {
        let grant = physical_test_close_grant();
        assert!(owner.next_close_copy_byte_demand().expect("current completed record copy demand") <= grant.maximum_copy_bytes);
        assert!(owner.next_close_capacity_byte_demand(grant.maximum_copy_bytes).expect("current completed record capacity demand") <= grant.maximum_capacity_bytes);
        assert!(owner.next_close_release_byte_demand().expect("current completed record release demand") <= grant.maximum_release_bytes);
        assert!(owner.next_close_depth_demand().expect("current completed record depth demand") <= grant.maximum_depth);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
        let step = step.expect("completed record bounded close");
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        born += heap.requested_bytes;
        released += heap.released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(owner.terminal_is_empty());
            let frame = std::mem::size_of_val(owner.as_ref());
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, frame));
            eprintln!("[DEBUG] original completed record actual nested births={born} physical={released} separate original frame={frame}");
            return;
        }
    }
    panic!("completed record must reach terminal empty under repeated one-owner grants");
}

fn admit_completed_record(registry: &ArtifactEnvelopeCompletedRecordRegistry<(), ()>, owner: Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>>) -> ArtifactEnvelopeCompletedRecordTicket {
    match registry.try_admit(owner) {
        Ok(ticket) => ticket,
        Err((fault, owner)) => {
            drain_completed_record(owner);
            panic!("completed record admission unexpectedly failed: {fault:?}")
        }
    }
}

struct TestCompletedRecordTarget {
    expected: &'static str,
    adopted: Option<Box<dyn ArtifactEnvelopeCompletedRecord<(), ()>>>,
}

impl ArtifactEnvelopeCompletedRecordTarget<(), ()> for TestCompletedRecordTarget {
    fn try_adopt_completed(&mut self, envelope: ArtifactEnvelope<(), ()>) -> Result<(), ArtifactEnvelope<(), ()>> {
        if envelope.id != self.expected || self.adopted.is_some() {
            return Err(envelope);
        }
        self.adopted = Some(Box::new(ArtifactEnvelopeCompletedRecordOwner::new(envelope, Arc::new(UnitOwnedRetirementFactory), Arc::new(UnitOwnedRetirementFactory))));
        Ok(())
    }
}

#[test]
fn completed_envelope_registry_rejects_capacity_plus_one_and_reuses_slots_with_aba_safety() {
    let registry = ArtifactEnvelopeCompletedRecordRegistry::<(), ()>::new();
    let mut tickets = Vec::new();
    for index in 0..ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY {
        tickets.push(admit_completed_record(&registry, completed_record_owner(&format!("record-{index}"))));
    }
    let (fault, rejected) = registry.try_admit(completed_record_owner("plus-one")).expect_err("capacity plus one returns exact owner");
    assert_eq!(fault, ArtifactEnvelopeCompletedRecordFault::Capacity);
    drain_completed_record(rejected);

    let stale = tickets.remove(0);
    let owner = registry.try_detach(stale).expect("exact first completed owner");
    drain_completed_record(owner);
    let reused = admit_completed_record(&registry, completed_record_owner("reused"));
    assert_eq!(reused.index(), stale.index());
    assert_ne!(reused.generation(), stale.generation());
    match registry.try_detach(stale) {
        Err(fault) => assert_eq!(fault, ArtifactEnvelopeCompletedRecordFault::Stale),
        Ok(owner) => {
            drain_completed_record(owner);
            panic!("stale completed ticket detached the reused owner")
        }
    }
    tickets.push(reused);
    for ticket in tickets {
        drain_completed_record(registry.try_detach(ticket).expect("exact completed owner drains"));
    }
    assert!(registry.terminal_is_empty());
}

#[test]
fn completed_envelope_registry_publication_is_exact_once_and_close_cursor_is_interruptible() {
    let registry = ArtifactEnvelopeCompletedRecordRegistry::<(), ()>::new();
    let ticket = admit_completed_record(&registry, completed_record_owner("take-once"));
    let mut rejected = TestCompletedRecordTarget { expected: "wrong", adopted: None };
    assert!(!registry.try_publish_to(ticket, &mut rejected).expect("consumer rejection retains exact record"));
    let mut target = TestCompletedRecordTarget { expected: "take-once", adopted: None };
    let (published, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.try_publish_to(ticket, &mut target));
    assert!(published.expect("exact consumer atomically adopts completed record"));
    assert_eq!(heap.released_bytes, 0, "publication retains original record frame and factory sources for their separately granted close");
    assert!(!registry.ticket_reclaimed(ticket));
    let mut close_cursor = 0;
    assert_eq!(registry.try_next_close_ticket(&mut close_cursor).expect("published original close ticket"), Some(ticket));
    assert_eq!(registry.try_publish_to(ticket, &mut target).expect_err("completed record is exact once"), ArtifactEnvelopeCompletedRecordFault::Stale);
    drain_completed_record(registry.try_detach(ticket).expect("same published owner retires on original close pump"));
    assert!(registry.ticket_reclaimed(ticket));
    drain_completed_record(target.adopted.take().expect("test target retained adopted owner"));

    let ticket = admit_completed_record(&registry, completed_record_owner("interrupted-close"));
    let mut cursor = 0;
    assert_eq!(registry.try_next_ticket(&mut cursor).expect("fixed next ticket"), Some(ticket));
    let owner = registry.try_detach(ticket).expect("close pump detaches exact record");
    drain_completed_record(owner);
    assert!(registry.terminal_is_empty());
}


#[test]
fn artifact_envelope_decode_withholds_success_until_field_and_page_owners_are_terminal_empty() {
    use semio_framework_job::InteractiveJob;

    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut job = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed field decoder admission"));
    let mut preview_sequence = 0;
    let mut detached = None;
    for turn in 0..1_000 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let mut original_retained_progress_5=semio_framework_value::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(91),
            semio_framework_job::Generation(7),
            semio_framework_job::StepBudget::new(4, u64::MAX, schema_test_close_grant()),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
         &mut original_retained_progress_5);
        match job.step(&mut context).expect("original decoder outcome admission") {
            None | Some(semio_framework_job::JobOutcomeBorrow::Yield { .. }) => assert!(!job.terminal_is_empty() || registry.terminal_is_empty(), "a yielded decode retains exact owner authority until its funded outcome turn"),
            Some(semio_framework_job::JobOutcomeBorrow::Complete { state: None, output: None, .. }) => {
                assert!(job.terminal_is_empty(), "success is exposed only after exact field and page terminal witnesses");
                assert!(registry.terminal_is_empty(), "successful decode reclaimed its exact registered field owner");
                assert!(turn > 1, "decode and close span multiple bounded turns");
                return;
            }
            outcome => panic!("valid envelope decode produced {outcome:?}"),
        }
    }
    panic!("valid envelope decode did not terminate");
}

#[test]
fn cancelled_and_rejected_envelope_decodes_close_one_exact_owner_per_grant() {
    use semio_framework_job::InteractiveJob;

    let first = [b' '; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut cancelled = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[&first, br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed cancelled decoder admission"));
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    let mut preview_sequence = 0;
    let mut turns = 0;
    let mut detached = None;
    loop {
        drive_test_envelope_field_return(&registry, &mut detached);
        let mut original_retained_progress_6=semio_framework_value::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(91),
            semio_framework_job::Generation(7),
            semio_framework_job::StepBudget::new(1, u64::MAX, schema_test_close_grant()),
            cancel.clone(),
            semio_framework_job::default_now_us,
            &mut preview_sequence,
         &mut original_retained_progress_6);
        turns += 1;
        match cancelled.step(&mut context).expect("original cancelled decoder outcome admission") {
            None | Some(semio_framework_job::JobOutcomeBorrow::Yield { .. }) => assert!(!cancelled.terminal_is_empty() || registry.terminal_is_empty(), "a cancelled decode retains exact owner authority until its funded outcome turn"),
            Some(semio_framework_job::JobOutcomeBorrow::Cancelled { .. }) => break,
            outcome => panic!("cancelled envelope decode produced {outcome:?}"),
        }
        assert!(turns < 256);
    }
    assert!(cancelled.terminal_is_empty());
    assert!(registry.terminal_is_empty());
    assert!(turns >= 5, "field authority, two pages, and terminal shells require distinct grants");

    let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.test-rejected", offset: 0, line: 1, column: 1, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
    let mut rejected = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed rejected decoder admission"))
    .reject(diagnostic)
    .unwrap_or_else(|_| panic!("public exact rejection transfer"));
    let mut close_turns = 0;
    let mut detached = None;
    while !rejected.terminal_is_empty() {
        drive_test_envelope_field_return(&registry, &mut detached);
        let step = rejected.close_step(envelope_field_close_grant()).expect("retained decode rejection closes");
        assert!(step.progress().fits(envelope_field_close_grant()));
        close_turns += 1;
        assert!(close_turns < 256);
    }
    assert!(close_turns >= 3, "rejected field, page, and terminal record are independently witnessed");
    assert!(registry.terminal_is_empty());
}

#[test]
fn artifact_envelope_public_rejection_preserves_record_lease_ticket_double_return_and_generation_reuse() {
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let record = artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]);
    let record_identity = record.tokens.pages.slots.as_ptr();
    let source = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(record, &registry, Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 })).unwrap_or_else(|_| panic!("fixed public rejection decoder admission"));
    let field_ticket = source.field_ticket;
    let registry_identity = Arc::as_ptr(&source.field_registry);
    let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.public-rejected", offset: 7, line: 2, column: 3, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
    let mut rejected = source.reject(diagnostic).unwrap_or_else(|_| panic!("public rejection returns the exact owner authority"));
    assert_eq!(rejected.record.as_ref().map(|record| record.tokens.pages.slots.as_ptr()), Some(record_identity));
    assert_eq!(rejected.fields.as_ref().map(ArtifactEnvelopeFieldDecoderLease::ticket), Some(field_ticket));
    assert_eq!(Arc::as_ptr(&rejected.field_registry), registry_identity);
    assert_eq!(rejected.diagnostic, diagnostic);
    assert!(!registry.ticket_reclaimed(field_ticket));
    assert_eq!(rejected.close_step(RetainedCloneGrant::default()), Ok(RetainedCloneStep::Progress(Default::default())));
    assert_eq!(rejected.record.as_ref().map(|record| record.tokens.pages.slots.as_ptr()), Some(record_identity));
    assert_eq!(rejected.fields.as_ref().map(ArtifactEnvelopeFieldDecoderLease::ticket), Some(field_ticket));

    let mut detached = None;
    for _ in 0..256 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let step = rejected.close_step(envelope_field_close_grant()).expect("public rejection closes one exact owner");
        assert!(step.progress().fits(envelope_field_close_grant()));
        if rejected.terminal_is_empty() {
            break;
        }
    }
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(rejected.terminal_is_empty());
    assert!(registry.ticket_reclaimed(field_ticket));
    assert!(registry.terminal_is_empty());
    assert!(detached.is_none());

    let mut reused = registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("reused field generation"));
    let reused_ticket = reused.ticket();
    assert_eq!(reused_ticket.index(), field_ticket.index());
    assert_ne!(reused_ticket.generation(), field_ticket.generation());
    assert!(reused.return_now());
    assert!(!reused.return_now());
    drop(reused);
    drive_test_envelope_field_return(&registry, &mut detached);
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(registry.ticket_reclaimed(reused_ticket));
    assert!(registry.terminal_is_empty());
}

#[test]
fn interactive_envelope_close_reclaims_every_owner_through_the_job_protocol() {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};

    let mut caller=OriginalEnvelopeCaller::admit_cleanup("interactive-envelope");
    let mut cx=caller.context();
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut job = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed close-protocol decoder admission"));
    let grant=envelope_field_close_grant();
    job.begin_close();
    let mut detached = None;
    for _ in 0..256 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let before=cx.retained_progress();
        let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||cx.with_retained_loan(grant,|cx|job.close_step(cx)));
        if let Ok(step)=&result{assert_eq!(cx.retained_progress(),before.checked_add(step.progress()).unwrap());assert_eq!((born,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
        match result {
            Ok(InteractiveJobCloseStep::Pending { progress }) => {
                assert!(progress.fits(envelope_field_close_grant()));
            }
            Ok(InteractiveJobCloseStep::Blocked) => {}
            Err(error) => panic!("interactive envelope close refused: {error:?}"),
            Ok(InteractiveJobCloseStep::Complete { .. }) => {
                assert!(job.terminal_is_empty());
                assert!(registry.terminal_is_empty());
                drop(cx);
                caller.retire(grant);
                return;
            }
        }
    }
    panic!("interactive envelope close did not reclaim every exact owner");
}

#[test]
fn envelope_field_registry_retains_late_return_and_generation_reuse_until_bounded_app_reclaim() {
    let registry = ArtifactEnvelopeFieldDecoderRegistry::<(), ()>::new();
    let first = registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("first exact decoder lease"));
    let first_ticket = first.ticket();
    drop(first);
    assert_eq!(registry.next_returned_ticket(), Some(first_ticket));
    assert!(!registry.ticket_reclaimed(first_ticket));
    let mut detached = None;
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(registry.ticket_reclaimed(first_ticket));
    assert!(registry.terminal_is_empty(), "the registry is empty only after exact ownership transferred to the app close pump");
    assert!(detached.is_some(), "the detached owner remains retained outside the registry until its terminal step");
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(detached.is_none());
    assert!(registry.terminal_is_empty());

    let second = registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("reused exact decoder lease"));
    let second_ticket = second.ticket();
    assert_eq!(second_ticket.index(), first_ticket.index());
    assert_ne!(second_ticket.generation(), first_ticket.generation());
    assert!(!registry.ticket_reclaimed(second_ticket), "an old reclaimed generation cannot certify a reused slot");
    drop(second);
    drive_test_envelope_field_return(&registry, &mut detached);
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(registry.ticket_reclaimed(second_ticket));
    assert!(registry.terminal_is_empty());
}

#[test]
fn envelope_field_registry_capacity_and_contention_return_the_exact_decoder_before_adoption() {
    let registry = ArtifactEnvelopeFieldDecoderRegistry::<(), ()>::new();
    let mut leases = Vec::with_capacity(ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY);
    for _ in 0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY {
        leases.push(registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("fixed decoder capacity admits its exact maximum")));
    }
    let (fault, rejected) = match registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })) {
        Ok(_) => panic!("fixed decoder capacity plus one must fail closed"),
        Err(rejected) => rejected,
    };
    assert_eq!(fault, ArtifactEnvelopeFieldDecoderRegistryFault::Capacity);
    assert!(rejected.terminal_is_empty());
    drop(rejected);

    while let Some(lease) = leases.pop() {
        drop(lease);
        let mut detached = None;
        drive_test_envelope_field_return(&registry, &mut detached);
        drive_test_envelope_field_return(&registry, &mut detached);
        assert!(detached.is_none());
    }
    assert!(registry.terminal_is_empty());

    let state = registry.state.lock().expect("test owns exact registry contention guard");
    let (fault, rejected) = match registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })) {
        Ok(_) => panic!("contended decoder admission must fail closed"),
        Err(rejected) => rejected,
    };
    assert_eq!(fault, ArtifactEnvelopeFieldDecoderRegistryFault::Contended);
    assert!(rejected.terminal_is_empty());
    drop(rejected);
    drop(state);
    assert!(registry.terminal_is_empty());
}

/// 🎟️ A lease is returned EXACTLY ONCE, on either of the registry's two return paths, and the
/// path a return takes is decided by whether its root is still aliased outside the registry.
///
/// An aliased root has nothing for the retirement pump to dispose, so `try_release_aliased`
/// reclaims its slot on the returning thread and the pump never sees it. A sole-owner root is the
/// last `Arc` in the process, so it is counted into `returned` and handed to the pump, which
/// retires its domain tree in bounded steps. Both paths must refuse a second return of the same
/// generation: a lease that answered twice once leaked a `returned` credit against a slot that had
/// already been freed and reissued.
#[test]
fn snapshot_read_double_return_is_counted_once_and_reclaimed_once() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let aliased_root = Arc::new(11u32);
    let mut aliased = registry.try_issue(aliased_root.clone()).expect("exact aliased lease");
    assert!(aliased.return_now());
    assert!(!aliased.return_now(), "the same generation cannot be returned twice on the aliased path");
    assert_eq!(registry.returned.load(std::sync::atomic::Ordering::Acquire), 0, "an aliased root is reclaimed on the returning thread, not charged to the pump");
    assert_eq!(Arc::strong_count(&aliased_root), 1, "the aliased slot released its own owner alias in the same call");
    assert!(registry.try_admit_one_returned::<u32,_>(physical_test_close_grant(),|original,_|Ok((original,RetainedCloneProgress{copied_items:1,..Default::default()}))).map(|(original,progress)|{assert!(progress.fits(physical_test_close_grant()));original}).expect("one fixed return probe").is_none(), "the pump has nothing to reclaim for an aliased return");
    drop(aliased);
    drop(aliased_root);
    assert!(registry.terminal_is_empty());

    let mut lease = registry.try_issue(Arc::new(11u32)).expect("exact sole-owner lease");
    assert!(lease.return_now());
    assert!(!lease.return_now(), "the same generation cannot be returned twice on the pump path");
    assert_eq!(registry.returned.load(std::sync::atomic::Ordering::Acquire), 1);
    let mut reclaimed = None;
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        if let Some(owner) = registry.try_admit_one_returned::<u32,_>(physical_test_close_grant(),|original,_|Ok((original,RetainedCloneProgress{copied_items:1,..Default::default()}))).map(|(original,progress)|{assert!(progress.fits(physical_test_close_grant()));original}).expect("one fixed return probe") {
            reclaimed = Some(owner);
            break;
        }
    }
    assert_eq!(reclaimed.as_deref(), Some(&11));
    assert_eq!(registry.returned.load(std::sync::atomic::Ordering::Acquire), 0);
    assert!(registry.try_admit_one_returned::<u32,_>(physical_test_close_grant(),|original,_|Ok((original,RetainedCloneProgress{copied_items:1,..Default::default()}))).map(|(original,progress)|{assert!(progress.fits(physical_test_close_grant()));original}).expect("one fixed return probe").is_none(), "a reclaimed generation is never handed out a second time");
    drop(reclaimed);
    drop(lease);
    assert!(registry.terminal_is_empty());
}

#[test]
fn stale_snapshot_read_generation_cannot_aba_remove_a_reused_slot() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let owner = Arc::new(1u16);
    let first = registry.try_issue(owner.clone()).expect("first exact lease");
    let stale_index = first.index;
    let stale_generation = first.generation;
    let first_read = ErasedSnapshotRead::new(owner, first);
    drop(match first_read.into_typed::<u16>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("first exact lease retires"),
    });

    let mut reads = Vec::with_capacity(SNAPSHOT_READ_LEASE_CAPACITY);
    let mut reused_generation = None;
    for value in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        let owner = Arc::new(value as u16);
        let lease = registry.try_issue(owner.clone()).expect("fixed ring re-admits every slot");
        if lease.index == stale_index {
            reused_generation = Some(lease.generation);
        }
        reads.push(ErasedSnapshotRead::new(owner, lease));
    }
    assert!(reused_generation.is_some_and(|generation| generation != stale_generation));
    assert!(registry.try_take(stale_index, stale_generation).is_err(), "stale generation cannot remove the reused exact slot");
    for read in reads {
        drop(match read.into_typed::<u16>(&registry) {
            Ok(owner) => owner,
            Err(_) => panic!("every reused exact lease retires"),
        });
    }
    assert!(registry.terminal_is_empty());
}


/// 🧹️ Drives any space member's owned close to its terminal-empty witness on its own exact quoted per-axis demands.
fn drive_member_close<M: SpaceMember>(member: &mut M) {
    const CEILING: usize = 4096;
    let mut last = (semio_framework_value::RetirementDemand::default(), RetainedCloneProgress::default());
    for _ in 0..65_536 {
        if member.close_owned_terminal_is_empty() {
            return;
        }
        let demand = member.close_owned_demands(CEILING).expect("current independent member close frontier");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(CEILING), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = member.close_owned_step(grant).expect("member closes under its exact quoted owner grant");
        assert!(step.progress().fits(grant) && step.progress().copied_items <= 1);
        last = (demand, step.progress());
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(member.close_owned_terminal_is_empty());
            return;
        }
    }
    panic!("member retirement did not reach its terminal-empty witness; last frontier {:?} receipt {:?}", last.0, last.1);
}


#[semio_framework_async_macros::async_test]
async fn artifact_store_snapshot_roots_and_final_envelope_transfer_in_exact_close_order() {
    let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "root-close", DemoSnapshot { n: Some(7) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid empty-history store");
    store.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    *store.live.tail_undo_cache = Some(("tail-owner".repeat(64), Arc::clone(&*store.live.current)));
    let original = store.live.tail_undo_cache.as_ref().unwrap().0.as_ptr();
    let phase = ArtifactStoreCursorDisposerPhase::RuntimeOwners(5);
    let demand = store.close_cursor_demands(phase, 7).unwrap();
    let grant = RetainedCloneGrant { maximum_copy_bytes: 7, maximum_release_bytes: 0, ..physical_test_close_grant() };
    let (denied, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| store.close_cursor_admit(phase, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }));
    assert!(denied.is_err());
    assert_eq!((requested, released), (0, 0));
    assert_eq!(store.live.tail_undo_cache.as_ref().unwrap().0.as_ptr(), original);
    let (admitted, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| store.close_cursor_admit(phase, grant));
    let (tail_id, receipt) = admitted.unwrap();
    assert_eq!((requested, released), (receipt.retained_capacity_bytes, receipt.released_bytes));
    assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
    drive_retirement_terminal(tail_id.expect("original tail id retires before snapshot"));
    assert_eq!(store.live.tail_undo_cache.as_ref().unwrap().0.capacity(), 0);
    let (tail, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| store.close_cursor_admit(ArtifactStoreCursorDisposerPhase::TailSnapshot, grant));
    assert!(tail.unwrap().0.is_none());
    assert_eq!((requested, released), (0, 0));
    for phase in [ArtifactStoreCursorDisposerPhase::CurrentSnapshot, ArtifactStoreCursorDisposerPhase::Envelope] {
        let demand = store.close_cursor_demands(phase, 7).unwrap();
        let grant = RetainedCloneGrant { maximum_copy_bytes: 7, maximum_release_bytes: 0, ..physical_test_close_grant() };
        let (admitted, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| store.close_cursor_admit(phase, grant));
        let (owner, receipt) = admitted.unwrap();
        assert_eq!((requested, released), (receipt.retained_capacity_bytes, receipt.released_bytes));
        assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
        drive_retirement_terminal(owner.expect("current and whole envelope transfer exactly once"));
        assert!(store.close_cursor_admit(phase, grant).unwrap().0.is_none());
    }
    assert!(store.live.current_detached && store.log.envelope_detached);
    close_test_store(&mut store);
}


#[test]
fn rejected_history_mutation_and_metadata_owners_close_under_one_item_grants() {
    let edit = Edit {
        line: None,
        id: "rejected-edit".repeat(32),
        actor: Some("actor".repeat(64).into()),
        forwards: vec![DemoMutation::SetN(SetN { n: 7 })],
        inverse: vec![DemoMutation::SetN(SetN { n: 3 })].into(),
        mutation_meta: Vec::new(),
        verb: None,
        sequence_number: 1,
        started_at: "started".repeat(64),
        finished_at: Some("finished".repeat(64)),
    };
    drive_retirement_terminal(Box::new(ArtifactStoreDecodedEditRetirement::new(edit, Arc::new(DemoMutationRetirementFactory))));
    drive_retirement_terminal(Box::new(ArtifactStoreHistoryMetadataRetirement::change(Change {
        id: "change".repeat(64),
        edit_ids: vec!["edit-a".repeat(64), "edit-b".repeat(64)],
        description: Some("metadata".repeat(64)),
        saved_at: "timestamp".repeat(64),
    })));
}


/// 📏️ Closes an erased member publication on its exact quoted per-axis demands.
fn close_erased_publication_quoted(publication: &mut dyn ErasedMemberStoreOneItemPublication) {
    publication.begin_close();
    for _ in 0..65_536 {
        if publication.terminal_is_empty() {
            return;
        }
        let demand = publication.retirement_demands(4096).expect("current member publication close frontier");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = publication.close_step(grant).expect("member publication closes on its quoted grant");
        assert!(step.progress().fits(grant));
    }
    panic!("member publication did not retire its exact owners");
}


#[semio_framework_async_macros::async_test]
async fn member_owned_batch_admission_keeps_exact_source_on_wrong_type_stale_revision_and_metadata_refusal() {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut member = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "owned-batch", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    member.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
    let make_request = |mutations| MemberStoreOwnedBatchRequest { operation: semio_framework_job::OperationId(715), expected_generation: member.generation_now(), expected_revision: member.content_revision_now(), actor: fixture["metadata"]["actor"].as_str().unwrap().into(), group_id: Some(fixture["metadata"]["groupId"].as_str().unwrap().into()), transaction: None, mutations };
    let wrong: Vec<i32> = serde_json::from_value(fixture["wrongType"].clone()).unwrap();
    let wrong_pointer = wrong.as_ptr();
    let (wrong, _) = MemberStoreOwnedBatch::try_new(wrong, grant).unwrap_or_else(|_| panic!("funded wrong-type owner"));
    let mut wrong = make_request(wrong);
    assert!(member.begin_one_item_owned_publication(&mut wrong, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).is_err());
    assert_eq!(wrong.mutations.mutations::<i32>().unwrap().as_ptr(), wrong_pointer);
    assert_eq!(serde_json::to_value(wrong.mutations.mutations::<i32>().unwrap()).unwrap(), fixture["wrongType"]);
    for _ in 0..1024 { if wrong.mutations.terminal_is_empty() { break; } assert!(wrong.mutations.close_granted(grant).unwrap().progress().fits(grant)); }
    assert!(wrong.mutations.terminal_is_empty());
    let values = fixture["values"].as_array().unwrap().iter().map(|row| DemoMutation::SetN(SetN { n: row.as_i64().unwrap() as i32 })).collect::<Vec<_>>();
    let pointer = values.as_ptr();
    let (values, _) = MemberStoreOwnedBatch::try_new(values, grant).unwrap_or_else(|_| panic!("funded exact mutation vector"));
    let mut request = make_request(values);
    request.expected_generation += 1;
    assert!(member.begin_one_item_owned_publication(&mut request, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).is_err());
    assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().as_ptr(), pointer);
    request.expected_generation -= 1;
    request.expected_revision[0] ^= 1;
    assert!(member.begin_one_item_owned_publication(&mut request, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).is_err());
    assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().as_ptr(), pointer);
    request.expected_revision[0] ^= 1;
    let original_actor=std::mem::take(&mut request.actor);
    assert!(member.begin_one_item_owned_publication(&mut request, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).is_err());
    assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().as_ptr(), pointer);
    request.actor = original_actor;
    for (index, mutation) in request.mutations.mutations::<DemoMutation>().unwrap().iter().enumerate() {
        match mutation { DemoMutation::SetN(row) => assert_eq!(row.n as i64, fixture["values"][index].as_i64().unwrap()), _ => panic!("exact mutation variant/order changed") }
    }
    let mut publication = member.begin_one_item_owned_publication(&mut request, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).expect("same original owner admits after exact authority restoration").expect("funded publication birth");
    assert!(request.mutations.mutations::<DemoMutation>().is_none());
    assert_eq!(member.snapshot_ref().n, Some(0));
    close_erased_member_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() });
    for _ in 0..1024 { if request.mutations.terminal_is_empty() { break; } assert!(request.mutations.close_granted(grant).unwrap().progress().fits(grant)); }
    assert!(request.mutations.terminal_is_empty());
    assert_eq!(member.snapshot_ref().n, Some(0));
    for _ in 0..65_536 { if matches!(SpaceMember::close_owned_step(&mut member, physical_test_close_grant()).unwrap(), RetainedCloneStep::Complete(_)) { break; } }
    assert!(SpaceMember::close_owned_terminal_is_empty(&member));
    println!("[DEBUG] Exact typed member source admission retained original allocation/order through wrong type, generation, revision and metadata refusal; fresh candidate cancelled without publication");
}
