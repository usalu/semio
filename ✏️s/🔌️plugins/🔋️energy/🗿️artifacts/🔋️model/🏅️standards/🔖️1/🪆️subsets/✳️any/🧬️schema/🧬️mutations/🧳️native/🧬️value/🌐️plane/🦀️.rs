//! 🌐️ Fenestration validation inspects each original point and host edge before the shared owned-field producer runs.
use super::*;
use crate::snapshot::state::{StateFenestration,StateSurface};
/// 📐️ The authored geometry engine supplies its defining edge sum, normalization and plane tolerance.
pub trait OriginalFenestrationPlane:Send+Sync+'static{const TOLERANCE:f64;fn accumulate(sum:&mut[f64;3],a:[f64;3],b:[f64;3]);fn normal(sum:[f64;3])->[f64;3];}
pub struct FenestrationPlaneValidation<S:OriginalFenestrationPlane>{input:Bound,original:Bound,row:Bound,body:Option<usize>,lookup:Option<Box<RetainedEntityLookup<StateSurface>>>,host:Option<Box<RetainedOwnedProjection<StateSurface>>>,offset:usize,sum:[f64;3],normal:[f64;3],result:Option<Option<RowOutcome>>,phase:u8,closing:bool,flight:Flight,marker:std::marker::PhantomData<S>}
impl<S:OriginalFenestrationPlane> FenestrationPlaneValidation<S>{
 pub fn new()->Self{Self{input:Default::default(),original:Default::default(),row:Default::default(),body:None,lookup:None,host:None,offset:0,sum:[0.0;3],normal:[0.0;3],result:None,phase:0,closing:false,flight:Default::default(),marker:Default::default()}}
 fn check(&self,i:RetainedCloneRef<'_,Vec<[f64;3]>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,r:RetainedCloneRef<'_,StateFenestration>)->Result<(),ValueError>{i.check_binding(&self.input.0)?;o.check_binding(&self.original.0)?;r.check_binding(&self.row.0)?;Ok(())}
 fn finish(&mut self,result:Option<RowOutcome>){self.result=Some(result);self.phase=99;}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 custody_impl!(lookup,host,input,row,original);
}
crate::custody::std_cursor!([S:OriginalFenestrationPlane] FenestrationPlaneValidation<S>,[],|s|s.begin_close());
impl<S:OriginalFenestrationPlane> OwnedRowValidation<Vec<[f64;3]>,StateFenestration> for FenestrationPlaneValidation<S>{
 fn quote(&self,i:RetainedCloneRef<'_,Vec<[f64;3]>>,_:RetainedCloneRef<'_,u32>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,r:RetainedCloneRef<'_,StateFenestration>,body:usize,_:u64)->Result<Quote,ValueError>{if self.closing||self.result.is_some()||self.body.is_some_and(|v|v!=body)||!self.flight.idle(){return Err(invalid());}self.check(i,o,r)?;
  let lookup=||self.lookup.as_ref().ok_or_else(invalid);
  match self.phase{0=>Ok((nested(scalar(i.binding_copy_bytes()+size_of::<usize>()),1)?,i.binding_work_demand(&self.input.0)?)),1=>Ok((nested(scalar(r.binding_copy_bytes()),1)?,r.binding_work_demand(&self.row.0)?)),2=>Ok((nested(scalar(o.binding_copy_bytes()),1)?,o.binding_work_demand(&self.original.0)?)),3=>Ok((scalar(4*size_of::<usize>()+2),1)),4=>Ok((scalar(2*size_of::<[f64;3]>()+2*size_of::<usize>()+2),1)),5=>Ok((birth::<RetainedEntityLookup<StateSurface>>(),1)),
   6=>{let rows=o.project(0,|s|&s.model.geometry.get().surfaces);let key=r.project(0,|r|&r.surface_id.0);Ok((nested(lookup()?.demand(rows,key)?,1)?,lookup()?.advance_work_demand(rows,key)?))},
   7=>if lookup()?.result()==Some(None){Ok((scalar(2),1))}else{Ok((nested(lookup()?.take_demand()?,2*size_of::<Option<RetainedOwnedProjection<StateSurface>>>()+1)?,1))},
   8=>Ok((Cursor::demand(&self.lookup,body)?,Cursor::work(&self.lookup,body)?)),9=>Ok((scalar(6*size_of::<[f64;3]>()+4*size_of::<usize>()+1),1)),10=>Ok((scalar(4*size_of::<[f64;3]>()+2*size_of::<usize>()+1),1)),11=>Ok((scalar(6*size_of::<[f64;3]>()+2*size_of::<usize>()+2),1)),_=>Err(invalid())}}
 fn advance_reply<'r>(&mut self,i:RetainedCloneRef<'_,Vec<[f64;3]>>,key:RetainedCloneRef<'_,u32>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,r:RetainedCloneRef<'_,StateFenestration>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let quote=if recover{None}else{Some(self.quote(i,key,o,r,body,max))};let g=effect.grant();
  custody::gated(effect,reply,quote,max,|effect,work|{let mut p=receipt(scalar(1));self.body=Some(body);match self.phase{
   0=>{p=add(bind(i,&mut self.input,&mut effect.nested(child(g,1+size_of::<usize>())),work)?,1+size_of::<usize>())?;self.phase=1;},1=>{p=add(bind(r,&mut self.row,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=2;},2=>{p=add(bind(o,&mut self.original,&mut effect.nested(child(g,1)),work)?,1)?;self.phase=3;},
   3=>{if i.get().is_empty(){self.finish(None);}else if i.get().len()<3{self.finish(Some(RowOutcome::Invalid));}else{self.phase=4;}},4=>{if self.offset==i.get().len(){self.offset=0;self.phase=5;}else{let point=i.get()[self.offset];if !point[0].is_finite()||!point[1].is_finite()||!point[2].is_finite(){self.finish(Some(RowOutcome::Invalid));}else{self.offset+=1;}}},
   5=>{self.lookup=Some(Box::new(RetainedEntityLookup::new()));self.phase=6;p=receipt(birth::<RetainedEntityLookup<StateSurface>>());},
   6=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;p=add(lookup.advance(o.project(0,|s|&s.model.geometry.get().surfaces),r.project(0,|r|&r.surface_id.0),&mut effect.nested(child(g,1)))?.progress(),1)?;if lookup.result().is_some(){self.phase=7;}},
   7=>{let lookup=self.lookup.as_mut().ok_or_else(invalid)?;if lookup.result()==Some(None){self.finish(None);}else{let extra=2*size_of::<Option<RetainedOwnedProjection<StateSurface>>>()+1;let(host,paid)=lookup.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;self.host=Some(Box::new(host));self.phase=8;lookup.begin_close();p=add(paid,extra)?;}},
   8=>{let step=custody::finish(&mut self.flight,&mut self.lookup,body,effect,work,recover)?;p=step.progress();if self.lookup.is_none(){self.phase=9;}},
   9=>{let host=self.host.as_ref().ok_or_else(invalid)?.borrow()?;let points=host.get().vertices_m.get();if points.len()<3{self.finish(None);}else if self.offset==points.len(){self.phase=10;}else{S::accumulate(&mut self.sum,points[self.offset],points[(self.offset+1)%points.len()]);self.offset+=1;}},10=>{self.normal=S::normal(self.sum);self.offset=0;if self.normal[0].abs()+self.normal[1].abs()+self.normal[2].abs()<1e-12{self.finish(None);}else{self.phase=11;}},11=>{if self.offset==i.get().len(){self.finish(None);}else{let host=self.host.as_ref().ok_or_else(invalid)?.borrow()?;let anchor=host.get().vertices_m.get()[0];let v=i.get()[self.offset];let n=self.normal;let distance=((v[0]-anchor[0])*n[0]+(v[1]-anchor[1])*n[1]+(v[2]-anchor[2])*n[2]).abs();if distance<=S::TOLERANCE{self.offset+=1;}else{self.finish(Some(RowOutcome::InvalidContext));}}},
   _=>return Err(invalid().into())}
   Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 fn result(&self)->Option<Option<RowOutcome>>{self.result}
 fn begin_close(&mut self){Self::begin_close(self)}
}
