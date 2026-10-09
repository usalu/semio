use super::*;
use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

fn drain<T:RetireOwned+Clone+PartialEq+std::fmt::Debug>(value:T,source_bytes:usize){
 let before=value.clone();let(mut owner,allocation)=observe(||ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("{error}")));assert_eq!((allocation.requested_bytes,allocation.released_bytes),(0,0));let mut born=0;let mut released=0;
 for _ in 0..100000{
  if owner.terminal_is_empty(){break;}
  let copy=owner.next_copy_byte_demand().unwrap();let funding=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
  let(step,allocation)=observe(||owner.step(RetainedCloneGrant{maximum_items:0,..funding}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((allocation.requested_bytes,allocation.released_bytes),(0,0));if let Some(original)=owner.original(){assert_eq!(original,&before);}
  for grant in (funding.maximum_capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:funding.maximum_capacity_bytes.saturating_sub(1),..funding}).into_iter().chain((funding.maximum_release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:funding.maximum_release_bytes.saturating_sub(1),..funding})){
   let(result,allocation)=observe(||owner.step(grant));assert_eq!((allocation.requested_bytes,allocation.released_bytes),(0,0));assert!(result.is_err()||result.unwrap().progress()==RetainedCloneProgress::default());
  }
  let(step,allocation)=observe(||owner.step(funding).unwrap());let receipt=step.progress();assert!(receipt.fits(funding));assert!(!allocation.overflowed);assert_eq!((allocation.requested_bytes,allocation.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=allocation.requested_bytes;released+=allocation.released_bytes;
 }
 assert!(owner.terminal_is_empty());assert_eq!(released,source_bytes+born);let(_,allocation)=observe(||drop(owner));assert_eq!((allocation.requested_bytes,allocation.released_bytes),(0,0));eprintln!("[DEBUG] Retained command metadata original={source_bytes} born={born} release={released} terminalDrop=0");
}
#[test]
fn retained_command_metadata_preserves_original_history_and_physical_authority(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let text=row["text"].as_str().unwrap();let rows=row["rows"].as_u64().unwrap()as usize;
  let(operation,allocation)=observe(||AppOperationContext{app_instance_id:1,parent_document_id:text.into(),operation_id:42,generation:7,canonical_base_revision:[3;32],authoring_seed:text.into()});drain(operation,allocation.requested_bytes-allocation.released_bytes);
  let(history,allocation)=observe(||{
   let mut history=HistoryView::empty();history.active_alternative_id=Some(text.into());history.current_checkpoint_id=Some(text.into());
   for index in 0..rows{
    history.alternatives.push(AlternativeView{id:text.into(),name:text.into(),current:index==0,author:Some(text.into()),branched_at:Some(text.into()),edited:true});
    history.columns.push(semio_framework_os_kernel::store::HistoryColumn{checkpoint_id:text.into(),timestamp:text.into(),labels:vec![text.into()],authors:vec![semio_framework_os_kernel::vcs::Author{id:text.into(),name:text.into(),avatar:Some(text.into())}],parent_checkpoint_id:Some(text.into()),description:Some(text.into()),lane:index,alternative_ids:vec![text.into()]});
    history.commands.push(CommandView{seq:index as u64,action_id:text.into(),label:semio_framework_ui_locale::LocalizedLabel::data(text),kind:semio_framework::ActionKind::Mutation,timestamp:text.into(),edit_id:Some(text.into()),child_edit_ids:vec![text.into()],transition_id:Some(text.into()),author:Some(text.into()),op_lines:vec![text.into()],op_count:1,applied:true,revertible:true,count:1,inverse:Some(InverseAction{action_id:text.into(),args:Some(semio_framework_value::DslValue::String(text.into()))}),transaction:Some(protocol::TransactionRef{id:text.into(),tool:text.into()}),mutations:vec![MutationView{mutation_id:text.into(),position:0,op_index:0,label:semio_framework_ui_locale::LocalizedLabel::data(text),worst:None,messages:Vec::new(),superseded:false,withdrawn:false,editable:true,withdrawable:true,store:Some(text.into())}]});
   }
   history
  });
  for command in &history.commands{let expected:semio_framework_ui_locale::LocalizedLabel=serde_json::from_value(serde_json::to_value(&command.label).unwrap()).unwrap();assert_eq!(command.label,expected);}
  drain(history,allocation.requested_bytes-allocation.released_bytes);
 }
}
