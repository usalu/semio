//! 🫗️ The shared borrowed traversal retains its financed frontier while borrowing each caller output slice only for one action.
use super::{JsonWriteFrame,JsonWriteSource,JsonWriteOutput,advance_source,MAX_DEPTH};
use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};

const ATOM_BYTES:usize=64;
/// 📊️ Each action reports only its accepted caller prefix and actual retained state changes.
pub struct JsonBorrowedSinkStep{pub written_bytes:usize,pub progress:RetainedCloneProgress,pub complete:bool}
/// 🧭️ No source references or output String survive a call; owned frontier and pending octets survive every pause.
pub struct JsonBorrowedSinkCursor{
 frames:ManuallyDrop<Vec<JsonWriteFrame>>,path:ManuallyDrop<Vec<usize>>,maximum_depth:usize,
 pending:[u8;ATOM_BYTES],pending_length:usize,pending_position:usize,written:usize,phase:u8,failed:bool,
 closing:Option<ControlledRetirement<(Vec<JsonWriteFrame>,Vec<usize>)>>,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed JSON sink cursor has no admitted next action")}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
struct Atom<'a>{bytes:&'a mut[u8;ATOM_BYTES],length:usize}
impl JsonWriteOutput for Atom<'_>{
 fn raw(&mut self,text:&str,_:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  let end=self.length.checked_add(text.len()).filter(|end|*end<=ATOM_BYTES).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical JSON atom exceeds fixed scalar cell"))?;
  self.bytes[self.length..end].copy_from_slice(text.as_bytes());self.length=end;Ok(())
 }
}
impl JsonBorrowedSinkCursor{
 /// 🌱️ Empty traversal owns no frontier allocations until the caller separately admits them.
 pub fn empty(maximum_depth:usize)->Result<Self,ValueError>{
  if maximum_depth==0||maximum_depth>MAX_DEPTH as usize{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"borrowed JSON sink depth exceeds structural policy"))}
  Ok(Self{frames:ManuallyDrop::new(Vec::new()),path:ManuallyDrop::new(Vec::new()),maximum_depth,pending:[0;ATOM_BYTES],pending_length:0,pending_position:0,written:0,phase:0,failed:false,closing:None})
 }
 pub fn written_bytes(&self)->usize{self.written}
 pub fn pending_bytes(&self)->usize{self.pending_length-self.pending_position}
 pub fn is_complete(&self)->bool{self.phase==4&&!self.failed}
 pub fn terminal_is_empty(&self)->bool{self.frames.capacity()==0&&self.path.capacity()==0&&self.closing.is_none()&&(self.phase==0||self.phase==6)}
 pub fn failed(&self)->bool{self.failed}
 fn metadata()->usize{3*size_of::<JsonWriteFrame>()+6*size_of::<usize>()+2*size_of::<u8>()}
 /// 📏️ Quotes one original allocation, bounded traversal atom or positive caller-copy action.
 pub fn next_demands(&self,output_bytes:usize,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing.is_some()||self.phase==5{return self.close_demands(body)}
  if self.failed{return Err(invalid())}
  let d=match self.phase{
   0=>RetirementDemand{copy_bytes:2*size_of::<Vec<JsonWriteFrame>>()+size_of::<u8>(),capacity_bytes:(self.maximum_depth+1)*size_of::<JsonWriteFrame>(),depth:1,..Default::default()},
   1=>RetirementDemand{copy_bytes:2*size_of::<Vec<usize>>()+size_of::<u8>(),capacity_bytes:self.maximum_depth*size_of::<usize>(),depth:1,..Default::default()},
   2=>RetirementDemand{copy_bytes:2*size_of::<JsonWriteFrame>()+size_of::<usize>()+size_of::<u8>(),depth:1,..Default::default()},
   3 if self.pending_position<self.pending_length=>{
    let metadata=3*size_of::<usize>();let count=(self.pending_length-self.pending_position).min(output_bytes).min(body.saturating_sub(metadata));
    RetirementDemand{copy_bytes:metadata+count.max(1),depth:1,..Default::default()}
   },
   3=>RetirementDemand{copy_bytes:Self::metadata()+ATOM_BYTES,depth:self.path.len()+1,..Default::default()},
   4|6=>Default::default(),
   _=>return Err(invalid()),
  };Ok(d)
 }
 /// ▶️ One shared JSON transition or one positive flush spends one actual caller work unit; zero work preserves all state.
 pub fn step<S:JsonWriteSource+?Sized>(&mut self,source:&S,output:&mut[u8],grant:RetainedCloneGrant,maximum_work:u64,control:&mut NativeEncodeControl<'_>)->RetainedWorkOutcome<JsonBorrowedSinkStep>{
  let mut progress=RetainedCloneProgress::default();let mut written_bytes=0;let mut work=0;
  let result=(||{
   if maximum_work==0||grant.maximum_items==0||self.is_complete(){return Ok(())}
   let d=match self.next_demands(output.len(),grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>{work=1;return Err(error)}};if !fits(d,grant){return Ok(())}
   if self.phase==3&&self.pending_position<self.pending_length&&output.is_empty(){return Ok(())}
   work=1;control.checkpoint()?;
   match self.phase{
    0=>{*self.frames=Vec::with_capacity(self.maximum_depth+1);self.phase=1;progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()};},
    1=>{*self.path=Vec::with_capacity(self.maximum_depth);self.phase=2;progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()};},
    2=>{self.frames.push(JsonWriteFrame{state:0,index:0,position:0,source_index:0,next_cursor:0});self.phase=3;progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};},
    3 if self.pending_position<self.pending_length=>{
     let count=d.copy_bytes-3*size_of::<usize>();let end=self.pending_position+count;output[..count].copy_from_slice(&self.pending[self.pending_position..end]);self.pending_position=end;self.written+=count;written_bytes=count;
     if self.pending_position==self.pending_length{self.pending_position=0;self.pending_length=0}
     progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
    },
    3=>{
     if self.frames.is_empty(){self.phase=4;progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()};}
     else{
      let mut atom=Atom{bytes:&mut self.pending,length:0};
      let result=advance_source(&mut self.frames,&mut self.path,&mut atom,source,control,0,false);
      self.pending_length=atom.length;self.pending_position=0;
      let metadata=match &result{Ok(bytes)=>*bytes,Err(error)=>error.retained_progress().copied_bytes};
      progress=RetainedCloneProgress{copied_items:1,copied_bytes:metadata+atom.length+2*size_of::<usize>(),..Default::default()};
      if let Err(error)=result{self.failed=true;progress.copied_bytes+=size_of::<bool>();return Err(error)}
     }
    },
    _=>return Err(invalid()),
   }
   Ok(())
  })();
  RetainedWorkOutcome{work,result:result.map(|_|JsonBorrowedSinkStep{written_bytes,progress,complete:self.is_complete()}).map_err(|error:ValueError|error.with_retained_progress(progress))}
 }
 /// 🧹️ Closure retains every paid frame and pending atom independently of source availability.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  if let Some(owner)=self.closing.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<(Vec<JsonWriteFrame>,Vec<usize>)>>>()+size_of::<u8>(),depth:1,..Default::default()})}else{owner.next_demand(body)}}
  Ok(RetirementDemand{copy_bytes:2*size_of::<(Vec<JsonWriteFrame>,Vec<usize>)>()+size_of::<Option<ControlledRetirement<(Vec<JsonWriteFrame>,Vec<usize>)>>>()+size_of::<u8>(),depth:1,..Default::default()})
 }
 pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
  use semio_framework_value::RetainedWorkPoll::Complete as Settled;let grant=effect.grant();
  let done=|result:Result<RetainedCloneStep,ValueError>,work:u128|Settled(RetainedWorkOutcome{work,result:result.map_err(Into::into)});
  let pay=|effect:&mut semio_framework_value::RetainedAllocationEffect<'_>|->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"borrowed JSON sink closure exceeded its original work"))}};
  if self.terminal_is_empty(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0)}
  let d=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(Err(error),0)};
  if !fits(d,grant){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  if let Some(owner)=self.closing.as_mut(){
   if owner.terminal_is_empty(){if let Err(error)=pay(effect){return done(Err(error),1)}self.closing=None;self.phase=6;return done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),1)}
   return owner.step(effect)
  }
  if let Err(error)=pay(effect){return done(Err(error),1)}
  self.closing=Some(ControlledRetirement::retain((std::mem::take(&mut*self.frames),std::mem::take(&mut*self.path))));self.phase=5;
  done(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),1)
 }
}
impl Drop for JsonBorrowedSinkCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"borrowed JSON sink cursor requires funded frontier closure")}}

