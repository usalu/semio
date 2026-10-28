//! 🧷️ Original composition pins sort in place with bounded URI comparisons and funded record swaps.
use super::*;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceUriView;
use std::cmp::Ordering;
/// 🌳️ A fixed heapsort cursor preserves the actual original pin vector and all owned field allocations.
#[derive(Default)]
pub struct OriginalCompositionPinOrder{source:Option<(usize,usize,usize)>,phase:u8,build:usize,end:usize,root:usize,best:usize,left:usize,right:usize,offset:usize,index:usize,goal:u8,building:bool,closing:bool}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"composition pin ordering changed its original owner")}
impl OriginalCompositionPinOrder{
 /// 📏️ A comparison reads at most two 64-byte URI fragments; swaps move three complete original frames.
 pub fn demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+128+if self.phase==4||self.phase==5&&self.end>1{3*size_of::<CompositionPin>()}else{0},depth:1,..Default::default()}}
 fn compare(&mut self,left:usize,right:usize,goal:u8){self.left=left;self.right=right;self.offset=0;self.goal=goal;self.phase=3;}
 fn sift_finished(&mut self){self.phase=if self.building{1}else{5};}
 /// 🪡️ No comparison, swap or source binding runs before its independently admitted fixed event.
 pub fn advance(&mut self,pins:&mut Vec<CompositionPin>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=(pins as*const _ as usize,pins.as_ptr()as usize,pins.len());if self.closing||self.source.is_some_and(|original|original!=source){return Err(invalid())}if self.ready(){return Ok(Default::default())}let d=self.demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}self.source=Some(source);
  match self.phase{
   0=>{self.end=pins.len();self.build=pins.len()/2;self.building=true;self.phase=if pins.len()<2{7}else{1};},
   1=>{if self.build>0{self.build-=1;self.root=self.build;self.phase=2;}else{self.building=false;self.phase=5;}},
   2=>{let child=self.root.checked_mul(2).and_then(|n|n.checked_add(1)).ok_or_else(invalid)?;if child>=self.end{self.sift_finished();}else if child+1<self.end{self.compare(child,child+1,0);}else{self.best=child;self.compare(self.root,self.best,1);}},
   3=>{let left=ArtifactReferenceUriView::new(&pins[self.left].child_ref);let right=ArtifactReferenceUriView::new(&pins[self.right].child_ref);let(mut a,mut b)=([0;64],[0;64]);let na=left.read_bytes(self.offset,&mut a);let nb=right.read_bytes(self.offset,&mut b);let order=a[..na].cmp(&b[..nb]);if order==Ordering::Equal&&na!=0{self.offset+=na;}else{match self.goal{0=>{self.best=if order==Ordering::Less{self.right}else{self.left};self.compare(self.root,self.best,1);},1=>{if order==Ordering::Less{self.phase=4;}else{self.sift_finished();}},2=>{if order!=Ordering::Less{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"composition pins require unique original child URIs").with_retained_progress(progress(d)))}self.index+=1;self.phase=6;},_=>return Err(invalid())}}},
   4=>{pins.swap(self.root,self.best);self.root=self.best;self.phase=2;},
   5=>{if self.end<=1{self.index=0;self.phase=6;}else{pins.swap(0,self.end-1);self.end-=1;self.root=0;self.phase=2;}},
   6=>{if self.index+1<pins.len(){self.compare(self.index,self.index+1,2);}else{self.phase=7;}},
   _=>return Err(invalid()),
  }Ok(progress(d))
 }
 /// 🏁️ A canonical sorted and duplicate-free vector remains in the caller's original storage.
 pub fn ready(&self)->bool{self.phase==7&&!self.closing}
 /// 📐️ The actual fixed source-coordinate return needs no allocation or payload release.
 pub fn close_demands(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<(usize,usize,usize)>>()+size_of::<bool>(),depth:1,..Default::default()}}
 /// ♻️ Cancelling leaves every partially reordered original pin owned by its enclosing source.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{if self.terminal_is_empty(){return Default::default()}let d=self.close_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Default::default()}self.source=None;self.closing=true;progress(d)}
 /// 🫙️ No borrowed vector coordinates remain after the funded return.
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
}
#[cfg(test)]
#[path="pin-order-tests.rs"]
mod tests;
