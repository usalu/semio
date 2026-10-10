/// 🧵️ A task retains every original capture through its defining typed source.
pub trait AsyncTaskSource<Mutation,ConfigMutation,DraftMutation>:semio_framework_value::retirement::RetireOwned{
 type Future:Future<Output=Result<TaskResolution<Mutation,ConfigMutation,DraftMutation>,Fault>>+'static;
 fn run(self,context:TaskCtx)->Self::Future;
}
/// 🪪️ Original typed task dispatch exposes its complete physical close authority.
pub trait AsyncTaskRun<Mutation,ConfigMutation,DraftMutation>:Send{
 fn run(self:Box<Self>,context:TaskCtx)->Pin<Box<dyn Future<Output=Result<TaskResolution<Mutation,ConfigMutation,DraftMutation>,Fault>>>>;
 fn demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>;
 fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>;
 fn terminal_is_empty(&self)->bool;
 fn allocation_bytes(&self)->usize;
}
struct TypedAsyncTaskRun<T:semio_framework_value::retirement::RetireOwned,M,C,D>{source:semio_framework_value::retirement::controlled::ControlledRetirement<T>,kind:std::marker::PhantomData<fn()->(M,C,D)>}
impl<T:AsyncTaskSource<M,C,D>,M:'static,C:'static,D:'static> AsyncTaskRun<M,C,D> for TypedAsyncTaskRun<T,M,C,D>{
 fn run(mut self:Box<Self>,context:TaskCtx)->Pin<Box<dyn Future<Output=Result<TaskResolution<M,C,D>,Fault>>>>{let source=self.source.take_original().expect("original task run must retain untouched source");Box::pin(source.run(context))}
 fn demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:self.source.next_copy_byte_demand()?,capacity_bytes:self.source.next_capacity_byte_demand(copy)?,release_bytes:self.source.next_release_byte_demand()?,depth:self.source.next_depth_demand()?})}
 fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{self.source.step(grant)}
 fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()}
 fn allocation_bytes(&self)->usize{std::mem::size_of::<Self>()}
}
struct AsyncTaskRunSource<M:'static,C:'static,D:'static>{original:std::mem::ManuallyDrop<Option<Box<dyn AsyncTaskRun<M,C,D>>>>}
impl<M:'static,C:'static,D:'static> semio_framework_value::retirement::RetirementCursor for AsyncTaskRunSource<M,C,D>{
 fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep;let Some(source)=self.original.as_mut()else{return RetirementStep::Complete};if grant.maximum_items==0{return RetirementStep::BudgetExhausted}if source.terminal_is_empty(){let bytes=source.allocation_bytes();if grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return RetirementStep::BudgetExhausted}drop(self.original.take());return RetirementStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})}match source.close_step(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.original.as_ref().map_or(Ok(0),|source|source.demands(0).map(|d|d.copy_bytes))}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.original.as_ref().map_or(Some(0),|source|source.demands(copy).ok().map(|d|d.capacity_bytes))}
 fn next_close_byte_demand(&self)->Option<usize>{self.original.as_ref().map_or(Some(0),|source|if source.terminal_is_empty(){Some(source.allocation_bytes())}else{source.demands(0).ok().map(|d|d.release_bytes)})}
 fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.original.as_ref().map_or(Ok(0),|source|if source.terminal_is_empty(){Ok(1)}else{source.demands(0).map(|d|d.depth)})}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl<M:'static,C:'static,D:'static> Drop for AsyncTaskRunSource<M,C,D>{fn drop(&mut self){assert!(std::thread::panicking()||self.original.is_none(),"original typed task source requires full granted retirement");}}
impl<M:'static,C:'static,D:'static> semio_framework_value::retirement::RetireOwned for AsyncTaskRunSource<M,C,D>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
 fn controlled_retirement_supported()->bool{true}
}
impl<M:Send+'static,C:Send+'static,D:Send+'static> semio_framework_value::retirement::RetireOwned for AsyncTask<M,C,D>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::RetireOwned;let Self{label,key,restart,run}=self;(label,key,restart,AsyncTaskRunSource{original:std::mem::ManuallyDrop::new(Some(run))}).retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes};sequence_birth_bytes(&[deferred_birth_bytes::<String>(),deferred_birth_bytes::<Option<String>>(),deferred_birth_bytes::<Option<Vec<u8>>>(),deferred_birth_bytes::<AsyncTaskRunSource<M,C,D>>()])}
 fn controlled_retirement_supported()->bool{true}
}
