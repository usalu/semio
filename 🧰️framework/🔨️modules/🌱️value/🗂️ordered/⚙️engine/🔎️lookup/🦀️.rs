//! 🪞️ Native lookup retains the genuine immutable map and String borrows through paid closure.
use crate::{Root,Node,native_len as len};
use std::{cmp::Ordering,mem::size_of};
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedWorkOutcome,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind,retained_clone_grant_admits};

fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native lookup requires its actual retained immutable query")}
fn permits(d:RetirementDemand,e:&RetainedAllocationEffect<'_>,work:u64)->Result<bool,ValueError>{Ok(retained_clone_grant_admits(e.grant(),d)&&e.remaining_work_units()?>=work)}
fn physical(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:usize::from(copy_bytes!=0),..Default::default()}}

/// 🪞️ Holds real borrows instead of reconstructing a query owner from an address.
/// ```compile_fail
/// use semio_framework_value_native_ordered_engine::BorrowedLookupCursor;
/// use semio_framework_value_control_contract::RetainedAllocationEffect;
/// fn owned_only(cursor:&mut BorrowedLookupCursor<'_,'_,u8>,effect:&mut RetainedAllocationEffect<'_>){cursor.advance_original(effect);}
/// ```
#[repr(C)]
pub struct BorrowedLookupCursor<'map,'query,V>{
 base:Option<&'map Root<V>>,query:Option<&'query String>,current:Option<&'map Node<V>>,
 rank:usize,offset:usize,compared:usize,last_compared:usize,left_byte:Option<u8>,ordering:Option<Ordering>,complete:bool,closed:bool,
}
impl<'map,'query,V> BorrowedLookupCursor<'map,'query,V>{
 /// 📏️ Quotes the concrete original caller-resident cursor slot.
 pub fn constructor_demand()->RetirementDemand{physical(size_of::<Option<Self>>())}
 /// 📥️ Captures the actual immutable issued tree and query borrows into the resident output.
 pub fn admit_into(root:&'map Root<V>,query:&'query String,output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{
  let before=effect.performed_boundary();let result=(||{if output.is_some(){return Err(invalid())}let d=Self::constructor_demand();if !permits(d,effect,1)?{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(invalid())}*output=Some(Self{base:Some(root),query:Some(query),current:root.as_ref().map(|node|&**node),rank:0,offset:0,compared:0,last_compared:0,left_byte:None,ordering:None,complete:root.is_none(),closed:false});Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same native lookup construction receipt")}
 }

 fn validate_query(&self,query:&String)->Result<(),ValueError>{if self.closed||!self.query.is_some_and(|original|std::ptr::eq(original,query)){return Err(invalid())}Ok(())}
 fn reads(&self,query:&String,body:usize)->Result<usize,ValueError>{self.validate_query(query)?;if self.complete||self.ordering.is_some(){return Ok(0)}let Some(node)=self.current else{return Ok(0)};let remaining=query.len().min(node.entry.key.len()).checked_sub(self.offset).ok_or_else(invalid)?;let remaining=remaining.checked_mul(2).and_then(|n|n.checked_sub(usize::from(self.left_byte.is_some()))).ok_or_else(invalid)?;Ok(remaining.min(body).min(64))}
 /// 📏️ Quotes the original semantic byte quantum separately from its physical metadata.
 pub fn next_demands(&self,query:&String,body:usize)->Result<RetirementDemand,ValueError>{
  self.validate_query(query)?;if self.complete{return Ok(Default::default())}if self.current.is_none(){return Ok(physical(size_of::<bool>()+size_of::<usize>()))}if let Some(ordering)=self.ordering{return Ok(physical(if ordering==Ordering::Equal{2*size_of::<usize>()+size_of::<bool>()}else{size_of::<Option<&Node<V>>>()+3*size_of::<usize>()+size_of::<Option<u8>>()+size_of::<Option<Ordering>>()}))}
  let reads=self.reads(query,body)?;if reads==0&&self.reads(query,64)?!=0{return Ok(Default::default())}Ok(physical(3*size_of::<usize>()+size_of::<Option<u8>>()+size_of::<Option<Ordering>>()+reads))
 }
 /// ⛽️ Prices every original source-byte read and its bounded native state publication.
 pub fn next_work_demand(&self,query:&String,body:usize)->Result<u64,ValueError>{let d=self.next_demands(query,body)?;if d==Default::default(){return Ok(0)}u64::try_from(self.reads(query,body)?).ok().and_then(|n|n.checked_add(1)).ok_or_else(invalid)}
 /// 🔎️ Reads only its lifetime-bound inputs and records actual Work before mutation.
 pub fn advance_into(&mut self,query:&String,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{
  let before=effect.performed_boundary();let result=(||{let d=self.next_demands(query,body)?;if self.complete{return Ok(RetainedCloneStep::Complete(Default::default()))}let work=self.next_work_demand(query,body)?;if work==0||!permits(d,effect,work)?{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(invalid())}
   let mut bytes=0;let copied_bytes=if let Some(node)=self.current{if let Some(ordering)=self.ordering{if ordering==Ordering::Equal{self.rank=self.rank.checked_add(len(&node.left)).ok_or_else(invalid)?;self.complete=true;self.last_compared=0;d.copy_bytes}else{let rank=if ordering==Ordering::Greater{self.rank.checked_add(len(&node.left)).and_then(|n|n.checked_add(1)).ok_or_else(invalid)?}else{self.rank};self.current=if ordering==Ordering::Less{node.left.as_ref().map(|node|&**node)}else{node.right.as_ref().map(|node|&**node)};self.rank=rank;self.offset=0;self.left_byte=None;self.ordering=None;self.last_compared=0;d.copy_bytes}}else{
    let limit=self.reads(query,body)?;let(mut offset,mut left,mut ordering)=(self.offset,self.left_byte,self.ordering);while bytes<limit&&ordering.is_none(){if !effect.performed(1)?{return Err(invalid())}bytes+=native_compare_bytes(query.as_bytes(),node.entry.key.as_bytes(),&mut offset,&mut left,&mut ordering,1)}if offset==query.len().min(node.entry.key.len())&&ordering.is_none(){ordering=Some(query.len().cmp(&node.entry.key.len()))}let compared=self.compared.checked_add(bytes).ok_or_else(invalid)?;self.offset=offset;self.left_byte=left;self.ordering=ordering;self.compared=compared;self.last_compared=bytes;d.copy_bytes-limit+bytes
   }}else{self.complete=true;self.last_compared=0;d.copy_bytes};
   let p=RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()};Ok(if self.complete{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same native lookup comparison receipt")}
 }
 /// 👁️ Borrows the matched original payload without cloning it.
 pub fn result(&self)->Option<&V>{if !self.complete||self.ordering!=Some(Ordering::Equal){return None}self.current.map(|node|&*node.entry.value)}
 pub fn result_rank(&self)->Option<usize>{(self.complete&&self.ordering==Some(Ordering::Equal)).then_some(self.rank)}
 pub fn is_complete(&self)->bool{self.complete}
 pub fn last_compared_bytes(&self)->usize{self.last_compared}
 pub fn compared_bytes(&self)->usize{self.compared}
 /// 📏️ Quotes the live cursor slot and its real native Drop flag.
 pub fn close_demand(slot:&Option<Self>)->RetirementDemand{physical(usize::from(slot.is_some())*(size_of::<Option<Self>>()+size_of::<bool>()))}
 /// 🧹️ Pays for the real native header clear and invokes its actual terminal destructor.
 pub fn close_from(slot:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep>{
  let before=effect.performed_boundary();let result=(||{if slot.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=Self::close_demand(slot);if !permits(d,effect,1)?{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(invalid())}slot.as_mut().expect("validated native cursor").closed=true;drop(slot.take());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same native lookup header receipt")}
 }
}
impl<V> Drop for BorrowedLookupCursor<'_,'_,V>{fn drop(&mut self){assert!(self.closed||std::thread::panicking(),"native borrowed lookup requires paid header closure")}}

pub fn native_compare_bytes(key: &[u8], other: &[u8], offset: &mut usize, left_byte: &mut Option<u8>, ordering: &mut Option<Ordering>, maximum_bytes: usize) -> usize {
    if ordering.is_none() && *offset==key.len().min(other.len()) {*ordering=Some(key.len().cmp(&other.len()));}
    let mut bytes = 0;
    while ordering.is_none() && bytes < maximum_bytes {
        if *offset == key.len().min(other.len()) { *ordering = Some(key.len().cmp(&other.len())); break; }
        if let Some(left) = left_byte.take() {
            let right = other[*offset]; bytes += 1; *offset += 1;
            if left != right { *ordering = Some(left.cmp(&right)); }
        } else { *left_byte = Some(key[*offset]); bytes += 1; }
    }
    bytes
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
