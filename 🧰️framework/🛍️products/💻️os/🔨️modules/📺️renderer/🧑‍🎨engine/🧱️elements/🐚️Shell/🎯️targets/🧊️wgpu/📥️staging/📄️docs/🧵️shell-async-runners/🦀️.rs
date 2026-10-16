//! 📄️ `shell-docs` staging of `shell-async-runners`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
#[cfg(not(target_arch = "wasm32"))]
pub fn terminal_directory_home_ack(result: &semio_framework::kernel::InvocationResult, bootstrap_epoch: u64) -> Result<DirectoryEventPageAckV1, String> {
    const RECEIPT_SCHEMA: &str = "semio.space.home.directory-projection-receipt.v1";
    let mut acknowledgement = None;
    for effect in &result.requested_effects {
        let semio_framework::kernel::Effect::PublishEvent { topic, payload } = effect else {
            return Err("retained Home directory publication emitted a non-receipt effect".into());
        };
        if topic != RECEIPT_SCHEMA || acknowledgement.is_some() {
            return Err("retained Home directory publication emitted an unexpected receipt".into());
        }
        let value = store::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        let semio_framework_value::DslValue::Object(fields) = &value else {
            return Err("retained Home directory receipt is not an object".into());
        };
        let expected_keys = ["schema", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive", "receiptSha256"];
        if fields.len() != expected_keys.len() || !fields.iter().all(|(key, _)| expected_keys.contains(&key.as_str())) {
            return Err("retained Home directory receipt has an inexact field set".into());
        }
        let field = |key: &str| value.get(key).ok_or_else(|| format!("retained Home directory receipt lacks {key}"));
        if field("schema")?.as_str() != Some(RECEIPT_SCHEMA) {
            return Err("retained Home directory receipt schema mismatch".into());
        }
        acknowledgement = Some(DirectoryEventPageAckV1 {
            bootstrap_epoch,
            session_binding_sha256: field("sessionBindingSha256")?.as_str().ok_or("retained Home directory receipt binding is not text")?.to_string(),
            authorization_generation: field("authorizationGeneration")?.as_u64().ok_or("retained Home directory receipt generation is not an unsigned integer")?,
            through_seq_inclusive: field("throughSeqInclusive")?.as_u64().ok_or("retained Home directory receipt frontier is not an unsigned integer")?,
            receipt_sha256: field("receiptSha256")?.as_str().ok_or("retained Home directory page receipt is not text")?.to_string(),
        });
    }
    if !result.mutations.is_empty() {
        return Err("retained Home directory publication escaped its Config-only lane".into());
    }
    acknowledgement.ok_or_else(|| "retained Home directory publication returned no terminal receipt".into())
}

//#region 📄️ShellDocumentRetirement
pub const SHELL_DOCUMENT_RETIREMENT_CAPACITY: usize = UI_DOCUMENT_LEASE_SLOTS * UI_DOCUMENT_LEASE_ALIASES as usize;

pub const SHELL_DOCUMENT_RETIREMENT_WORD_BITS: usize = u64::BITS as usize;

pub const SHELL_DOCUMENT_RETIREMENT_WORDS: usize = (SHELL_DOCUMENT_RETIREMENT_CAPACITY + SHELL_DOCUMENT_RETIREMENT_WORD_BITS - 1) / SHELL_DOCUMENT_RETIREMENT_WORD_BITS;

pub struct ShellDocumentRetirementSlot {
    pub epoch: u64,
    pub generation: u64,
    pub surface: Option<SurfaceId>,
    pub document: UiDocumentLease,
}

pub struct ShellDocumentRetirementRegistry {
    pub slots: Box<[Option<ShellDocumentRetirementSlot>; SHELL_DOCUMENT_RETIREMENT_CAPACITY]>,
    pub epochs: [u64; SHELL_DOCUMENT_RETIREMENT_CAPACITY],
    pub occupied: [u64; SHELL_DOCUMENT_RETIREMENT_WORDS],
    pub occupied_count: u64,
    pub cursor: usize,
}

impl Default for ShellDocumentRetirementRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; SHELL_DOCUMENT_RETIREMENT_CAPACITY], occupied: [0; SHELL_DOCUMENT_RETIREMENT_WORDS], occupied_count: 0, cursor: 0 }
    }
}

impl ShellDocumentRetirementRegistry {
    pub fn first_vacant_index(&self) -> Option<usize> {
        self.occupied.iter().enumerate().find_map(|(word_index, occupied)| {
            let first_index = word_index * SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
            let remaining = SHELL_DOCUMENT_RETIREMENT_CAPACITY.saturating_sub(first_index);
            let valid = if remaining >= SHELL_DOCUMENT_RETIREMENT_WORD_BITS { u64::MAX } else { (1_u64 << remaining) - 1 };
            let vacant = !occupied & valid;
            (vacant != 0).then_some(first_index + vacant.trailing_zeros() as usize)
        })
    }

    pub fn next_occupied_index(&self) -> Option<usize> {
        if self.occupied_count == 0 {
            return None;
        }
        let start_word = self.cursor / SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
        let start_bit = self.cursor % SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
        let first = self.occupied[start_word] & (u64::MAX << start_bit);
        if first != 0 {
            return Some(start_word * SHELL_DOCUMENT_RETIREMENT_WORD_BITS + first.trailing_zeros() as usize);
        }
        for word_index in start_word + 1..SHELL_DOCUMENT_RETIREMENT_WORDS {
            let occupied = self.occupied[word_index];
            if occupied != 0 {
                return Some(word_index * SHELL_DOCUMENT_RETIREMENT_WORD_BITS + occupied.trailing_zeros() as usize);
            }
        }
        for word_index in 0..start_word {
            let occupied = self.occupied[word_index];
            if occupied != 0 {
                return Some(word_index * SHELL_DOCUMENT_RETIREMENT_WORD_BITS + occupied.trailing_zeros() as usize);
            }
        }
        if start_bit == 0 {
            return None;
        }
        let wrapped = self.occupied[start_word] & ((1_u64 << start_bit) - 1);
        (wrapped != 0).then_some(start_word * SHELL_DOCUMENT_RETIREMENT_WORD_BITS + wrapped.trailing_zeros() as usize)
    }

    pub fn mark_occupied(&mut self, index: usize) {
        let word = index / SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
        let bit = 1_u64 << (index % SHELL_DOCUMENT_RETIREMENT_WORD_BITS);
        assert_eq!(self.occupied[word] & bit, 0, "shell document retirement slot was indexed twice");
        self.occupied[word] |= bit;
        self.occupied_count = self.occupied_count.checked_add(1).expect("shell document retirement occupied count overflow");
    }

    pub fn clear_occupied(&mut self, index: usize) {
        let word = index / SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
        let bit = 1_u64 << (index % SHELL_DOCUMENT_RETIREMENT_WORD_BITS);
        assert_ne!(self.occupied[word] & bit, 0, "shell document retirement slot lost its occupied index");
        self.occupied[word] &= !bit;
        self.occupied_count = self.occupied_count.checked_sub(1).expect("shell document retirement occupied count underflow");
    }

    pub fn try_admit(&mut self, document: UiDocumentLease) -> Result<(), UiDocumentLease> {
        let Some(admission) = self.try_reserve_admission() else { return Err(document) };
        self.admit_reserved(admission, document);
        Ok(())
    }

    pub fn try_reserve_admission(&self) -> Option<ShellDocumentRetirementAdmission> {
        let index = self.first_vacant_index()?;
        let epoch = self.epochs[index].checked_add(1)?;
        Some(ShellDocumentRetirementAdmission { index, epoch })
    }

    pub fn admit_reserved(&mut self, admission: ShellDocumentRetirementAdmission, document: UiDocumentLease) {
        assert_eq!(self.first_vacant_index(), Some(admission.index), "shell document retirement admission lost its exact vacancy");
        assert_eq!(self.epochs[admission.index].checked_add(1), Some(admission.epoch), "shell document retirement admission epoch changed before transfer");
        assert!(self.slots[admission.index].is_none(), "shell document retirement vacancy index disagreed with owner storage");
        let generation = document.generation();
        let surface = document.header().ok().map(|header| header.surface);
        self.epochs[admission.index] = admission.epoch;
        self.slots[admission.index] = Some(ShellDocumentRetirementSlot { epoch: admission.epoch, generation, surface, document });
        self.mark_occupied(admission.index);
    }

    /// ♻️ Advances the NEXT occupied slot by one PAGE.
    ///
    /// ⚖️ Two prices were wrong here and both were invisible: the cursor advanced by one position per
    /// call whether or not that position held anything, so a registry holding six documents in a
    /// 512-slot ring spent 99 % of its steps on vacancies; and each real step was granted
    /// `close_step()`'s one item / 4 KiB, which is a per-ITEM price for a walk whose unit is a page.
    /// Together they made [`SHELL_DOCUMENT_RETIREMENT_STEPS`] buy ~128 real steps per document, far
    /// short of what one 120-node document's typed descendants need — so a refresh's previous set was
    /// still resident when the next asked the process-wide aggregate for credit, and every surface of
    /// the second refresh was refused `Capacity`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Priced per page, exactly as
    /// `📓️close-ladder-budget-2026-09-12.md` §3 prices every other retirement in this shell.
    /// The fixed occupied-word index preserves the same physical-slot cursor order while bounding
    /// sparse discovery to eight words; `occupied_count` makes the empty decision scalar.
    pub fn close_one(&mut self) -> bool {
        let Some(index) = self.next_occupied_index() else { return false };
        self.cursor = (index + 1) % SHELL_DOCUMENT_RETIREMENT_CAPACITY;
        let slot = self.slots[index].as_mut().expect("the located shell document retirement slot is occupied");
        let terminal = slot.document.close_step_with_grant(SHELL_DOCUMENT_RETIREMENT_ITEMS, SHELL_DOCUMENT_RETIREMENT_BYTES).is_ok_and(|step| step.complete) && slot.document.terminal_is_empty();
        if terminal {
            let terminal = self.slots[index].take().expect("terminal shell document retirement slot");
            assert!(terminal.document.terminal_is_empty(), "shell document retirement witness changed before removal");
            self.clear_occupied(index);
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.occupied_count == 0
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn qualified_owners(&self) -> impl Iterator<Item = (u64, u64, Option<&SurfaceId>)> {
        self.slots.iter().flatten().map(|slot| (slot.epoch, slot.generation, slot.surface.as_ref()))
    }
}

//#endregion 📄️ShellDocumentRetirement
//#endregion ShellTypes

//#region ShellTypes
//#region 📄️ShellDocumentRetirement
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShellDocumentRetirementAdmission {
    pub index: usize,
    pub epoch: u64,
}
//#endregion 📄️ShellDocumentRetirement
//#endregion ShellTypes

//#region ShellTypes
//#region 📄️ShellDocumentRetirement
pub const SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES: usize = std::mem::size_of::<[u64; SHELL_DOCUMENT_RETIREMENT_WORDS]>() + std::mem::size_of::<u64>();
//#endregion 📄️ShellDocumentRetirement
//#endregion ShellTypes
