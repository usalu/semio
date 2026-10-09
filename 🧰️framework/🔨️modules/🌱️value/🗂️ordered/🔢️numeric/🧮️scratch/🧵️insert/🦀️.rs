//! 🧵️ One numeric comparison, page admission, link or AVL ancestor repair per native grant.
use super::{NumericIndex,Node,Ordering};
use crate::{NativeDecodeControl,ValueError,ValueRefusalKind};
use crate::retirement::{RetireOwned,RetirementCursor,sequence,deferred,sequence_birth_bytes,deferred_birth_bytes_for};

pub struct NumericInsertCursor<K,V>{key:K,value:Option<V>,previous:Option<V>,current:Option<usize>,parent:Option<usize>,phase:u8}
impl<K:Copy+Ord,V:Copy> NumericInsertCursor<K,V>{
 pub fn new(key:K,value:V)->Self{Self{key,value:Some(value),previous:None,current:None,parent:None,phase:0}}
 pub fn is_complete(&self)->bool{self.phase==5}
 pub fn take_previous(&mut self)->Option<V>{assert!(self.is_complete());self.previous.take()}
 pub fn next_capacity_byte_demand(&self,index:&NumericIndex<K,V>)->Result<usize,ValueError>{if self.phase==2&&!index.nodes.has_reserved_slot(){index.nodes.next_allocation_bytes().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"numeric page demand refused"))}else{Ok(0)}}
 pub fn next_depth_demand(&self,index:&NumericIndex<K,V>)->Result<usize,ValueError>{if self.phase==2{index.nodes.next_reserve_depth_demand().map(|depth|depth.max(1)).map_err(|_|ValueError::literal(ValueRefusalKind::DepthLimit,"numeric page depth demand refused"))}else{Ok(1)}}
 pub fn step(&mut self,index:&mut NumericIndex<K,V>,maximum_units:usize,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{
  for _ in 0..maximum_units{
   control.checkpoint()?;if self.is_complete(){return Ok(true)}
   match self.phase{
    0=>{self.current=index.root;self.phase=1;},
    1=>if let Some(current)=self.current{match self.key.cmp(&index.node(current).key){
     Ordering::Equal=>{let generation=index.generation;let node=index.node_mut(current);let previous=node.value.replace(self.value.take().unwrap());self.previous=if node.generation==generation{previous}else{None};node.generation=generation;self.phase=4;},
     order=>{self.parent=Some(current);self.current=if order==Ordering::Less{index.node(current).left}else{index.node(current).right};},
    }}else{self.phase=2;},
    2=>{if index.nodes.has_reserved_slot(){self.phase=3;}else{let bytes=index.nodes.next_allocation_bytes().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"numeric page demand refused"))?;control.charge(bytes)?;index.nodes.reserve_one(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"numeric page admission refused"))?;}},
    3=>{let placed=index.nodes.len();let node=Node{key:self.key,value:self.value.take(),generation:index.generation,count_generation:index.generation,count:1,height:1,left:None,right:None,parent:self.parent};index.nodes.push_reserved(node).unwrap_or_else(|_|unreachable!());if let Some(parent)=self.parent{if self.key<index.node(parent).key{index.node_mut(parent).left=Some(placed)}else{index.node_mut(parent).right=Some(placed)}}else{index.root=Some(placed)}self.current=self.parent;self.phase=4;},
    4=>if let Some(current)=self.current{index.update(current);let (left,right)={let node=index.node(current);(node.left,node.right)};let balance=index.height(left)as i16-index.height(right)as i16;let top=if balance>1{let child=left.unwrap();if index.height(index.node(child).left)<index.height(index.node(child).right){index.rotate_left(child);}index.rotate_right(current)}else if balance< -1{let child=right.unwrap();if index.height(index.node(child).right)<index.height(index.node(child).left){index.rotate_right(child);}index.rotate_left(current)}else{current};self.current=index.node(top).parent;}else{self.phase=5;},
    _=>unreachable!(),
   }
   control.step()?;
  }
  Ok(self.is_complete())
 }
}
impl<K:RetireOwned,V:RetireOwned> RetireOwned for NumericInsertCursor<K,V>{
 fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.key),deferred(self.value),deferred(self.previous),deferred(self.current),deferred(self.parent),deferred(self.phase)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.key),deferred_birth_bytes_for(&self.value),deferred_birth_bytes_for(&self.previous),deferred_birth_bytes_for(&self.current),deferred_birth_bytes_for(&self.parent),deferred_birth_bytes_for(&self.phase)])}
 fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
}
