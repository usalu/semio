//! 🔠️ Borrowed virtual labels and stable index merges move only individually paid original fields.
use super::super::ports::PortName;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetireOwned,ValueError,ValueRefusalKind};
use std::mem::size_of;
use super::{fixed,paid};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original layout index ordering changed its borrowed labels")}
#[derive(Clone,Copy)]
pub(super) struct Label<'a>{pub(super) head:&'a str,pub(super) tail:Option<PortName<'a>>}
impl<'a> Label<'a>{
 pub(super) fn text(head:&'a str)->Self{Self{head,tail:None}}
 fn number_length(value:usize)->usize{if value==0{1}else{value.ilog10()as usize+1}}
 fn tail_length(self)->usize{match self.tail{None=>0,Some(PortName::Text(text))=>text.len(),Some(PortName::Number(number))=>Self::number_length(number)}}
 fn len(self)->usize{self.head.len()+usize::from(self.tail.is_some())+self.tail_length()}
 fn byte(self,index:usize)->u8{if index<self.head.len(){return self.head.as_bytes()[index]}if index==self.head.len(){return b'@'}let at=index-self.head.len()-1;match self.tail.unwrap(){PortName::Text(text)=>text.as_bytes()[at],PortName::Number(value)=>b'0'+((value/10usize.pow((Self::number_length(value)-at-1)as u32))%10)as u8}}
 fn identity(self)->[u64;5]{let mut key=[self.head.as_ptr()as usize as u64,self.head.len()as u64,0,0,0];match self.tail{None=>{},Some(PortName::Text(text))=>{key[2]=1;key[3]=text.as_ptr()as usize as u64;key[4]=text.len()as u64},Some(PortName::Number(number))=>{key[2]=2;key[3]=number as u64}}key}
}
#[derive(RetireOwned)]
pub(super) struct TextOrder{binding:Option<([u64;5],[u64;5])>,offset:usize,done:Option<i8>}
impl TextOrder{
 pub(super) fn new()->Self{Self{binding:None,offset:0,done:None}}
 pub(super) fn result(&self)->Option<i8>{self.done}
 pub(super) fn step(&mut self,left:Label<'_>,right:Label<'_>,cx:&mut StepContext<'_>)->Result<Option<i8>,OriginalTurnFailure>{
  let key=(left.identity(),right.identity());if self.binding.is_some_and(|original|original!=key){return Err(invalid().into())}if self.done.is_some(){return Ok(self.done)}
  let common=left.len().min(right.len());let count=common.saturating_sub(self.offset).min(64).min(cx.retained_grant().maximum_copy_bytes.saturating_sub(size_of::<Self>())/2);
  if count==0&&self.offset<common{return Ok(None)}if !paid(cx,fixed(size_of::<Self>()+2*count))?{return Ok(None)}
  self.binding=Some(key);let mut a=[0;64];let mut b=[0;64];for i in 0..count{a[i]=left.byte(self.offset+i);b[i]=right.byte(self.offset+i)}
  self.done=match a[..count].cmp(&b[..count]){std::cmp::Ordering::Less=>Some(-1),std::cmp::Ordering::Greater=>Some(1),std::cmp::Ordering::Equal=>None};self.offset+=count;
  if self.done.is_none()&&self.offset==common{self.done=Some(match left.len().cmp(&right.len()){std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1})}Ok(self.done)
 }
 pub(super) fn reset(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !paid(cx,fixed(size_of::<Self>()))?{return Ok(false)}*self=Self::new();Ok(true)}
}
#[derive(RetireOwned)]
pub(super) struct IndexOrder{lex:TextOrder,width:usize,left:usize,middle:usize,end:usize,p:usize,q:usize,k:usize,choice:Option<bool>,phase:u8}
impl IndexOrder{
 pub(super) fn new()->Self{Self{lex:TextOrder::new(),width:1,left:0,middle:0,end:0,p:0,q:0,k:0,choice:None,phase:0}}
 pub(super) fn step<'a>(&mut self,order:&mut Vec<usize>,scratch:&mut Vec<usize>,label:impl Fn(usize)->Label<'a>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if order.len()!=scratch.len(){return Err(invalid().into())}
  match self.phase{
   0=>{if !paid(cx,fixed(size_of::<Self>()+2*size_of::<Vec<usize>>()))?{return Ok(false)}if self.width>=order.len(){self.phase=4;return Ok(true)}if self.left>=order.len(){std::mem::swap(order,scratch);self.width=self.width.saturating_mul(2);self.left=0}else{self.middle=self.left.saturating_add(self.width).min(order.len());self.end=self.middle.saturating_add(self.width).min(order.len());self.p=self.left;self.q=self.middle;self.k=self.left;self.phase=1}},
   1=>{if self.k==self.end{if paid(cx,fixed(size_of::<Self>()))?{self.left=self.end;self.phase=0}return Ok(false)}if self.p==self.middle||self.q==self.end{if paid(cx,fixed(size_of::<Self>()))?{self.choice=Some(self.q==self.end);self.phase=3}return Ok(false)}
    if self.lex.result().is_none(){self.lex.step(label(order[self.p]),label(order[self.q]),cx)?;return Ok(false)}
    if paid(cx,fixed(size_of::<Option<bool>>()+size_of::<u8>()))?{self.choice=Some(self.lex.result().unwrap()<=0);self.phase=2}
   },
   2=>{if self.lex.reset(cx)?{self.phase=3}},
   3=>{if paid(cx,fixed(size_of::<Self>()+size_of::<usize>()))?{if self.choice.take().ok_or_else(invalid)?{scratch[self.k]=order[self.p];self.p+=1}else{scratch[self.k]=order[self.q];self.q+=1}self.k+=1;self.phase=1}},
   4=>return Ok(true),_=>return Err(invalid().into()),
  }Ok(false)
 }
}
