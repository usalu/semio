//! 📬️ The retained wire constants, packets, lease, lossless queues, publication channels and the commit reservation.

use super::*;

pub const ENERGY_WIRE_MAGIC: [u8; 8] = *b"SMENERGY";

pub const ENERGY_WIRE_VERSION: u16 = 1;

pub const ENERGY_WIRE_QUEUE_SLOTS: usize = 4;

pub const ENERGY_WIRE_LEASE_SLOTS: usize = 64;

pub const ENERGY_WIRE_HEADER_BYTES: usize = 80;

pub const ENERGY_CHECKPOINT_BYTES: usize = 196;

/// 💸️ Charges one owner close turn to the running step's own wallet and reports whether that owner finished.
pub fn energy_spend(context: &mut StepContext<'_>, step: Result<RetainedCloneStep, ValueError>) -> Result<bool, EnergyWireRejection> {
    let (progress, done) = match step {
        Ok(RetainedCloneStep::Progress(progress)) => (progress, false),
        Ok(RetainedCloneStep::Complete(progress)) => (progress, true),
        Err(_) => return Err(EnergyWireRejection::Backing),
    };
    context.consume_retained(progress).map_err(|_| EnergyWireRejection::Backing)?;
    Ok(done)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnergyWireKind {
    Preview = 1,
    Checkpoint = 2,
    Commit = 3,
    Fault = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyWireIdentity {
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
    pub seed: u64,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergyWireRejection {
    Header,
    Version,
    Kind,
    Identity,
    Items,
    Pages,
    Operations,
    ProcessJobs,
    Saturated,
    Backing,
}

pub struct EnergyWirePacket {
    pub kind: EnergyWireKind,
    pub identity: EnergyWireIdentity,
    pub payload: RetainedJobPayload,
    pub preview: Option<EnergyJobPreview>,
    pub reservation: Option<EnergyCommitReservation>,
}

pub struct EnergyWireLease {
    pub(super) kind: EnergyWireKind,
    token: u64,
    recovery_slot: usize,
    pub(super) packet: Option<EnergyWirePacket>,
}

impl std::fmt::Debug for EnergyWireLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("EnergyWireLease").field("kind", &self.kind).field("token", &self.token).field("identity", &self.packet.as_ref().map(|packet| packet.identity)).finish()
    }
}

impl EnergyWireLease {
    pub fn identity(&self) -> EnergyWireIdentity {
        self.packet.as_ref().expect("live lease owns packet").identity
    }

    pub fn packet(&self) -> &EnergyWirePacket {
        self.packet.as_ref().expect("live lease owns packet")
    }

    pub fn packet_mut(&mut self) -> &mut EnergyWirePacket {
        self.packet.as_mut().expect("live lease owns packet")
    }
}

#[expect(clippy::large_enum_variant, reason = "The fixed recovery registry accepts abandoned packet ownership from Drop without allocating.")]
pub enum EnergyWireLeaseRecoverySlot {
    Vacant,
    Reserved(u64),
    Abandoned(u64, EnergyWirePacket),
}

pub static ENERGY_WIRE_LEASE_RECOVERY: Mutex<[EnergyWireLeaseRecoverySlot; ENERGY_WIRE_LEASE_SLOTS]> = Mutex::new([const { EnergyWireLeaseRecoverySlot::Vacant }; ENERGY_WIRE_LEASE_SLOTS]);

impl Drop for EnergyWireLease {
    fn drop(&mut self) {
        let Some(packet) = self.packet.take() else { return };
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        debug_assert!(matches!(registry[self.recovery_slot], EnergyWireLeaseRecoverySlot::Reserved(token) if token == self.token));
        registry[self.recovery_slot] = EnergyWireLeaseRecoverySlot::Abandoned(self.token, packet);
    }
}

impl std::fmt::Debug for EnergyWirePacket {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("EnergyWirePacket").field("kind", &self.kind).field("identity", &self.identity).field("pages", &self.payload.page_count()).field("bytes", &self.payload.len()).finish()
    }
}

impl EnergyWirePacket {
    pub fn identity(&self) -> EnergyWireIdentity {
        self.identity
    }

    pub fn preview(&self) -> Option<&EnergyJobPreview> {
        self.preview.as_ref()
    }

    /// 📏️ Quotes the next original payload page or ledger this packet would release.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        self.payload.retirement_demands()
    }

    /// 🎟️ Releases one payload frontier only when the caller's grant covers its quote.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.payload.retirement_demands()?;
        if !energy_grant_covers(grant, demand) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.payload.close_step(grant)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.payload.terminal_is_empty()
    }
}

pub struct EnergyWireQueue {
    pub(super) kind: EnergyWireKind,
    slots: [Option<EnergyWirePacket>; ENERGY_WIRE_QUEUE_SLOTS],
    head: usize,
    pub len: usize,
    next_token: u64,
    pub in_flight: Option<EnergyWireInFlight>,
    pub reserved_push: bool,
}

#[derive(Clone, Copy)]
pub struct EnergyWireInFlight {
    token: u64,
    recovery_slot: usize,
    pub(super) generation: u64,
    pub(super) pages: usize,
    bytes: usize,
}

impl EnergyWireQueue {
    pub(super) fn new(kind: EnergyWireKind) -> Self {
        Self { kind, slots: std::array::from_fn(|_| None), head: 0, len: 0, next_token: 0, in_flight: None, reserved_push: false }
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn push(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
        if packet.kind != self.kind || self.len == ENERGY_WIRE_QUEUE_SLOTS || self.reserved_push {
            return Err(packet);
        }
        let index = (self.head + self.len) % ENERGY_WIRE_QUEUE_SLOTS;
        self.slots[index] = Some(packet);
        self.len += 1;
        Ok(())
    }

    pub fn reserve_push(&mut self) -> Result<(), EnergyWireRejection> {
        if self.len == ENERGY_WIRE_QUEUE_SLOTS || self.reserved_push {
            return Err(EnergyWireRejection::Saturated);
        }
        self.reserved_push = true;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn push_reserved(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
        if packet.kind != self.kind || !self.reserved_push || self.len == ENERGY_WIRE_QUEUE_SLOTS {
            return Err(packet);
        }
        self.reserved_push = false;
        let index = (self.head + self.len) % ENERGY_WIRE_QUEUE_SLOTS;
        self.slots[index] = Some(packet);
        self.len += 1;
        Ok(())
    }

    pub fn take(&mut self) -> Option<EnergyWireLease> {
        self.recover_lost();
        if self.len == 0 || self.in_flight.is_some() {
            return None;
        }
        let packet = self.slots[self.head].take()?;
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(recovery_slot) = registry.iter().position(|slot| matches!(slot, EnergyWireLeaseRecoverySlot::Vacant)) else {
            self.slots[self.head] = Some(packet);
            return None;
        };
        let token = self.next_token;
        self.next_token = self.next_token.checked_add(1).expect("Energy queue lease sequence exhausted");
        registry[recovery_slot] = EnergyWireLeaseRecoverySlot::Reserved(token);
        self.in_flight = Some(EnergyWireInFlight { token, recovery_slot, generation: packet.identity.generation, pages: packet.payload.page_count(), bytes: packet.payload.len() });
        Some(EnergyWireLease { kind: self.kind, token, recovery_slot, packet: Some(packet) })
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        if lease.kind != self.kind || !self.in_flight.is_some_and(|flight| flight.token == lease.token && flight.recovery_slot == lease.recovery_slot && lease.packet.as_ref().is_some_and(|packet| packet.identity.generation == flight.generation)) {
            return Err(lease);
        }
        let packet = lease.packet.take().expect("retry lease owns exact packet");
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(registry[lease.recovery_slot], EnergyWireLeaseRecoverySlot::Reserved(token) if token == lease.token) {
            lease.packet = Some(packet);
            return Err(lease);
        }
        registry[lease.recovery_slot] = EnergyWireLeaseRecoverySlot::Vacant;
        self.slots[self.head] = Some(packet);
        self.in_flight = None;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack(&mut self, mut lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        if lease.kind != self.kind
            || !self.in_flight.is_some_and(|flight| flight.token == lease.token && flight.recovery_slot == lease.recovery_slot && lease.packet.as_ref().is_some_and(|packet| packet.identity.generation == flight.generation))
            || !lease.packet().terminal_is_empty()
        {
            return Err(lease);
        }
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(registry[lease.recovery_slot], EnergyWireLeaseRecoverySlot::Reserved(token) if token == lease.token) {
            return Err(lease);
        }
        lease.packet.take();
        registry[lease.recovery_slot] = EnergyWireLeaseRecoverySlot::Vacant;
        self.in_flight = None;
        self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
        self.len -= 1;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_transfer(&mut self, mut lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
        if lease.kind != self.kind || !self.in_flight.is_some_and(|flight| flight.token == lease.token && flight.recovery_slot == lease.recovery_slot && lease.packet.as_ref().is_some_and(|packet| packet.identity.generation == flight.generation)) {
            return Err(lease);
        }
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(registry[lease.recovery_slot], EnergyWireLeaseRecoverySlot::Reserved(token) if token == lease.token) {
            return Err(lease);
        }
        let packet = lease.packet.take().expect("transfer lease owns exact packet");
        registry[lease.recovery_slot] = EnergyWireLeaseRecoverySlot::Vacant;
        self.in_flight = None;
        self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
        self.len -= 1;
        Ok(packet)
    }

    fn recover_lost(&mut self) -> bool {
        let Some(EnergyWireInFlight { token, recovery_slot, generation, .. }) = self.in_flight else { return false };
        let mut registry = ENERGY_WIRE_LEASE_RECOVERY.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(&registry[recovery_slot], EnergyWireLeaseRecoverySlot::Abandoned(candidate, packet) if *candidate == token && packet.identity.generation == generation) {
            return false;
        }
        let EnergyWireLeaseRecoverySlot::Abandoned(_, packet) = std::mem::replace(&mut registry[recovery_slot], EnergyWireLeaseRecoverySlot::Vacant) else { unreachable!("matched abandoned wire lease") };
        self.slots[self.head] = Some(packet);
        self.in_flight = None;
        true
    }

    pub fn take_terminal(&mut self) -> Option<EnergyWirePacket> {
        self.recover_lost();
        if self.len == 0 || self.in_flight.is_some() {
            return None;
        }
        let packet = self.slots[self.head].take()?;
        self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
        self.len -= 1;
        Some(packet)
    }

    pub fn retained_totals_at(&self, index: usize) -> Option<(usize, usize, usize)> {
        if index < ENERGY_WIRE_QUEUE_SLOTS {
            return self.slots[index].as_ref().map(|packet| (packet.payload.page_count(), packet.payload.len(), packet.payload.page_count().saturating_add(1)));
        }
        if index == ENERGY_WIRE_QUEUE_SLOTS {
            return self.in_flight.map(|flight| (flight.pages, flight.bytes, flight.pages.saturating_add(1)));
        }
        None
    }

    pub fn retirement_demands(&self) -> Option<Result<RetirementDemand, ValueError>> {
        if self.in_flight.is_some() {
            return Some(Ok(RetirementDemand::default()));
        }
        if self.len == 0 && self.reserved_push {
            return Some(Ok(RetirementDemand { depth: 1, ..Default::default() }));
        }
        self.slots[self.head].as_ref().map(EnergyWirePacket::retirement_demands)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<InteractiveJobCloseStep>, ValueError> {
        self.recover_lost();
        if self.in_flight.is_some() {
            return Ok(Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }));
        }
        if self.len == 0 && self.reserved_push {
            if !energy_grant_covers(grant, RetirementDemand { depth: 1, ..Default::default() }) {
                return Ok(Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }));
            }
            self.reserved_push = false;
            return Ok(Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }));
        }
        let Some(packet) = self.slots[self.head].as_mut() else { return Ok(None) };
        let (step, done) = energy_turn(packet.close_step(grant))?;
        if done {
            self.slots[self.head] = None;
            self.head = (self.head + 1) % ENERGY_WIRE_QUEUE_SLOTS;
            self.len -= 1;
        }
        Ok(Some(step))
    }
}

pub struct EnergyPublicationChannels {
    pub preview: Option<EnergyWirePacket>,
    pub retiring_preview: Option<EnergyWirePacket>,
    pub checkpoints: EnergyWireQueue,
    pub commits: EnergyWireQueue,
    pub faults: EnergyWireQueue,
}

impl Default for EnergyPublicationChannels {
    fn default() -> Self {
        Self { preview: None, retiring_preview: None, checkpoints: EnergyWireQueue::new(EnergyWireKind::Checkpoint), commits: EnergyWireQueue::new(EnergyWireKind::Commit), faults: EnergyWireQueue::new(EnergyWireKind::Fault) }
    }
}

pub struct EnergyWireBuild {
    pub kind: EnergyWireKind,
    pub identity: EnergyWireIdentity,
    pub writer: semio_framework_job::RetainedJobPayloadWriter,
    pub field: u8,
}

pub fn energy_wire_header(kind: EnergyWireKind, identity: EnergyWireIdentity, census: EnergyNumericalCensus, stage: EnergyJobStage, tier: EnergyQualityTier) -> [u8; ENERGY_WIRE_HEADER_BYTES] {
    let mut bytes = [0; ENERGY_WIRE_HEADER_BYTES];
    bytes[..8].copy_from_slice(&ENERGY_WIRE_MAGIC);
    bytes[8..10].copy_from_slice(&ENERGY_WIRE_VERSION.to_le_bytes());
    bytes[10] = kind as u8;
    bytes[11] = stage.code();
    bytes[12] = tier.code();
    bytes[16..24].copy_from_slice(&identity.operation.to_le_bytes());
    bytes[24..32].copy_from_slice(&identity.base_revision.to_le_bytes());
    bytes[32..40].copy_from_slice(&identity.generation.to_le_bytes());
    bytes[40..48].copy_from_slice(&identity.seed.to_le_bytes());
    bytes[48..56].copy_from_slice(&identity.sequence.to_le_bytes());
    bytes[56..64].copy_from_slice(&(census.observed_items as u64).to_le_bytes());
    bytes[64..72].copy_from_slice(&(census.pages as u64).to_le_bytes());
    bytes[72..76].copy_from_slice(&(census.operations as u32).to_le_bytes());
    bytes[76..80].copy_from_slice(&(census.process_jobs as u32).to_le_bytes());
    bytes
}

#[derive(Debug)]
pub struct EnergyCommitReservation {
    pub pages: usize,
    pub bytes: usize,
    pub items: usize,
    pub prepared: usize,
    pub sources: [Option<semio_framework_job::JobPayloadPageSource>; semio_framework_job::JOB_PAYLOAD_OPERATION_PAGES],
}

impl EnergyCommitReservation {
    pub fn new(pages: usize, bytes: usize, items: usize) -> Self {
        Self { pages, bytes, items, prepared: 0, sources: std::array::from_fn(|_| None) }
    }

    pub fn take_source(&mut self, index: usize) -> Option<semio_framework_job::JobPayloadPageSource> {
        self.sources.get_mut(index)?.take()
    }

    pub fn restore_source(&mut self, index: usize, source: semio_framework_job::JobPayloadPageSource) {
        self.sources[index] = Some(source);
    }

    pub fn close_source_demand(&self) -> Option<RetirementDemand> {
        self.sources.iter().flatten().next().map(|source| RetirementDemand { release_bytes: source.allocated_capacity_bytes(), depth: 1, ..Default::default() })
    }

    pub fn close_source_step(&mut self) -> Option<usize> {
        let source = self.sources.iter_mut().find(|source| source.is_some())?;
        source.take().map(|source| source.allocated_capacity_bytes())
    }
}
