//! 🎨️ Retained fallback parsing preserves previous answers without materializing a second host.
use super::*;
use super::display::FlowDisplayCursor;
use semio_framework_value::DslValue;
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeContinuation};
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy};
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowPaintCursor{parser:Option<JsonGrammarCursor<DslValue>>,control:Option<NativeDecodeContinuation>,fallback:Option<DslValue>,display:FlowDisplayCursor,binding:Option<(usize,usize)>,parsed:bool,progress:RetainedCloneProgress,work:u64}
impl FlowPaintCursor{
 pub fn new()->Self{Self{parser:None,control:None,fallback:None,display:FlowDisplayCursor::new(),binding:None,parsed:false,progress:Default::default(),work:0}}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 fn check(&self,text:&str)->Result<(),ValueError>{if self.binding.is_some_and(|binding|binding!=(text.as_ptr()as usize,text.len())){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original converged painting source changed"))}Ok(())}
 pub fn demands(&self,source:&dyn FlowSnapshotSource,channels:&EvalChannels,infos:&HistoryFoldIndex<String,OperatorInfo>,text:&str)->Result<RetirementDemand,ValueError>{self.check(text)?;if !self.parsed&&!text.is_empty(){if let Some(parser)=self.parser.as_ref(){return parser.normal_step_demands(text).map_err(|e|e.into_value_error())}return Ok(RetirementDemand{depth:1,..Default::default()})}self.display.demands_with_fallback(source,channels,infos,self.fallback.as_ref())}
 pub fn step(&mut self,source:&dyn FlowSnapshotSource,channels:&EvalChannels,infos:&HistoryFoldIndex<String,OperatorInfo>,text:&str,grant:RetainedCloneGrant)->Result<Option<String>,ValueError>{
  self.progress=Default::default();self.work=0;let d=self.demands(source,channels,infos,text)?;if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(None)}self.binding=Some((text.as_ptr()as usize,text.len()));
  if !self.parsed{if text.is_empty(){self.parsed=true;self.progress.copied_items=1;self.work=1;return Ok(None)}if self.parser.is_none(){self.parser=Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject));self.progress.copied_items=1;self.work=1;return Ok(None)}
   let mut accepted=|_|true;let mut control=match self.control.take(){Some(receipt)=>NativeDecodeControl::resume(receipt,&mut accepted)?,None=>NativeDecodeControl::new_retained(&mut accepted)};let parser=self.parser.as_mut().unwrap();let result=parser.step_source(text,1,&mut control,grant,true);self.progress=parser.normal_step_progress();self.work=parser.last_turn_work();self.control=Some(control.pause()?);if let Some(value)=result.map_err(|e|e.into_value_error().with_retained_progress(self.progress))?{self.fallback=Some(value);self.parsed=true}return Ok(None)
  }
  let result=self.display.step_with_fallback(source,channels,infos,self.fallback.as_ref(),grant);self.progress=result.as_ref().copied().unwrap_or_else(|e|e.retained_progress());self.work=u64::from(self.progress!=Default::default()||result.is_err());result?;Ok(if self.display.complete(){self.display.take_text()}else{None})
 }
}
