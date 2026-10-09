// 🪪️ Ledger clones retain scalar identities while the paged registry owns every original source once.
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,list::PagedList};
const CAPACITY:usize=isize::MAX as usize;
/// 🪪️ A runtime-local monotone identity never contains or clones source bytes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RetainedSourceIdentity(u64);
struct Cell<C>{identity:RetainedSourceIdentity,owner:Option<C>,retired:bool}
/// 🗂️ The same original source remains owned through tick replacement, refusal, commit and host close.
pub struct RetainedSourceRegistry<C:ErasedSnapshotRetirement>{cells:PagedList<Cell<C>,CAPACITY>,next:u64,owners:usize,index:usize,closing:bool}
impl<C:ErasedSnapshotRetirement> RetainedSourceRegistry<C>{
    pub fn empty()->Self{Self{cells:PagedList::empty(),next:1,owners:0,index:0,closing:false}}
    fn invalid(detail:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,detail)}
    pub fn allocated_bytes(&self)->usize{self.cells.allocated_bytes()}
    pub fn retained_count(&self)->usize{self.owners}
    /// ➕️ The incoming unique owner transfers only after actual page admission and a reserved slot.
    pub fn insert(&mut self,incoming:&mut Option<C>,control:&mut NativeEncodeControl<'_>)->Result<RetainedSourceIdentity,ValueError>{
        if self.closing||incoming.is_none(){return Err(Self::invalid("source registry cannot accept this caller owner"))}let next=self.next.checked_add(1).ok_or_else(||Self::invalid("source identities exhausted"))?;let owners=self.owners.checked_add(1).ok_or_else(||Self::invalid("source owner count exhausted"))?;
        while !self.cells.has_reserved_slot(){let required=self.cells.next_allocation_bytes().map_err(ValueError::from)?;if required>4096{return Err(Self::invalid("source registry page exceeds physical authority"))}control.checkpoint()?;control.charge(required)?;let step=self.cells.reserve_one(4096).map_err(|error|ValueError::from(error.refusal()))?;if !step.progressed||step.allocated_bytes!=required{return Err(Self::invalid("source registry allocation differs from exact admission"))}}
        control.checkpoint()?;let identity=RetainedSourceIdentity(self.next);let cell=Cell{identity,owner:incoming.take(),retired:false};if let Err(mut cell)=self.cells.push_reserved(cell){*incoming=cell.owner.take();return Err(Self::invalid("source registry lost its admitted slot"))}self.next=next;self.owners=owners;Ok(identity)
    }
    pub fn borrow(&self,identity:RetainedSourceIdentity)->Option<&C>{if self.closing{return None}let index=usize::try_from(identity.0.checked_sub(1)?).ok()?;let cell=self.cells.get(index)?;if cell.identity!=identity||cell.retired{return None}cell.owner.as_ref()}
    /// 🔎️ Every absence decision must come from a fresh immutable accepted-ledger source.
    pub fn reconcile(&mut self,mut referenced:impl FnMut(RetainedSourceIdentity,&mut NativeEncodeControl<'_>)->Result<bool,ValueError>,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if self.closing{return Err(Self::invalid("retiring registry cannot reconcile"))}for cell in &mut self.cells{control.checkpoint()?;if cell.owner.is_some()&&!cell.retired{cell.retired=!referenced(cell.identity,control)?;}control.step()?;}self.index=0;Ok(())}
    /// 📤️ Transfers the actual unique owner; the physical cell stays retained as a tombstone.
    pub fn take(&mut self,identity:RetainedSourceIdentity,control:&mut NativeEncodeControl<'_>)->Result<C,ValueError>{if self.closing{return Err(Self::invalid("retiring registry cannot publish"))}control.checkpoint()?;let index=identity.0.checked_sub(1).and_then(|index|usize::try_from(index).ok()).ok_or_else(||Self::invalid("accepted identity exceeds registry addressability"))?;let cell=self.cells.get_mut(index).filter(|cell|cell.identity==identity).ok_or_else(||Self::invalid("accepted ledger names no retained source"))?;if cell.retired{return Err(Self::invalid("displaced source cannot publish"))}let owner=cell.owner.take().ok_or_else(||Self::invalid("source identity has already transferred"))?;self.owners-=1;Ok(owner)}
    pub fn begin_close(&mut self){self.closing=true;self.index=0;}
}
impl<C:ErasedSnapshotRetirement> RetainedSourceRegistry<C>{
 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(cell)=self.cells.get(self.index){if let Some(owner)=cell.owner.as_ref().filter(|_|cell.retired||self.closing){if !owner.terminal_is_empty(){return super::paged_owner::child_demands(owner,copy)}return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<C>>(),depth:1,..Default::default()})}return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<usize>(),depth:1,..Default::default()})}if !self.closing{return Ok(Default::default())}if !self.cells.is_empty(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Cell<C>>(),depth:1,..Default::default()})}if !self.cells.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:self.cells.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()})}Ok(Default::default())}
}
impl<C:ErasedSnapshotRetirement> ErasedSnapshotRetirement for RetainedSourceRegistry<C>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}let demand=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original child retirement exceeds caller depth"))}if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
  if self.index<self.cells.len(){let cell=self.cells.get_mut(self.index).unwrap();if (!cell.retired&&!self.closing)||cell.owner.is_none(){self.index+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}let owner=cell.owner.as_mut().unwrap();if !owner.terminal_is_empty(){return super::paged_owner::close_child(owner,grant)}cell.owner.take();self.owners-=1;self.index+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}
  if !self.closing{return Ok(RetainedCloneStep::Progress(empty))}if self.owners!=0{return Err(Self::invalid("source registry completed its original sweep with live owners"))}if self.cells.pop().is_some(){self.index=self.cells.len();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}let step=self.cells.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..empty}))
 }
 fn terminal_is_empty(&self)->bool{self.cells.terminal_is_empty()}

 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
impl<C:ErasedSnapshotRetirement> Drop for RetainedSourceRegistry<C>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"source registry retains original owners and exact metadata allocations");}}
