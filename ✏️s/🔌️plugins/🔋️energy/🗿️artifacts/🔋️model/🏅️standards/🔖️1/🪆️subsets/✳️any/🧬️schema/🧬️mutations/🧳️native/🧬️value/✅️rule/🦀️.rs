//! ✅️ Owned field rules inspect bounded fragments from the original input and genuine target row.
use super::*;
/// 📏️ Authored policies declare fixed inspection cost and retain all finite validation state explicitly.
pub trait OriginalValueRule:Send+Sync+'static{type Input:Send+Sync;type Row:EntityRow;type State:Copy+Send;const INSPECTION_BYTES:usize;fn initial()->Self::State;fn len(input:&Self::Input)->usize;fn valid_len(length:usize,row:&Self::Row)->bool;fn inspect(input:&Self::Input,index:usize,row:&Self::Row,original:&EnergyModelSnapshot,state:&mut Self::State)->bool;fn finish(_:Self::State,_:&Self::Row)->bool{true}}
pub struct ValueValidation<P:OriginalValueRule>{input:Bound,row:Bound,original:Bound,offset:usize,state:P::State,phase:u8,result:Option<Option<RowOutcome>>,closing:bool,flight:Flight}
impl<P:OriginalValueRule> ValueValidation<P>{
 pub fn new()->Self{Self{input:Default::default(),row:Default::default(),original:Default::default(),offset:0,state:P::initial(),phase:0,result:None,closing:false,flight:Default::default()}}
 fn fixed()->usize{2*size_of::<usize>()+2*size_of::<P::State>()+3}
 fn end(&self,input:&P::Input,body:usize)->usize{self.offset+(body.saturating_sub(Self::fixed())/P::INSPECTION_BYTES.max(1)).min(P::len(input).saturating_sub(self.offset))}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(input,row,original);
}
crate::custody::std_cursor!([P:OriginalValueRule] ValueValidation<P>,[],|s|s.begin_close());
impl<P:OriginalValueRule> OwnedRowValidation<P::Input,P::Row> for ValueValidation<P>{
 fn quote(&self,input:RetainedCloneRef<'_,P::Input>,_:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,row:RetainedCloneRef<'_,P::Row>,body:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.result.is_some()||!self.flight.idle(){return Err(invalid());}input.check_binding(&self.input.0)?;row.check_binding(&self.row.0)?;original.check_binding(&self.original.0)?;match self.phase{0=>Ok((nested(scalar(input.binding_copy_bytes()),1)?,input.binding_work_demand(&self.input.0)?)),1=>Ok((nested(scalar(row.binding_copy_bytes()),1)?,row.binding_work_demand(&self.row.0)?)),2=>Ok((nested(scalar(original.binding_copy_bytes()),1)?,original.binding_work_demand(&self.original.0)?)),3|5=>Ok((scalar(Self::fixed()),1)),4=>{let n=self.end(input.get(),body)-self.offset;Ok((scalar(Self::fixed()+n*P::INSPECTION_BYTES),1+u64::try_from(n).map_err(|_|invalid())?))},_=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,input:RetainedCloneRef<'_,P::Input>,key:RetainedCloneRef<'_,u32>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,row:RetainedCloneRef<'_,P::Row>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.quote(input,key,original,row,body,max))};let g=effect.grant();
  custody::gated(effect,reply,quote,max,|effect,work|{let mut p=RetainedCloneProgress::default();match self.phase{
   0=>{p=add(bind(input,&mut self.input,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=1;},1=>{p=add(bind(row,&mut self.row,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=2;},2=>{p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=3;},
   3=>{if P::valid_len(P::len(input.get()),row.get()){self.phase=4;}else{self.result=Some(Some(RowOutcome::Invalid));}p=receipt(scalar(Self::fixed()));},
   4=>{let end=self.end(input.get(),body);while self.offset<end{if !P::inspect(input.get(),self.offset,row.get(),original.get(),&mut self.state){self.result=Some(Some(RowOutcome::Invalid));break;}self.offset+=1;}if self.result.is_none()&&self.offset==P::len(input.get()){self.phase=5;}p=receipt(scalar(Self::fixed()));},
   5=>{self.result=Some((!P::finish(self.state,row.get())).then_some(RowOutcome::Invalid));p=receipt(scalar(Self::fixed()));},
   _=>return Err(invalid().into())}
   Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 fn result(&self)->Option<Option<RowOutcome>>{self.result}
 fn begin_close(&mut self){Self::begin_close(self)}
}
