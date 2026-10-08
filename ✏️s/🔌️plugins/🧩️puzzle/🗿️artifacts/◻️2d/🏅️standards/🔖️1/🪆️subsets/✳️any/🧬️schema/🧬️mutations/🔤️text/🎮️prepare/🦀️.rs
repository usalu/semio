//! 🔤️ Borrowed optional UTF8 intents compare original native owners before plan publication.
use super::{ChangeNodeIcon,EditTargetRegionLabel};
use crate::{Puzzle2dSnapshot,standards::v1::subsets::any::schema::snapshot::lookup::{Puzzle2dLookupCursor,Puzzle2dLookupScope,Puzzle2dLookupStep}};
use semio_framework_value::{paged::PagedUtf8,SnapshotRetirementStep,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneBinding,RetainedCloneRef,paged::PagedUtf8BoundedOrdCursor,ordered_map::{BoundedOrdCursor,BoundedOrdGrant,BoundedOrdProgress,BoundedOrdStep}}};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Puzzle2dTextDisposition {Changed,NoOp,TargetMissing}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Puzzle2dTextPlan {pub disposition:Puzzle2dTextDisposition,pub index:Option<usize>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Puzzle2dTextPreparationStep {Pending(BoundedOrdProgress),Complete{plan:Puzzle2dTextPlan,progress:BoundedOrdProgress}}

pub trait Puzzle2dTextIntent:Sync {
 const SCOPE:Puzzle2dLookupScope;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>;
 fn next(&self)->&Option<PagedUtf8<{usize::MAX}>>;
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->&Option<PagedUtf8<{usize::MAX}>>;
}
impl Puzzle2dTextIntent for ChangeNodeIcon {
 const SCOPE:Puzzle2dLookupScope=Puzzle2dLookupScope::Node;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>{&self.id}
 fn next(&self)->&Option<PagedUtf8<{usize::MAX}>>{&self.new_icon_kind}
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->&Option<PagedUtf8<{usize::MAX}>>{&snapshot.nodes.get(index).expect("immutable icon target").icon_kind}
}
impl Puzzle2dTextIntent for EditTargetRegionLabel {
 const SCOPE:Puzzle2dLookupScope=Puzzle2dLookupScope::Region;
 fn identifier(&self)->&PagedUtf8<{usize::MAX}>{&self.id}
 fn next(&self)->&Option<PagedUtf8<{usize::MAX}>>{&self.new_label}
 fn previous(snapshot:&Puzzle2dSnapshot,index:usize)->&Option<PagedUtf8<{usize::MAX}>>{&snapshot.target_regions.get(index).expect("immutable label target").label}
}

pub struct Puzzle2dTextPreparationCursor<T:Puzzle2dTextIntent> {
 source:Option<RetainedCloneBinding>,mutation:Option<RetainedCloneBinding>,lookup:Puzzle2dLookupCursor,comparison:PagedUtf8BoundedOrdCursor<{usize::MAX}>,index:Option<usize>,disposition:Puzzle2dTextDisposition,phase:u8,output:Option<Puzzle2dTextPlan>,closing:bool,spent:bool,kind:std::marker::PhantomData<T>,
}
impl<T:Puzzle2dTextIntent> Default for Puzzle2dTextPreparationCursor<T>{fn default()->Self{Self{source:None,mutation:None,lookup:Puzzle2dLookupCursor::new(T::SCOPE),comparison:Default::default(),index:None,disposition:Puzzle2dTextDisposition::TargetMissing,phase:0,output:None,closing:false,spent:false,kind:std::marker::PhantomData}}}
impl<T:Puzzle2dTextIntent> Puzzle2dTextPreparationCursor<T>{
 pub fn advance(&mut self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,mutation:RetainedCloneRef<'_,T>,grant:BoundedOrdGrant)->Result<Puzzle2dTextPreparationStep,ValueError>{
  if self.closing||self.spent{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"text preparation is closing or spent"))}
  if grant.maximum_items==0{return Ok(Puzzle2dTextPreparationStep::Pending(Default::default()))}
  source.bind(&mut self.source)?;mutation.bind(&mut self.mutation)?;
  if let Some(plan)=self.output{return Ok(Puzzle2dTextPreparationStep::Complete{plan,progress:Default::default()})}
  if self.phase==0{return match self.lookup.advance(source,mutation.project(1,T::identifier),grant)?{Puzzle2dLookupStep::Pending(progress)=>Ok(Puzzle2dTextPreparationStep::Pending(progress)),Puzzle2dLookupStep::Complete{location,progress}=>{self.index=location.map(|value|value.outer);self.lookup.take();self.phase=1;Ok(Puzzle2dTextPreparationStep::Pending(progress))}}}
  if self.phase==1{
   let bytes=2*std::mem::size_of::<bool>();if grant.maximum_bytes<bytes{return Ok(Puzzle2dTextPreparationStep::Pending(Default::default()))}
   self.phase=3;
   if let Some(index)=self.index{match(T::previous(source.get(),index).is_some(),mutation.get().next().is_some()){(false,false)=>self.disposition=Puzzle2dTextDisposition::NoOp,(true,true)=>self.phase=2,_=>self.disposition=Puzzle2dTextDisposition::Changed}}
   if self.phase==3{self.comparison.begin_close();}
   return Ok(Puzzle2dTextPreparationStep::Pending(BoundedOrdProgress{compared_items:1,compared_bytes:bytes}))
  }
  if self.phase==2{
   let index=self.index.expect("immutable text ordinal");
   let left=source.project(2,|snapshot|T::previous(snapshot,index).as_ref().expect("immutable previous text"));
   let right=mutation.project(2,|payload|payload.next().as_ref().expect("immutable next text"));
   return match self.comparison.compare(left,right,grant)?{BoundedOrdStep::Progress(progress)=>Ok(Puzzle2dTextPreparationStep::Pending(progress)),BoundedOrdStep::Complete{ordering,progress}=>{self.disposition=if ordering==std::cmp::Ordering::Equal{Puzzle2dTextDisposition::NoOp}else{Puzzle2dTextDisposition::Changed};self.phase=3;self.comparison.begin_close();Ok(Puzzle2dTextPreparationStep::Pending(progress))}}
  }
  if self.phase==3{let step=self.comparison.close_step(grant.maximum_items,grant.maximum_bytes)?;if step!=SnapshotRetirementStep::Complete{return Ok(Puzzle2dTextPreparationStep::Pending(BoundedOrdProgress{compared_items:usize::from(matches!(step,SnapshotRetirementStep::Pending{released_items:1,..})),compared_bytes:0}))}self.phase=4;self.lookup.begin_close();return Ok(Puzzle2dTextPreparationStep::Pending(BoundedOrdProgress{compared_items:1,compared_bytes:0}))}
  if self.phase==4{let step=self.lookup.close_step(grant.maximum_items,grant.maximum_bytes)?;if step!=SnapshotRetirementStep::Complete{return Ok(Puzzle2dTextPreparationStep::Pending(BoundedOrdProgress{compared_items:usize::from(matches!(step,SnapshotRetirementStep::Pending{released_items:1,..})),compared_bytes:0}))}self.phase=5;return Ok(Puzzle2dTextPreparationStep::Pending(BoundedOrdProgress{compared_items:1,compared_bytes:0}))}
  let plan=Puzzle2dTextPlan{disposition:self.disposition,index:self.index};self.output=Some(plan);Ok(Puzzle2dTextPreparationStep::Complete{plan,progress:BoundedOrdProgress{compared_items:1,compared_bytes:0}})
 }
 pub fn take(&mut self)->Option<Puzzle2dTextPlan>{if self.closing{return None}let output=self.output.take();if output.is_some(){self.spent=true}output}
 pub fn begin_close(&mut self){self.closing=true;self.lookup.begin_close();self.comparison.begin_close();}
 pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
  if !self.closing{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"text preparation close was not begun"))}
  if self.output.is_some(){if maximum_items==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}self.output=None;return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})}
  let step=self.comparison.close_step(maximum_items,maximum_bytes)?;if step!=SnapshotRetirementStep::Complete{return Ok(step)}
  let step=self.lookup.close_step(maximum_items,maximum_bytes)?;if step!=SnapshotRetirementStep::Complete{return Ok(step)}
  let step=RetainedCloneBinding::close_one(&mut self.source,maximum_items)?;if step!=SnapshotRetirementStep::Complete{return Ok(step)}
  RetainedCloneBinding::close_one(&mut self.mutation,maximum_items)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.comparison.terminal_is_empty()&&self.lookup.terminal_is_empty()&&self.source.is_none()&&self.mutation.is_none()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="📸️candidate/🦀️.rs"]
pub mod candidate;
