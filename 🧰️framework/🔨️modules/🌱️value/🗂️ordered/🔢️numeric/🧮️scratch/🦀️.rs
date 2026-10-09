//! 🔢️ Paged mutable AVL indexes for fixed-width scratch keys and retained physical ownership.
use crate::list::{PagedList,PagedListError};
use crate::retirement::{RetireOwned,RetirementCursor};
use std::cmp::Ordering;

struct Node<K,V>{key:K,value:Option<V>,generation:u64,count_generation:u64,count:usize,height:u8,left:Option<usize>,right:Option<usize>,parent:Option<usize>}
impl<K:RetireOwned,V:RetireOwned> RetireOwned for Node<K,V>{
 fn retirement(self)->Box<dyn RetirementCursor>{crate::retirement::sequence(vec![crate::retirement::deferred(self.key),crate::retirement::deferred(self.value),crate::retirement::deferred(self.generation),crate::retirement::deferred(self.count_generation),crate::retirement::deferred(self.count),crate::retirement::deferred(self.height),crate::retirement::deferred(self.left),crate::retirement::deferred(self.right),crate::retirement::deferred(self.parent)])}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::sequence_birth_bytes(&[crate::retirement::deferred_birth_bytes_for(&self.key),crate::retirement::deferred_birth_bytes_for(&self.value),crate::retirement::deferred_birth_bytes::<u64>(),crate::retirement::deferred_birth_bytes::<u64>(),crate::retirement::deferred_birth_bytes::<usize>(),crate::retirement::deferred_birth_bytes::<u8>(),crate::retirement::deferred_birth_bytes::<Option<usize>>(),crate::retirement::deferred_birth_bytes::<Option<usize>>(),crate::retirement::deferred_birth_bytes::<Option<usize>>()])}
 fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
}

/// 🌳️ Retains tombstones and backing pages while live-key operations remain logarithmic.
pub struct NumericIndex<K,V>{nodes:PagedList<Node<K,V>,1048576>,root:Option<usize>,generation:u64}
impl<K,V> Default for NumericIndex<K,V>{fn default()->Self{Self{nodes:PagedList::empty(),root:None,generation:1}}}
impl<K:Copy+Ord,V> NumericIndex<K,V>{
 pub fn new()->Self{Self::default()}
 fn node(&self,index:usize)->&Node<K,V>{self.nodes.get(index).unwrap()}
 fn node_mut(&mut self,index:usize)->&mut Node<K,V>{self.nodes.get_mut(index).unwrap()}
 fn height(&self,index:Option<usize>)->u8{index.map_or(0,|i|self.node(i).height)}
 fn count(&self,index:Option<usize>)->usize{index.map_or(0,|i|{let n=self.node(i);if n.count_generation==self.generation{n.count}else{0}})}
 fn find(&self,key:&K)->Option<usize>{let mut current=self.root;while let Some(index)=current{let node=self.node(index);match key.cmp(&node.key){Ordering::Less=>current=node.left,Ordering::Greater=>current=node.right,Ordering::Equal=>return Some(index)}}None}
 fn update(&mut self,index:usize){let n=self.node(index);let (left,right,alive)=(n.left,n.right,n.generation==self.generation&&n.value.is_some());let count=self.count(left)+self.count(right)+usize::from(alive);let height=1+self.height(left).max(self.height(right));let generation=self.generation;let n=self.node_mut(index);n.count=count;n.count_generation=generation;n.height=height;}
 fn replace_child(&mut self,parent:Option<usize>,before:usize,after:usize){if let Some(parent)=parent{let n=self.node_mut(parent);if n.left==Some(before){n.left=Some(after)}else{n.right=Some(after)}}else{self.root=Some(after)}self.node_mut(after).parent=parent;}
 fn rotate_left(&mut self,index:usize)->usize{let (parent,right)={let n=self.node(index);(n.parent,n.right.unwrap())};let middle=self.node(right).left;self.replace_child(parent,index,right);self.node_mut(index).right=middle;if let Some(middle)=middle{self.node_mut(middle).parent=Some(index)}self.node_mut(right).left=Some(index);self.node_mut(index).parent=Some(right);self.update(index);self.update(right);right}
 fn rotate_right(&mut self,index:usize)->usize{let (parent,left)={let n=self.node(index);(n.parent,n.left.unwrap())};let middle=self.node(left).right;self.replace_child(parent,index,left);self.node_mut(index).left=middle;if let Some(middle)=middle{self.node_mut(middle).parent=Some(index)}self.node_mut(left).right=Some(index);self.node_mut(index).parent=Some(left);self.update(index);self.update(left);left}
 fn repair(&mut self,mut current:Option<usize>){while let Some(index)=current{self.update(index);let (left,right)={let n=self.node(index);(n.left,n.right)};let balance=self.height(left) as i16-self.height(right) as i16;let top=if balance>1{let child=left.unwrap();if self.height(self.node(child).left)<self.height(self.node(child).right){self.rotate_left(child);}self.rotate_right(index)}else if balance< -1{let child=right.unwrap();if self.height(self.node(child).right)<self.height(self.node(child).left){self.rotate_right(child);}self.rotate_left(index)}else{index};current=self.node(top).parent;}}
 pub fn len(&self)->usize{self.count(self.root)}
 pub fn is_empty(&self)->bool{self.len()==0}
 pub fn stored_keys(&self)->usize{self.nodes.len()}
 pub fn get(&self,key:&K)->Option<&V>{let n=self.node(self.find(key)?);if n.generation==self.generation{n.value.as_ref()}else{None}}
 pub fn get_mut(&mut self,key:&K)->Option<&mut V>{let index=self.find(key)?;let generation=self.generation;let n=self.node_mut(index);if n.generation==generation{n.value.as_mut()}else{None}}
 pub fn contains_key(&self,key:&K)->bool{self.get(key).is_some()}
 pub fn next_allocation_bytes(&self)->Result<usize,PagedListError>{self.nodes.next_allocation_bytes()}
 pub fn reserve_one(&mut self,bytes:usize)->Result<crate::list::PagedListProgress,crate::list::PagedListAllocationError>{self.nodes.reserve_one(bytes)}
 pub fn insert_reserved(&mut self,key:K,value:V)->Result<Option<V>,(K,V)>{
  let mut parent=None;let mut current=self.root;while let Some(index)=current{match key.cmp(&self.node(index).key){Ordering::Equal=>{let generation=self.generation;let n=self.node_mut(index);let previous=n.value.replace(value);let previous=if n.generation==generation{previous}else{None};n.generation=generation;self.repair(Some(index));return Ok(previous)},order=>{parent=Some(index);current=if order==Ordering::Less{self.node(index).left}else{self.node(index).right}}}}
  if !self.nodes.has_reserved_slot(){return Err((key,value))}let index=self.nodes.len();let node=Node{key,value:Some(value),generation:self.generation,count_generation:self.generation,count:1,height:1,left:None,right:None,parent};self.nodes.push_reserved(node).unwrap_or_else(|_|unreachable!());if let Some(parent)=parent{if key<self.node(parent).key{self.node_mut(parent).left=Some(index)}else{self.node_mut(parent).right=Some(index)}}else{self.root=Some(index)}self.repair(parent);Ok(None)
 }
 pub fn insert(&mut self,key:K,value:V)->Option<V>{if self.find(&key).is_none(){while !self.nodes.has_reserved_slot(){let required=self.nodes.next_allocation_bytes().expect("numeric index page demand");self.nodes.reserve_one(required).expect("numeric index page admission");}}self.insert_reserved(key,value).unwrap_or_else(|_|unreachable!())}
 pub fn get_or_insert_default(&mut self,key:K)->&mut V where V:Default{if !self.contains_key(&key){assert!(self.insert(key,V::default()).is_none(),"non-copy numeric value reset refused");}self.get_mut(&key).unwrap()}
 pub fn remove(&mut self,key:&K)->Option<V>{let index=self.find(key)?;if self.node(index).generation!=self.generation{return None}let value=self.node_mut(index).value.take();self.repair(Some(index));value}
 pub fn pop_first(&mut self)->Option<(K,V)>{let mut current=self.root?;if self.count(Some(current))==0{return None}loop{let n=self.node(current);if self.count(n.left)>0{current=n.left.unwrap();continue}if n.generation==self.generation&&n.value.is_some(){let key=n.key;return self.remove(&key).map(|value|(key,value))}current=n.right.unwrap();}}
}
impl<K:Copy+Ord,V:Copy> NumericIndex<K,V>{pub fn reset(&mut self){self.generation=self.generation.checked_add(1).expect("numeric scratch generation exhausted");}}
impl<K:RetireOwned,V:RetireOwned> RetireOwned for NumericIndex<K,V>{
 fn retirement(self)->Box<dyn RetirementCursor>{crate::retirement::sequence(vec![crate::retirement::deferred(self.nodes),crate::retirement::deferred(self.root),crate::retirement::deferred(self.generation)])}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::sequence_birth_bytes(&[crate::retirement::deferred_birth_bytes_for(&self.nodes),crate::retirement::deferred_birth_bytes_for(&self.root),crate::retirement::deferred_birth_bytes_for(&self.generation)])}
 fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
