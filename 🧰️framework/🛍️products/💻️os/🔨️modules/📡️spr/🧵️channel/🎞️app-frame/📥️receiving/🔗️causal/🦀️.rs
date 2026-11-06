//! 🔗️ Causal rows keep their shared actor, canonical payload pages and every pending wire field.
use super::*;
#[path="📦️payload/🦀️.rs"]mod payload;
use payload::Payload;
#[derive(Default,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub(super) struct Envelope{texts:[Option<String>;11],numbers:[Option<u64>;7],actor:Option<protocol::ActorId>,dependencies:Vec<protocol::MutationId>,target:Vec<String>,payloads:[Payload;2],field:OriginalCommandField,varint:OriginalCommandVarint,dependencies_reserved:bool,target_reserved:bool,stage:u8,value:Option<protocol::causal::MutationEnvelope>}
impl Envelope{
 fn text(&mut self,source:&[u8],offset:&mut usize,index:usize,maximum:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.texts[index].is_some(){return Ok(true)}let turn=if self.field.bytes().is_some(){self.field.transfer_text_source_step(&mut self.texts[index],cx.retained_grant(),cx.fuel_remaining())}else{self.field.step(&mut OriginalBinarySlice::new(source,offset)?,maximum,true,cx.retained_grant(),cx.fuel_remaining())};debit(turn,cx)?;Ok(false)}
 fn number(&mut self,source:&[u8],offset:&mut usize,index:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.numbers[index].is_some(){return Ok(true)}let turn=if self.varint.value().is_some(){self.varint.transfer_source_step(&mut self.numbers[index],cx.retained_grant(),cx.fuel_remaining())}else{self.varint.step(&mut OriginalBinarySlice::new(source,offset)?,cx.retained_grant(),cx.fuel_remaining())};debit(turn,cx)?;Ok(false)}
 fn t(&mut self,index:usize)->String{self.texts[index].take().unwrap()}
 fn count(&self,index:usize,maximum:usize)->Result<usize,OriginalTurnFailure>{let n=self.numbers[index].unwrap();if n>maximum as u64{return Err(invalid("causal AppFrame roster exceeds its declared count"))}Ok(n as usize)}
 pub fn advance(&mut self,source:&[u8],offset:&mut usize,limits:NativeAppFrameLimits,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.value.is_some(){return Ok(true)}let ready=match self.stage{
   0..=2=>self.text(source,offset,self.stage as usize,limits.maximum_field_bytes,cx)?,
   3=>{if self.actor.is_some(){true}else{let copy=size_of::<Option<String>>()+size_of::<Option<protocol::ActorId>>();let capacity=semio_framework_value::shared_retirement_allocation_bytes::<String>();let grant=cx.retained_grant();if cx.fuel_remaining()<1||!retained_clone_grant_admits(grant,Demand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()}){return Ok(false)}self.actor=Some(protocol::ActorId(semio_framework_value::SharedUtf8::from_funded_arc(std::sync::Arc::new(self.texts[2].take().unwrap()))));cx.consume_fuel(1);cx.consume_retained(Progress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()})?;return Ok(false)}},
   4=>{if !self.number(source,offset,0,cx)?{return Ok(false)}let n=self.count(0,limits.maximum_entries)?;lists::reserve(&mut self.dependencies,n,&mut self.dependencies_reserved,cx)?},
   5=>{let n=self.count(0,limits.maximum_entries)?;if self.dependencies.len()==n{true}else{if !self.text(source,offset,10,limits.maximum_field_bytes,cx)?{return Ok(false)}if pay(cx,size_of::<Option<String>>()+size_of::<protocol::MutationId>()+size_of::<Vec<protocol::MutationId>>(),1)?{let item=self.t(10);self.dependencies.push(protocol::MutationId(item))}return Ok(false)}},
   6=>{if !self.number(source,offset,1,cx)?{return Ok(false)}if self.numbers[1].unwrap()>1{return Err(invalid("causal observed presence must be zero or one"))}true},
   7=>if self.numbers[1]==Some(1){self.text(source,offset,9,limits.maximum_field_bytes,cx)?}else{true},
   8=>{if !self.number(source,offset,2,cx)?{return Ok(false)}let n=self.count(2,limits.maximum_entries)?;lists::reserve(&mut self.target,n,&mut self.target_reserved,cx)?},
   9=>{let n=self.count(2,limits.maximum_entries)?;if self.target.len()==n{true}else{if !self.text(source,offset,10,limits.maximum_field_bytes,cx)?{return Ok(false)}if pay(cx,size_of::<Option<String>>()+size_of::<String>()+size_of::<Vec<String>>(),1)?{let item=self.t(10);self.target.push(item)}return Ok(false)}},
   10=>self.text(source,offset,3,limits.maximum_field_bytes,cx)?,11=>self.payloads[0].advance(source,offset,limits.maximum_field_bytes,cx)?,
   12=>self.text(source,offset,4,limits.maximum_field_bytes,cx)?,13=>self.payloads[1].advance(source,offset,limits.maximum_field_bytes,cx)?,
   14..=16=>self.number(source,offset,(self.stage-11)as usize,cx)?,
   17=>{if !self.number(source,offset,6,cx)?{return Ok(false)}if self.numbers[6].unwrap()>7{return Err(invalid("causal trailing flags exceed their authored bit set"))}true},
   18|19=>if self.numbers[6].unwrap()&1!=0{self.text(source,offset,(self.stage-13)as usize,limits.maximum_field_bytes,cx)?}else{true},
   20=>if self.numbers[6].unwrap()&2!=0{self.text(source,offset,7,limits.maximum_field_bytes,cx)?}else{true},
   21=>if self.numbers[6].unwrap()&4!=0{self.text(source,offset,8,limits.maximum_field_bytes,cx)?}else{true},
   22=>{if !pay(cx,size_of::<Self>()+size_of::<Option<protocol::causal::MutationEnvelope>>(),1)?{return Ok(false)}let flags=self.numbers[6].unwrap();let value=protocol::causal::MutationEnvelope{mutation_id:protocol::MutationId(self.t(0)),document_id:protocol::ArtifactId(self.t(1)),actor:self.actor.take().unwrap(),dependencies:std::mem::take(&mut self.dependencies),observed:if self.numbers[1]==Some(1){Some(protocol::MutationId(self.t(9)))}else{None},target:std::mem::take(&mut self.target),diff:protocol::ArtifactDiff{schema:protocol::SchemaId(self.t(3)),payload:self.payloads[0].take()},inverse:protocol::InverseMutation{schema:protocol::SchemaId(self.t(4)),payload:self.payloads[1].take()},timestamp:protocol::HybridLogicalTimestamp{actor:self.numbers[3].unwrap(),physical_ms:self.numbers[4].unwrap(),logical:self.numbers[5].unwrap()},transaction:if flags&1!=0{Some(protocol::TransactionRef{id:self.t(5),tool:self.t(6)})}else{None},verb:if flags&2!=0{Some(self.t(7))}else{None},line:if flags&4!=0{Some(self.t(8))}else{None}};self.value=Some(value);return Ok(false)},
   _=>return Err(invalid("causal AppFrame decoder has an invalid frontier"))
  };if ready&&pay(cx,size_of::<u8>(),1)?{self.stage+=1}Ok(false)
 }
 pub fn take(&mut self)->protocol::causal::MutationEnvelope{let value=self.value.take().unwrap();*self=Self::default();value}
}
