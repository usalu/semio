//! 🧪️ Native S composition laws retain the original rich store and exact registered roster.
use super::*;
#[path="../🚪️io/🧪️tests/🦀️.rs"]pub(crate) mod archive_laws;
#[path="🪆️content/🦀️.rs"]mod content_laws;
#[path="✏️editor/🦀️.rs"]mod editor_laws;
#[path="🗂️layout/🦀️.rs"]mod layout_laws;
use semio_s_artifact_trinity_jack_graph_query::{JackContentVisual, JackGraphQuery};
use jack::standards::v1::subsets::any::schema::content_read::JackContentProjection;
use pack::snapshot::ArtifactPack;

fn rich_snapshot(parent:&jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot)->SemioGraphSnapshot{
 let owner=jack::standards::v1::subsets::any::schema::content::jack_content_for_handle(&parent.content).expect("retained original owner");
 SemioGraphSnapshot::decode_pack(&owner.read().genesis_pack().expect("original pack")).expect("original rich pack")
}

#[test]
fn native_admission_refusal_returns_the_original_rich_owner(){
 let fixture:semio_framework_pack_json::Value=semio_framework_pack_json::from_str(include_str!("../../../../🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🧫️fixtures/🪆️query/🔣️.json")).unwrap();
 let identity:jack::standards::v1::subsets::any::schema::content::JackContentChild=semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&fixture["identity"])).unwrap();
 let parent=SemioJackComposition::initial_snapshot();
 let original=rich_snapshot(&parent);
 let pack=original.encode_pack();
 for coordinate in 0..5{
  let exact=jack::standards::v1::subsets::any::schema::content::JackContentChild::new(identity.child_id.clone(),identity.target.clone());
  let mut selected=jack::standards::v1::subsets::any::schema::content::JackContentChild::new(identity.child_id.clone(),identity.target.clone());
  match coordinate{0=>selected.child_id.push('!'),1=>selected.target.artifact_id.push('!'),2=>selected.target.dialect.artifact_kind.push('!'),3=>selected.target.dialect.standard.push('!'),4=>selected.target.dialect.subset.push('!'),_=>unreachable!()}
  let source=Box::new(content::SemioJackOwnedContent::new(exact,original.clone()));
  let failure=jack::standards::v1::subsets::any::schema::content::jack_content_with_owner(selected,source).err().expect("different exact coordinate refuses admission");
  let(error,returned,source)=failure.into_parts();
  assert_eq!(source.genesis_pack().unwrap(),pack);
  assert_ne!(jack::standards::v1::subsets::any::schema::content::jack_content_identity(&returned),source.identity());
  let exact=jack::standards::v1::subsets::any::schema::content::JackContentChild::new(identity.child_id.clone(),identity.target.clone());
  let retained=jack::standards::v1::subsets::any::schema::content::jack_content_with_owner(exact,source).expect("original source admits its original address");
  jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(retained).unwrap();
  jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(returned).unwrap();
  jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(error).unwrap();
 }
 jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(original).unwrap();
 jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(parent).unwrap();
 jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(identity).unwrap();
}

#[test]
fn native_composition_retains_original_rich_genesis(){
 use semio_s_artifact_stdio_semio_graph::schema::snapshot::{SemioGraphNode,SemioGraphPort,SemioGraphPortKind,GraphNodeId};
 use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
 use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
 use semio_s_artifact_stdio_semio_value_model::schema::snapshot::ValueId;
 let fixture:semio_framework_pack_json::Value=semio_framework_pack_json::from_str(include_str!("../../../../🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🧫️fixtures/👁️rich-read/🔣️.json")).unwrap();
 let node=&fixture["nodes"][0];
 let rich=SemioGraphSnapshot{nodes:vec![SemioGraphNode{id:GraphNodeId::new(node["id"].as_str().unwrap()),kind:"rich".into(),label:String::new(),ports:vec![SemioGraphPort{name:"both@port".into(),kind:SemioGraphPortKind::InOut,category:"rich".into(),properties:vec![]}],properties:vec![SemioValueEntry{key:"int".into(),value:SemioValue::Int{lexeme:"18446744073709551615".into()}},SemioValueEntry{key:"bytes".into(),value:SemioValue::Bytes{value:vec![0,255]}},SemioValueEntry{key:"reference".into(),value:SemioValue::Ref{id:ValueId::new("original")}}],..Default::default()}],..Default::default()};
 let original_pack=SemioGraphSnapshot::encode_pack(&rich);
 let content=content::jack_content_child_with_snapshot(rich);
 let parent=jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot{content,..Default::default()};
 let owner=jack::standards::v1::subsets::any::schema::content::jack_content_for_handle(&parent.content).unwrap();
 assert_eq!(owner.read().genesis_pack().unwrap(),original_pack);
 assert_eq!(owner.read().node_view(0).unwrap().presented_label(),node["expectedLabel"].as_str().unwrap());
 assert!(owner.read().port_view(0,0).unwrap().inbound&&owner.read().port_view(0,0).unwrap().outbound);
 assert_eq!(owner.read().identity(),jack::standards::v1::subsets::any::schema::content::jack_content_identity(&parent.content));
 assert!(owner.read().query_node(0,4096).is_err());
 drop(owner);
 jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(parent).unwrap();
}

#[test]
fn native_composition_parameterizes_actual_registered_members(){
 let mut original_caller=semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();
 let mut original_context=original_caller.context();
 let cx=&mut original_context;
 use semio_framework_plugin::{artifact_app_laws,App,EditorApp,ArtifactApp};
 use store::SpaceMember;
 type Editor=EditorApp<jack::editor::jack::TrinityJackPlayApp<SemioJackComposition>>;
 let mut app=semio_framework_async::poll::resolve_ready(artifact_app_laws::new_app_with_registry_and_members::<Editor,semio_s_artifact_stdio_semio::SemioMembers>(||App{definition:jack::editor::jack::create_trinity_jack_app(),examples:Vec::new()},store::ActorId(store::LOCAL_ACTOR_ID.into()),jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy(),&mut artifact_app_laws::fixture_identity()));
 let parent=app.snapshot().unwrap();
 let child=semio_framework_async::poll::resolve_ready(app.child_store("content",&parent.content.child_id)).expect("actual rich member store");
 let bytes=semio_framework_async::poll::resolve_ready(child.document_pack_bytes()).expect("actual rich child pack");
 let rich=SemioGraphSnapshot::decode_pack(&bytes).unwrap();
 assert_eq!((rich.nodes.len(),rich.edges.len()),(9,6));
 assert_eq!(rich.nodes[0].label,"b");
 jack::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(rich).unwrap();
 drop(child);
 artifact_app_laws::close_registered_fixture_app(&mut app,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy(),cx).expect("original paid close");
 let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).unwrap();
}

#[test]
fn native_action_ingress_retains_original_sources_through_paid_close(){
 use semio_framework_plugin::action_ingress::{ActionIngressRecord,ActionIngressSource};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as heap;
 fn data(value:&semio_framework_value::DslValue)->usize{match value{semio_framework_value::DslValue::Object(fields)=>fields.as_ptr()as usize,_=>panic!("closed original fixture must be an object")}}
 fn original(value:&serde_json::Value)->semio_framework_value::DslValue{semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::from_str::<semio_framework_pack_json::Value>(&value.to_string()).unwrap())}
 fn close(owner:&mut ActionIngressRecord,cx:&mut semio_framework_job::StepContext<'_>){for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if owner.terminal_is_empty(){break}let(result,physical)=heap(||{owner.close_original_step(cx)});let posted=result.unwrap().progress();assert_eq!((physical.requested_bytes,physical.released_bytes),(posted.retained_capacity_bytes,posted.released_bytes));}assert!(owner.terminal_is_empty())}
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/📥️ingress/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let mut caller=semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut context=caller.context();let cx=&mut context;let recipient=cx.original_recipient_identity();
 for case in fixture["cases"].as_array().unwrap(){
  let raw=&case["source"];let fault=if case["takeFault"].as_bool().unwrap(){let raw=&raw["fault"];let mut fault=Fault::new(FaultOrigin::Extension,FaultCode::new(raw["code"].as_str().unwrap()),raw["message"].as_str().unwrap()).with_domain(original(&raw["domain"])).unwrap();fault.causes=raw["causes"].as_array().unwrap().iter().map(|cause|semio_framework::FaultCause{message:cause["message"].as_str().unwrap().into(),code:Some(FaultCode::new(cause["code"].as_str().unwrap())),retained_progress:None}).collect();Some(fault)}else{None};
  let mut source=Some(ActionIngressSource{action:None,prepared:Some(original(&fixture["prepared"]["projection"])),assembled:None,args:Some(original(&raw["args"])),fault});
  let original_args=data(source.as_ref().unwrap().args.as_ref().unwrap());let prepared=data(source.as_ref().unwrap().prepared.as_ref().unwrap());let original_fault=source.as_ref().unwrap().fault.as_ref().map(|fault|(data(fault.domain.as_ref().unwrap()),fault.causes.as_ptr()as usize));
  let mut original_caller=None;let before=cx.retained_progress();let(result,physical)=heap(||cx.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_items:0,..cx.retained_grant()},|same|ActionIngressRecord::receive(&mut source,&mut original_caller,same).map(|record|record.is_none())));assert!(result.unwrap());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(data(source.as_ref().unwrap().args.as_ref().unwrap()),original_args);
  let(result,physical)=heap(||ActionIngressRecord::receive(&mut source,&mut original_caller,cx));let mut owner=result.unwrap().unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert!(source.is_none());
  let mut foreign=semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let foreign_context=foreign.context();assert_eq!((foreign_context.operation(),foreign_context.generation()),(cx.operation(),cx.generation()));let(refusal,physical)=heap(||owner.validate_context(&foreign_context));let refusal=refusal.expect_err("same coordinate foreign roots cannot own the admitted original source");assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));let(_,physical)=heap(||drop(refusal));assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));let fuel=foreign_context.fuel_remaining();drop(foreign_context);foreign.retire(fuel).unwrap();
  let(args,fault)=owner.parser_slots().unwrap();assert_eq!(data(args.as_ref().unwrap()),prepared);let returned=fault.take();if let Some(fault)=returned.as_ref(){assert_eq!((data(fault.domain.as_ref().unwrap()),fault.causes.as_ptr()as usize),original_fault.unwrap());assert_eq!(fault.causes[0].message,raw["fault"]["causes"][0]["message"].as_str().unwrap());}
  for row in fixture["parserSettlement"].as_array().unwrap(){let(loan,physical)=heap(||owner.loan_source(cx));let loan=loan.unwrap().expect("funded original parser header loan");assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(data(loan.args.as_ref().unwrap()),original_args);assert_eq!(data(loan.prepared.as_ref().unwrap()),prepared);let terminal=row["terminal"].as_bool().unwrap();let(_,physical)=heap(||owner.return_source(std::mem::ManuallyDrop::into_inner(loan),terminal));assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(owner.ready(),terminal);if !terminal{assert!(!owner.terminal_is_empty());assert_eq!(data(owner.parser_slots().unwrap().0.as_ref().unwrap()),prepared);}}
  let(zero,physical)=heap(||cx.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_items:0,..cx.retained_grant()},|same|owner.close_original_step(same)));assert_eq!(zero.unwrap().progress(),Default::default());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));close(&mut owner,cx);let(_,physical)=heap(||drop(owner));assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));
  if let Some(fault)=returned{let mut source=Some(ActionIngressSource{action:None,args:None,prepared:None,assembled:None,fault:Some(fault)});let mut owner=ActionIngressRecord::receive(&mut source,&mut original_caller,cx).unwrap().unwrap();owner.finish_parser();close(&mut owner,cx);let(_,physical)=heap(||drop(owner));assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));}
  assert_eq!(cx.original_recipient_identity(),recipient);
 }
 let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).unwrap();
}

#[test]
fn native_parsed_command_moves_original_address_backing_without_allocation(){
 use semio_framework_plugin::action_ingress::{OriginalParsedCommandKind,OriginalParsedCommandSource};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as heap;
 use semio_framework_value::DslValue;
 use store::os_store::snapshot_read::{artifact_retirement_owned_birth_demands,artifact_retirement_admit_owned,artifact_retirement_box_demands,artifact_retirement_box_close_step};
 fn data(value:&DslValue)->usize{match value{DslValue::Object(fields)=>fields.as_ptr()as usize,_=>panic!("original arguments are an object")}}
 fn text(value:&DslValue)->usize{match value{DslValue::String(text)=>text.as_ptr()as usize,_=>panic!("original address field is text")}}
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/📥️ingress/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let mut caller=semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut context=caller.context();let cx=&mut context;
 let mut issuer=semio_framework_job_native_decode::OriginalNativeDecodeSource::empty(semio_framework_job_native_decode::OriginalNativeDecodePolicy{maximum_owned_bytes:fixture["nativeParsed"]["maximumOwnedBytes"].as_u64().unwrap()as usize});
 for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if issuer.admit_step(cx).unwrap(){break}}
 for row in fixture["parsed"].as_array().unwrap(){
  let original=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::from_str::<semio_framework_pack_json::Value>(&row["envelope"].to_string()).unwrap());
  let kind=if row["kind"]=="artifact"{OriginalParsedCommandKind::Artifact}else{OriginalParsedCommandKind::Command};let key=if row["kind"]=="artifact"{"args"}else{"arguments"};
  let arguments=data(original.get(key).unwrap());let address=original.get("address");let identity=address.map(|address|text(address.get(if address.get("windowInstanceId").is_some(){"pluginId"}else{"commandId"}).unwrap()));
  let mut source=OriginalParsedCommandSource::new(kind);source.envelope=Some(original);
  let maximum_work=cx.fuel_remaining();let before=cx.retained_progress();let(result,physical)=heap(||cx.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_items:0,..cx.retained_grant()},|same|issuer.receive_step(same,|control,grant|source.prepare(control,grant,maximum_work)).map(|outcome|outcome.is_none())));assert!(result.unwrap());assert_eq!(cx.retained_progress(),before);assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert!(source.args.is_none());
  let(outcome,physical)=heap(||issuer.receive_step(cx,|control,grant|source.prepare(control,grant,0)));let outcome=outcome.unwrap().unwrap();assert!(outcome.receiving_refusal.is_none());assert!(!outcome.value.unwrap());assert_eq!(outcome.progress,Default::default());assert_eq!(outcome.work,0);assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert!(source.args.is_none());
  let(outcome,physical)=heap(||issuer.receive_step(cx,|control,grant|source.prepare(control,grant,maximum_work)));let outcome=outcome.unwrap().unwrap();assert!(outcome.receiving_refusal.is_none());assert!(outcome.value.unwrap());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!((outcome.progress.retained_capacity_bytes,outcome.progress.released_bytes),(0,0));assert_eq!(data(source.args.as_ref().unwrap()),arguments);
  if let Some(identity)=identity{let projected=source.action_address.as_ref().map(|address|address.plugin_id.as_ptr()as usize).or_else(||source.command_address.as_ref().map(|address|address.command_id.as_ptr()as usize)).unwrap();assert_eq!(projected,identity);}
  let mut pending=Some(source);let mut retirement=None;
  for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if pending.is_none()&&retirement.is_none(){break}let demand=if pending.is_some(){artifact_retirement_owned_birth_demands(&pending).unwrap()}else{artifact_retirement_box_demands(&retirement,cx.retained_grant().maximum_copy_bytes).unwrap()};let(result,physical)=heap(||semio_framework_job::close_original_retained_step(cx,demand,|grant|if pending.is_some(){artifact_retirement_admit_owned(&mut pending,&mut retirement,grant)}else{artifact_retirement_box_close_step(&mut retirement,grant)}));let progress=result.unwrap().progress();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
  assert!(pending.is_none()&&retirement.is_none());
 }
 for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if issuer.terminal_is_empty(){break}let(result,physical)=heap(||issuer.close_step(cx));let progress=result.unwrap().progress();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
 assert!(issuer.terminal_is_empty());drop(issuer);let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).unwrap();
}

#[test]
fn native_jack_record_reconstruction_moves_original_fields_through_paid_close(){
 use jack::standards::v1::subsets::any::io::binary::snapshot::receiving::{JackRecordReconstruction,JackRecordReconstructionPolicy};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as heap;
 use semio_framework_value::{RetainedCloneProgress,ToValue};
 use store::os_store::snapshot_read::{artifact_retirement_owned_birth_demands,artifact_retirement_admit_owned,artifact_retirement_box_demands,artifact_retirement_box_close_step};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🫴️receiving/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let mut expected=SemioJackComposition::initial_snapshot();
 expected.name=fixture["source"]["name"].as_str().unwrap().into();
 expected.manifest.node_kinds.push(jack::standards::v1::subsets::any::schema::manifest::NodeKindDef{name:"nested original".into(),properties:vec![semio_framework_graph_manifest::PropertyDef{name:"schema original".into(),kind:semio_framework_graph_manifest::PropertyKind::Derived,value_type:semio_framework_value::ValueType::List(Box::new(semio_framework_value::ValueType::Schema("s.fixture.original".into()))),expr:Some("original expression".into())}],port_kinds:vec!["original port".into()]});
 let bytes=expected.encode_pack();let(_,body)=store::semio_format::unwrap_binary(&bytes).unwrap();let(record,_)=store::pack_rt::decode_document(&body,&jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot::record_spec().unwrap(),&Default::default()).unwrap();
 let semio_framework_dsl_record::FieldValue::Text(text)=record.get(1).unwrap()else{panic!("original native name is text")};let pointer=text.as_ptr()as usize;
 let mut source=Some(record);let policy=&fixture["nativePolicy"];let mut cursor=JackRecordReconstruction::empty(JackRecordReconstructionPolicy{maximum_rows:policy["maximumRows"].as_u64().unwrap()as usize,maximum_type_depth:policy["maximumTypeDepth"].as_u64().unwrap()as usize});
 let mut caller=semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut context=caller.context();let cx=&mut context;
 let before=cx.retained_progress();let(posted,physical)=heap(||cx.with_retained_loan(semio_framework_value::RetainedCloneGrant{maximum_items:0,..cx.retained_grant()},|same|cursor.receive(&mut source,same.retained_grant())));assert_eq!(posted.unwrap(),RetainedCloneProgress::default());assert!(source.is_some());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);
 let(posted,physical)=heap(||cursor.receive(&mut source,cx.retained_grant()));let posted=posted.unwrap();cx.consume_retained(posted).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(posted.retained_capacity_bytes,posted.released_bytes));assert!(source.is_none());
 let mut issuer=semio_framework_job_native_decode::OriginalNativeDecodeSource::empty(semio_framework_job_native_decode::OriginalNativeDecodePolicy{maximum_owned_bytes:cx.retained_grant().maximum_capacity_bytes});for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if issuer.admit_step(cx).unwrap(){break}}
 let(outcome,physical)=heap(||issuer.receive_step(cx,|native,grant|cursor.step(native,grant,0)));let outcome=outcome.unwrap().unwrap();assert!(outcome.receiving_refusal.is_none());assert!(!outcome.value.unwrap());assert_eq!(outcome.progress,Default::default());assert_eq!(outcome.work,0);assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));
 for _ in 0..3{let fuel=cx.fuel_remaining();let(outcome,physical)=heap(||issuer.receive_step(cx,|native,grant|cursor.step(native,grant,1)));let outcome=outcome.unwrap().unwrap();assert!(outcome.receiving_refusal.is_none());assert!(!outcome.value.unwrap());assert_eq!(outcome.work,1);assert_eq!(cx.fuel_remaining(),fuel-1);assert_eq!(outcome.progress,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert!(cursor.candidate().is_none());let record=cursor.original_record().expect("partial shape keeps original record at receiving entrance");let semio_framework_dsl_record::FieldValue::Text(text)=record.get(1).unwrap()else{unreachable!()};assert_eq!(text.as_ptr()as usize,pointer);}
 let mut turns=0;while !cursor.is_ready(){assert!(turns<semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns());let(outcome,physical)=heap(||issuer.receive_step(cx,|native,grant|cursor.step(native,grant,fixture["maximumWorkPerStep"].as_u64().unwrap())));if let Some(outcome)=outcome.unwrap(){assert!(outcome.receiving_refusal.is_none());outcome.value.unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(outcome.progress.retained_capacity_bytes,outcome.progress.released_bytes));assert!(outcome.work<=1);}turns+=1;}
 assert_eq!(cursor.candidate().unwrap().name.as_ptr()as usize,pointer);assert_eq!(cursor.candidate().unwrap().to_value(),expected.to_value());
 let candidate=cursor.candidate().unwrap()as *const _ as usize;let(posted,physical)=heap(||cursor.close_step(Default::default()));assert_eq!(posted.unwrap().progress(),Default::default());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(cursor.candidate().unwrap()as *const _ as usize,candidate);
 for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if cursor.terminal_is_empty(){break}let demand=cursor.close_demands(cx.retained_grant().maximum_copy_bytes).unwrap();let(step,physical)=heap(||semio_framework_job::close_original_retained_step(cx,demand,|grant|cursor.close_step(grant)));let progress=step.unwrap().progress();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}assert!(cursor.terminal_is_empty());drop(cursor);
 for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if issuer.terminal_is_empty(){break}issuer.close_step(cx).unwrap();}assert!(issuer.terminal_is_empty());drop(issuer);
 let mut expected=Some(expected);let mut closing=None;for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if expected.is_none()&&closing.is_none(){break}let demand=if expected.is_some(){artifact_retirement_owned_birth_demands(&expected).unwrap()}else{artifact_retirement_box_demands(&closing,cx.retained_grant().maximum_copy_bytes).unwrap()};semio_framework_job::close_original_retained_step(cx,demand,|grant|if expected.is_some(){artifact_retirement_admit_owned(&mut expected,&mut closing,grant)}else{artifact_retirement_box_close_step(&mut closing,grant)}).unwrap();}
 let fuel=cx.fuel_remaining();drop(context);caller.retire(fuel).unwrap();
}
