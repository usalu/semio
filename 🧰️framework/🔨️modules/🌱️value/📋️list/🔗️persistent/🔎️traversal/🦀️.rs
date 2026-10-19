//! 🔎️ Borrowed ranked page traversal enters each page once and skips complete subtrees by their original cardinalities.
use super::{Page,PersistentList,page_len};
const MAXIMUM_PAGE_DEPTH:usize=usize::BITS as usize/4;
struct Frame<'a,T:crate::retirement::RetireOwned+Send+Sync+'static>{page:&'a Page<T>,next:usize}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Copy for Frame<'_,T>{}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Clone for Frame<'_,T>{fn clone(&self)->Self{*self}}
/// 🧭️ A fixed-depth borrowed stack never duplicates row owners or allocates traversal storage.
pub struct PersistentListIter<'a,T:crate::retirement::RetireOwned+Send+Sync+'static>{
 stack:[Option<Frame<'a,T>>;MAXIMUM_PAGE_DEPTH],depth:usize,remaining:usize,
 #[cfg(test)] pub(super) visited_pages:usize,
}
impl<'a,T:crate::retirement::RetireOwned+Send+Sync+'static> PersistentListIter<'a,T>{
 fn push(&mut self,page:&'a Page<T>){
  assert!(self.depth<MAXIMUM_PAGE_DEPTH,"persistent page depth exceeded its admitted ranked address space");
  self.stack[self.depth]=Some(Frame{page,next:0});self.depth+=1;
  #[cfg(test)]{self.visited_pages+=1;}
 }
 fn exhaust(&mut self){self.stack=[None;MAXIMUM_PAGE_DEPTH];self.depth=0;self.remaining=0;}
 fn consume(&mut self,mut skip:usize)->Option<&'a T>{
  if skip>=self.remaining{self.exhaust();return None;}
  loop{
   let position=self.depth-1;let frame=self.stack[position].as_mut().expect("remaining immutable rows have an original page");
   match frame.page{
    Page::Leaf{rows,len}=>{
     let available=len-frame.next;
     if skip>=available{skip-=available;self.remaining-=available;self.stack[position]=None;self.depth-=1;continue;}
     frame.next+=skip;self.remaining-=skip+1;let row=rows[frame.next].as_ref().expect("original immutable leaf has a dense occupied prefix");frame.next+=1;return Some(row.get());
    },
    Page::Branch{children,..}=>{
     let child=children.get(frame.next).and_then(Option::as_ref);
     let Some(child)=child else{self.stack[position]=None;self.depth-=1;continue;};
     frame.next+=1;let length=page_len(child.get());
     if skip>=length{skip-=length;self.remaining-=length;continue;}
     self.push(child.get());
    },
   }
  }
 }
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> PersistentList<T>{
 /// 👁️ Scans original rows in ranked order with one fixed-depth stack and no payload cloning.
 pub fn iter(&self)->PersistentListIter<'_,T>{
  let mut cursor=PersistentListIter{stack:[None;MAXIMUM_PAGE_DEPTH],depth:0,remaining:self.len,#[cfg(test)]visited_pages:0};
  if let Some(root)=self.root.as_ref(){cursor.push(root.get());}cursor
 }
}
impl<'a,T:crate::retirement::RetireOwned+Send+Sync+'static> Iterator for PersistentListIter<'a,T>{
 type Item=&'a T;
 fn next(&mut self)->Option<Self::Item>{self.consume(0)}
 fn nth(&mut self,n:usize)->Option<Self::Item>{self.consume(n)}
 fn size_hint(&self)->(usize,Option<usize>){(self.remaining,Some(self.remaining))}
 fn count(self)->usize{self.remaining}
 fn last(mut self)->Option<Self::Item>{self.remaining.checked_sub(1).and_then(|n|self.consume(n))}
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> ExactSizeIterator for PersistentListIter<'_,T>{fn len(&self)->usize{self.remaining}}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> std::iter::FusedIterator for PersistentListIter<'_,T>{}
impl<'a,T:crate::retirement::RetireOwned+Send+Sync+'static> IntoIterator for &'a PersistentList<T>{
 type Item=&'a T;type IntoIter=PersistentListIter<'a,T>;
 fn into_iter(self)->Self::IntoIter{self.iter()}
}