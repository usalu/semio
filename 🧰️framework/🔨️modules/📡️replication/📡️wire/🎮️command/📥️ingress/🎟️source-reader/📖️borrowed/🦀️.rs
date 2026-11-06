//! 📖️ A bounded reader borrows original immutable bytes without adopting or duplicating their owner.
use super::*;
/// 🛣️ Binary atoms share bounded reads across retained pages and original borrowed payloads.
pub trait OriginalBinaryReadSource{
 fn source_page_exhausted(&self)->bool;
 fn source_input_consumed(&self)->bool;
 fn admit_field_source_step(&mut self,output:&mut Option<Vec<u8>>,length:usize,grant:Grant,work:u64)->Turn<Step>;
 fn read_byte_source_step(&mut self,output:&mut Option<u8>,grant:Grant,work:u64)->Turn<Step>;
 fn copy_field_source_step(&mut self,output:&mut Vec<u8>,length:usize,grant:Grant,work:u64)->Turn<Step>;
 fn release_consumed_source_step(&mut self,grant:Grant,work:u64)->Turn<Step>;
}
impl OriginalBinaryReadSource for PagedCommandReader{
 fn admit_field_source_step(&mut self,output:&mut Option<Vec<u8>>,length:usize,grant:Grant,work:u64)->Turn<Step>{PagedCommandReader::admit_field_source_step(output,length,grant,work)}
 fn source_page_exhausted(&self)->bool{PagedCommandReader::source_page_exhausted(self)}
 fn source_input_consumed(&self)->bool{PagedCommandReader::source_input_consumed(self)}
 fn read_byte_source_step(&mut self,output:&mut Option<u8>,grant:Grant,work:u64)->Turn<Step>{PagedCommandReader::read_byte_source_step(self,output,grant,work)}
 fn copy_field_source_step(&mut self,output:&mut Vec<u8>,length:usize,grant:Grant,work:u64)->Turn<Step>{PagedCommandReader::copy_field_source_step(self,output,length,grant,work)}
 fn release_consumed_source_step(&mut self,grant:Grant,work:u64)->Turn<Step>{PagedCommandReader::release_consumed_source_step(self,grant,work)}
}
/// 🔎️ Only the enclosing Source owner retains identity; this short borrow carries no authority.
pub struct OriginalBinarySlice<'a>{bytes:&'a[u8],offset:&'a mut usize}
impl<'a> OriginalBinarySlice<'a>{
 pub fn new(bytes:&'a[u8],offset:&'a mut usize)->Result<Self,ValueError>{if *offset>bytes.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed binary offset exceeds its actual source"))}Ok(Self{bytes,offset})}
}
impl OriginalBinaryReadSource for OriginalBinarySlice<'_>{
 fn admit_field_source_step(&mut self,output:&mut Option<Vec<u8>>,length:usize,grant:Grant,work:u64)->Turn<Step>{admit_binary_field_source_step(output,length,self.bytes.len(),grant,work)}
 fn source_page_exhausted(&self)->bool{false}
 fn source_input_consumed(&self)->bool{*self.offset==self.bytes.len()}
 fn read_byte_source_step(&mut self,output:&mut Option<u8>,grant:Grant,work:u64)->Turn<Step>{
  if output.is_some(){return refused("borrowed binary byte requires an empty destination")}
  let copy=size_of::<Option<u8>>()+size_of::<usize>();if work<1||!retained_clone_grant_admits(grant,Demand{copy_bytes:copy,depth:1,..Default::default()}){return waiting()}
  let Some(byte)=self.bytes.get(*self.offset).copied()else{return Turn{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"borrowed binary ended inside a field")),work:0}};
  *output=Some(byte);*self.offset+=1;complete(Progress{copied_items:1,copied_bytes:copy,..Default::default()},1)
 }
 fn copy_field_source_step(&mut self,output:&mut Vec<u8>,length:usize,grant:Grant,work:u64)->Turn<Step>{
  if output.len()>length||output.capacity()<length{return refused("borrowed binary field lost its admitted capacity")}
  if output.len()==length{return complete(Default::default(),0)}
  if grant.maximum_items==0||grant.maximum_depth==0{return waiting()}
  let Some(available)=self.bytes.get(*self.offset..)else{return refused("borrowed binary read exceeds its actual source")};
  if available.is_empty(){return Turn{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"borrowed binary ended inside a field")),work:0}}
  let metadata=size_of::<Vec<u8>>()+size_of::<usize>();let n=(length-output.len()).min(available.len()).min(output.capacity()-output.len()).min(grant.maximum_copy_bytes.saturating_sub(metadata)).min(usize::try_from(work.saturating_sub(1)).unwrap_or(usize::MAX));if n==0{return waiting()}
  output.extend_from_slice(&available[..n]);*self.offset+=n;let progress=Progress{copied_items:1,copied_bytes:metadata+n,..Default::default()};Turn{result:Ok(if output.len()==length{Step::Complete(progress)}else{Step::Progress(progress)}),work:n as u64+1}
 }
 fn release_consumed_source_step(&mut self,_grant:Grant,_work:u64)->Turn<Step>{complete(Default::default(),0)}
}
