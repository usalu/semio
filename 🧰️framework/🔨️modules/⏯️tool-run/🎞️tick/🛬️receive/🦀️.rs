//! 🛬️ Original wire pages construct native Tick fields under the caller's physical recipient.
use super::*;
use pack::{PackLimits,record::{RetainedRecordBodyCursor,RetainedRecordBodyToken,RetainedValueToken as Token,RetainedValueRole as Role,RetainedValueContainer as Container}};
use semio_framework_value::{NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};

/// 📄️ The caller reborrows the same producer pages without transferring or flattening them.
pub trait OriginalTickWireSource{fn identity(&self)->usize;fn byte_len(&self)->usize;fn page(&self,index:usize)->Option<&[u8]>;}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Kind{Tick,Identity,Progress,Steps,ProgressSteps,Step,Trace,Ops,Update}
#[derive(Clone,Copy)]
struct Frame{kind:Kind,field:u8,seen:u16,remaining:usize,values:[u64;10],revision:[u8;32]}
impl Frame{fn new(kind:Kind,count:usize)->Self{Self{kind,field:0,seen:0,remaining:count,values:[0;10],revision:[0;32]}}}
#[derive(Clone,Copy)]
enum Bytes{None,Revision,Entities,Counters,Args,Payload,Op,Trace}
#[derive(Clone,Copy)]
struct ByteState{kind:Bytes,remaining:usize,length:usize,position:usize,scalar:[u8;10]}
impl Default for ByteState{fn default()->Self{Self{kind:Bytes::None,remaining:0,length:0,position:0,scalar:[0;10]}}}
#[derive(semio_framework_value::RetireOwned)]
struct Owned{tick:Option<ToolRunTick>,step:Option<ToolRunStep>}
fn empty_identity()->ToolRunIdentity{ToolRunIdentity{id:ToolRunId{app_instance_id:0,run:0},generation:0,base_revision:[0;32]}}
fn malformed(detail:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,detail)}
fn pack_error(_:pack::PackRefusal)->ValueError{malformed("original Tick Pack record refused")}
fn funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)}
fn reserve<T>(column:&mut Vec<T>,count:usize)->Result<usize,ValueError>{if column.capacity()!=0||!column.is_empty(){return Err(malformed("native Tick column was already admitted"))}column.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native Tick column backing refused"))?;Ok(column.capacity()*size_of::<T>())}

/// 🧳️ Typed partial output, parser state and the original input fence survive every denied turn.
pub struct ToolRunTickNativeCursor{
 parser:Option<RetainedRecordBodyCursor>,source:(usize,usize),offset:usize,page:usize,page_offset:usize,sealed:bool,pending:Option<Token>,frames:[Frame;5],depth:usize,bytes:ByteState,
 owned:ManuallyDrop<Owned>,retiring:Option<ControlledRetirement<Owned>>,trace:trace::NativeTraceCursor,trace_pending:bool,complete:bool,closing:bool,closed:bool,
}
impl ToolRunTickNativeCursor{
 /// 🎟️ Captures only the original page authority and inline empty native columns.
 pub fn empty()->Self{Self{parser:None,source:(0,0),offset:0,page:0,page_offset:0,sealed:false,pending:None,frames:[Frame::new(Kind::Tick,0);5],depth:0,bytes:Default::default(),owned:ManuallyDrop::new(Owned{tick:None,step:None}),retiring:None,trace:trace::NativeTraceCursor::empty(),trace_pending:false,complete:false,closing:true,closed:true}}
 pub fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<RetainedRecordBodyCursor>()+size_of::<Option<RetainedRecordBodyCursor>>()+2*size_of::<ToolRunTick>()+size_of::<Option<ToolRunTick>>()+8*size_of::<usize>()+8*size_of::<bool>()+size_of::<ByteState>(),depth:1,..Default::default()}}
 pub fn admit(&mut self,source:&dyn OriginalTickWireSource,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  let demand=Self::birth_demands();if !funds(grant,demand){return Ok(None)}if !self.terminal_is_empty(){return Err(malformed("original Tick admission requires its original empty receiver"))}
  if source.byte_len()>TOOL_RUN_TICK_BYTES_MAX{return Err(malformed("original Tick wire exceeds its bounded extent"))}
  let limits=PackLimits{max_file_len:TOOL_RUN_TICK_BYTES_MAX as u64,max_segment_len:TOOL_RUN_TICK_BYTES_MAX as u64,max_symbols:1,max_depth:8,max_items:TOOL_RUN_TICK_BYTES_MAX as u64,max_total_alloc:(TOOL_RUN_TICK_BYTES_MAX*16)as u64};
  let parser=RetainedRecordBodyCursor::try_new(limits,0,0,0,TOOL_RUN_TICK_BYTES_MAX*16).map_err(pack_error)?;
  let tick=ToolRunTick{identity:empty_identity(),sequence:0,progress:None,steps:Vec::new(),trace:Vec::new(),append_ops:Vec::new(),append_entities:Vec::new(),retract_to:None,payload:None,payload_update:None};
  self.parser=Some(parser);self.source=(source.identity(),source.byte_len());self.offset=0;self.page=0;self.page_offset=0;self.sealed=false;self.pending=None;self.depth=0;self.bytes=Default::default();self.owned.tick=Some(tick);self.trace_pending=false;self.complete=false;self.closing=false;self.closed=false;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
 pub fn consumed_bytes(&self)->usize{self.offset}
 pub fn is_complete(&self)->bool{self.complete}
 fn tick(&self)->&ToolRunTick{self.owned.tick.as_ref().unwrap()}
 fn tick_mut(&mut self)->&mut ToolRunTick{self.owned.tick.as_mut().unwrap()}
 fn frame(&self)->Result<Frame,ValueError>{self.depth.checked_sub(1).map(|index|self.frames[index]).ok_or_else(||malformed("native Tick record has no frame"))}
 fn push(&mut self,kind:Kind,count:usize)->Result<(),ValueError>{if self.depth==self.frames.len(){return Err(malformed("native Tick semantic depth exceeded"))}self.frames[self.depth]=Frame::new(kind,count);self.depth+=1;Ok(())}
 fn element(&mut self)->Result<(),ValueError>{let frame=&mut self.frames[self.depth-1];frame.remaining=frame.remaining.checked_sub(1).ok_or_else(||malformed("native Tick container has too many values"))?;Ok(())}
 fn allocation(&self,token:Token)->Result<usize,ValueError>{
  let current=if self.depth==0{None}else{Some(self.frame()?)};
  Ok(match(token,current){
   (Token::Begin{kind:Container::List,count},Some(frame))=>{let count=usize::try_from(count).map_err(|_|malformed("native Tick list count exceeds platform"))?;let size=match(frame.kind,frame.field){(Kind::Tick,4)=>size_of::<ToolRunStep>(),(Kind::Tick,5)=>size_of::<ToolRunTracePage>(),(Kind::Tick,6)=>size_of::<Vec<u8>>(),(Kind::Progress,10)=>size_of::<ToolRunStep>(),_=>return Err(malformed("native Tick list field is undeclared"))};if (frame.kind==Kind::Progress&&count>TOOL_RUN_STEP_RING_CAPACITY)||(frame.field==4&&count>TOOL_RUN_STEP_RING_CAPACITY){return Err(malformed("native Tick step list exceeds ring"))}count.checked_mul(size).ok_or_else(||malformed("native Tick list ownership overflow"))?},
   (Token::Unsigned{role:Role::BytesLength,value},Some(frame))=>{let length=usize::try_from(value).map_err(|_|malformed("native Tick byte count exceeds platform"))?;if length>TOOL_RUN_TICK_BYTES_MAX{return Err(malformed("native Tick byte column exceeds wire"))}match(frame.kind,frame.field){(Kind::Tick,9)|(Kind::Ops,_)=>length,(Kind::Tick,7)=>{if length%8!=0{return Err(malformed("native entity column width"))}length},(Kind::Progress,7)=>{if length%10!=0||length/10>TOOL_RUN_COUNTERS_MAX{return Err(malformed("native counter column width"))}(length/10)*size_of::<ToolRunCounter>()},(Kind::Step,7)=>{if length%9!=0||length/9>TOOL_RUN_STEP_ARGS_MAX{return Err(malformed("native argument column width"))}(length/9)*size_of::<ToolRunStepArg>()},(Kind::Identity,4)if length==32=>0,(Kind::Trace,_)=>0,_=>return Err(malformed("native Tick byte field is undeclared"))}},
   _=>0,
  })
 }
 /// 📏️ Quotes the next native transition independently of input parsing and terminal retirement.
 pub fn next_demands(&mut self)->Result<RetirementDemand,ValueError>{
  if self.closing||self.complete{return Ok(Default::default())}
  if self.trace_pending{let mut demand=self.trace.next_demands()?;demand.copy_bytes+=size_of::<Frame>()+size_of::<bool>();demand.depth+=1;return Ok(demand)}
  if let Some(token)=self.pending{let trace_birth=if matches!(token,Token::Unsigned{role:Role::BytesLength,..})&&self.frame()?.kind==Kind::Trace{trace::NativeTraceCursor::birth_copy_bytes()}else{0};return Ok(RetirementDemand{copy_bytes:size_of::<Token>()+size_of::<Frame>()+size_of::<ByteState>()+size_of::<ToolRunStep>()+trace_birth,capacity_bytes:self.allocation(token)?,depth:self.depth+1,..Default::default()})}
  let mut demand=self.parser.as_mut().unwrap().record_storage_admission_demands().map_err(pack_error)?;if demand.capacity_bytes==0{demand.copy_bytes=size_of::<Option<Token>>()+3*size_of::<usize>()+size_of::<bool>();}demand.depth=demand.depth.max(1);Ok(demand)
 }
 /// ⏱️ Reborrows the exact same source, decoder ledger, cancellation observer and construction recipient.
 pub fn advance(&mut self,source:&dyn OriginalTickWireSource,control:&mut NativeDecodeControl<'_>)->Result<RetainedCloneStep,semio_framework_value::native_decoding::NativeConstructionStepFailure<ValueError>>{
  if self.source!=(source.identity(),source.byte_len()){return Err(malformed("native Tick requires the same original wire pages").into())}if self.closing{return Err(malformed("native Tick receive after close").into())}if self.complete{return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.next_demands()?;control.receive_construction_step(demand,|control,grant|self.advance_admitted(source,control,grant,demand))
 }
 fn advance_admitted(&mut self,source:&dyn OriginalTickWireSource,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,demand:RetirementDemand)->Result<RetainedCloneStep,ValueError>{
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  let result=(||{if self.trace_pending{let parent=size_of::<Frame>()+size_of::<bool>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_depth:grant.maximum_depth-1,..grant};if self.trace.is_complete(){let(page,receipt)=self.trace.take(child).expect("quoted original trace output move");self.tick_mut().trace.push(page);progress=receipt;}else{progress=self.trace.advance(control,child)?.progress();}if self.trace.terminal_is_empty(){self.element()?;self.trace_pending=false;}else if self.trace.ready_for_input(){self.trace_pending=false;}progress.copied_items=progress.copied_items.max(1);progress.copied_bytes+=parent; }
  else if let Some(token)=self.pending{control.charge(demand.capacity_bytes)?;progress.retained_capacity_bytes=self.token(token)?;self.pending=None;}
  else if demand.capacity_bytes!=0{control.charge(demand.capacity_bytes)?;progress=self.parser.as_mut().unwrap().admit_record_storage(grant).map_err(pack_error)?;}
  else if let Some(token)=self.parser.as_mut().unwrap().grant().map_err(pack_error)?{match token{RetainedRecordBodyToken::Value(token)=>self.pending=Some(token),RetainedRecordBodyToken::Catalog{symbols}if symbols!=0=>return Err(malformed("native Tick wire has foreign symbols")),_=>{}}}
  else if self.parser.as_ref().unwrap().ingress_ready(){if self.offset<self.source.1{let page=source.page(self.page).ok_or_else(||malformed("native Tick original page missing"))?;if page.is_empty(){return Err(malformed("native Tick original page is empty"))}if self.page_offset==page.len(){self.page+=1;self.page_offset=0;}else{let byte=*page.get(self.page_offset).ok_or_else(||malformed("native Tick original page position differs"))?;self.parser.as_mut().unwrap().admit_byte(self.offset as u64,byte).map_err(|_|malformed("native Tick input handback"))?;self.offset+=1;self.page_offset+=1;}}else if !self.sealed{self.parser.as_mut().unwrap().seal(self.offset as u64).map_err(pack_error)?;self.sealed=true;}else{return Err(malformed("native Tick parser stalled after final byte"))}}
  Ok(())})();result.map_err(|error:ValueError|error.with_retained_progress(progress))?;Ok(if self.complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 fn begin(&mut self,kind:Container,count:u64)->Result<usize,ValueError>{
  let count=usize::try_from(count).map_err(|_|malformed("native Tick count exceeds platform"))?;
  if self.depth==0{if kind!=Container::Record{return Err(malformed("native Tick root is not a record"))}self.push(Kind::Tick,count)?;return Ok(0)}
  let frame=self.frame()?;let mut born=0;
  let semantic=match(kind,frame.kind,frame.field){
   (Container::Record,Kind::Tick,1)|(Container::Record,Kind::Progress,1)=>Kind::Identity,
   (Container::Record,Kind::Tick,3)=>{self.tick_mut().progress=Some(ToolRunProgress{identity:empty_identity(),sequence:0,state:ToolRunState::Starting,stage:0,completed:0,total:None,counters:Vec::new(),units_per_second:0.,conflicts:0,steps:ToolRunStepRing::new()});Kind::Progress},
   (Container::Record,Kind::Tick,10)=>Kind::Update,
   (Container::Record,Kind::Steps,_)|(Container::Record,Kind::ProgressSteps,_)=>{self.owned.step=Some(ToolRunStep::new(0,ToolRunStepKind::Info,0,0));Kind::Step},
   (Container::List,Kind::Tick,4)=>{born=reserve(&mut self.tick_mut().steps,count)?;Kind::Steps},
   (Container::List,Kind::Tick,5)=>{born=reserve(&mut self.tick_mut().trace,count)?;Kind::Trace},
   (Container::List,Kind::Tick,6)=>{born=reserve(&mut self.tick_mut().append_ops,count)?;Kind::Ops},
   (Container::List,Kind::Progress,10)=>{let steps=&mut self.tick_mut().progress.as_mut().unwrap().steps.steps;steps.try_reserve_exact(count).map_err(|_|malformed("native progress step backing refused"))?;born=steps.capacity()*size_of::<ToolRunStep>();Kind::ProgressSteps},
   _=>return Err(malformed("native Tick container shape is undeclared")),
  };self.push(semantic,count)?;Ok(born)
 }
 fn bytes_begin(&mut self,length:usize)->Result<usize,ValueError>{
  let frame=self.frame()?;let(mut born,kind)=(0,match(frame.kind,frame.field){(Kind::Identity,4)=>Bytes::Revision,(Kind::Tick,7)=>Bytes::Entities,(Kind::Progress,7)=>Bytes::Counters,(Kind::Step,7)=>Bytes::Args,(Kind::Tick,9)=>Bytes::Payload,(Kind::Ops,_)=>Bytes::Op,(Kind::Trace,_)=>Bytes::Trace,_=>return Err(malformed("native Tick byte destination undeclared"))});
  match kind{Bytes::Payload=>{self.tick_mut().payload=Some(Vec::new());born=reserve(self.tick_mut().payload.as_mut().unwrap(),length)?;},Bytes::Op=>{let mut bytes=Vec::new();born=reserve(&mut bytes,length)?;self.tick_mut().append_ops.push(bytes);},Bytes::Entities=>born=reserve(&mut self.tick_mut().append_entities,length/8)?,Bytes::Counters=>born=reserve(&mut self.tick_mut().progress.as_mut().unwrap().counters,length/10)?,Bytes::Args=>born=reserve(&mut self.owned.step.as_mut().unwrap().args,length/9)?,Bytes::Trace=>{self.trace.start(length)?;},_=>{}}
  self.bytes=ByteState{kind,remaining:length,length,position:0,scalar:[0;10]};if length==0{self.bytes_end()?;}Ok(born)
 }
 fn bytes_end(&mut self)->Result<(),ValueError>{let kind=self.bytes.kind;self.bytes.kind=Bytes::None;match kind{Bytes::Op=>self.element()?,Bytes::Trace=>{self.trace.seal()?;self.trace_pending=true;},_=>{}}Ok(())}
 fn byte(&mut self,byte:u8)->Result<(),ValueError>{
  if self.bytes.remaining==0{return Err(malformed("native Tick unexpected octet"))}let position=self.bytes.position;
  match self.bytes.kind{
   Bytes::Revision=>self.frames[self.depth-1].revision[position]=byte,
   Bytes::Payload=>self.tick_mut().payload.as_mut().unwrap().push(byte),Bytes::Op=>self.tick_mut().append_ops.last_mut().unwrap().push(byte),
   Bytes::Trace=>{self.trace.admit_byte(position,byte)?;self.trace_pending=true;},
   kind=>{let width=match kind{Bytes::Entities=>8,Bytes::Counters=>10,Bytes::Args=>9,_=>return Err(malformed("native Tick byte destination absent"))};self.bytes.scalar[position%width]=byte;if position%width==width-1{let scalar=self.bytes.scalar;match kind{Bytes::Entities=>self.tick_mut().append_entities.push(u64::from_le_bytes(scalar[..8].try_into().unwrap())),Bytes::Counters=>self.tick_mut().progress.as_mut().unwrap().counters.push(ToolRunCounter{counter:u16::from_le_bytes(scalar[..2].try_into().unwrap()),value:u64::from_le_bytes(scalar[2..10].try_into().unwrap())}),Bytes::Args=>self.owned.step.as_mut().unwrap().args.push(match scalar[0]{0=>ToolRunStepArg::Unsigned(u64::from_le_bytes(scalar[1..9].try_into().unwrap())),1=>ToolRunStepArg::Float(f64::from_le_bytes(scalar[1..9].try_into().unwrap())),_=>return Err(malformed("native Tick argument ordinal"))}),_=>{}}}},
  }
  self.bytes.position+=1;self.bytes.remaining-=1;if self.bytes.remaining==0{self.bytes_end()?;}Ok(())
 }
 fn end(&mut self,kind:Container)->Result<(),ValueError>{
  let frame=self.frame()?;let record=!matches!(frame.kind,Kind::Steps|Kind::ProgressSteps|Kind::Trace|Kind::Ops);if record!=(kind==Container::Record)||frame.remaining!=0{return Err(malformed("native Tick container finality differs"))}
  let required=match frame.kind{Kind::Tick=>3,Kind::Identity=>15,Kind::Progress=>0b110011111,Kind::Step=>0b101111,Kind::Update=>1,_=>0};if frame.seen&required!=required{return Err(malformed("native Tick required field absent"))}
  self.depth-=1;
  match frame.kind{
   Kind::Identity=>{let identity=ToolRunIdentity{id:ToolRunId{app_instance_id:u32::try_from(frame.values[0]).map_err(|_|malformed("native app identity overflow"))?,run:frame.values[1]},generation:u32::try_from(frame.values[2]).map_err(|_|malformed("native generation overflow"))?,base_revision:frame.revision};if self.frame()?.kind==Kind::Progress{self.tick_mut().progress.as_mut().unwrap().identity=identity}else{self.tick_mut().identity=identity}},
   Kind::Tick=>validate_tick_payload(self.tick().payload.as_deref(),self.tick().payload_update).map_err(|_|malformed("native Tick publication extent refused"))?,
   Kind::Step=>{if self.owned.step.as_ref().unwrap().repeat==0{return Err(malformed("native Tick zero repeat"))}let step=self.owned.step.take().unwrap();if self.frame()?.kind==Kind::ProgressSteps{self.tick_mut().progress.as_mut().unwrap().steps.steps.push_back(step)}else{self.tick_mut().steps.push(step)}self.element()?;},
   Kind::Update=>{self.tick_mut().payload_update=Some(match(frame.values[0],frame.seen){(0,1)=>ToolRunPayloadUpdate::Replace,(1,63)=>ToolRunPayloadUpdate::Range{publication:frame.values[1],total_bytes:frame.values[2],offset:frame.values[3],chunk_bytes:u32::try_from(frame.values[4]).map_err(|_|malformed("native publication stride overflow"))?,final_chunk:match frame.values[5]{0=>false,1=>true,_=>return Err(malformed("native publication finality ordinal"))}},_=>return Err(malformed("native publication update is not closed"))});},
   _=>{}
  }Ok(())
 }
 fn token(&mut self,token:Token)->Result<usize,ValueError>{
  match token{
   Token::Begin{kind,count}=>return self.begin(kind,count),Token::End(kind)=>self.end(kind)?,
   Token::Unsigned{role:Role::FieldId,value}=>{let frame=&mut self.frames[self.depth-1];let maximum=match frame.kind{Kind::Tick|Kind::Progress=>10,Kind::Identity=>4,Kind::Step=>7,Kind::Update=>6,_=>return Err(malformed("native Tick field in list"))};let field=u8::try_from(value).map_err(|_|malformed("native Tick field exceeds ordinal"))?;if field==0||field>maximum||frame.seen&(1<<(field-1))!=0{return Err(malformed("native Tick field duplicate or foreign"))}frame.field=field;frame.seen|=1<<(field-1);self.element()?;},
   Token::Unsigned{role:Role::Unsigned,value}=>{let frame=self.frame()?;self.frames[self.depth-1].values[usize::from(frame.field-1)]=value;match(frame.kind,frame.field){(Kind::Tick,2)=>self.tick_mut().sequence=value,(Kind::Tick,8)=>self.tick_mut().retract_to=Some(u32::try_from(value).map_err(|_|malformed("native retract count overflow"))?),(Kind::Identity,1..=3)|(Kind::Update,1..=6)=>{},(Kind::Step,field)=>{let step=self.owned.step.as_mut().unwrap();match field{1=>step.sequence=value,2=>step.kind=ToolRunStepKind::from_ordinal(value).ok_or_else(||malformed("native step ordinal"))?,3=>step.stage=u16::try_from(value).map_err(|_|malformed("native step stage overflow"))?,4=>step.reason=u16::try_from(value).map_err(|_|malformed("native step reason overflow"))?,5=>step.subject=Some(value),6=>step.repeat=u32::try_from(value).map_err(|_|malformed("native step repeat overflow"))?,_=>return Err(malformed("native step unsigned field undeclared"))}},(Kind::Progress,field)=>{let progress=self.tick_mut().progress.as_mut().unwrap();match field{2=>progress.sequence=value,3=>progress.state=ToolRunState::from_ordinal(value).ok_or_else(||malformed("native progress state ordinal"))?,4=>progress.stage=u16::try_from(value).map_err(|_|malformed("native progress stage overflow"))?,5=>progress.completed=value,6=>progress.total=Some(value),9=>progress.conflicts=u32::try_from(value).map_err(|_|malformed("native progress conflicts overflow"))?,_=>return Err(malformed("native progress unsigned field undeclared"))}},_=>return Err(malformed("native Tick unsigned field undeclared"))}},
   Token::Unsigned{role:Role::BytesLength,value}=>return self.bytes_begin(usize::try_from(value).map_err(|_|malformed("native byte length overflow"))?),
   Token::Byte(byte)=>self.byte(byte)?,Token::F64(bits)=>{let frame=self.frame()?;if(frame.kind,frame.field)!=(Kind::Progress,8){return Err(malformed("native Tick float field undeclared"))}self.tick_mut().progress.as_mut().unwrap().units_per_second=f64::from_bits(bits)as f32;},
   Token::Tag{value,..}=>{let frame=self.frame()?;let expected=match(frame.kind,frame.field){(Kind::Tick,1|3|10)|(Kind::Progress,1)|(Kind::Steps,_)|(Kind::ProgressSteps,_)=>0x0d,(Kind::Tick,4|5|6)|(Kind::Progress,10)=>0x0c,(Kind::Identity,4)|(Kind::Tick,7|9)|(Kind::Progress,7)|(Kind::Step,7)|(Kind::Trace,_)|(Kind::Ops,_)=>0x08,(Kind::Progress,8)=>0x05,_=>0x04};if value!=expected{return Err(malformed("native Tick wire tag disagrees with schema"))}},
   Token::Complete{..}=>{if self.depth!=0||self.parser.as_ref().unwrap().consumed_bytes()as usize!=self.source.1||self.bytes.remaining!=0{return Err(malformed("native Tick trailing or incomplete original bytes"))}self.complete=true;},
   _=>return Err(malformed("native Tick token is undeclared")),
  }Ok(0)
 }
 /// 🎁️ Moves the same original native Tick only after all parser and publication finality checks.
 pub fn take_output(&mut self,grant:RetainedCloneGrant)->Option<(ToolRunTick,RetainedCloneProgress)>{if !self.complete||self.closing||self.owned.tick.is_none()||!funds(grant,RetirementDemand{copy_bytes:size_of::<ToolRunTick>(),depth:1,..Default::default()}){return None}Some((self.owned.tick.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ToolRunTick>(),..Default::default()}))}
 /// ♻️ Quotes the original partial native output before parser backing release.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.trace.terminal_is_empty(){return self.trace.close_demands(body)}if let Some(owner)=&self.retiring{if !owner.terminal_is_empty(){return owner.next_demand(body)}return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Owned>>>(),depth:1,..Default::default()})}else if !self.closing{return Ok(RetirementDemand{copy_bytes:size_of::<Owned>(),depth:1,..Default::default()})}if self.parser.as_ref().is_some_and(RetainedRecordBodyCursor::terminal_is_empty){return Ok(RetirementDemand{copy_bytes:size_of::<Option<RetainedRecordBodyCursor>>()+size_of::<bool>(),depth:1,..Default::default()})}self.parser.as_ref().ok_or_else(||malformed("native Tick close lost parser"))?.record_storage_retirement_demands(body).map_err(pack_error)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.close_demands(grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.trace.terminal_is_empty(){return self.trace.close_step(grant)}if !self.closing{let owned=std::mem::replace(&mut*self.owned,Owned{tick:None,step:None});self.retiring=Some(ControlledRetirement::new(owned).unwrap_or_else(|_|unreachable!("native Tick owns its original retirement issuer")));self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}if let Some(owner)=&mut self.retiring{if !owner.terminal_is_empty(){return owner.step(grant)}self.retiring=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}if self.parser.as_ref().is_some_and(RetainedRecordBodyCursor::terminal_is_empty){self.parser=None;self.closed=true;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}self.parser.as_mut().unwrap().retire_record_storage(grant.maximum_copy_bytes,grant).map_err(pack_error)}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.parser.is_none()&&self.retiring.is_none()&&self.trace.terminal_is_empty()&&self.owned.tick.is_none()&&self.owned.step.is_none()}
}
impl Drop for ToolRunTickNativeCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original native Tick receiver requires funded terminal closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owned)}}}}
#[path="📼️trace/🦀️.rs"]mod trace;
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
