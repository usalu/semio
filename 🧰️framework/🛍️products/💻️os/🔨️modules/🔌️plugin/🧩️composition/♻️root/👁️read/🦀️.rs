/// 👁️ Returns exact child reads to their original registries before releasing final view backing.
pub struct ChildContentReadRetirement {
 view:std::mem::ManuallyDrop<ChildContentView>,
 pending:std::mem::ManuallyDrop<Option<ChildContentEntry>>,
 returned:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<store::SnapshotReadReturn>>>,
 metadata:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<ChildContentReadMetadata>>>,
}

#[derive(semio_framework_value::RetireOwned)]
struct ChildContentReadMetadata {owner:String,slot:String,child_id:String,artifact_id:String,dialect:ArtifactDialect,revision:[u8;32]}

struct ChildContentReadAliases;
impl ChildContentRetainers for ChildContentReadAliases {
 fn retains_root(&self,_view:&ChildContentView)->bool{false}
 fn retains_page(&self,_index:usize,_page:&std::sync::Arc<ChildContentPage>)->bool{false}
 fn retains_entry(&self,_entry:&std::sync::Arc<ChildContentEntry>)->bool{false}
}

impl ChildContentReadRetirement {
 pub fn new(view:ChildContentView)->Self{Self{view:std::mem::ManuallyDrop::new(view),pending:std::mem::ManuallyDrop::new(None),returned:std::mem::ManuallyDrop::new(None),metadata:std::mem::ManuallyDrop::new(None)}}
 fn child_demands<T:semio_framework_value::retirement::RetireOwned>(owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>,copy:usize)->Result<RetirementDemand,ValueError>{if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original child read depth overflow"))?})}
 pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.returned.as_ref(){return Self::child_demands(owner,copy)}
  if let Some(owner)=self.metadata.as_ref(){return Self::child_demands(owner,copy)}
  if self.pending.is_some(){return Ok(store::ErasedSnapshotRead::prepared_return_demands())}
  Ok(RetirementDemand{release_bytes:self.view.next_take_release_bytes(),depth:usize::from(!self.terminal_is_empty()),..Default::default()})
 }
 pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}let d=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(RetainedCloneStep::Progress(empty))}
  if let Some(owner)=self.returned.as_mut(){if owner.terminal_is_empty(){self.returned.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}return owner.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(owner)=self.metadata.as_mut(){if owner.terminal_is_empty(){self.metadata.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}return owner.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if let Some(entry)=self.pending.take(){let ChildContentEntry{owner,slot,child_id,artifact_id,dialect,revision,snapshot}=entry;match snapshot.try_return_to_registry_witness(grant){Ok((returned,progress))=>{*self.returned=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(returned).unwrap_or_else(|_|unreachable!("original returned read defines full closure")));*self.metadata=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(ChildContentReadMetadata{owner,slot,child_id,artifact_id,dialect,revision}).unwrap_or_else(|_|unreachable!("original child read metadata defines full closure")));return Ok(RetainedCloneStep::Progress(progress))},Err((error,snapshot))=>{*self.pending=Some(ChildContentEntry{owner,slot,child_id,artifact_id,dialect,revision,snapshot});return Err(error)}}}
  match self.view.take_one(&ChildContentView::EMPTY,&ChildContentReadAliases){ChildContentTake::Blocked=>Ok(RetainedCloneStep::Progress(empty)),ChildContentTake::Snapshot(entry,released_bytes)=>{*self.pending=Some(entry);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..empty}))},ChildContentTake::Released(released_bytes)=>Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..empty})),ChildContentTake::Complete(released_bytes)=>Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes,..empty}))}
 }
 pub fn terminal_is_empty(&self)->bool{self.view.root.is_none()&&self.pending.is_none()&&self.returned.is_none()&&self.metadata.is_none()}
}
impl semio_framework_value::retirement::RetirementCursor for ChildContentReadRetirement {
 fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep;match self.step(grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Progress(p)|RetainedCloneStep::Complete(p))if p!=Default::default()=>RetirementStep::Progress(p),Ok(RetainedCloneStep::Complete(_))=>RetirementStep::Complete,Ok(_)=>RetirementStep::BudgetExhausted}}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl semio_framework_value::retirement::RetireOwned for ChildContentView {
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(ChildContentReadRetirement::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<ChildContentReadRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl Drop for ChildContentReadRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original child view abandoned exact read registry custody");if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.view);std::mem::ManuallyDrop::drop(&mut self.pending);std::mem::ManuallyDrop::drop(&mut self.returned);std::mem::ManuallyDrop::drop(&mut self.metadata);}}}}
