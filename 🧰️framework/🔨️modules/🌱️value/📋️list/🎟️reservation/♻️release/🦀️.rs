//! ♻️ Paid selection identifies one retained empty backing before its disposal is admitted.
use super::*;
use semio_framework_value_control_contract::{RetainedWorkOutcome,RetirementDemand};
const HEIGHT:usize=(usize::BITS as usize+3)/4;
#[derive(Clone,Copy,Debug)]
pub struct PagedListReleaseSelection{
 demand:RetirementDemand,kind:PagedListReservationKind,path:[u8;HEIGHT],length:usize,metadata_branch:bool,
 binding:(usize,usize,usize,usize,usize,usize,usize,usize,usize),
}
fn invalid(reason:&'static str)->PagedListError{PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason}}
impl PagedListReleaseSelection{
 /// 📏️ Quotes the selected backing and its original page path.
 pub fn demand(&self)->RetirementDemand{self.demand}
 /// 🗂️ Identifies the retained payload or metadata backing.
 pub fn kind(&self)->PagedListReservationKind{self.kind}
 /// ⛽️ Bounds cached-path revalidation, sixteen child headers and the release.
 pub const fn release_work_demand(&self)->u64{2*self.length as u64+if matches!(self.kind,PagedListReservationKind::Payload){7}else{5+if self.metadata_branch{FANOUT as u64}else{0}}}
}
impl<T,const N:usize>PagedList<T,N>{
 /// 🔎️ The platform-derived bound is available before any page storage is inspected.
 pub const fn release_selection_work_demand()->u64{(FANOUT as u64+2)*HEIGHT as u64+3}
 /// 🧭️ Counts every reverse child probe while retaining the same list and all its backings.
 pub fn select_release(&self,maximum_work_units:u64)->RetainedWorkOutcome<Option<PagedListReleaseSelection>,PagedListError>{self.select_release_inner(maximum_work_units,Self::release_selection_work_demand())}
 /// 🎟️ Uses the paid root header's tighter bound before inspecting reverse child links.
 pub fn select_release_from_header(&self,header:&PagedListSelectionHeader,maximum_work_units:u64)->RetainedWorkOutcome<Option<PagedListReleaseSelection>,PagedListError>{
  let bound=header.release_selection_work_demand();if maximum_work_units<bound{return RetainedWorkOutcome{result:Ok(None),work:0}}
  if header.binding!=self.reservation_binding(){return RetainedWorkOutcome{result:Err(invalid("paged selection header changed")),work:1}}
  self.select_release_inner(maximum_work_units,bound)
 }
 fn select_release_inner(&self,maximum_work_units:u64,bound:u64)->RetainedWorkOutcome<Option<PagedListReleaseSelection>,PagedListError>{
  if maximum_work_units<bound{return RetainedWorkOutcome{result:Ok(None),work:0}}
  let mut work=1;let result=(||{
   if self.root.is_empty()&&self.root.capacity()==0{return Ok(None)}
   let mut link=&self.root;let mut path=[0;HEIGHT];let mut length=0;
   let(kind,bytes,depth,metadata_branch)=loop{
    work+=1;match link.first(){
     None=>break(PagedListReservationKind::Metadata,link.capacity()*size_of::<Page<T>>(),length+2,false),
     Some(Page::Branch{children,..})=>{
      if length==HEIGHT{return Err(invalid("paged release branch exceeds addressable page indices"))}
      let mut selected=None;for index in (0..FANOUT).rev(){work+=1;if !children[index].is_empty()||children[index].capacity()!=0{selected=Some(index);break}}
      if let Some(index)=selected{work+=1;path[length]=index as u8;length+=1;link=&children[index];}else{break(PagedListReservationKind::Metadata,link.capacity()*size_of::<Page<T>>(),length+2,true)}
     }
     Some(Page::Leaf{items,slots})=>{
      if !items.is_empty(){return Err(invalid("fixed list payload must retire before its page"))}
      if *slots!=0||size_of::<T>()!=0&&items.capacity()!=0{break(PagedListReservationKind::Payload,if size_of::<T>()==0{0}else{items.capacity()*size_of::<T>()},length+3,false)}
      break(PagedListReservationKind::Metadata,link.capacity()*size_of::<Page<T>>(),length+2,false)
     }
    }
   };
   if bytes>self.allocated{return Err(invalid("paged release backing exceeds its owned allocation counter"))}
   work+=1;Ok(Some(PagedListReleaseSelection{demand:RetirementDemand{release_bytes:bytes,depth,..Default::default()},kind,path,length,metadata_branch,binding:self.reservation_binding()}))
  })();RetainedWorkOutcome{result,work}
 }
 /// 🪙️ Follows the selected bounded path only after physical and independent work grants fit.
 pub fn release_selected(&mut self,selected:&PagedListReleaseSelection,grant:semio_framework_value_control_contract::RetainedCloneGrant,maximum_work_units:u64)->RetainedWorkOutcome<PagedListProgress,PagedListError>{
  let mut work=0;let result=(||{
   if maximum_work_units<selected.release_work_demand()||!semio_framework_value_control_contract::retained_clone_grant_admits(grant,selected.demand){return Ok(Default::default())}
   work=1;if selected.binding!=self.reservation_binding(){return Err(invalid("paged release selected frontier changed"))}
   let mut link=&mut self.root;
   for &index in &selected.path[..selected.length]{work+=2;let Some(Page::Branch{children,..})=link.first_mut()else{return Err(invalid("paged release selected branch changed"))};link=&mut children[index as usize];}
   work+=1;let bytes=selected.demand.release_bytes;
   match selected.kind{
    PagedListReservationKind::Payload=>{
     let Some(Page::Leaf{items,slots})=link.first_mut()else{return Err(invalid("paged release selected payload changed"))};
     if !items.is_empty()||*slots>self.capacity||(if size_of::<T>()==0{0}else{items.capacity()*size_of::<T>()})!=bytes{return Err(invalid("paged release selected payload extent changed"))}
     if bytes>self.allocated{return Err(invalid("paged release backing exceeds its owned allocation counter"))}
     *items=Vec::new();self.capacity-=*slots;*slots=0;work+=5;
    }
    PagedListReservationKind::Metadata=>{
     if link.capacity()*size_of::<Page<T>>()!=bytes||bytes>self.allocated{return Err(invalid("paged release selected metadata extent changed"))}
     match link.first(){
      Some(Page::Branch{children,..})=>{for child in children{work+=1;if !child.is_empty()||child.capacity()!=0{return Err(invalid("paged release metadata retains a child backing"))}}},
      Some(Page::Leaf{items,slots})=>if !items.is_empty()||*slots!=0||size_of::<T>()!=0&&items.capacity()!=0{return Err(invalid("paged release metadata retains its payload"))},
      None=>{},
     }
     *link=Vec::new();work+=3;
    }
   }
   self.allocated-=bytes;self.reservation_epoch=self.reservation_epoch.saturating_add(1);
   Ok(PagedListProgress{work,progressed:true,released_allocation_bytes:bytes,..Default::default()})
  })();RetainedWorkOutcome{result,work:u128::from(work)}
 }
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
