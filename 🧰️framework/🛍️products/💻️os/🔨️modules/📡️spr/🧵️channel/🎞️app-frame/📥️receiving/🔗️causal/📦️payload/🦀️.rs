//! 📦️ Causal payload bytes enter their final canonical pages under the original Source recipient.
use super::super::*;
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeReceivingEffect,OriginalReceivingPrimary,owned::tree::{NativeSequenceCollection,NativeSequenceReservation}};
type Collection=protocol::CausalPayload;
type Storage=<Collection as NativeSequenceCollection<u8>>::Storage;
type Reservation=<Collection as NativeSequenceCollection<u8>>::Reservation;
#[derive(Default,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub(super) struct Payload{length:Option<u64>,varint:OriginalCommandVarint,storage:Option<Storage>,reservation:Reservation,storage_effect:NativeReceivingEffect,push_effect:NativeReceivingEffect,retirement_effect:NativeReceivingEffect<semio_framework_value::OriginalRetirementFailure>,byte:Option<u8>,index:usize,stage:u8,output:Option<Collection>}
fn available(control:&mut NativeDecodeControl<'_>)->Result<u64,OriginalReceivingPrimary>{control.available_work_units().map_err(Into::into)}
impl Payload{
 fn count(&self)->usize{self.length.unwrap()as usize}
 pub fn advance(&mut self,source:&[u8],offset:&mut usize,maximum:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.output.is_some(){return Ok(true)}
  match self.stage{
   0=>{if self.length.is_none(){let turn=if self.varint.value().is_some(){self.varint.transfer_source_step(&mut self.length,cx.retained_grant(),cx.fuel_remaining())}else{self.varint.step(&mut OriginalBinarySlice::new(source,offset)?,cx.retained_grant(),cx.fuel_remaining())};debit(turn,cx)?;return Ok(false)}if self.length.unwrap()>maximum as u64||self.length.unwrap()>source.len().saturating_sub(*offset)as u64{return Err(invalid("causal payload exceeds its bounded actual input"))}if pay(cx,size_of::<u8>(),1)?{self.stage=1}},
   1=>{if self.storage.is_none(){let count=self.count();let grant=cx.retained_grant();receive_native_step(cx,|control|{let maximum=match available(control){Ok(maximum)=>maximum,Err(error)=>return Turn{result:Err(error),work:0}};Collection::admit_storage(&mut self.storage,count,&mut self.storage_effect,control,maximum,grant)})?;return Ok(false)}if pay(cx,size_of::<u8>(),1)?{self.stage=2}},
   2=>{let count=self.count();if self.index==count{if pay(cx,size_of::<u8>(),1)?{self.stage=5}return Ok(false)}let storage=self.storage.as_mut().unwrap();if Collection::reserve_demand(storage,self.index,count,&self.reservation)?.is_some(){let grant=cx.retained_grant();receive_native_step(cx,|control|{let maximum=match available(control){Ok(maximum)=>maximum,Err(error)=>return Turn{result:Err(error),work:0}};Collection::reserve_step(storage,self.index,count,&mut self.reservation,control,maximum,grant)})?;return Ok(false)}if pay(cx,size_of::<u8>(),1)?{self.stage=3}},
   3=>{if self.byte.is_none(){debit(OriginalBinarySlice::new(source,offset)?.read_byte_source_step(&mut self.byte,cx.retained_grant(),cx.fuel_remaining()),cx)?;return Ok(false)}if pay(cx,size_of::<u8>(),1)?{self.stage=4}},
   4=>{if self.byte.is_none(){if pay(cx,size_of::<usize>()+size_of::<u8>(),1)?{self.index+=1;self.stage=2}return Ok(false)}let storage=self.storage.as_mut().unwrap();let index=self.index;let demand=Demand{copy_bytes:Collection::push_copy_bytes(storage,index)?,depth:Collection::push_depth(storage,index)?,..Default::default()};receive_native_step(cx,|control|{let maximum=match available(control){Ok(maximum)=>maximum,Err(error)=>return Turn{result:Err(error),work:0}};let work=match Collection::push_work_demand(storage,index){Ok(work)=>work,Err(error)=>return Turn{result:Err(error.into()),work:0}};if maximum<work{return Turn{result:Ok(Step::Progress(Default::default())),work:0}}let turn=control.receive_allocation(demand,&mut self.push_effect,false,work,|g,_|{let turn=Collection::push_step(storage,index,&mut self.byte,maximum,g);Turn{result:turn.result.map(|p|Step::Progress(p.unwrap_or_default())),work:turn.work}});Turn{result:turn.result.map(Step::Progress),work:turn.work}})?;},
   5=>{if !self.reservation.terminal_is_empty(){let demand=self.reservation.close_demand()?;let work=self.reservation.close_work_demand()?;receive_native_step(cx,|control|{let turn=control.receive_retirement(demand,&mut self.retirement_effect,false,work,|g,_|self.reservation.close_step(g));Turn{result:turn.result.map(Step::Progress),work:turn.work}})?;return Ok(false)}if pay(cx,size_of::<Option<Storage>>()+size_of::<Option<Collection>>(),1)?{self.output=Some(Collection::finish(self.storage.take().unwrap()))}},
   _=>return Err(invalid("causal payload has an invalid receiving phase"))
  }Ok(false)
 }
 pub fn take(&mut self)->Collection{self.output.take().unwrap()}
}
