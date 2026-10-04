//! 🗂️ Concrete controlled slots for schema producer identities and semantic digest buckets.
use semio_framework_dsl_record::NativeSchemaControl;
use semio_framework_value::{ValueError,ValueRefusalKind};
pub(super)trait Key:Eq{fn index_hash(&self)->usize;}
fn hash(bytes:impl IntoIterator<Item=u8>)->usize{let mut hash=0xcbf29ce484222325u64;for byte in bytes{hash=(hash^u64::from(byte)).wrapping_mul(0x100000001b3);}hash as usize}
impl Key for usize{fn index_hash(&self)->usize{hash(self.to_le_bytes())}}
impl Key for [u8;32]{fn index_hash(&self)->usize{hash(self.iter().copied())}}
pub(super)struct Index<K,V>{slots:Vec<Option<(K,V)>>,length:usize}
enum Position{Found(usize),Vacant(usize)}
impl<K:Key,V>Index<K,V>{
 pub(super)fn new()->Self{Self{slots:Vec::new(),length:0}}
 fn position<C:NativeSchemaControl>(&self,key:&K,control:&mut C)->Result<Position,ValueError>{
  control.scoped_stage(|control|{
   control.begin_stage(0)?;let count=self.slots.len();
   if count==0{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"schema lookup requires initialized slots"))}
   let mut index=key.index_hash()&(count-1);
   for _ in 0..count{control.step()?;match &self.slots[index]{None=>return Ok(Position::Vacant(index)),Some((stored,_))if stored==key=>return Ok(Position::Found(index)),_=>index=(index+1)&(count-1)}}
   Err(ValueError::new(ValueRefusalKind::InvariantViolated,"schema index exhausted its controlled probe domain"))
  })
 }
 pub(super)fn lookup<C:NativeSchemaControl>(&self,key:&K,control:&mut C)->Result<Option<&V>,ValueError>{
  if self.slots.is_empty(){return Ok(None)}
  Ok(match self.position(key,control)?{Position::Found(index)=>Some(&self.slots[index].as_ref().expect("found schema slot").1),Position::Vacant(_)=>None})
 }
 pub(super)fn lookup_mut<C:NativeSchemaControl>(&mut self,key:&K,control:&mut C)->Result<Option<&mut V>,ValueError>{
  if self.slots.is_empty(){return Ok(None)}
  Ok(match self.position(key,control)?{Position::Found(index)=>Some(&mut self.slots[index].as_mut().expect("found schema slot").1),Position::Vacant(_)=>None})
 }
 fn grow<C:NativeSchemaControl>(&mut self,control:&mut C)->Result<(),ValueError>{
  let capacity=if self.slots.is_empty(){2}else{self.slots.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"schema slot capacity overflow"))?};
  let mut next=control.allocate_vec(capacity)?;
  control.scoped_stage(|control|{control.begin_stage(capacity)?;for _ in 0..capacity{next.push(None);control.step()?;}Ok::<_,ValueError>(())})?;
  let mut previous=std::mem::replace(&mut self.slots,next);self.length=0;
  control.scoped_stage(|control|{
   control.begin_stage(previous.len())?;
   for slot in &mut previous{
    if let Some(entry)=slot.take(){
     let Position::Vacant(index)=self.position(&entry.0,control)?else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"schema replacement encountered duplicate identities"))};
     self.slots[index]=Some(entry);self.length=self.length.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"schema identity count overflow"))?;
    }
    control.step()?;
   }Ok(())
  })
 }
 pub(super)fn bind<C:NativeSchemaControl>(&mut self,key:K,value:V,control:&mut C)->Result<(),ValueError>{
  let length=self.length.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"schema identity count overflow"))?;
  let demand=length.checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"schema index load overflow"))?;
  if demand>self.slots.len(){self.grow(control)?;}
  let Position::Vacant(index)=self.position(&key,control)?else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"schema index identity already bound"))};
  self.slots[index]=Some((key,value));self.length=length;Ok(())
 }
}

