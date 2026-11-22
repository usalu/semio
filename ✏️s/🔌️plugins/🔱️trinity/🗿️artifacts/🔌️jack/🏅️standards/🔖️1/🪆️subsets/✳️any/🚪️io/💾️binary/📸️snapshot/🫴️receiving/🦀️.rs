//! 🫴️ Original Jack record backing moves into one typed candidate through finite receiving turns.
use crate::standards::v1::subsets::any::schema::graph::Camera;
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::standards::v1::subsets::any::schema::content::JackContentChild;
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_dsl_record::receiving::owned::{
    admits, done, idle, normal, perform, DslOwnedReceivingField,
    OwnedAdvanceSelection, OwnedFieldReceiving, OwnedPoll, OwnedSlot,
};
use crate::standards::v1::subsets::any::io::text::snapshot::JackPackRecord;
use semio_framework_value::{
    OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress,
    RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind,
};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};
struct Owners{parsed:Option<JackPackRecord>,candidate:Option<JackSnapshot>}
semio_framework_value::artifact_retire_struct!(Owners{parsed,candidate});
/// 📦️ Partial typed owners and unconsumed original fields stay together through refusal and paid close.
pub struct JackRecordReconstruction{source:OwnedSlot<FieldValue>,fields:ManuallyDrop<<JackPackRecord as DslOwnedReceivingField>::Cursor>,fields_done:bool,captured:bool,owners:ManuallyDrop<Owners>,retiring:Option<ControlledRetirement<Owners>>,root:u16,ready:bool,closing:bool,closed:bool}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,text)}
fn empty_snapshot()->JackSnapshot{JackSnapshot{schema:String::new(),name:String::new(),manifest_id:String::new(),camera:Camera::default(),content:JackContentChild::new(String::new(),semio_framework_artifact_reference::ArtifactRef{artifact_id:String::new(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()}}),root_node_id:None,query:String::new()}}
impl JackRecordReconstruction{
 /// 🕳️ Creates only empty fixed receiving headers; no schema, record or candidate backing is allocated.
 pub fn empty()->Self{Self{source:Default::default(),fields:ManuallyDrop::new(Default::default()),fields_done:false,captured:false,owners:ManuallyDrop::new(Owners{parsed:None,candidate:None}),retiring:None,root:0,ready:false,closing:false,closed:false}}
 /// 📥️ Captures the exact original fields only after fixed header admission, leaving refused input untouched.
 pub fn receive(&mut self,source:&mut Option<RecordValue>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let bytes=size_of::<FieldValue>();if self.captured||self.closing{return Err(invalid("Jack original reconstruction already owns a source"))}if source.is_none()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(Default::default())}*self.source.value=Some(FieldValue::Record(source.take().unwrap()));self.captured=true;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})}

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
            capacity_bytes: 0,
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
  if self.root==7{self.ready=true;*progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(())}
  let root=self.root;
  let owners=&mut*self.owners;let raw=owners.parsed.as_mut().unwrap();let target=owners.candidate.as_mut().unwrap();
  match root{
   0=>{if raw.schema!=JackSnapshot::SCHEMA{return Err(invalid("Jack original snapshot schema differs from its declared owner"))}target.schema=std::mem::take(&mut raw.schema);progress.copied_bytes=size_of::<String>()},
   1=>{target.name=std::mem::take(&mut raw.name);progress.copied_bytes=size_of::<String>()},
   2=>{target.manifest_id=std::mem::take(&mut raw.manifest_id);progress.copied_bytes=size_of::<String>()},
   3=>{if !raw.camera.x.is_finite()||!raw.camera.y.is_finite()||!raw.camera.zoom.is_finite(){return Err(invalid("Jack original camera requires finite binary64"))}target.camera=std::mem::replace(&mut raw.camera,Camera::default());progress.copied_bytes=size_of::<Camera>()},
   4=>{let child=&mut raw.content;target.content=JackContentChild::new(std::mem::take(&mut child.child_id),semio_framework_artifact_reference::ArtifactRef{artifact_id:std::mem::take(&mut child.artifact_id),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:std::mem::take(&mut child.artifact_kind),standard:std::mem::take(&mut child.standard),subset:std::mem::take(&mut child.subset)}});progress.copied_bytes=size_of::<JackContentChild>()},
   5=>{target.root_node_id=raw.root_node_id.take();progress.copied_bytes=size_of::<Option<String>>()},
   6=>{target.query=std::mem::take(&mut raw.query);progress.copied_bytes=size_of::<String>()},
   _=>unreachable!()
  }self.root+=1;progress.copied_items=1;Ok(())
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
    let empty = Owners { parsed: None, candidate: None };
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
