//! 🧪️ One original caller owns transient registration, publication and cancellation against the exact registry.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::RetainedCloneStep;
fn retired(cx:&mut semio_framework_job::StepContext<'_>,d:RetirementDemand,close:impl FnOnce(RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{semio_framework_job::close_original_retained_step(cx,d,|g|semio_framework_value::RetainedWorkOutcome{result:close(g),work:1})}
struct FixtureOwner;
impl WindowTransientOwner for FixtureOwner{
 const WINDOW_KIND_ID:&'static str="original-transient-fixture";
 type State=semio_plugin_testing_fixtures_publication::publication_fixture::transient::PublicationTransient;
 type Mutation=semio_plugin_testing_fixtures_publication::publication_fixture::transient::mutations::PublicationTransientMutation;
 fn owners_birth_demands()->RetirementDemand{transfer_owners_birth_demands::<Self::State,Self::Mutation>()}
 fn build_owners()->WindowTransientOwnerBundle<Self::State,Self::Mutation>{semio_plugin_testing_fixtures_publication::window_transient_owners::owners()}
}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128}}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:4096}}
fn close(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,registry:&mut WindowTransientOwnerRegistry,source:&mut Option<Box<OriginalWindowTransientOwnerRegistration<FixtureOwner>>>){
 for _ in 0..65536{if source.as_ref().unwrap().terminal_is_empty(){break}caller.turn_with_decoder(grant(),decoder,|cx|{source.as_mut().unwrap().close_step(registry,cx)?;Ok(())}).unwrap();}
 assert!(source.as_ref().unwrap().terminal_is_empty());
 caller.turn_with_decoder(grant(),decoder,|cx|{cx.with_original_retirement(|cx|{let d=OriginalWindowTransientOwnerRegistration::<FixtureOwner>::boxed_release_demands();retired(cx,d,|_|{drop(source.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;Ok(())})}).unwrap();
}
#[test]
fn original_transient_registration_publishes_only_its_actual_owner(){
 let fixture:semio_framework_value::serde_json::Value=semio_framework_value::serde_json::from_str(include_str!("../fixture.json")).unwrap();
 assert_eq!(grant().maximum_copy_bytes,fixture["maximumCopyBytes"].as_u64().unwrap()as usize);
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(570),Generation(24),stock(),||Some(1),u64::MAX,1<<24);
 let mut decoder=OriginalJobDecodeOwner::empty();let mut registry=WindowTransientOwnerRegistry::for_document_generation(9);let mut source=None;
 caller.turn_with_decoder(grant(),&mut decoder,|cx|{registry.admit_original_owner::<FixtureOwner>(cx,&mut source)?;Ok(())}).unwrap();assert!(registry.is_empty());
 let mut foreign=WindowTransientOwnerRegistry::for_document_generation(9);
 caller.turn_with_decoder(grant(),&mut decoder,|cx|{assert!(source.as_mut().unwrap().step(&mut foreign,cx).is_err());Ok(())}).unwrap();assert!(foreign.is_empty());
 for _ in 0..65536{let mut done=false;caller.turn_with_decoder(grant(),&mut decoder,|cx|{done=source.as_mut().unwrap().step(&mut registry,cx)?;Ok(())}).unwrap();if done{break}}
 assert!(registry.owners.contains_key(FixtureOwner::WINDOW_KIND_ID));assert_eq!(registry.owner_open,1);close(&mut caller,&mut decoder,&mut registry,&mut source);
 for _ in 0..65536{if registry.terminal_is_empty(){break}caller.turn_with_decoder(grant(),&mut decoder,|cx|cx.with_original_retirement(|cx|{let d=registry.retirement_demands(32768)?;retired(cx,d,|g|Ok(match registry.close_step(g).unwrap(){PluginLifecycleStep::Complete(p)=>RetainedCloneStep::Complete(p),step=>RetainedCloneStep::Progress(step.progress().unwrap_or_default())}))?;Ok(())})).unwrap();}
 assert!(registry.terminal_is_empty());while !decoder.terminal_is_empty(){caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;semio_framework_job::close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})).unwrap();}caller.retire(stock());
}
#[test]
fn cancelled_transient_registration_keeps_the_registry_empty(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(571),Generation(25),stock(),||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();let mut registry=WindowTransientOwnerRegistry::default();let mut source=None;
 caller.turn_with_decoder(grant(),&mut decoder,|cx|{registry.admit_original_owner::<FixtureOwner>(cx,&mut source)?;Ok(())}).unwrap();caller.cancel().cancel_now();close(&mut caller,&mut decoder,&mut registry,&mut source);assert!(registry.terminal_is_empty());while !decoder.terminal_is_empty(){caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;semio_framework_job::close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})).unwrap();}caller.retire(stock());
}
