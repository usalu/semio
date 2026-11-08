//! 🧪️ Neutral cursor vectors deny original work before effects and preserve third-party JSON values.
use semio_framework_value_clone_source::RetainedCloneSource;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use semio_framework_value_retained_clone::*;
use semio_framework_value_paged_list::PagedList;
use serde_json::{Value,json};
use std::collections::BTreeMap;

fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn work_oracle(local:u64,child:u64,available:u64)->(u64,u64){let law:Value=serde_json::from_str(include_str!("fixture.json")).unwrap();let row=law["cases"].as_array().unwrap().iter().find(|row|row["phase"]=="advance"&&row["local"]==local&&row["child"]==child&&row["available"]==available).expect("authored selected work vector");(row["quoted"].as_u64().unwrap(),row["performed"].as_u64().unwrap())}
fn exercise<T:RetainedClone>(owner:T,expected:&Value,inspect:impl Fn(&T)->Value,law:&Value)->(u64,u64){
 let source=RetainedCloneSource::<T>::from_owner(owner);let mut cursor=T::retained_clone_cursor();let body=law["bodyBytes"].as_u64().unwrap()as usize;let limit=law["maximumWork"].as_u64().unwrap();let denied=law["deniedWork"].as_u64().unwrap();let turns=law["maximumTurns"].as_u64().unwrap();let mut composite_denials=0;let mut delegated_denials=0;let mut binding=true;
 for _ in 0..turns{if cursor.output_ready(){break}let quote=cursor.advance_work_demand(source.borrow(),body,limit).unwrap();let mut authored=if binding{Some(work_oracle(1,0,limit))}else{None};assert!(quote>0&&quote<=limit);let demand=cursor.advance_demands(source.borrow(),body,limit).unwrap();assert_eq!(cursor.advance_work_demand(source.borrow(),body,denied).unwrap(),0);let no=cursor.advance(source.borrow(),body,denied,grant(demand));assert_eq!(no.work,0);assert_eq!(no.result.unwrap().progress(),Default::default());assert!(!cursor.output_ready());assert_eq!(cursor.advance_work_demand(source.borrow(),body,limit).unwrap(),quote);let insufficient=quote-1;if insufficient>0&&cursor.advance_work_demand(source.borrow(),body,insufficient).unwrap()==0{let no=cursor.advance(source.borrow(),body,insufficient,grant(demand));assert_eq!(no.work,0);assert_eq!(no.result.unwrap().progress(),Default::default());assert_eq!(cursor.advance_work_demand(source.borrow(),body,limit).unwrap(),quote);composite_denials+=1;if quote==2||quote==law["childWork"].as_u64().unwrap(){let denied_oracle=work_oracle(0,quote,insufficient);assert_eq!(quote,denied_oracle.0);assert_eq!(no.work,denied_oracle.1);authored=Some(work_oracle(0,quote,limit));}if quote==law["childWork"].as_u64().unwrap(){delegated_denials+=1;}}let turn=cursor.advance(source.borrow(),body,limit,grant(demand));assert_eq!(turn.work,quote);if let Some((expected_quote,expected_work))=authored{assert_eq!(quote,expected_quote);assert_eq!(turn.work,expected_work);}binding=false;assert!(turn.result.unwrap().progress().fits(grant(demand)));}
 assert!(cursor.output_ready());let take_quote=cursor.take_work_demand().unwrap();let turn=cursor.take(grant(cursor.take_demand().unwrap()));assert_eq!(turn.work,take_quote);let(output,_)=turn.result.unwrap().unwrap();assert_eq!(inspect(&output),*expected);assert!(cursor.advance_work_demand(source.borrow(),body,limit).is_err());assert!(cursor.advance(source.borrow(),body,limit,grant(Default::default())).result.is_err());let output_source=RetainedCloneSource::<T>::from_owner(output);
 let close_quote=cursor.begin_close_work_demand().unwrap();let turn=cursor.begin_close_paid();assert_eq!(turn.work,close_quote);turn.result.unwrap();assert!(cursor.close_started());assert_eq!(cursor.begin_close_work_demand().unwrap(),0);assert_eq!(cursor.begin_close_paid().work,0);
 for _ in 0..turns{if cursor.terminal_is_empty(){break}let quote=cursor.close_work_demand(body).unwrap();let demand=cursor.close_demands(body).unwrap();let turn=cursor.close_step(grant(demand));assert_eq!(turn.work,quote);assert!(turn.result.unwrap().progress().fits(grant(demand)));}assert!(cursor.terminal_is_empty());drop(cursor);drop(output_source);drop(source);(composite_denials,delegated_denials)
}
#[test]
fn selected_cursor_work_preserves_neutral_values(){
 let law:Value=serde_json::from_str(include_str!("cursors.json")).unwrap();for row in law["cases"].as_array().unwrap(){let authored:Value=serde_json::from_str(row["source"].as_str().unwrap()).unwrap();let expected=authored.clone();let composite_denials=match row["cursor"].as_str().unwrap(){
 "scalar"=>exercise(expected.as_u64().unwrap(),&expected,|v|json!(v),&law),
 "array"=>exercise(serde_json::from_value::<[u64;2]>(authored.clone()).unwrap(),&expected,|v|json!(v),&law),
 "tuple"=>exercise(serde_json::from_value::<(String,u64)>(authored.clone()).unwrap(),&expected,|v|json!(v),&law),
 "dynamic"=>exercise(DslValue::Array(vec![DslValue::String(authored[0].as_str().unwrap().to_owned())]),&expected,|v|match v{DslValue::Array(a)=>match&a[0]{DslValue::String(s)=>json!([s]),_=>panic!("dynamic string absent")},_=>panic!("dynamic array absent")},&law),
 "paged-list"=>{let mut values=PagedList::<String,8>::default();while !values.has_reserved_slot(){let d=values.next_allocation_bytes().unwrap();values.reserve_one(d).unwrap();}values.push_reserved(authored[0].as_str().unwrap().to_owned()).unwrap();exercise(values,&expected,|v|json!([v.get(0).unwrap()]),&law)},
 "std-collections"=>exercise(serde_json::from_value::<BTreeMap<String,u64>>(authored.clone()).unwrap(),&expected,|v|json!(v),&law),
 "issuance-clone"=>{use shared_issuance::{SealedShared,SharedIssuer};let mut actual=None;let turn=SealedShared::admit(authored.as_u64().unwrap(),SharedIssuer::owned(),grant(RetirementDemand{capacity_bytes:SealedShared::<u64>::birth_bytes(),depth:1,..Default::default()}),&mut actual);let(value,_)=turn.result.unwrap();exercise(value,&expected,|v|json!(v.get()),&law)},
 "tuple-costed"=>exercise((CostedAlias{value:authored[0].as_u64().unwrap(),cost:law["childWork"].as_u64().unwrap(),fail:false},authored[1].as_u64().unwrap()),&expected,|v|json!([v.0.value,v.1]),&law),
 "paged-list-costed"=>{let mut values=PagedList::<CostedAlias,8>::default();while !values.has_reserved_slot(){let d=values.next_allocation_bytes().unwrap();values.reserve_one(d).unwrap();}values.push_reserved(CostedAlias{value:authored[0].as_u64().unwrap(),cost:law["childWork"].as_u64().unwrap(),fail:false}).unwrap();exercise(values,&expected,|v|json!([v.get(0).unwrap().value]),&law)},
 "std-collections-costed"=>exercise(BTreeMap::from([(7u64,CostedAlias{value:authored["7"].as_u64().unwrap(),cost:law["childWork"].as_u64().unwrap(),fail:false})]),&expected,|v|json!({"7":v.get(&7).unwrap().value}),&law),
 _=>panic!("unrecognized neutral cursor family")};assert!(composite_denials.0>=row["minimumCompositeDenials"].as_u64().unwrap());assert!(composite_denials.1>=row["minimumDelegatedDenials"].as_u64().unwrap());}
}

#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="semio_framework_value_retained_clone")]
struct CostedAlias{value:u64,cost:u64,fail:bool}
impl RetainedClone for CostedAlias{type Cursor=RetainedAliasCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
impl RetainedAlias for CostedAlias{
 fn alias_demand(&self)->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 fn alias_work_demand(&self)->u64{self.cost}
 fn admit_alias_into<'reply>(&self,output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{let result=if output.is_some(){Err(ValueError::invariant("alias output occupied").into())}else if !effect.performed(self.cost)?{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"alias work refused").into())}else if self.fail{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"authored child failure after paid work").into())}else{*output=Some(Self{value:self.value,cost:self.cost,fail:false});Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))};Ok(reply.complete(result))} fn recover_alias_into<'reply>(&self,_output:&mut Option<Self>,_effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
}
fn failing<T:RetainedClone>(owner:T,law:&Value){
 let source=RetainedCloneSource::<T>::from_owner(owner);let mut cursor=T::retained_clone_cursor();let body=law["bodyBytes"].as_u64().unwrap()as usize;let limit=law["maximumWork"].as_u64().unwrap();let cost=law["childWork"].as_u64().unwrap();let delegated=work_oracle(0,cost,limit);let denied=work_oracle(0,cost,cost-1);let turns=law["maximumTurns"].as_u64().unwrap();let mut failed=false;
 for _ in 0..turns{let quote=cursor.advance_work_demand(source.borrow(),body,limit).unwrap();let demand=cursor.advance_demands(source.borrow(),body,limit).unwrap();if quote==cost{assert_eq!(cursor.advance_work_demand(source.borrow(),body,cost-1).unwrap(),0);let turn=cursor.advance(source.borrow(),body,cost-1,grant(demand));assert_eq!(turn.work,denied.1);assert_eq!(quote,denied.0);assert_eq!(turn.result.unwrap().progress(),Default::default());}let turn=cursor.advance(source.borrow(),body,limit,grant(demand));assert_eq!(turn.work,quote);if turn.result.is_err(){assert_eq!(quote,delegated.0);assert_eq!(turn.work,delegated.1);assert_eq!(turn.work,law["failureWork"].as_u64().unwrap());failed=true;break;}}
 assert!(failed);assert!(!cursor.output_ready());let quote=cursor.begin_close_work_demand().unwrap();let turn=cursor.begin_close_paid();assert_eq!(turn.work,quote);turn.result.unwrap();for _ in 0..turns{if cursor.terminal_is_empty(){break}let quote=cursor.close_work_demand(body).unwrap();let demand=cursor.close_demands(body).unwrap();let turn=cursor.close_step(grant(demand));assert_eq!(turn.work,quote);turn.result.unwrap();}assert!(cursor.terminal_is_empty());drop(cursor);drop(source);
}
#[test]
fn selected_child_failure_retains_paid_work_and_closes_partial_owners(){
 let law:Value=serde_json::from_str(include_str!("cursors.json")).unwrap();for family in law["failures"].as_array().unwrap(){let child=CostedAlias{value:7,cost:law["childWork"].as_u64().unwrap(),fail:true};match family.as_str().unwrap(){
 "tuple-costed"=>failing((child,11u64),&law),
 "paged-list-costed"=>{let mut values=PagedList::<CostedAlias,8>::default();while !values.has_reserved_slot(){let d=values.next_allocation_bytes().unwrap();values.reserve_one(d).unwrap();}values.push_reserved(child).unwrap();failing(values,&law)},
 "std-collections-costed"=>failing(BTreeMap::from([(7u64,child)]),&law),
 _=>panic!("unrecognized failing child family")}}
}
