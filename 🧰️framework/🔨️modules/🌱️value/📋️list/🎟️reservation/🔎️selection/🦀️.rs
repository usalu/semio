//! 🔎️ A work-admitted read selects one current reservation frontier without allocating or moving pages.
use super::*;
use semio_framework_value_control_contract::{RetainedWorkOutcome,RetirementDemand};
const HEIGHT_BOUND:u64=(usize::BITS as u64+3)/4;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PagedListReservationKind{Metadata,Payload}
impl PagedListReservationKind{
 /// 🏷️ Portable names describe the selected allocation without exposing native layout.
 pub const fn as_str(self)->&'static str{match self{Self::Metadata=>"metadata",Self::Payload=>"payload"}}
}
#[derive(Clone,Copy,Debug)]
pub struct PagedListReservationSelection{
 demand:RetirementDemand,kind:PagedListReservationKind,limit:usize,root_height:usize,target_height:usize,execution_work:u64,
 binding:(usize,usize,usize,usize,usize,usize,usize,usize,usize),
}
impl PagedListReservationSelection{
 /// 🧾️ Borrows the original logical extent captured by paid selection.
 pub const fn limit(&self)->usize{self.limit}

 /// 📏️ Quotes the physical allocation selected by this paid read.
 pub fn demand(&self)->RetirementDemand{self.demand}
 /// 🗂️ Distinguishes page metadata from the initialized payload backing.
 pub fn kind(&self)->PagedListReservationKind{self.kind}
}
fn invalid(reason:&'static str)->PagedListError{PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason}}
fn exhausted(reason:&'static str)->PagedListError{PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason}}
impl<T,const N:usize>PagedList<T,N>{
 pub(super) fn reservation_binding(&self)->(usize,usize,usize,usize,usize,usize,usize,usize,usize){(self as*const Self as usize,self.root.as_ptr()as usize,self.root.len(),self.root.capacity(),self.length,self.capacity,self.allocated,self.payload_page_bytes,self.reservation_epoch)}
 /// ⛽️ Two fixed metadata events, at most one nibble event and two link events per addressable level, then the selected node and result.
 pub const fn reservation_selection_work_demand()->u64{3*HEIGHT_BOUND+4}
 /// 🧭️ Selection consumes its own actual work; its result carries no original-source or allocation authority.
 pub fn select_reservation(&self,limit:usize,maximum_work_units:u64)->RetainedWorkOutcome<Option<PagedListReservationSelection>,PagedListError>{self.select_reservation_inner(limit,maximum_work_units,None)}
 /// 🎟️ Reuses a genuinely paid fixed header without another unadmitted page-header read.
 pub fn select_reservation_from_header(&self,header:&PagedListSelectionHeader,limit:usize,maximum_work_units:u64)->RetainedWorkOutcome<Option<PagedListReservationSelection>,PagedListError>{
  if maximum_work_units<header.reservation_selection_work_demand(limit){return RetainedWorkOutcome{result:Ok(None),work:0}}
  if header.binding!=self.reservation_binding(){return RetainedWorkOutcome{result:Err(invalid("paged selection header changed")),work:1}}
  self.select_reservation_inner(limit,maximum_work_units,Some(header))
 }
 fn select_reservation_inner(&self,limit:usize,maximum_work_units:u64,header:Option<&PagedListSelectionHeader>)->RetainedWorkOutcome<Option<PagedListReservationSelection>,PagedListError>{
  let bound=header.map_or(Self::reservation_selection_work_demand(),|header|header.reservation_selection_work_demand(limit));
  if maximum_work_units<bound{return RetainedWorkOutcome{result:Ok(None),work:0}}
  let mut work=1;
  let result=(||{
   if limit>N||limit<self.length{return Err(exhausted("paged reservation final extent is outside its logical authority"))}
   if self.reservation_epoch==usize::MAX{return Err(exhausted("paged reservation epoch exhausted"))}
   if limit<=self.capacity{return Ok(None)}
   let page_items=self.page_items();if self.capacity%page_items!=0{return Err(exhausted("funded final payload extent cannot extend or reallocate"))}
   let page=self.capacity/page_items;let(root_height,target_height)=if let Some(header)=header{(header.root_height,header.target_height)}else{work+=1;let root_height=self.height();if root_height as u64>HEIGHT_BOUND{return Err(invalid("paged reservation root height exceeds addressable page indices"))}let mut remaining=page;let mut target_height=0;while remaining!=0{remaining>>=4;target_height+=1;work+=1}(root_height,target_height)};
   let(kind,bytes,execution_work)=if target_height>root_height{(PagedListReservationKind::Metadata,size_of::<Page<T>>(),FANOUT as u64+6)}else{
    let mut selected=None;let mut link=&self.root;
    for height in (0..=root_height).rev(){
     work+=1;match link.first(){
      None=>{selected=Some((PagedListReservationKind::Metadata,if link.capacity()==0{size_of::<Page<T>>()}else{0},4+2*(root_height-height)as u64+if height==0{1}else{FANOUT as u64+1}));break},
      Some(Page::Branch{children,..})=>{if height==0{return Err(invalid("paged reservation branch exceeds its declared height"))}work+=1;link=&children[Self::slot(page,height)]},
      Some(Page::Leaf{..})=>{let bytes=page_items.min(limit-self.capacity).checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(||exhausted("paged reservation payload layout exceeds its addressable extent"))?;selected=Some((PagedListReservationKind::Payload,bytes,7+2*(root_height-height)as u64));break},
     }
    }
    selected.ok_or_else(||invalid("paged reservation page authority is missing"))?
   };
   self.allocated.checked_add(bytes).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(||exhausted("fixed list allocation counter exceeds addressable ownership"))?;
   work+=1;
   Ok(Some(PagedListReservationSelection{demand:RetirementDemand{capacity_bytes:bytes,depth:root_height.max(target_height)+3,..Default::default()},kind,limit,root_height,target_height,execution_work,binding:self.reservation_binding()}))
  })();RetainedWorkOutcome{result,work}
 }
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
impl PagedListReservationSelection{
 /// 🧱️ Bounds one guarded allocation, its page-path inspections and sixteen fixed branch headers.
 pub const fn allocation_work_demand()->u64{2*HEIGHT_BOUND+FANOUT as u64+8}
 pub const fn reserve_work_demand(&self)->u64{self.execution_work}
}
impl<T,const N:usize>PagedList<T,N>{
 /// 🪙️ Executes this paid frontier under independent work and physical grants, retaining the actual allocation marker.
 pub fn reserve_selected(&mut self,selected:&PagedListReservationSelection,grant:semio_framework_value_control_contract::RetainedCloneGrant,maximum_work_units:u64,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>{self.reserve_selected_using::<ExactAllocation>(selected,grant,maximum_work_units,actual)}
 pub(super) fn reserve_selected_using<A:PageAllocation>(&mut self,selected:&PagedListReservationSelection,grant:semio_framework_value_control_contract::RetainedCloneGrant,maximum_work_units:u64,actual:&mut Option<usize>)->Result<PagedListProgress,PagedListAllocationError>{
  let refused=|work,allocated_bytes,kind,reason|PagedListAllocationError{work,allocated_bytes,kind,reason};
  if maximum_work_units<selected.reserve_work_demand()||!semio_framework_value_control_contract::retained_clone_grant_admits(grant,selected.demand){return Ok(Default::default())}
  if actual.is_some()||selected.binding!=self.reservation_binding(){return Err(refused(1,0,PagedListRefusalKind::InvariantViolated,"paged reservation selected frontier changed"))}
  let next_epoch=self.reservation_epoch.checked_add(1).ok_or_else(||refused(0,0,PagedListRefusalKind::OwnershipLimit,"paged reservation epoch exhausted"))?;
  let mut work=1;let page_items=self.page_items();let page=self.capacity/page_items;
  if selected.target_height>selected.root_height{
   let mut root=Vec::new();work+=1;
   A::reserve(&mut root,1,Some(&mut *actual)).map_err(|_|refused(work,0,PagedListRefusalKind::AllocationFailed,"fixed list metadata allocation failed"))?;
   if root.capacity()<1{return Err(refused(work,0,PagedListRefusalKind::InvariantViolated,"fixed list metadata allocation provided no reserved slot"))}
   let bytes=root.capacity()*size_of::<Page<T>>();work+=FANOUT as u64+3;
   let mut children=std::array::from_fn(|_|Vec::new());children[0]=std::mem::take(&mut self.root);
   root.push(Page::Branch{height:selected.root_height+1,children});self.root=root;
   self.allocated=self.allocated.checked_add(bytes).expect("each original layout fits signed addressable ownership");self.reservation_epoch=next_epoch;work+=1;
   if self.allocated>isize::MAX as usize{return Err(refused(work,bytes,PagedListRefusalKind::OwnershipLimit,"fixed list actual allocation exceeds addressable ownership; owner retained"))}
   if bytes>grant.maximum_capacity_bytes{return Err(refused(work,bytes,PagedListRefusalKind::InvariantViolated,"fixed list metadata allocation exceeded admission; owner retained"))}
   return Ok(PagedListProgress{work,progressed:true,allocated_bytes:bytes,..Default::default()})
  }
  let mut link=&mut self.root;
  for height in (0..=selected.root_height).rev(){
   work+=1;
   if link.is_empty(){
    let prior=link.capacity();work+=1;
    A::reserve(link,1,Some(&mut *actual)).map_err(|_|refused(work,0,PagedListRefusalKind::AllocationFailed,"fixed list metadata allocation failed"))?;
    if link.capacity()<1{return Err(refused(work,0,PagedListRefusalKind::InvariantViolated,"fixed list metadata allocation provided no reserved slot"))}
    let bytes=if prior==0{link.capacity()*size_of::<Page<T>>()}else{0};
    work+=if height==0{1}else{FANOUT as u64+1};
    link.push(if height==0{Page::Leaf{items:Vec::new(),slots:0}}else{Page::Branch{height,children:std::array::from_fn(|_|Vec::new())}});
    self.allocated=self.allocated.checked_add(bytes).expect("each original layout fits signed addressable ownership");self.reservation_epoch=next_epoch;work+=1;
    if self.allocated>isize::MAX as usize{return Err(refused(work,bytes,PagedListRefusalKind::OwnershipLimit,"fixed list actual allocation exceeds addressable ownership; owner retained"))}
    if bytes>grant.maximum_capacity_bytes{return Err(refused(work,bytes,PagedListRefusalKind::InvariantViolated,"fixed list metadata allocation exceeded admission; owner retained"))}
    return Ok(PagedListProgress{work,progressed:true,allocated_bytes:bytes,..Default::default()})
   }
   work+=1;
   match &mut link[0]{
    Page::Branch{children,..}=>{if height==0{return Err(refused(work,0,PagedListRefusalKind::InvariantViolated,"paged reservation branch exceeds its declared height"))}link=&mut children[Self::slot(page,height)]},
    Page::Leaf{items,slots}=>{
     let count=page_items.min(selected.limit-self.capacity);
     if *slots==0&&size_of::<T>()!=0&&items.capacity()!=0{return Err(refused(work,0,PagedListRefusalKind::InvariantViolated,"fixed list partial allocation requires funded settlement before reservation"))}
     let prior=items.capacity();work+=1;
     A::reserve(items,count,Some(&mut *actual)).map_err(|_|refused(work,0,PagedListRefusalKind::AllocationFailed,"fixed list payload allocation failed"))?;
     let bytes=if size_of::<T>()==0||prior!=0{0}else{items.capacity()*size_of::<T>()};
     self.allocated=self.allocated.checked_add(bytes).expect("each original layout fits signed addressable ownership");self.reservation_epoch=next_epoch;work+=1;
     if items.capacity()<count{return Err(refused(work,bytes,PagedListRefusalKind::InvariantViolated,"fixed list payload allocation provided fewer reserved slots; owner retained"))}
     *slots=count;self.capacity+=count;work+=2;
     if self.allocated>isize::MAX as usize{return Err(refused(work,bytes,PagedListRefusalKind::OwnershipLimit,"fixed list actual allocation exceeds addressable ownership; owner retained"))}
     if bytes>grant.maximum_capacity_bytes{return Err(refused(work,bytes,PagedListRefusalKind::InvariantViolated,"fixed list payload allocation exceeded admission; owner retained"))}
     return Ok(PagedListProgress{work,progressed:true,allocated_bytes:bytes,..Default::default()})
    }
   }
  }
  Err(refused(work,0,PagedListRefusalKind::InvariantViolated,"fixed list page authority is missing"))
 }
}
