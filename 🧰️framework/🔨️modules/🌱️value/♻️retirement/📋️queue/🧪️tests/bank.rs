//! 🏧️ One finite effect bank lends every queue test its Source-bound original effect and exact settlement.
use semio_framework_value_control_contract::*;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
/// 🏦️ Opens the finite wallet every queue law spends from.
pub fn bank()->EffectBank{let law:serde_json::Value=serde_json::from_str(include_str!("bank.json")).unwrap();EffectBank::new(Wallet::from_profile(&law["stock"]))}
/// 🧵️ Runs one synchronous original action and returns its settled turn with the physical marker.
pub fn poll<T,E>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->(RetainedWorkOutcome<T,E>,Option<usize>){let(poll,actual)=bank.run(grant,maximum,operation);let turn=match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous queue action stayed pending")};assert_ne!(bank.take_admitted(),Some(false));(turn,actual)}
/// ⚙️ Runs one synchronous original action that already settles its own outcome.
pub fn sync<T,E>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<T,E>)->(RetainedWorkOutcome<T,E>,Option<usize>){poll(bank,grant,maximum,|effect|RetainedWorkPoll::Complete(operation(effect)))}
/// 🎟️ Grants exactly one item and every currency a quoted demand names.
pub fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🔎️ Names the primary refusal of an original retirement failure.
pub fn kind(failure:&OriginalRetirementFailure)->ValueRefusalKind{failure.primary().expect("original failure keeps a primary refusal").kind}
/// 🎯️ Grants the exact quote of the selected action under the same copy body the step quotes with.
pub fn quoted(body:usize,quote:impl Fn(usize)->RetirementDemand)->RetainedCloneGrant{let mut copy=body;for _ in 0..8{let d=quote(copy);let want=body.max(d.copy_bytes);if want==copy{return RetainedCloneGrant{maximum_copy_bytes:copy,..grant(d)}}copy=want}panic!("queue quote did not settle on one copy body")}
