//! 🧪️ Immutable aliases preserve original ownership under a denied work limit.
use semio_framework_value_clone_source::RetainedCloneSource;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use semio_framework_value_derive::RetireOwned;
use semio_framework_value_retained_clone::*;
#[path="../../🧪️tests/⏱️effect/🦀️.rs"]
mod effect;
#[derive(Debug, PartialEq, RetireOwned)]
#[value(crate="semio_framework_value_retained_clone")]
struct Alias(u64);
impl RetainedClone for Alias{type Cursor=RetainedAliasCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl RetainedAlias for Alias{
 fn alias_demand(&self)->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 fn alias_work_demand(&self)->u64{1}
 fn admit_alias_into<'reply>(&self,output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{let result=if output.is_some(){Err(ValueError::invariant("alias output occupied").into())}else if !effect.performed(1)?{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"alias work refused").into())}else{*output=Some(Self(self.0));Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))};Ok(reply.complete(result))} fn recover_alias_into<'reply>(&self,_output:&mut Option<Self>,_effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[test]
fn selected_alias_work_is_quoted_before_each_effect(){
 let law:serde_json::Value=serde_json::from_str(include_str!("work.json")).unwrap();let source=RetainedCloneSource::<Alias>::from_owner(Alias(law["source"].as_u64().unwrap()));let mut cursor=Alias::retained_clone_cursor();let body=4096;let no=law["deniedLimit"].as_u64().unwrap();let yes=law["admittedLimit"].as_u64().unwrap();
 let mut stages=0;while !cursor.output_ready(){stages+=1;assert!(stages<=16);let quote=cursor.advance_work_demand(source.borrow(),body,yes).unwrap();assert!(quote>0);let demand=cursor.advance_demands(source.borrow(),body,yes).unwrap();let denied=effect::advance(&mut cursor,source.borrow(),body,no,grant(demand));assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap().progress(),Default::default());assert!(!cursor.output_ready());let turn=effect::advance(&mut cursor,source.borrow(),body,yes,grant(demand));assert_eq!(turn.work,u128::from(quote));turn.result.unwrap();}
 assert_eq!(stages,law["expectedStages"].as_u64().unwrap());assert!(cursor.output_ready());let start=cursor.begin_close_work_demand().unwrap();let turn={let demand=cursor.begin_close_demand().unwrap();effect::begin_close_paid(&mut cursor,grant(demand))};assert_eq!(turn.work,u128::from(start));turn.result.unwrap();for _ in 0..10000{if cursor.terminal_is_empty(){break}let demand=cursor.close_demands(body).unwrap();let quote=cursor.close_work_demand(body).unwrap();let turn=effect::close_step(&mut cursor,grant(demand));assert!(turn.work<=u128::from(quote));turn.result.unwrap();}assert!(cursor.terminal_is_empty());drop(cursor);drop(source);
}
