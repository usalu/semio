//! 🧾️ Private effect assembly retains each simultaneous refusal.
#[test]
fn snapshot_effect_keeps_each_simultaneous_refusal_and_performed_work(){
 use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
 let error=|message|ValueError::literal(ValueRefusalKind::Canceled,message);
 let turn=super::effect_outcome(Err(error("operation")),Err(error("work")),Err(error("local")),Err(error("physical")),RetainedCloneProgress{copied_items:1,..Default::default()});
 assert_eq!(turn.work,1);let failure=turn.result.unwrap_err();
 assert_eq!(failure.message(),"operation");assert_eq!(failure.work_refusal().unwrap().message.as_ref(),"work");assert_eq!(failure.settlement_refusal().unwrap().message.as_ref(),"local");assert_eq!(failure.receipt_refusal().unwrap().message.as_ref(),"physical");assert_eq!(failure.retained_progress().copied_items,1);
}
