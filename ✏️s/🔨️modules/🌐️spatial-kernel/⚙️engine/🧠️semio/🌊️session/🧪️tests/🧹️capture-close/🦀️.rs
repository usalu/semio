//! 🧹️ Checks one-fuel original capture close and cancellation custody.

use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,close_original_retained_step};
#[test]
fn original_session_capture_close_keeps_actual_one_fuel_work_and_cancel_custody(){
 let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096};
 for cancel in [None,Some(0),Some(1),Some(3)]{
  let(mut source,(births,releases))=physical_session_custody::observe(||SessionCapture::owned(Session::new()));let initial=births-releases;let(mut born,mut free)=(0,0);let mut caller=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),stock,||Some(1000),1500,32768);let mut within=false;
  for call in 0..100000{if source.terminal_is_empty(){within|=call<=200;break}if cancel==Some(call){caller.cancel().cancel_now()}let d=semio_framework_value::RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(stock.maximum_copy_bytes).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()};
   let(_,heap)=physical_session_custody::observe(||caller.turn(RetainedCloneGrant{maximum_items:0,..stock},|cx|cx.with_original_retirement(|cx|cx.with_original_work_loan(1,1500,|cx|{let turn=source.close_original_step(cx.retained_grant());assert_eq!(turn.work,0);assert_eq!(turn.result?.progress(),Default::default());Ok(())}))).unwrap());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());
   let(_,heap)=physical_session_custody::observe(||caller.turn(RetainedCloneGrant{maximum_items:512,..stock},|cx|cx.with_original_retirement(|cx|cx.with_original_work_loan(1,1500,|cx|{close_original_retained_step(cx,d,|g|{let turn=source.close_original_step(g);assert!(turn.work<=1);turn})?;Ok(())}))).unwrap());let progress=caller.receipt();assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;free+=heap.1;
  }
  let closed=source.terminal_is_empty();drop(source);caller.retire(stock);assert!(closed&&within);assert_eq!(initial+born,free);
 }
}
