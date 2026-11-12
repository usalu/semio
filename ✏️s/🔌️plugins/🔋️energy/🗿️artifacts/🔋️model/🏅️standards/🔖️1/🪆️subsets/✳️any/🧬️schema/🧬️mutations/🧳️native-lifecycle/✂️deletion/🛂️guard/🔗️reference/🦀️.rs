//! 🔗️ A native reference restriction reads original pages and individual source keys before admitting removal.
use super::*;
use semio_framework_value_persistent::list::persistent::PersistentList;
/// 🧭️ Each authored relation names its actual referencing collection and original scalar or vector key source.
pub trait RowReferenceSource:Send+Sync+'static{type Row:EntityRow;fn rows(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>;fn len(row:&Self::Row)->usize;fn key(row:&Self::Row,index:usize)->Result<u32,ValueError>;}
/// 🧳️ The scan reads the current original row while a wide reference vector is inspected one key per turn.
pub struct ReferenceRowGuard<S:RowReferenceSource>{original:Bound,deleted:Bound,position:usize,index:usize,allowed:Option<bool>,phase:u8,closing:bool,flight:Flight,marker:std::marker::PhantomData<fn()->S>}
impl<S:RowReferenceSource> ReferenceRowGuard<S>{
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(deleted,original);
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:RowReferenceSource] ReferenceRowGuard<S>,[],|s|s.begin_close());
impl<S:RowReferenceSource,R:EntityRow> RowDeletionGuard<R> for ReferenceRowGuard<S>{
 fn new()->Self{Self{original:Default::default(),deleted:Default::default(),position:0,index:0,allowed:None,phase:0,closing:false,flight:Default::default(),marker:Default::default()}}
 fn quote(&self,row:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,_:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.phase==99||!self.flight.idle(){return Err(invalid());}row.check_binding(&self.deleted.0)?;original.check_binding(&self.original.0)?;match self.phase{
  0=>Ok((nested(scalar(row.binding_copy_bytes()),1)?,row.binding_work_demand(&self.deleted.0)?)),1=>Ok((nested(scalar(original.binding_copy_bytes()),1)?,original.binding_work_demand(&self.original.0)?)),
  2=>Ok((scalar(size_of::<u32>()*2+4*size_of::<usize>()+2*size_of::<Option<bool>>()+1),1)),_=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,deleted:RetainedCloneRef<'_,R>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,m:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let q=if recover{None}else{Some(<Self as RowDeletionGuard<R>>::quote(self,deleted,original,body,m))};let g=effect.grant();
  custody::gated(effect,reply,q,m,|effect,work|{let p=match self.phase{
   0=>{let p=add(bind(deleted,&mut self.deleted,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=1;p},
   1=>{let p=add(bind(original,&mut self.original,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=2;p},
   2=>{let rows=S::rows(original);match rows.get().get(self.position){None=>{self.allowed=Some(true);self.phase=99;},Some(row)=>{if self.index>=S::len(row){self.position+=1;self.index=0;}else{let found=S::key(row,self.index)?==deleted.get().entity_id();self.index+=1;if found{self.allowed=Some(false);self.phase=99;}}}}receipt(scalar(1))},
   _=>return Err(invalid().into())};
   Ok(if self.phase==99{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 fn result(&self)->Option<bool>{if !self.closing&&self.phase==99{self.allowed}else{None}}
}
