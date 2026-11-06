//! 🪪️ A paid fixed root-header inspection supplies later source-free selection bounds.
use super::*;
use semio_framework_value_control_contract::RetainedWorkOutcome;
const HEIGHT:usize=(usize::BITS as usize+3)/4;
#[derive(Clone,Copy,Debug)]
pub struct PagedListSelectionHeader{
 pub(super) binding:(usize,usize,usize,usize,usize,usize,usize,usize,usize),
 pub(super) root_height:usize,pub(super) target_height:usize,
}
impl PagedListSelectionHeader{
 /// ⛽️ Cached header metadata quotes the next reservation without inspecting a page.
 pub const fn reservation_selection_work_demand(&self,limit:usize)->u64{if limit<=self.binding.5{1}else if self.target_height>self.root_height{2}else{2*self.root_height as u64+3}}
 /// ♻️ Cached root height bounds every reverse child probe and selected result.
 pub const fn release_selection_work_demand(&self)->u64{(FANOUT as u64+2)*self.root_height as u64+3}
}
impl<T,const N:usize>PagedList<T,N>{
 /// 🔎️ One fixed metadata turn reads the root header and performs only fixed-width input arithmetic.
 pub fn inspect_selection_header(&self,maximum_work_units:u64)->RetainedWorkOutcome<Option<PagedListSelectionHeader>,PagedListError>{
  if maximum_work_units==0{return RetainedWorkOutcome{result:Ok(None),work:0}}
  let root_height=self.height();if root_height>HEIGHT{return RetainedWorkOutcome{result:Err(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"paged selection root height exceeds addressable page indices"}),work:1}}
  let page=self.capacity/self.page_items();let target_height=if page==0{0}else{(usize::BITS as usize-page.leading_zeros()as usize+3)/4};
  RetainedWorkOutcome{result:Ok(Some(PagedListSelectionHeader{binding:self.reservation_binding(),root_height,target_height})),work:1}
 }
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
