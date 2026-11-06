use super::*;
#[derive(Clone,Copy)]
struct Origin{recipient:usize,grant:RetainedCloneGrant,fuel:u64,spent:P,work:u128}
impl OriginalReturnOrigin for Origin{
 fn same_origin(&self,other:&Self)->bool{self.recipient==other.recipient}
 fn recipient(&self)->usize{self.recipient}
 fn source_identity(&self)->usize{1}
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
