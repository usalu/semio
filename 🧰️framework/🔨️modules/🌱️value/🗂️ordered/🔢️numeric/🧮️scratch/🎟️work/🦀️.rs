//! 🎟️ One original ledger admits numeric key search, real paged nodes, links and AVL repair.
use super::{NumericIndex,Node,Ordering};
use semio_framework_value_persistent::work::RetainedWorkLedger;
use crate::RetirementDemand;
use crate::retirement::{RetireOwned,RetirementCursor};
fn repair_one<K:Copy+Ord,V>(index:&mut NumericIndex<K,V>,current:usize)->Option<usize>{
 index.update(current);let(left,right)={let node=index.node(current);(node.left,node.right)};let balance=index.height(left)as i16-index.height(right)as i16;
 let top=if balance>1{let child=left.unwrap();if index.height(index.node(child).left)<index.height(index.node(child).right){index.rotate_left(child);}index.rotate_right(current)}else if balance< -1{let child=right.unwrap();if index.height(index.node(child).right)<index.height(index.node(child).left){index.rotate_right(child);}index.rotate_left(current)}else{current};
 index.node(top).parent
}
enum NumericWorkStep{Pending,Progress,Complete}
pub struct NumericWorkInsertCursor<K,V>{key:K,value:Option<V>,previous:Option<V>,current:Option<usize>,parent:Option<usize>,phase:u8,pending:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64}
impl<K:Copy+Ord,V> NumericWorkInsertCursor<K,V>{
 pub fn new(key:K,value:V)->Self{Self{key,value:Some(value),previous:None,current:None,parent:None,phase:0,pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0}}
 pub fn is_complete(&self)->bool{self.phase==4}
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn take_previous(&mut self)->Option<V>{assert!(self.is_complete());self.previous.take()}
 fn step(&mut self,index:&mut NumericIndex<K,V>,work:&mut RetainedWorkLedger)->Result<NumericWorkStep,&'static str>{
  match self.phase{
   0=>{self.current=index.root;self.phase=1;},
   1=>{if let Some(current)=self.current{match self.key.cmp(&index.node(current).key){
    Ordering::Equal=>{let generation=index.generation;let node=index.node_mut(current);self.previous=node.value.replace(self.value.take().ok_or("value absent")?);node.generation=generation;self.phase=3;},
    order=>{self.parent=Some(current);self.current=if order==Ordering::Less{index.node(current).left}else{index.node(current).right};},
   }}else{self.phase=2;}},
   2=>{if !work.slot(&mut index.nodes)?{return Ok(NumericWorkStep::Pending);}let placed=index.nodes.len();let node=Node{key:self.key,value:self.value.take(),generation:index.generation,count_generation:index.generation,count:1,height:1,left:None,right:None,parent:self.parent};work.push(&mut index.nodes,node).map_err(|(error,_)|error)?;if let Some(parent)=self.parent{if self.key<index.node(parent).key{index.node_mut(parent).left=Some(placed)}else{index.node_mut(parent).right=Some(placed)}}else{index.root=Some(placed)}self.current=self.parent;self.phase=3;},
   3=>{if let Some(current)=self.current{self.current=repair_one(index,current);}else{self.phase=4;}},
   _=>return Ok(NumericWorkStep::Pending),
  }Ok(if self.phase==4{NumericWorkStep::Complete}else{NumericWorkStep::Progress})
 }
 pub fn advance(&mut self,index:&mut NumericIndex<K,V>,items:usize,work:&mut RetainedWorkLedger)->Result<bool,&'static str>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if items==0{return Err("numeric original work requires positive items");}let d=self.pending;
  if !work.admits_absolute(d){work.require_absolute(d);return Ok(self.is_complete());}
  let Some(root)=work.enter()else{self.pending=work.next_demand();return Ok(self.is_complete());};
  let result=(||{for _ in 0..items{if self.is_complete()||!work.can_step(){break;}let before=work.receipt();let result=self.step(index,work);let semantic=matches!(&result,Ok(NumericWorkStep::Complete)|Err(_));let actual=!matches!(&result,Ok(NumericWorkStep::Pending))||work.receipt()!=before;if actual{work.note_source(root);self.last_turn_work+=1;}if semantic{self.last_turn_semantic_work+=1;}self.pending=work.next_demand();result?;if work.blocked(){break;}}Ok(self.is_complete())})();work.exit();result
 }
}
impl<K:RetireOwned,V:RetireOwned> RetireOwned for NumericWorkInsertCursor<K,V>{
 fn retirement(self)->Box<dyn RetirementCursor>{use crate::retirement::{sequence,deferred};sequence(vec![deferred(self.key),deferred(self.value),deferred(self.previous),deferred(self.current),deferred(self.parent),deferred(self.phase)])}
 fn retirement_birth_bytes(&self)->Option<usize>{use crate::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.key),deferred_birth_bytes_for(&self.value),deferred_birth_bytes_for(&self.previous),deferred_birth_bytes_for(&self.current),deferred_birth_bytes_for(&self.parent),deferred_birth_bytes_for(&self.phase)])}
 fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
}
/// 🧹️ Tombstones one actual key while retaining its former payload through paid ancestor repair.
pub struct NumericWorkRemoveCursor<K,V>{key:K,value:Option<V>,current:Option<usize>,phase:u8,pending:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64}
impl<K:Copy+Ord,V> NumericWorkRemoveCursor<K,V>{
 pub fn new(key:K)->Self{Self{key,value:None,current:None,phase:0,pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0}}
 pub fn is_complete(&self)->bool{self.phase==4}
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn take_value(&mut self)->Option<V>{assert!(self.is_complete());self.value.take()}
 fn step(&mut self,index:&mut NumericIndex<K,V>,_work:&mut RetainedWorkLedger)->Result<NumericWorkStep,&'static str>{
  match self.phase{
   0=>{self.current=index.root;self.phase=1;},
   1=>{if let Some(current)=self.current{match self.key.cmp(&index.node(current).key){Ordering::Equal=>self.phase=2,order=>self.current=if order==Ordering::Less{index.node(current).left}else{index.node(current).right}}}else{self.phase=4;}},
   2=>{let current=self.current.ok_or("current absent")?;if index.node(current).generation==index.generation{self.value=index.node_mut(current).value.take();self.phase=3;}else{self.phase=4;}},
   3=>{if let Some(current)=self.current{self.current=repair_one(index,current);}else{self.phase=4;}},
   _=>return Ok(NumericWorkStep::Pending),
  }Ok(if self.phase==4{NumericWorkStep::Complete}else{NumericWorkStep::Progress})
 }
 pub fn advance(&mut self,index:&mut NumericIndex<K,V>,items:usize,work:&mut RetainedWorkLedger)->Result<bool,&'static str>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if items==0{return Err("numeric original removal requires positive items");}let d=self.pending;
  if !work.admits_absolute(d){work.require_absolute(d);return Ok(self.is_complete());}
  let Some(root)=work.enter()else{self.pending=work.next_demand();return Ok(self.is_complete());};
  let result=(||{for _ in 0..items{if self.is_complete()||!work.can_step(){break;}let before=work.receipt();let result=self.step(index,work);let semantic=matches!(&result,Ok(NumericWorkStep::Complete)|Err(_));let actual=!matches!(&result,Ok(NumericWorkStep::Pending))||work.receipt()!=before;if actual{work.note_source(root);self.last_turn_work+=1;}if semantic{self.last_turn_semantic_work+=1;}self.pending=work.next_demand();result?;if work.blocked(){break;}}Ok(self.is_complete())})();work.exit();result
 }
}
impl<K:RetireOwned,V:RetireOwned> RetireOwned for NumericWorkRemoveCursor<K,V>{
 fn retirement(self)->Box<dyn RetirementCursor>{use crate::retirement::{sequence,deferred};sequence(vec![deferred(self.key),deferred(self.value),deferred(self.current),deferred(self.phase)])}
 fn retirement_birth_bytes(&self)->Option<usize>{use crate::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.key),deferred_birth_bytes_for(&self.value),deferred_birth_bytes_for(&self.current),deferred_birth_bytes_for(&self.phase)])}
 fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
}
