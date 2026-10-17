//! 🗂️ World3d dynamic, mesh and draw registries, opaque quarantine, session and interaction state.

use super::*;

//#region World3dState
/// 🎟️ Funds one original World mesh turn with independent ownership currencies.
pub fn world_mesh_ownership_grant(copy: usize, capacity: usize, release: usize, depth: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth }
}

/// 🧾️ Verifies the original owner receipt before World advances its publication or close phase.
pub fn world_mesh_ownership_complete(grant: semio_framework_value::retained_clone::RetainedCloneGrant, step: ui_wgpu::wgpu::Mesh3dOwnershipStep) -> Result<bool, ui_wgpu::wgpu::Mesh3dFault> {
    if !step.progress.fits(grant) { return Err(ui_wgpu::wgpu::Mesh3dFault::Closing); }
    Ok(step.complete)
}

pub const WORLD_DYNAMIC_MESH_CAPACITY: usize = 256;

const WORLD_DYNAMIC_DRAW_CAPACITY: usize = 256;

pub const WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY: usize = 4_096;

pub const WORLD_DYNAMIC_DRAW_BYTE_CAPACITY: usize = 16 * 1024 * 1024;

pub const WORLD_DYNAMIC_PIXEL_CAPACITY: usize = 256;

const WORLD_OPAQUE_QUARANTINE_CAPACITY: usize = 1024;

pub const WORLD_DYNAMIC_ID_BYTE_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldDynamicToken {
    pub slot: u16,
    pub epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldDynamicFault {
    Mesh(ui_wgpu::wgpu::Mesh3dFault),
    Ownership(semio_framework_value::ValueRefusalKind),
    Canceled,
    IdCapacity,
    RegistryCapacity,
    StaleToken,
    QuarantineCapacity,
    Closing,
    InstanceCapacity,
    ByteCapacity,
}

#[derive(Debug)]
pub struct WorldDynamicEntry<T> {
    pub id: String,
    pub epoch: u64,
    pub value: T,
}

#[derive(Debug)]
pub struct WorldDynamicRejected<T> {
    pub fault: WorldDynamicFault,
    pub id: String,
    pub value: T,
}

pub struct WorldDynamicRegistry<T, const N: usize> {
    pub slots: Box<[Option<WorldDynamicEntry<T>>; N]>,
    pub epochs: [u64; N],
    pub len: u16,
    pub closing: bool,
}

#[derive(Debug)]
pub enum WorldMeshSource {
    Owned{lease:Option<Mesh3dLease>,closing:bool},
    Native(WorldNativeMeshSource),
}

#[derive(Debug)]
pub struct WorldNativeMeshSource {
    pub loan: WorldNativeMeshLoan,
    pub receipt: semio_framework::kernel::ActorUiPatchReceipt,
    pub revision: WorldNativeUiRevision,
    pub resource_ordinal: usize,
    pub lookup_extent: usize,
    pub frozen: bool,
    pub gpu_returned: bool,
}

impl WorldMeshSource {
    pub fn lease(&self)->Option<&Mesh3dLease>{match self{Self::Owned{lease,..}=>lease.as_ref(),Self::Native(source) if !source.frozen=>source.loan.lease_ref(),Self::Native(_)=>None}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ui_wgpu::wgpu::Mesh3dFault>{match self{
        Self::Owned{lease,closing}=>{let Some(value)=*lease else{return Ok((true,Default::default()))};if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}if !*closing{match mesh3d_begin_close(value){Ok(())|Err(ui_wgpu::wgpu::Mesh3dFault::Closing)=>*closing=true,Err(ui_wgpu::wgpu::Mesh3dFault::Stale)=>{lease.take();return Ok((true,RetainedCloneProgress{copied_items:1,..Default::default()}))},Err(fault)=>return Err(fault)}return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))}let step=mesh3d_close_step(value,grant)?;if step.complete{lease.take();}Ok((step.complete,step.progress))},
        Self::Native(source)=>{if !source.gpu_returned{return Ok((false,Default::default()))}let step=source.loan.return_step(grant)?;Ok((step.complete,step.progress))}
    }}
}

#[derive(Default)]
pub struct WorldMeshRegistry(pub WorldDynamicRegistry<WorldMeshSource,WORLD_DYNAMIC_MESH_CAPACITY>);

impl std::ops::Deref for WorldMeshRegistry{type Target=WorldDynamicRegistry<WorldMeshSource,WORLD_DYNAMIC_MESH_CAPACITY>;fn deref(&self)->&Self::Target{&self.0}}

impl std::ops::DerefMut for WorldMeshRegistry{fn deref_mut(&mut self)->&mut Self::Target{&mut self.0}}

impl WorldMeshRegistry {
    pub fn get(&self,id:&str)->Option<&Mesh3dLease>{self.0.get(id)?.lease()}
    #[cfg(any(test, feature = "testkit"))]
    pub fn iter(&self)->impl Iterator<Item=(&String,&Mesh3dLease)>{self.0.slots.iter().flatten().filter_map(|entry|entry.value.lease().map(|lease|(&entry.id,lease)))}
    #[cfg(any(test, feature = "testkit"))]
    pub fn insert(&mut self,id:String,value:Mesh3dLease)->Result<(WorldDynamicToken,Option<WorldDynamicEntry<WorldMeshSource>>),WorldDynamicRejected<Mesh3dLease>>{self.0.insert(id,WorldMeshSource::Owned{lease:Some(value),closing:false}).map_err(|rejected|WorldDynamicRejected{fault:rejected.fault,id:rejected.id,value})}
    pub fn commit_insert(&mut self,plan:WorldDynamicInsertPlan,id:String,value:Mesh3dLease)->Result<(WorldDynamicToken,Option<WorldDynamicEntry<WorldMeshSource>>),WorldDynamicRejected<Mesh3dLease>>{self.0.commit_insert(plan,id,WorldMeshSource::Owned{lease:Some(value),closing:false}).map_err(|rejected|WorldDynamicRejected{fault:rejected.fault,id:rejected.id,value})}
}

pub struct WorldNativeMeshAdmission {
    pub identity:ui_wgpu::wgpu::kernel_3d_scene::Mesh3dResourceIdentity,
    pub lease:Mesh3dLease,
    pub receipt:semio_framework::kernel::ActorUiPatchReceipt,
    pub revision:WorldNativeUiRevision,
    pub resource_ordinal:usize,
    pub key:String,
    pub extent:usize,
    pub interaction_id:WorldInteractionId,
    pub interaction_offset:usize,
    pub allocated:bool,
}

#[derive(Debug)]
pub enum WorldNativeMeshFailure { Binding, Registry(WorldDynamicFault), Mesh(ui_wgpu::wgpu::Mesh3dFault), Gpu(&'static str), Close(semio_framework_value::ValueError) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WorldDynamicInsertPlan {
    pub slot: u16,
    pub epoch: u64,
    pub occupied: bool,
}

impl<T, const N: usize> Default for WorldDynamicRegistry<T, N> {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; N], len: 0, closing: false }
    }
}

impl<T, const N: usize> WorldDynamicRegistry<T, N> {
    pub fn get(&self, id: &str) -> Option<&T> {
        self.slots.iter().flatten().find(|entry| entry.id == id).map(|entry| &entry.value)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        self.slots.iter_mut().flatten().find(|entry| entry.id == id).map(|entry| &mut entry.value)
    }

    pub fn contains_key(&self, id: &str) -> bool {
        self.get(id).is_some()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.slots.iter().flatten().map(|entry| &entry.id)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.slots.iter().flatten().map(|entry| (&entry.id, &entry.value))
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn token(&self, id: &str) -> Option<WorldDynamicToken> {
        self.slots.iter().enumerate().find_map(|(slot, entry)| entry.as_ref().filter(|entry| entry.id == id).map(|entry| WorldDynamicToken { slot: slot as u16, epoch: entry.epoch }))
    }

    pub fn insert(&mut self, id: String, value: T) -> Result<(WorldDynamicToken, Option<WorldDynamicEntry<T>>), WorldDynamicRejected<T>> {
        if self.closing {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value });
        }
        if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::IdCapacity, id, value });
        }
        if let Some((slot, entry)) = self.slots.iter_mut().enumerate().find(|(_, entry)| entry.as_ref().is_some_and(|entry| entry.id == id)) {
            self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
            let epoch = self.epochs[slot];
            let previous = entry.replace(WorldDynamicEntry { id, epoch, value });
            return Ok((WorldDynamicToken { slot: slot as u16, epoch }, previous));
        }
        let Some(slot) = self.slots.iter().position(Option::is_none) else {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::RegistryCapacity, id, value });
        };
        self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
        let epoch = self.epochs[slot];
        self.slots[slot] = Some(WorldDynamicEntry { id, epoch, value });
        self.len += 1;
        Ok((WorldDynamicToken { slot: slot as u16, epoch }, None))
    }

    pub fn plan_insert(&self, id: &str) -> Result<WorldDynamicInsertPlan, WorldDynamicFault> {
        if self.closing {
            return Err(WorldDynamicFault::Closing);
        }
        if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(WorldDynamicFault::IdCapacity);
        }
        if let Some((slot, entry)) = self.slots.iter().enumerate().find_map(|(slot, entry)| entry.as_ref().filter(|entry| entry.id == id).map(|entry| (slot, entry))) {
            return Ok(WorldDynamicInsertPlan { slot: slot as u16, epoch: entry.epoch, occupied: true });
        }
        let Some(slot) = self.slots.iter().position(Option::is_none) else {
            return Err(WorldDynamicFault::RegistryCapacity);
        };
        Ok(WorldDynamicInsertPlan { slot: slot as u16, epoch: self.epochs[slot], occupied: false })
    }

    pub fn commit_insert(&mut self, plan: WorldDynamicInsertPlan, id: String, value: T) -> Result<(WorldDynamicToken, Option<WorldDynamicEntry<T>>), WorldDynamicRejected<T>> {
        if self.closing {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value });
        }
        let slot = usize::from(plan.slot);
        let Some(entry) = self.slots.get(slot) else {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::StaleToken, id, value });
        };
        let matches = if plan.occupied { entry.as_ref().is_some_and(|entry| entry.id == id && entry.epoch == plan.epoch) } else { entry.is_none() && self.epochs[slot] == plan.epoch };
        if !matches {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::StaleToken, id, value });
        }
        let epoch = self.epochs[slot].wrapping_add(1).max(1);
        self.epochs[slot] = epoch;
        let previous = self.slots[slot].replace(WorldDynamicEntry { id, epoch, value });
        if previous.is_none() {
            self.len += 1;
        }
        Ok((WorldDynamicToken { slot: plan.slot, epoch }, previous))
    }

    pub fn remove(&mut self, id: &str) -> Option<WorldDynamicEntry<T>> {
        let slot = self.slots.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.id == id))?;
        self.len -= 1;
        self.slots[slot].take()
    }

    pub fn remove_token(&mut self, token: WorldDynamicToken) -> Result<WorldDynamicEntry<T>, WorldDynamicFault> {
        let slot = usize::from(token.slot);
        let Some(entry) = self.slots.get(slot).and_then(Option::as_ref) else {
            return Err(WorldDynamicFault::StaleToken);
        };
        if entry.epoch != token.epoch {
            return Err(WorldDynamicFault::StaleToken);
        }
        self.len -= 1;
        Ok(self.slots[slot].take().expect("validated world dynamic slot"))
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn take_one(&mut self) -> Option<WorldDynamicEntry<T>> {
        let slot = self.slots.iter().position(Option::is_some)?;
        self.len -= 1;
        self.slots[slot].take()
    }

    fn restore(&mut self, entry: WorldDynamicEntry<T>) {
        let slot = self.slots.iter().position(Option::is_none).expect("world registry restores only an owner detached from itself");
        self.slots[slot] = Some(entry);
        self.len += 1;
    }
}

#[cfg(not(any(test, feature = "testkit")))]
impl<T, const N: usize> Drop for WorldDynamicRegistry<T, N> {
    fn drop(&mut self) {
        assert!(self.is_empty(), "world dynamic registry reached Drop before its exact terminal-empty witness");
    }
}

pub struct WorldDrawRegistry {
    pub slots: Box<[Option<WorldDynamicEntry<SceneDraw3d>>; WORLD_DYNAMIC_DRAW_CAPACITY]>,
    pub epochs: [u64; WORLD_DYNAMIC_DRAW_CAPACITY],
    pub len: u16,
    pub closing: bool,
}

impl Default for WorldDrawRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; WORLD_DYNAMIC_DRAW_CAPACITY], len: 0, closing: false }
    }
}

impl WorldDrawRegistry {
    pub fn get(&self, index: usize) -> Option<&SceneDraw3d> {
        self.slots.get(index).and_then(Option::as_ref).map(|entry| &entry.value)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut SceneDraw3d> {
        self.slots.get_mut(index).and_then(Option::as_mut).map(|entry| &mut entry.value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SceneDraw3d> {
        self.slots[..usize::from(self.len)].iter().filter_map(|entry| entry.as_ref().map(|entry| &entry.value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SceneDraw3d> {
        self.slots[..usize::from(self.len)].iter_mut().filter_map(|entry| entry.as_mut().map(|entry| &mut entry.value))
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn push(&mut self, value: SceneDraw3d) -> Result<WorldDynamicToken, WorldDynamicRejected<SceneDraw3d>> {
        let id = value.mesh_key.clone();
        if self.closing {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value });
        }
        if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::IdCapacity, id, value });
        }
        if value.instances.len() > WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::InstanceCapacity, id, value });
        }
        let mut bytes = id.len();
        for instance in &value.instances {
            if instance.id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
                return Err(WorldDynamicRejected { fault: WorldDynamicFault::IdCapacity, id, value });
            }
            let Some(next) = bytes.checked_add(instance.id.len()).and_then(|bytes| bytes.checked_add(size_of::<Instance3d>())) else {
                return Err(WorldDynamicRejected { fault: WorldDynamicFault::ByteCapacity, id, value });
            };
            if next > WORLD_DYNAMIC_DRAW_BYTE_CAPACITY {
                return Err(WorldDynamicRejected { fault: WorldDynamicFault::ByteCapacity, id, value });
            }
            bytes = next;
        }
        self.push_prevalidated(value)
    }

    pub fn push_prevalidated(&mut self, value: SceneDraw3d) -> Result<WorldDynamicToken, WorldDynamicRejected<SceneDraw3d>> {
        let id = value.mesh_key.clone();
        if self.closing {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value });
        }
        let slot = usize::from(self.len);
        if slot == WORLD_DYNAMIC_DRAW_CAPACITY {
            return Err(WorldDynamicRejected { fault: WorldDynamicFault::RegistryCapacity, id, value });
        }
        self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
        let epoch = self.epochs[slot];
        self.slots[slot] = Some(WorldDynamicEntry { id, epoch, value });
        self.len += 1;
        Ok(WorldDynamicToken { slot: slot as u16, epoch })
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[cfg(any(test, feature = "testkit"))]
    fn clear_into_quarantine(&mut self) -> Result<(), WorldDynamicFault> {
        while let Some(entry) = self.take_last() {
            if let Err(entry) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                self.slots[usize::from(self.len)] = Some(match entry {
                    WorldOpaqueOwner::Draw(entry) => entry,
                    _ => unreachable!("draw quarantine returned draw owner"),
                });
                self.len += 1;
                return Err(WorldDynamicFault::QuarantineCapacity);
            }
        }
        Ok(())
    }

    pub fn take_last(&mut self) -> Option<WorldDynamicEntry<SceneDraw3d>> {
        let len = usize::from(self.len);
        if len == 0 {
            return None;
        }
        self.len -= 1;
        self.slots[len - 1].take()
    }

    pub fn restore_last(&mut self, entry: WorldDynamicEntry<SceneDraw3d>) {
        let slot = usize::from(self.len);
        self.slots[slot] = Some(entry);
        self.len += 1;
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldDrawRebuildDescriptor {
    pub generation: u64,
    pub revision: u64,
    pub draw_count: u16,
    pub instance_count: u32,
    pub byte_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldDrawRebuildStep {
    Pending,
    Complete,
    Stale,
    Fault,
}

struct WorldDrawDraft {
    pub mesh_key: String,
    pub mesh_version: u64,
    pub slots: Box<[MaybeUninit<Instance3d>]>,
    pub admitted: usize,
    taken: usize,
}

impl WorldDrawDraft {
    pub fn new(mesh_key: String, mesh_version: u64, instance_count: usize) -> Self {
        Self { mesh_key, mesh_version, slots: Box::<[Instance3d]>::new_uninit_slice(instance_count), admitted: 0, taken: 0 }
    }

    pub fn push(&mut self, instance: Instance3d) -> Result<(), Instance3d> {
        if self.admitted == self.slots.len() {
            return Err(instance);
        }
        self.slots[self.admitted].write(instance);
        self.admitted += 1;
        Ok(())
    }

    pub fn take_next(&mut self) -> Option<Instance3d> {
        if self.taken == self.admitted {
            return None;
        }
        let instance = unsafe { self.slots[self.taken].assume_init_read() };
        self.taken += 1;
        Some(instance)
    }

    pub fn complete(&self) -> bool {
        self.admitted == self.slots.len()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.taken == self.admitted
    }
}

impl Drop for WorldDrawDraft {
    fn drop(&mut self) {
        #[cfg(not(any(test, feature = "testkit")))]
        assert!(self.terminal_is_empty() || std::thread::panicking(), "WorldDrawDraft reached Drop before every admitted instance reached its terminal handback");
        #[cfg(any(test, feature = "testkit"))]
        while self.take_next().is_some() {}
    }
}

pub struct WorldDrawRebuildCursor {
    pub descriptor: WorldDrawRebuildDescriptor,
    pub drafts: Box<[Option<WorldDrawDraft>; WORLD_DYNAMIC_DRAW_CAPACITY]>,
    admitted_draws: u16,
    admitted_instances: u32,
    admitted_bytes: u32,
    pub publishing: bool,
    pub publish_draw: u16,
    pub output: Option<SceneDraw3d>,
    pub staged: WorldDrawRegistry,
    pub faulted: bool,
}

impl WorldDrawRebuildCursor {
    pub fn new(descriptor: WorldDrawRebuildDescriptor) -> Result<Self, WorldDynamicFault> {
        if usize::from(descriptor.draw_count) > WORLD_DYNAMIC_DRAW_CAPACITY || usize::try_from(descriptor.instance_count).ok().is_none_or(|count| count > WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY) {
            return Err(WorldDynamicFault::InstanceCapacity);
        }
        if usize::try_from(descriptor.byte_count).ok().is_none_or(|bytes| bytes > WORLD_DYNAMIC_DRAW_BYTE_CAPACITY) {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        Ok(Self {
            descriptor,
            drafts: semio_framework_async::boxed_fixed_slots(|| None),
            admitted_draws: 0,
            admitted_instances: 0,
            admitted_bytes: 0,
            publishing: false,
            publish_draw: 0,
            output: None,
            staged: WorldDrawRegistry::default(),
            faulted: false,
        })
    }

    pub fn admit_draw(&mut self, mesh_key: &str, mesh_version: u64, instance_count: u16) -> Result<(), WorldDynamicFault> {
        if self.publishing || self.faulted || self.admitted_draws == self.descriptor.draw_count {
            return Err(WorldDynamicFault::Closing);
        }
        if mesh_key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY || usize::from(instance_count) > WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY {
            return Err(if mesh_key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY { WorldDynamicFault::IdCapacity } else { WorldDynamicFault::InstanceCapacity });
        }
        let bytes = mesh_key.len().checked_add(size_of::<SceneDraw3d>()).ok_or(WorldDynamicFault::ByteCapacity)?;
        let next = usize::try_from(self.admitted_bytes).unwrap_or(usize::MAX).checked_add(bytes).ok_or(WorldDynamicFault::ByteCapacity)?;
        if next > usize::try_from(self.descriptor.byte_count).unwrap_or(0) {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        let slot = usize::from(self.admitted_draws);
        self.drafts[slot] = Some(WorldDrawDraft::new(mesh_key.to_owned(), mesh_version, usize::from(instance_count)));
        self.admitted_draws += 1;
        self.admitted_bytes = next as u32;
        Ok(())
    }

    pub fn admit_instance(&mut self, draw: u16, id: &str, model: Mat4, color: [f32; 4], color_source: SceneColorSource3d, selected: bool, hovered: bool, component_source: Option<ComponentSource3d>) -> Result<(), WorldDynamicFault> {
        if self.publishing || self.faulted || id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(if id.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY { WorldDynamicFault::IdCapacity } else { WorldDynamicFault::Closing });
        }
        let Some(draft) = self.drafts.get_mut(usize::from(draw)).and_then(Option::as_mut) else {
            return Err(WorldDynamicFault::StaleToken);
        };
        let bytes = id.len().saturating_add(size_of::<Instance3d>());
        let next = usize::try_from(self.admitted_bytes).unwrap_or(usize::MAX).saturating_add(bytes);
        if next > usize::try_from(self.descriptor.byte_count).unwrap_or(0) || self.admitted_instances == self.descriptor.instance_count {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        if draft.admitted == draft.slots.len() {
            return Err(WorldDynamicFault::InstanceCapacity);
        }
        let instance = Instance3d { component_source, id: id.to_owned(), model, color, selected, hovered, material: SceneInstanceMaterial3d { color_source, ..Default::default() } };
        draft.push(instance).map_err(|_| WorldDynamicFault::InstanceCapacity)?;
        self.admitted_instances += 1;
        self.admitted_bytes = next as u32;
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), WorldDynamicFault> {
        if self.admitted_draws != self.descriptor.draw_count
            || self.admitted_instances != self.descriptor.instance_count
            || self.admitted_bytes != self.descriptor.byte_count
            || self.drafts[..usize::from(self.admitted_draws)].iter().any(|draft| draft.as_ref().is_none_or(|draft| !draft.complete()))
        {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        self.publishing = true;
        Ok(())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.output.as_ref().is_none_or(|draw| draw.instances.is_empty()) && self.drafts.iter().flatten().all(WorldDrawDraft::terminal_is_empty) && self.staged.is_empty()
    }
}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldDrawRebuildCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "WorldDrawRebuildCursor reached Drop before every draft, instance, output, and staged draw reached terminal handback");
    }
}

impl<'a> IntoIterator for &'a WorldDrawRegistry {
    type Item = &'a SceneDraw3d;
    type IntoIter = Box<dyn Iterator<Item = &'a SceneDraw3d> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

impl<'a> IntoIterator for &'a mut WorldDrawRegistry {
    type Item = &'a mut SceneDraw3d;
    type IntoIter = Box<dyn Iterator<Item = &'a mut SceneDraw3d> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter_mut())
    }
}

impl Index<usize> for WorldDrawRegistry {
    type Output = SceneDraw3d;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("world draw index")
    }
}

impl IndexMut<usize> for WorldDrawRegistry {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("world draw index")
    }
}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldDrawRegistry {
    fn drop(&mut self) {
        assert!(self.is_empty() || std::thread::panicking(), "world draw registry reached Drop before its exact terminal-empty witness");
    }
}

/// 🖼️ Bounded identity and lease metadata; dropping it releases one token while the pool retains pixel storage.
#[derive(Debug)]
pub struct WorldSceneRaster {
    pub identity: SceneRasterIdentity,
    pub pending: Mutex<Option<SceneRasterLease>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct World3dPrimitiveMaterial {
    pub first_index: u32,
    pub index_count: u32,
    pub material: SceneAuthoredMaterial3d,
}

#[derive(Debug)]
pub struct World3dMeshAppearance {
    pub primitives: Vec<World3dPrimitiveMaterial>,
    pub textures: Vec<(String, WorldSceneRaster)>,
}

impl World3dMeshAppearance {
    pub fn new(primitives: Vec<World3dPrimitiveMaterial>, textures: Vec<(String, SceneRasterLease)>) -> Result<Self, (Vec<World3dPrimitiveMaterial>, Vec<(String, SceneRasterLease)>)> {
        if primitives.is_empty() || primitives.len() > WORLD_DYNAMIC_MESH_CAPACITY || textures.len() > WORLD_DYNAMIC_PIXEL_CAPACITY || primitives.iter().any(|primitive| primitive.index_count == 0) {
            return Err((primitives, textures));
        }
        if textures.iter().any(|(key, _)| key.is_empty() || key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY) {
            return Err((primitives, textures));
        }
        let textures = textures.into_iter().map(|(key, lease)| (key, WorldSceneRaster { identity: lease.identity(), pending: Mutex::new(Some(lease)) })).collect();
        Ok(Self { primitives, textures })
    }

    pub fn untextured(primitives: Vec<World3dPrimitiveMaterial>) -> Result<Self, Vec<World3dPrimitiveMaterial>> {
        match Self::new(primitives, Vec::new()) {
            Ok(appearance) => Ok(appearance),
            Err((primitives, _)) => Err(primitives),
        }
    }

    pub fn texture_upload(&self, key: &str, pool: &SceneRasterPool) -> Option<SceneRasterLease> {
        let (_, raster) = self.textures.iter().find(|(candidate, _)| candidate == key)?;
        if pool.gpu_resident(key, raster.identity) {
            raster.pending.lock().ok()?.take();
            return None;
        }
        raster.pending.lock().ok()?.take().or_else(|| pool.acquire(raster.identity).ok())
    }

    pub fn primitives(&self) -> &[World3dPrimitiveMaterial] {
        &self.primitives
    }

    pub fn texture_count(&self) -> usize {
        self.textures.len()
    }

    /// 🧹️ Releases one primitive or pending texture owner per call.
    pub fn close_step(&mut self) -> bool {
        if self.primitives.pop().is_some() {
            return false;
        }
        if self.textures.pop().is_some() {
            return false;
        }
        true
    }

    /// 🧾️ Exact terminal witness for request-owned appearance retirement.
    pub fn terminal_is_empty(&self) -> bool {
        self.primitives.is_empty() && self.textures.is_empty()
    }
}

#[derive(Debug)]
pub struct World3dMeshAsset {
    pub mesh: Mesh3dLease,
    pub appearance: World3dMeshAppearance,
}

impl World3dMeshAsset {
    /// 🧹️ Retires appearance owners before advancing the paged mesh close ladder.
    pub fn close_step(&mut self) -> bool {
        if !self.appearance.close_step() {
            return false;
        }
        match mesh3d_begin_close(self.mesh) {
            Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => {}
            Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => return true,
            Err(_) => return false,
        }
        ui_wgpu::wgpu::mesh3d_close_demands(self.mesh, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_close_step(self.mesh, grant)?) }).is_ok_and(|complete| complete) || mesh3d_terminal_is_empty(self.mesh)
    }

    /// 🧾️ Exact terminal witness for cancellation before World publication.
    pub fn terminal_is_empty(&self) -> bool {
        self.appearance.terminal_is_empty() && mesh3d_terminal_is_empty(self.mesh)
    }
}

#[derive(Debug)]
pub struct World3dMeshAssetRejected {
    pub fault: WorldDynamicFault,
    pub id: String,
    pub value: World3dMeshAsset,
}

pub enum WorldOpaqueOwner {
    Draw(WorldDynamicEntry<SceneDraw3d>),
}

pub struct WorldOpaqueQuarantine<const N: usize> {
    pub slots: Box<[Option<WorldOpaqueOwner>; N]>,
    pub len: u16,
    pub saturated: u64,
}

impl<const N: usize> Default for WorldOpaqueQuarantine<N> {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), len: 0, saturated: 0 }
    }
}

impl<const N: usize> WorldOpaqueQuarantine<N> {
    pub fn admit(&mut self, owner: WorldOpaqueOwner) -> Result<WorldDynamicToken, WorldOpaqueOwner> {
        let slot = usize::from(self.len);
        if slot == N {
            self.saturated = self.saturated.saturating_add(1);
            return Err(owner);
        }
        self.slots[slot] = Some(owner);
        self.len += 1;
        Ok(WorldDynamicToken { slot: slot as u16, epoch: 1 })
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn take_one(&mut self) -> Option<WorldOpaqueOwner> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.slots[usize::from(self.len)].take()
    }
}

static WORLD_OPAQUE_QUARANTINE: LazyLock<Mutex<WorldOpaqueQuarantine<WORLD_OPAQUE_QUARANTINE_CAPACITY>>> = LazyLock::new(|| Mutex::new(WorldOpaqueQuarantine::default()));

static WORLD_SCENE_RASTER_POOL: LazyLock<SceneRasterPool> = LazyLock::new(SceneRasterPool::new);

pub fn world_scene_raster_pool() -> SceneRasterPool {
    WORLD_SCENE_RASTER_POOL.clone()
}

pub fn quarantine_world_owner(owner: WorldOpaqueOwner) -> Result<WorldDynamicToken, WorldOpaqueOwner> {
    let Ok(mut quarantine) = WORLD_OPAQUE_QUARANTINE.lock() else {
        return Err(owner);
    };
    quarantine.admit(owner)
}

pub fn world3d_opaque_quarantine_status() -> (usize, u64) {
    WORLD_OPAQUE_QUARANTINE.lock().map_or((WORLD_OPAQUE_QUARANTINE_CAPACITY, u64::MAX), |quarantine| (usize::from(quarantine.len), quarantine.saturated))
}

/// 🌡️ What became of the scene's scalar field on the mesh it names, so the legend is shown only for a heatmap that exists
/// (`applied`) and a mismatch is announced instead of a wrong colouring — the React host's `data-world-scalar-field` status.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum World3dScalarFieldStatus {
    #[default]
    None,
    Applied,
    Mismatch,
    Missing,
}

/// 🖍️ The scene's sub-element highlight tokens resolved through the live theme; an absent entry keeps the engine's built-in palette.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WorldSubElementPaint {
    pub face_select: Option<[f32; 4]>,
    pub face_hover: Option<[f32; 4]>,
    pub edge_select: Option<[f32; 4]>,
    pub edge_hover: Option<[f32; 4]>,
    pub vertex_select: Option<[f32; 4]>,
    pub vertex_hover: Option<[f32; 4]>,
    pub vertex_mark_px: Option<f32>,
}

/// 🧩️ World3d session component: surface identity, bounds, domain binding, utilities and presentation of one 3D surface.
pub struct WorldSessionState {

    pub surface_id: String,

    pub controller_id: String,

    pub bounds: Rect,

    pub pick_bounds: Rect,

    /// 🪟️ The app-bound `InteractionDefinition` id for this window (see `World3dScene.domain_id`'s
    /// doc comment) — `None` means this window binds no app domain, so plain picks/hover target the
    /// OS's own shared `world` board domain (see `resolved_domain_id`).
    pub bound_domain_id: Option<String>,

    /// 🎯️ The bound domain's granularity id for a plain (non-component) pick/hover hit.
    pub bound_domain_granularity_id: Option<String>,

    pub active_utility: String,

    /// 📇️ The action ids the WINDOW KIND hosting this surface declares — the app manifest's
    /// `window_kind_action_refs`, republished by the shell on every engine-surface sync
    /// ([`set_world3d_declared_actions`]). A surface may only mint a verb that appears here.
    ///
    /// 🩸️ Without it the gumball was an unconditional affordance of every World3d surface, so the
    /// procedural VIEWER — whose `procedural-view-preview` kind declares only `exportDocument` and
    /// the camera verbs — offered a translate gizmo and published `translateSelection` into a window
    /// that refuses it, measured verbatim on 6118 as `handle_action promise failed: window kind
    /// procedural-view-preview does not own action translateSelection`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.2).
    /// Empty means "no window kind has declared anything for this surface yet", which offers nothing.
    pub declared_action_ids: Vec<String>,

    pub presentation: World3dPresentation,

    pub interaction_revision: u64,

    pub snapshot_fault: Option<World3dSnapshotFault>,

    pub modelling_locale: &'static str,
}

/// 🧩️ World3d interaction component: interaction mesh/object registries and instance interaction ids.
pub struct WorldInteractionState {

    /// 🎯️ Rendered instance id → the TOPOLOGY target it stands for, from the scene's own
    /// `instances[].interactionId`. Only the entries that actually differ are kept, so a lookup miss
    /// means "this instance is its own target" — see [`instance_interaction_id`].
    pub instance_interaction_ids: HashMap<String, String>,

    pub instance_interaction_granularity_ids: HashMap<String, String>,

    pub interaction_meshes: WorldInteractionMeshRegistry,

    pub interaction_objects: WorldInteractionObjectRegistry,
}

impl WorldSessionState {
    pub fn new(surface_id: String, controller_id: String) -> Self {
        Self {
            surface_id,
            controller_id,
            bounds: Rect::default(),
            pick_bounds: Rect::default(),
            bound_domain_id: None,
            bound_domain_granularity_id: None,
            active_utility: "select".into(),
            declared_action_ids: Vec::new(),
            presentation: World3dPresentation::default(),
            interaction_revision: 0,
            snapshot_fault: None,
            modelling_locale: "en",
        }
    }
}

impl WorldInteractionState {
    pub fn new() -> Self {
        Self {
            instance_interaction_ids: HashMap::new(),
            instance_interaction_granularity_ids: HashMap::new(),
            interaction_meshes: WorldInteractionMeshRegistry::default(),
            interaction_objects: WorldInteractionObjectRegistry::default(),
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn world_test_mesh_query<T>(mut query:impl FnMut()->Result<T,ui_wgpu::wgpu::Mesh3dFault>)->Result<T,ui_wgpu::wgpu::Mesh3dFault>{
    let deadline=std::time::Instant::now()+std::time::Duration::from_millis(5000);
    let cancel=std::sync::atomic::AtomicBool::new(false);
    for _ in 0..4096{
        if cancel.load(std::sync::atomic::Ordering::Acquire)||std::time::Instant::now()>=deadline{return Err(ui_wgpu::wgpu::Mesh3dFault::Busy);}
        match query(){Err(ui_wgpu::wgpu::Mesh3dFault::Busy)=>std::thread::yield_now(),result=>return result}
    }
    Err(ui_wgpu::wgpu::Mesh3dFault::Busy)
}
//#endregion World3dState
