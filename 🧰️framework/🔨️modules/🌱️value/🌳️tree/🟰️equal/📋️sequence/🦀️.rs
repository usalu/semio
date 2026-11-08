//! 📋️ Primitive sequence equality and its original co-located laws.
macro_rules! native_reply_try{($value:expr,$reply:ident)=>{match $value{Ok(value)=>value,Err(error)=>return Ok($reply.complete(Err(error.into())))}}}
semio_framework_value_native_slice_equality::native_equality_sequence!();

#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn inline_sequence_respects_neutral_zero_body_vectors(){
  let rows:serde_json::Value=serde_json::from_str(include_str!("../../../📋️list/🟰️equal/🧪️tests/🧫️fixtures/⛽️work.json")).unwrap();
  for row in rows.as_array().unwrap().iter().filter(|row|row["payloadBodyBytes"].as_u64().unwrap()<2){
   let left:Vec<u8>=row["left"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let right:Vec<u8>=row["right"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let mut c=RetainedSequenceEquality::<u8>::new();let body=row["payloadBodyBytes"].as_u64().unwrap()as usize;let work=row["maximumWork"].as_u64().unwrap();let d=c.demand(body,work).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let mut actual=None;assert_eq!(crate::eff::with_effect(g,work,&mut actual,|effect|RetainedSequenceEquality::<u8>::advance(&mut c,&left,&right,body,work,effect)).work,3);assert!(actual.is_none());assert_eq!(c.demand(body,work).unwrap(),RetirementDemand::default());assert_eq!(c.work_demand(body,work).unwrap(),row["work"].as_u64().unwrap());c.begin_close();while !c.terminal_is_empty(){let d=c.close_demand(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};crate::eff::close_cursor::<Vec<u8>,_>(&mut c,g).result.unwrap();}
  }
 }
}
