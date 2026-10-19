/// 🏷️ Catalog staging, command refusal and direct fixture laws: the original catalog constructor and member group staging, the one item grant, the canonical command and pack refusals and the direct store fixture cases.

#[test]
fn original_catalog_constructor_stages_retain_original_sources_and_conserve_native_heap(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🔌️plugin/⏪️time-travel/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let policy=&corpus["constructorGrant"];let n=|key:&str|policy[key].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 let (admitted,source_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bounded_artifact_store_owners::<DemoSnapshot,DemoMutation>(grant));let(mut owners,source_receipt)=admitted.unwrap_or_else(|_|panic!("independent constructor authority admits original sources"));assert!(source_receipt.fits(grant));assert_eq!(source_heap.requested_bytes,source_receipt.retained_capacity_bytes);assert_eq!(source_heap.released_bytes,source_receipt.released_bytes);let mut births=source_heap.requested_bytes;let mut releases=source_heap.released_bytes;let mut turns=0;
 while !owners.constructor_is_complete(){assert!(turns<512,"original constructor must progress under unchanged independent authority");let pointers=||[owners.snapshot_retirement.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize),owners.initial_snapshot_retirement.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize),owners.mutation_retirement.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize)];let original=pointers();
  for axis in 0..5{let denied=match axis{0=>RetainedCloneGrant{maximum_items:0,..grant},1=>RetainedCloneGrant{maximum_copy_bytes:0,..grant},2=>RetainedCloneGrant{maximum_capacity_bytes:0,..grant},3=>RetainedCloneGrant{maximum_release_bytes:0,..grant},_=>RetainedCloneGrant{maximum_depth:0,..grant}};let demand=owners.constructor_demands(grant.maximum_copy_bytes).unwrap();if axis==1&&demand.copy_bytes==0||axis==2&&demand.capacity_bytes==0||axis==3&&demand.release_bytes==0{continue}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.admit_constructor(denied));assert_eq!(heap.requested_bytes,0,"denied original constructor axis {axis}");assert_eq!(heap.released_bytes,0,"denied original constructor axis {axis}");let receipt=match result{Ok(receipt)=>receipt,Err((_,receipt))=>receipt};assert_eq!(receipt,RetainedCloneProgress::default());let current=[owners.snapshot_retirement.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize),owners.initial_snapshot_retirement.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize),owners.mutation_retirement.as_ref().map(|v|Arc::as_ptr(v)as*const()as usize)];assert_eq!(current,original);}
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.admit_constructor(grant));let receipt=result.unwrap_or_else(|_|panic!("unchanged independent constructor policy must admit next original stage"));assert!(receipt.fits(grant));assert!(receipt.copied_items>0);assert_eq!(heap.requested_bytes,receipt.retained_capacity_bytes);assert_eq!(heap.released_bytes,receipt.released_bytes);births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;
 }
 while !owners.uninstalled_owners_terminal_is_empty(){assert!(turns<4096,"original uninstalled catalog must close under independent authority");let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.close_uninstalled_owners_step(grant));let step=step.unwrap();let receipt=step.progress();assert!(receipt.fits(grant));assert!(receipt.copied_items>0||matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(heap.requested_bytes,receipt.retained_capacity_bytes);assert_eq!(heap.released_bytes,receipt.released_bytes);births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;}
 let(_,terminal)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owners));assert_eq!(terminal.requested_bytes,0);assert_eq!(terminal.released_bytes,0);assert_eq!(births,releases);
}

#[semio_framework_async_macros::async_test]
async fn original_member_group_stage_and_adoption_preserve_physical_receipts_and_visibility(){
 fn original_heap<T>(action:impl FnOnce()->T)->(T,(usize,usize)){let(value,requested,released)=semio_framework_trace_memory_testing::observe_backing(action);(value,(requested,released))}
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧩️composition/📬️publication/🤝️group/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
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
  let diagnostic_rows:Vec<_>=fixture["diagnostics"].as_array().unwrap().iter().map(|row|semio_framework_os_spr::MutationMessage{level:semio_framework_diagnostic::Severity::from_value(row["level"].clone().into()).unwrap(),code:row["code"].as_str().unwrap().to_owned().into(),message:row["message"].as_str().unwrap().to_owned(),target:row["target"].as_array().unwrap().iter().map(|target|target.as_str().unwrap().to_owned()).collect(),op_index:Some(row["opIndex"].as_u64().unwrap()as u32)}).collect();let diagnostic_pointers:Vec<_>=diagnostic_rows.iter().map(|row|row.message.as_ptr()).collect();let mut diagnostic_owner=semio_framework_os_spr::command::MutationMessagePublication::new(diagnostic_rows);while !diagnostic_owner.ready(){diagnostic_owner.advance(grant.retained_grant()).unwrap();}stage.diagnostics=super::ArtifactStoreDiagnosticStage::from_publication(Some(diagnostic_owner));
publication.stage=Some(Box::new(stage));publication.phase=ArtifactStoreOneItemPublicationPhase::Publishing;
 let original_stage=std::ptr::from_ref(publication.stage.as_ref().unwrap().as_ref());let original_flags=publication.stage.as_ref().unwrap().unit_flags.as_ptr();
 let mut publisher=semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();let visibility=publisher.view();
 publication.group_preparation=Some(semio_framework_os_store_one_item_batch::lifted_member_group::MemberGroupPreparation::new(&visibility,None,None));
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
  assert_eq!(store.messages_for_edit(id).iter().map(|row|row.message.as_ptr()).collect::<Vec<_>>(),diagnostic_pointers);assert_eq!(store.pending_report.messages.len(),1);assert_eq!(store.pending_report.messages[0].edit_id,id);assert_eq!(store.pending_report.messages[0].messages,store.messages_for_edit(id));assert_eq!(store.pending_report.worst,Some(semio_framework_diagnostic::Severity::Warning));for(original,report)in store.messages_for_edit(id).iter().zip(&store.pending_report.messages[0].messages){assert_ne!(original.message.as_ptr(),report.message.as_ptr());}eprintln!("[DEBUG] Original group diagnostic publication operation=919 generation={} edit={} rows={} worst=warning beforeACK=true",store.generation,id,store.messages_for_edit(id).len());
assert!(publication.acknowledge());
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if publication.terminal_is_empty(){break;}let(step,heap)=original_heap(||publication.close_step(grant.retained_grant()).unwrap());let receipt=step.progress();assert!(receipt.fits(grant.retained_grant()));assert_eq!((heap.0,heap.1),(receipt.retained_capacity_bytes,receipt.released_bytes));}
 assert!(publication.terminal_is_empty());let((),heap)=original_heap(||drop(publication));assert_eq!((heap.0,heap.1),(0,fixture["law"]["terminalDropBytes"].as_u64().unwrap()as usize));
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
}

#[test]
fn command_decode_canonical_pack_refusal_retains_actual_reader_and_operation_sources(){
 let mut reader=pack::ByteReader::new(&[0x80]);let refusal=reader.read_varint_u64().unwrap_err();let kind=refusal.kind();
 let command=CommandDecodeError::from(refusal);let CommandDecodeError::Layout(semio_framework_os_spr::ProtocolError::Pack(error))=&command else{panic!("actual primitive layout cause");};assert_eq!(error.refusal_kind(),Some(kind));
 let source=std::error::Error::source(&command).unwrap().downcast_ref::<semio_framework_os_spr::ProtocolError>().unwrap();assert!(std::ptr::eq(source,match &command{CommandDecodeError::Layout(error)=>error,_=>unreachable!()}));
 let mut reader=pack::ByteReader::new(&[]);let refusal=reader.read_u8().unwrap_err();let operation=CommandDecodeError::Operation{index:7,error:refusal.into()};
 let CommandDecodeError::Operation{index,error}=&operation else{unreachable!()};assert_eq!(*index,7);assert!(std::ptr::eq(error,std::error::Error::source(&operation).unwrap().downcast_ref::<semio_framework_os_spr::ProtocolError>().unwrap()));
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
}


fn direct_fixture_cases() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️.json")).unwrap()
}

fn assert_fixture_case<Op>(row: &serde_json::Value)
where
    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + ColdMutation<DemoSnapshot,ReferenceDiff=DemoDiff> + ToValue + FromValue + OpText + OpBinary + PartialEq + std::fmt::Debug,
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
    let actual = protocol::apply_diff_cold(&diff, &before).unwrap();
    let decoded_diff = serde_json::from_value::<DemoDiff>(serde_json::to_value(&diff).unwrap()).unwrap();
    assert_eq!(protocol::apply_diff_cold(&decoded_diff, &before).unwrap(), actual);
    assert_eq!(actual.n, serde_json::from_value::<Option<i32>>(row["after"].clone()).unwrap());
    let level = messages.first().map(|message| format!("{:?}", message.level).to_lowercase());
    assert_eq!(level.as_deref(), row["level"].as_str());
    let mut restored = actual;
    for inverse in mutation.inverse(&before).expect("valid retained mutation inverse fixture").into_iter().rev() {
        restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap();
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
    Op: Mutation<DemoSnapshot, Diff = DemoDiff> + ColdMutation<DemoSnapshot,ReferenceDiff=DemoDiff> + ToValue + FromValue + OpText + OpBinary + PartialEq + std::fmt::Debug,
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
    let deleted = protocol::apply_diff_cold(mutation.diff(&before).diff(), &before).unwrap();
    assert_eq!(deleted.n, None);
    assert_eq!(protocol::apply_diff_cold(mutation.inverse(&before).expect("valid retained mutation inverse fixture")[0].diff(&deleted).diff(), &deleted).unwrap(), before);
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
            actual = protocol::apply_diff_cold(&diff, &actual).unwrap();
            combined.absorb(diff);
        }
        let expected = DemoSnapshot { n: serde_json::from_value(row["after"].clone()).unwrap() };
        assert_eq!(actual, expected);
        assert_eq!(protocol::apply_diff_cold(&combined, &before).unwrap(), expected);
        let round_trip = serde_json::from_value::<DemoDiff>(serde_json::to_value(&combined).unwrap()).unwrap();
        assert_eq!(protocol::apply_diff_cold(&round_trip, &before).unwrap(), expected);
        let restore = DemoMutation::AssignN(AssignN { n: before.n });
        assert_eq!(protocol::apply_diff_cold(restore.diff(&actual).diff(), &actual).unwrap(), before);
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
