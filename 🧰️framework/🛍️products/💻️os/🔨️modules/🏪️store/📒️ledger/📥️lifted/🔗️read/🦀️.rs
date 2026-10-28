//! 📥️ Items of `snapshot_read` that belong to `os-store-ledger` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🔗️read/🦀️.rs`.
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::ValueError;
use semio_framework_os_spr::{Edit, Mutation, MutationMeta};
use semio_framework_os_vcs::Author;
use semio_framework_os_vcs::{Alternative, Change, Checkpoint};
use semio_framework_value_snapshot_read::*;
pub struct ArtifactStoreStringRetirement {
    pub owner: semio_framework_value::retirement::controlled::ControlledRetirement<String>,
}

impl ArtifactStoreStringRetirement {
    pub fn new(value: String) -> Self {
        Self { owner: semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_|unreachable!("String declares controlled retirement")) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreStringRetirement {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.owner.next_demand(body) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.owner.step(grant)}
    fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(copy)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
}

pub struct ArtifactStoreEditRetirement<Mutation> {
    pub forwards: std::mem::ManuallyDrop<Option<Vec<Mutation>>>,
    pub inverse: std::mem::ManuallyDrop<Option<semio_framework_value_persistent::list::PagedList<Mutation, {usize::MAX}>>>,
    pub state: std::mem::ManuallyDrop<Option<ArtifactStoreEditRetirementState>>,
}

pub type ArtifactStoreEditMetadata = (String, Option<semio_framework_value::SharedUtf8>, (Option<String>, Vec<MutationMeta>), (Option<String>, String, Option<String>));

pub type ArtifactStoreEditRetirementState = semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactStoreEditMetadata>;

impl<Mutation> ArtifactStoreEditRetirement<Mutation> {
    pub fn new(edit: Edit<Mutation>) -> Result<Self, Edit<Mutation>> {
        if !edit.forwards.is_empty() || !edit.inverse.is_empty() || edit.inverse.has_staged_root() { return Err(edit); }
        let Edit { id, actor, line, forwards, inverse, mutation_meta, verb, sequence_number: _, started_at, finished_at } = edit;
        let state=ArtifactStoreEditRetirementState::new((id,actor,(line,mutation_meta),(verb,started_at,finished_at))).unwrap_or_else(|_|unreachable!("original edit metadata declares controlled retirement"));
        Ok(Self { forwards:std::mem::ManuallyDrop::new(Some(forwards)),inverse:std::mem::ManuallyDrop::new(Some(inverse.into_original().ok().expect("settled original inverse"))),state:std::mem::ManuallyDrop::new(Some(state)) })
    }
    pub fn demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        use semio_framework_value::RetirementDemand;
        if let Some(state)=self.state.as_ref(){if state.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ArtifactStoreEditRetirementState>>(),depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:state.next_copy_byte_demand()?,capacity_bytes:state.next_capacity_byte_demand(copy)?,release_bytes:state.next_release_byte_demand()?,depth:state.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original edit metadata depth overflow"))?});}
        if let Some(forwards)=self.forwards.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<Vec<Mutation>>(),release_bytes:std::alloc::Layout::array::<Mutation>(forwards.capacity()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original empty forward backing overflow"))?.size(),depth:1,..Default::default()});}
        if let Some(inverse)=self.inverse.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_value_persistent::list::PagedList<Mutation,{usize::MAX}>>(),release_bytes:inverse.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()});}
        Ok(Default::default())
    }
}

impl<Mutation:Send+'static> ErasedSnapshotRetirement for ArtifactStoreEditRetirement<Mutation>{
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.demands(body) }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let zero=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(zero));}let demand=self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(zero));}
        if let Some(state)=self.state.as_mut(){if state.terminal_is_empty(){drop(self.state.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..zero}));}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=state.step(child)?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,state.terminal_is_empty(),"original edit native metadata")?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.forwards.is_some(){drop(self.forwards.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..zero}));}
        if let Some(inverse)=self.inverse.as_mut(){if inverse.terminal_is_empty(){drop(self.inverse.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..zero}));}let progress=inverse.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(progress.progressed),released_bytes:progress.released_allocation_bytes,..zero}));}
        Ok(RetainedCloneStep::Complete(zero))
    }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool{self.state.is_none()&&self.forwards.is_none()&&self.inverse.is_none()}
}

impl<Mutation> Drop for ArtifactStoreEditRetirement<Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.state.is_none() && self.forwards.is_none() && self.inverse.is_none()), "artifact store edit retirement reached Drop before every original typed owner and backing allocation closed");
        if self.state.is_none() && self.forwards.is_none() && self.inverse.is_none() { unsafe { std::mem::ManuallyDrop::drop(&mut self.state); std::mem::ManuallyDrop::drop(&mut self.forwards); std::mem::ManuallyDrop::drop(&mut self.inverse); } }
    }
}

pub struct ArtifactStoreHistoryMetadataRetirement {
    pub state: std::mem::ManuallyDrop<Option<ArtifactStoreHistoryMetadataRetirementState>>,
}

pub struct ArtifactStoreHistoryMetadataRetirementState {
    pub strings: [Option<String>; 8],
    string_list: Vec<String>,
    pub authors: Vec<Author>,
    pub pins: Vec<semio_framework_os_vcs::CompositionPin>,
    active_bytes: Option<Vec<u8>>,
}

impl ArtifactStoreHistoryMetadataRetirement {
    pub fn change(change: Change) -> Self {
        let Change { id, edit_ids, description, saved_at } = change;
        Self::from_parts([Some(id), description, Some(saved_at), None, None, None, None, None], edit_ids, Vec::new(), Vec::new())
    }

    pub fn checkpoint(checkpoint: Checkpoint) -> Self {
        let Checkpoint { id, change_ids, parent_id, authors, message, timestamp, composition_pins } = checkpoint;
        Self::from_parts([Some(id), parent_id, message, Some(timestamp), None, None, None, None], change_ids, authors, composition_pins)
    }

    pub fn alternative(alternative: Alternative) -> Self {
        let Alternative { id, name, checkpoint_ids } = alternative;
        Self::from_parts([Some(id), Some(name), None, None, None, None, None, None], checkpoint_ids, Vec::new(), Vec::new())
    }

    fn from_parts(strings: [Option<String>; 8], string_list: Vec<String>, authors: Vec<Author>, pins: Vec<semio_framework_os_vcs::CompositionPin>) -> Self {
        Self { state: std::mem::ManuallyDrop::new(Some(ArtifactStoreHistoryMetadataRetirementState { strings, string_list, authors, pins, active_bytes: None })) }
    }
}

impl ArtifactStoreHistoryMetadataRetirement {
    fn retained_backing_bytes(state: &ArtifactStoreHistoryMetadataRetirementState) -> Result<usize, ValueError> {
        let mut bytes = 0usize;
        for extent in [state.string_list.capacity().checked_mul(size_of::<String>()), state.authors.capacity().checked_mul(size_of::<Author>()), state.pins.capacity().checked_mul(size_of::<semio_framework_os_vcs::CompositionPin>())] {
            bytes = extent.and_then(|extent| bytes.checked_add(extent)).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history metadata backing extent overflow"))?;
        }
        Ok(bytes)
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreHistoryMetadataRetirement {
    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand {release_bytes:self.next_release_byte_demand()?,depth:usize::from(self.state.is_some()),..Default::default()})}
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        let Some(state) = self.state.as_mut() else { return Ok(RetainedCloneStep::Complete(empty)); };
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history metadata requires admitted depth")); }
        if let Some(bytes) = state.active_bytes.as_ref() {
            let released_bytes = bytes.capacity();
            if grant.maximum_release_bytes < released_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            drop(state.active_bytes.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }));
        }
        if let Some(value) = state.strings.iter_mut().rev().find_map(Option::take).or_else(|| state.string_list.pop()) {
            state.active_bytes = Some(value.into_bytes());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(author) = state.authors.pop() {
            state.strings[0] = Some(author.id);
            state.strings[1] = Some(author.name);
            state.strings[2] = author.avatar;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(pin) = state.pins.pop() {
            state.strings[0] = Some(pin.child_ref.artifact_id);
            state.strings[1] = Some(pin.child_ref.dialect.artifact_kind);
            state.strings[2] = Some(pin.child_ref.dialect.standard);
            state.strings[3] = Some(pin.child_ref.dialect.subset);
            state.strings[4] = Some(pin.checkpoint_id);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let released_bytes = Self::retained_backing_bytes(state)?;
        if grant.maximum_release_bytes < released_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        drop(self.state.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.state.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        let Some(state) = self.state.as_ref() else { return Ok(0); };
        if let Some(bytes) = state.active_bytes.as_ref() { return Ok(bytes.capacity()); }
        if state.strings.iter().any(Option::is_some) || !state.string_list.is_empty() || !state.authors.is_empty() || !state.pins.is_empty() { return Ok(0); }
        Self::retained_backing_bytes(state)
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.state.is_some())) }
}

impl Drop for ArtifactStoreHistoryMetadataRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.state.is_none()), "artifact store history metadata retirement reached Drop before every nested owner was terminal-empty");
    }
}

/// 🧾️ Retains the actual native forward or inverse collection without rebuilding its scaffold.
pub enum ArtifactStoreOperationRowOwners<Mu> {
    Forwards(Vec<Mu>),
    Inverse(semio_framework_value_persistent::list::PagedList<Mu, {usize::MAX}>),
}

impl<Mu> From<Vec<Mu>> for ArtifactStoreOperationRowOwners<Mu> {
    fn from(rows: Vec<Mu>) -> Self { Self::Forwards(rows) }
}

pub struct ArtifactStoreDisplacedOwnerReservation {
    pub slot: u8,
    pub generation: u64,
    pub remaining: usize,
}
