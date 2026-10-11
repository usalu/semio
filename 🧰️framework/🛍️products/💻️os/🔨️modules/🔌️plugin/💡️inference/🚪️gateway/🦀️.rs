//! 🚪️ The registered inference gateway retains one original caller and whole decoded request.
use crate::app::*;
use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,NativeDecodeControl,NativeEncodeContinuation,NativeEncodeControl,RetirementDemand,retirement::RetireOwned,native_decoding::NativeDecodeContinuation};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_pack_json::{Value,Number,JsonGrammarCursor,JsonSourceCursor,JsonReadLimits,JsonMemberPolicy,JsonWriteCursor,JsonWriteSource,JsonWriteNode};

/// ⚠️ Original typed causes cross the recipient without display allocation.
#[derive(semio_framework_value::RetireOwned)]
pub enum ArtifactInferenceGatewayFailure{Native(ValueError),Json(semio_framework_pack_json::JsonError),Execution(ArtifactInferenceExecutionError),Literal(&'static str)}
impl From<ValueError> for ArtifactInferenceGatewayFailure{fn from(value:ValueError)->Self{Self::Native(value)}}

impl ArtifactInferenceGatewayFailure {
 pub(crate) fn fault_wire_fields(&self)->(&str,&str,RetainedCloneProgress){
  match self{
   Self::Native(error)=>(error.kind.as_str(),&error.message,error.retained_progress()),
   Self::Execution(error)=>(error.code,error.cause.as_ref().map_or(error.message.as_str(),|cause|cause.message.as_ref()),error.retained_progress),
   Self::Literal(message)=>("inference.gateway.refusal",message,Default::default()),
   Self::Json(error)=>{use semio_framework_pack_json::JsonError;match error{
    JsonError::Native(error)=>(error.kind.as_str(),&error.message,error.retained_progress()),
    JsonError::DuplicateMember{name,..}=>("json.duplicateMember",name,Default::default()),
    JsonError::UnexpectedEof=>("json.unexpectedEof","unexpectedEof",Default::default()),JsonError::UnexpectedByte{..}=>("json.unexpectedByte","unexpectedByte",Default::default()),JsonError::InvalidNumber(_)=>("json.invalidNumber","invalidNumber",Default::default()),JsonError::InvalidEscape(_)=>("json.invalidEscape","invalidEscape",Default::default()),JsonError::InvalidUnicodeEscape(_)=>("json.invalidUnicodeEscape","invalidUnicodeEscape",Default::default()),JsonError::UnpairedSurrogate(_)=>("json.unpairedSurrogate","unpairedSurrogate",Default::default()),JsonError::ControlCharacterInString{..}=>("json.controlCharacterInString","controlCharacterInString",Default::default()),JsonError::InvalidUtf8=>("json.invalidUtf8","invalidUtf8",Default::default()),JsonError::TrailingData(_)=>("json.trailingData","trailingData",Default::default()),JsonError::MaxDepthExceeded(_)=>("json.maxDepthExceeded","maxDepthExceeded",Default::default()),
   }}
  }
 }
 pub(crate) fn source_identity(&self)->usize{self.fault_wire_fields().1.as_ptr()as usize}
}

/// 🧾️ Absence retains the gateway source; a refusal transfers its original typed cause.
pub struct ArtifactInferenceGatewayStep{pub payload:Option<Vec<u8>>,pub retained_progress:RetainedCloneProgress,pub refusal:Option<ArtifactInferenceGatewayFailure>}

/// 📦️ The original request and execution are borrowed directly by the response codec.
#[derive(semio_framework_value::RetireOwned)]
struct InferenceEnvelopeSource{request:WireArtifactInferenceRequest,execution:ArtifactInferenceExecution}
const REQUEST_KEYS:[&str;20]=["wireVersion","owner","artifactKind","artifactSchema","artifactSchemaVersion","inferenceSchema","inferenceSchemaVersion","algorithmVersion","policyVersion","revision","generation","sourceDialect","policy","budgets","retained","cancellationId","previousState","requestedCacheMode","canonicalPayload","dependencies"];
const RESULT_KEYS:[&str;27]=["algorithmVersion","artifactKind","artifactSchema","artifactSchemaVersion","actualCacheMode","budgets","cancellationId","canonicalPayload","complete","dependencies","diagnostics","generation","inferenceSchema","inferenceSchemaVersion","owner","policy","policyVersion","previousState","provenance","quality","requestedCacheMode","retained","retirementProgress","revision","sourceDialect","validity","wireVersion"];
const GRANT_KEYS:[&str;5]=["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"];
const BUDGET_KEYS:[&str;3]=["allocationBytes","workUnits","recursionDepth"];
const RECEIPT_KEYS:[&str;4]=["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"];
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original inference wire field has a different declared type")}
fn uint(value:u64)->JsonWriteNode<'static>{JsonWriteNode::Number(semio_framework_value::Number::UInt(value))}
fn cache(value:&WireArtifactInferenceCacheMode)->&'static str{match value{WireArtifactInferenceCacheMode::Cold=>"cold",WireArtifactInferenceCacheMode::Incremental=>"incremental",WireArtifactInferenceCacheMode::Bypass=>"bypass"}}
impl JsonWriteSource for InferenceEnvelopeSource{
 fn node_at_path<'a>(&'a self,path:&[usize])->Result<JsonWriteNode<'a>,ValueError>{
  let r=&self.request;let e=&self.execution;
  let Some(field)=path.first()else{return if path.is_empty(){Ok(JsonWriteNode::Object(27))}else{Err(invalid())}};
  if path.len()==1{return Ok(match *field{
   0=>uint(u64::from(r.algorithm_version)),1=>JsonWriteNode::String(&r.artifact_kind),2=>JsonWriteNode::String(&r.artifact_schema),3=>uint(u64::from(r.artifact_schema_version)),4=>JsonWriteNode::String(cache(&e.actual_cache_mode)),5=>JsonWriteNode::Object(3),6=>JsonWriteNode::String(&r.cancellation_id),7=>e.canonical_payload.as_ref().map_or(JsonWriteNode::Null,|bytes|JsonWriteNode::Array(bytes.len())),8=>JsonWriteNode::Bool(e.complete),9=>JsonWriteNode::Array(r.dependencies.len()),10=>JsonWriteNode::Array(e.diagnostics.len()),11=>uint(r.generation),12=>JsonWriteNode::String(&r.inference_schema),13=>uint(u64::from(r.inference_schema_version)),14=>JsonWriteNode::String(&r.owner),15=>JsonWriteNode::Array(r.policy.len()),16=>uint(u64::from(r.policy_version)),17=>r.previous_state.as_ref().map_or(JsonWriteNode::Null,|bytes|JsonWriteNode::Array(bytes.len())),18=>JsonWriteNode::Object(5),19=>JsonWriteNode::String(&e.quality),20=>JsonWriteNode::String(cache(&r.requested_cache_mode)),21=>JsonWriteNode::Object(5),22=>JsonWriteNode::Object(4),23=>uint(r.revision),24=>JsonWriteNode::String(&r.source_dialect),25=>JsonWriteNode::String(&e.validity),26=>uint(u64::from(ARTIFACT_INFERENCE_WIRE_VERSION)),_=>return Err(invalid())});}
  match (*field,&path[1..]){
   (5,[index])=>Ok(uint(match index{0=>r.budgets.allocation_bytes,1=>r.budgets.work_units,2=>u64::from(r.budgets.recursion_depth),_=>return Err(invalid())})),
   (7,[index])=>e.canonical_payload.as_ref().and_then(|bytes|bytes.get(*index)).map(|byte|uint(u64::from(*byte))).ok_or_else(invalid),
   (15,[index])=>r.policy.get(*index).map(|byte|uint(u64::from(*byte))).ok_or_else(invalid),
   (17,[index])=>r.previous_state.as_ref().and_then(|bytes|bytes.get(*index)).map(|byte|uint(u64::from(*byte))).ok_or_else(invalid),
   (18,[index])=>Ok(match index{0=>JsonWriteNode::String(&r.owner),1=>JsonWriteNode::String(&r.inference_schema),2=>uint(u64::from(r.algorithm_version)),3=>uint(u64::from(r.policy_version)),4=>JsonWriteNode::String(&r.source_dialect),_=>return Err(invalid())}),
   (21,[index])=>Ok(uint(match index{0=>r.retained.maximum_items as u64,1=>r.retained.maximum_copy_bytes as u64,2=>r.retained.maximum_capacity_bytes as u64,3=>r.retained.maximum_release_bytes as u64,4=>r.retained.maximum_depth as u64,_=>return Err(invalid())})),
   (22,[index])=>Ok(uint(match index{0=>e.retirement_progress.copied_items as u64,1=>e.retirement_progress.copied_bytes as u64,2=>e.retirement_progress.retained_capacity_bytes as u64,3=>e.retirement_progress.released_bytes as u64,_=>return Err(invalid())})),
   (9,[index])=>r.dependencies.get(*index).map(|_|JsonWriteNode::Array(2)).ok_or_else(invalid),
   (9,[index,0])=>r.dependencies.get(*index).map(|(key,_)|JsonWriteNode::String(key)).ok_or_else(invalid),
   (9,[index,1])=>r.dependencies.get(*index).map(|(_,bytes)|JsonWriteNode::Array(bytes.len())).ok_or_else(invalid),
   (9,[index,1,byte])=>r.dependencies.get(*index).and_then(|(_,bytes)|bytes.get(*byte)).map(|byte|uint(u64::from(*byte))).ok_or_else(invalid),
   (10,[index])=>e.diagnostics.get(*index).map(|_|JsonWriteNode::Object(4)).ok_or_else(invalid),
   (10,[index,field])=>{let d=e.diagnostics.get(*index).ok_or_else(invalid)?;Ok(match field{0=>JsonWriteNode::String(&d.code),1=>JsonWriteNode::String(&d.message),2=>JsonWriteNode::String(&d.severity),3=>JsonWriteNode::Object(d.parameters.len()),_=>return Err(invalid())})},
   (10,[index,3,parameter])=>e.diagnostics.get(*index).and_then(|d|d.parameters.values().nth(*parameter)).map(|value|JsonWriteNode::String(value)).ok_or_else(invalid),
   _=>Err(invalid()),
  }
 }
 fn object_key_at_path<'a>(&'a self,path:&[usize],index:usize)->Result<&'a str,ValueError>{
  match path{
   []=>RESULT_KEYS.get(index).copied().filter(|_|index<27).ok_or_else(invalid),[5]=>BUDGET_KEYS.get(index).copied().ok_or_else(invalid),[18]=>["owner","inferenceSchema","algorithmVersion","policyVersion","sourceDialect"].get(index).copied().ok_or_else(invalid),[21]=>GRANT_KEYS.get(index).copied().ok_or_else(invalid),[22]=>RECEIPT_KEYS.get(index).copied().ok_or_else(invalid),[10,_]=>["code","message","severity","parameters"].get(index).copied().ok_or_else(invalid),[10,d,3]=>self.execution.diagnostics.get(*d).and_then(|d|d.parameters.keys().nth(index)).map(String::as_str).ok_or_else(invalid),_=>Err(invalid())
  }
 }
}

/// 🪪️ A single resource slot pins the original raw caller and typed request through publication.
pub struct ArtifactInferenceGateway{
 operation:u64,generation:u64,pointer:usize,length:usize,stage:u8,
 parser:Option<JsonGrammarCursor<Value>>,decoding:Option<NativeDecodeContinuation>,root:Option<Value>,
 request:Option<WireArtifactInferenceRequest>,field:usize,required:u32,nested:usize,position:usize,born:bool,dependency:usize,
 normal_progress:RetainedCloneProgress,discarded_execution:Option<ArtifactInferenceExecution>,writer:Option<JsonWriteCursor<InferenceEnvelopeSource>>,encoding:Option<NativeEncodeContinuation>,wire:Option<String>,active:Option<Box<dyn ErasedSnapshotRetirement>>,child_live:bool,closing:bool,
}
impl ArtifactInferenceGateway{
 pub fn new(operation:u64,generation:u64,source:&[u8])->Self{Self{operation,generation,pointer:source.as_ptr()as usize,length:source.len(),stage:0,parser:None,decoding:None,root:None,request:None,field:0,required:0,nested:0,position:0,born:false,dependency:0,normal_progress:Default::default(),discarded_execution:None,writer:None,encoding:None,wire:None,active:None,child_live:false,closing:false}}
 pub fn matches(&self,operation:u64,generation:u64,source:&[u8])->bool{self.operation==operation&&self.generation==generation&&self.pointer==source.as_ptr()as usize&&self.length==source.len()}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn child_closed(&mut self){self.child_live=false;}
 pub fn terminal_is_empty(&self)->bool{!self.child_live&&self.parser.is_none()&&self.decoding.is_none()&&self.root.is_none()&&self.request.is_none()&&self.discarded_execution.is_none()&&self.writer.is_none()&&self.encoding.is_none()&&self.wire.is_none()&&self.active.is_none()}
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(active)=&self.active{return semio_framework_value::factory_ticket_demands(active,copy);}
  if self.parser.is_some(){return Ok(birth::<JsonGrammarCursor<Value>>());}if self.root.is_some(){return Ok(birth::<Value>());}if self.discarded_execution.is_some(){return Ok(birth::<ArtifactInferenceExecution>());}if self.request.is_some(){return Ok(birth::<WireArtifactInferenceRequest>());}if self.writer.is_some(){return Ok(birth::<JsonWriteCursor<InferenceEnvelopeSource>>());}if self.wire.is_some(){return Ok(birth::<String>());}Ok(RetirementDemand{depth:usize::from(self.decoding.is_some()||self.encoding.is_some()),..Default::default()})
 }
 pub fn retirement_demands_with_context(&self,context:Option<&dyn std::any::Any>,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.child_live||self.active.is_some()||self.discarded_execution.is_some(){return self.retirement_demands(copy)}
  let request=self.request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"live original inference child lost its request"))?;
  let service=crate::app::artifact_inference_service(&request.artifact_kind,&request.inference_schema)?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"live original inference child lost its registered service"))?;
  let view=ArtifactInferenceExecutionRequest{operation:self.operation,generation:self.generation,cancelled:true,policy:&request.policy,budgets:&request.budgets,retained:request.retained,cancellation_id:&request.cancellation_id,previous_state:request.previous_state.as_deref(),requested_cache_mode:request.requested_cache_mode.clone(),canonical_payload:&request.canonical_payload,dependencies:&request.dependencies};
  service.retirement_demands(&view,context,copy)
 }
 fn close_discarded_execution(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.active.is_some(){semio_framework_value::close_factory_ticket(&mut self.active,grant)}else{admit(&mut self.discarded_execution,&mut self.active,grant)}
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.active.is_some()||self.discarded_execution.is_some(){return self.close_discarded_execution(grant);}
  if self.child_live{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.active.is_some(){return semio_framework_value::close_factory_ticket(&mut self.active,grant);}
  if self.parser.is_some(){return admit(&mut self.parser,&mut self.active,grant);}if self.root.is_some(){return admit(&mut self.root,&mut self.active,grant);}if self.discarded_execution.is_some(){return admit(&mut self.discarded_execution,&mut self.active,grant);}if self.request.is_some(){return admit(&mut self.request,&mut self.active,grant);}if self.writer.is_some(){return admit(&mut self.writer,&mut self.active,grant);}if self.wire.is_some(){return admit(&mut self.wire,&mut self.active,grant);}
  if self.decoding.is_some()||self.encoding.is_some(){if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original inference continuation requires depth"));}if self.decoding.is_some(){self.decoding=None;}else{self.encoding=None;}return Ok(RetainedCloneStep::Progress(item()));}Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn bytes(source:&Value,destination:&mut Vec<u8>,position:&mut usize,born:&mut bool,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
  let Value::Array(values)=source else{return Err(invalid());};
  if !*born{if grant.maximum_capacity_bytes<values.len(){return Ok((false,Default::default()));}destination.try_reserve_exact(values.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"inference original byte destination allocation failed"))?;*born=true;return Ok((false,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:destination.capacity(),..Default::default()}));}
  let count=grant.maximum_copy_bytes.min(values.len().saturating_sub(*position));if count==0&&*position!=values.len(){return Ok((false,Default::default()));}
  let before=destination.len();
  for value in &values[*position..*position+count]{let Value::Number(Number::UInt(byte))=value else{return Err(invalid().with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:destination.len()-before,..Default::default()}));};let byte=u8::try_from(*byte).map_err(|_|invalid().with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:destination.len()-before,..Default::default()}))?;destination.push(byte);}
  *position+=count;Ok((*position==values.len(),RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()}))
 }
 fn decode(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if grant.maximum_depth<2{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original inference object requires admitted depth"));}
  if self.request.is_none(){self.request=Some(WireArtifactInferenceRequest{wire_version:0,owner:String::new(),artifact_kind:String::new(),artifact_schema:String::new(),artifact_schema_version:0,inference_schema:String::new(),inference_schema_version:0,algorithm_version:0,policy_version:0,revision:0,generation:0,source_dialect:String::new(),policy:Vec::new(),budgets:WireArtifactInferenceBudget{allocation_bytes:0,work_units:0,recursion_depth:0},retained:grant,cancellation_id:String::new(),previous_state:None,requested_cache_mode:WireArtifactInferenceCacheMode::Cold,canonical_payload:Vec::new(),dependencies:Vec::new()});return Ok(item());}
  let Value::Object(root)=self.root.as_mut().ok_or_else(invalid)?else{return Err(invalid());};
  let Some((key,value))=root.iter_mut().nth(self.field)else{if self.required!=(1u32<<20)-1{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original inference request requires all declared fields"));}self.stage=2;return Ok(item());};
  let index=REQUEST_KEYS.iter().position(|name|*name==key).ok_or_else(invalid)?;let r=self.request.as_mut().unwrap();
  let destination=match index{1=>Some(&mut r.owner),2=>Some(&mut r.artifact_kind),3=>Some(&mut r.artifact_schema),5=>Some(&mut r.inference_schema),11=>Some(&mut r.source_dialect),15=>Some(&mut r.cancellation_id),_=>None};
  if let Some(destination)=destination{if !matches!(value,Value::String(_)){return Err(invalid());}let Value::String(original)=std::mem::replace(value,Value::Null)else{unreachable!()};*destination=original;}
  else if matches!(index,12|16|18){
   if index==16&&matches!(value,Value::Null){}else{let destination=match index{12=>&mut r.policy,18=>&mut r.canonical_payload,_=>r.previous_state.get_or_insert_with(Vec::new)};let (done,receipt)=Self::bytes(value,destination,&mut self.position,&mut self.born,grant)?;if !done{return Ok(receipt);}self.position=0;self.born=false;self.required|=1<<index;self.field+=1;return Ok(receipt);}
  }else if index==13||index==14{
   let Value::Object(object)=value else{return Err(invalid());};let Some((axis,value))=object.iter().nth(self.nested)else{if self.nested!=if index==13{3}else{5}{return Err(invalid());}self.nested=0;self.required|=1<<index;self.field+=1;return Ok(item());};let Value::Number(Number::UInt(number))=value else{return Err(invalid());};
   if index==13{match axis{"allocationBytes"=>r.budgets.allocation_bytes=*number,"workUnits"=>r.budgets.work_units=*number,"recursionDepth"=>r.budgets.recursion_depth=u32::try_from(*number).map_err(|_|invalid())?,_=>return Err(invalid())}}
   else{let number=usize::try_from(*number).map_err(|_|invalid())?;match axis{"maximumItems"=>r.retained.maximum_items=number,"maximumCopyBytes"=>r.retained.maximum_copy_bytes=number,"maximumCapacityBytes"=>r.retained.maximum_capacity_bytes=number,"maximumReleaseBytes"=>r.retained.maximum_release_bytes=number,"maximumDepth"=>r.retained.maximum_depth=number,_=>return Err(invalid())}}
   self.nested+=1;return Ok(item());
  }else if index==17{let Value::String(mode)=value else{return Err(invalid());};r.requested_cache_mode=match mode.as_str(){"cold"=>WireArtifactInferenceCacheMode::Cold,"incremental"=>WireArtifactInferenceCacheMode::Incremental,"bypass"=>WireArtifactInferenceCacheMode::Bypass,_=>return Err(invalid())};}
  else if index==19{
   let Value::Array(rows)=value else{return Err(invalid());};
   if r.dependencies.capacity()==0&&!rows.is_empty(){let bytes=rows.len().checked_mul(std::mem::size_of::<(String,Vec<u8>)>()).ok_or_else(invalid)?;if grant.maximum_capacity_bytes<bytes{return Ok(Default::default());}r.dependencies.try_reserve_exact(rows.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original dependency rows allocation failed"))?;return Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:r.dependencies.capacity()*std::mem::size_of::<(String,Vec<u8>)>(),..Default::default()});}
   if let Some(row)=rows.get_mut(self.dependency){let Value::Array(fields)=row else{return Err(invalid());};if fields.len()!=2{return Err(invalid());}
    if r.dependencies.len()==self.dependency{if !matches!(fields[0],Value::String(_)){return Err(invalid());}let Value::String(original)=std::mem::replace(&mut fields[0],Value::Null)else{unreachable!()};r.dependencies.push((original,Vec::new()));return Ok(item());}
    let (done,receipt)=Self::bytes(&fields[1],&mut r.dependencies[self.dependency].1,&mut self.position,&mut self.born,grant)?;if done{self.dependency+=1;self.position=0;self.born=false;}return Ok(receipt);
   }
  }else{let Value::Number(Number::UInt(number))=value else{return Err(invalid());};match index{0=>r.wire_version=u32::try_from(*number).map_err(|_|invalid())?,4=>r.artifact_schema_version=u32::try_from(*number).map_err(|_|invalid())?,6=>r.inference_schema_version=u32::try_from(*number).map_err(|_|invalid())?,7=>r.algorithm_version=u32::try_from(*number).map_err(|_|invalid())?,8=>r.policy_version=u32::try_from(*number).map_err(|_|invalid())?,9=>r.revision=*number,10=>r.generation=*number,_=>return Err(invalid())}}
  self.required|=1<<index;self.field+=1;Ok(item())
 }
 fn advance(&mut self,source:&[u8],grant:RetainedCloneGrant,context:Option<&dyn std::any::Any>,cancelled:bool)->Result<(Option<Vec<u8>>,RetainedCloneProgress),ArtifactInferenceGatewayFailure>{
  self.normal_progress=Default::default();
  if cancelled{self.closing=true;}
  if self.closing&&(self.active.is_some()||self.discarded_execution.is_some()){return self.close_discarded_execution(grant).map(|step|(None,step.progress())).map_err(Into::into);}
  if self.closing&&!self.child_live{return self.close_step(grant).map(|step|(None,step.progress())).map_err(Into::into);}
  if self.stage==0{
   if self.parser.is_none(){let cursor=JsonSourceCursor::<_,Value>::new(source,JsonMemberPolicy::Reject,JsonReadLimits{maximum_bytes:source.len()as u64,maximum_allocation_bytes:grant.maximum_capacity_bytes,maximum_depth:grant.maximum_depth,maximum_items:source.len()as u64})?;self.parser=Some(cursor.into_grammar().1);return Ok((None,item()));}
   let mut callback=|_|!cancelled;let mut control=match self.decoding.take(){Some(original)=>NativeDecodeControl::resume(original,&mut callback)?,None=>NativeDecodeControl::new_retained(&mut callback)};let mut cursor=JsonSourceCursor::from_grammar(source,self.parser.take().unwrap());let result=cursor.step(1,&mut control,grant);let receipt=cursor.normal_step_progress();self.normal_progress=receipt;self.parser=Some(cursor.into_grammar().1);self.decoding=Some(control.pause()?);match result{Ok(Some(original))=>{self.root=Some(original);self.stage=1;},Ok(None)=>{},Err(error)=>return Err(ArtifactInferenceGatewayFailure::Json(error))}return Ok((None,receipt));
  }
  if self.stage==1{return self.decode(grant).map(|receipt|(None,receipt)).map_err(Into::into);}
  if self.stage==2{
   let step=if self.active.is_some(){semio_framework_value::close_factory_ticket(&mut self.active,grant)?}else if self.parser.is_some(){admit(&mut self.parser,&mut self.active,grant)?}else if self.root.is_some(){admit(&mut self.root,&mut self.active,grant)?}else if self.decoding.is_some(){self.decoding=None;RetainedCloneStep::Progress(item())}else{self.stage=3;RetainedCloneStep::Progress(item())};return Ok((None,step.progress()));
  }
  if self.stage==3{
   let request=self.request.as_ref().unwrap();let service=crate::app::artifact_inference_service(&request.artifact_kind,&request.inference_schema).map_err(ArtifactInferenceGatewayFailure::Native)?.ok_or(ArtifactInferenceGatewayFailure::Literal("original inference service is not registered"))?;
   if request.wire_version!=ARTIFACT_INFERENCE_WIRE_VERSION{return Err(ArtifactInferenceGatewayFailure::Literal("original inference wire version differs"));}
   let metadata=service.metadata();if request.owner!=metadata.owner||request.artifact_kind!=metadata.artifact_kind||request.artifact_schema!=metadata.artifact_schema||request.artifact_schema_version!=metadata.artifact_schema_version||request.inference_schema!=metadata.inference_schema||request.inference_schema_version!=metadata.inference_schema_version||request.algorithm_version!=metadata.algorithm_version||request.policy_version!=metadata.policy_version{return Err(ArtifactInferenceGatewayFailure::Literal("original inference request identity differs from its registered service"));}
   let retained=if self.closing{grant}else{intersect(grant,request.retained)};let view=ArtifactInferenceExecutionRequest{operation:self.operation,generation:self.generation,cancelled:cancelled||self.closing,policy:&request.policy,budgets:&request.budgets,retained,cancellation_id:&request.cancellation_id,previous_state:request.previous_state.as_deref(),requested_cache_mode:request.requested_cache_mode.clone(),canonical_payload:&request.canonical_payload,dependencies:&request.dependencies};
   let result=match context{Some(context)=>service.infer_with_context(&view,context),None=>service.infer(&view)}.map_err(|error|{self.normal_progress=error.retained_progress;self.child_live=!error.terminal;ArtifactInferenceGatewayFailure::Execution(error)})?;
   let receipt=result.retained_progress;self.normal_progress=receipt;
   self.child_live=!result.terminal;
   let Some(execution)=result.execution else{if result.terminal&&!self.closing{return Err(ArtifactInferenceGatewayFailure::Literal("terminal original inference callback has no result"));}return Ok((None,receipt));};
   if !result.terminal{self.discarded_execution=Some(execution);return Err(ArtifactInferenceGatewayFailure::Literal("original inference callback retains a live child before response publication"));}
   if self.closing{self.discarded_execution=Some(execution);return Ok((None,receipt));}
   self.writer=Some(JsonWriteCursor::new(InferenceEnvelopeSource{request:self.request.take().unwrap(),execution}));
   if !receipt.fits(retained){return Err(ArtifactInferenceGatewayFailure::Literal("original inference callback exceeded remaining authority"));}
   self.stage=4;return Ok((None,receipt));
  }
  if self.stage==4{
   let mut callback=|_|!cancelled;let mut control=match self.encoding.take(){Some(original)=>NativeEncodeControl::resume(original,&mut callback)?,None=>NativeEncodeControl::new_retained(&mut callback)};let writer=self.writer.as_mut().unwrap();let result=writer.step(1,&mut control,grant);let receipt=writer.normal_step_progress();self.normal_progress=receipt;self.encoding=Some(control.pause()?);match result{Ok(Some(original))=>{self.wire=Some(original);self.stage=5;},Ok(None)=>{},Err(error)=>return Err(error.into())}return Ok((None,receipt));
  }
  if self.stage==5{
   if self.active.is_some(){return semio_framework_value::close_factory_ticket(&mut self.active,grant).map(|step|(None,step.progress())).map_err(Into::into);}
   if self.writer.is_some(){return admit(&mut self.writer,&mut self.active,grant).map(|step|(None,step.progress())).map_err(Into::into);}
   if self.encoding.is_some(){self.encoding=None;return Ok((None,item()));}self.stage=6;return Ok((self.wire.take().map(String::into_bytes),item()));
  }
  Ok((None,Default::default()))
 }
 pub fn step(&mut self,source:&[u8],cx:&mut semio_framework_job::StepContext<'_>,context:Option<&dyn std::any::Any>)->ArtifactInferenceGatewayStep{
  let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield(){return ArtifactInferenceGatewayStep{payload:None,retained_progress:Default::default(),refusal:None};}
  if !self.matches(cx.operation().0,cx.generation().0,source){return ArtifactInferenceGatewayStep{payload:None,retained_progress:Default::default(),refusal:Some(ArtifactInferenceGatewayFailure::Literal("original inference gateway retains another caller"))};}
  let (payload,receipt,mut refusal)=match self.advance(source,grant,context,cx.is_cancelled()){Ok((payload,receipt))=>(payload,receipt,None),Err(error)=>{self.closing=true;let receipt=if self.normal_progress!=Default::default(){self.normal_progress}else{match &error{ArtifactInferenceGatewayFailure::Native(error)=>error.retained_progress(),_=>Default::default()}};(None,receipt,Some(error))}};
  if let Err(error)=cx.consume_retained(receipt){if refusal.is_none(){refusal=Some(error.into());}}cx.consume_fuel(1);ArtifactInferenceGatewayStep{payload,retained_progress:receipt,refusal}
 }
}
fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn birth<T:RetireOwned>()->RetirementDemand{RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()}}
fn admit<T:RetireOwned>(source:&mut Option<T>,active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_items==0||grant.maximum_capacity_bytes<birth::<T>().capacity_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original inference handoff requires depth"));}let original=source.take().ok_or_else(invalid)?;match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,receipt))=>{*active=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*source=Some(original);Err(error)}}}
fn intersect(a:RetainedCloneGrant,b:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:a.maximum_items.min(b.maximum_items),maximum_copy_bytes:a.maximum_copy_bytes.min(b.maximum_copy_bytes),maximum_capacity_bytes:a.maximum_capacity_bytes.min(b.maximum_capacity_bytes),maximum_release_bytes:a.maximum_release_bytes.min(b.maximum_release_bytes),maximum_depth:a.maximum_depth.min(b.maximum_depth)}}

/// 📨️ Advances the original gateway through the current retained extension context.
pub fn wire_artifact_infer(gateway:&mut ArtifactInferenceGateway,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->ArtifactInferenceGatewayStep{match crate::plugin_runtime::with_original_extension_inference_context(|context|Ok(gateway.step(request,cx,context))){Ok(step)=>step,Err(error)=>ArtifactInferenceGatewayStep{payload:None,retained_progress:error.retained_progress(),refusal:Some(error.into())}}}

/// ♻️ Borrows the same registered child without creating or spending a wallet.
pub fn original_inference_gateway_demands(gateway:&ArtifactInferenceGateway,copy:usize)->Result<RetirementDemand,ValueError>{crate::plugin_runtime::with_original_extension_inference_context(|context|gateway.retirement_demands_with_context(context,copy))}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
