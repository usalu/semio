//! 🔀️ Original reorder validation compares two independently retained sorted key tables through bounded heap-sort work.
use super::*;
use semio_framework_value_edit::{HeapSortCursor,HeapSortWork};
/// 🧭️ A permutation path lends fixed original keys from the authored vector and selected row.
pub trait OriginalPermutationPath:OwnedRowPath{fn input_len(v:&Self::Value)->usize;fn input_key(v:&Self::Value,index:usize)->u32;fn held_len(r:&Self::Row)->usize;fn held_key(r:&Self::Row,index:usize)->u32;}
pub struct PermutationValidation<S:OriginalPermutationPath>{input:Bound,row:Bound,body:Option<usize>,wanted:Held<Vec<u32>>,held:Held<Vec<u32>>,sort:HeapSortCursor,index:usize,phase:u8,finished:bool,mismatch:bool,closing:bool,flight:Flight,marker:std::marker::PhantomData<fn()->S>}
impl<S:OriginalPermutationPath> PermutationValidation<S>{
 pub fn new()->Self{Self{input:Default::default(),row:Default::default(),body:None,wanted:Default::default(),held:Default::default(),sort:HeapSortCursor::new(0),index:0,phase:0,finished:false,mismatch:false,closing:false,flight:Default::default(),marker:Default::default()}}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(wanted,held,input,row);
}
crate::custody::std_cursor!([S:OriginalPermutationPath] PermutationValidation<S>,[],|s|s.begin_close());
impl<S:OriginalPermutationPath> OwnedRowValidation<S::Value,S::Row> for PermutationValidation<S>{
 fn quote(&self,v:RetainedCloneRef<'_,S::Value>,_:RetainedCloneRef<'_,u32>,_:RetainedCloneRef<'_,EnergyModelSnapshot>,r:RetainedCloneRef<'_,S::Row>,body:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.finished||self.body.is_some_and(|b|b!=body)||!self.flight.idle(){return Err(invalid());}v.check_binding(&self.input.0)?;r.check_binding(&self.row.0)?;match self.phase{0=>Ok((nested(scalar(v.binding_copy_bytes()+size_of::<usize>()),1)?,v.binding_work_demand(&self.input.0)?)),1=>Ok((nested(scalar(r.binding_copy_bytes()),1)?,r.binding_work_demand(&self.row.0)?)),2=>Ok((scalar(6*size_of::<usize>()+2),1)),3|7=>Ok((RetirementDemand{copy_bytes:2*size_of::<Option<Vec<u32>>>()+1,capacity_bytes:S::input_len(v.get()).checked_mul(size_of::<u32>()).ok_or_else(invalid)?,depth:1,..Default::default()},1)),4|8|11=>Ok((scalar(4*size_of::<u32>()+4*size_of::<usize>()+4),1)),5|6|9|10=>Ok((scalar(2*size_of::<HeapSortCursor>()+4*size_of::<u32>()+4*size_of::<usize>()+1),1)),_=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,v:RetainedCloneRef<'_,S::Value>,key:RetainedCloneRef<'_,u32>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,r:RetainedCloneRef<'_,S::Row>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.quote(v,key,o,r,body,max))};let g=effect.grant();
  custody::gated(effect,reply,quote,max,|effect,work|{let mut p=receipt(scalar(1));match self.phase{
 0=>{p=add(bind(v,&mut self.input,&mut effect.nested(child(g,1+size_of::<usize>())),work)?,1+size_of::<usize>())?;self.body=Some(body);self.phase=1;},1=>{p=add(bind(r,&mut self.row,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=2;},
 2=>{if S::input_len(v.get())!=S::held_len(r.get()){self.finished=true;self.mismatch=true;}else{self.phase=3;}},
 3=>{let(values,bytes)=custody::exact_vec::<u32>(S::input_len(v.get()),effect)?;p.retained_capacity_bytes=bytes;self.wanted.set(values);self.phase=4;},
 4=>{if self.index==S::input_len(v.get()){self.phase=5;}else{self.wanted.get_mut().ok_or_else(invalid)?.push(S::input_key(v.get(),self.index));self.index+=1;}},
 5|9=>{self.sort=HeapSortCursor::new(S::input_len(v.get()));self.phase+=1;},
 6|10=>{let values=if self.phase==6{self.wanted.get_mut()}else{self.held.get_mut()}.ok_or_else(invalid)?;match self.sort.next(){HeapSortWork::Compare{left,right}=>self.sort.advance(Some(values[left].cmp(&values[right]))).map_err(|_|invalid())?,HeapSortWork::Swap{left,right}=>{values.swap(left,right);self.sort.advance(None).map_err(|_|invalid())?;},HeapSortWork::Advance=>self.sort.advance(None).map_err(|_|invalid())?,HeapSortWork::Complete=>{self.index=0;self.phase+=1;}}},
 7=>{let(values,bytes)=custody::exact_vec::<u32>(S::held_len(r.get()),effect)?;p.retained_capacity_bytes=bytes;self.held.set(values);self.phase=8;},
 8=>{if self.index==S::held_len(r.get()){self.phase=9;}else{self.held.get_mut().ok_or_else(invalid)?.push(S::held_key(r.get(),self.index));self.index+=1;}},
 11=>{if self.index==S::input_len(v.get()){self.finished=true;}else if self.wanted.get().ok_or_else(invalid)?[self.index]!=self.held.get().ok_or_else(invalid)?[self.index]{self.finished=true;self.mismatch=true;}else{self.index+=1;}},
 _=>return Err(invalid().into())}
   Ok(if self.finished{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 fn result(&self)->Option<Option<RowOutcome>>{self.finished.then_some(self.mismatch.then_some(RowOutcome::InvalidContext))}
 fn begin_close(&mut self){Self::begin_close(self)}
}
