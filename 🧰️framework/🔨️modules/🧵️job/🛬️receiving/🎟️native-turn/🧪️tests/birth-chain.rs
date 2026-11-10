//! 🌱️ The inline birth chain births origin, table and publisher atomically per Source turn; each stage quote equals its actual Work and a budget one below the quote spends nothing.
use semio_framework_job_context::{Generation,OperationId,OriginalTurnFailure,RetainedCloneGrant,testkit};
use semio_framework_job_native_turn::{OriginalJobReturnBirth,OriginalJobReturnBirthTurn,original_return_birth_quote};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<12,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:64}}
struct Turn{quote:[u64;7],step:OriginalJobReturnBirthTurn,spent:u64,present:bool}
fn cut(fuel:u64,serial:u64)->(Turn,Option<Turn>){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(9200+serial),Generation(1),grant(),||Some(1),u64::MAX,1<<20);let mut birth=OriginalJobReturnBirth::new();
 for _ in 0..64{if birth.authorities_ready(){break}caller.turn(grant(),|cx|{testkit::renew_turn(cx,64,grant())?;Ok(birth.advance(cx)?)}).unwrap();}
 assert!(birth.authorities_ready(),"original birth authorities never admitted");
 let mut turn=|fuel:u64|caller.turn(grant(),|cx|{testkit::renew_turn(cx,fuel,grant())?;testkit::request_return_publisher(cx);let quote=original_return_birth_quote(cx).map_err(OriginalTurnFailure::from)?;let before=cx.fuel_remaining();let step=birth.advance(cx)?;Ok(Turn{quote,step,spent:before-cx.fuel_remaining(),present:testkit::return_table_present(cx)})}).unwrap();
 let starved=turn(0);assert_eq!(starved.step,OriginalJobReturnBirthTurn::Unaffordable{needed:starved.quote.iter().sum()},"fuel 0: the starved turn is exactly unaffordable and leaves no stale clock state");assert_eq!(starved.spent,0);let first=turn(fuel);let second=if first.present{Some(turn(1<<16))}else{None};
 std::mem::forget(birth);std::mem::forget(caller);(first,second)
}
#[test]
fn birth_chain_quotes_equal_spend_and_a_grant_one_below_is_refused(){
 let law:serde_json::Value=serde_json::from_str(include_str!("birth-chain.json")).unwrap();let stages=|key:&str|law[key].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()).collect::<Vec<_>>();
 let fresh=stages("fresh");let refresh=stages("refresh");let needed:u64=fresh.iter().sum();
 for(serial,row)in law["cuts"].as_array().unwrap().iter().enumerate(){
  let fuel=row["fuel"].as_u64().unwrap();let(first,second)=cut(fuel,serial as u64);assert_eq!(first.quote.to_vec(),fresh,"fuel {fuel}: fresh quote");
  if fuel>=needed{
   let OriginalJobReturnBirthTurn::Born{stages:spent,..}=first.step else{panic!("fuel {fuel}: {:?}",first.step)};assert!(first.present,"fuel {fuel}: table born");
   assert_eq!(spent.to_vec(),fresh,"fuel {fuel}: quote equals spend per stage");assert_eq!(first.spent,needed,"fuel {fuel}: exact Work");
   let second=second.expect("table present");let OriginalJobReturnBirthTurn::Born{stages:again,..}=second.step else{panic!("fuel {fuel}: refresh {:?}",second.step)};
   assert_eq!(second.quote.to_vec(),refresh,"fuel {fuel}: refresh quote");assert_eq!(again.to_vec(),refresh,"fuel {fuel}: refresh spend");assert_eq!(second.spent,refresh.iter().sum::<u64>());
  }else{
   assert_eq!(first.step,OriginalJobReturnBirthTurn::Unaffordable{needed},"fuel {fuel}");assert_eq!(first.spent,0,"fuel {fuel}: refused without side effects");assert!(!first.present);
  }
 }
}
#[test]
fn a_source_with_a_born_table_and_refreshed_origin_closes_terminal_empty(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(9900),Generation(1),grant(),||Some(1),u64::MAX,1<<20);caller.audit_heap();
 let mut turns=0;loop{turns+=1;assert!(turns<=64,"table never born");let present=caller.turn(grant(),|cx|{cx.admit_original_return_table()?;Ok(testkit::return_table_present(cx))}).unwrap();if present{break}}
 for _ in 0..2{caller.turn(grant(),|cx|{cx.admit_original_return_table()?;Ok(())}).unwrap();}
 caller.retire(grant());
}
