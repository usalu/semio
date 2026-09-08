use std::{cell::RefCell, rc::Rc};

use semio_framework::kernel::{
    ActorInstanceLifetime, COLD_PAIR_MAXIMUM_BYTES, COLD_PAIR_MAXIMUM_PAGES, COLD_PAIR_PAGE_MAXIMUM_BYTES, ColdDocumentPairApplied, ColdDocumentPairCursor, ColdDocumentPairHeader, ColdDocumentPairPage, ColdPairIngressStatus,
};

const COLD_PAIR_FAULT_MAXIMUM_BYTES: usize = 4 * 1024;

enum ColdDocumentPairPhase {
    Receiving,
    Verified,
    Loading,
    Faulted,
    Applied,
    Closing,
}

struct ColdDocumentPairOwner {
    header: ColdDocumentPairHeader,
    files: Rc<store::ArtifactPackFiles>,
    pack_hash: semio_framework_hash::Sha256,
    spr_hash: semio_framework_hash::Sha256,
    aggregate_hash: semio_framework_hash::Sha256,
    reserved_bytes: usize,
    reserved_pages: u32,
    next_page: u32,
    phase: ColdDocumentPairPhase,
}

impl ColdDocumentPairOwner {
    fn reserve(header: ColdDocumentPairHeader) -> Result<Self, &'static str> {
        header.validate()?;
        let pack_length = usize::try_from(header.pack_length).map_err(|_| "cold-pair.length")?;
        let spr_length = usize::try_from(header.spr_length).map_err(|_| "cold-pair.length")?;
        let mut pack = Vec::new();
        let mut spr = Vec::new();
        pack.try_reserve_exact(pack_length).map_err(|_| "cold-pair.capacity")?;
        spr.try_reserve_exact(spr_length).map_err(|_| "cold-pair.capacity")?;
        let reserved_bytes = pack.capacity().checked_add(spr.capacity()).ok_or("cold-pair.capacity")?;
        let reserved_pages = header.page_count;
        Ok(Self {
            header,
            files: Rc::new(store::ArtifactPackFiles { pack, spr, ops: String::new() }),
            pack_hash: semio_framework_hash::Sha256::new(),
            spr_hash: semio_framework_hash::Sha256::new(),
            aggregate_hash: semio_framework_hash::Sha256::new(),
            reserved_bytes,
            reserved_pages,
            next_page: 0,
            phase: ColdDocumentPairPhase::Receiving,
        })
    }

    fn accept(&mut self, page: &ColdDocumentPairPage) -> Result<bool, &'static str> {
        if !matches!(self.phase, ColdDocumentPairPhase::Receiving) || page.header != self.header || page.page_index != self.next_page {
            return Err("cold-pair.cursor");
        }
        if page.bytes.len() != self.header.page_length(page.page_index)? {
            return Err("cold-pair.page-length");
        }
        let files = Rc::get_mut(&mut self.files).ok_or("cold-pair.owner-busy")?;
        let pack_remaining = usize::try_from(self.header.pack_length).map_err(|_| "cold-pair.length")?.checked_sub(files.pack.len()).ok_or("cold-pair.page-index")?;
        let pack_count = page.bytes.len().min(pack_remaining);
        let (pack, spr) = page.bytes.split_at(pack_count);
        files.pack.extend_from_slice(pack);
        files.spr.extend_from_slice(spr);
        self.pack_hash.update(pack);
        self.spr_hash.update(spr);
        self.aggregate_hash.update(&page.bytes);
        self.next_page = self.next_page.checked_add(1).ok_or("cold-pair.page-index")?;
        Ok(self.next_page == self.header.page_count)
    }

    fn finish_receiving(&mut self) -> bool {
        let verified = self.files.pack.len() as u64 == self.header.pack_length
            && self.files.spr.len() as u64 == self.header.spr_length
            && self.pack_hash.clone().finalize() == self.header.pack_sha256
            && self.spr_hash.clone().finalize() == self.header.spr_sha256
            && self.aggregate_hash.clone().finalize() == self.header.aggregate_sha256;
        self.phase = if verified { ColdDocumentPairPhase::Verified } else { ColdDocumentPairPhase::Faulted };
        verified
    }

    fn cursor(&self) -> ColdDocumentPairCursor {
        self.header.cursor(self.next_page.saturating_sub(1).min(self.header.page_count - 1))
    }

    #[cfg(test)]
    fn retained_bytes(&self) -> usize {
        self.files.pack.len() + self.files.spr.len()
    }

    fn close_step(&mut self) -> ColdDocumentPairCloseStep {
        self.phase = ColdDocumentPairPhase::Closing;
        let Some(files) = Rc::get_mut(&mut self.files) else {
            return ColdDocumentPairCloseStep { wiped_bytes: 0, closed: false };
        };
        let mut remaining = COLD_PAIR_PAGE_MAXIMUM_BYTES;
        let spr_count = remaining.min(files.spr.len());
        if spr_count != 0 {
            let start = files.spr.len() - spr_count;
            files.spr[start..].fill(0);
            files.spr.truncate(start);
            remaining -= spr_count;
        }
        let pack_count = remaining.min(files.pack.len());
        if pack_count != 0 {
            let start = files.pack.len() - pack_count;
            files.pack[start..].fill(0);
            files.pack.truncate(start);
        }
        files.ops.clear();
        ColdDocumentPairCloseStep { wiped_bytes: spr_count + pack_count, closed: files.pack.is_empty() && files.spr.is_empty() }
    }
}

pub(crate) struct ColdDocumentPairLoad {
    owner: Rc<RefCell<ColdDocumentPairOwner>>,
    files: Rc<store::ArtifactPackFiles>,
    finished: bool,
}

impl ColdDocumentPairLoad {
    #[cfg(test)]
    pub(crate) fn lifetime(&self) -> ActorInstanceLifetime {
        self.owner.borrow().header.lifetime
    }

    pub(crate) fn files(&self) -> &store::ArtifactPackFiles {
        &self.files
    }
}

impl Drop for ColdDocumentPairLoad {
    fn drop(&mut self) {
        if !self.finished {
            let mut owner = self.owner.borrow_mut();
            if matches!(owner.phase, ColdDocumentPairPhase::Loading) {
                owner.phase = ColdDocumentPairPhase::Faulted;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ColdDocumentPairCloseStep {
    pub(crate) wiped_bytes: usize,
    pub(crate) closed: bool,
}

pub(crate) struct ColdDocumentPairIngressRegistry<const N: usize> {
    slots: [Option<Rc<RefCell<ColdDocumentPairOwner>>>; N],
    closes: [Option<ColdDocumentPairCloseState>; N],
    reserved_bytes: usize,
    reserved_pages: u32,
    close_cursor: usize,
}

#[derive(Clone, Copy)]
struct ColdDocumentPairCloseState {
    key: super::instance_lifetime::NativeCloseKey,
    active: bool,
    complete: bool,
}

impl<const N: usize> ColdDocumentPairIngressRegistry<N> {
    pub(crate) fn new() -> Self {
        assert!(N > 0, "cold pair ingress requires fixed capacity");
        Self { slots: std::array::from_fn(|_| None), closes: [None; N], reserved_bytes: 0, reserved_pages: 0, close_cursor: 0 }
    }

    fn slot_index(lifetime: ActorInstanceLifetime) -> usize {
        lifetime.instance_id as usize % N
    }

    fn fault(cursor: ColdDocumentPairCursor, code: &'static str) -> ColdPairIngressStatus {
        ColdPairIngressStatus::Fault { cursor, fault: code.as_bytes()[..code.len().min(COLD_PAIR_FAULT_MAXIMUM_BYTES)].to_vec() }
    }

    pub(crate) fn accept_page(&mut self, page: ColdDocumentPairPage, live: Option<ActorInstanceLifetime>) -> ColdPairIngressStatus {
        let cursor = page.header.cursor(page.page_index);
        if let Err(code) = page.header.validate() {
            return Self::fault(cursor, code);
        }
        if live != Some(page.header.lifetime) {
            return Self::fault(cursor, "cold-pair.not-live");
        }
        if page.page_index >= page.header.page_count {
            return Self::fault(cursor, "cold-pair.page-index");
        }
        if page.bytes.len() != page.header.page_length(page.page_index).unwrap_or(0) {
            return Self::fault(cursor, "cold-pair.page-length");
        }
        let index = Self::slot_index(page.header.lifetime);
        if self.closes[index].is_some() {
            return Self::fault(cursor, "cold-pair.not-live");
        }
        let owner = if let Some(owner) = &self.slots[index] {
            if owner.borrow().header.lifetime != page.header.lifetime {
                return Self::fault(cursor, "cold-pair.slot-collision");
            }
            Rc::clone(owner)
        } else {
            if page.page_index != 0 {
                return Self::fault(cursor, "cold-pair.cursor");
            }
            let declared_bytes = page.header.total_length().unwrap_or(COLD_PAIR_MAXIMUM_BYTES.saturating_add(1));
            if self.reserved_bytes.checked_add(declared_bytes).is_none_or(|reserved| reserved > COLD_PAIR_MAXIMUM_BYTES) || self.reserved_pages.checked_add(page.header.page_count).is_none_or(|reserved| reserved > COLD_PAIR_MAXIMUM_PAGES) {
                return Self::fault(cursor, "cold-pair.capacity");
            }
            let owner = match ColdDocumentPairOwner::reserve(page.header.clone()) {
                Ok(owner) => Rc::new(RefCell::new(owner)),
                Err(code) => return Self::fault(cursor, code),
            };
            let (reserved_bytes, reserved_pages) = {
                let owner = owner.borrow();
                (owner.reserved_bytes, owner.reserved_pages)
            };
            let Some(next_reserved_bytes) = self.reserved_bytes.checked_add(reserved_bytes) else {
                return Self::fault(cursor, "cold-pair.capacity");
            };
            let Some(next_reserved_pages) = self.reserved_pages.checked_add(reserved_pages) else {
                return Self::fault(cursor, "cold-pair.capacity");
            };
            if next_reserved_bytes > COLD_PAIR_MAXIMUM_BYTES || next_reserved_pages > COLD_PAIR_MAXIMUM_PAGES {
                return Self::fault(cursor, "cold-pair.capacity");
            }
            self.reserved_bytes = next_reserved_bytes;
            self.reserved_pages = next_reserved_pages;
            self.slots[index] = Some(Rc::clone(&owner));
            owner
        };
        let mut owner = owner.borrow_mut();
        if !matches!(owner.phase, ColdDocumentPairPhase::Receiving) {
            return ColdPairIngressStatus::Backpressure(owner.cursor());
        }
        let terminal = match owner.accept(&page) {
            Ok(terminal) => terminal,
            Err(code) => return Self::fault(cursor, code),
        };
        if !terminal {
            return ColdPairIngressStatus::PageAccepted(cursor);
        }
        if !owner.finish_receiving() {
            return Self::fault(cursor, "cold-pair.hash");
        }
        ColdPairIngressStatus::Loading(cursor)
    }

    pub(crate) fn begin_load(&mut self, lifetime: ActorInstanceLifetime, transfer_generation: u64, live: Option<ActorInstanceLifetime>) -> Option<ColdDocumentPairLoad> {
        if live != Some(lifetime) {
            return None;
        }
        if self.closes[Self::slot_index(lifetime)].is_some() {
            return None;
        }
        let owner = Rc::clone(self.slots[Self::slot_index(lifetime)].as_ref()?);
        let files = {
            let mut state = owner.borrow_mut();
            if state.header.lifetime != lifetime || state.header.transfer_generation != transfer_generation || !matches!(state.phase, ColdDocumentPairPhase::Verified) {
                return None;
            }
            state.phase = ColdDocumentPairPhase::Loading;
            Rc::clone(&state.files)
        };
        Some(ColdDocumentPairLoad { owner, files, finished: false })
    }

    pub(crate) fn finish_load(&mut self, mut load: ColdDocumentPairLoad, result: Result<(), Vec<u8>>, live: Option<ActorInstanceLifetime>) -> ColdPairIngressStatus {
        let header = load.owner.borrow().header.clone();
        let cursor = header.cursor(header.page_count - 1);
        let index = Self::slot_index(header.lifetime);
        if self.slots[index].as_ref().is_none_or(|owner| !Rc::ptr_eq(owner, &load.owner)) || !matches!(load.owner.borrow().phase, ColdDocumentPairPhase::Loading) {
            load.finished = true;
            return Self::fault(cursor, "cold-pair.stale-load");
        }
        if live != Some(header.lifetime) {
            load.owner.borrow_mut().phase = ColdDocumentPairPhase::Closing;
            load.finished = true;
            return Self::fault(cursor, "cold-pair.not-live");
        }
        let status = match result {
            Ok(()) => {
                let receipt = ColdDocumentPairApplied { lifetime: header.lifetime, transfer_generation: header.transfer_generation, baseline_frontier: header.baseline_frontier.clone(), aggregate_sha256: header.aggregate_sha256 };
                load.owner.borrow_mut().phase = ColdDocumentPairPhase::Applied;
                ColdPairIngressStatus::Applied(receipt)
            }
            Err(mut fault) => {
                fault.truncate(COLD_PAIR_FAULT_MAXIMUM_BYTES);
                load.owner.borrow_mut().phase = ColdDocumentPairPhase::Faulted;
                ColdPairIngressStatus::Fault { cursor, fault }
            }
        };
        load.finished = true;
        status
    }

    #[cfg(test)]
    pub(crate) fn request_close(&mut self, lifetime: ActorInstanceLifetime) -> bool {
        let Some(owner) = self.slots[Self::slot_index(lifetime)].as_ref() else {
            return false;
        };
        if owner.borrow().header.lifetime != lifetime {
            return false;
        }
        owner.borrow_mut().phase = ColdDocumentPairPhase::Closing;
        true
    }

    pub(crate) fn preflight_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        let index = Self::slot_index(key.lifetime());
        if self.slots[index].as_ref().is_some_and(|owner| owner.borrow().header.lifetime != key.lifetime()) {
            return Err("cold pair slot belongs to another lifetime");
        }
        if self.closes[index].is_some_and(|close| close.key != key) {
            return Err("cold pair close slot belongs to another allocation");
        }
        Ok(())
    }

    pub(crate) fn reserve_close_instance(&mut self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        self.preflight_close_instance(key)?;
        let index = Self::slot_index(key.lifetime());
        if self.closes[index].is_none() {
            self.closes[index] = Some(ColdDocumentPairCloseState { key, active: false, complete: false });
        }
        Ok(())
    }

    pub(crate) fn activate_close_instance(&mut self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        let index = Self::slot_index(key.lifetime());
        let close = self.closes[index].as_mut().filter(|close| close.key == key).ok_or("exact cold pair close reservation missing")?;
        close.active = true;
        if let Some(owner) = &self.slots[index] {
            if owner.borrow().header.lifetime != key.lifetime() {
                return Err("cold pair close lifetime changed");
            }
            owner.borrow_mut().phase = ColdDocumentPairPhase::Closing;
        }
        Ok(())
    }

    pub(crate) fn advance_close_one(&mut self) -> bool {
        let Some(index) = (0..N).map(|offset| (self.close_cursor + offset) % N).find(|index| self.closes[*index].is_some_and(|close| close.active && !close.complete)) else {
            return false;
        };
        self.close_cursor = (index + 1) % N;
        let key = self.closes[index].expect("selected cold pair close owner").key;
        let complete = self.close_step(key.lifetime()).closed;
        self.closes[index].as_mut().expect("retained cold pair close owner").complete = complete;
        true
    }

    pub(crate) fn close_instance_complete(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<bool, &'static str> {
        self.closes[Self::slot_index(key.lifetime())].filter(|close| close.key == key).map(|close| close.complete).ok_or("exact cold pair close receipt missing")
    }

    pub(crate) fn release_close_instance(&mut self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
        let index = Self::slot_index(key.lifetime());
        if !self.closes[index].is_some_and(|close| close.key == key && close.complete) {
            return Err("cold pair close receipt is not terminal");
        }
        self.closes[index] = None;
        Ok(())
    }

    pub(crate) fn close_step(&mut self, lifetime: ActorInstanceLifetime) -> ColdDocumentPairCloseStep {
        let index = Self::slot_index(lifetime);
        let Some(owner) = self.slots[index].as_ref().cloned() else {
            return ColdDocumentPairCloseStep { wiped_bytes: 0, closed: true };
        };
        if owner.borrow().header.lifetime != lifetime {
            return ColdDocumentPairCloseStep { wiped_bytes: 0, closed: false };
        }
        let step = owner.borrow_mut().close_step();
        if step.closed {
            let (reserved_bytes, reserved_pages) = {
                let owner = owner.borrow();
                (owner.reserved_bytes, owner.reserved_pages)
            };
            self.reserved_bytes = self.reserved_bytes.checked_sub(reserved_bytes).expect("cold pair byte capacity owner");
            self.reserved_pages = self.reserved_pages.checked_sub(reserved_pages).expect("cold pair page capacity owner");
            self.slots[index] = None;
        }
        step
    }

    #[cfg(test)]
    fn retained_bytes(&self, lifetime: ActorInstanceLifetime) -> usize {
        self.slots[Self::slot_index(lifetime)].as_ref().filter(|owner| owner.borrow().header.lifetime == lifetime).map_or(0, |owner| owner.borrow().retained_bytes())
    }

    #[cfg(test)]
    fn is_mounted(&self, lifetime: ActorInstanceLifetime) -> bool {
        self.slots[Self::slot_index(lifetime)].as_ref().is_some_and(|owner| owner.borrow().header.lifetime == lifetime)
    }

    #[cfg(test)]
    fn is_applied(&self, lifetime: ActorInstanceLifetime) -> bool {
        self.slots[Self::slot_index(lifetime)].as_ref().is_some_and(|owner| {
            let owner = owner.borrow();
            owner.header.lifetime == lifetime && matches!(owner.phase, ColdDocumentPairPhase::Applied)
        })
    }
}

impl<const N: usize> Drop for ColdDocumentPairIngressRegistry<N> {
    fn drop(&mut self) {
        assert!(self.slots.iter().all(Option::is_none) && self.closes.iter().all(Option::is_none) && self.reserved_bytes == 0 && self.reserved_pages == 0, "cold pair ingress requires bounded terminal close before teardown");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
