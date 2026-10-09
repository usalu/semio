//! ✅️ Bounded validation of the same original finished output owner.
use crate::{Registry,Dictionary,Value,Atom,ChannelSpec,Cardinality,EvalError,RetainedCloneGrant,RetainedCloneProgress,SCHEMA_KEY};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};

/// 🧭️ Keeps scalar positions into the original Registry, channels, and immutable output.
#[derive(Clone,Copy,Default)]
pub struct OperatorFinishCursor{slot:usize,record:Option<usize>,channel:usize,rank:usize,value_rank:Option<usize>,item:usize,item_schema:Option<usize>,first_item:Option<usize>,first_schema:Option<usize>,schema:Option<usize>,offset:usize,count:usize,number:usize,digits:bool,numeric:bool,phase:u8,progress:RetainedCloneProgress}
semio_framework_value::artifact_retire_leaf!(OperatorFinishCursor);
impl OperatorFinishCursor{
 pub fn new()->Self{Self::default()}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 fn value<'a>(&self,output:&'a Dictionary)->Option<&'a Value>{output.entry_at_rank(self.value_rank?).map(|(_,value)|value)}
 fn list<'a>(&self,output:&'a Dictionary)->Option<&'a Dictionary>{self.value(output)?.as_dictionary()}
 fn item<'a>(&self,output:&'a Dictionary,rank:usize)->Option<&'a Value>{self.list(output)?.entry_at_rank(rank).map(|(_,value)|value)}
 fn schema(dictionary:&Dictionary,rank:Option<usize>)->&str{rank.and_then(|rank|dictionary.entry_at_rank(rank)).and_then(|(_,value)|value.as_atom()).and_then(Atom::as_str).unwrap_or("")}
 fn compare(&mut self,left:&str,right:&str,grant:RetainedCloneGrant)->Option<bool>{let length=left.len().min(right.len());let count=(length-self.offset).min(grant.maximum_copy_bytes);if count==0&&self.offset<length{self.progress=Default::default();return None}let same=left.as_bytes()[self.offset..self.offset+count]==right.as_bytes()[self.offset..self.offset+count];self.offset+=count;self.progress.copied_bytes=count;if !same{self.offset=0;Some(false)}else if self.offset==length{self.offset=0;Some(left.len()==right.len())}else{None}}
 fn next_channel(&mut self){self.channel+=1;self.rank=0;self.offset=0;self.value_rank=None;self.schema=None;self.first_item=None;self.first_schema=None;self.item_schema=None;self.item=0;self.count=0;self.phase=1;}
 fn next_item(&mut self){self.item+=1;self.offset=0;self.number=0;self.digits=false;self.numeric=true;self.phase=6;}
 fn invalid(&self,message:&'static str)->EvalError{ValueError::literal(ValueRefusalKind::InvalidValue,message).with_retained_progress(self.progress).into()}
}
impl Registry{
 /// 🪙️ Quotes one exact borrowed output contract event without minting a wallet.
 pub fn next_finish_job_demands(&self,kind:&str,output:&Option<Dictionary>,cursor:&OperatorFinishCursor,_copy:usize)->Result<RetirementDemand,ValueError>{
  let original=output.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original finished output is absent"))?;let bytes=match cursor.phase{
   0=>self.operators.slot_entry(cursor.slot).map_or(0,|(key,_)|usize::from(cursor.offset<key.len().min(kind.len()))),
   1=>{let channel=&self.operators.slot_entry(cursor.record.unwrap()).unwrap().1.info.outputs;if let Some(channel)=channel.get(cursor.channel){usize::from(channel.name.len()==1)}else{0}},
   2=>original.entry_at_rank(cursor.rank).map_or(0,|(key,_)|usize::from(cursor.offset<key.len().min(self.operators.slot_entry(cursor.record.unwrap()).unwrap().1.info.outputs[cursor.channel].name.len()))),
   4=>cursor.list(original).and_then(|list|list.entry_at_rank(cursor.rank)).map_or(0,|(key,_)|usize::from(cursor.offset<key.len().min(SCHEMA_KEY.len()))),
   5=>usize::from(cursor.offset<OperatorFinishCursor::schema(cursor.list(original).unwrap(),cursor.schema).len().min(4)),
   6=>cursor.list(original).and_then(|list|list.entry_at_rank(cursor.item)).map_or(0,|(key,_)|usize::from(cursor.offset<key.len())),
   8=>cursor.item(original,cursor.item).and_then(Value::as_dictionary).and_then(|item|item.entry_at_rank(cursor.rank)).map_or(0,|(key,_)|usize::from(cursor.offset<key.len().min(SCHEMA_KEY.len()))),
   9=>{let first=cursor.item(original,cursor.first_item.unwrap()).and_then(Value::as_dictionary).unwrap();let item=cursor.item(original,cursor.item).and_then(Value::as_dictionary).unwrap();usize::from(cursor.offset<OperatorFinishCursor::schema(first,cursor.first_schema).len().min(OperatorFinishCursor::schema(item,cursor.item_schema).len()))},_=>0};Ok(RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()})
 }
 /// 🧵️ Validates one event and hands back the exact original output only after its full contract.
 pub fn finish_job(&self,kind:&str,output:&mut Option<Dictionary>,cursor:&mut OperatorFinishCursor,grant:RetainedCloneGrant)->Result<Option<Dictionary>,EvalError>{
  cursor.progress=Default::default();if grant.maximum_items==0{return Ok(None)}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original output contract requires admitted depth").into())}let demand=self.next_finish_job_demands(kind,output,cursor,grant.maximum_copy_bytes)?;if grant.maximum_copy_bytes<demand.copy_bytes{return Ok(None)}let original=output.as_ref().unwrap();cursor.progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  if cursor.phase==0{if cursor.slot==self.operators.slot_count(){return Err(cursor.invalid("original finished operator is absent"))}if let Some((key,_))=self.operators.slot_entry(cursor.slot){match cursor.compare(key,kind,grant){Some(true)=>{cursor.record=Some(cursor.slot);cursor.phase=1;},Some(false)=>cursor.slot+=1,None=>{}}}else{cursor.slot+=1}return Ok(None)}
  let record=self.operators.slot_entry(cursor.record.unwrap()).unwrap().1;if record.info.variadic_output.is_some(){cursor.phase=10;}
  let channel=record.info.outputs.get(cursor.channel);
  match cursor.phase{
   1=>{if let Some(channel)=channel{if channel.name.len()==1{cursor.progress.copied_bytes=1;if channel.name.as_bytes()[0]==b'*'{cursor.next_channel();return Ok(None)}}cursor.rank=0;cursor.phase=2;}else{cursor.phase=10;}},
   2=>{if let Some((key,_))=original.entry_at_rank(cursor.rank){match cursor.compare(key,&channel.unwrap().name,grant){Some(true)=>{cursor.value_rank=Some(cursor.rank);cursor.phase=3;cursor.rank=0;},Some(false)=>cursor.rank+=1,None=>{}}}else{cursor.value_rank=None;cursor.phase=3;cursor.rank=0;}},
   3=>{let channel=channel.unwrap();let value=cursor.value(original);if value.is_some_and(Value::is_null){cursor.next_channel();}else if channel.cardinality.is_collection(){if value.is_none(){if !channel.cardinality.accepts(0){return Err(cursor.invalid("original collection output rejects zero cardinality"))}cursor.next_channel();}else if value.and_then(Value::as_dictionary).is_none(){return Err(cursor.invalid("original collection output is not a list dictionary"))}else{cursor.rank=0;cursor.phase=4;}}else if !channel.cardinality.accepts(usize::from(value.is_some())){return Err(cursor.invalid("original output rejects scalar cardinality"))}else{cursor.next_channel();}},
   4=>{let list=cursor.list(original).unwrap();if let Some((key,_))=list.entry_at_rank(cursor.rank){match cursor.compare(key,SCHEMA_KEY,grant){Some(true)=>{cursor.schema=Some(cursor.rank);cursor.phase=5;},Some(false)=>cursor.rank+=1,None=>{}}}else{cursor.schema=None;cursor.phase=5;}},
   5=>{let schema=OperatorFinishCursor::schema(cursor.list(original).unwrap(),cursor.schema);match cursor.compare(schema,"list",grant){Some(true)=>{cursor.item=0;cursor.offset=0;cursor.number=0;cursor.digits=false;cursor.numeric=true;cursor.phase=6;},Some(false)=>return Err(cursor.invalid("original collection output lacks list schema")),None=>{}}},
   6=>{let list=cursor.list(original).unwrap();if let Some((key,_))=list.entry_at_rank(cursor.item){let count=(key.len()-cursor.offset).min(grant.maximum_copy_bytes);for index in cursor.offset..cursor.offset+count{let byte=key.as_bytes()[index];if index==0&&byte==b'+'{}else if byte.is_ascii_digit(){cursor.digits=true;if let Some(number)=cursor.number.checked_mul(10).and_then(|number|number.checked_add((byte-b'0')as usize)){cursor.number=number;}else{cursor.numeric=false;}}else{cursor.numeric=false;}}cursor.offset+=count;cursor.progress.copied_bytes=count;if cursor.offset==key.len(){cursor.offset=0;if cursor.numeric&&cursor.digits{cursor.count+=1;cursor.phase=7;}else{cursor.next_item();}}}else if !channel.unwrap().cardinality.accepts(cursor.count){return Err(cursor.invalid("original list output rejects exact cardinality"))}else{cursor.next_channel();}},
   7=>{let value=cursor.item(original,cursor.item).unwrap();if value.is_null(){cursor.next_item();}else if value.as_dictionary().is_none(){return Err(cursor.invalid("original list output item is not a dictionary"))}else{cursor.rank=0;cursor.item_schema=None;cursor.phase=8;}},
   8=>{let item=cursor.item(original,cursor.item).and_then(Value::as_dictionary).unwrap();if let Some((key,_))=item.entry_at_rank(cursor.rank){match cursor.compare(key,SCHEMA_KEY,grant){Some(true)=>{cursor.item_schema=Some(cursor.rank);cursor.phase=9;},Some(false)=>cursor.rank+=1,None=>{}}}else{cursor.item_schema=None;cursor.phase=9;}if cursor.phase==9&&cursor.first_item.is_none(){cursor.first_item=Some(cursor.item);cursor.first_schema=cursor.item_schema;cursor.next_item();}},
   9=>{let first=cursor.item(original,cursor.first_item.unwrap()).and_then(Value::as_dictionary).unwrap();let item=cursor.item(original,cursor.item).and_then(Value::as_dictionary).unwrap();match cursor.compare(OperatorFinishCursor::schema(first,cursor.first_schema),OperatorFinishCursor::schema(item,cursor.item_schema),grant){Some(true)=>cursor.next_item(),Some(false)=>return Err(cursor.invalid("original list output mixes item schemas")),None=>{}}},
   10=>{cursor.phase=11;return Ok(output.take())},
   _=>return Err(cursor.invalid("original finished output already handed off")),
  }Ok(None)
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
