//! 🏛️ Mesh authority and api: the slots, the exclusive `Mesh3dAuthority` and the `mesh3d_*` begin, allocate, write, read, update, seal, abort and close entry points. See [`super`].

use super::*;

//#region Mesh
struct Mesh3dSlot {
    pub(super) epoch: u64,
    pub(super) reserved_pages: u16,
    pub(super) state: Mesh3dSlotState,
}

pub(super) struct Mesh3dAuthority {
    pub(super) slots: [Option<Mesh3dSlot>; MESH3D_AUTHORITY_CAPACITY],
    epochs: [u64; MESH3D_AUTHORITY_CAPACITY],
    pub(super) reserved_pages: usize,
}

impl Mesh3dAuthority {
    pub(super) const fn new() -> Self {
        Self { slots: [const {None};MESH3D_AUTHORITY_CAPACITY], epochs: [0; MESH3D_AUTHORITY_CAPACITY], reserved_pages: 0 }
    }

    pub(super) fn begin(&mut self, generation: u64, revision: u64, schema: Mesh3dSchema,grant:RetainedCloneGrant) -> Result<(Mesh3dWriteToken,RetainedCloneProgress), Mesh3dFault> {
        let layout = schema.layout()?;
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<mesh3d_begin_capacity_byte_demand(){return Err(Mesh3dFault::ByteCapacity);}
        let pages = usize::from(layout.page_count);
        if self.reserved_pages.checked_add(pages).is_none_or(|pages| pages > MESH3D_AUTHORITY_PAGE_CAPACITY) {
            return Err(Mesh3dFault::PageCapacity);
        }
        let slot = self.slots.iter().position(Option::is_none).ok_or(Mesh3dFault::ItemCapacity)?;
        let epoch = self.epochs[slot].wrapping_add(1).max(1);
        self.epochs[slot] = epoch;
        self.slots[slot] = Some(Mesh3dSlot { epoch, reserved_pages: layout.page_count, state: Mesh3dSlotState::Writing(Mesh3dOwner::new(generation, revision, schema, layout)) });
        self.reserved_pages += pages;
        Ok((Mesh3dWriteToken { slot: slot as u16, epoch, generation, revision },RetainedCloneProgress{copied_items:1,retained_capacity_bytes:mesh3d_begin_capacity_byte_demand(),..Default::default()}))
    }

    pub(super) fn writing(&mut self, token: Mesh3dWriteToken) -> Result<&mut Mesh3dOwner, Mesh3dFault> {
        let slot = self.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == token.epoch).ok_or(Mesh3dFault::Stale)?;
        match &mut slot.state {
            Mesh3dSlotState::Writing(owner) if owner.generation == token.generation && owner.revision == token.revision => Ok(owner),
            Mesh3dSlotState::Closing(_) => Err(Mesh3dFault::Closing),
            _ => Err(Mesh3dFault::Stale),
        }
    }

    pub(super) fn writing_ref(&self, token: Mesh3dWriteToken) -> Result<&Mesh3dOwner, Mesh3dFault> {
        let slot = self.slots.get(usize::from(token.slot)).and_then(Option::as_ref).filter(|slot| slot.epoch == token.epoch).ok_or(Mesh3dFault::Stale)?;
        match &slot.state {
            Mesh3dSlotState::Writing(owner) if owner.generation == token.generation && owner.revision == token.revision => Ok(owner),
            Mesh3dSlotState::Closing(_) => Err(Mesh3dFault::Closing),
            _ => Err(Mesh3dFault::Stale),
        }
    }

    pub(super) fn ready(&self, lease: Mesh3dLease) -> Result<&Mesh3dOwner, Mesh3dFault> {
        let slot = self.slots.get(usize::from(lease.slot)).and_then(Option::as_ref).filter(|slot| slot.epoch == lease.epoch).ok_or(Mesh3dFault::Stale)?;
        match &slot.state {
            Mesh3dSlotState::Ready(owner) if owner.generation == lease.generation && owner.revision == lease.revision => Ok(owner),
            Mesh3dSlotState::Closing(_) => Err(Mesh3dFault::Closing),
            _ => Err(Mesh3dFault::Stale),
        }
    }

    pub(super) fn ready_mut(&mut self, lease: Mesh3dLease) -> Result<&mut Mesh3dOwner, Mesh3dFault> {
        let slot = self.slots.get_mut(usize::from(lease.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == lease.epoch).ok_or(Mesh3dFault::Stale)?;
        match &mut slot.state {
            Mesh3dSlotState::Ready(owner) if owner.generation == lease.generation && owner.revision == lease.revision => Ok(owner),
            Mesh3dSlotState::Closing(_) => Err(Mesh3dFault::Closing),
            _ => Err(Mesh3dFault::Stale),
        }
    }

    fn reserve_component_credit(&mut self, token: Mesh3dWriteToken, credit: usize) -> Result<(), Mesh3dFault> {
        let geometry_pages = usize::from(self.writing_ref(token)?.layout.page_count);
        let bytes = geometry_pages.checked_mul(MESH3D_PAGE_BYTES).and_then(|bytes| bytes.checked_add(credit)).ok_or(Mesh3dFault::ByteCapacity)?;
        if bytes > MESH3D_OWNER_BYTE_CAPACITY { return Err(Mesh3dFault::ByteCapacity); }
        let pages = bytes.div_ceil(MESH3D_PAGE_BYTES);
        let previous = usize::from(self.slots[usize::from(token.slot)].as_ref().ok_or(Mesh3dFault::Stale)?.reserved_pages);
        let added = pages.saturating_sub(previous);
        if self.reserved_pages.checked_add(added).is_none_or(|pages| pages > MESH3D_AUTHORITY_PAGE_CAPACITY) { return Err(Mesh3dFault::PageCapacity); }
        self.reserved_pages += added;
        self.slots[usize::from(token.slot)].as_mut().ok_or(Mesh3dFault::Stale)?.reserved_pages = pages as u16;
        self.writing(token)?.component_credit = credit;
        Ok(())
    }

    pub(super) fn move_component_references(&mut self, token: Mesh3dWriteToken, original: &mut ComponentReferenceTable,grant:RetainedCloneGrant) -> Result<RetainedCloneProgress, Mesh3dFault> {
        let owner = self.writing(token)?;
        if owner.component_references.is_some() { return Err(Mesh3dFault::Order); }
        let lengths=component_lengths(original)?;let birth=mesh3d_component_capacity_byte_demand(original)?;
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<birth{return Err(Mesh3dFault::ByteCapacity);}
        owner.component_seen=NumericSeen::new(*lengths.iter().max().unwrap());owner.component_ineligible=IneligibleGroups::new(lengths);
        owner.component_references = Some(std::mem::take(original));
        owner.component_complete = false;
        let result = owner.component_header_credit().and_then(|credit| self.reserve_component_credit(token, credit));
        if let Err(fault) = result { self.writing(token)?.component_fault = Some(fault); }
        result.map(|()|RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth,..Default::default()})
    }

    pub(super) fn component_admission_step(&mut self, token: Mesh3dWriteToken,grant:RetainedCloneGrant) -> Result<Mesh3dComponentAdmissionStep, Mesh3dFault> {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_copy_bytes;
        let owner = self.writing_ref(token)?;
        if let Some(fault) = owner.component_fault { return Err(fault); }
        if owner.component_complete || maximum_items == 0 || maximum_bytes == 0||grant.maximum_depth==0 { return Ok(Mesh3dComponentAdmissionStep { complete: owner.component_complete, ..Default::default() }); }
        let bytes = owner.component_turn_byte_demand();
        if maximum_bytes < bytes { return Ok(Mesh3dComponentAdmissionStep::default()); }
        let (credit, number) = owner.component_turn_credit()?;
        if let Err(fault) = self.reserve_component_credit(token, credit) { self.writing(token)?.component_fault = Some(fault); return Err(fault); }
        let mut step=self.writing(token)?.component_turn(number);step.ownership=RetainedCloneProgress{copied_items:step.processed_items+step.retired_items,copied_bytes:step.processed_bytes+step.retired_bytes,..Default::default()};Ok(step)
    }

    pub(super) fn seal(&mut self, token: Mesh3dWriteToken) -> Result<Mesh3dLease, Mesh3dFault> {
        let slot = self.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == token.epoch).ok_or(Mesh3dFault::Stale)?;
        let Mesh3dSlotState::Writing(owner) = &slot.state else { return Err(Mesh3dFault::Stale) };
        if let Some(fault) = owner.component_fault { return Err(fault); }
        if !owner.terminal_is_complete() {
            return Err(Mesh3dFault::Incomplete);
        }
        if owner.schema.indices == 0 && owner.schema.edges == 0 && !owner.has_topological_vertex { return Err(Mesh3dFault::Schema); }
        let owner = match std::mem::replace(&mut slot.state, Mesh3dSlotState::Transition) {
            Mesh3dSlotState::Writing(owner) => owner,
            other => {
                slot.state = other;
                return Err(Mesh3dFault::Stale);
            }
        };
        slot.state = Mesh3dSlotState::Ready(owner);
        Ok(Mesh3dLease { slot: token.slot, epoch: token.epoch, generation: token.generation, revision: token.revision })
    }

    pub(super) fn begin_close_write(&mut self, token: Mesh3dWriteToken) -> Result<(), Mesh3dFault> {
        let slot = self.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == token.epoch).ok_or(Mesh3dFault::Stale)?;
        if matches!(slot.state, Mesh3dSlotState::Closing(_)) {
            return Ok(());
        }
        let mut owner = match std::mem::replace(&mut slot.state, Mesh3dSlotState::Transition) {
            Mesh3dSlotState::Writing(owner) => owner,
            other => {
                slot.state = other;
                return Err(Mesh3dFault::Stale);
            }
        };
        owner.closing = true;
        slot.state = Mesh3dSlotState::Closing(owner);
        Ok(())
    }

    pub(super) fn begin_close(&mut self, lease: Mesh3dLease) -> Result<(), Mesh3dFault> {
        if self.ready(lease).is_ok_and(|owner| owner.resource_claimed || owner.resource_loans != 0) { return Err(Mesh3dFault::Order); }
        let slot = self.slots.get_mut(usize::from(lease.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == lease.epoch).ok_or(Mesh3dFault::Stale)?;
        if matches!(slot.state, Mesh3dSlotState::Closing(_)) {
            return Ok(());
        }
        let mut owner = match std::mem::replace(&mut slot.state, Mesh3dSlotState::Transition) {
            Mesh3dSlotState::Ready(owner) => owner,
            other => {
                slot.state = other;
                return Err(Mesh3dFault::Stale);
            }
        };
        owner.closing = true;
        slot.state = Mesh3dSlotState::Closing(owner);
        Ok(())
    }

    pub(super) fn close_demands(&self,slot_index:u16,epoch:u64,work:usize)->Result<RetirementDemand,Mesh3dFault> {
        let slot=self.slots.get(usize::from(slot_index)).and_then(Option::as_ref).filter(|slot|slot.epoch==epoch).ok_or(Mesh3dFault::Stale)?;
        let Mesh3dSlotState::Closing(owner)=&slot.state else{return Err(Mesh3dFault::Closing)};owner.close_demands(work)
    }

    pub(super) fn begin_close_resource(&mut self, lease: Mesh3dLease) -> Result<(), Mesh3dFault> {
        let owner = self.ready_mut(lease)?;
        if !owner.resource_claimed || !owner.resource_closing || owner.resource_loans != 0 { return Err(Mesh3dFault::Order); }
        owner.resource_claimed = false;
        self.begin_close(lease)
    }

    pub(super) fn close_step(&mut self, slot_index: u16, epoch: u64,grant:RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
        let index = usize::from(slot_index);
        let slot = self.slots.get_mut(index).and_then(Option::as_mut).filter(|slot| slot.epoch == epoch).ok_or(Mesh3dFault::Stale)?;
        let Mesh3dSlotState::Closing(owner) = &mut slot.state else { return Err(Mesh3dFault::Closing) };
        let step=owner.close_step(grant)?;
        if !step.complete {
            return Ok(step);
        }
        let pages = usize::from(slot.reserved_pages);
        let _ = owner;
        self.slots[index] = None;
        self.reserved_pages = self.reserved_pages.saturating_sub(pages);
        Ok(step)
    }
}

pub(super) fn mesh3d_authority() -> &'static semio_framework_value::retirement::controlled::RetainedOwnerGate<Mesh3dAuthority> {
    static AUTHORITY:semio_framework_value::retirement::controlled::RetainedOwnerGate<Mesh3dAuthority>=semio_framework_value::retirement::controlled::RetainedOwnerGate::new(Mesh3dAuthority::new());
    &AUTHORITY
}

pub const fn mesh3d_begin_capacity_byte_demand()->usize {size_of::<[Option<Mesh3dPage>;MESH3D_OWNER_PAGE_CAPACITY]>()}

pub fn mesh3d_begin(generation: u64, revision: u64, schema: Mesh3dSchema,grant:RetainedCloneGrant) -> Result<(Mesh3dWriteToken,RetainedCloneProgress), Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.begin(generation, revision, schema,grant)
}

pub fn mesh3d_allocate_capacity_byte_demand(token:Mesh3dWriteToken)->Result<usize,Mesh3dFault> {
    let authority=mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)?;let owner=authority.writing_ref(token)?;Ok(if owner.allocated_pages==owner.layout.page_count{0}else{MESH3D_PAGE_BYTES})
}

pub fn mesh3d_allocate_step(token: Mesh3dWriteToken,grant:RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
    Ok(mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.writing(token)?.allocate_step(grant))
}

pub fn mesh3d_write_vec3(token: Mesh3dWriteToken, field: Mesh3dField, value: [f32; 3]) -> Result<(), Mesh3dFault> {
    if !matches!(field, Mesh3dField::Positions | Mesh3dField::Normals) || !value.iter().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut bytes = [0; 12];
    for (index, value) in value.into_iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing(token)?;
    let item = owner.write(field, &bytes)?;
    if field == Mesh3dField::Positions {
        for (axis, coordinate) in value.into_iter().enumerate() {
            owner.aabb_min[axis] = owner.aabb_min[axis].min(coordinate);
            owner.aabb_max[axis] = owner.aabb_max[axis].max(coordinate);
        }
    }
    let _ = item;
    Ok(())
}

pub fn mesh3d_write_edge(token: Mesh3dWriteToken, value: [[f32; 3]; 2]) -> Result<(), Mesh3dFault> {
    if !value.iter().flatten().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut bytes = [0; 24];
    for (index, value) in value.into_iter().flatten().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing(token)?;
    owner.write(Mesh3dField::Edges, &bytes)?;
    for point in value {
        for (axis, coordinate) in point.into_iter().enumerate() {
            owner.aabb_min[axis] = owner.aabb_min[axis].min(coordinate);
            owner.aabb_max[axis] = owner.aabb_max[axis].max(coordinate);
        }
    }
    Ok(())
}

pub fn mesh3d_write_vec2(token: Mesh3dWriteToken, field: Mesh3dField, value: [f32; 2]) -> Result<(), Mesh3dFault> {
    if !matches!(field,Mesh3dField::Uvs | Mesh3dField::UvsMetallicRoughness | Mesh3dField::UvsNormal | Mesh3dField::UvsOcclusion | Mesh3dField::UvsEmissive) || !value.iter().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut bytes = [0; 8];
    for (index, value) in value.into_iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.writing(token)?.write(field, &bytes)?;
    Ok(())
}

pub fn mesh3d_write_vec4(token: Mesh3dWriteToken, field: Mesh3dField, value: [f32; 4]) -> Result<(), Mesh3dFault> {
    if !matches!(field,Mesh3dField::Colors | Mesh3dField::Tangents) || !value.iter().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut bytes = [0; 16];
    for (index, value) in value.into_iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.writing(token)?.write(field, &bytes)?;
    Ok(())
}

pub fn mesh3d_write_u32(token: Mesh3dWriteToken, field: Mesh3dField, value: u32) -> Result<(), Mesh3dFault> {
    if !matches!(field, Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds) {
        return Err(Mesh3dFault::Schema);
    }
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.writing(token)?.write(field, &value.to_le_bytes())?;
    Ok(())
}

pub fn mesh3d_read_write_vec3(token: Mesh3dWriteToken, field: Mesh3dField, item: u32) -> Result<[f32; 3], Mesh3dFault> {
    if !matches!(field, Mesh3dField::Positions | Mesh3dField::Normals) {
        return Err(Mesh3dFault::Schema);
    }
    let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing_ref(token)?;
    if item >= owner.written[field.index()] {
        return Err(Mesh3dFault::Incomplete);
    }
    let bytes = owner.item_bytes::<12>(field, item)?;
    Ok([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..8].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[8..].try_into().expect("fixed mesh scalar"))])
}

pub fn mesh3d_read_write_u32(token: Mesh3dWriteToken, field: Mesh3dField, item: u32) -> Result<u32, Mesh3dFault> {
    if !matches!(field, Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds) {
        return Err(Mesh3dFault::Schema);
    }
    let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing_ref(token)?;
    if item >= owner.written[field.index()] {
        return Err(Mesh3dFault::Incomplete);
    }
    Ok(u32::from_le_bytes(owner.item_bytes::<4>(field, item)?))
}

pub fn mesh3d_update_vec3(token: Mesh3dWriteToken, field: Mesh3dField, item: u32, value: [f32; 3]) -> Result<(), Mesh3dFault> {
    if !matches!(field, Mesh3dField::Positions | Mesh3dField::Normals) || !value.iter().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing(token)?;
    if item >= owner.written[field.index()] {
        return Err(Mesh3dFault::Incomplete);
    }
    let absolute = owner.layout.offsets[field.index()].checked_add(item as usize * 12).ok_or(Mesh3dFault::ByteCapacity)?;
    let mut bytes = [0; 12];
    for (index, value) in value.into_iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    owner.write_at(absolute, &bytes)
}

pub fn mesh3d_seal(token: Mesh3dWriteToken) -> Result<Mesh3dLease, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.seal(token)
}

/// 🎯️ Moves the original label table into the same mesh owner before publication.
pub fn mesh3d_component_capacity_byte_demand(original:&ComponentReferenceTable)->Result<usize,Mesh3dFault> {
    let lengths=component_lengths(original)?;NumericSeen::capacity_bytes(*lengths.iter().max().unwrap()).and_then(|bytes|bytes.checked_add(lengths.iter().sum())).ok_or(Mesh3dFault::ByteCapacity)
}

pub fn mesh3d_move_component_references(token: Mesh3dWriteToken, original: &mut ComponentReferenceTable,grant:RetainedCloneGrant) -> Result<RetainedCloneProgress, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.move_component_references(token, original,grant)
}

/// 🧮️ Admits one original typed label or retires one scratch entry under fixed credits.
pub fn mesh3d_component_admission_step(token: Mesh3dWriteToken,grant:RetainedCloneGrant) -> Result<Mesh3dComponentAdmissionStep, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.component_admission_step(token,grant)
}

pub fn mesh3d_abort(token: Mesh3dWriteToken) -> Result<(), Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.begin_close_write(token)
}

pub fn mesh3d_abort_demands(token:Mesh3dWriteToken,work:usize)->Result<RetirementDemand,Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)?.close_demands(token.slot,token.epoch,work)
}

pub fn mesh3d_abort_step(token: Mesh3dWriteToken,grant:RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.close_step(token.slot, token.epoch,grant)
}

pub fn mesh3d_begin_close(lease: Mesh3dLease) -> Result<(), Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.begin_close(lease)
}

pub fn mesh3d_close_demands(lease:Mesh3dLease,work:usize)->Result<RetirementDemand,Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)?.close_demands(lease.slot,lease.epoch,work)
}

pub fn mesh3d_close_step(lease: Mesh3dLease,grant:RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.close_step(lease.slot, lease.epoch,grant)
}

pub fn mesh3d_terminal_is_empty(lease: Mesh3dLease) -> bool {
    mesh3d_authority().try_lock().is_ok_and(|authority| authority.slots.get(usize::from(lease.slot)).and_then(Option::as_ref).is_none_or(|slot| slot.epoch != lease.epoch))
}
//#endregion Mesh
