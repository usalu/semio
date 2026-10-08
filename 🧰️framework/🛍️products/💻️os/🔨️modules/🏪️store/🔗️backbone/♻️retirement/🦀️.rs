//! ♻️ Original transport queues and message backing remain attached until physical release admission.
use super::{Backbones, BackboneMessage, ErasedSnapshotRetirement, VecDeque, Arc, Mutex};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::shared::shared_retirement_allocation_bytes};
use std::mem::ManuallyDrop;

type Wake = std::sync::OnceLock<Arc<dyn Fn() + Send + Sync>>;
type Queue = Mutex<VecDeque<BackboneMessage>>;

pub(super) struct ArtifactStoreBackboneRetirement {
    backbone: ManuallyDrop<Option<Backbones>>,
    queue: ManuallyDrop<Option<VecDeque<BackboneMessage>>>,
    message: ManuallyDrop<Option<BackboneMessage>>,
    bytes: ManuallyDrop<Option<Vec<u8>>>,
    wake: ManuallyDrop<Option<Arc<Wake>>>,
}

impl ArtifactStoreBackboneRetirement {
    pub(super) fn new(backbone: Backbones) -> Self { Self { backbone: ManuallyDrop::new(Some(backbone)), queue: ManuallyDrop::new(None), message: ManuallyDrop::new(None), bytes: ManuallyDrop::new(None), wake: ManuallyDrop::new(None) } }
    pub(super) fn from_queue(queue: VecDeque<BackboneMessage>) -> Self { Self { backbone: ManuallyDrop::new(None), queue: ManuallyDrop::new(Some(queue)), message: ManuallyDrop::new(None), bytes: ManuallyDrop::new(None), wake: ManuallyDrop::new(None) } }
    pub(super) const fn constructor_capacity_bytes() -> usize { size_of::<Self>() }
    pub(super) fn admit_queue(queue: VecDeque<BackboneMessage>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, VecDeque<BackboneMessage>)> { Self::admit_frame(queue, grant, Self::from_queue) }
    pub(super) fn admit_backbone(backbone: Backbones, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Backbones)> { Self::admit_frame(backbone, grant, Self::new) }
    pub(super) fn admit_attached(queue: &mut VecDeque<BackboneMessage>, backbone: &mut Option<Backbones>, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        if queue.capacity() != 0 {
            return match Self::admit_queue(std::mem::take(queue), grant) {
                Ok((owner, progress)) => Ok((Some(owner), progress)),
                Err((error, original)) => { *queue = original; Err(error) }
            };
        }
        let Some(original) = backbone.take() else { return Ok((None, RetainedCloneProgress { copied_items: usize::from(grant.maximum_items != 0 && grant.maximum_depth != 0), ..Default::default() })); };
        match Self::admit_backbone(original, grant) {
            Ok((owner, progress)) => Ok((Some(owner), progress)),
            Err((error, original)) => { *backbone = Some(original); Err(error) }
        }
    }
    fn admit_frame<T>(original: T, grant: RetainedCloneGrant, create: impl FnOnce(T) -> Self) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)> {
        let refusal = if grant.maximum_items == 0 { Some((ValueRefusalKind::WorkLimit, "backbone frame requires one admitted item")) }
        else if grant.maximum_depth == 0 { Some((ValueRefusalKind::DepthLimit, "backbone frame requires admitted depth")) }
        else if grant.maximum_capacity_bytes < Self::constructor_capacity_bytes() { Some((ValueRefusalKind::OwnershipLimit, "backbone frame exceeds admitted capacity")) } else { None };
        if let Some((kind, message)) = refusal { return Err((ValueError::literal(kind, message), original)); }
        let layout = std::alloc::Layout::new::<Self>();
        let Some(pointer) = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<Self>()) else { return Err((ValueError::literal(ValueRefusalKind::AllocationFailed, "backbone frame allocation failed"), original)); };
        let owner = unsafe { pointer.as_ptr().write(create(original)); Box::from_raw(pointer.as_ptr()) };
        Ok((owner, RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: layout.size(), ..Default::default() }))
    }
    fn transfer_string(value: &mut String) -> Option<Vec<u8>> { (value.capacity() != 0).then(|| std::mem::take(value).into_bytes()) }
    fn message_release(message: &BackboneMessage) -> usize {
        match message {
            BackboneMessage::Ack { op_ids } | BackboneMessage::Retract { mutation_ids: op_ids } if op_ids.is_empty() => op_ids.capacity() * size_of::<String>(),
            _ => 0,
        }
    }
    pub(super) fn demands(&self) -> Result<RetirementDemand, ValueError> {
        let release_bytes = if let Some(bytes) = self.bytes.as_ref() { bytes.capacity() }
        else if let Some(message) = self.message.as_ref() { Self::message_release(message) }
        else if let Some(queue) = self.queue.as_ref() { if queue.is_empty() { queue.capacity() * size_of::<BackboneMessage>() } else { 0 } }
        else if let Some(wake) = self.wake.as_ref() {
            if wake.get().is_some() { return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "backbone wake callback has no admitted retirement authority")); }
            shared_retirement_allocation_bytes::<Wake>()
        } else if let Some(backbone) = self.backbone.as_ref() {
            match backbone {
                Backbones::Port(value) if value.uri.capacity() == 0 && value.channel.is_some() => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "external backbone channel has no admitted retirement authority")),
                Backbones::Memory(value) if value.uri.capacity() == 0 && (value.inbox.is_some() || value.outbox.is_some()) => shared_retirement_allocation_bytes::<Queue>(),
                Backbones::Channel(value) if value.uri.capacity() == 0 && (value.inbound.is_some() || value.outbound.is_some()) => shared_retirement_allocation_bytes::<Queue>(),
                _ => 0,
            }
        } else { 0 };
        Ok(RetirementDemand { release_bytes, depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
    fn detach_queue(owner: &mut Option<Arc<Queue>>) -> Option<VecDeque<BackboneMessage>> {
        let queue = owner.as_ref()?;
        if Arc::strong_count(queue) != 1 || Arc::weak_count(queue) != 0 { return None; }
        match Arc::try_unwrap(owner.take().unwrap()) {
            Ok(queue) => Some(queue.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner)),
            Err(queue) => { *owner = Some(queue); None }
        }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreBackboneRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands()?;
        if grant.maximum_depth < demand.depth || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty };
        if self.bytes.is_some() { drop(self.bytes.take()); }
        else if let Some(message) = self.message.as_mut() {
            match message {
                BackboneMessage::Genesis { pack } | BackboneMessage::Mutations { envelopes: pack } if pack.capacity() != 0 => { *self.bytes = Some(std::mem::take(pack)); }
                BackboneMessage::Ack { op_ids } | BackboneMessage::Retract { mutation_ids: op_ids } => {
                    if let Some(id) = op_ids.pop() { *self.bytes = Some(id.into_bytes()); }
                    else if op_ids.capacity() != 0 { drop(std::mem::take(op_ids)); }
                    else { drop(self.message.take()); }
                }
                BackboneMessage::Member { owner, slot, child_id, envelopes } => {
                    if envelopes.capacity() != 0 { *self.bytes = Some(std::mem::take(envelopes)); }
                    else if let Some(bytes) = Self::transfer_string(owner).or_else(|| Self::transfer_string(slot)).or_else(|| Self::transfer_string(child_id)) { *self.bytes = Some(bytes); }
                    else { drop(self.message.take()); }
                }
                _ => { drop(self.message.take()); }
            }
        } else if let Some(queue) = self.queue.as_mut() {
            if let Some(message) = queue.pop_front() { *self.message = Some(message); }
            else { drop(self.queue.take()); }
        } else if let Some(wake) = self.wake.as_ref() {
            if Arc::strong_count(wake) != 1 || Arc::weak_count(wake) != 0 { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            match Arc::try_unwrap(self.wake.take().unwrap()) {
                Ok(wake) => drop(wake),
                Err(wake) => { *self.wake = Some(wake); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            }
        } else {
            let backbone = self.backbone.as_mut().unwrap();
            let uri = match backbone { Backbones::Port(value) => &mut value.uri, Backbones::Memory(value) => &mut value.uri, Backbones::Channel(value) => &mut value.uri };
            if let Some(bytes) = Self::transfer_string(uri) { *self.bytes = Some(bytes); }
            else {
                let queue = match backbone {
                    Backbones::Memory(value) => if value.inbox.is_some() { Some(&mut value.inbox) } else if value.outbox.is_some() { Some(&mut value.outbox) } else { None },
                    Backbones::Channel(value) => if value.inbound.is_some() { Some(&mut value.inbound) } else if value.outbound.is_some() { Some(&mut value.outbound) } else { None },
                    Backbones::Port(_) => None,
                };
                if let Some(queue) = queue {
                    if let Some(queue) = Self::detach_queue(queue) { *self.queue = Some(queue); }
                    else { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
                } else if let Some(Backbones::Channel(value)) = self.backbone.take() { *self.wake = Some(value.outbound_wake); }
                else { drop(self.backbone.take()); }
            }
        }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
    fn terminal_is_empty(&self) -> bool { self.backbone.is_none() && self.queue.is_none() && self.message.is_none() && self.bytes.is_none() && self.wake.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands()?.copy_bytes) }
    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.demands()?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands()?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands()?.depth) }
}

impl Drop for ArtifactStoreBackboneRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "backbone retirement abandoned original transport ownership");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.backbone); ManuallyDrop::drop(&mut self.queue); ManuallyDrop::drop(&mut self.message); ManuallyDrop::drop(&mut self.bytes); ManuallyDrop::drop(&mut self.wake); } }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
