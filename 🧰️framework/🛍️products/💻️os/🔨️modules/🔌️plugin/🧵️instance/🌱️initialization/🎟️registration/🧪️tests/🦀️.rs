//! 🧪️ The actual declared native Window App opens, donates its original allocation and closes under mounted authority.
use crate::fixture;
use semio_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_instance_construction::{OriginalVcsApp,factory::original_instance_factory};
use semio_plugin_instance_initialization::registration::OriginalSingleWindowRegistration;
use semio_framework_plugin_reactor_app::{PluginApp,OriginalPluginAppSource};
use semio_plugin_window_kits::RegisteredAppDefinition;
use semio_framework_os_kernel::NoMembers;
use semio_framework_job::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
type App=fixture::NativeWindowApp;
type Recipient=OriginalVcsApp<App,NoMembers>;
type Registration=OriginalSingleWindowRegistration<App,NoMembers,fixture::Owner>;
const POLICY:semio_plugin_mounted_owner::MountedOwnerPolicyV1=semio_plugin_mounted_owner::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
#[track_caller]
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,action:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->(T,RetainedCloneProgress){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,action));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes),"actual native mounted owner physical receipt {p:?}, caller={}, failure={:?}",std::panic::Location::caller(),result.as_ref().err());(result.unwrap(),p)}
fn cancellation_boundary_reached(boundary:Option<&str>,progress:RetainedCloneProgress)->bool{
 let cursor=semio_framework_plugin_instance_construction::OriginalArtifactInstanceAdmission::<App,NoMembers>::INITIALIZATION_CURSOR_BYTES;
 let(copy,capacity)=match boundary{
 Some("final-allocation-before-service-initialization")=>(None,semio_framework_plugin_instance_construction::OriginalArtifactInstanceAdmission::<App,NoMembers>::birth_demands().capacity_bytes),
 Some("after-child-service-backing")=>(Some(semio_plugin_instance_app::ChildComposition::<NoMembers>::initial_copy_bytes()+cursor),semio_plugin_instance_app::ChildComposition::<NoMembers>::birth_demands().unwrap().capacity_bytes),
 Some("after-tool-service-backing")=>{let demand=semio_plugin_instance_app::ToolOperations::<App>::initial_field_demands(semio_plugin_instance_app::ToolOperations::<App>::EFFECT_OUTBOX_FIELD,0,POLICY.preparation.maximum_copy_bytes-cursor,true).unwrap();(Some(demand.copy_bytes+cursor),demand.capacity_bytes)},
 Some("during-large-absent-field")=>(Some(POLICY.preparation.maximum_copy_bytes),0),
 None=>return false,Some(_)=>panic!("unknown neutral native cancellation boundary"),
 };progress.retained_capacity_bytes==capacity&&copy.is_none_or(|bytes|progress.copied_bytes==bytes)}
fn exercise_original_native_window(cancel_at:Option<&str>) -> (Option<(String,String)>,bool) {
 let factory=original_instance_factory::<App,NoMembers,Recipient,Registration>();
 let((mut definition,mut registration),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut definition=semio_framework_async::poll::resolve_ready(semio_framework_manifest_app_surface::testkit::app_with(Vec::new(),Vec::new()));definition.id=App::APP_ID.into();definition.controller_id=definition.id.clone();definition.dialect=App::DIALECT.into();definition.io.artifact_schema=App::DOCUMENT_SCHEMA.into();let mut definition=RegisteredAppDefinition::declared(definition,true);let mut registration=(factory.registration)(definition.original_source().unwrap()).unwrap();definition.commit_producer();registration.commit_producer();(definition,registration)});
 let initial=heap.requested_bytes-heap.released_bytes;
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(44711),Generation(78),RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},default_now_us,u64::MAX,1<<30);
 let mut decoder=OriginalJobDecodeOwner::empty();let mut construction:Option<Box<dyn semio_framework_plugin_reactor_app::OriginalPluginAppConstruction<Recipient>>>=None;let mut app:Option<Recipient>=None;let(mut born,mut released)=(0,0);let mut last_progress=(0,RetainedCloneProgress::default());
 for index in 0..1048576{if index<512{eprintln!("[DEBUG] native opening turn={index}");}let(_,p)=turn(&mut caller,&mut decoder,POLICY.preparation,|cx|{if let Some(construction)=construction.as_mut(){let source=definition.original_source().unwrap();construction.advance(OriginalPluginAppSource{definition:&source,definition_source:definition.original_source().unwrap(),registration:registration.as_ref(),actor:"native-window-actor",instance_id:77,policy:POLICY},cx,&mut app)?;}else{(factory.begin)(cx,&mut construction)?;}Ok(())});born+=p.retained_capacity_bytes;released+=p.released_bytes;if p.copied_items>0||p.retained_capacity_bytes>0||p.released_bytes>0{last_progress=(index,p);}if app.is_some()||cancellation_boundary_reached(cancel_at,p){break}assert!(index<1048575,"[DEBUG] actual declared native App opening stalled: construction={} born={} released={} last_progress={:?}",construction.is_some(),born,released,last_progress);}
 if cancel_at==Some("during-large-absent-field"){let cursor=semio_framework_plugin_instance_construction::OriginalArtifactInstanceAdmission::<App,NoMembers>::INITIALIZATION_CURSOR_BYTES;for bytes in [0,cursor-1,cursor]{let grant=RetainedCloneGrant{maximum_copy_bytes:bytes,..POLICY.preparation};let(_,progress)=turn(&mut caller,&mut decoder,grant,|cx|{let source=definition.original_source().unwrap();construction.as_mut().unwrap().advance(OriginalPluginAppSource{definition:&source,definition_source:definition.original_source().unwrap(),registration:registration.as_ref(),actor:"native-window-actor",instance_id:77,policy:POLICY},cx,&mut app)?;Ok(())});assert_eq!(progress,RetainedCloneProgress::default(),"zero/subexact/cursor-only copy grants cannot advance an unfinished absent field");assert!(app.is_none());}}
 assert_eq!(app.is_none(),cancel_at.is_some());
 let identity=app.as_ref().map(|app|{assert_eq!(semio_framework_plugin_reactor_app::PluginAppDelegate::delegate_ref(app).snapshot_ref().zoom,1.25);(app.document_identity().expect("native document identity"),semio_framework_async::poll::resolve_ready(app.app_id()).to_owned())});
 for index in 0..1048576{if construction.as_ref().unwrap().terminal_is_empty(){break}let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|construction.as_mut().unwrap().close_step(cx).map(|_|())));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(index<1048575,"actual native opening retained its original sources after donation");}
 let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|{let d=construction.as_ref().unwrap().boxed_release_demands()?;close_original_retained_step(cx,d,|_|{drop(construction.take());semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(())}));born+=p.retained_capacity_bytes;released+=p.released_bytes;
 if app.is_some(){for index in 0..1048576{if app.as_ref().unwrap().close_terminal_is_empty(){break}let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|{let grant=cx.retained_grant();app.as_mut().unwrap().close_step(cx,grant).map_err(OriginalTurnFailure::from)?;Ok(())}));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(index<1048575,"actual native App retained an original payload or physical Box");}assert!(app.take().unwrap().close_terminal_is_empty());}
 for index in 0..1048576{if registration.terminal_is_empty(){break}let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|registration.close_step(cx).map(|_|())));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(index<1048575,"actual native tool producer retained an instance alias");}
 let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|{let d=registration.boxed_release_demands();close_original_retained_step(cx,d,|_|{drop(registration);semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(())}));born+=p.retained_capacity_bytes;released+=p.released_bytes;
 for index in 0..1048576{if definition.terminal_is_empty(){break}let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|{let d=definition.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|semio_framework_value::RetainedWorkOutcome{result:definition.close_step(g),work:1})?;Ok(())}));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(index<1048575,"actual native definition retained an instance alias");}
 while !decoder.terminal_is_empty(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(POLICY.close,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes),"native decoder close physical receipt {p:?}");result.unwrap();born+=p.retained_capacity_bytes;released+=p.released_bytes;}
 assert_eq!(initial+born,released,"actual declaration and native instance graphs must close physically");caller.retire(POLICY.close);
 (identity,initial+born==released)
}

#[test]
fn actual_native_window_app_opens_and_closes_with_its_original_registered_sources() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🪪️identity.json")).unwrap();
    let first = exercise_original_native_window(None).0.unwrap();
    let second = exercise_original_native_window(None).0.unwrap();
    let actual = serde_json::json!({
        "distinct": first.0 != second.0,
        "sameController": first.1 == law["controllerId"].as_str().unwrap() && second.1 == first.1,
        "firstLength": first.0.len(),
        "secondLength": second.0.len()
    });
    assert_eq!(actual, law["expected"]);
    let cancellation:serde_json::Value=serde_json::from_str(include_str!("🛑️cancellation.json")).unwrap();
    for case in cancellation["cases"].as_array().unwrap(){let cancelled=exercise_original_native_window(Some(case["cancelAt"].as_str().unwrap()));assert_eq!(serde_json::json!({"published":cancelled.0.is_some(),"balanced":cancelled.1}),case["expected"]);}
}
