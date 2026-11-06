//! 🧪️ Immutable aliases preserve original ownership under a denied work limit.
use semio_framework_value_clone_source::RetainedCloneSource;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use semio_framework_value_derive::RetireOwned;
use semio_framework_value_retained_clone::*;
#[derive(Debug, PartialEq, RetireOwned)]
#[value(crate="semio_framework_value_retained_clone")]
struct Alias(u64);
impl RetainedClone for Alias{type Cursor=RetainedAliasCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl RetainedAlias for Alias{
 fn alias_demand(&self)->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 fn alias_work_demand(&self)->u64{1}
 fn admit_alias(&self,_:RetainedCloneGrant)->RetainedWorkOutcome<(Self,RetainedCloneProgress)>{RetainedWorkOutcome{result:Ok((Self(self.0),RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[test]
fn selected_alias_work_is_quoted_before_each_effect(){
 let law:serde_json::Value=serde_json::from_str(include_str!("work.json")).unwrap();let source=RetainedCloneSource::<Alias>::from_owner(Alias(law["source"].as_u64().unwrap()));let mut cursor=Alias::retained_clone_cursor();let body=4096;let no=law["deniedLimit"].as_u64().unwrap();let yes=law["admittedLimit"].as_u64().unwrap();
 for _ in 0..2{let quote=cursor.advance_work_demand(source.borrow(),body,yes).unwrap();assert_eq!(quote,law["expectedWork"].as_u64().unwrap());let demand=cursor.advance_demands(source.borrow(),body,yes).unwrap();let denied=cursor.advance(source.borrow(),body,no,grant(demand));assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap().progress(),Default::default());assert!(!cursor.output_ready());let turn=cursor.advance(source.borrow(),body,yes,grant(demand));assert_eq!(turn.work,quote);turn.result.unwrap();}
 assert!(cursor.output_ready());let start=cursor.begin_close_work_demand().unwrap();let turn=cursor.begin_close_paid(grant(cursor.begin_close_demand().unwrap()));assert_eq!(turn.work,start);turn.result.unwrap();for _ in 0..10000{if cursor.terminal_is_empty(){break}let demand=cursor.close_demands(body).unwrap();let quote=cursor.close_work_demand(body).unwrap();let turn=cursor.close_step(grant(demand));assert!(turn.work<=quote);turn.result.unwrap();}assert!(cursor.terminal_is_empty());drop(cursor);drop(source);
}
