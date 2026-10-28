//! 🌱️ Row creation chooses its authored rank while checking actual original identifiers.
use super::*;
use crate::snapshot::state::fields::EntityRow;
use semio_framework_value_persistent::list::persistent::{PersistentListRead,PersistentListReadStep};
/// 🧭️ Duplicate identities and out of range authored ranks preserve distinct semantic diagnostics.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum RowAdmission{Rank(usize),Duplicate,Missing}
/// 🔎️ The original page cursor inspects one genuine row at a time without collecting a model.
pub struct RowAdmissionScan<T:EntityRow>{reader:PersistentListRead<T>,key:Option<RetainedCloneBinding>,index:Option<usize>,sorted:Option<usize>,result:Option<RowAdmission>,closing:bool}
impl<T:EntityRow> RowAdmissionScan<T>{
 pub fn new(index:Option<usize>)->Self{Self{reader:PersistentListRead::new(),key:None,index,sorted:None,result:None,closing:false}}
 pub fn result(&self)->Option<RowAdmission>{self.result}
 fn fixed()->usize{2*size_of::<Option<usize>>()+2*size_of::<Option<RowAdmission>>()+2*size_of::<usize>()+size_of::<u32>()}
 pub fn demand(&self,rows:RetainedCloneRef<'_,PersistentList<T>>,key:RetainedCloneRef<'_,u32>)->Result<RetirementDemand,ValueError>{if self.closing||self.result.is_some(){return Err(invalid());}key.check_binding(&self.key)?;if self.key.is_none(){return nested(scalar(key.binding_copy_bytes()),1);}nested(self.reader.demand(rows)?,Self::fixed())}
 pub fn advance(&mut self,rows:RetainedCloneRef<'_,PersistentList<T>>,key:RetainedCloneRef<'_,u32>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(rows,key)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.key.is_none(){let p=key.bind(&mut self.key,child(g,1))?.ok_or_else(invalid)?;return Ok(RetainedCloneStep::Progress(add(p,1)?));}let extra=Self::fixed();let s=self.reader.advance(rows,child(g,extra)).map_err(|e|tag(e,extra))?;let p=add(s.progress(),extra)?;match s{PersistentListReadStep::Row(_,rank,row)=>{let id=row.get().entity_id();if id==*key.get(){self.result=Some(RowAdmission::Duplicate);}else if self.sorted.is_none()&&id>*key.get(){self.sorted=Some(rank);}},PersistentListReadStep::Complete(_)=>{let rank=self.index.or(self.sorted).unwrap_or(rows.get().len());self.result=Some(if rank>rows.get().len(){RowAdmission::Missing}else{RowAdmission::Rank(rank)});},_=>{}}Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn begin_close(&mut self){self.closing=true;self.reader.begin_close();}
}
impl<T:EntityRow> CursorClose for RowAdmissionScan<T>{
 fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if !self.reader.terminal_is_empty(){return self.reader.close_demand(b);}RetainedCloneBinding::close_demand(&self.key,b)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.reader.terminal_is_empty(){return self.reader.close_step(g).map(|s|RetainedCloneStep::Progress(s.progress()));}let s=RetainedCloneBinding::close_one(&mut self.key,g)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})}
 fn terminal_is_empty(&self)->bool{self.closing&&self.reader.terminal_is_empty()&&self.key.is_none()}
}
impl<T:EntityRow> Drop for RowAdmissionScan<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original row admission retains identifier or page source");}}
