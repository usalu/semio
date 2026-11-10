//! ⏱️ Test-only adapters run one shared UTF8 action inside one actual original effect.
use semio_framework_value_shared_utf8::*;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};

pub fn bank()->EffectBank{EffectBank::new(Wallet::from_profile(&serde_json::json!({"items":1u64<<20,"copyBytes":1u64<<30,"capacityBytes":1u64<<30,"releaseBytes":1u64<<30,"depth":4096u64,"fuel":1u64<<40})))}
pub fn run<R>(bank:&mut EffectBank,grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 let mut out=None;
 let(poll,_)=bank.run::<(),()>(grant,1<<20,|effect|{out=Some(operation(effect));RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(()),work:effect.performed_work()})});
 assert!(matches!(poll,RetainedWorkPoll::Complete(_)));out.unwrap()
}
pub fn admit(original:String,grant:RetainedCloneGrant)->RetainedWorkOutcome<(SharedUtf8,RetainedCloneProgress),(ValueError,String)>{let mut actual=None;SharedUtf8::admit(original,grant,&mut actual)}
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}

