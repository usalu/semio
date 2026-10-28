//! 🔎️ Original continuations borrow owner-qualified declarations and exact source slots without admitting values.
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::size_of;
pub struct IfcQueryPart<'a,V>{pub name:&'a str,pub args:&'a[V]}
/// 🏛️ The original enclosing owner keeps this immutable population alive through query retirement.
pub trait IfcAttributeQueryPopulation: 'static{type Value;fn query_part_count(&self,row:usize)->Option<usize>;fn query_part(&self,row:usize,part:usize)->Option<IfcQueryPart<'_,Self::Value>>;}
pub struct IfcQueryAttributes<A:'static>{pub key:&'static str,pub attributes:&'static[A]}
/// 🧊️ Implementations perform only bounded immutable metadata lookups; accessors never prepare catalogs.
pub trait IfcAttributeQueryMetadata: 'static{type Attribute:'static;type Expression:'static;fn attributes(name:&str,direct:bool)->Option<IfcQueryAttributes<Self::Attribute>>;fn name(attribute:&'static Self::Attribute)->&'static str;fn declared_by(attribute:&'static Self::Attribute)->&'static str;fn type_expr(attribute:&'static Self::Attribute)->&'static Self::Expression;}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IfcAttributeQueryStatus{Missing,Unique,Ambiguous,Unknown}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct IfcAttributeAddress{pub entity:usize,pub part:usize,pub index:usize}
impl std::fmt::Display for IfcAttributeAddress{fn fmt(&self,output:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(output,"/entities/{}/",self.entity)?;if self.part>0{write!(output,"complex/{}/",self.part-1)?;}write!(output,"args/{}",self.index)}}
pub struct IfcAttributeMatch<'a,V,A:'static,E:'static>{pub value:Option<&'a V>,pub declaration:&'static A,pub type_expr:&'static E,pub address:IfcAttributeAddress}
pub struct IfcAttributeQueryResult<'a,V,A:'static,E:'static>{pub status:IfcAttributeQueryStatus,pub matches:[Option<IfcAttributeMatch<'a,V,A,E>>;2]}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Input{source:usize,source_type:std::any::TypeId,epoch:u64,row:usize,name:(usize,usize),owner:Option<(usize,usize)>,profile:std::any::TypeId}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Slot{address:IfcAttributeAddress,key:&'static str,direct:bool}
#[derive(Debug,PartialEq,Eq)]
pub struct IfcAttributeQueryWork{input:Option<Input>,original:Option<OriginalSourceBinding>,expected:[u8;256],qualified:[u8;256],expected_len:usize,qualified_len:usize,word:usize,part:usize,attribute:usize,key:Option<&'static str>,slots:[Option<Slot>;2],count:usize,phase:u8,unknown:bool,done:bool,closed:bool}
impl Default for IfcAttributeQueryWork{fn default()->Self{Self{input:None,original:None,expected:[0;256],qualified:[0;256],expected_len:0,qualified_len:0,word:0,part:0,attribute:0,key:None,slots:[None;2],count:0,phase:0,unknown:false,done:false,closed:false}}}
semio_framework_value::artifact_retire_leaf!(IfcAttributeQueryWork);
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"ifc.query.original-source")}
fn cancelled()->ValueError{ValueError::literal(ValueRefusalKind::Canceled,"io.cancelled")}
fn input<C:IfcAttributeQueryMetadata,S:IfcAttributeQueryPopulation>(source:&S,row:usize,name:&str,owner:Option<&str>,epoch:u64)->Input{Input{source:source as*const S as usize,source_type:std::any::TypeId::of::<S>(),epoch,row,name:(name.as_ptr()as usize,name.len()),owner:owner.map(|value|(value.as_ptr()as usize,value.len())),profile:std::any::TypeId::of::<C>()}}
fn identifier(byte:u8,index:usize)->bool{byte.is_ascii_alphabetic()||byte==b'_'||index>0&&byte.is_ascii_digit()}
fn equal(text:&str,key:&[u8])->bool{text.len()==key.len()&&text.as_bytes().eq_ignore_ascii_case(key)}
impl IfcAttributeQueryWork{
    pub fn demands(&self)->RetirementDemand{Self::core_demands()}
    pub(super) fn core_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>()+4*256,depth:1,..Default::default()}}
    pub(super) fn check_core<C:IfcAttributeQueryMetadata,S:IfcAttributeQueryPopulation>(&self,source:&S,row:usize,name:&str,owner:Option<&str>,epoch:u64)->Result<(),ValueError>{if self.closed||row as u128>9_007_199_254_740_991u128||self.input.is_some_and(|old|old!=input::<C,S>(source,row,name,owner,epoch)){return Err(invalid());}Ok(())}
    pub(super) fn advance_core<C:IfcAttributeQueryMetadata,S:IfcAttributeQueryPopulation>(&mut self,source:&S,row:usize,name:&str,owner:Option<&str>,epoch:u64)->Result<bool,ValueError>{self.check_core::<C,S>(source,row,name,owner,epoch)?;if self.done{return Ok(true);}if self.input.is_none(){self.input=Some(input::<C,S>(source,row,name,owner,epoch));self.phase=1;}else{self.advance_one::<C,S>(source,row,name,owner)?;}Ok(self.done)}
    pub fn terminal_is_empty(&self)->bool{self.input.is_none()}
    pub fn advance<C:IfcAttributeQueryMetadata,S:IfcAttributeQueryPopulation>(&mut self,source:&S,row:usize,name:&str,owner:Option<&str>,epoch:u64,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if cx.is_cancelled(){return Err(cancelled().into());}self.check_core::<C,S>(source,row,name,owner,epoch)?;if self.original.is_some_and(|old|!old.matches(cx)){return Err(invalid().into());}let demand=self.demands();if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(false);}if self.done{return Ok(true);}
        if self.input.is_none(){self.original=Some(OriginalSourceBinding::borrow(cx));}self.advance_core::<C,S>(source,row,name,owner,epoch)?;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(self.done)
    }
    fn advance_one<C:IfcAttributeQueryMetadata,S:IfcAttributeQueryPopulation>(&mut self,source:&S,row:usize,name:&str,owner:Option<&str>)->Result<(),ValueError>{
        match self.phase{
            1|2=>{let value=if self.phase==1{name}else if let Some(owner)=owner{owner}else{self.phase=3;self.word=0;return Ok(())};if value.is_empty()||value.len()>256{self.unknown=true;self.done=true;}else if self.word<value.len(){let byte=value.as_bytes()[self.word];if !identifier(byte,self.word){self.unknown=true;self.done=true;}else{if self.phase==1{self.expected[self.word]=byte.to_ascii_uppercase();}else{self.qualified[self.word]=byte.to_ascii_uppercase();}self.word+=1;}}else{if self.phase==1{self.expected_len=value.len();}else{self.qualified_len=value.len();}self.word=0;self.phase+=1;}},
            3=>{let parts=source.query_part_count(row).ok_or_else(invalid)?;if self.part==parts{self.done=true;}else if self.part>parts{return Err(invalid());}else{let part=source.query_part(row,self.part).ok_or_else(invalid)?;if part.name.is_empty()||part.name.len()>256{self.unknown=true;self.part+=1;}else{self.word=0;self.phase=4;}}},
            4=>{let part=source.query_part(row,self.part).ok_or_else(invalid)?;if self.word<part.name.len(){if !identifier(part.name.as_bytes()[self.word],self.word){self.unknown=true;self.part+=1;self.phase=3;}else{self.word+=1;}}else{let direct=source.query_part_count(row).ok_or_else(invalid)?>1;if let Some(declaration)=C::attributes(part.name,direct){if self.count==2{self.part+=1;self.phase=3;}else{self.key=Some(declaration.key);self.attribute=0;self.phase=5;}}else{self.unknown=true;self.part+=1;self.phase=3;}}},
            5=>{let direct=source.query_part_count(row).ok_or_else(invalid)?>1;let declaration=C::attributes(self.key.ok_or_else(invalid)?,direct).ok_or_else(invalid)?;if let Some(attribute)=declaration.attributes.get(self.attribute){if equal(C::name(attribute),&self.expected[..self.expected_len])&&(owner.is_none()||equal(C::declared_by(attribute),&self.qualified[..self.qualified_len])){self.slots[self.count]=Some(Slot{address:IfcAttributeAddress{entity:row,part:self.part,index:self.attribute},key:declaration.key,direct});self.count+=1;}self.attribute+=1;if self.count==2{self.part+=1;self.phase=3;}}else{self.part+=1;self.phase=3;}},
            _=>return Err(invalid()),
        }Ok(())
    }
    pub fn completed<'a,C:IfcAttributeQueryMetadata,S:IfcAttributeQueryPopulation>(&self,source:&'a S,row:usize,name:&str,owner:Option<&str>,epoch:u64,cx:&StepContext<'_>)->Result<Option<IfcAttributeQueryResult<'a,S::Value,C::Attribute,C::Expression>>,ValueError>{
        if cx.is_cancelled(){return Err(cancelled());}if self.closed{return Err(invalid());}if self.input.is_none(){return Ok(None);}if self.original.is_none_or(|original|!original.matches(cx)){return Err(invalid());}self.completed_core::<C,S>(source,row,name,owner,epoch)
    }
    pub(super) fn completed_core<'a,C:IfcAttributeQueryMetadata,S:IfcAttributeQueryPopulation>(&self,source:&'a S,row:usize,name:&str,owner:Option<&str>,epoch:u64)->Result<Option<IfcAttributeQueryResult<'a,S::Value,C::Attribute,C::Expression>>,ValueError>{
        self.check_core::<C,S>(source,row,name,owner,epoch)?;if self.input.is_none()||!self.done{return Ok(None);}let mut matches=std::array::from_fn(|_|None);for(index,slot)in self.slots.iter().enumerate(){if let Some(slot)=slot{let declaration=C::attributes(slot.key,slot.direct).ok_or_else(invalid)?;let attribute=declaration.attributes.get(slot.address.index).ok_or_else(invalid)?;let part=source.query_part(row,slot.address.part).ok_or_else(invalid)?;matches[index]=Some(IfcAttributeMatch{value:part.args.get(slot.address.index),declaration:attribute,type_expr:C::type_expr(attribute),address:slot.address});}}Ok(Some(IfcAttributeQueryResult{status:if self.unknown{IfcAttributeQueryStatus::Unknown}else if self.count==2{IfcAttributeQueryStatus::Ambiguous}else if self.count==1{IfcAttributeQueryStatus::Unique}else{IfcAttributeQueryStatus::Missing},matches}))
    }
    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        if self.original.is_some_and(|original|!original.matches(cx)){return Err(invalid().into());}if self.terminal_is_empty(){return Ok(());}let demand=self.demands();semio_framework_job::close_original_retained_step(cx,demand,|_|{*self=Self{closed:true,..Default::default()};Ok(semio_framework_value::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})?;Ok(())
    }
}
