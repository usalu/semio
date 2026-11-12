//! 🧱️ Surface removal preserves the original geometry while its three genuine private collection roots are filtered.
use super::*;
use super::filter::CascadeFilter;
use crate::custody::Sourced;
use crate::snapshot::state::{ModelGeometryState,StateSurface,StateFenestration,StateAdjacencyPair,fields::RetainedBranchProjection};
use semio_framework_value::{shared_issuance::SealedShared,retained_clone::StagedSourceAdmission};
use semio_framework_value_persistent::list::persistent::PersistentList;
use protocol::RetainedFieldPreparation;
type Geometry=ModelGeometryState;type GeometryAlias=SealedShared<Geometry>;
/// 🧭️ Authored cascade paths install the genuine typed creation and polygon inverse constructors.
pub trait SurfaceDeletePath:Send+Sync+'static{
 type Payload:Send+Sync+'static;type Inverse:RetireOwned+Send+Sync+'static;
 type Surface:RowBirthSpec<Payload=StateSurface,Context=usize,Row=Self::Inverse>;
 type Fenestration:RowBirthSpec<Payload=StateFenestration,Context=usize,Row=Self::Inverse>;
 type Vertices:RowBirthSpec<Payload=StateFenestration,Context=usize,Row=Self::Inverse>;
 const CASCADE:protocol::RetainedDiagnosticSpec;
 fn key(p:RetainedCloneRef<'_,Self::Payload>)->RetainedCloneRef<'_,u32>;
 fn pair(row:&StateAdjacencyPair,rank:usize)->Result<Self::Inverse,ValueError>;
}
/// 🧳️ Original rows, immutable aliases, private filters and each typed inverse retain paid cancellation custody.
pub struct SurfaceDeletePreparation<S:SurfaceDeletePath>{body:Option<usize>,input:Bound,original:Bound,key:u32,phase:u8,closing:bool,outcome:Option<RowOutcome>,scan:usize,rank:Option<usize>,count:usize,
 lookup:Option<Box<RetainedEntityLookup<StateSurface>>>,row:Option<Box<RetainedOwnedProjection<StateSurface>>>,alias_stage:Option<Box<RetainedFieldPreparation<GeometryAlias>>>,alias:Held<GeometryAlias>,source:Sourced<GeometryAlias>,admit:Held<StagedSourceAdmission<GeometryAlias,GeometryAlias>>,
 surfaces:Option<Box<CascadeFilter<StateSurface>>>,windows:Option<Box<CascadeFilter<StateFenestration>>>,pairs:Option<Box<CascadeFilter<StateAdjacencyPair>>>,next_surfaces:Held<PersistentList<StateSurface>>,next_windows:Held<PersistentList<StateFenestration>>,next_pairs:Held<PersistentList<StateAdjacencyPair>>,old_surfaces:Held<PersistentList<StateSurface>>,old_windows:Held<PersistentList<StateFenestration>>,old_pairs:Held<PersistentList<StateAdjacencyPair>>,branch:Option<Box<RetainedBranchProjection<Geometry>>>,decision:Held<EnergyModelDiff>,
 surface_inverse:Option<Box<RowBirthPreparation<S::Surface,OwnedRecordForm>>>,window_inverse:Option<Box<RowBirthPreparation<S::Fenestration,OwnedRecordForm>>>,vertices_inverse:Option<Box<RowBirthPreparation<S::Vertices,OwnedRecordForm>>>,inverse:Held<S::Inverse>,inverse_mode:u8,inverse_rank:usize,inverse_phase:u8,flight:Flight}
impl<S:SurfaceDeletePath>SurfaceDeletePreparation<S>{
 fn geometry(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Geometry>{o.project(2,|o|o.model.geometry.get())}
 fn window_of(source:&Sourced<GeometryAlias>,rank:usize)->Result<RetainedCloneRef<'_,StateFenestration>,ValueError>{source.get().ok_or_else(invalid)?.borrow().try_project(rank,|g|g.get().fenestrations.get(rank).ok_or_else(invalid))}
 fn check_body(&self,b:usize)->Result<(),ValueError>{if self.body.is_some_and(|v|v!=b){Err(invalid())}else{Ok(())}}
 fn begin_close(&mut self){self.closing=true;self.mark_close();}
 semio_s_artifact_energy_model_native_mutations::custody_impl!(surface_inverse,window_inverse,vertices_inverse,branch,surfaces,windows,pairs,alias_stage,lookup,row,source,admit,inverse,decision,alias,next_surfaces,next_windows,next_pairs,old_surfaces,old_windows,old_pairs,original,input);
 fn admit_extra()->usize{2*size_of::<Option<GeometryAlias>>()+2*size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<GeometryAlias>>>()+2*size_of::<Option<StagedSourceAdmission<GeometryAlias,GeometryAlias>>>()+1}
 fn surface_quote(&self,p:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,max:u64)->Result<Quote,ValueError>{
  p.check_binding(&self.input.0)?;o.check_binding(&self.original.0)?;self.check_body(b)?;if self.closing||self.phase>=99||!self.flight.idle(){return Err(invalid())}let ge=Self::geometry(o);let nb=b.saturating_sub(1);
  match self.phase{
   0=>Ok((nested(scalar(p.binding_copy_bytes()),size_of::<usize>()+size_of::<u32>()+1)?,p.binding_work_demand(&self.input.0)?)),
   1=>Ok((nested(scalar(o.binding_copy_bytes()),1)?,o.binding_work_demand(&self.original.0)?)),
   2=>Ok((birth::<RetainedEntityLookup<StateSurface>>(),1)),
   3=>{let l=self.lookup.as_ref().ok_or_else(invalid)?;let rows=ge.project(0,|g|&g.surfaces);Ok((nested(l.demand(rows,S::key(p))?,1)?,l.advance_work_demand(rows,S::key(p))?))},
   4=>{let l=self.lookup.as_ref().ok_or_else(invalid)?;if l.result()==Some(None){Ok((scalar(2*size_of::<Option<RowOutcome>>()+1),1))}else{let mut d=nested(l.take_demand()?,retained_row_extra::<StateSurface>(2*size_of::<Option<usize>>()+1))?;d.capacity_bytes=d.capacity_bytes.checked_add(size_of::<RetainedOwnedProjection<StateSurface>>()).ok_or_else(invalid)?;Ok((d,1))}},
   5=>Ok((Cursor::demand(&self.lookup,b)?,Cursor::work(&self.lookup,b)?)),
   6=>Ok((scalar(size_of::<StateSurface>()+4*size_of::<usize>()+2*size_of::<Option<RowOutcome>>()+size_of::<u32>()+1),1)),
   7=>Ok((birth::<RetainedFieldPreparation<GeometryAlias>>(),1)),
   8=>{let s=self.alias_stage.as_ref().ok_or_else(invalid)?;let src=o.project(3,|o|&o.model.geometry);Ok((nested(s.demand(src,nb,max)?,1)?,s.advance_work_demand(src,nb,max)?))},
   9=>Ok((nested(self.alias_stage.as_ref().ok_or_else(invalid)?.exchange_demand()?,2*size_of::<Option<GeometryAlias>>()+1)?,1)),
   10=>Ok((Cursor::demand(&self.alias_stage,b)?,Cursor::work(&self.alias_stage,b)?)),
   11=>{let extra=Self::admit_extra();match self.admit.get(){Some(a)=>{let d=a.next_demand()?;Ok((RetirementDemand{copy_bytes:d.copy_bytes+extra,depth:d.depth+1,..d},a.next_work_demand()?))},None=>Ok((RetirementDemand{copy_bytes:2*size_of::<StagedSourceAdmission<GeometryAlias,GeometryAlias>>()+extra,depth:2,..Default::default()},1))}},
   12=>Ok((birth::<CascadeFilter<StateSurface>>(),1)),
   13=>{let(d,w)=self.surfaces.as_ref().ok_or_else(invalid)?.quote(ge.project(0,|g|&g.surfaces),nb,max)?;Ok((nested(d,1)?,w))},
   14=>{let(d,w)=self.surfaces.as_ref().ok_or_else(invalid)?.take_quote(nb)?;Ok((nested(d,2*size_of::<Option<PersistentList<StateSurface>>>()+1)?,w))},
   15=>Ok((Cursor::demand(&self.surfaces,b)?,Cursor::work(&self.surfaces,b)?)),
   16=>Ok((birth::<CascadeFilter<StateFenestration>>(),1)),
   17=>{let(d,w)=self.windows.as_ref().ok_or_else(invalid)?.quote(ge.project(1,|g|&g.fenestrations),nb,max)?;Ok((nested(d,1)?,w))},
   18=>{let(d,w)=self.windows.as_ref().ok_or_else(invalid)?.take_quote(nb)?;Ok((nested(d,2*size_of::<Option<PersistentList<StateFenestration>>>()+2*size_of::<usize>()+1)?,w))},
   19=>Ok((Cursor::demand(&self.windows,b)?,Cursor::work(&self.windows,b)?)),
   20=>Ok((birth::<CascadeFilter<StateAdjacencyPair>>(),1)),
   21=>{let(d,w)=self.pairs.as_ref().ok_or_else(invalid)?.quote(ge.project(2,|g|&g.adjacency_pairs),nb,max)?;Ok((nested(d,1)?,w))},
   22=>{let(d,w)=self.pairs.as_ref().ok_or_else(invalid)?.take_quote(nb)?;Ok((nested(d,2*size_of::<Option<PersistentList<StateAdjacencyPair>>>()+3*size_of::<usize>()+1)?,w))},
   23=>Ok((Cursor::demand(&self.pairs,b)?,Cursor::work(&self.pairs,b)?)),
   24=>Ok((birth::<RetainedBranchProjection<Geometry>>(),1)),
   25|30=>{let c=self.branch.as_ref().ok_or_else(invalid)?;Ok((nested(c.demand(ge,nb,max)?,1)?,c.advance_work_demand(ge,nb,max)?))},
   26=>Ok((nested(RetainedBranchProjection::<Geometry>::exchange_demand::<PersistentList<StateSurface>>(),1)?,1)),
   27=>Ok((nested(RetainedBranchProjection::<Geometry>::exchange_demand::<PersistentList<StateFenestration>>(),1)?,1)),
   28=>Ok((nested(RetainedBranchProjection::<Geometry>::exchange_demand::<PersistentList<StateAdjacencyPair>>(),1)?,1)),
   29=>Ok((scalar(1),1)),
   31=>Ok((nested(self.branch.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<EnergyModelDiff>()+2*size_of::<Option<EnergyModelDiff>>()+1)?,1)),
   32=>Ok((Cursor::demand(&self.branch,b)?,Cursor::work(&self.branch,b)?)),
   _=>Err(invalid()),
  }
 }
 fn surface_advance<'r>(&mut self,payload:RetainedCloneRef<'_,S::Payload>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
  let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
   let mut units=0u64;
   if !recover{
    if !self.flight.idle(){return Err(invalid().into())}
    let(d,u)=self.surface_quote(payload,o,b,max)?;
    if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
    units=u;
   }
   let mut p=RetainedCloneProgress::default();let ge=Self::geometry(o);let nb=b.saturating_sub(1);
   macro_rules! routed{($c:expr,$($args:expr),*)=>{protocol::retained_routed!(self.flight,effect,recover,1,b,max,None,work,|r| $c.advance_reply($($args,)*nb,max,&mut effect.nested(child(g,1)),r,false),$c.advance_reply($($args,)*nb,max,&mut effect.nested(child(g,1)),r,true))}}
   match self.phase{
    0=>{let extra=size_of::<usize>()+size_of::<u32>()+1;p=add(bind(payload,&mut self.input,&mut effect.nested(child(g,extra)),&mut work)?,extra)?;self.body=Some(b);self.key=*S::key(payload).get();self.phase=1;},
    1=>{p=add(bind(o,&mut self.original,&mut effect.nested(child(g,1)),&mut work)?,1)?;self.phase=2;},
    2=>{self.lookup=Some(Box::new(RetainedEntityLookup::new()));self.phase=3;p=receipt(birth::<RetainedEntityLookup<StateSurface>>());},
    3=>{let c=self.lookup.as_mut().ok_or_else(invalid)?;p=add(c.advance(ge.project(0,|g|&g.surfaces),S::key(payload),&mut effect.nested(child(g,1)))?.progress(),1)?;if c.result().is_some(){self.phase=4;}},
    4=>{let c=self.lookup.as_mut().ok_or_else(invalid)?;if c.result()==Some(None){self.outcome=Some(RowOutcome::Missing);self.phase=99;p=receipt(scalar(2));}else{
     let extra=retained_row_extra::<StateSurface>(2*size_of::<Option<usize>>()+1);let cap=size_of::<RetainedOwnedProjection<StateSurface>>();
     let(row,paid)=c.take(&mut effect.nested(RetainedCloneGrant{maximum_capacity_bytes:g.maximum_capacity_bytes.saturating_sub(cap),..child(g,extra)}))?.ok_or_else(invalid)?;
     self.row=Some(Box::new(row));self.rank=c.result().ok_or_else(invalid)?;c.begin_close();self.phase=5;p=add(paid,extra)?;p.retained_capacity_bytes=p.retained_capacity_bytes.checked_add(cap).ok_or_else(invalid)?;}},
    5=>{let step=custody::finish(&mut self.flight,&mut self.lookup,b,effect,&mut work,recover)?;p=step.progress();if self.lookup.is_none(){self.phase=6;}},
    6=>{p=receipt(scalar(1));let rows=&ge.get().surfaces;if self.scan==rows.len(){self.phase=7;}else{let row=rows.get(self.scan).ok_or_else(invalid)?;if row.outside_boundary_condition.get().interzone_partner().is_some_and(|id|id.0==self.key){self.outcome=Some(RowOutcome::Referenced);self.phase=99;}else{self.scan+=1;}}},
    7=>{self.alias_stage=Some(Box::default());self.phase=8;p=receipt(birth::<RetainedFieldPreparation<GeometryAlias>>());},
    8=>{let src=o.project(3,|o|&o.model.geometry);let c=self.alias_stage.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,b,max,None,work,|r| c.advance_into(src,nb,max,&mut effect.nested(child(g,1)),r),c.recover_advance_into(src,nb,max,&mut effect.nested(child(g,1)),r));p=add(step?.progress(),1)?;if c.value().is_some(){self.phase=9;}},
    9=>{let extra=2*size_of::<Option<GeometryAlias>>()+1;let Self{alias_stage,alias,..}=self;let c=alias_stage.as_deref_mut().ok_or_else(invalid)?;let slot=alias.slot();let step=native_child(effect,&mut work,|e,r|c.take_into(slot,&mut e.nested(child(g,extra)),r))??;p=add(step.progress(),extra)?;self.phase=10;},
    10=>{let step=custody::finish(&mut self.flight,&mut self.alias_stage,b,effect,&mut work,recover)?;p=step.progress();if self.alias_stage.is_none(){self.phase=11;}},
    11=>{let extra=Self::admit_extra();if self.admit.is_none()&&self.source.get().is_none(){let value=self.alias.take().ok_or_else(invalid)?;self.admit.set(StagedSourceAdmission::owned(value));p=receipt(scalar(extra));}else{let a=self.admit.get_mut().ok_or_else(invalid)?;let turn=if a.is_projecting(){a.project_owned(effect)}else{a.advance(self.source.slot(),effect)};work+=turn.work;p=add(turn.result?.progress(),extra)?;if a.terminal_is_empty(){drop(self.admit.take());}}if self.source.get().is_some(){self.phase=12;}},
    12=>{self.surfaces=Some(Box::new(CascadeFilter::new(|r:&StateSurface,k|usize::from(r.id.0==k))));self.phase=13;p=receipt(birth::<CascadeFilter<StateSurface>>());},
    13=>{let key=self.key;let c=self.surfaces.as_mut().ok_or_else(invalid)?;let src=ge.project(0,|g|&g.surfaces);let step=protocol::retained_routed!(self.flight,effect,recover,1,b,max,None,work,|r| c.advance_reply(src,key,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(src,key,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if c.ready(){self.phase=14;}},
    14=>{let extra=2*size_of::<Option<PersistentList<StateSurface>>>()+1;let c=self.surfaces.as_mut().ok_or_else(invalid)?;let(v,paid)=c.take(nb,effect,&mut work)?;p=add(paid,extra)?;if let Some(v)=v{self.next_surfaces.set(v);c.begin_close();self.phase=15;}},
    15=>{let step=custody::finish(&mut self.flight,&mut self.surfaces,b,effect,&mut work,recover)?;p=step.progress();if self.surfaces.is_none(){self.phase=16;}},
    16=>{self.windows=Some(Box::new(CascadeFilter::new(|r:&StateFenestration,k|if r.surface_id.0==k{1+usize::from(!r.vertices_m.get().is_empty())}else{0})));self.phase=17;p=receipt(birth::<CascadeFilter<StateFenestration>>());},
    17=>{let key=self.key;let c=self.windows.as_mut().ok_or_else(invalid)?;let src=ge.project(1,|g|&g.fenestrations);let step=protocol::retained_routed!(self.flight,effect,recover,1,b,max,None,work,|r| c.advance_reply(src,key,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(src,key,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if c.ready(){self.phase=18;}},
    18=>{let extra=2*size_of::<Option<PersistentList<StateFenestration>>>()+2*size_of::<usize>()+1;let c=self.windows.as_mut().ok_or_else(invalid)?;let(v,paid)=c.take(nb,effect,&mut work)?;p=add(paid,extra)?;if let Some(v)=v{self.count=1+c.weight();self.next_windows.set(v);c.begin_close();self.phase=19;}},
    19=>{let step=custody::finish(&mut self.flight,&mut self.windows,b,effect,&mut work,recover)?;p=step.progress();if self.windows.is_none(){self.phase=20;}},
    20=>{self.pairs=Some(Box::new(CascadeFilter::new(|r:&StateAdjacencyPair,k|usize::from(r.surface_a_id.0==k||r.surface_b_id.0==k))));self.phase=21;p=receipt(birth::<CascadeFilter<StateAdjacencyPair>>());},
    21=>{let key=self.key;let c=self.pairs.as_mut().ok_or_else(invalid)?;let src=ge.project(2,|g|&g.adjacency_pairs);let step=protocol::retained_routed!(self.flight,effect,recover,1,b,max,None,work,|r| c.advance_reply(src,key,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(src,key,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if c.ready(){self.phase=22;}},
    22=>{let extra=2*size_of::<Option<PersistentList<StateAdjacencyPair>>>()+3*size_of::<usize>()+1;let c=self.pairs.as_mut().ok_or_else(invalid)?;let(v,paid)=c.take(nb,effect,&mut work)?;p=add(paid,extra)?;if let Some(v)=v{self.count=self.count.checked_add(c.weight()).ok_or_else(invalid)?;self.inverse_rank=ge.get().adjacency_pairs.len();self.next_pairs.set(v);c.begin_close();self.phase=23;}},
    23=>{let step=custody::finish(&mut self.flight,&mut self.pairs,b,effect,&mut work,recover)?;p=step.progress();if self.pairs.is_none(){self.phase=24;}},
    24=>{self.branch=Some(Box::new(RetainedBranchProjection::new()));self.phase=25;p=receipt(birth::<RetainedBranchProjection<Geometry>>());},
    25|30=>{let c=self.branch.as_mut().ok_or_else(invalid)?;let step=protocol::retained_routed!(self.flight,effect,recover,1,b,max,None,work,|r| c.advance_reply(ge,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(ge,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if self.phase==25&&c.raw_ready(){self.phase=26;}else if self.phase==30&&c.ready(){self.phase=31;}},
    26=>{let Self{branch,next_surfaces,old_surfaces,..}=self;let c=branch.as_mut().ok_or_else(invalid)?;p=add(c.exchange(|b|&mut b.surfaces,next_surfaces.slot(),old_surfaces.slot(),&mut effect.nested(child(g,1)))?.1,1)?;self.phase=27;},
    27=>{let Self{branch,next_windows,old_windows,..}=self;let c=branch.as_mut().ok_or_else(invalid)?;p=add(c.exchange(|b|&mut b.fenestrations,next_windows.slot(),old_windows.slot(),&mut effect.nested(child(g,1)))?.1,1)?;self.phase=28;},
    28=>{let Self{branch,next_pairs,old_pairs,..}=self;let c=branch.as_mut().ok_or_else(invalid)?;p=add(c.exchange(|b|&mut b.adjacency_pairs,next_pairs.slot(),old_pairs.slot(),&mut effect.nested(child(g,1)))?.1,1)?;self.phase=29;},
    29=>{self.branch.as_mut().ok_or_else(invalid)?.begin_seal()?;self.phase=30;p=receipt(scalar(1));},
    31=>{let extra=2*size_of::<EnergyModelDiff>()+2*size_of::<Option<EnergyModelDiff>>()+1;let c=self.branch.as_mut().ok_or_else(invalid)?;let(v,paid)=c.take(&mut effect.nested(child(g,extra)))?.ok_or_else(invalid)?;let mut diff=EnergyModelDiff::default();diff.model.geometry=Some(v);self.decision.set(diff);c.begin_close();self.phase=32;p=add(paid,extra)?;},
    32=>{let step=custody::finish(&mut self.flight,&mut self.branch,b,effect,&mut work,recover)?;p=step.progress();if self.branch.is_none(){self.outcome=Some(RowOutcome::Changed);self.phase=99;}},
    _=>return Err(invalid().into()),
   }
   admit_retained_clone_progress(g,p,"original Energy surface cascade")?;
   work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
   Ok(if self.phase==99{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn inverse_phase_quote(&self,b:usize,max:u64)->Result<Quote,ValueError>{
  self.check_body(b)?;if self.closing||self.phase!=100||self.inverse.is_some()||!self.flight.idle(){return Err(invalid())}let nb=b.saturating_sub(1);let extra=2*size_of::<Option<S::Inverse>>()+4*size_of::<usize>()+2;
  match self.inverse_phase{
   0=>Ok((scalar(size_of::<StateAdjacencyPair>()+size_of::<StateFenestration>()+2*size_of::<S::Inverse>()+extra),1)),
   1=>match self.inverse_mode{1=>Ok((birth::<RowBirthPreparation<S::Vertices,OwnedRecordForm>>(),1)),2=>Ok((birth::<RowBirthPreparation<S::Fenestration,OwnedRecordForm>>(),1)),3=>Ok((birth::<RowBirthPreparation<S::Surface,OwnedRecordForm>>(),1)),_=>Err(invalid())},
   2=>match self.inverse_mode{
    1=>{let(d,w)=self.vertices_inverse.as_ref().ok_or_else(invalid)?.quote(Self::window_of(&self.source,self.inverse_rank)?,nb,max)?;Ok((nested(d,1)?,w))},
    2=>{let(d,w)=self.window_inverse.as_ref().ok_or_else(invalid)?.quote(Self::window_of(&self.source,self.inverse_rank)?,nb,max)?;Ok((nested(d,1)?,w))},
    3=>{let(d,w)=self.surface_inverse.as_ref().ok_or_else(invalid)?.quote(self.row.as_ref().ok_or_else(invalid)?.borrow()?,nb,max)?;Ok((nested(d,1)?,w))},
    _=>Err(invalid())},
   3=>{let(d,w)=match self.inverse_mode{1=>self.vertices_inverse.as_ref().ok_or_else(invalid)?.take_quote()?,2=>self.window_inverse.as_ref().ok_or_else(invalid)?.take_quote()?,3=>self.surface_inverse.as_ref().ok_or_else(invalid)?.take_quote()?,_=>return Err(invalid())};Ok((nested(d,extra)?,w))},
   4=>match self.inverse_mode{1=>Ok((Cursor::demand(&self.vertices_inverse,b)?,Cursor::work(&self.vertices_inverse,b)?)),2=>Ok((Cursor::demand(&self.window_inverse,b)?,Cursor::work(&self.window_inverse,b)?)),3=>Ok((Cursor::demand(&self.surface_inverse,b)?,Cursor::work(&self.surface_inverse,b)?)),_=>Err(invalid())},
   _=>Err(invalid()),
  }
 }
 fn inverse_advance<'r>(&mut self,b:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let q=if recover{None}else{Some(self.inverse_phase_quote(b,max))};let g=effect.grant();let nb=b.saturating_sub(1);let extra=2*size_of::<Option<S::Inverse>>()+4*size_of::<usize>()+2;
  custody::gated(effect,reply,q,max,|effect,work|{let mut p=receipt(scalar(1));
   match self.inverse_phase{
    0=>{let ge=self.source.get().ok_or_else(invalid)?.borrow().project(0,|s|s.get());match self.inverse_mode{
      0=>{if self.inverse_rank==0{self.inverse_mode=1;self.inverse_rank=ge.get().fenestrations.len();}else{self.inverse_rank-=1;let row=ge.get().adjacency_pairs.get(self.inverse_rank).ok_or_else(invalid)?;if row.surface_a_id.0==self.key||row.surface_b_id.0==self.key{self.inverse.set(S::pair(row,self.inverse_rank)?);}}},
      1=>{if self.inverse_rank==0{self.inverse_mode=3;self.inverse_phase=1;}else{self.inverse_rank-=1;let row=ge.get().fenestrations.get(self.inverse_rank).ok_or_else(invalid)?;if row.surface_id.0==self.key{self.inverse_mode=if row.vertices_m.get().is_empty(){2}else{1};self.inverse_phase=1;}}},
      3=>self.inverse_phase=1,
      _=>return Err(invalid().into())}},
    1=>{match self.inverse_mode{1=>self.vertices_inverse=Some(Box::new(RowBirthPreparation::with_context(self.inverse_rank))),2=>self.window_inverse=Some(Box::new(RowBirthPreparation::with_context(self.inverse_rank))),3=>self.surface_inverse=Some(Box::new(RowBirthPreparation::with_context(self.rank.ok_or_else(invalid)?))),_=>return Err(invalid().into())}self.inverse_phase=2;p=receipt(scalar(1));},
    2=>{let rank=self.inverse_rank;let Self{source,row,vertices_inverse,window_inverse,surface_inverse,flight,inverse_mode,inverse_phase,..}=self;
     macro_rules! drive{($c:expr,$src:expr)=>{{let c=$c.as_deref_mut().ok_or_else(invalid)?;let s=$src;let step=protocol::retained_routed!(*flight,effect,recover,1,b,max,None,*work,|r| c.advance_reply(s,nb,max,&mut effect.nested(child(g,1)),r,false),c.advance_reply(s,nb,max,&mut effect.nested(child(g,1)),r,true));p=add(step?.progress(),1)?;if c.ready(){*inverse_phase=3;}}}}
     match *inverse_mode{1=>drive!(vertices_inverse,Self::window_of(source,rank)?),2=>drive!(window_inverse,Self::window_of(source,rank)?),3=>drive!(surface_inverse,row.as_ref().ok_or_else(invalid)?.borrow()?),_=>return Err(invalid().into())}},
    3=>{let (value,paid)=match self.inverse_mode{1=>self.vertices_inverse.as_mut().ok_or_else(invalid)?.take(&mut effect.nested(child(g,extra)))?,2=>self.window_inverse.as_mut().ok_or_else(invalid)?.take(&mut effect.nested(child(g,extra)))?,3=>self.surface_inverse.as_mut().ok_or_else(invalid)?.take(&mut effect.nested(child(g,extra)))?,_=>return Err(invalid().into())}.ok_or_else(invalid)?;self.inverse.set(value);match self.inverse_mode{1=>self.vertices_inverse.as_mut().ok_or_else(invalid)?.begin_close(),2=>self.window_inverse.as_mut().ok_or_else(invalid)?.begin_close(),3=>self.surface_inverse.as_mut().ok_or_else(invalid)?.begin_close(),_=>{}}self.inverse_phase=4;p=add(paid,extra)?;},
    4=>{let step=match self.inverse_mode{1=>custody::finish(&mut self.flight,&mut self.vertices_inverse,b,effect,work,recover)?,2=>custody::finish(&mut self.flight,&mut self.window_inverse,b,effect,work,recover)?,3=>custody::finish(&mut self.flight,&mut self.surface_inverse,b,effect,work,recover)?,_=>return Err(invalid().into())};p=step.progress();
     let empty=match self.inverse_mode{1=>self.vertices_inverse.is_none(),2=>self.window_inverse.is_none(),3=>self.surface_inverse.is_none(),_=>false};
     if empty{match self.inverse_mode{1=>{self.inverse_mode=2;self.inverse_phase=1;},2=>{self.inverse_mode=1;self.inverse_phase=0;},3=>{self.inverse_mode=4;self.inverse_phase=5;},_=>return Err(invalid().into())}}},
    _=>return Err(invalid().into()),
   }
   Ok(if self.inverse_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})
 }
 fn inverse_ready(&self)->bool{!self.closing&&self.inverse.is_some()}
}
semio_s_artifact_energy_model_native_mutations::std_cursor!([S:SurfaceDeletePath] SurfaceDeletePreparation<S>,[],|s|s.begin_close());
impl<S:SurfaceDeletePath>RowFieldPreparation for SurfaceDeletePreparation<S>{
 type Input=S::Payload;type Inverse=S::Inverse;
 fn new()->Self{Self{body:None,input:Default::default(),original:Default::default(),key:0,phase:0,closing:false,outcome:None,scan:0,rank:None,count:0,lookup:None,row:None,alias_stage:None,alias:Default::default(),source:Default::default(),admit:Default::default(),surfaces:None,windows:None,pairs:None,next_surfaces:Default::default(),next_windows:Default::default(),next_pairs:Default::default(),old_surfaces:Default::default(),old_windows:Default::default(),old_pairs:Default::default(),branch:None,decision:Default::default(),surface_inverse:None,window_inverse:None,vertices_inverse:None,inverse:Default::default(),inverse_mode:0,inverse_rank:0,inverse_phase:0,flight:Default::default()}}
 fn check(&self,p:RetainedCloneRef<'_,S::Payload>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{p.check_binding(&self.input.0)?;o.check_binding(&self.original.0)?;Ok(())}
 fn quote(&self,p:RetainedCloneRef<'_,S::Payload>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64)->Result<Quote,ValueError>{self.surface_quote(p,o,b,m)}
 fn advance_reply<'r>(&mut self,p:RetainedCloneRef<'_,S::Payload>,_:Option<RetainedCloneRef<'_,u32>>,o:RetainedCloneRef<'_,EnergyModelSnapshot>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.surface_advance(p,o,b,m,e,r,recover)}
 fn ready(&self)->bool{!self.closing&&self.phase==99}
 fn outcome(&self)->Option<RowOutcome>{self.outcome}
 fn changed_diagnostic(&self)->Option<protocol::RetainedDiagnosticSpec>{(self.count>1).then_some(S::CASCADE)}
 fn take_quote(&self)->Result<Quote,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid())}Ok((scalar(2*size_of::<Option<EnergyModelDiff>>()+1),1))}
 fn take(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_quote()?;if effect.remaining_work_units()?<1||!retained_mutation_demand_fits(d,effect.grant()){return Ok(None)}if !effect.performed(1)?{return Err(invalid())}self.phase=100;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 fn inverse_count(&self)->Result<usize,ValueError>{if self.phase!=100||self.count==0{return Err(invalid())}Ok(self.count)}
 fn inverse_quote(&self,b:usize,m:u64)->Result<Quote,ValueError>{self.inverse_phase_quote(b,m)}
 fn prepare_inverse_reply<'r>(&mut self,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.inverse_advance(b,m,e,r,recover)}
 fn inverse_ready(&self)->bool{Self::inverse_ready(self)}
 fn take_inverse_quote(&self)->Result<Quote,ValueError>{if !Self::inverse_ready(self){return Err(invalid())}Ok((scalar(2*size_of::<Option<S::Inverse>>()+2),1))}
 fn take_inverse(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(S::Inverse,RetainedCloneProgress)>,ValueError>{let(d,_)=self.take_inverse_quote()?;if !retained_mutation_demand_fits(d,effect.grant()){return Ok(None)}Ok(Some((self.inverse.take().ok_or_else(invalid)?,receipt(d))))}
}
