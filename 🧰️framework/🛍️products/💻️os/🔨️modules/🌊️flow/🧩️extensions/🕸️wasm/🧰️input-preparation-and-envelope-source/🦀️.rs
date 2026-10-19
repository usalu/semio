//! 🧰️ Input preparation and envelope source of the wasm extension glue: the retained input preparation and its retirement, the evaluation step, the job capacity and the envelope source projection. See [`super`].

use super::*;

// #region ⏱️EvaluationJobs
/// 🎒️ Moves the existing canonical JSON and typed input owners through retained admission.
pub struct EvaluationInputPreparation {
    pub(super) input:Option<String>,dependency:Option<String>,pub(super) operator:Option<String>,pub(super) version:Option<String>,
    pub parser:Option<semio_framework_pack_json::JsonParseCursor>,pub(super) projection:Option<semio_framework_pack_json::JsonValueProjection>,
    pub writer:Option<semio_framework_pack_json::JsonWriteCursor<DslValue>>,pub(super) encoding:Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,pub(super) decoding:Option<semio_framework_value_native_owned::native_decoding::NativeDecodeContinuation>,
    pub binding:Option<neural_engine::retirement::RetainedDictionaryInput>,pub(super) dictionary:Option<Dictionary>,
    canonical_input:Option<String>,canonical_dependency:Option<String>,pub(super) identity:Option<String>,
    pub(super) retirement:neural_engine::ValueRetirement,pub(super) stage:u8,pub(super) units:usize,pub(super) cancelled:bool,closing_fields:u16,fault_member:Option<String>,pub(super) normal_progress:RetainedCloneProgress,
    pub(super) active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl EvaluationInputPreparation {
    /// 🌱️ Takes one initial request's source allocation without scanning or copying its payload.
    pub fn new(input:String,dependency:String,operator:String,version:String)->Self {
        Self {input:Some(input),dependency:Some(dependency),operator:Some(operator),version:Some(version),parser:Some(semio_framework_pack_json::JsonParseCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject)),projection:None,writer:None,encoding:None,decoding:None,binding:None,dictionary:None,canonical_input:None,canonical_dependency:None,identity:None,active:std::mem::ManuallyDrop::new(None),retirement:Default::default(),stage:0,units:0,cancelled:false,closing_fields:0,fault_member:None,normal_progress:Default::default()}
    }
    /// 🪪️ Exposes the still-owned source allocation for ownership laws.
    pub fn source_ptr(&self)->Option<*const u8> {self.input.as_ref().map(|source|source.as_ptr())}
    /// 📍️ Reports this same candidate's retained input and identity frontier.
    pub fn progress(&self)->(usize,usize,&'static str) {
        let phase=if !self.retirement.terminal_is_empty()||self.active.is_some()||self.closing_fields!=0 {"input-retire"} else {match self.stage {0=>"input-parse",1=>"input-order",2=>"input-canonical",3=>"input-bind",4=>"dependency-parse",5=>"dependency-order",6=>"dependency-canonical",7=>"identity-canonical",_=>"input-retire"}};
        (self.units,self.units.saturating_add(usize::from(self.stage<9)),phase)
    }
    fn write_step(&mut self,grant:RetainedCloneGrant)->Result<(Option<String>,RetainedCloneProgress),EvaluationFailure> {
        let mut admitted=|_|true;
        let mut control=match self.encoding.take() {Some(receipt)=>semio_framework_value::NativeEncodeControl::resume(receipt,&mut admitted),None=>Ok(semio_framework_value::NativeEncodeControl::new_retained(&mut admitted))}.map_err(EvaluationFailure::from)?;
        let result=self.writer.as_mut().unwrap().step(1,&mut control,grant).map_err(EvaluationFailure::from);let progress=self.writer.as_ref().unwrap().normal_step_progress();self.normal_progress=progress;
        self.encoding=Some(control.pause().map_err(EvaluationFailure::from)?);result.map(|value|(value,progress))
    }
    pub(super) fn close_intermediate(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if grant.maximum_items==0{return Ok(Default::default());}
        if !self.retirement.terminal_is_empty(){return self.retirement.close_step(grant).map(|step|step.progress());}
        if self.active.is_some(){return evaluation_child_close(&mut self.active,grant).map(|step|step.progress());}
        if self.closing_fields==0{return Ok(Default::default());}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation intermediate handoff requires admitted depth"));}
        let field=self.closing_fields&self.closing_fields.wrapping_neg();
        let progress=match field{
            1=>evaluation_admit_original(&mut self.parser,&mut self.active,grant)?.progress(),
            2=>evaluation_admit_original(&mut self.projection,&mut self.active,grant)?.progress(),
            4=>evaluation_admit_original(&mut self.writer,&mut self.active,grant)?.progress(),
            8|16|128=>{
                let source=match field{8=>&mut self.input,16=>&mut self.dependency,_=>&mut self.fault_member};let value=source.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation intermediate source lost its field"))?;
                match self.retirement.text(value,grant){Ok(progress)=>progress,Err((error,value))=>{*source=Some(value);return Err(error);}}
            },
            32|64=>{
                if field==32{self.encoding=None;}else{self.decoding=None;}RetainedCloneProgress {copied_items:1,..Default::default()}
            },
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation input has an unknown original close field")),
        };
        if progress.copied_items!=0{self.closing_fields&=!field;}
        Ok(progress)
    }
    /// ⏱️ Advances the original codec authority or one separately granted physical cleanup turn.
    pub fn normal_step_progress(&self)->RetainedCloneProgress {self.normal_progress}
    pub fn step(&mut self,maximum_units:usize,maximum_bytes:usize,grant:RetainedCloneGrant)->Result<(Option<(Dictionary,String)>,RetainedCloneProgress),EvaluationFailure>{
        self.normal_progress=Default::default();let result=self.step_original(maximum_units,maximum_bytes,grant);if let Ok((_,progress))=&result {self.normal_progress=*progress;}result
    }
    pub(super) fn step_original(&mut self,maximum_units:usize,maximum_bytes:usize,grant:RetainedCloneGrant)->Result<(Option<(Dictionary,String)>,RetainedCloneProgress),EvaluationFailure>{
        let empty=RetainedCloneProgress::default();
        if maximum_units==0||maximum_bytes==0||grant.maximum_items==0{return Ok((None,empty));}
        if self.cancelled{return Err("evaluation input admission canceled".into());}
        if !self.retirement.terminal_is_empty()||self.active.is_some()||self.closing_fields!=0{
            let receipt=self.close_intermediate(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;self.units=self.units.saturating_add(receipt.copied_items);return Ok((None,receipt));
        }
        match self.stage{
            0|4=>{
                let source=if self.stage==0{self.input.as_deref().unwrap()}else{self.dependency.as_deref().unwrap()};
                if source.len()>16*1024*1024{return Err("evaluation source exceeds admission limit".into());}
                let mut accepted=|_|true;let mut control=match self.decoding.take(){Some(receipt)=>semio_framework_value_native_owned::native_decoding::NativeDecodeControl::resume(receipt,&mut accepted),None=>Ok(semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut accepted))}.map_err(EvaluationFailure::from)?;
                let parsed=self.parser.as_mut().unwrap().step(source,1,&mut control,grant);let progress=self.parser.as_ref().unwrap().normal_step_progress();self.normal_progress=progress;self.decoding=Some(control.pause().map_err(EvaluationFailure::from)?);
                match parsed{
                    Ok(Some(value))=>{self.projection=Some(semio_framework_pack_json::JsonValueProjection::new_ordered(value));self.closing_fields=1|if self.stage==0{8}else{16};self.stage+=1;},
                    Ok(None)=>{},
                    Err(error) if self.stage==4&&error.kind()==ValueRefusalKind::InvalidValue=>{self.canonical_dependency=self.dependency.take();self.closing_fields=1|64;self.stage=7;if let semio_framework_pack_json::JsonError::DuplicateMember{name,..}=error{self.fault_member=Some(name);self.closing_fields|=128;}},
                    Err(error)=>return Err(error.into()),
                }
                self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));
            },
            1|5=>{
                let mut accepted=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::resume(self.decoding.take().unwrap(),&mut accepted).map_err(EvaluationFailure::from)?;
                let projected=self.projection.as_mut().unwrap().step(1,maximum_bytes,&mut control,grant);let progress=self.projection.as_ref().unwrap().normal_step_progress();self.normal_progress=progress;self.decoding=Some(control.pause().map_err(EvaluationFailure::from)?);
                if let Some(value)=projected.map_err(EvaluationFailure::from)?{self.writer=Some(semio_framework_pack_json::JsonWriteCursor::new(value));self.closing_fields=2|64;self.stage+=1;}
                self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));
            },
            2|6=>{let (text,progress)=self.write_step(grant)?;if let Some(text)=text{
                if self.stage==2{self.canonical_input=Some(text);self.binding=Some(neural_engine::retirement::RetainedDictionaryInput::new(self.writer.as_mut().unwrap().take_source().unwrap()));self.stage=3;}
                else{self.canonical_dependency=Some(text);self.stage=7;}
                self.closing_fields=4|32;
            }self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));},
            3=>{
                let binding=self.binding.as_mut().unwrap();let step=binding.step(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;
                if !step.progress.fits(grant){return Err("evaluation input binding exceeded caller grant".into());}
                if let Some(dictionary)=step.dictionary{assert!(binding.terminal_is_empty());self.binding=None;self.dictionary=Some(dictionary);self.parser=Some(semio_framework_pack_json::JsonParseCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject));self.stage=4;}
                self.units=self.units.saturating_add(step.progress.copied_items);return Ok((None,step.progress));
            },
            7=>{
                if self.writer.is_none(){
                    let bytes=4*size_of::<DslValue>();
                    if grant.maximum_items==0||grant.maximum_capacity_bytes<bytes{return Ok((None,empty));}
                    if grant.maximum_depth==0{return Err("evaluation identity birth exceeds admitted depth".into());}
                    let mut values=Vec::new();values.try_reserve_exact(4).map_err(|_|EvaluationFailure::Native(ValueError::literal(ValueRefusalKind::AllocationFailed,"evaluation identity allocation failed")))?;
                    values.extend([self.operator.take().unwrap(),self.canonical_input.take().unwrap(),self.canonical_dependency.take().unwrap(),self.version.take().unwrap()].map(DslValue::String));
                    let retained_capacity_bytes=values.capacity()*size_of::<DslValue>();self.writer=Some(semio_framework_pack_json::JsonWriteCursor::new(DslValue::Array(values)));
                    return Ok((None,RetainedCloneProgress {copied_items:1,retained_capacity_bytes,..empty}));
                }
                let (text,progress)=self.write_step(grant)?;if let Some(text)=text{self.identity=Some(text);self.closing_fields=4|32;self.stage=8;}self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));
            },
            8=>{self.stage=9;self.units=self.units.saturating_add(1);return Ok((Some((self.dictionary.take().unwrap(),self.identity.take().unwrap())),empty));},
            _=>return Ok((None,empty)),
        }
        self.units=self.units.saturating_add(1);Ok((None,empty))
    }
    /// 🛑️ Marks the current candidate for close without dropping a partial frontier.
    pub fn cancel(&mut self) {self.cancelled=true;if let Some(binding)=&mut self.binding {binding.cancel();}}
    /// 📏️ Borrows the next original field constructor or admitted child without moving it.
    pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        if !self.retirement.terminal_is_empty(){return evaluation_value_demands(&self.retirement,copy);}
        if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
        if let Some(binding)=&self.binding{return Ok(RetirementDemand {copy_bytes:binding.next_close_copy_byte_demand()?,capacity_bytes:binding.next_close_capacity_byte_demand(copy)?,release_bytes:binding.next_close_release_byte_demand()?,depth:binding.next_close_depth_demand()?});}
        if self.parser.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonParseCursor>());}
        if self.projection.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonValueProjection>());}
        if self.writer.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonWriteCursor<DslValue>>());}
        Ok(RetirementDemand {depth:usize::from(!self.terminal_is_empty()),..Default::default()})
    }
    /// ♻️ Preserves independent credits through original fields and separately admitted child shells.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        self.cancel();
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){self.stage=9;return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation input close exceeds supplied depth"));}
        if !self.retirement.terminal_is_empty(){let step=self.retirement.close_step(grant)?;return self.admit_close(grant,step);}
        if self.active.is_some(){let step=evaluation_child_close(&mut self.active,grant)?;return self.admit_close(grant,step);}
        if let Some(binding)=&mut self.binding {
            let step=binding.close_step(grant)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,binding.terminal_is_empty(),"evaluation input binding")?;
            if binding.terminal_is_empty(){self.binding=None;}
            return self.admit_close(grant,step);
        }
        if self.parser.is_some(){return evaluation_admit_original(&mut self.parser,&mut self.active,grant);}
        if self.projection.is_some(){return evaluation_admit_original(&mut self.projection,&mut self.active,grant);}
        if self.writer.is_some(){return evaluation_admit_original(&mut self.writer,&mut self.active,grant);}
        if self.encoding.is_some()||self.decoding.is_some(){
            if self.encoding.is_some(){self.encoding=None;}else{self.decoding=None;}
            return self.admit_close(grant,RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));
        }
        let progress=if let Some(value)=self.dictionary.take(){
            match self.retirement.push_dictionary(value,grant){Ok(progress)=>progress,Err((error,value))=>{self.dictionary=Some(value);return Err(error);}}
        }else{
            let source=if self.input.is_some(){&mut self.input}else if self.dependency.is_some(){&mut self.dependency}else if self.operator.is_some(){&mut self.operator}else if self.version.is_some(){&mut self.version}else if self.canonical_input.is_some(){&mut self.canonical_input}else if self.canonical_dependency.is_some(){&mut self.canonical_dependency}else if self.identity.is_some(){&mut self.identity}else{&mut self.fault_member};
            let value=source.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation input close has no original field"))?;
            match self.retirement.text(value,grant){Ok(progress)=>progress,Err((error,value))=>{*source=Some(value);return Err(error);}}
        };
        self.admit_close(grant,RetainedCloneStep::Progress(progress))
    }
    fn admit_close(&self,grant:RetainedCloneGrant,step:RetainedCloneStep)->Result<RetainedCloneStep,ValueError>{
        let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,RetainedCloneStep::Progress(step.progress()),false,"evaluation input parent")?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    /// 🔒️ Includes both opaque scalar continuations and the admitted original child shell.
    pub fn terminal_is_empty(&self)->bool {self.input.is_none()&&self.dependency.is_none()&&self.operator.is_none()&&self.version.is_none()&&self.parser.is_none()&&self.projection.is_none()&&self.writer.is_none()&&self.binding.is_none()&&self.dictionary.is_none()&&self.canonical_input.is_none()&&self.canonical_dependency.is_none()&&self.identity.is_none()&&self.fault_member.is_none()&&self.encoding.is_none()&&self.decoding.is_none()&&self.active.is_none()&&self.retirement.terminal_is_empty()}
}

impl Drop for EvaluationInputPreparation {fn drop(&mut self) {assert!(std::thread::panicking()||self.terminal_is_empty(),"evaluation input candidate dropped before terminal-empty");}}

struct EvaluationInputRetirement(EvaluationInputPreparation);

impl semio_framework_value::retirement::RetirementCursor for EvaluationInputRetirement {
        fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.0.retirement_demands(body) }
fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::RetirementStep;
        match self.0.close_step(grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}
    }
    fn terminal_is_empty(&self)->bool {self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands(0)?.copy_bytes)}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.retirement_demands(copy).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.retirement_demands(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands(0)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

impl RetireOwned for EvaluationInputPreparation {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {Box::new(EvaluationInputRetirement(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<EvaluationInputRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

/// ⏱️ Retained resumable evaluations keyed by `(operatorId, nodeHash)`. Bounded: a new job past the
/// ceiling refuses admission until an existing owner finishes retirement.
pub(super) const EVALUATION_JOB_CAPACITY: usize = 16;

pub enum EvaluationStep {Fault(EvaluationFailure),Working(neural_engine::OperatorProgress),Done(EvaluationReply),Cancelled(neural_engine::OperatorProgress)}

/// 📤️ Borrows inline envelope fields while retaining the same original native Dictionary.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct EvaluationEnvelopeSource {pub(super) output:Option<Dictionary>,pub(super) done:bool,pub(super) cancellable:bool,pub(super) phase:&'static str,pub(super) units_done:usize,pub(super) units_total:usize,pub(super) pending:Option<neural_engine::PendingExtensionEval>}

impl semio_framework_job::OriginalDomainProjection for EvaluationEnvelopeSource {
 fn projection_view(&self,path:&[usize])->Result<semio_framework_job::OriginalDomainView<'_>,ValueError>{
  use semio_framework_job::OriginalDomainView as View;
  if path.first()==Some(&2){return match &self.output{Some(output)=>semio_framework_job::OriginalDomainProjection::projection_view(output,&path[1..]),None if path.len()==1=>Ok(View::IntrinsicNull),_=>Err(evaluation_projection_error())}}
  Ok(match path{[]=>View::IntrinsicObject(if self.pending.is_some(){7}else{6}),[0]=>View::IntrinsicBool(self.cancellable),[1]=>View::IntrinsicBool(self.done),[3]=>View::IntrinsicText(self.phase),[4]=>View::IntrinsicNumber(semio_framework_value::Number::UInt(self.units_done as u64)),[5]=>View::IntrinsicNumber(semio_framework_value::Number::UInt(self.units_total as u64)),[6] if self.pending.is_some()=>View::IntrinsicObject(5),[6,index]=>{let original=self.pending.as_ref().ok_or_else(evaluation_projection_error)?;match index{0=>View::IntrinsicText(&original.extension_id),1=>View::IntrinsicText(&original.input_json),2=>View::IntrinsicText(&original.neuron_id),3=>View::IntrinsicNumber(semio_framework_value::Number::UInt(original.node_hash)),4=>View::IntrinsicText(&original.operator_id),_=>return Err(evaluation_projection_error())}},_=>return Err(evaluation_projection_error())})
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  if path.first()==Some(&2){return self.output.as_ref().ok_or_else(evaluation_projection_error).and_then(|output|semio_framework_job::OriginalDomainProjection::projection_key(output,&path[1..],index))}
  if path==[6]&&self.pending.is_some(){return ["extensionId","inputJson","neuronId","nodeHash","operatorId"].get(index).copied().ok_or_else(evaluation_projection_error)}
  if !path.is_empty(){return Err(evaluation_projection_error())}
  ["cancellable","done","output","phase","unitsDone","unitsTotal","pending"].get(index).copied().filter(|_|index<6||self.pending.is_some()).ok_or_else(evaluation_projection_error)
 }
}

fn evaluation_projection_error()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original evaluation Pack projection path is absent")}
// #endregion ⏱️EvaluationJobs
