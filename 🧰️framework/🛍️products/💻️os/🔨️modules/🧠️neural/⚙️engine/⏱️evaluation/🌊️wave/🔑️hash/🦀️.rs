//! 🔑️ Funded traversal of the original node-hash byte stream.
use super::*;
use std::hash::{Hash,Hasher};
pub(super) struct NodeHashCursor{path:Vec<usize>,hasher:std::collections::hash_map::DefaultHasher,phase:u8,offset:usize,progress:RetainedCloneProgress}
impl NodeHashCursor{
 pub fn new()->Self{Self{path:Vec::new(),hasher:Default::default(),phase:0,offset:0,progress:Default::default()}}
 fn dictionary<'a>(&self,input:&'a Dictionary)->&'a Dictionary{let mut value=input;for rank in &self.path[..self.path.len()-1]{value=value.entry_at_rank(*rank).unwrap().1.as_dictionary().unwrap()}value}
 fn value<'a>(&self,input:&'a Dictionary)->Option<(&'a String,&'a Value)>{self.dictionary(input).entry_at_rank(*self.path.last().unwrap())}
 pub fn demands(&self,kind:&str,input:&Dictionary)->RetirementDemand{let mut d=RetirementDemand{depth:self.path.len()+1,..Default::default()};d.copy_bytes=match self.phase{0=>{d.capacity_bytes=256*std::mem::size_of::<usize>();0},1=>usize::from(self.offset<kind.len()).max(1),2=>self.value(input).map_or(0,|(key,_)|usize::from(self.offset<key.len()).max(1)),3=>self.value(input).map_or(0,|(_,value)|match value{Value::Atom(Atom::Integer(_)|Atom::Decimal(_))=>8,_=>1}),4=>1,_=>0};d}
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 fn text(&mut self,text:&str,grant:RetainedCloneGrant)->bool{if self.offset<text.len(){let count=(text.len()-self.offset).min(grant.maximum_copy_bytes);self.hasher.write(&text.as_bytes()[self.offset..self.offset+count]);self.offset+=count;self.progress.copied_bytes=count;false}else{self.hasher.write_u8(255);self.offset=0;self.progress.copied_bytes=1;true}}
 fn advance_rank(&mut self){*self.path.last_mut().unwrap()+=1;self.phase=2;}
 pub fn step(&mut self,kind:&str,input:&Dictionary,grant:RetainedCloneGrant)->Result<Option<u64>,ValueError>{self.progress=Default::default();let d=self.demands(kind,input);if grant.maximum_items==0{return Ok(None)}if grant.maximum_depth<d.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original hash exceeds admitted source depth"))}if grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes{return Ok(None)}self.progress.copied_items=1;match self.phase{
  0=>{self.path.try_reserve_exact(256).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original hash path allocation refused"))?;self.progress.retained_capacity_bytes=self.path.capacity()*std::mem::size_of::<usize>();self.path.push(0);self.phase=1;},
  1=>{if self.text(kind,grant){self.phase=2}},
  2=>{if let Some((key,_))=self.value(input){let pointer=key.as_ptr();let length=key.len();if self.text(unsafe{std::str::from_utf8_unchecked(std::slice::from_raw_parts(pointer,length))},grant){self.phase=3}}else if self.path.len()==1{self.phase=255;return Ok(Some(self.hasher.finish()))}else{self.path.pop();self.advance_rank()}},
  3=>{let(_,value)=self.value(input).unwrap();match value{Value::Atom(Atom::Null)=>{0u8.hash(&mut self.hasher);self.progress.copied_bytes=1;self.advance_rank()},Value::Atom(Atom::Boolean(value))=>{value.hash(&mut self.hasher);self.progress.copied_bytes=1;self.advance_rank()},Value::Atom(Atom::Integer(value))=>{value.hash(&mut self.hasher);self.progress.copied_bytes=8;self.advance_rank()},Value::Atom(Atom::Decimal(value))=>{value.to_bits().hash(&mut self.hasher);self.progress.copied_bytes=8;self.advance_rank()},Value::Atom(Atom::String(_))=>self.phase=4,Value::Dictionary(_)=>{if self.path.len()==256{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original hash source exceeds retained path extent"))}0u8.hash(&mut self.hasher);self.progress.copied_bytes=1;self.path.push(0);self.phase=2}}},
  4=>{let text=self.value(input).unwrap().1.as_atom().unwrap().as_str().unwrap();let pointer=text.as_ptr();let length=text.len();if self.text(unsafe{std::str::from_utf8_unchecked(std::slice::from_raw_parts(pointer,length))},grant){self.advance_rank()}},
  _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original hash was already handed off")),
 }Ok(None)}
}
impl RetireOwned for NodeHashCursor{fn retirement(self)->Box<dyn RetirementCursor>{self.path.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.path.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
