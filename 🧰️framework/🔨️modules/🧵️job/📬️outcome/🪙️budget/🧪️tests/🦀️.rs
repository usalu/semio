use super::*;
#[derive(Clone,Copy)]
struct Origin{recipient:usize,grant:RetainedCloneGrant,fuel:u64,spent:P,work:u128}
impl OriginalReturnOrigin for Origin{
 fn same_origin(&self,other:&Self)->bool{self.recipient==other.recipient}
 fn recipient(&self)->usize{self.recipient}
 fn source_identity(&self)->usize{1}
 fn source_turn(&self)->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>{None}
 fn granted(&self)->RetainedCloneGrant{self.grant}
 fn fuel(&self)->u64{self.fuel}
 fn spent(&self)->P{self.spent}
 fn work(&self)->u128{self.work}
}
#[test]
fn typed_budget_binding_keeps_original_destination_and_conserved_stock(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let mut budget=OriginalReturnBudget::<Origin>::empty(1);
  let input=Origin{recipient:&budget as*const _ as usize,grant:RetainedCloneGrant{maximum_items:row["grant"].as_u64().unwrap()as usize,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1},fuel:row["fuel"].as_u64().unwrap(),spent:P{copied_items:row["spent"].as_u64().unwrap()as usize,..Default::default()},work:u128::from(row["work"].as_u64().unwrap())};
  let foreign=Origin{recipient:input.recipient+1,..input};
  assert!(budget.bind(foreign).is_err());assert!(budget.origin.is_none());
  assert_eq!(budget.bind(input).is_ok(),row["accepted"].as_bool().unwrap());
  if row["accepted"]==true{assert_eq!(budget.bind(input).unwrap(),false);let(g,fuel)=budget.remaining().unwrap();assert_eq!(g.maximum_items,row["remaining"].as_u64().unwrap()as usize);assert_eq!(fuel,row["remainingFuel"].as_u64().unwrap());}
 }
}

#[test]
fn copied_origin_data_cannot_mint_a_claim_at_another_actual_budget_address(){let mut original=OriginalReturnBudget::<Origin>::empty(1);let input=Origin{recipient:&original as*const _ as usize,grant:RetainedCloneGrant{maximum_items:100,maximum_copy_bytes:100000,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:8},fuel:100,spent:P::default(),work:0};original.bind(input).unwrap();let mut foreign=OriginalReturnBudget::<Origin>::empty(2);foreign.origin=original.origin;assert!(foreign.claim(1,9,1).is_err());assert_eq!((foreign.claims,foreign.held_items,foreign.held_copy_bytes,foreign.held_fuel),(0,0,0,0));assert!(foreign.table.is_none());}

use semio_framework_value_control_contract as control;

struct ObservationRecipient;
impl control::OriginalWorkRecipient for ObservationRecipient {
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<control::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'a,'turn,'source>(&mut self,_:&mut (),quote:control::OriginalWorkQuote<'a>,_:u64,_:u64)->control::OriginalWorkQuoteComplete<'a>where 'source:'turn{quote.complete(Ok(true))}
 fn settle_partial<'a,'turn,'source>(&mut self,_:&mut (),check:control::OriginalWorkPartialCheck<'a>)->control::OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'a,'turn,'source>(&mut self,_:&mut (),check:control::OriginalWorkPartialCheck<'a>)->control::OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}
}
impl control::OriginalConstructionRecipient for ObservationRecipient {
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<control::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:control::OriginalConstructionGrant<'a>)->control::OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(Default::default()))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:control::OriginalConstructionAccept<'a>)->control::OriginalConstructionAcceptComplete<'a>where 'source:'turn{receipt.complete(Ok(()))}
 fn submitted<'turn,'source>(&self,_:&())->P where 'source:'turn{P::default()}
}

fn observation_fence()->OriginalReturnBirthLock{
 let mut payer=control::OriginalWorkAuthority::new(ObservationRecipient);
 let mut construction=control::OriginalConstructionAuthority::new(ObservationRecipient);
 payer.with_loan(&mut (),|loan|{
  let mut loan=loan.reborrow();
  let mut work=control::OriginalWork::empty();
  work.recipient=Some(&mut loan);
  let mut action=None;
  assert_eq!(work.prepare_for_action(1,&mut action).unwrap(),Some(true));
  let fence=construction.with_loan(&mut (),|loan|{
   let mut slot=None;
   loan.begin(Default::default(),&mut slot,&control::OriginalWorkBorrow::new(&mut work)).unwrap();
   let original=slot.as_mut().unwrap();
   let fence=OriginalReturnBirthLock{action:original.identity(),work:original.work_binding(),work_action:action.unwrap()};
   loan.cancel(original,false,P::default(),None).unwrap();
   loan.finish(&mut slot).unwrap();
   fence
  }).unwrap();
  work.cancel_for_action(&mut action).unwrap();
  fence
 }).unwrap()
}

fn observation_stock(items:usize,depth:usize)->RetainedCloneGrant{
 RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:items*2,maximum_capacity_bytes:items*3,maximum_release_bytes:items*4,maximum_depth:depth}
}

fn observation_pair(value:&serde_json::Value)->(usize,u64){
 (value[0].as_u64().unwrap().try_into().unwrap(),value[1].as_u64().unwrap())
}

fn assert_observation_balance(actual:Result<(RetainedCloneGrant,u64),ValueError>,expected:&serde_json::Value,depth:usize){
 if let Some(expected)=expected.as_str(){
  let (kind,message)=match expected{
   "absent"=>(ValueRefusalKind::InvariantViolated,E::STOCK_ABSENT),
   "spent"=>(ValueRefusalKind::InvariantViolated,"original return source spent exceeds its original stock"),
   "held"=>(ValueRefusalKind::InvariantViolated,"original return holds exceed retained lifetime stock"),
   "heldFuel"=>(ValueRefusalKind::InvariantViolated,"original return holds exceed original CPU stock"),
   "refused"=>(ValueRefusalKind::Canceled,"original observation refused"),
   other=>panic!("unknown refusal {other}"),
  };
  let error=actual.unwrap_err();
  assert_eq!(error.kind,kind);
  assert_eq!(error.message,message);
 }else{
  let (items,fuel)=observation_pair(expected);
  assert_eq!(actual.unwrap(),(observation_stock(items,depth),fuel));
 }
}

#[test]
fn return_budget_observations_preserve_visibility_and_balances(){
 let cases:Vec<serde_json::Value>=serde_json::from_str(include_str!("observation.json")).unwrap();
 let fence=observation_fence();
 for row in cases{
  let mut budget=OriginalReturnBudget::<Origin>::empty(1);
  let recipient=&budget as *const _ as usize;
  let origin=|value:&serde_json::Value|(!value.is_null()).then(||{
   let (items,fuel)=observation_pair(value);
   Origin{recipient,grant:observation_stock(items,1),fuel,spent:P::default(),work:0}
  });
  budget.origin=origin(&row["current"]);
  budget.origin_prior=origin(&row["prior"]);
  if row["phase"]=="fenced"{budget.origin_commit=Some(fence);}
  if row["phase"]=="staged"{budget.origin_pending=Some(Origin{recipient,grant:observation_stock(80,1),fuel:100,spent:P::default(),work:0});}
  let (spent,work)=observation_pair(&row["spent"]);
  let (held,held_fuel)=observation_pair(&row["held"]);
  budget.spent=P{copied_items:spent,copied_bytes:spent*2,retained_capacity_bytes:spent*3,released_bytes:spent*4};
  budget.work=u128::from(work);
  (budget.held_items,budget.held_copy_bytes,budget.held_capacity_bytes,budget.held_release_bytes,budget.held_fuel)=(held,held*2,held*3,held*4,held_fuel);
  if row["refused"]==true{budget.failure=Some((ValueRefusalKind::Canceled,"original observation refused"));}
  let visible=origin(&row["visible"]).map(|origin|(origin.grant,origin.fuel));
  assert_eq!(budget.is_bound(),visible.is_some(),"{row}");
  assert_eq!(budget.origin().map(|origin|(origin.grant,origin.fuel)),visible,"{row}");
  let snapshot=budget.snapshot();
  let (items,fuel)=observation_pair(&row["snapshot"]);
  let depth=usize::from(budget.origin.is_some());
  assert_eq!((snapshot.granted,snapshot.fuel),(observation_stock(items,depth),fuel),"{row}");
  assert_eq!((snapshot.spent,snapshot.work,snapshot.held_items,snapshot.held_copy_bytes,snapshot.held_capacity_bytes,snapshot.held_release_bytes,snapshot.held_fuel),
    (budget.spent,budget.work,held,held*2,held*3,held*4,held_fuel),"{row}");
  assert_observation_balance(budget.raw_remaining(),&row["raw"],depth);
  assert_observation_balance(budget.remaining(),&row["net"],depth);
  budget.origin_prior=None;
  budget.origin_pending=None;
  budget.origin_commit=None;
 }
}
