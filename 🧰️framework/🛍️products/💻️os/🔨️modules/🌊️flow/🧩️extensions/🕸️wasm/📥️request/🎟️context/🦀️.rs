//! 🎟️ The named callback prepares the same decoded request under original outer authority.
use super::*;
use semio_framework_pack_json::{JsonBorrowedWriteCursor,JsonWriteNode,JsonWriteSource};
use semio_framework_value::{DslValue as Value,Number,NativeEncodeControl,NativeEncodeContinuation};

/// 🫴️ Borrows the actual retained gateway source for one paid preparation turn.
pub struct EvaluationRequestContextInput<'a>{pub cancellation_id:&'a str,pub policy:&'a[u8],pub dependencies:&'a[(String,Vec<u8>)],pub work_units:u64,pub wait_terminal:bool}

struct DependencySource<'a>{original:&'a str,value:Option<&'a Value>,policy:&'a[u8],dependencies:&'a[(String,Vec<u8>)]}
impl JsonWriteSource for DependencySource<'_>{
 fn node_at_path<'a>(&'a self,path:&[usize])->Result<JsonWriteNode<'a>,ValueError>{
  if !self.policy.is_empty(){match path{[]=>return Ok(JsonWriteNode::Object(2)),[1]=>return Ok(JsonWriteNode::Array(self.policy.len())),[1,index]=>return self.policy.get(*index).map(|byte|JsonWriteNode::Number(Number::UInt(u64::from(*byte)))).ok_or_else(invalid),[0,..]=>{},_=>return Err(invalid())}}
  let path=if self.policy.is_empty(){path}else{&path[1..]};
  if !self.dependencies.is_empty(){match path{[]=>return Ok(JsonWriteNode::Object(2)),[1]=>return Ok(JsonWriteNode::Array(self.dependencies.len())),[1,index]=>return self.dependencies.get(*index).map(|_|JsonWriteNode::Object(2)).ok_or_else(invalid),[1,index,0]=>return self.dependencies.get(*index).map(|(owner,_)|JsonWriteNode::String(owner)).ok_or_else(invalid),[1,index,1]=>return self.dependencies.get(*index).map(|(_,bytes)|JsonWriteNode::Array(bytes.len())).ok_or_else(invalid),[1,index,1,byte]=>return self.dependencies.get(*index).and_then(|(_,bytes)|bytes.get(*byte)).map(|byte|JsonWriteNode::Number(Number::UInt(u64::from(*byte)))).ok_or_else(invalid),[0,..]=>{},_=>return Err(invalid())}}
  let path=if self.dependencies.is_empty(){path}else{&path[1..]};match self.value{Some(original)=>original.node_at_path(path),None if path.is_empty()=>Ok(JsonWriteNode::String(self.original)),None=>Err(invalid())}
 }
 fn object_key_at_path<'a>(&'a self,path:&[usize],index:usize)->Result<&'a str,ValueError>{
  if !self.policy.is_empty(){if path.is_empty(){return ["source","policy"].get(index).copied().ok_or_else(invalid)}if path.first()!=Some(&0){return Err(invalid())}}
  let path=if self.policy.is_empty(){path}else{&path[1..]};
  if !self.dependencies.is_empty(){match path{[]=>return ["source","dependencies"].get(index).copied().ok_or_else(invalid),[1,_]=>return ["owner","payload"].get(index).copied().ok_or_else(invalid),[0,..]=>{},_=>return Err(invalid())}}
  let path=if self.dependencies.is_empty(){path}else{&path[1..]};self.value.ok_or_else(invalid)?.object_key_at_path(path,index)
 }
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original geometry dependency source differs from its declared shape")}
fn structural()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}

pub(super) struct EvaluationRequestContext{
 stage:u8,position:usize,header:Option<Vec<u8>>,original_cancellation:Option<String>,original_dependency:Option<String>,replacement:Option<String>,
 parser:Option<JsonGrammarCursor<Value>>,value:Option<Value>,decoding:Option<NativeDecodeContinuation>,writer:Option<JsonBorrowedWriteCursor>,encoding:Option<NativeEncodeContinuation>,
 active:Option<Box<dyn ErasedSnapshotRetirement>>,normal_progress:RetainedCloneProgress,
}
impl EvaluationRequestContext{
 pub fn new()->Self{Self{stage:0,position:0,header:None,original_cancellation:None,original_dependency:None,replacement:None,parser:None,value:None,decoding:None,writer:None,encoding:None,active:None,normal_progress:Default::default()}}
 pub fn terminal_is_empty(&self)->bool{self.header.is_none()&&self.original_cancellation.is_none()&&self.original_dependency.is_none()&&self.replacement.is_none()&&self.parser.is_none()&&self.value.is_none()&&self.decoding.is_none()&&self.writer.is_none()&&self.encoding.is_none()&&self.active.is_none()}
 pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.active.is_some(){return evaluation_child_demands(&self.active,copy)}
  if self.writer.is_some(){return Ok(evaluation_owned_birth::<JsonBorrowedWriteCursor>())}if self.parser.is_some(){return Ok(evaluation_owned_birth::<JsonGrammarCursor<Value>>())}if self.value.is_some(){return Ok(evaluation_owned_birth::<Value>())}
  if self.original_cancellation.is_some()||self.original_dependency.is_some()||self.replacement.is_some(){return Ok(evaluation_owned_birth::<String>())}if self.header.is_some(){return Ok(evaluation_owned_birth::<Vec<u8>>())}Ok(RetirementDemand{depth:usize::from(self.decoding.is_some()||self.encoding.is_some()),..Default::default()})
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if self.active.is_some(){return evaluation_child_close(&mut self.active,grant)}
  if self.writer.is_some(){return evaluation_admit_original(&mut self.writer,&mut self.active,grant)}if self.parser.is_some(){return evaluation_admit_original(&mut self.parser,&mut self.active,grant)}if self.value.is_some(){return evaluation_admit_original(&mut self.value,&mut self.active,grant)}
  if self.original_cancellation.is_some(){return evaluation_admit_original(&mut self.original_cancellation,&mut self.active,grant)}if self.original_dependency.is_some(){return evaluation_admit_original(&mut self.original_dependency,&mut self.active,grant)}if self.replacement.is_some(){return evaluation_admit_original(&mut self.replacement,&mut self.active,grant)}if self.header.is_some(){return evaluation_admit_original(&mut self.header,&mut self.active,grant)}
  if self.decoding.is_some()||self.encoding.is_some(){if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original geometry continuation requires admitted depth"))}if self.decoding.is_some(){self.decoding=None}else{self.encoding=None}return Ok(RetainedCloneStep::Progress(structural()))}Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_progress}
 pub fn step(&mut self,payload:&mut EvaluateRequest,input:EvaluationRequestContextInput<'_>,grant:RetainedCloneGrant,cancelled:bool)->Result<bool,EvaluationFailure>{
  self.normal_progress=Default::default();let result=self.advance(payload,input,grant,cancelled);if let Err(EvaluationFailure::Native(error))=&result{if self.normal_progress==Default::default(){self.normal_progress=error.retained_progress();}}result.map_err(|error|match error{EvaluationFailure::Native(error)=>EvaluationFailure::Native(error.with_retained_progress(self.normal_progress)),error=>error})
 }
 fn advance(&mut self,payload:&mut EvaluateRequest,input:EvaluationRequestContextInput<'_>,grant:RetainedCloneGrant,cancelled:bool)->Result<bool,EvaluationFailure>{
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  match self.stage{
   0=>{if grant.maximum_capacity_bytes<input.cancellation_id.len(){return Ok(false)}let mut header=Vec::new();header.try_reserve_exact(input.cancellation_id.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original cancellation destination allocation failed"))?;let capacity=header.capacity();self.header=Some(header);self.stage=1;self.normal_progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()};},
   1=>{let count=grant.maximum_copy_bytes.min(input.cancellation_id.len().saturating_sub(self.position));if count==0&&self.position!=input.cancellation_id.len(){return Ok(false)}self.header.as_mut().unwrap().extend_from_slice(&input.cancellation_id.as_bytes()[self.position..self.position+count]);self.position+=count;if self.position==input.cancellation_id.len(){self.stage=2}self.normal_progress=RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()};},
   2=>{if self.original_cancellation.is_some()||self.active.is_some(){self.normal_progress=self.close_step(grant)?.progress();return Ok(false)}let header=self.header.take().unwrap();self.original_cancellation=Some(std::mem::replace(&mut payload.cancellation_id,String::from_utf8(header).expect("original caller cancellation is UTF8")));self.stage=3;self.normal_progress=structural();},
   3=>{if self.original_cancellation.is_some()||self.active.is_some(){self.normal_progress=self.close_step(grant)?.progress();return Ok(false)}if input.policy.is_empty()&&input.dependencies.is_empty(){self.stage=7;self.normal_progress=structural();return Ok(false)}
    if self.parser.is_none(){let first=payload.dependency_json.bytes().find(|byte|!byte.is_ascii_whitespace());let json=first.is_some_and(|byte|matches!(byte,b'{'|b'['|b'"'|b'-'|b'0'..=b'9'|b't'|b'f'|b'n'));if !json{self.stage=4;self.normal_progress=structural();return Ok(false)}let cursor=JsonSourceCursor::<_,Value>::new(payload.dependency_json.as_bytes(),JsonMemberPolicy::Reject,JsonReadLimits{maximum_bytes:payload.dependency_json.len()as u64,maximum_allocation_bytes:grant.maximum_capacity_bytes,maximum_items:payload.dependency_json.len()as u64,maximum_depth:grant.maximum_depth})?;self.parser=Some(cursor.into_grammar().1);self.normal_progress=structural();return Ok(false)}
    let mut callback=|_|!cancelled;let mut control=match self.decoding.take(){Some(original)=>NativeDecodeControl::resume(original,&mut callback)?,None=>NativeDecodeControl::new_retained(&mut callback)};let mut cursor=JsonSourceCursor::from_grammar(payload.dependency_json.as_bytes(),self.parser.take().unwrap());let result=cursor.step(1,&mut control,grant);self.normal_progress=cursor.normal_step_progress();self.parser=Some(cursor.into_grammar().1);self.decoding=Some(control.pause()?);if let Some(original)=result?{self.value=Some(original);self.stage=4;}
   },
   4=>{if self.writer.is_none(){self.writer=Some(JsonBorrowedWriteCursor::new());self.normal_progress=structural();return Ok(false)}let source=DependencySource{original:&payload.dependency_json,value:self.value.as_ref(),policy:input.policy,dependencies:input.dependencies};let mut callback=|_|!cancelled;let mut control=match self.encoding.take(){Some(original)=>NativeEncodeControl::resume(original,&mut callback)?,None=>NativeEncodeControl::new_retained(&mut callback)};let writer=self.writer.as_mut().unwrap();let result=writer.step(&source,1,&mut control,grant);self.normal_progress=writer.normal_step_progress();self.encoding=Some(control.pause()?);if let Some(original)=result?{self.replacement=Some(original);self.stage=5;}},
   5=>{self.original_dependency=Some(std::mem::replace(&mut payload.dependency_json,self.replacement.take().unwrap()));self.stage=6;self.normal_progress=structural();},
   6=>{if !self.terminal_is_empty(){self.normal_progress=self.close_step(grant)?.progress();return Ok(false)}self.stage=7;self.normal_progress=structural();},
   7=>{payload.budget=input.work_units;payload.round_units=input.work_units;self.stage=8;self.normal_progress=structural();},
   _=>return Ok(true),
  }
  Ok(false)
 }
}
