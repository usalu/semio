//! 🧪️ The actual declared native Window App opens, donates its original allocation and closes under mounted authority.
use crate::fixture;
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_instance_construction::{OriginalVcsApp,factory::original_instance_factory};
use semio_plugin_instance_initialization::registration::OriginalSingleWindowRegistration;
use semio_plugin_reactor_app::{PluginApp,OriginalPluginAppSource};
use semio_plugin_ownership::original_store_genesis::OriginalStoreBirth;
use semio_plugin_window_kits::RegisteredAppDefinition;
use semio_os_kernel::NoMembers;
use semio_framework_job::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
type App=fixture::NativeWindowApp;
type Recipient=OriginalVcsApp<App,NoMembers>;
type Registration=OriginalSingleWindowRegistration<App,NoMembers,fixture::Owner>;
const POLICY:semio_plugin_mounted_owner::MountedOwnerPolicyV1=semio_plugin_mounted_owner::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
#[track_caller]
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,action:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->(T,RetainedCloneProgress){
    recorded_turn(caller,decoder,grant,"",action)
}
#[track_caller]
fn recorded_turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,label:&str,action:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->(T,RetainedCloneProgress){
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,action));
    let progress=caller.receipt();
    assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes),"native mounted owner receipt {progress:?} at {} {label}, failure={:?}",std::panic::Location::caller(),result.as_ref().err());
    (result.unwrap(),progress)
}
fn cancellation_boundary_reached(boundary:Option<&str>,progress:RetainedCloneProgress)->bool{
 let cursor=semio_plugin_instance_construction::OriginalArtifactInstanceAdmission::<App,NoMembers>::INITIALIZATION_CURSOR_BYTES;
 let(copy,capacity)=match boundary{
 Some("final-allocation-before-service-initialization")=>(None,semio_plugin_instance_construction::OriginalArtifactInstanceAdmission::<App,NoMembers>::birth_demands().capacity_bytes),
 Some("after-child-service-backing")=>(Some(semio_plugin_instance_app::ChildComposition::<NoMembers>::initial_copy_bytes()+cursor),semio_plugin_instance_app::ChildComposition::<NoMembers>::birth_demands().unwrap().capacity_bytes),
 Some("after-tool-service-backing")=>{let demand=semio_plugin_instance_app::ToolOperations::<App>::initial_field_demands(semio_plugin_instance_app::ToolOperations::<App>::EFFECT_OUTBOX_FIELD,0,POLICY.preparation.maximum_copy_bytes-cursor,true).unwrap();(Some(demand.copy_bytes+cursor),demand.capacity_bytes)},
 Some("during-large-absent-field")=>(Some(POLICY.preparation.maximum_copy_bytes),0),
 None=>return false,Some(_)=>panic!("unknown neutral native cancellation boundary"),
 };progress.retained_capacity_bytes==capacity&&copy.is_none_or(|bytes|progress.copied_bytes==bytes)}
fn observe_post_capture_maintenance(
    mut app: Recipient,
    caller: &mut OriginalJobFixtureCaller,
    decoder: &mut OriginalJobDecodeOwner,
) -> (Recipient, RetainedCloneProgress) {
    use semio_plugin_job::reserved_job_outcomes::CleanupTurn;
    use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use semio_plugin_plugin_program::AppInstance;
    use semio_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell;
    use semio_plugin_reactor_lifecycle::life_cleanup_pumps::RuntimeLiveCleanupJob;
    use semio_plugin_reactor_lifecycle::life_cleanup_vocabulary::RuntimeMaintenanceStatus;
    use semio_plugin_runtime_instances::inst_cleanup_turns::runtime_live_cleanup_nonterminal_status;

    let id = 77;
    let mut aggregate = RetainedCloneProgress::default();
    let mut birth = semio_plugin_reactor_app::InstanceBirth::default();
    assert!((0..128).any(|_| {
        let (ready, receipt) = turn(caller, decoder, POLICY.preparation, |cx| app.advance_instance_birth(&mut birth, id, "native-window-actor", cx));
        aggregate = aggregate.checked_add(receipt).unwrap();
        ready
    }), "captured app paid its runtime birth");
    for _ in 0..128 {
        if birth.construction.terminal_is_empty() { break; }
        let (_, receipt) = turn(caller, decoder, POLICY.close, |cx| {
            cx.with_original_retirement(|cx| birth.construction.close_step(cx))
        });
        aggregate = aggregate.checked_add(receipt).unwrap();
    }
    assert!(birth.construction.terminal_is_empty());
    let cell = RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, POLICY);
    drop(cell.instance.lock().expect("fixture cell lock"));
    for _ in 0..128 {
        let (step, receipt) = turn(caller, decoder, POLICY.maintenance, |cx| {
            cx.with_original_retirement(|cx| {
                let mut step = None;
                let mut contended = false;
                match RuntimeLiveCleanupJob::advance_source(&cell, id, &mut step, &mut contended, cx) {
                    CleanupTurn::OriginalFailure(failure) => return Err(failure),
                    CleanupTurn::Fault(message) => panic!("fresh mounted maintenance faulted: {message:?}"),
                    _ => {}
                }
                assert!(!contended, "the captured fixture instance has no concurrent crossing");
                assert!(step.is_some(), "mounted maintenance must run after capture");
                assert!(matches!(runtime_live_cleanup_nonterminal_status(contended, step, &cell.maintenance_stalled_steps), RuntimeMaintenanceStatus::Ready), "initial cleanup exhausted its zero-progress credit");
                Ok(step.unwrap())
            })
        });
        aggregate = aggregate.checked_add(receipt).unwrap();
        assert!(!matches!(step, PluginLifecycleStep::AwaitingInput { .. }), "fresh capture has no outstanding host receipt");
        if matches!(step, PluginLifecycleStep::Complete(_)) {
            return (cell.instance.into_inner().ok().expect("uncontended fixture app").app, aggregate);
        }
    }
    panic!("initial maintenance did not finish before first dispatch")
}

fn native_window_definition() -> RegisteredAppDefinition {
 let mut definition=semio_framework_async::poll::resolve_ready(semio_framework_manifest_app_surface::testkit::app_with(Vec::new(),Vec::new()));
 definition.id=App::APP_ID.into();
 definition.controller_id=definition.id.clone();
 definition.dialect=App::DIALECT.into();
 definition.io.artifact_schema=App::DOCUMENT_SCHEMA.into();
 RegisteredAppDefinition::declared(definition,true)
}

fn close_native_window_declarations(
 mut definition:RegisteredAppDefinition,
 mut registration:Box<dyn semio_plugin_reactor_app::OriginalPluginAppRegistration>,
 mut caller:OriginalJobFixtureCaller,
 mut decoder:OriginalJobDecodeOwner,
)->(usize,usize){
 let(mut born,mut released)=(0,0);
 for index in 0..1048576{if registration.terminal_is_empty(){break}let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|registration.close_step(cx).map(|_|())));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(index<1048575,"actual native tool producer retained an instance alias");}
 let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|{let d=registration.boxed_release_demands();close_original_retained_step(cx,d,|_|{drop(registration);semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(())}));born+=p.retained_capacity_bytes;released+=p.released_bytes;
 for index in 0..1048576{if definition.terminal_is_empty(){break}let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|{let d=definition.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_effect_step(cx,d,|effect|definition.close_step(effect))?;Ok(())}));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(index<1048575,"actual native definition retained an instance alias");}
 while !decoder.terminal_is_empty(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(POLICY.close,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes),"native decoder close physical receipt {p:?}");result.unwrap();born+=p.retained_capacity_bytes;released+=p.released_bytes;}
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.retire(POLICY.close));
 born+=heap.requested_bytes;
 released+=heap.released_bytes;
 (born,released)
}

fn exercise_original_native_window(cancel_at:Option<&str>) -> (Option<(String,String)>,bool) {
 let factory=original_instance_factory::<App,NoMembers,Recipient,Registration>();
 let declare=||{
  let mut definition=native_window_definition();
  let mut registration=(factory.registration)(definition.original_source().unwrap()).unwrap();
  definition.commit_producer();
  registration.commit_producer();
  let caller=OriginalJobFixtureCaller::admit(OperationId(44711),Generation(78),RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},default_now_us,u64::MAX,1<<30);
  (definition,registration,caller)
 };
 static REGISTER_ACTION_BUS:std::sync::Once=std::sync::Once::new();
 REGISTER_ACTION_BUS.call_once(||{
  let(definition,registration,caller)=declare();
  close_native_window_declarations(definition,registration,caller,OriginalJobDecodeOwner::empty());
 });
 let((definition,registration,mut caller),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(declare);
 let initial=heap.requested_bytes-heap.released_bytes;
 let mut decoder=OriginalJobDecodeOwner::empty();let mut factory_birth=OriginalStoreBirth::default();let mut construction:Option<Box<dyn semio_plugin_reactor_app::OriginalPluginAppConstruction<Recipient>>>=None;let mut app:Option<Recipient>=None;let(mut born,mut released)=(0,0);let mut last_progress=(0,RetainedCloneProgress::default());
 for index in 0..1048576{let grant=if cancel_at==Some("during-large-absent-field")&&construction.is_some(){POLICY.preparation}else{RetainedCloneGrant{maximum_copy_bytes:1<<20,..POLICY.preparation}};let(_,p)=turn(&mut caller,&mut decoder,grant,|cx|{if let Some(construction)=construction.as_mut(){let source=definition.original_source().unwrap();construction.advance(OriginalPluginAppSource{definition:&source,definition_source:definition.original_source().unwrap(),registration:registration.as_ref(),actor:"native-window-actor",instance_id:77,policy:POLICY},cx,&mut app)?;}else{(factory.begin)(cx,&mut factory_birth,&mut construction)?;}Ok(())});born+=p.retained_capacity_bytes;released+=p.released_bytes;if p.copied_items>0||p.retained_capacity_bytes>0||p.released_bytes>0{last_progress=(index,p);}if app.is_some()||cancellation_boundary_reached(cancel_at,p){break}assert!(index<1048575,"actual declared native App opening stalled: construction={} born={} released={} last_progress={:?}",construction.is_some(),born,released,last_progress);}
 if cancel_at==Some("during-large-absent-field"){let cursor=semio_plugin_instance_construction::OriginalArtifactInstanceAdmission::<App,NoMembers>::INITIALIZATION_CURSOR_BYTES;for bytes in [0,cursor-1,cursor]{let grant=RetainedCloneGrant{maximum_copy_bytes:bytes,..POLICY.preparation};let(_,progress)=turn(&mut caller,&mut decoder,grant,|cx|{let source=definition.original_source().unwrap();construction.as_mut().unwrap().advance(OriginalPluginAppSource{definition:&source,definition_source:definition.original_source().unwrap(),registration:registration.as_ref(),actor:"native-window-actor",instance_id:77,policy:POLICY},cx,&mut app)?;Ok(())});assert_eq!(progress,RetainedCloneProgress::default(),"zero/subexact/cursor-only copy grants cannot advance an unfinished absent field");assert!(app.is_none());}}
 assert_eq!(app.is_none(),cancel_at.is_some());
 let identity=app.as_ref().map(|app|{
  let delegate=semio_plugin_reactor_app::PluginAppDelegate::delegate_ref(app);
  assert_eq!(delegate.snapshot_ref().zoom,1.25);
  assert_eq!(delegate.time_travel_ref().time_travel_retirement_demands(POLICY.maintenance.maximum_copy_bytes).expect("fresh TimeTravel maintenance quote"),Default::default(),"opening must leave TimeTravel live maintenance idle");
  (app.document_identity().expect("native document identity"),semio_framework_async::poll::resolve_ready(app.app_id()).to_owned())
 });
 for index in 0..1048576{if construction.as_ref().unwrap().terminal_is_empty(){break}let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|construction.as_mut().unwrap().close_step(cx).map(|_|())));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(index<1048575,"actual native opening retained its original sources after donation");}
 let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|{let d=construction.as_ref().unwrap().boxed_release_demands()?;close_original_retained_step(cx,d,|_|{drop(construction.take());semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(())}));born+=p.retained_capacity_bytes;released+=p.released_bytes;
 for index in 0..1048576{if factory_birth.terminal_is_empty(){break}let(_,p)=turn(&mut caller,&mut decoder,POLICY.close,|cx|cx.with_original_retirement(|cx|factory_birth.close_step(cx).map(|_|())));born+=p.retained_capacity_bytes;released+=p.released_bytes;assert!(index<1048575,"actual native factory retained its admission");}
 if let Some(native)=app.take(){let(native,maintenance)=observe_post_capture_maintenance(native,&mut caller,&mut decoder);born+=maintenance.retained_capacity_bytes;released+=maintenance.released_bytes;app=Some(native);}
 if app.is_some(){let mut idle=None;for index in 0..1048576{if app.as_ref().unwrap().close_terminal_is_empty(){break}let label=format!("App close turn {index}, {}",app.as_ref().unwrap().close_diagnostic());let((step,grant,fuel),p)=recorded_turn(&mut caller,&mut decoder,POLICY.close,&label,|cx|cx.with_original_retirement(|cx|{let grant=cx.retained_grant();let fuel=cx.fuel_remaining();app.as_mut().unwrap().close_step(cx,grant).map(|step|(step,grant,fuel)).map_err(OriginalTurnFailure::from)}));born+=p.retained_capacity_bytes;released+=p.released_bytes;if step.progress().is_none_or(|progress|progress==RetainedCloneProgress::default()){let start=idle.get_or_insert((index,grant,fuel));assert!(index-start.0<128,"actual native App close stalled: {step:?}, {}, next demand {:?}, first idle {start:?}, last grant {grant:?}, fuel {fuel}",app.as_ref().unwrap().close_diagnostic(),app.as_ref().unwrap().close_retirement_demands(grant.maximum_copy_bytes));}else{idle=None;}assert!(index<1048575,"actual native App close exceeded the turn bound");}assert!(app.take().unwrap().close_terminal_is_empty());}
 let(declaration_born,declaration_released)=close_native_window_declarations(definition,registration,caller,decoder);
 born+=declaration_born;
 released+=declaration_released;
 assert_eq!(initial+born,released,"actual declaration, native instance and caller graphs must close physically");
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
