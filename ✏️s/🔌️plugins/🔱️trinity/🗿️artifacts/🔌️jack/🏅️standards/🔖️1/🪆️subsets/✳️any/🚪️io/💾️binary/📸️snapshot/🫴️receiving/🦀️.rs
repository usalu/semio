//! 🫴️ Original Jack record backing moves into one typed candidate through finite receiving turns.
use crate::standards::v1::subsets::any::schema::graph::Camera;
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::standards::v1::subsets::any::schema::content::JackContentChild;
use crate::standards::v1::subsets::any::schema::manifest::{Manifest, NodeKindDef, EdgeKindDef, PortKindDef};
use semio_framework_graph_manifest::{PortDirection, PropertyDef, PropertyKind};
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_dsl_record::receiving::owned::{
    admits, allocate_empty_vec, done, idle, normal, perform, DslOwnedReceivingField,
    OwnedAdvanceSelection, OwnedFieldReceiving, OwnedPoll, OwnedSlot,
};
use crate::standards::v1::subsets::any::io::text::snapshot::JackPackRecord;
use semio_framework_value::{
    OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress,
    RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind, ValueType,
};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};
/// 📏️ Declared structural bounds belong to the actual caller, independently of its supplied turn grant.
#[derive(Clone,Copy)]
pub struct JackRecordReconstructionPolicy{pub maximum_rows:usize,pub maximum_type_depth:usize}
struct Owners{parsed:Option<JackPackRecord>,candidate:Option<JackSnapshot>,name:String,properties:Vec<PropertyDef>,ports:Vec<String>,property_name:String,property_type:Option<ValueType>,expression:Option<String>}
semio_framework_value::artifact_retire_struct!(Owners{parsed,candidate,name,properties,ports,property_name,property_type,expression});
/// 📦️ Partial typed owners and unconsumed original fields stay together through refusal and paid close.
pub struct JackRecordReconstruction{source:OwnedSlot<FieldValue>,fields:ManuallyDrop<<JackPackRecord as DslOwnedReceivingField>::Cursor>,fields_done:bool,captured:bool,owners:ManuallyDrop<Owners>,retiring:Option<ControlledRetirement<Owners>>,policy:JackRecordReconstructionPolicy,root:u16,phase:u8,kind:usize,property:usize,property_phase:u8,path:usize,port:usize,direction:PortDirection,property_kind:PropertyKind,ready:bool,closing:bool,closed:bool}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,text)}
fn allocate<T>(count: usize, progress: &mut RetainedCloneProgress) -> Result<Vec<T>, ValueError> {
    let mut capacity = None;
    let vector = allocate_empty_vec(count, &mut capacity)?;
    progress.retained_capacity_bytes = capacity.unwrap_or_default();
    Ok(vector)
}
fn empty_snapshot()->JackSnapshot{JackSnapshot{schema:String::new(),name:String::new(),manifest_id:None,manifest:Manifest::default(),camera:Camera::default(),content:JackContentChild::new(String::new(),semio_framework_artifact_reference::ArtifactRef{artifact_id:String::new(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()}}),root_node_id:None,query:String::new()}}
impl JackRecordReconstruction{
 /// 🕳️ Creates only empty fixed receiving headers; no schema, record or candidate backing is allocated.
 pub fn empty(policy:JackRecordReconstructionPolicy)->Self{Self{source:Default::default(),fields:ManuallyDrop::new(Default::default()),fields_done:false,captured:false,owners:ManuallyDrop::new(Owners{parsed:None,candidate:None,name:String::new(),properties:Vec::new(),ports:Vec::new(),property_name:String::new(),property_type:None,expression:None}),retiring:None,policy,root:0,phase:0,kind:0,property:0,property_phase:0,path:0,port:0,direction:PortDirection::In,property_kind:PropertyKind::Data,ready:false,closing:false,closed:false}}
 /// 📥️ Captures the exact original fields only after fixed header admission, leaving refused input untouched.
 pub fn receive(&mut self,source:&mut Option<RecordValue>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let bytes=size_of::<FieldValue>();if self.captured||self.closing{return Err(invalid("Jack original reconstruction already owns a source"))}if source.is_none()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(Default::default())}*self.source.value=Some(FieldValue::Record(source.take().unwrap()));self.captured=true;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})}
 fn parsed(&self)->&JackPackRecord{self.owners.parsed.as_ref().unwrap()}
 fn property_count(&self)->usize{let raw=self.parsed();match self.root{3=>raw.node_kinds[self.kind].properties.len(),4=>raw.edge_kinds[self.kind].properties.len(),_=>raw.port_kinds[self.kind].properties.len()}}
 fn property_record(&self)->&crate::standards::v1::subsets::any::io::text::snapshot::records::PropertyRecord{let raw=self.parsed();match self.root{3=>&raw.node_kinds[self.kind].properties[self.property],4=>&raw.edge_kinds[self.kind].properties[self.property],_=>&raw.port_kinds[self.kind].properties[self.property]}}
 fn property_record_mut(&mut self)->&mut crate::standards::v1::subsets::any::io::text::snapshot::records::PropertyRecord{let raw=self.owners.parsed.as_mut().unwrap();match self.root{3=>&mut raw.node_kinds[self.kind].properties[self.property],4=>&mut raw.edge_kinds[self.kind].properties[self.property],_=>&mut raw.port_kinds[self.kind].properties[self.property]}}
 fn kind_count(&self)->usize{let raw=self.parsed();match self.root{3=>raw.node_kinds.len(),4=>raw.edge_kinds.len(),_=>raw.port_kinds.len()}}
 fn count(&self,count:usize)->Result<usize,ValueError>{if count>self.policy.maximum_rows{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Jack original reconstruction exceeds its declared row bound"))}Ok(count)}
 fn allocation_bytes(&self)->Result<usize,ValueError>{if self.ready||self.closing||!(3..=5).contains(&self.root){return Ok(0)}let bytes=if self.phase==0{self.count(self.kind_count())?.checked_mul(match self.root{3=>size_of::<NodeKindDef>(),4=>size_of::<EdgeKindDef>(),_=>size_of::<PortKindDef>()})}else if self.phase==2{self.count(self.property_count())?.checked_mul(size_of::<PropertyDef>())}else if self.phase==4&&self.root==3{self.count(self.parsed().node_kinds[self.kind].port_kinds.len())?.checked_mul(size_of::<String>())}else if self.phase==3&&self.property_phase==3&&self.path>0{Some(size_of::<ValueType>())}else{Some(0)};bytes.ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Jack original reconstruction backing extent overflow"))}
 /// 🧾️ Runs one bounded reconstruction instruction under the unchanged original issuer and reports actual effects.
 pub fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OwnedPoll {
    if self.closing || !self.captured {
        return done(effect, Err(invalid("Jack original reconstruction has no active source").into()));
    }
    if self.ready {
        return idle(effect);
    }
    let body = effect.grant().maximum_copy_bytes;
    if !self.fields_done {
        if self.fields.is_ready() {
            let turn = self.fields.take_into(&mut self.owners.parsed, effect);
            self.fields_done = self.owners.parsed.is_some();
            return normal(turn);
        }
        return match self.fields.select_advance(&self.source.value, body, effect.maximum_work_units()) {
            Ok(OwnedAdvanceSelection::Local { .. }) => self.fields.advance_local(&mut self.source.value, body, effect),
            Ok(OwnedAdvanceSelection::Nested) => done(effect, Err(invalid("Jack snapshot record requires local field receiving").into())),
            Err(error) => done(effect, Err(error.into())),
        };
    }

    let mut progress = RetainedCloneProgress::default();
    let result = (|| {
        let demand = RetirementDemand {
            copy_bytes: size_of::<JackSnapshot>(),
            capacity_bytes: self.allocation_bytes()?,
            depth: 4,
            ..Default::default()
        };
        if !admits(effect, demand) {
            return Ok(progress);
        }
        perform(effect)?;
        if self.owners.candidate.is_none() {
            self.owners.candidate = Some(empty_snapshot());
            progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<JackSnapshot>(), ..Default::default() };
        } else {
            self.advance(&mut progress)?;
            if progress.retained_capacity_bytes != 0 {
                *effect.actual() = Some(progress.retained_capacity_bytes);
            }
        }
        Ok(progress)
    })().map_err(|error: ValueError| error.with_retained_progress(progress).into());
    done(effect, result)
 }
 fn advance(&mut self,progress:&mut RetainedCloneProgress)->Result<(),ValueError>{
  if self.root==10{self.ready=true;*progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(())}
  if !(3..=5).contains(&self.root){
   let root=self.root;
   let owners=&mut*self.owners;let raw=owners.parsed.as_mut().unwrap();let target=owners.candidate.as_mut().unwrap();
   match root{
    0=>{if raw.schema!=JackSnapshot::SCHEMA{return Err(invalid("Jack original snapshot schema differs from its declared owner"))}target.schema=std::mem::take(&mut raw.schema);progress.copied_bytes=size_of::<String>()},
    1=>{target.name=std::mem::take(&mut raw.name);progress.copied_bytes=size_of::<String>()},
    2=>{target.manifest_id=raw.manifest_id.take();progress.copied_bytes=size_of::<Option<String>>()},
    8=>{target.root_node_id=raw.root_node_id.take();progress.copied_bytes=size_of::<Option<String>>()},
    9=>{target.query=std::mem::take(&mut raw.query);progress.copied_bytes=size_of::<String>()},
    6=>{if !raw.camera.x.is_finite()||!raw.camera.y.is_finite()||!raw.camera.zoom.is_finite(){return Err(invalid("Jack original camera requires finite binary64"))}target.camera=std::mem::replace(&mut raw.camera,Camera::default());progress.copied_bytes=size_of::<Camera>()},
    7=>{let child=&mut raw.content;target.content=JackContentChild::new(std::mem::take(&mut child.child_id),semio_framework_artifact_reference::ArtifactRef{artifact_id:std::mem::take(&mut child.artifact_id),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:std::mem::take(&mut child.artifact_kind),standard:std::mem::take(&mut child.standard),subset:std::mem::take(&mut child.subset)}});progress.copied_bytes=size_of::<JackContentChild>()},_=>unreachable!()
   }self.root+=1;progress.copied_items=1;return Ok(())
  }
  match self.phase{
   0=>{let count=self.count(self.kind_count())?;let target=self.owners.candidate.as_mut().unwrap();match self.root{3=>target.manifest.node_kinds=allocate(count,progress)?,4=>target.manifest.edge_kinds=allocate(count,progress)?,_=>target.manifest.port_kinds=allocate(count,progress)?}self.phase=1;self.kind=0},
   1=>{if self.kind==self.kind_count(){self.root+=1;self.phase=0;}else{if self.root==5{self.direction=match self.parsed().port_kinds[self.kind].direction.as_str(){"in"=>PortDirection::In,"out"=>PortDirection::Out,_=>return Err(invalid("Jack original port direction is invalid"))}}let raw=self.owners.parsed.as_mut().unwrap();self.owners.name=match self.root{3=>std::mem::take(&mut raw.node_kinds[self.kind].name),4=>std::mem::take(&mut raw.edge_kinds[self.kind].name),_=>std::mem::take(&mut raw.port_kinds[self.kind].name)};progress.copied_bytes=size_of::<String>();self.phase=2}},
   2=>{let count=self.count(self.property_count())?;self.owners.properties=allocate(count,progress)?;self.property=0;self.property_phase=0;self.phase=3},
   3=>{if self.property==self.property_count(){self.phase=4;}else{self.advance_property(progress)?;}},
   4=>{if self.root==3{let count=self.count(self.parsed().node_kinds[self.kind].port_kinds.len())?;self.owners.ports=allocate(count,progress)?;self.port=0;self.phase=5;}else{self.phase=6;}},
   5=>{if self.port==self.parsed().node_kinds[self.kind].port_kinds.len(){self.phase=6;}else{let port=self.port;let kind=self.kind;let value=std::mem::take(&mut self.owners.parsed.as_mut().unwrap().node_kinds[kind].port_kinds[port]);self.owners.ports.push(value);self.port+=1;progress.copied_bytes=size_of::<String>();}},
   6=>{let name=std::mem::take(&mut self.owners.name);let properties=std::mem::take(&mut self.owners.properties);let ports=std::mem::take(&mut self.owners.ports);let target=self.owners.candidate.as_mut().unwrap();match self.root{3=>{target.manifest.node_kinds.push(NodeKindDef{name,properties,port_kinds:ports});progress.copied_bytes=size_of::<NodeKindDef>()},4=>{target.manifest.edge_kinds.push(EdgeKindDef{name,properties});progress.copied_bytes=size_of::<EdgeKindDef>()},_=>{target.manifest.port_kinds.push(PortKindDef{name,properties,direction:self.direction});progress.copied_bytes=size_of::<PortKindDef>()}}self.kind+=1;self.phase=1},_=>unreachable!()
  }progress.copied_items=1;Ok(())
 }
 fn advance_property(&mut self,progress:&mut RetainedCloneProgress)->Result<(),ValueError>{
  match self.property_phase{
   0=>{let name=std::mem::take(&mut self.property_record_mut().name);self.owners.property_name=name;progress.copied_bytes=size_of::<String>();self.property_phase=1},
   1=>{self.property_kind=match self.property_record().kind.as_str(){"data"=>PropertyKind::Data,"derived"=>PropertyKind::Derived,_=>return Err(invalid("Jack original property kind is invalid"))};self.property_phase=2},
   2=>{let raw=self.property_record();let path=&raw.type_path;if path.is_empty()||path.len()>self.policy.maximum_type_depth{return Err(invalid("Jack original property type path exceeds its declared bound"))}let at=path.len()-1;let terminal=path[at].as_str();let ty=match terminal{"boolean"=>ValueType::Boolean,"integer"=>ValueType::Integer,"decimal"=>ValueType::Decimal,"text"=>ValueType::Text,"any"=>ValueType::Any,"schema"=>{if raw.type_schema.is_none(){return Err(invalid("Jack original schema type has no schema"))}ValueType::Schema(String::new())},_=>return Err(invalid("Jack original property type leaf is invalid"))};if terminal!="schema"&&raw.type_schema.is_some(){return Err(invalid("Jack original property type has undeclared schema"))}let ty=if matches!(ty,ValueType::Schema(_)){ValueType::Schema(self.property_record_mut().type_schema.take().unwrap())}else{ty};self.path=at;self.owners.property_type=Some(ty);self.property_phase=3;progress.copied_bytes=size_of::<ValueType>()},
   3=>{if self.path==0{self.property_phase=4}else{let index=self.path-1;if self.property_record().type_path[index]!="list"{return Err(invalid("Jack original property type ancestor must be list"))}let value=self.owners.property_type.take().unwrap();self.owners.property_type=Some(ValueType::List(Box::new(value)));self.path=index;progress.retained_capacity_bytes=size_of::<ValueType>();progress.copied_bytes=size_of::<ValueType>();}},
   4=>{let expression=self.property_record_mut().expression.take();self.owners.expression=expression;self.property_phase=5;progress.copied_bytes=size_of::<Option<String>>()},
   5=>{let value=PropertyDef{name:std::mem::take(&mut self.owners.property_name),kind:self.property_kind,value_type:self.owners.property_type.take().unwrap(),expr:self.owners.expression.take()};self.owners.properties.push(value);self.property+=1;self.property_phase=0;progress.copied_bytes=size_of::<PropertyDef>()},_=>unreachable!()
  }Ok(())
 }
 /// 🔎️ A refused declared shape retains the exact original record at its owned receiving entrance.
 pub fn original_record(&self)->Option<&RecordValue>{match self.source.value.as_ref(){Some(FieldValue::Record(record))=>Some(record),_=>None}}
 /// 🎯️ Borrows the exact candidate while renewal admission and publication remain deferred.
 pub fn candidate(&self)->Option<&JackSnapshot>{self.owners.candidate.as_ref()}
 pub fn is_ready(&self)->bool{self.ready&&!self.closing&&self.owners.candidate.is_some()}
 /// 📤️ Transfers the valid typed candidate only into an empty funded receiving slot.
 pub fn take_into(&mut self,target:&mut Option<JackSnapshot>,grant:RetainedCloneGrant)->RetainedCloneProgress{if !self.is_ready()||target.is_some()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<JackSnapshot>(){return Default::default()}*target=self.owners.candidate.take();RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<JackSnapshot>(),..Default::default()}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.fields.terminal_is_empty(){return self.fields.close_demands(body)}if !self.source.terminal_is_empty(){return self.source.close_demands(body)}if let Some(owner)=&self.retiring{owner.selected_demand(body)}else if !self.closed{Ok(RetirementDemand{copy_bytes:size_of::<Owners>(),depth:1,..Default::default()})}else{Ok(Default::default())}}
 pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll<RetainedCloneStep> {
    if self.closed {
        return done(effect, Ok(RetainedCloneStep::Complete(Default::default())));
    }
    let body = effect.grant().maximum_copy_bytes;
    let demand = match self.close_demands(body) {
        Ok(demand) => demand,
        Err(error) => return done(effect, Err(error.into())),
    };
    if !admits(effect, demand) {
        return done(effect, Ok(RetainedCloneStep::Progress(Default::default())));
    }
    self.closing = true;
    if !self.fields.terminal_is_empty() {
        return self.fields.close_step(effect).map(RetainedCloneStep::Progress);
    }
    if !self.source.terminal_is_empty() {
        return self.source.close_step(effect).map(RetainedCloneStep::Progress);
    }
    if let Some(owner) = &mut self.retiring {
        let turn = owner.step(body, effect);
        if owner.terminal_is_empty() {
            self.retiring.take();
            self.closed = true;
        }
        return turn;
    }
    if let Err(error) = perform(effect) {
        return done(effect, Err(error.into()));
    }
    let empty = Owners { parsed: None, candidate: None, name: String::new(), properties: Vec::new(), ports: Vec::new(), property_name: String::new(), property_type: None, expression: None };
    let owned = std::mem::replace(&mut *self.owners, empty);
    let result = match ControlledRetirement::new(owned) {
        Ok(owner) => {
            self.retiring = Some(owner);
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
        }
        Err((failure, owned)) => {
            *self.owners = owned;
            Err(failure.into())
        }
    };
    done(effect, result)
 }
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.retiring.is_none()&&self.fields.terminal_is_empty()&&self.source.terminal_is_empty()}
}
impl Drop for JackRecordReconstruction{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Jack original reconstruction reached Drop before its paid close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owners);ManuallyDrop::drop(&mut self.fields)}}}}
