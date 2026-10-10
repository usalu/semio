//! 🌐️ Original foreign sources produce and retire proposal fields under caller-owned currencies.
/// 🌉️ A mutation step aimed at an artifact OTHER than the one being mutated. Cross-boundary
/// identity travels as plain strings, never `semio_framework::*`/`io::*` types — see the
/// dependency-edge law at `.🧬semio/🦑️repo/🎫️tickets/26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/📋️contract-freeze.md`
/// §0.
#[derive(Clone, Debug, PartialEq)]
pub struct ForeignTarget {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub dialect: Option<String>,
}

/// 🌉️ Hand-written, not derived — same DAG reason as `HybridLogicalTimestamp`/`ids`/`UndoPolicy`
/// (this crate sits below `os-kernel`). Mirrors `#[serde(rename_all = "camelCase")]` field naming
/// and the `dialect` sparse-emission by hand.
impl crate::value::ToValue for ForeignTarget {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![
            ("artifactId".to_string(), crate::value::ToValue::to_value(&self.artifact_id)),
            ("artifactKind".to_string(), crate::value::ToValue::to_value(&self.artifact_kind)),
        ];
        if self.dialect.is_some() {
            entries.push(("dialect".to_string(), crate::value::ToValue::to_value(&self.dialect)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for ForeignTarget {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for ForeignTarget, found {value:?}")));
        };
        let mut artifact_id = None;
        let mut artifact_kind = None;
        let mut dialect = None;
        for (key, entry) in fields {
            match key.as_str() {
                "artifactId" => artifact_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("artifactId"))?),
                "artifactKind" => artifact_kind = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("artifactKind"))?),
                "dialect" => dialect = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("dialect"))?,
                _ => {}
            }
        }
        Ok(ForeignTarget {
            artifact_id: artifact_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ForeignTarget missing artifactId"))?,
            artifact_kind: artifact_kind.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ForeignTarget missing artifactKind"))?,
            dialect,
        })
    }
}

/// 🪜️ One foreign hop of a [`Planner`]'s plan: the target artifact, the mutation/contributed
/// id it dispatches, its already-encoded payload, and a human label.
#[derive(Clone, Debug, PartialEq)]
pub struct ForeignStep {
    pub target: ForeignTarget,
    pub mutation_id: crate::ids::SchemaId,
    pub payload: Vec<u8>,
    pub label: String,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin above
/// documents.
impl crate::value::ToValue for ForeignStep {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("target".to_string(), crate::value::ToValue::to_value(&self.target)),
            ("mutationId".to_string(), crate::value::ToValue::to_value(&self.mutation_id)),
            ("payload".to_string(), crate::value::ToValue::to_value(&self.payload)),
            ("label".to_string(), crate::value::ToValue::to_value(&self.label)),
        ])
    }
}
impl crate::value::FromValue for ForeignStep {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for ForeignStep, found {value:?}")));
        };
        let mut target = None;
        let mut mutation_id = None;
        let mut payload = None;
        let mut label = None;
        for (key, entry) in fields {
            match key.as_str() {
                "target" => target = Some(<ForeignTarget as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?),
                "mutationId" => mutation_id = Some(<crate::ids::SchemaId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("mutationId"))?),
                "payload" => payload = Some(<Vec<u8> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("payload"))?),
                "label" => label = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("label"))?),
                _ => {}
            }
        }
        Ok(ForeignStep {
            target: target.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ForeignStep missing target"))?,
            mutation_id: mutation_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ForeignStep missing mutationId"))?,
            payload: payload.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ForeignStep missing payload"))?,
            label: label.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ForeignStep missing label"))?,
        })
    }
}
use semio_framework_value::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::ManuallyDrop;

#[derive(Clone,Copy)]
pub struct ForeignStepSource<'a> { pub artifact_id:&'a str,pub artifact_kind:&'a str,pub dialect:Option<&'a str>,pub mutation_id:&'a str,pub payload:&'a[u8],pub label:&'a str }
impl<'a> ForeignStepSource<'a> {
 pub fn borrowed(value:&'a ForeignStep)->Self {Self{artifact_id:&value.target.artifact_id,artifact_kind:&value.target.artifact_kind,dialect:value.target.dialect.as_deref(),mutation_id:&value.mutation_id.0,payload:&value.payload,label:&value.label}}
 fn identity(self)->[(usize,usize);6] {let dialect=self.dialect.map_or((0,0),|s|(s.as_ptr()as usize,s.len()));[(self.artifact_id.as_ptr()as usize,self.artifact_id.len()),(self.artifact_kind.as_ptr()as usize,self.artifact_kind.len()),dialect,(self.mutation_id.as_ptr()as usize,self.mutation_id.len()),(self.payload.as_ptr()as usize,self.payload.len()),(self.label.as_ptr()as usize,self.label.len())]}
 fn text(self,phase:u8)->Option<&'a str>{match phase{0=>Some(self.artifact_id),1=>Some(self.artifact_kind),2=>Some(self.dialect.unwrap_or("")),3=>Some(self.mutation_id),5=>Some(self.label),_=>None}}
 fn bytes(self,phase:u8)->&'a[u8] {match phase{0=>self.artifact_id.as_bytes(),1=>self.artifact_kind.as_bytes(),2=>self.dialect.unwrap_or("").as_bytes(),3=>self.mutation_id.as_bytes(),4=>self.payload,5=>self.label.as_bytes(),_=>&[]}}
}
fn refusal(kind:ValueRefusalKind)->ValueError {ValueError::literal(kind,"foreign output requires its original admitted owner")}
fn permits(grant:RetainedCloneGrant,capacity:usize,release:usize)->bool {grant.maximum_items>0&&grant.maximum_depth>0&&grant.maximum_capacity_bytes>=capacity&&grant.maximum_release_bytes>=release}
fn exact_bytes(capacity:usize)->Result<Vec<u8>,ValueError> {
 if capacity==0{return Ok(Vec::new());}
 let layout=std::alloc::Layout::array::<u8>(capacity).map_err(|_|refusal(ValueRefusalKind::OwnershipLimit))?;
 let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}).ok_or_else(||refusal(ValueRefusalKind::AllocationFailed))?;
 Ok(unsafe{Vec::from_raw_parts(pointer.as_ptr(),0,capacity)})
}
struct ForeignStepOwner {output:ManuallyDrop<Option<ForeignStep>>,closing:bool,close_phase:u8}
impl ForeignStepOwner {
 fn new(output:ForeignStep)->Self {Self{output:ManuallyDrop::new(Some(output)),closing:false,close_phase:0}}
 fn field(&self,phase:u8)->Option<&String>{let v=self.output.as_ref()?;match phase{0=>Some(&v.target.artifact_id),1=>Some(&v.target.artifact_kind),2=>v.target.dialect.as_ref(),3=>Some(&v.mutation_id.0),5=>Some(&v.label),_=>None}}
 fn field_mut(&mut self,phase:u8)->Option<&mut String>{let v=self.output.as_mut()?;match phase{0=>Some(&mut v.target.artifact_id),1=>Some(&mut v.target.artifact_kind),2=>v.target.dialect.as_mut(),3=>Some(&mut v.mutation_id.0),5=>Some(&mut v.label),_=>None}}
 fn length(&self,phase:u8)->usize {if phase==4{self.output.as_ref().map_or(0,|v|v.payload.len())}else{self.field(phase).map_or(0,String::len)}}
 fn capacity(&self,phase:u8)->usize {if phase==4{self.output.as_ref().map_or(0,|v|v.payload.capacity())}else{self.field(phase).map_or(0,String::capacity)}}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let release=self.capacity(self.close_phase);
  if !self.closing||!permits(grant,0,release){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.close_phase<6{
   if self.close_phase==4{drop(std::mem::take(&mut self.output.as_mut().ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?.payload));}
   else if let Some(field)=self.field_mut(self.close_phase){drop(std::mem::take(field));}
   self.close_phase+=1;
   return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()}));
  }
  drop(self.output.take());
  Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 fn terminal_is_empty(&self)->bool {self.closing&&self.output.is_none()}
}
impl Drop for ForeignStepOwner {fn drop(&mut self){let empty=self.output.is_none();assert!(empty||std::thread::panicking(),"foreign output retains its original backings until paid closure or transfer");if empty{unsafe{ManuallyDrop::drop(&mut self.output);}}}}

/// 🌐️ Reborrows exact immutable fields on each turn while retaining every partial output backing.
pub struct ForeignStepCopy {identity:[(usize,usize);6],owner:ForeignStepOwner,phase:u8,allocated:bool,cancelled:bool}
impl ForeignStepCopy {
 /// 🎟️ Admits allocation-free source aliases and empty inline owners; no payload is copied.
 pub fn admit(source:ForeignStepSource<'_>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,ForeignStepSource<'_>)>{
  if !permits(grant,0,0){return Err((refusal(if grant.maximum_items==0{ValueRefusalKind::WorkLimit}else{ValueRefusalKind::DepthLimit}),source));}
  let output=ForeignStep{target:ForeignTarget{artifact_id:String::new(),artifact_kind:String::new(),dialect:source.dialect.map(|_|String::new())},mutation_id:crate::ids::SchemaId(String::new()),payload:Vec::new(),label:String::new()};
  Ok((Self{identity:source.identity(),owner:ForeignStepOwner::new(output),phase:0,allocated:false,cancelled:false},RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 fn check(&self,source:ForeignStepSource<'_>)->Result<(),ValueError>{if self.identity!=source.identity(){Err(refusal(ValueRefusalKind::InvariantViolated))}else{Ok(())}}
 pub fn next_capacity_byte_demand(&self,source:ForeignStepSource<'_>)->Result<usize,ValueError>{self.check(source)?;Ok(if self.phase<6&&!self.allocated{source.bytes(self.phase).len()}else{0})}
 pub fn next_copy_byte_demand(&self,source:ForeignStepSource<'_>)->Result<usize,ValueError>{self.check(source)?;if self.phase>=6||!self.allocated{return Ok(0);}let bytes=source.bytes(self.phase);let length=self.owner.length(self.phase);if length==bytes.len(){return Ok(0);}if self.phase==4{return Ok(1);}Ok(source.text(self.phase).ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?.get(length..).and_then(|s|s.chars().next()).ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?.len_utf8())}
 pub fn advance(&mut self,source:ForeignStepSource<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  self.check(source)?;
  if self.cancelled||self.owner.closing||!permits(grant,0,0){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.phase==6{return Ok(RetainedCloneStep::Complete(Default::default()));}
  let capacity=self.next_capacity_byte_demand(source)?;
  if capacity>grant.maximum_capacity_bytes||self.next_copy_byte_demand(source)?>grant.maximum_copy_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  if !self.allocated{
   let bytes=exact_bytes(capacity)?;
   if self.phase==4{self.owner.output.as_mut().ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?.payload=bytes;}
   else if let Some(field)=self.owner.field_mut(self.phase){*field=unsafe{String::from_utf8_unchecked(bytes)};}
   progress.retained_capacity_bytes=capacity;self.allocated=true;
  }else{
   let input=source.bytes(self.phase);let length=self.owner.length(self.phase);
   if length==input.len(){self.phase+=1;self.allocated=false;}
   else if self.phase==4{let count=(input.len()-length).min(grant.maximum_copy_bytes).min(4);if count==0{return Ok(RetainedCloneStep::Progress(Default::default()));}self.owner.output.as_mut().ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?.payload.extend_from_slice(&input[length..length+count]);progress.copied_bytes=count;}
   else {let text=source.text(self.phase).ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?;let scalar=text.get(length..).and_then(|s|s.chars().next()).ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?;self.owner.field_mut(self.phase).ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?.push(scalar);progress.copied_bytes=scalar.len_utf8();}
  }
  Ok(if self.phase==6{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 /// 📬️ Transfers an already funded output allocation-free under the original caller item/depth.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Option<(ForeignStep,RetainedCloneProgress)>{if self.phase!=6||self.cancelled||self.owner.closing||!permits(grant,0,0){return None;}self.owner.output.take().map(|v|(v,RetainedCloneProgress{copied_items:1,..Default::default()}))}
 pub fn cancel(&mut self){self.cancelled=true;self.owner.closing=true;}
 pub fn begin_close(&mut self){self.owner.closing=true;}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.owner.close_step(grant)}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 pub fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.owner.capacity(self.owner.close_phase))}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal_is_empty()))}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
}
impl ErasedSnapshotRetirement for ForeignStepCopy {
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,g)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_close_copy_byte_demand()}
 fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{self.next_close_capacity_byte_demand(b)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Self::next_release_byte_demand(self)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Self::next_depth_demand(self)}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
}
/// ♻️ Keeps the original accepted proposal field backings inline until individually paid release.
pub struct ForeignStepRetirement {owner:ForeignStepOwner}
impl ForeignStepRetirement {
 pub fn admit(value:ForeignStep,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,ForeignStep)>{if !permits(grant,0,0){return Err((refusal(if grant.maximum_items==0{ValueRefusalKind::WorkLimit}else{ValueRefusalKind::DepthLimit}),value));}let mut owner=ForeignStepOwner::new(value);owner.closing=true;Ok((Self{owner},RetainedCloneProgress{copied_items:1,..Default::default()}))}
}
impl ErasedSnapshotRetirement for ForeignStepRetirement {
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.owner.close_step(g)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.owner.capacity(self.owner.close_phase))}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal_is_empty()))}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
}
/// 🧺️ Retains original proposal rows and pays every contiguous backing transfer or release.
pub struct ForeignStepsOwner { values:ManuallyDrop<Vec<ForeignStep>>,child:Option<ForeignStepRetirement>,closing:bool }
impl ForeignStepsOwner {
 pub fn empty()->Self{Self{values:ManuallyDrop::new(Vec::new()),child:None,closing:false}}
 pub fn len(&self)->usize{self.values.len()}
 pub fn get(&self,index:usize)->Option<&ForeignStep>{self.values.get(index)}
 pub fn append_demands(&self)->Result<semio_framework_value::RetirementDemand,ValueError>{
  let width=std::mem::size_of::<ForeignStep>();let count=self.len().checked_add(1).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit))?;
  Ok(semio_framework_value::RetirementDemand{copy_bytes:count.checked_mul(width).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit))?,capacity_bytes:count.checked_mul(width).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit))?,release_bytes:self.values.capacity().checked_mul(width).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit))?,depth:1})
 }
 pub fn try_append(&mut self,value:ForeignStep,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,ForeignStep)>{
  if self.closing||self.child.is_some(){return Err((refusal(ValueRefusalKind::InvariantViolated),value));}
  let demand=match self.append_demands(){Ok(d)=>d,Err(e)=>return Err((e,value))};
  if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Err((refusal(ValueRefusalKind::OwnershipLimit),value));}
  let count=self.len()+1;let layout=match std::alloc::Layout::array::<ForeignStep>(count){Ok(v)=>v,Err(_)=>return Err((refusal(ValueRefusalKind::OwnershipLimit),value))};
  let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<ForeignStep>())else{return Err((refusal(ValueRefusalKind::AllocationFailed),value));};
  let length=self.len();unsafe{std::ptr::copy_nonoverlapping(self.values.as_ptr(),pointer.as_ptr(),length);pointer.as_ptr().add(length).write(value);}
  let next=unsafe{Vec::from_raw_parts(pointer.as_ptr(),count,count)};
  let mut original=std::mem::replace(&mut*self.values,next);unsafe{original.set_len(0);}drop(original);
  Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes})
 }
 /// 📬️ Moves the already paid original row sequence; no source encoding or allocation occurs.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Option<(Vec<ForeignStep>,RetainedCloneProgress)>{if self.closing||self.child.is_some()||!permits(grant,0,0){return None;}Some((std::mem::take(&mut*self.values),RetainedCloneProgress{copied_items:1,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if !self.closing||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  if let Some(child)=self.child.as_mut(){let step=child.close_step(child_grant)?;if !step.progress().fits(child_grant){return Err(refusal(ValueRefusalKind::InvariantViolated));}if child.terminal_is_empty(){self.child=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if !self.values.is_empty(){if child_grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let original=self.values.pop().ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?;return match ForeignStepRetirement::admit(original,child_grant){Ok((child,p))=>{self.child=Some(child);Ok(RetainedCloneStep::Progress(p))},Err((e,original))=>{self.values.push(original);Err(e)}};}
  let release=self.values.capacity().checked_mul(std::mem::size_of::<ForeignStep>()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit))?;if grant.maximum_release_bytes<release{return Ok(RetainedCloneStep::Progress(Default::default()));}
  drop(std::mem::take(&mut*self.values));Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()}))
 }
 pub fn terminal_is_empty(&self)->bool{self.values.is_empty()&&self.values.capacity()==0&&self.child.is_none()}
}
impl ErasedSnapshotRetirement for ForeignStepsOwner {
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close_one(g)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(c)=&self.child{c.next_copy_byte_demand()}else{Ok(0)}}
 fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{if let Some(c)=&self.child{c.next_capacity_byte_demand(b)}else{Ok(0)}}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(c)=&self.child{c.next_release_byte_demand()}else if self.values.is_empty(){self.values.capacity().checked_mul(std::mem::size_of::<ForeignStep>()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit))}else{Ok(0)}}
 fn next_depth_demand(&self)->Result<usize,ValueError>{if let Some(c)=&self.child{c.next_depth_demand()?.checked_add(1).ok_or_else(||refusal(ValueRefusalKind::DepthLimit))}else{Ok(if self.values.is_empty(){usize::from(!self.terminal_is_empty())}else{2})}}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
}
impl Drop for ForeignStepsOwner {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"foreign proposal sequence abandoned before paid close or transfer");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.values);}}}}
/// 🪜️ Keeps each indexed borrowed row and every paid partial field until transfer or cancellation.
pub struct ForeignStepsPreparation {
 index:usize,copy:Option<ForeignStepCopy>,copy_complete:bool,
 pending:ManuallyDrop<Option<ForeignStep>>,retiring:Option<ForeignStepRetirement>,
 values:ForeignStepsOwner,complete:bool,closing:bool,
}
impl ForeignStepsPreparation {
 pub fn empty()->Self{Self{index:0,copy:None,copy_complete:false,pending:ManuallyDrop::new(None),retiring:None,values:ForeignStepsOwner::empty(),complete:false,closing:false}}
 /// 🔀️ Begins the next original forward only after its entire indexed source has settled.
 pub fn begin_next_source(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  if self.closing||!self.complete||self.copy.is_some()||self.pending.is_some()||self.retiring.is_some(){return Err(refusal(ValueRefusalKind::InvariantViolated));}
  let copied_bytes=std::mem::size_of::<usize>()+std::mem::size_of::<bool>();
  if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copied_bytes{return Ok(None);}
  self.index=0;self.complete=false;
  Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}))
 }
 pub fn source_index(&self)->usize{self.index}
 pub fn is_complete(&self)->bool{self.complete&&!self.closing}
 pub fn len(&self)->usize{self.values.len()}
 fn child_grant(grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{let maximum_depth=grant.maximum_depth.checked_sub(1).ok_or_else(||refusal(ValueRefusalKind::DepthLimit))?;Ok(RetainedCloneGrant{maximum_depth,..grant})}
 pub fn preparation_demands(&self,source:Option<ForeignStepSource<'_>>)->Result<semio_framework_value::RetirementDemand,ValueError>{
  use semio_framework_value::RetirementDemand;
  if self.closing{return Err(refusal(ValueRefusalKind::InvariantViolated));}
  if self.complete{return Ok(Default::default());}
  if let Some(copy)=&self.copy{if self.copy_complete{return Ok(RetirementDemand{depth:2,..Default::default()});}let source=source.ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?;return Ok(RetirementDemand{copy_bytes:copy.next_copy_byte_demand(source)?,capacity_bytes:copy.next_capacity_byte_demand(source)?,depth:2,..Default::default()});}
  if self.pending.is_some(){let mut d=self.values.append_demands()?;d.depth=d.depth.checked_add(1).ok_or_else(||refusal(ValueRefusalKind::DepthLimit))?;return Ok(d);}
  Ok(RetirementDemand{depth:if source.is_some(){2}else{1},..Default::default()})
 }
 /// ⏭️ Performs one actual field, row admission or sequence-boundary turn under unchanged caller currencies.
 pub fn advance_source(&mut self,source:Option<ForeignStepSource<'_>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.closing{return Err(refusal(ValueRefusalKind::InvariantViolated));}
  if self.complete{return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.preparation_demands(source)?;
  if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let child_grant=Self::child_grant(grant)?;
  if let Some(copy)=self.copy.as_mut(){
   if self.copy_complete{
    if self.pending.is_none()&&copy.owner.output.is_some(){let(value,p)=copy.take(child_grant).ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?;*self.pending=Some(value);copy.begin_close();return Ok(RetainedCloneStep::Progress(p));}
    let step=copy.close_step(child_grant)?;let mut progress=step.progress();if copy.terminal_is_empty(){self.copy=None;self.copy_complete=false;progress.copied_items=progress.copied_items.max(1);}return Ok(RetainedCloneStep::Progress(progress));
   }
   let step=copy.advance(source.ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?,child_grant)?;self.copy_complete=matches!(step,RetainedCloneStep::Complete(_));return Ok(RetainedCloneStep::Progress(step.progress()));
  }
  if self.pending.is_some(){
   let next=self.index.checked_add(1).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit))?;
   let value=self.pending.take().ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?;
   return match self.values.try_append(value,child_grant){Ok(p)=>{self.index=next;Ok(RetainedCloneStep::Progress(p))},Err((e,value))=>{*self.pending=Some(value);Err(e)}};
  }
  if let Some(source)=source{let(copy,p)=ForeignStepCopy::admit(source,child_grant).map_err(|(e,_)|e)?;self.copy=Some(copy);return Ok(RetainedCloneStep::Progress(p));}
  self.complete=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 /// 📬️ Transfers only an entirely prepared original sequence without allocating or encoding.
 pub fn take_prepared(&mut self,grant:RetainedCloneGrant)->Option<(Vec<ForeignStep>,RetainedCloneProgress)>{if !self.is_complete()||self.copy.is_some()||self.pending.is_some(){return None;}let child=Self::child_grant(grant).ok()?;self.values.take(child)}
 pub fn begin_close(&mut self){self.closing=true;if let Some(copy)=self.copy.as_mut(){copy.cancel();}self.values.begin_close();}
 pub fn terminal_is_empty(&self)->bool{self.copy.is_none()&&self.pending.is_none()&&self.retiring.is_none()&&self.values.terminal_is_empty()}
 fn close_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  if !self.closing||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let child_grant=Self::child_grant(grant)?;
  if let Some(copy)=self.copy.as_mut(){let step=copy.close_step(child_grant)?;let mut progress=step.progress();if copy.terminal_is_empty(){self.copy=None;progress.copied_items=progress.copied_items.max(1);}return Ok(RetainedCloneStep::Progress(progress));}
  if let Some(retiring)=self.retiring.as_mut(){let step=retiring.close_step(child_grant)?;if retiring.terminal_is_empty(){self.retiring=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
  if self.pending.is_some(){if !permits(child_grant,0,0){return Ok(RetainedCloneStep::Progress(Default::default()));}let value=self.pending.take().ok_or_else(||refusal(ValueRefusalKind::InvariantViolated))?;return match ForeignStepRetirement::admit(value,child_grant){Ok((owner,p))=>{self.retiring=Some(owner);Ok(RetainedCloneStep::Progress(p))},Err((e,value))=>{*self.pending=Some(value);Err(e)}};}
  let step=self.values.close_step(child_grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
}
impl ErasedSnapshotRetirement for ForeignStepsPreparation {
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close_one(g)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(v)=&self.copy{v.next_close_copy_byte_demand()}else if let Some(v)=&self.retiring{v.next_copy_byte_demand()}else{self.values.next_copy_byte_demand()}}
 fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{if let Some(v)=&self.copy{v.next_close_capacity_byte_demand(b)}else if let Some(v)=&self.retiring{v.next_capacity_byte_demand(b)}else{self.values.next_capacity_byte_demand(b)}}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(v)=&self.copy{v.next_release_byte_demand()}else if let Some(v)=&self.retiring{v.next_release_byte_demand()}else if self.pending.is_some(){Ok(0)}else{self.values.next_release_byte_demand()}}
 fn next_depth_demand(&self)->Result<usize,ValueError>{let depth=if let Some(v)=&self.copy{v.next_depth_demand()?}else if let Some(v)=&self.retiring{v.next_depth_demand()?}else if self.pending.is_some(){1}else{self.values.next_depth_demand()?};depth.checked_add(usize::from(!self.terminal_is_empty())).ok_or_else(||refusal(ValueRefusalKind::DepthLimit))}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
}
impl Drop for ForeignStepsPreparation {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"indexed foreign preparation abandoned an original partial backing");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.pending);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
