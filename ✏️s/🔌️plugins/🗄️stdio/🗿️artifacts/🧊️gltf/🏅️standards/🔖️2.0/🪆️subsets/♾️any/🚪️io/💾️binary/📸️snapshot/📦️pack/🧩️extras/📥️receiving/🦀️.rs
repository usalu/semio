//! 📥️ GLTF extras receive their flat node list through the derived receiver, then rebuild the tree in one funded mapping turn.
use super::*;
use semio_framework_dsl_record::receiving::owned::{DslOwnedReceivingField,MappedReceiving,OwnedReceivingMap,OwnedTake,admits,done,idle,invariant,receipt};
use semio_framework_value::{RetainedAllocationEffect,RetirementDemand};
pub struct GltfJsonMap;
pub type GltfJsonReceiving=MappedReceiving<Json,GltfJsonMap>;
impl DslOwnedReceivingField for GltfJson{type Cursor=GltfJsonReceiving;fn receiving_cursor()->Self::Cursor{MappedReceiving::new()}}
fn entities(json:&Json)->usize{json.nodes.iter().fold(json.nodes.len(),|total,node|total.saturating_add(node.items.len()).saturating_add(node.members.len()))}
impl OwnedReceivingMap<Json> for GltfJsonMap{
 type Output=GltfJson;
 fn map_demand(input:&Json)->Result<RetirementDemand,ValueError>{let bytes=entities(input).checked_mul(std::mem::size_of::<Option<GltfJson>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"GLTF extras rebuild extent overflow"))?;Ok(RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()})}
 fn map_work_demand(input:&Json)->Result<u64,ValueError>{u64::try_from(entities(input).max(1)).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"GLTF extras rebuild work overflow"))}
 fn map_into(input:&mut Option<Json>,output:&mut Option<GltfJson>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{
  let Some(json)=input.as_ref()else{return done(effect,Err(invariant()))};if output.is_some(){return done(effect,Err(invariant()))}
  let(demand,work)=match(Self::map_demand(json),Self::map_work_demand(json)){(Ok(demand),Ok(work))=>(demand,work),(Err(error),_)|(_,Err(error))=>return done(effect,Err(error))};
  if !admits(effect,demand)||effect.remaining_work_units().map_or(true,|remaining|remaining<work){return idle(effect)}
  let result=effect.performed(work).and_then(|funded|if funded{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"owned receiving action exceeded its original work"))}).and_then(|()|{let json=input.take().ok_or_else(invariant)?;GltfJson::try_from(json).map_err(|message|ValueError::new(ValueRefusalKind::InvalidValue,message))}).map(|value|{*output=Some(value);receipt(demand)});
  done(effect,result)
 }
}
