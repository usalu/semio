//! 🎟️ Bounded command reads retain original pages until their exact physical release is funded.
use super::*;
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetainedWorkOutcome as Turn,RetirementDemand as Demand,ValueError,ValueRefusalKind,retained_clone_grant_admits};
fn refused(message:&'static str)->Turn<Step>{Turn{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,message)),work:0}}
fn waiting()->Turn<Step>{Turn{result:Ok(Step::Progress(Default::default())),work:0}}
fn complete(progress:Progress,work:u64)->Turn<Step>{Turn{result:Ok(Step::Complete(progress)),work}}
fn admit_binary_field_source_step(output:&mut Option<Vec<u8>>,length:usize,maximum:usize,grant:Grant,work:u64)->Turn<Step>{
  if output.is_some(){return refused("command field admission requires an empty destination")}
  if length>maximum{return Turn{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"command field exceeds its assembled authority")),work:0}}
  let demand=PagedCommandReader::source_field_demand(length);if work<1||!retained_clone_grant_admits(grant,demand){return waiting()}
  let mut bytes=Vec::new();if bytes.try_reserve_exact(length).is_err(){return Turn{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"command field backing allocation refused")),work:1}}
  let progress=Progress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:bytes.capacity(),..Default::default()};*output=Some(bytes);
  if !progress.fits(grant){return Turn{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"command field backing exceeds its funded quote").with_retained_progress(progress)),work:1}}
  complete(progress,1)
 }
impl PagedCommandReader{
 /// 🏠️ Provides an allocation-free vacant home for a paid original reader transfer.
 pub fn empty_source_home()->Self{Self{command:PagedCommand{pages:std::collections::VecDeque::new(),byte_len:0,kind:0,metadata:0,item_count:0},offset:0}}
 /// 📏️ Quotes the actual field home and byte backing before admission.
 pub fn source_field_demand(length:usize)->Demand{Demand{copy_bytes:size_of::<Option<Vec<u8>>>(),capacity_bytes:length,depth:1,..Default::default()}}
 /// 🌱️ Reserves a field once and preserves actual capacity even if allocation exceeds its quote.
 pub fn admit_field_source_step(output:&mut Option<Vec<u8>>,length:usize,grant:Grant,work:u64)->Turn<Step>{admit_binary_field_source_step(output,length,COMMAND_MAXIMUM_BYTES,grant,work)}
 /// 📄️ Reports an exhausted physical page without releasing its backing.
 pub fn source_page_exhausted(&self)->bool{self.command.pages.front().is_some_and(|page|self.offset==page.len())}
 /// 🏁️ Checks logical input completion independently of retained page and spine ownership.
 pub fn source_input_consumed(&self)->bool{self.command.byte_len==self.offset}
 /// 🔎️ Borrows one original byte only after funding its output and cursor placements.
 pub fn read_byte_source_step(&mut self,output:&mut Option<u8>,grant:Grant,work:u64)->Turn<Step>{
  if output.is_some(){return refused("command byte admission requires an empty destination")}
  let bytes=size_of::<Option<u8>>()+size_of::<usize>();let demand=Demand{copy_bytes:bytes,depth:1,..Default::default()};
  if work<1||!retained_clone_grant_admits(grant,demand){return waiting()}
  if self.source_page_exhausted(){return waiting()}
  let Some(byte)=self.command.pages.front().and_then(|page|page.as_slice().get(self.offset)).copied()else{return Turn{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"paged command ended inside a field")),work:0}};
  *output=Some(byte);self.offset+=1;complete(Progress{copied_items:1,copied_bytes:bytes,..Default::default()},1)
 }
 /// 📥️ Copies only the funded field chunk into already admitted spare capacity.
 pub fn copy_field_source_step(&mut self,output:&mut Vec<u8>,length:usize,grant:Grant,work:u64)->Turn<Step>{
  if output.len()>length||output.capacity()<length{return refused("command field lost its exact reserved authority")}
  if output.len()==length{return complete(Default::default(),0)}
  if grant.maximum_items==0||grant.maximum_depth==0{return waiting()}
  if self.source_page_exhausted(){return waiting()}
  let Some(page)=self.command.pages.front()else{return Turn{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"paged command ended inside a field")),work:0}};
  let Some(available)=page.as_slice().get(self.offset..)else{return refused("command reader offset exceeds its retained page")};
  let metadata=size_of::<Vec<u8>>()+size_of::<usize>();
  let n=(length-output.len()).min(available.len()).min(output.capacity()-output.len()).min(grant.maximum_copy_bytes.saturating_sub(metadata)).min(usize::try_from(work.saturating_sub(1)).unwrap_or(usize::MAX));
  if n==0{return waiting()}
  output.extend_from_slice(&available[..n]);self.offset+=n;
  let progress=Progress{copied_items:1,copied_bytes:metadata+n,..Default::default()};
  Turn{result:Ok(if output.len()==length{Step::Complete(progress)}else{Step::Progress(progress)}),work:n as u64+1}
 }
 /// 🪦️ Releases the actual 4096-byte block only after every physical and metadata axis is funded.
 pub fn release_consumed_source_step(&mut self,grant:Grant,work:u64)->Turn<Step>{
  let Some(page)=self.command.pages.front()else{return complete(Default::default(),0)};
  if self.offset!=page.len(){return refused("command page release requires its complete read frontier")}
  let copy=size_of::<FixedCommandPage>()+size_of::<std::collections::VecDeque<FixedCommandPage>>()+2*size_of::<usize>();
  let demand=Demand{copy_bytes:copy,release_bytes:COMMAND_PAGE_MAXIMUM_BYTES,depth:1,..Default::default()};
  if work<1||!retained_clone_grant_admits(grant,demand){return waiting()}
  let page=self.command.pages.pop_front().expect("funded retained command page");self.command.byte_len-=page.len();self.offset=0;drop(page);
  complete(Progress{copied_items:1,copied_bytes:copy,released_bytes:COMMAND_PAGE_MAXIMUM_BYTES,..Default::default()},1)
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="🔤️.rs"]
mod atoms;
pub use atoms::{OriginalCommandVarint,OriginalCommandField};

#[path="📖️borrowed/🦀️.rs"]
mod borrowed;
pub use borrowed::{OriginalBinaryReadSource,OriginalBinarySlice};
