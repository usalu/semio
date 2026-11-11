/// 🧵️ A task retains every original capture through its defining typed source.
pub trait AsyncTaskSource<Mutation,ConfigMutation,DraftMutation>:semio_framework_value::retirement::RetireOwned{
 type Future:Future<Output=Result<TaskResolution<Mutation,ConfigMutation,DraftMutation>,Fault>>+'static;
 fn run(self,context:TaskCtx)->Self::Future;
}
/// 🪪️ Original typed task dispatch exposes its complete physical close authority.
pub trait AsyncTaskRun<Mutation,ConfigMutation,DraftMutation>:Send{
 fn run(self:Box<Self>,context:TaskCtx)->Pin<Box<dyn Future<Output=Result<TaskResolution<Mutation,ConfigMutation,DraftMutation>,Fault>>>>;
 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>;
 fn work(&self,body:usize)->Result<u64,ValueError>;
 fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
 fn allocation_bytes(&self)->usize;
}
struct TypedAsyncTaskRun<T:semio_framework_value::retirement::RetireOwned,M,C,D>{source:ControlledRetirement<T>,kind:std::marker::PhantomData<fn()->(M,C,D)>}
impl<T:AsyncTaskSource<M,C,D>,M:'static,C:'static,D:'static> AsyncTaskRun<M,C,D> for TypedAsyncTaskRun<T,M,C,D>{
 fn run(mut self:Box<Self>,context:TaskCtx)->Pin<Box<dyn Future<Output=Result<TaskResolution<M,C,D>,Fault>>>>{let source=self.source.take_original().expect("original task run must retain untouched source");Box::pin(source.run(context))}
 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.source.next_demand(copy)}
 fn work(&self,body:usize)->Result<u64,ValueError>{self.source.next_computational_work(body)}
 fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{self.source.step(body,effect)}
 fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()}
 fn allocation_bytes(&self)->usize{size_of::<Self>()}
}
#[doc(hidden)]
pub struct AsyncTaskRunSource<M:'static,C:'static,D:'static>{original:ManuallyDrop<Option<Box<dyn AsyncTaskRun<M,C,D>>>>}
impl<M:'static,C:'static,D:'static> AsyncTaskRunSource<M,C,D>{
 pub fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::retirement::RetirementStep{
  use semio_framework_value::retirement::RetirementStep;
  let Some(source)=self.original.as_mut()else{return RetirementStep::Complete(0)};
  let grant=effect.grant();
  if grant.maximum_items==0||effect.maximum_work_units()==0{return RetirementStep::BudgetExhausted}
  if source.terminal_is_empty(){
   let bytes=source.allocation_bytes();
   if grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return RetirementStep::BudgetExhausted}
   let work=match semio_framework_value::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};
   drop(self.original.take());
   return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()},work)
  }
  match source.close_step(body,effect){
   semio_framework_value::RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),
   semio_framework_value::RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)},
  }
 }
}
impl<M:'static,C:'static,D:'static> semio_framework_value::retirement::RetirementCursor for AsyncTaskRunSource<M,C,D>{
 semio_framework_value::native_retirement_reply!();
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.original.as_ref().map_or(Ok(0),|source|if source.terminal_is_empty(){Ok(1)}else{source.work(body)})}
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let Some(source)=self.original.as_ref()else{return Ok(Default::default())};if source.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:source.allocation_bytes(),depth:1,..Default::default()})}source.demands(maximum_body_bytes)}

 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.original.as_ref().map_or(Ok(0),|source|source.demands(0).map(|d|d.copy_bytes))}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.original.as_ref().map_or(Some(0),|source|source.demands(copy).ok().map(|d|d.capacity_bytes))}
 fn next_close_byte_demand(&self)->Option<usize>{self.original.as_ref().map_or(Some(0),|source|if source.terminal_is_empty(){Some(source.allocation_bytes())}else{source.demands(0).ok().map(|d|d.release_bytes)})}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.original.as_ref().map_or(Ok(0),|source|if source.terminal_is_empty(){Ok(1)}else{source.demands(0).map(|d|d.depth)})}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<M:'static,C:'static,D:'static> Drop for AsyncTaskRunSource<M,C,D>{fn drop(&mut self){assert!(std::thread::panicking()||self.original.is_none(),"original typed task source requires full granted retirement");}}
impl<M:'static,C:'static,D:'static> semio_framework_value::retirement::RetireOwned for AsyncTaskRunSource<M,C,D>{
 type RetirementCursor=Self;
 fn retirement_seed(self)->Self{self}
 fn controlled_retirement_supported()->bool{true}
}
impl<M:Send+'static,C:Send+'static,D:Send+'static> semio_framework_value::retirement::RetireOwned for AsyncTask<M,C,D>{
 type RetirementCursor=<(String,Option<String>,Option<Vec<u8>>,AsyncTaskRunSource<M,C,D>) as semio_framework_value::retirement::RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{let Self{label,key,restart,run}=self;semio_framework_value::retirement::RetireOwned::retirement_seed((label,key,restart,AsyncTaskRunSource{original:ManuallyDrop::new(Some(run))}))}
 fn controlled_retirement_supported()->bool{true}
}

const COLD_TASK_RETIREMENT_STEPS:usize=1<<16;
const COLD_TASK_RETIREMENT_BODY_BYTES:usize=1<<16;
/// 🧊️ Retires a task run that never reached a hot grant — refused at admission or cancelled before its first poll — to exact terminal emptiness under its own quoted grant, the cold-path precedent of `ExtensionBundle::dispose_cold`.
pub fn dispose_cold_task_run<M,C,D>(mut run:Box<dyn AsyncTaskRun<M,C,D>>){
 for _ in 0..COLD_TASK_RETIREMENT_STEPS{
  if run.terminal_is_empty(){return}
  let Ok(demand)=run.demands(COLD_TASK_RETIREMENT_BODY_BYTES)else{return};
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(COLD_TASK_RETIREMENT_BODY_BYTES),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)};
  let Ok(mut turn)=semio_framework_job::OriginalRetirementPump::new().turn(1<<16,u64::MAX,||Some(0),grant)else{return};
  let Ok(Some((poll,_)))=turn.with_effect(|effect|run.close_step(COLD_TASK_RETIREMENT_BODY_BYTES,effect))else{return};
  if let semio_framework_value::RetainedWorkPoll::Complete(done)=poll{if done.result.is_err(){return}}
 }
}
/// 🧊️ The executor-owned custody of a spawned task's run: dropped before its first poll, the run retires cold instead of abandoning its controlled source.
pub struct ColdTaskRun<M,C,D>(Option<Box<dyn AsyncTaskRun<M,C,D>>>);
impl<M,C,D> ColdTaskRun<M,C,D>{
 pub fn new(run:Box<dyn AsyncTaskRun<M,C,D>>)->Self{Self(Some(run))}
 pub fn take(&mut self)->Box<dyn AsyncTaskRun<M,C,D>>{self.0.take().expect("cold task run is taken once, at its first poll")}
}
impl<M,C,D> Drop for ColdTaskRun<M,C,D>{fn drop(&mut self){if let Some(run)=self.0.take(){dispose_cold_task_run(run)}}}
