//! 🧪️ The real Raster lane defaults, pointer handoff and cancelled prefixes use one original receiving continuation.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,StepContext,OriginalTurnFailure,OperationId,Generation};
use semio_framework_value::{RetainedCloneStep,RetainedWorkOutcome,ToValue};
type Source=original_ephemeral::OriginalEphemeralSource<semio_framework_plugin::EditorApp<RasterPlayApp>,RasterEphemeralDeclaration>;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:4096}}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{caller.turn_with_decoder(grant(),decoder,run).unwrap()}
#[test]
fn real_raster_ephemeral_lanes_preserve_authored_values_and_original_roots(){
 let fixture:semio_framework_value::serde_json::Value=semio_framework_value::serde_json::from_str(include_str!("../fixture.json")).unwrap();
 for cutoff in [0usize,1,3,5,10,16,usize::MAX]{
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(576),Generation(26),stock(),||Some(1),u64::MAX,1<<24);let mut decoder=OriginalJobDecodeOwner::empty();let mut source=None;
  assert!(turn(&mut caller,&mut decoder,|cx|Source::admit(cx,&mut source)));let mut ready=false;
  for _ in 0..cutoff.min(65536){ready=turn(&mut caller,&mut decoder,|cx|source.as_mut().unwrap().advance(cx));if ready{break}}
  let mut stores=None;
  if cutoff==usize::MAX{assert!(ready);let root=std::sync::Arc::as_ptr(source.as_ref().unwrap().stores_ref().unwrap().presence().original_local_root().unwrap());assert!(turn(&mut caller,&mut decoder,|cx|source.as_mut().unwrap().take(cx,&mut stores)));let actual=stores.as_ref().unwrap().presence();assert_eq!(std::sync::Arc::as_ptr(actual.original_local_root().unwrap()),root);assert_eq!(semio_framework_value::serde_json::to_value(actual.local().to_value()).unwrap(),fixture["presence"]);assert!(source.as_ref().unwrap().terminal_is_empty());}
  caller.cancel().cancel_now();for _ in 0..65536{if source.as_ref().unwrap().terminal_is_empty(){break}turn(&mut caller,&mut decoder,|cx|cx.with_original_retirement(|cx|source.as_mut().unwrap().close_step(cx)));}assert!(source.as_ref().unwrap().terminal_is_empty());
  let d=Source::boxed_release_demands();turn(&mut caller,&mut decoder,|cx|cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|_|{source=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(())}));
  if let Some(stores)=stores.as_mut(){for _ in 0..65536{if stores.terminal_is_empty(){break}turn(&mut caller,&mut decoder,|cx|cx.with_original_retirement(|cx|stores.close_step(cx)));}assert!(stores.terminal_is_empty());}
  while !decoder.terminal_is_empty(){caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;semio_framework_job::close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})).unwrap();}caller.retire(stock());
 }
}
