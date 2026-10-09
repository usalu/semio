//! ♻️ Returns each original locale cell allocation under an exact caller release grant.

use super::{Cow,Locale,LocalizedLabel,Terminology};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::mem::ManuallyDrop;

struct LocalizedLabelRetirement{cells:ManuallyDrop<[[Cow<'static,str>;Locale::COUNT];Terminology::COUNT]>,index:usize}
impl LocalizedLabelRetirement{
 fn new(label:LocalizedLabel)->Self{Self{cells:ManuallyDrop::new(label.cells),index:0}}
 fn current(&self)->Option<&Cow<'static,str>>{if self.terminal_is_empty(){None}else{Some(&self.cells[self.index/Locale::COUNT][self.index%Locale::COUNT])}}
}
impl RetirementCursor for LocalizedLabelRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if self.terminal_is_empty(){return RetirementStep::Complete;}
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}
  if grant.maximum_depth==0{return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit,"localized label retirement requires one admitted depth"));}
  let bytes=self.next_close_byte_demand().unwrap();
  if bytes>grant.maximum_release_bytes{return RetirementStep::BudgetExhausted;}
  self.cells[self.index/Locale::COUNT][self.index%Locale::COUNT]=Cow::Borrowed("");self.index+=1;
  RetirementStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})
 }
 fn terminal_is_empty(&self)->bool{self.index==Terminology::COUNT*Locale::COUNT}
 fn next_close_byte_demand(&self)->Option<usize>{Some(match self.current(){Some(Cow::Owned(text))=>text.capacity(),_=>0})}
 fn next_birth_bytes(&self,_copy:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for LocalizedLabelRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"localized label retirement abandoned original cells");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cells);}}}}
impl RetireOwned for LocalizedLabel{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(LocalizedLabelRetirement::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<LocalizedLabelRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="../../🧪️tests/♻️localized-label-retirement/🦀️.rs"]
mod localized_label_retirement_tests;
