//! 🎬️ One immutable AVL reconstruction retains every unallocated input and builds one real node per turn.
use super::*;
pub(super) struct EntryBuilder<V>{pub(super) key:Option<SharedOwner<String>>,pub(super) value:Option<SharedOwner<V>>}
impl<V> EntryBuilder<V>{pub(super) fn held(&self)->usize{usize::from(self.key.is_some())+usize::from(self.value.is_some())}pub(super) fn take_owner(&mut self)->Option<Owner<V>>{self.key.take().map(Owner::Key).or_else(||self.value.take().map(Owner::Value))}}
pub(super) struct NodeBuilder<V>{pub(super) entry:Option<SharedOwner<Entry<V>>>,pub(super) left:Root<V>,pub(super) right:Root<V>,pub(super) height:usize,pub(super) len:usize}
impl<V> NodeBuilder<V>{
 pub(super) fn validate(&mut self)->Result<(),ValueError>{self.height=1+height(&self.left).max(height(&self.right));self.len=1usize.checked_add(len(&self.left)).and_then(|n|n.checked_add(len(&self.right))).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered staged node length overflow"))?;if self.height>MAX_AVL_HEIGHT{return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered staged node exceeds AVL metadata height"))}Ok(())}
 pub(super) fn held(&self)->usize{usize::from(self.entry.is_some())+usize::from(self.left.is_some())+usize::from(self.right.is_some())}
 pub(super) fn take_owner(&mut self)->Option<Owner<V>>{self.left.take().or_else(||self.right.take()).map(Owner::Node).or_else(||self.entry.take().map(Owner::Entry))}
 pub(super) fn take_node(&mut self)->Option<Node<V>>{let entry=self.entry.take()?;Some(Node{entry,left:self.left.take(),right:self.right.take(),height:self.height,len:self.len})}
 pub(super) fn restore(&mut self,node:Node<V>){self.entry=Some(node.entry);self.left=node.left;self.right=node.right;self.height=node.height;self.len=node.len;}
}
enum Link<V>{Existing(Root<V>),Built(usize)}
impl<V> Link<V>{fn held(&self)->usize{match self{Self::Existing(root)=>usize::from(root.is_some()),Self::Built(_)=>0}}fn resolve(self,outputs:&mut[Root<V>;3])->Result<Root<V>,crate::ValueError>{match self{Self::Existing(root)=>Ok(root),Self::Built(index)=>outputs.get_mut(index).and_then(Option::take).map(Some).ok_or_else(||crate::ValueError::invariant("prior planned node is absent"))}}fn take_owner(&mut self)->Option<Owner<V>>{match self{Self::Existing(root)=>root.take().map(Owner::Node),Self::Built(_)=>None}}}
struct Spec<V>{entry:Option<SharedOwner<Entry<V>>>,left:Link<V>,right:Link<V>}
pub(super) struct BalancePlan<V>{specs:[Option<Spec<V>>;3],outputs:[Root<V>;3],pub(super) next:usize,count:usize,pub(super) after:Phase}
impl<V> BalancePlan<V>{
 pub(super) fn single(entry:SharedOwner<Entry<V>>,left:Root<V>,right:Root<V>,after:Phase)->Self{let mut specs=std::array::from_fn(|_|None);specs[0]=Some(Spec{entry:Some(entry),left:Link::Existing(left),right:Link::Existing(right)});Self{specs,outputs:std::array::from_fn(|_|None),next:0,count:1,after}}
 pub(super) fn new_aliases(left:&Root<V>,right:&Root<V>)->usize{
  let children=|branch:&Node<V>|usize::from(branch.left.is_some())+usize::from(branch.right.is_some());
  if let Some(branch)=left.as_ref().filter(|_|height(left)>height(right)+1){if height(&branch.left)>=height(&branch.right){1+children(branch)}else{2+usize::from(branch.left.is_some())+branch.right.as_ref().map_or(0,|pivot|children(pivot))}}
  else if let Some(branch)=right.as_ref().filter(|_|height(right)>height(left)+1){if height(&branch.right)>=height(&branch.left){1+children(branch)}else{2+usize::from(branch.right.is_some())+branch.left.as_ref().map_or(0,|pivot|children(pivot))}}
  else{0}
 }
 pub(super) fn balanced(entry:SharedOwner<Entry<V>>,left:Root<V>,right:Root<V>,after:Phase,retired:&mut InlineStack<SharedOwner<Node<V>>,UPDATE_RETIRED_ROOTS>)->Self{
  let mut specs=std::array::from_fn(|_|None);let count;
  let(left_heavy,right_heavy)=(height(&left)>height(&right)+1,height(&right)>height(&left)+1);
  match(left_heavy,right_heavy,left,right){
   (true,_,Some(root),right)=>{let branch=&root;match branch.right.as_ref().filter(|_|height(&branch.left)<height(&branch.right)){None=>{specs[0]=Some(Spec{entry:Some(entry),left:Link::Existing(branch.right.clone()),right:Link::Existing(right)});specs[1]=Some(Spec{entry:Some(SharedOwner::clone(&branch.entry)),left:Link::Existing(branch.left.clone()),right:Link::Built(0)});count=2;},Some(pivot)=>{specs[0]=Some(Spec{entry:Some(SharedOwner::clone(&branch.entry)),left:Link::Existing(branch.left.clone()),right:Link::Existing(pivot.left.clone())});specs[1]=Some(Spec{entry:Some(entry),left:Link::Existing(pivot.right.clone()),right:Link::Existing(right)});specs[2]=Some(Spec{entry:Some(SharedOwner::clone(&pivot.entry)),left:Link::Built(0),right:Link::Built(1)});count=3;}}retired.push_front(root);},
   (_,true,left,Some(root))=>{let branch=&root;match branch.left.as_ref().filter(|_|height(&branch.right)<height(&branch.left)){None=>{specs[0]=Some(Spec{entry:Some(entry),left:Link::Existing(left),right:Link::Existing(branch.left.clone())});specs[1]=Some(Spec{entry:Some(SharedOwner::clone(&branch.entry)),left:Link::Built(0),right:Link::Existing(branch.right.clone())});count=2;},Some(pivot)=>{specs[0]=Some(Spec{entry:Some(entry),left:Link::Existing(left),right:Link::Existing(pivot.left.clone())});specs[1]=Some(Spec{entry:Some(SharedOwner::clone(&branch.entry)),left:Link::Existing(pivot.right.clone()),right:Link::Existing(branch.right.clone())});specs[2]=Some(Spec{entry:Some(SharedOwner::clone(&pivot.entry)),left:Link::Built(0),right:Link::Built(1)});count=3;}}retired.push_front(root);},
   (_,_,left,right)=>return Self::single(entry,left,right,after),
  }
  Self{specs,outputs:std::array::from_fn(|_|None),next:0,count,after}
 }
 pub(super) fn ready(&self)->bool{self.next==self.count}
 pub(super) fn prepare(&mut self)->Result<NodeBuilder<V>,crate::ValueError>{let spec=self.specs.get_mut(self.next).and_then(Option::take).ok_or_else(||crate::ValueError::invariant("planned node spec is absent"))?;Ok(NodeBuilder{entry:spec.entry,left:spec.left.resolve(&mut self.outputs)?,right:spec.right.resolve(&mut self.outputs)?,height:0,len:0})}
 pub(super) fn accept(&mut self,node:SharedOwner<Node<V>>){self.outputs[self.next]=Some(node);self.next+=1;}
 pub(super) fn take_result(&mut self)->Root<V>{self.outputs[self.count-1].take()}
 pub(super) fn held(&self)->usize{self.outputs.iter().filter(|root|root.is_some()).count()+self.specs.iter().flatten().map(|spec|usize::from(spec.entry.is_some())+spec.left.held()+spec.right.held()).sum::<usize>()}
 pub(super) fn take_owner(&mut self)->Option<Owner<V>>{for spec in self.specs.iter_mut().flatten(){if let Some(entry)=spec.entry.take(){return Some(Owner::Entry(entry))}if let Some(owner)=spec.left.take_owner().or_else(||spec.right.take_owner()){return Some(owner)}}self.outputs.iter_mut().find_map(|root|root.take().map(Owner::Node))}
}
