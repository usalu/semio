//! 📏️ Project-explicit length unit resolution over a borrowed original graph.
use super::graph::{GeometryEntity,GeometryGraph,GeometryStandard,GeometryValue};
use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum LengthUnitResult{Pending,Resolved{project:u64,unit:u64,metres_per_unit:f64},Invalid{entity:u64,reason:&'static str},Limited{entity:u64,reason:&'static str},Cancelled}
#[derive(Clone,Copy,Debug)]
pub struct LengthUnitLimits{pub entities:usize,pub references:usize,pub depth:usize}
impl Default for LengthUnitLimits{fn default()->Self{Self{entities:200_000,references:200_000,depth:64}}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum LengthUnitControl{Continue,Yield,Cancel}
#[derive(Clone,Copy)]
enum Phase{Index,Project,Assignment,Units,Resolve}
/// 🧭️ Retain only IDs and ordinals; the caller holds its captured snapshot through controlled close.
pub struct LengthUnitWork{project:u64,source_version:u64,limits:LengthUnitLimits,index:BTreeMap<u64,usize>,visited:BTreeSet<u64>,assigned_units:BTreeSet<u64>,cursor:usize,assignment:u64,assigned:usize,unit:u64,selected:u64,scale:f64,phase:Phase,source_identity:Option<usize>,source_length:Option<usize>,result:LengthUnitResult,closed:bool,pub work:usize}
impl LengthUnitWork{
 pub fn new(project:u64,source_version:u64,limits:LengthUnitLimits)->Self{let mut result=Self{project,source_version,limits,index:BTreeMap::new(),visited:BTreeSet::new(),assigned_units:BTreeSet::new(),cursor:0,assignment:0,assigned:0,unit:0,selected:0,scale:1.,phase:Phase::Index,source_identity:None,source_length:None,result:LengthUnitResult::Pending,closed:false,work:0};if project==0||limits.entities==0||limits.references==0||limits.depth==0{result.fail(project,"invalid-length-unit-request");}result}
 fn fail(&mut self,entity:u64,reason:&'static str){self.result=LengthUnitResult::Invalid{entity,reason};}
 fn limited(&mut self,entity:u64,reason:&'static str){self.result=LengthUnitResult::Limited{entity,reason};}
 fn entity<'a,G:GeometryGraph>(&self,graph:&'a G,id:u64)->Option<GeometryEntity<'a,G::Value>>{graph.entity(*self.index.get(&id)?)}
 pub fn advance<G:GeometryGraph>(&mut self,graph:&G,source_version:u64,maximum_work:usize,control:&mut dyn FnMut(usize)->LengthUnitControl)->LengthUnitResult{
  if self.closed{return LengthUnitResult::Cancelled}
  if self.source_identity.is_some_and(|identity|identity!=graph.source_identity())||source_version!=self.source_version||self.source_length.is_some_and(|length|length!=graph.len()){self.fail(self.project,"source-version-changed");return self.result}
  if self.result!=LengthUnitResult::Pending{return self.result}
  if maximum_work==0{if control(self.work)==LengthUnitControl::Cancel{self.result=LengthUnitResult::Cancelled;}return self.result}
  if graph.len()>self.limits.entities{self.limited(self.project,"entity-limit");return self.result}self.source_identity=Some(graph.source_identity());self.source_length=Some(graph.len());
  for _ in 0..maximum_work{if self.result!=LengthUnitResult::Pending{break}match control(self.work){LengthUnitControl::Cancel=>{self.result=LengthUnitResult::Cancelled;break},LengthUnitControl::Yield=>break,LengthUnitControl::Continue=>{}}self.work=self.work.saturating_add(1);self.step(graph);}
  self.result
 }
 fn step<G:GeometryGraph>(&mut self,graph:&G){
  match self.phase{
   Phase::Index=>{if self.cursor==graph.len(){self.phase=Phase::Project;return}let Some(entity)=graph.entity(self.cursor)else{self.fail(self.project,"invalid-instance");return};if entity.id==0{self.fail(self.project,"invalid-instance");return}if self.index.insert(entity.id,self.cursor).is_some(){self.fail(entity.id,"duplicate-instance");return}self.cursor+=1;return},
   Phase::Project=>{let Some(entity)=self.entity(graph,self.project).filter(|entity|entity.name=="IFCPROJECT"&&entity.constituent_count==1)else{self.fail(self.project,"missing-project");return};let Some(assignment)=entity.args.get(8).and_then(GeometryValue::as_reference)else{self.fail(self.project,"missing-unit-assignment");return};self.assignment=assignment;self.phase=Phase::Assignment;return},
   Phase::Assignment=>{let Some(entity)=self.entity(graph,self.assignment).filter(|entity|entity.name=="IFCUNITASSIGNMENT"&&entity.constituent_count==1)else{self.fail(self.assignment,"invalid-unit-assignment");return};let Some(units)=entity.args.first().and_then(GeometryValue::as_aggregate)else{self.fail(self.assignment,"invalid-unit-assignment");return};if units.len()>self.limits.references{self.limited(self.assignment,"reference-limit");return}self.phase=Phase::Units;return},
   Phase::Units=>{let Some(units)=self.entity(graph,self.assignment).and_then(|entity|entity.args.first()).and_then(GeometryValue::as_aggregate)else{self.fail(self.assignment,"source-version-changed");return};if self.assigned==units.len(){if self.selected==0{self.fail(self.assignment,"missing-length-unit");return}self.unit=self.selected;self.phase=Phase::Resolve;return}let Some(id)=units[self.assigned].as_reference()else{self.fail(self.assignment,"invalid-assigned-unit");return};self.assigned+=1;if !self.assigned_units.insert(id){self.fail(id,"duplicate-assigned-unit");return}let Some(entity)=self.entity(graph,id)else{self.fail(id,"missing-assigned-unit");return};if entity.constituent_count!=1{self.fail(id,"unsupported-complex-unit");return}if entity.name=="IFCMONETARYUNIT"{return}if !matches!(entity.name,"IFCSIUNIT"|"IFCCONVERSIONBASEDUNIT"|"IFCCONVERSIONBASEDUNITWITHOFFSET"|"IFCCONTEXTDEPENDENTUNIT"|"IFCDERIVEDUNIT"){self.fail(id,"invalid-assigned-unit");return}if entity.args.get(1).and_then(GeometryValue::as_enum)!=Some("LENGTHUNIT"){return}if self.selected!=0{self.fail(id,"ambiguous-length-unit");return}self.selected=id;return},
   Phase::Resolve=>{}
  }
  let id=self.unit;let Some(entity)=self.entity(graph,id).filter(|entity|entity.constituent_count==1)else{self.fail(id,"invalid-length-unit");return};
  if self.visited.contains(&id){self.fail(id,"cyclic-length-unit");return}if self.visited.len()>=self.limits.depth{self.limited(id,"depth-limit");return}self.visited.insert(id);
  if entity.args.get(1).and_then(GeometryValue::as_enum)!=Some("LENGTHUNIT"){self.fail(id,"invalid-length-unit-type");return}
  if entity.name=="IFCSIUNIT"{
   if !entity.args.first().is_some_and(GeometryValue::is_derived){self.fail(id,"invalid-si-dimensions");return}if entity.args.get(3).and_then(GeometryValue::as_enum)!=Some("METRE"){self.fail(id,"invalid-si-length-name");return}
   let power=match entity.args.get(2){Some(value)if value.is_unset()=>Some(0),Some(value)=>value.as_enum().and_then(prefix_power),None=>None};let Some(power)=power else{self.fail(id,"invalid-si-prefix");return};let scale=self.scale*10_f64.powi(power);if !scale.is_finite()||scale<=0.{self.fail(id,"unrepresentable-length-scale");return}self.result=LengthUnitResult::Resolved{project:self.project,unit:self.selected,metres_per_unit:scale};return
  }
  if !matches!(entity.name,"IFCCONVERSIONBASEDUNIT"|"IFCCONVERSIONBASEDUNITWITHOFFSET"){self.fail(id,"unsupported-length-unit");return}
  if entity.name=="IFCCONVERSIONBASEDUNITWITHOFFSET"&&(graph.standard()==GeometryStandard::Ifc2x3||entity.args.get(4).and_then(GeometryValue::as_real)!=Some(0.)){self.fail(id,"nonlinear-length-unit");return}
  let dimensions=entity.args.first().and_then(GeometryValue::as_reference).and_then(|id|self.entity(graph,id));if !dimensions.is_some_and(|value|value.constituent_count==1&&value.name=="IFCDIMENSIONALEXPONENTS"&&value.args.len()==7&&value.args.iter().enumerate().all(|(index,exponent)|exponent.as_integer()==Some(if index==0{1}else{0}))){self.fail(id,"invalid-length-dimensions");return}
  let Some(factor)=entity.args.get(3).and_then(GeometryValue::as_reference).and_then(|id|self.entity(graph,id)).filter(|value|value.constituent_count==1&&value.name=="IFCMEASUREWITHUNIT")else{self.fail(id,"invalid-length-factor");return};
  let value=factor.args.first().and_then(GeometryValue::as_typed).filter(|(name,items)|items.len()==1&&(matches!(*name,"IFCLENGTHMEASURE"|"IFCPOSITIVELENGTHMEASURE")||graph.standard()==GeometryStandard::Ifc4&&*name=="IFCNONNEGATIVELENGTHMEASURE")).and_then(|(_,items)|items[0].as_real());let next=factor.args.get(1).and_then(GeometryValue::as_reference);let(Some(value),Some(next))=(value,next)else{self.fail(id,"invalid-length-factor");return};if !value.is_finite()||value<=0.{self.fail(id,"invalid-length-factor");return}let scale=self.scale*value;if !scale.is_finite()||scale<=0.{self.fail(id,"unrepresentable-length-scale");return}self.scale=scale;self.unit=next;
 }
 /// 🧹️ Drain retained scalar lookup entries within the caller's original close work grant.
 pub fn close(&mut self,maximum_work:usize,control:&mut dyn FnMut(usize)->LengthUnitControl)->bool{
  for _ in 0..maximum_work{if self.index.is_empty()&&self.visited.is_empty()&&self.assigned_units.is_empty(){break}if control(self.work)==LengthUnitControl::Yield{return false}self.work=self.work.saturating_add(1);if !self.index.is_empty(){self.index.pop_first();}else if !self.visited.is_empty(){self.visited.pop_first();}else{self.assigned_units.pop_first();}}
  if !self.index.is_empty()||!self.visited.is_empty()||!self.assigned_units.is_empty(){return false}self.closed=true;self.source_identity=None;self.source_length=None;self.result=LengthUnitResult::Cancelled;true
 }
}
fn prefix_power(prefix:&str)->Option<i32>{Some(match prefix{"EXA"=>18,"PETA"=>15,"TERA"=>12,"GIGA"=>9,"MEGA"=>6,"KILO"=>3,"HECTO"=>2,"DECA"=>1,"DECI"=>-1,"CENTI"=>-2,"MILLI"=>-3,"MICRO"=>-6,"NANO"=>-9,"PICO"=>-12,"FEMTO"=>-15,"ATTO"=>-18,_=>return None})}
