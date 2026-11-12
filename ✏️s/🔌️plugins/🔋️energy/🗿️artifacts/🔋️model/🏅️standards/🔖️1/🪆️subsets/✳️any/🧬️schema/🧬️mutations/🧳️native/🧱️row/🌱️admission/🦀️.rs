//! 🌱️ Row creation chooses its authored rank while checking actual original identifiers.
use super::*;
use crate::snapshot::state::fields::EntityRow;
use semio_framework_value_persistent::list::persistent::{PersistentListRead,PersistentListReadStep};
/// 🧭️ Duplicate identities and out of range authored ranks preserve distinct semantic diagnostics.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum RowAdmission{Rank(usize),Duplicate,Missing}
/// 🔎️ The original page cursor inspects one genuine row at a time without collecting a model.
pub struct RowAdmissionScan<T:EntityRow>{reader:PersistentListRead<T>,key:Bound,index:Option<usize>,sorted:Option<usize>,result:Option<RowAdmission>,closing:bool,flight:Flight}
impl<T:EntityRow> RowAdmissionScan<T>{
 pub fn new(index:Option<usize>)->Self{Self{reader:PersistentListRead::new(),key:Default::default(),index,sorted:None,result:None,closing:false,flight:Default::default()}}
 pub fn result(&self)->Option<RowAdmission>{self.result}
 fn fixed()->usize{2*size_of::<Option<usize>>()+2*size_of::<Option<RowAdmission>>()+2*size_of::<usize>()+size_of::<u32>()}
 pub fn quote(&self,rows:RetainedCloneRef<'_,PersistentList<T>>,key:RetainedCloneRef<'_,u32>)->Result<Quote,ValueError>{if self.closing||self.result.is_some(){return Err(invalid());}key.check_binding(&self.key.0)?;if self.key.0.is_none(){return Ok((nested(scalar(key.binding_copy_bytes()),1)?,key.binding_work_demand(&self.key.0)?));}Ok((nested(self.reader.demand(rows)?,Self::fixed())?,self.reader.advance_work_demand(rows)?))}
 pub fn advance(&mut self,rows:RetainedCloneRef<'_,PersistentList<T>>,key:RetainedCloneRef<'_,u32>,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<RetainedCloneStep,ValueError>{
  let(d,u)=self.quote(rows,key)?;if effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,effect.grant()){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.key.0.is_none(){let p=bind(key,&mut self.key,effect,work)?;return Ok(RetainedCloneStep::Progress(add(p,1)?));}
  let extra=Self::fixed();let mut row=None;let reader=&mut self.reader;
  let s=native_child(effect,work,|e,r|reader.advance_into(rows,&mut row,e,r))??;let p=add(s.progress(),extra)?;
  if let Some((rank,projected))=row{let id=projected.get().entity_id();if id==*key.get(){self.result=Some(RowAdmission::Duplicate);}else if self.sorted.is_none()&&id>*key.get(){self.sorted=Some(rank);}}
  else if matches!(s,RetainedCloneStep::Complete(_)){let rank=self.index.or(self.sorted).unwrap_or(rows.get().len());self.result=Some(if rank>rows.get().len(){RowAdmission::Missing}else{RowAdmission::Rank(rank)});}
  Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(reader,key);
}
crate::custody::std_cursor!([T:EntityRow] RowAdmissionScan<T>,[],|s|s.begin_close());
