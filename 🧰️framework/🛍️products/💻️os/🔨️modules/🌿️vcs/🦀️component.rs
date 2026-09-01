//! 🗄️ Generic document version-graph algebra — Author/Change/Checkpoint/Alternative/ArtifactVcs,
//! `VcsError`, content-addressed checkpoint ids, and the raw collection-diff/operation helpers. Pure
//! data plus pure functions: nothing here touches a live document (that's `store::ArtifactStore`,
//! which depends on this crate — see `26/07/28/EXTRACT-STORE-INTO-ITS-OWN-TECHNOLOGY`).

use serde::{Deserialize, Serialize};

// This crate's own body spells the trait name bare (`self::Mutation<P>` in `apply_mutation`
// below, disambiguating the trait from the same-named generic parameter) — a private (non-`pub`)
// import keeps that ergonomics without re-exposing `crate::os_spr::Mutation` on `vcs`'s own public API
// (dependents import `crate::os_spr::Mutation` directly). `MutationDiff` is imported for its `apply`
// method, called on `Mutation::Diff` inside `apply_mutation`.
use crate::os_spr::{Edit, Mutation, MutationApplyError, MutationDiff};

//#region 🆔️Ids
/// @emoji 🔑 Content-addressed entity id: `{prefix}-{hex16(blake3(prefix || 0 || payload))}`.
pub async fn content_addressed_entity_id(prefix: &str, payload: &[u8]) -> String {
    let mut input = prefix.as_bytes().to_vec();
    input.push(0);
    input.extend_from_slice(payload);
    let digest = *semio_framework_hash::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{prefix}-{hex16}")
}

/// @emoji 🆔️ Deterministic child id scoped to an edit: blake3(`{edit_id}:{ordinal}`).
pub async fn edit_scoped_id(edit_id: &str, ordinal: u32) -> String {
    let digest = semio_framework_hash::hash(format!("{edit_id}:{ordinal}").as_bytes());
    let hex16: String = digest.as_bytes()[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("scoped-{hex16}")
}

/// @emoji ✏️ Content-addressed edit id from actor + sequence + forwards fingerprint (no global counter).
pub async fn mint_edit_id(actor: Option<&str>, sequence: i32, forwards_fingerprint: &[u8]) -> String {
    let mut payload = Vec::new();
    payload.extend_from_slice(actor.unwrap_or("").as_bytes());
    payload.push(0);
    payload.extend_from_slice(&sequence.to_le_bytes());
    payload.push(0);
    payload.extend_from_slice(forwards_fingerprint);
    content_addressed_entity_id("edit", &payload).await
}

/// @emoji 📦️ Content-addressed change id from ordered edit ids (+ optional description distinguisher).
pub async fn mint_change_id(edit_ids: &[String], description: Option<&str>) -> String {
    let mut payload = edit_ids.join("\0").into_bytes();
    payload.push(0);
    payload.extend_from_slice(description.unwrap_or("").as_bytes());
    content_addressed_entity_id("change", &payload).await
}

/// @emoji 🌿️ Content-addressed alternative id from name + ordered checkpoint ids.
pub async fn mint_alternative_id(name: &str, checkpoint_ids: &[String]) -> String {
    let mut payload = name.as_bytes().to_vec();
    payload.push(0);
    payload.extend_from_slice(checkpoint_ids.join("\0").as_bytes());
    content_addressed_entity_id("alternative", &payload).await
}

/// @emoji ⚙️ Content-addressed operation id from the operation's binary (or other) fingerprint bytes.
pub async fn mint_mutation_id(mutation_bytes: &[u8]) -> String {
    content_addressed_entity_id("mutation", mutation_bytes).await
}

/// @emoji 🆔️ Legacy-compatible prefix-only mint — identical inputs collide.
/// Prefer [`mint_edit_id`] / [`mint_change_id`] / [`mint_alternative_id`] / [`mint_mutation_id`] /
/// [`content_addressed_entity_id`] with a distinguishing payload.
pub async fn create_document_vcs_id(prefix: &str) -> String {
    content_addressed_entity_id(prefix, prefix.as_bytes()).await
}
//#endregion 🆔️Ids

//#region 🔖️Schemas
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

// 🎞️ `MutationMeta` lives in `protocol_command`; `Edit<Mutation>` (imported above) is this
// crate's own field type for `ArtifactVcs.edits` below.

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub id: String,
    pub edit_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub saved_at: String,
}

/// @emoji 🧩️ One owned child's checkpoint pin, captured on the parent's checkpoint so checking out
/// the parent can restore the whole composition. `child_ref` is the pinned child artifact's real
/// `crate::os_io::ArtifactRef` — **correction, `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/
/// b2-store-composition-report.md`**: the prior wave (`b1-spr-vcs-report.md`) believed `ArtifactRef`
/// (defined in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`) was unreachable from this crate and fell
/// back to the wire URI `String`. That was wrong: `io/🦀️component.rs` is dual-mounted — the
/// `semio-framework` crate mounts it as `io`, and THIS crate (`semio-framework-os-kernel`) mounts the
/// very same source file as `os_io` (see `💻️os/📦️packages/🦀️rust/📦️glue.rs:237-238`,
/// `pub mod os_io;`) — no cross-crate dependency-direction problem exists; `store` already reaches
/// `crate::os_io::ArtifactDialect` directly (`🏪️store/🦀️component.rs:88/105/662`). Sorting for
/// [`content_addressed_checkpoint_id`] below is therefore by `child_ref.to_uri()` (the same
/// deterministic string this field used to store literally), not by any `Ord` on `ArtifactRef`
/// itself (which does not implement one).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositionPin {
    pub child_ref: crate::os_io::ArtifactRef,
    pub checkpoint_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checkpoint {
    pub id: String,
    pub change_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: String,
    /// @emoji 🧩️ Which checkpoint each owned child was at when this checkpoint was committed —
    /// empty for a non-composite artifact (every checkpoint before this ticket, and every leaf
    /// artifact after it). Additive; see [`CompositionPin`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub composition_pins: Vec<CompositionPin>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

pub const ARTIFACT_HISTORY_LEDGER_CAPACITY: usize = 64;

//#region 🧩️GroupHistoryVisibility
/// 🪟 A shared decision bit used by prepared read roots; only its unique publisher can switch it.
pub(crate) struct ArtifactGroupVisibility {
    state: std::sync::atomic::AtomicU8,
}

pub(crate) struct ArtifactGroupReadDecision<'a> {
    visibility: &'a ArtifactGroupVisibility,
    committed: bool,
}

impl ArtifactGroupReadDecision<'_> {
    pub(crate) fn committed_for(&self, visibility: &ArtifactGroupVisibility) -> Result<bool, ()> {
        if std::ptr::eq(self.visibility, visibility) { Ok(self.committed) } else { Err(()) }
    }
}

/// 🗝️ Unique low-level publication owner; preparation and freshness remain the Store coordinator's responsibility.
pub(crate) struct ArtifactGroupVisibilityOwner {
    view: std::sync::Arc<ArtifactGroupVisibility>,
}

impl ArtifactGroupVisibilityOwner {
    pub(crate) fn new() -> Self {
        Self { view: std::sync::Arc::new(ArtifactGroupVisibility { state: std::sync::atomic::AtomicU8::new(0) }) }
    }

    pub(crate) fn view(&self) -> std::sync::Arc<ArtifactGroupVisibility> {
        std::sync::Arc::clone(&self.view)
    }

    pub(crate) fn commit(&mut self) -> bool {
        self.view.state.compare_exchange(0, 1, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }

    pub(crate) fn abort(&mut self) -> bool {
        self.view.state.compare_exchange(0, 2, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }
}

impl Drop for ArtifactGroupVisibilityOwner {
    fn drop(&mut self) {
        let _ = self.abort();
    }
}

impl ArtifactGroupVisibility {
    pub(crate) fn capture(&self) -> ArtifactGroupReadDecision<'_> {
        ArtifactGroupReadDecision { visibility: self, committed: self.committed() }
    }

    pub(crate) fn committed(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Acquire) == 1
    }

    pub(crate) fn pending(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Acquire) == 0
    }
}

struct ArtifactHistoryGroupSuffix {
    visibility: std::sync::Arc<ArtifactGroupVisibility>,
    head: Option<u16>,
    tail: Option<u16>,
    len: usize,
}

#[cfg(test)]
mod group_history_visibility_tests {
    use super::*;

    fn observed(ledger: &ArtifactHistoryLedger<i32>) -> serde_json::Value {
        serde_json::to_value(ledger).expect("independent serializer sees the selected exact history")
    }

    #[test]
    fn retained_group_history_switches_every_direct_reader_at_one_decision() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("./🧪️group-history.json")).expect("group history fixture");
        let mut first = ArtifactHistoryLedger::new();
        let mut second = ArtifactHistoryLedger::new();
        first.try_push(0).expect("first seed");
        second.try_push(-1).expect("second seed");
        let mut owner = ArtifactGroupVisibilityOwner::new();
        let view = owner.view();
        for row in fixture["ordered"].as_array().expect("ordered fixture") {
            let ledger = if row["member"] == "a" { &mut first } else { &mut second };
            let reservation = ledger.reserve_group_one(&view).expect("exact suffix reservation");
            ledger.stage_group_reserved(reservation, row["value"].as_i64().expect("value") as i32, &view).expect("one prepared history owner");
            assert_eq!(observed(&first), fixture["members"][0]["before"]);
            assert_eq!(observed(&second), fixture["members"][1]["before"]);
            assert_eq!((first.len(), first.first(), first.last(), first.get(1)), (1, Some(&0), Some(&0), None));
            assert_eq!(first.iter().rev().copied().collect::<Vec<_>>(), vec![0]);
            assert!(first.reserve_one().is_err());
            assert_eq!(first.last_mut(), None);
        }
        assert!(owner.commit());
        assert!(!owner.commit());
        assert!(!owner.abort());
        assert_eq!(observed(&first), fixture["members"][0]["after"]);
        assert_eq!(observed(&second), fixture["members"][1]["after"]);
        assert_eq!((first.len(), first.last(), first.get(1)), (3, Some(&42), Some(&17)));
        assert_eq!(first.iter().rev().copied().collect::<Vec<_>>(), vec![42, 17, 0]);
        assert!(first.abort_group_one(&view).is_err());
        first.adopt_group(&view).expect("first non-publishing adoption");
        assert_eq!(observed(&second), fixture["members"][1]["after"]);
        second.adopt_group(&view).expect("second non-publishing adoption");
        assert_eq!(observed(&first), fixture["members"][0]["after"]);
        while first.pop().is_some() {}
        while second.pop().is_some() {}
        assert!(first.terminal_is_empty() && second.terminal_is_empty());
    }

    #[test]
    fn retained_group_history_abort_transfers_one_exact_owner_and_rejects_foreign_decisions() {
        let mut ledger = ArtifactHistoryLedger::new();
        ledger.try_push(0).expect("seed");
        let mut owner = ArtifactGroupVisibilityOwner::new();
        let view = owner.view();
        let mut foreign = ArtifactGroupVisibilityOwner::new();
        let wrong = foreign.view();
        for value in [17, 42] {
            let reservation = ledger.reserve_group_one(&view).expect("exact group reservation");
            ledger.stage_group_reserved(reservation, value, &view).expect("exact staged owner");
        }
        assert!(ledger.reserve_group_one(&wrong).is_err());
        assert!(foreign.abort());
        assert!(ledger.abort_group_one(&wrong).is_err());
        assert!(ledger.abort_group_one(&view).is_err());
        assert!(owner.abort());
        assert!(!owner.commit());
        assert_eq!(ledger.abort_group_one(&view), Ok(Some(42)));
        assert_eq!(observed(&ledger), serde_json::json!([0]));
        assert_eq!(ledger.abort_group_one(&view), Ok(Some(17)));
        assert!(!ledger.terminal_is_empty());
        assert_eq!(ledger.abort_group_one(&view), Ok(None));
        assert_eq!(ledger.pop(), Some(0));
        assert!(ledger.terminal_is_empty());
    }
}
//#endregion 🧩️GroupHistoryVisibility

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactHistoryKey {
    pub index: u16,
    pub generation: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArtifactHistoryReservation {
    authority: usize,
    index: u16,
    generation: u32,
}

struct ArtifactHistorySlot<T> {
    generation: u32,
    previous: Option<u16>,
    next: Option<u16>,
    free_next: Option<u16>,
    value: Option<T>,
}

/// @emoji 📚️ Fixed-capacity generation-keyed history authority. Live entries form one stable
/// linked order; removed slots are tombstoned and reused only after their generation advances.
pub struct ArtifactHistoryLedger<T> {
    slots: std::mem::ManuallyDrop<Vec<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>>,
    head: Option<u16>,
    tail: Option<u16>,
    free_head: Option<u16>,
    reservation: Option<ArtifactHistoryReservation>,
    group: Option<ArtifactHistoryGroupSuffix>,
    len: usize,
}

impl<T> ArtifactHistoryLedger<T> {
    pub fn new() -> Self {
        Self { slots: std::mem::ManuallyDrop::new(Vec::with_capacity(ARTIFACT_HISTORY_LEDGER_CAPACITY)), head: None, tail: None, free_head: None, reservation: None, group: None, len: 0 }
    }

    fn slot(&self, index: u16) -> &ArtifactHistorySlot<T> {
        unsafe { self.slots[index as usize].assume_init_ref() }
    }

    fn slot_mut(&mut self, index: u16) -> &mut ArtifactHistorySlot<T> {
        unsafe { self.slots[index as usize].assume_init_mut() }
    }

    fn authority(&self) -> usize {
        self.slots.as_ptr() as usize
    }

    pub fn reserve_one(&mut self) -> Result<ArtifactHistoryReservation, ()> {
        if self.group.is_some() {
            return Err(());
        }
        self.reserve_slot()
    }

    fn reserve_slot(&mut self) -> Result<ArtifactHistoryReservation, ()> {
        if self.reservation.is_some() {
            return Err(());
        }
        let (index, generation) = if let Some(index) = self.free_head {
            let generation = self.slot(index).generation.checked_add(1).ok_or(())?;
            (index, generation)
        } else {
            if self.slots.len() == ARTIFACT_HISTORY_LEDGER_CAPACITY {
                return Err(());
            }
            (self.slots.len() as u16, 1)
        };
        let reservation = ArtifactHistoryReservation { authority: self.authority(), index, generation };
        self.reservation = Some(ArtifactHistoryReservation { authority: reservation.authority, index, generation });
        Ok(reservation)
    }

    pub fn cancel_reservation(&mut self, reservation: ArtifactHistoryReservation) -> Result<(), ArtifactHistoryReservation> {
        if self.reservation.as_ref() != Some(&reservation) {
            return Err(reservation);
        }
        self.reservation = None;
        Ok(())
    }

    pub fn insert_reserved(&mut self, reservation: ArtifactHistoryReservation, value: T) -> Result<ArtifactHistoryKey, (ArtifactHistoryReservation, T)> {
        if self.group.is_some() {
            return Err((reservation, value));
        }
        let key = self.insert_owned_slot(reservation, value, self.tail)?;
        if self.head.is_none() {
            self.head = Some(key.index);
        }
        self.tail = Some(key.index);
        self.len += 1;
        Ok(key)
    }

    fn insert_owned_slot(&mut self, reservation: ArtifactHistoryReservation, value: T, previous: Option<u16>) -> Result<ArtifactHistoryKey, (ArtifactHistoryReservation, T)> {
        if reservation.authority != self.authority() || self.reservation.as_ref() != Some(&reservation) {
            return Err((reservation, value));
        }
        let index = reservation.index;
        let generation = reservation.generation;
        self.reservation = None;
        if Some(index) == self.free_head {
            let free_next = self.slot(index).free_next;
            self.free_head = free_next;
            let slot = self.slot_mut(index);
            slot.generation = generation;
            slot.previous = previous;
            slot.next = None;
            slot.free_next = None;
            slot.value = Some(value);
        } else if index as usize == self.slots.len() && self.slots.len() < ARTIFACT_HISTORY_LEDGER_CAPACITY {
            self.slots.push(std::mem::MaybeUninit::new(ArtifactHistorySlot { generation, previous, next: None, free_next: None, value: Some(value) }));
        } else {
            return Err((reservation, value));
        }
        if let Some(tail) = previous {
            self.slot_mut(tail).next = Some(index);
        }
        Ok(ArtifactHistoryKey { index, generation })
    }

    pub(crate) fn reserve_group_one(&mut self, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<ArtifactHistoryReservation, ()> {
        if !visibility.pending() || self.group.as_ref().is_some_and(|group| !std::sync::Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(());
        }
        let reservation = self.reserve_slot()?;
        if self.group.is_none() {
            self.group = Some(ArtifactHistoryGroupSuffix { visibility: std::sync::Arc::clone(visibility), head: None, tail: None, len: 0 });
        }
        Ok(reservation)
    }

    pub(crate) fn stage_group_reserved(&mut self, reservation: ArtifactHistoryReservation, value: T, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<ArtifactHistoryKey, (ArtifactHistoryReservation, T)> {
        let Some(group) = self.group.as_ref().filter(|group| std::sync::Arc::ptr_eq(&group.visibility, visibility) && visibility.pending()) else {
            return Err((reservation, value));
        };
        let previous = group.tail.or(self.tail);
        let key = self.insert_owned_slot(reservation, value, previous)?;
        let group = self.group.as_mut().expect("validated group suffix remains owned");
        if group.head.is_none() {
            group.head = Some(key.index);
        }
        group.tail = Some(key.index);
        group.len += 1;
        Ok(key)
    }

    pub(crate) fn adopt_group(&mut self, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<(), ()> {
        if self.reservation.is_some() || !visibility.committed() || !self.group.as_ref().is_some_and(|group| std::sync::Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(());
        }
        let group = self.group.take().expect("validated group suffix remains owned");
        self.head = self.head.or(group.head);
        self.tail = group.tail.or(self.tail);
        self.len += group.len;
        Ok(())
    }

    pub(crate) fn abort_group_one(&mut self, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<Option<T>, ()> {
        if self.reservation.is_some() || visibility.pending() || visibility.committed() {
            return Err(());
        }
        let group = self.group.as_ref().filter(|group| std::sync::Arc::ptr_eq(&group.visibility, visibility)).ok_or(())?;
        let Some(index) = group.tail else {
            self.group = None;
            return Ok(None);
        };
        let previous = self.slot(index).previous;
        if let Some(previous) = previous {
            self.slot_mut(previous).next = None;
        }
        let free_head = self.free_head;
        let value = {
            let slot = self.slot_mut(index);
            slot.previous = None;
            slot.next = None;
            slot.free_next = free_head;
            slot.value.take().expect("aborted group slot retains its exact entry owner")
        };
        self.free_head = Some(index);
        let group = self.group.as_mut().expect("validated aborted group remains owned");
        group.len -= 1;
        group.tail = if group.len == 0 { None } else { previous };
        if group.len == 0 {
            group.head = None;
        }
        Ok(Some(value))
    }

    fn visible_bounds(&self) -> (Option<u16>, Option<u16>, usize) {
        match self.group.as_ref().filter(|group| group.visibility.committed()) {
            Some(group) => (self.head.or(group.head), group.tail.or(self.tail), self.len + group.len),
            None => (self.head, self.tail, self.len),
        }
    }

    pub(crate) fn group_visibility(&self) -> Option<&ArtifactGroupVisibility> {
        self.group.as_ref().map(|group| group.visibility.as_ref())
    }

    pub(crate) fn read_group(&self, decision: Option<&ArtifactGroupReadDecision<'_>>) -> Result<ArtifactHistoryIter<'_, T>, ()> {
        let (front, back, remaining) = match self.group.as_ref() {
            Some(group) if decision.ok_or(())?.committed_for(&group.visibility)? => (self.head.or(group.head), group.tail.or(self.tail), self.len + group.len),
            _ => (self.head, self.tail, self.len),
        };
        Ok(ArtifactHistoryIter { ledger: self, front, back, remaining })
    }

    pub fn try_push(&mut self, value: T) -> Result<ArtifactHistoryKey, T> {
        let reservation = match self.reserve_one() {
            Ok(reservation) => reservation,
            Err(()) => return Err(value),
        };
        self.insert_reserved(reservation, value).map_err(|(_, value)| value)
    }

    pub fn try_from_preflighted(values: Vec<T>) -> Result<Self, Vec<T>> {
        if values.len() > ARTIFACT_HISTORY_LEDGER_CAPACITY {
            return Err(values);
        }
        let mut ledger = Self::new();
        let mut pending = values.into_iter();
        while let Some(value) = pending.next() {
            if let Err(value) = ledger.try_push(value) {
                let mut rejected = Vec::with_capacity(ARTIFACT_HISTORY_LEDGER_CAPACITY + 1);
                while let Some(established) = ledger.pop() {
                    rejected.push(established);
                }
                rejected.reverse();
                rejected.push(value);
                rejected.extend(pending);
                return Err(rejected);
            }
        }
        Ok(ledger)
    }

    pub fn remove_key(&mut self, key: ArtifactHistoryKey) -> Result<T, ArtifactHistoryKey> {
        if self.reservation.is_some() || self.group.is_some() {
            return Err(key);
        }
        if key.index as usize >= self.slots.len() {
            return Err(key);
        }
        let slot = self.slot(key.index);
        if slot.generation != key.generation || slot.value.is_none() {
            return Err(key);
        }
        let previous = slot.previous;
        let next = slot.next;
        if let Some(previous) = previous {
            self.slot_mut(previous).next = next;
        } else {
            self.head = next;
        }
        if let Some(next) = next {
            self.slot_mut(next).previous = previous;
        } else {
            self.tail = previous;
        }
        let free_head = self.free_head;
        let value = {
            let slot = self.slot_mut(key.index);
            slot.previous = None;
            slot.next = None;
            slot.free_next = free_head;
            slot.value.take().expect("validated history slot retains its exact owner")
        };
        self.free_head = Some(key.index);
        self.len -= 1;
        Ok(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = self.tail?;
        let generation = self.slot(index).generation;
        self.remove_key(ArtifactHistoryKey { index, generation }).ok()
    }

    pub fn first(&self) -> Option<&T> {
        self.visible_bounds().0.and_then(|index| self.slot(index).value.as_ref())
    }

    pub fn last(&self) -> Option<&T> {
        self.visible_bounds().1.and_then(|index| self.slot(index).value.as_ref())
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.group.is_some() {
            return None;
        }
        let index = self.tail?;
        self.slot_mut(index).value.as_mut()
    }

    pub fn get(&self, position: usize) -> Option<&T> {
        self.iter().nth(position)
    }

    pub fn get_mut(&mut self, position: usize) -> Option<&mut T> {
        self.iter_mut().nth(position)
    }

    pub fn len(&self) -> usize {
        self.visible_bounds().2
    }

    pub fn has_capacity(&self) -> bool {
        self.group.is_none() && self.reservation.is_none() && (self.free_head.is_some_and(|index| self.slot(index).generation != u32::MAX) || self.slots.len() < ARTIFACT_HISTORY_LEDGER_CAPACITY)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> ArtifactHistoryIter<'_, T> {
        let (front, back, remaining) = self.visible_bounds();
        ArtifactHistoryIter { ledger: self, front, back, remaining }
    }

    pub fn iter_mut(&mut self) -> ArtifactHistoryIterMut<'_, T> {
        assert!(self.group.is_none(), "mutable history iteration requires its staged group to be adopted or aborted");
        ArtifactHistoryIterMut { slots: &mut *self.slots, front: self.head, back: self.tail, remaining: self.len, marker: std::marker::PhantomData }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.head.is_none() && self.tail.is_none() && self.reservation.is_none() && self.group.is_none()
    }
}

impl<T> Drop for ArtifactHistoryLedger<T> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "artifact history ledger reached Drop before every exact entry owner was retired");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.slots) };
    }
}

pub struct ArtifactHistoryIter<'a, T> {
    ledger: &'a ArtifactHistoryLedger<T>,
    front: Option<u16>,
    back: Option<u16>,
    remaining: usize,
}

impl<T> Clone for ArtifactHistoryIter<'_, T> {
    fn clone(&self) -> Self {
        Self { ledger: self.ledger, front: self.front, back: self.back, remaining: self.remaining }
    }
}

impl<T: Serialize> Serialize for ArtifactHistoryIter<'_, T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.remaining))?;
        for entry in self.clone() { sequence.serialize_element(entry)?; }
        sequence.end()
    }
}

impl<'a, T> Iterator for ArtifactHistoryIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.front?;
        let slot = self.ledger.slot(index);
        self.front = slot.next;
        self.remaining -= 1;
        slot.value.as_ref()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> DoubleEndedIterator for ArtifactHistoryIter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.back?;
        let slot = self.ledger.slot(index);
        self.back = slot.previous;
        self.remaining -= 1;
        slot.value.as_ref()
    }
}

impl<T> ExactSizeIterator for ArtifactHistoryIter<'_, T> {}

pub struct ArtifactHistoryIterMut<'a, T> {
    slots: *mut Vec<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>,
    front: Option<u16>,
    back: Option<u16>,
    remaining: usize,
    marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for ArtifactHistoryIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.front?;
        let slot = unsafe { (&mut *self.slots)[index as usize].assume_init_mut() };
        self.front = slot.next;
        self.remaining -= 1;
        slot.value.as_mut().map(|value| unsafe { &mut *(value as *mut T) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> DoubleEndedIterator for ArtifactHistoryIterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.back?;
        let slot = unsafe { (&mut *self.slots)[index as usize].assume_init_mut() };
        self.back = slot.previous;
        self.remaining -= 1;
        slot.value.as_mut().map(|value| unsafe { &mut *(value as *mut T) })
    }
}

impl<T> ExactSizeIterator for ArtifactHistoryIterMut<'_, T> {}

impl<'a, T> IntoIterator for &'a ArtifactHistoryLedger<T> {
    type Item = &'a T;
    type IntoIter = ArtifactHistoryIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut ArtifactHistoryLedger<T> {
    type Item = &'a mut T;
    type IntoIter = ArtifactHistoryIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> std::ops::Index<usize> for ArtifactHistoryLedger<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("artifact history index outside the live deterministic order")
    }
}

impl<T> std::ops::IndexMut<usize> for ArtifactHistoryLedger<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("artifact history index outside the live deterministic order")
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ArtifactHistoryLedger<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq> PartialEq for ArtifactHistoryLedger<T> {
    fn eq(&self, other: &Self) -> bool {
        let left = self.iter();
        let right = other.iter();
        left.len() == right.len() && left.zip(right).all(|(left, right)| left == right)
    }
}

impl<T: Eq> Eq for ArtifactHistoryLedger<T> {}

impl<T: Serialize> Serialize for ArtifactHistoryLedger<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let entries = self.iter();
        let mut sequence = serializer.serialize_seq(Some(entries.len()))?;
        for entry in entries {
            sequence.serialize_element(entry)?;
        }
        sequence.end()
    }
}

#[derive(Debug, PartialEq)]
pub struct ArtifactVcs<P, Mutation> {
    pub initial_snapshot: P,
    pub edits: ArtifactHistoryLedger<Edit<Mutation>>,
    pub changes: ArtifactHistoryLedger<Change>,
    pub checkpoints: ArtifactHistoryLedger<Checkpoint>,
    pub alternatives: ArtifactHistoryLedger<Alternative>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtifactVcsRead<'a, P, Mutation> {
    initial_snapshot: &'a P,
    edits: ArtifactHistoryIter<'a, Edit<Mutation>>,
    changes: ArtifactHistoryIter<'a, Change>,
    checkpoints: ArtifactHistoryIter<'a, Checkpoint>,
    alternatives: ArtifactHistoryIter<'a, Alternative>,
}

impl<P, Mutation> ArtifactVcs<P, Mutation> {
    pub(crate) fn group_visibility(&self) -> Result<Option<&ArtifactGroupVisibility>, ()> {
        let mut visibility: Option<&ArtifactGroupVisibility> = None;
        for candidate in [self.edits.group_visibility(), self.changes.group_visibility(), self.checkpoints.group_visibility(), self.alternatives.group_visibility()].into_iter().flatten() {
            if visibility.is_some_and(|owner| !std::ptr::eq(owner, candidate)) { return Err(()); }
            visibility = Some(candidate);
        }
        Ok(visibility)
    }

    pub(crate) fn read_group(&self, decision: Option<&ArtifactGroupReadDecision<'_>>) -> Result<ArtifactVcsRead<'_, P, Mutation>, ()> {
        Ok(ArtifactVcsRead { initial_snapshot: &self.initial_snapshot, edits: self.edits.read_group(decision)?, changes: self.changes.read_group(decision)?, checkpoints: self.checkpoints.read_group(decision)?, alternatives: self.alternatives.read_group(decision)? })
    }
}

impl<P: Serialize, Mutation: Serialize> Serialize for ArtifactVcs<P, Mutation> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let decision = self.group_visibility().map_err(|()| serde::ser::Error::custom("VCS read contains different group visibility authorities"))?.map(ArtifactGroupVisibility::capture);
        self.read_group(decision.as_ref()).map_err(|()| serde::ser::Error::custom("VCS read lost its exact captured visibility decision"))?.serialize(serializer)
    }
}
//#endregion 🔖️Schemas
//#region 🔖️Errors
// 🎞️ `Eq` dropped: `Rejected` below carries
// `Vec<crate::os_spr::MutationMessage>`, and `MutationMessage` itself only derives `PartialEq`
// (not `Eq`) — see `📡️spr/🎮️command`'s `🔖️Message` region.
#[derive(Debug, PartialEq)]
pub enum VcsError {
    UnknownEdit(String),
    UnknownChange(String),
    UnknownAlternative(String),
    NoCheckpoint,
    EmptyApply,
    MutationApply(MutationApplyError),
    NothingToUndo,
    ForeignEdit(String),
    NothingToRedo,
    Serialize(String),
    Deserialize(String),
    Backbone(String),
    /// @emoji 🧬️ A migration/replay/merge was attempted across two envelopes/mutations whose
    /// `dialect` coordinates don't match (see `store::ArtifactEnvelope::dialect`, `26/08/10` D4
    /// evolution slice). Not yet raised by any call site in this pass — additive only.
    DialectMismatch(String),
    /// @emoji 🧬️ An operation needs a dialect migration to run first (see `store::migrate_document`)
    /// before it can proceed. Not yet raised by any call site in this pass — additive only.
    MigrationRequired(String),
    /// @emoji 🧬️ A registered dialect migration ran but failed. Not yet raised by any call site in
    /// this pass — additive only.
    MigrationFailed(String),
    /// @emoji 🔁️ A composition-pin graph traversal (parent → child → …) found a cycle back to an
    /// ancestor — an owned-child forest must stay acyclic. Raised by `store::CompositionGraph::
    /// would_cycle_owns`/`would_cycle_links` via `store::CompositionCoordinator::dispatch_group`'s
    /// phase-1 validation (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` `🔖️CompositionCoordinator`, wave B2).
    CompositionCycle(String),
    /// @emoji 🚫️ An operation would violate composition's single-ownership invariant (e.g.
    /// adopting a child that already has a different owner, or dispatching to a child a group's
    /// stated parent does not actually own). Raised by `store::CompositionGraph::insert_owns` and
    /// `store::CompositionCoordinator::dispatch_group`'s phase-1 ownership check.
    OwnershipViolation(String),
    /// @emoji 🛂️ A structural failure rejected an operation during
    /// `store::CompositionCoordinator::dispatch_group`'s phase-1 pass (or the object-safe
    /// `store::SpaceMember::preview_wire`/`dispatch_wire` bridge that pass uses) — the group is
    /// aborted with zero side effects anywhere. Reserved for structural failures only (ticket
    /// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): an ordinary
    /// mutation-level rejection now travels as a `MutationMessage` on the op's own
    /// `MutationOutcome`, never through this variant. Additive; not raised anywhere else in this
    /// crate (every other command path reports its own more specific `VcsError` variant).
    ValidationFailed(String),
    /// @emoji 🧯️ A `CompositionCoordinator::dispatch_group` call failed AFTER some members were
    /// already applied, and the reverse-order `Undo` compensation pass (see that method's doc
    /// comment) itself failed on at least one member — i.e. the group could not be fully rolled
    /// back. The message embeds a human-readable rollback report (which members compensated
    /// cleanly, which did not, and why) so a caller can surface/log the exact partial state rather
    /// than silently losing it. This is the one path in this crate where a command's Result can
    /// legitimately leave a multi-member gesture inconsistent — every other `VcsError` variant is
    /// raised BEFORE any mutation lands.
    CompensationFailed(String),
    /// @emoji 🛑️ A command was rejected WHOLESALE by the authority's own `crate::os_spr::MergePolicy`
    /// — nothing in the command was applied (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
    /// CLASS-CONFLICTS` §C6). Carries the policy that rejected it and every message the rejected
    /// replay produced, so a caller can explain the rejection without re-running anything.
    /// `ValidationFailed` survives ONLY for structural failures now — an ordinary mutation-level
    /// rejection travels through this variant instead.
    Rejected {
        policy: crate::os_spr::MergePolicy,
        messages: Vec<crate::os_spr::MutationMessage>,
    },
    /// @emoji ❓️ `store::ArtifactStore::resolve_conflict` was called with an id that names no
    /// currently-`Open` conflict on this store.
    UnknownConflict(String),
}

impl std::fmt::Display for VcsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownEdit(id) => write!(formatter, "unknown edit id: {id}"),
            Self::UnknownChange(id) => write!(formatter, "unknown change id: {id}"),
            Self::UnknownAlternative(id) => write!(formatter, "unknown alternative id: {id}"),
            Self::NoCheckpoint => formatter.write_str("no checkpoint for alternative"),
            Self::EmptyApply => formatter.write_str("empty apply command"),
            Self::MutationApply(error) => write!(formatter, "mutation diff rejected: {error}"),
            Self::NothingToUndo => formatter.write_str("nothing to undo"),
            Self::ForeignEdit(id) => write!(formatter, "cannot undo edit authored by another actor: {id}"),
            Self::NothingToRedo => formatter.write_str("nothing to redo"),
            Self::Serialize(message) => write!(formatter, "serialize error: {message}"),
            Self::Deserialize(message) => write!(formatter, "deserialize error: {message}"),
            Self::Backbone(message) => write!(formatter, "backbone error: {message}"),
            Self::DialectMismatch(message) => write!(formatter, "dialect mismatch: {message}"),
            Self::MigrationRequired(message) => write!(formatter, "migration required: {message}"),
            Self::MigrationFailed(message) => write!(formatter, "migration failed: {message}"),
            Self::CompositionCycle(message) => write!(formatter, "composition cycle: {message}"),
            Self::OwnershipViolation(message) => write!(formatter, "ownership violation: {message}"),
            Self::ValidationFailed(message) => write!(formatter, "validation failed: {message}"),
            Self::CompensationFailed(message) => write!(formatter, "group dispatch failed and rollback also failed: {message}"),
            Self::Rejected { policy, .. } => write!(formatter, "rejected by merge policy {policy:?}"),
            Self::UnknownConflict(id) => write!(formatter, "unknown conflict id: {id}"),
        }
    }
}

impl std::error::Error for VcsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MutationApply(error) => Some(error),
            _ => None,
        }
    }
}

impl From<MutationApplyError> for VcsError {
    fn from(error: MutationApplyError) -> Self {
        Self::MutationApply(error)
    }
}

protocol::fault_from_error!(VcsError, crate::os_dsl::FaultOrigin::Module, "module.vcs");

//#endregion 🔖️Errors
//#region 🔖️CollectionDiff
/// @emoji 🧩️ Sparse collection patch entry (mirrors semio_compose_rs `XModified`).
///
/// 🎞️ Canonical collection patch entry for sparse collection diffs (re-exported by `crate::os_spr`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {
    pub id: TId,
    pub patch: TPatch,
}

/// @emoji 🧩️ Sparse collection diff (mirrors semio_compose_rs `XCollectionDiff`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDiff<TId, TPatch, TAdded> {
    pub removed: Vec<TId>,
    pub modified: Vec<ItemPatch<TId, TPatch>>,
    pub added: Vec<TAdded>,
}

impl<TId, TPatch, TAdded> Default for CollectionDiff<TId, TPatch, TAdded> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}
//#endregion 🔖️CollectionDiff

//#region 🔖️CollectionMutation
/// @emoji 🏷️ Identifies an item within a `Vec` by a stable id, for generic collection operations.
pub trait Identified<TId> {
    // 🚫️async: E1 pure accessor — every real caller is a std `Iterator`/`Vec` closure
    // (`retain`/`position`/`find`), `FnMut(&T) -> bool` signature fixed outside this repo and
    // cannot be async — see R9, R10 residue shape #1. Two of the three known implementors
    // (🌊️flow/🌿️vcs, ♾️infinite/…/dag) already converged on sync independently.
    fn id(&self) -> &TId;
}

/// @emoji 🩹️ Applies a patch in place and returns the patch that undoes it (captured from prior state).
pub trait Patchable<TPatch>: Sized {
    fn apply_patch(&mut self, patch: &TPatch);
    fn diff_patch(&self, other: &Self) -> Option<TPatch>;
}

/// @emoji 🧺️ Generic ordered-collection operation (add/remove/move/patch) with mechanical pre-state inverses.
///
/// 🎞️ `crate::os_spr::command` re-exports this very type, so `index`/`to_index` is the one wire shape
/// every caller sees — there is no second spr-side schema to keep in step.
///
/// 🗣️ Semantic-mutations overhaul ruling (`.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`):
/// this type and its three helper fns below are an INTERNAL diff/inverse ENGINE for a
/// `🧬️mutations/<kind>/{🔺️diff,↩️inverse}` triad leaf to call — e.g. a `remove-stakeholder` leaf's
/// `inverse` fn may call [`inverse_collection_mutation`] to compute the captured-item re-add. They
/// are NOT public mutation vocabulary: no `pub enum *Mutation` dispatch variant may wrap
/// `CollectionMutation<..>` directly (that erases the verb — `Add`/`Remove`/`Move`/`Patch` say
/// nothing about *why*). `policySemanticVocabularyBreaches` in `📜️script.ts` enforces this on
/// `✏️s/**/🧬️mutations/**` dispatch enums once the fan-out wave lands.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CollectionMutation<TId, TItem, TPatch> {
    Add { index: usize, item: TItem },
    Remove { id: TId },
    Move { id: TId, to_index: usize },
    Patch { id: TId, patch: TPatch },
}

/// @emoji ▶️ Applies a `CollectionMutation` to a `Vec` in place.
pub fn apply_collection_mutation<TId, TItem, TPatch>(items: &mut Vec<TItem>, operation: &CollectionMutation<TId, TItem, TPatch>)
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionMutation::Add { index, item } => {
            let at = (*index).min(items.len());
            items.insert(at, item.clone());
        }
        CollectionMutation::Remove { id } => {
            items.retain(|item| item.id() != id);
        }
        CollectionMutation::Move { id, to_index } => {
            if let Some(from) = items.iter().position(|item| item.id() == id) {
                let item = items.remove(from);
                let at = (*to_index).min(items.len());
                items.insert(at, item);
            }
        }
        CollectionMutation::Patch { id, patch } => {
            if let Some(item) = items.iter_mut().find(|item| item.id() == id) {
                item.apply_patch(patch);
            }
        }
    }
}

/// @emoji ↩️ Computes the inverse `CollectionMutation` from the pre-state `items`. Panics if `operation` targets
/// an id absent from `items` (Remove/Move/Patch always target an existing item by construction).
pub fn inverse_collection_mutation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionMutation<TId, TItem, TPatch>) -> CollectionMutation<TId, TItem, TPatch>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionMutation::Add { item, .. } => CollectionMutation::Remove { id: item.id().clone() },
        CollectionMutation::Remove { id } => {
            let index = items.iter().position(|item| item.id() == id).expect("remove target must exist in pre-state");
            CollectionMutation::Add { index, item: items[index].clone() }
        }
        CollectionMutation::Move { id, .. } => {
            let index = items.iter().position(|item| item.id() == id).expect("move target must exist in pre-state");
            CollectionMutation::Move { id: id.clone(), to_index: index }
        }
        CollectionMutation::Patch { id, patch } => {
            let prior = items.iter().find(|item| item.id() == id).cloned().expect("patch target must exist in pre-state");
            let mut after = prior.clone();
            after.apply_patch(patch);
            let inverse_patch = after.diff_patch(&prior).expect("a patch that changed state must yield a computable inverse");
            CollectionMutation::Patch { id: id.clone(), patch: inverse_patch }
        }
    }
}

/// @emoji 🧮️ Projects a `CollectionMutation` onto a sparse {@link CollectionDiff}, so a plugin's
/// `Mutation::diff` can produce a diff in one call instead of hand-writing `removed`/`modified`/
/// `added`. `Add` → `added`, `Remove` → `removed`, `Patch` → `modified`. `CollectionDiff` has no
/// positional-move channel, so `Move` is encoded as `removed` + `added` (delete then re-add by
/// identity); a plugin that keeps items keyed by id reconstructs order from item identity.
pub fn collection_diff_from_mutation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionMutation<TId, TItem, TPatch>) -> CollectionDiff<TId, TPatch, TItem>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone,
    TPatch: Clone,
{
    let mut diff = CollectionDiff::default();
    match operation {
        CollectionMutation::Add { item, .. } => diff.added.push(item.clone()),
        CollectionMutation::Remove { id } => diff.removed.push(id.clone()),
        CollectionMutation::Patch { id, patch } => diff.modified.push(ItemPatch { id: id.clone(), patch: patch.clone() }),
        CollectionMutation::Move { id, .. } => {
            if let Some(item) = items.iter().find(|item| item.id() == id) {
                diff.removed.push(id.clone());
                diff.added.push(item.clone());
            }
        }
    }
    diff
}
//#endregion 🔖️CollectionMutation
//#region 🔖️Mutation
// 🎞️ `Mutation`/`MutationDiff`/`MutationMessage` live in `protocol_command`; this region just
// replays a snapshot through an operation's forward diff — the pure per-step transform every
// store-level replay uses.

/// @emoji ▶️ Computes `operation.diff(snapshot)`, applies the resulting diff, and returns the new
/// snapshot alongside every [`crate::os_spr::MutationMessage`] the outcome carried. Diff-apply
/// rejection is returned as its structured [`MutationApplyError`] before a snapshot is produced. A `Fatal`
/// message's diff is `D::default()` by construction (§C2 LAW 1), so applying it is always a no-op —
/// callers that must not silently apply a rejected op check `worst_level(&messages)` against their
/// `MergePolicy` themselves (this fn stays policy-agnostic, matching its old unconditional-apply
/// shape).
pub fn apply_mutation<P, Mutation>(snapshot: &P, operation: &Mutation) -> Result<(P, Vec<crate::os_spr::MutationMessage>), MutationApplyError>
where
    Mutation: self::Mutation<P>,
{
    let (diff, messages) = operation.diff(snapshot).into_parts();
    Ok((diff.apply(snapshot)?, messages))
}

//#endregion 🔖️Mutation
//#region 🔖️MergeStrategy
// 🎞️ The CRDT-era concurrent-diff merge helper this region used to point at is deleted
// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`) — concurrent-merge
// arbitration is now an authority's `MergePolicy`/`📡️spr/⚔️conflict` job. The checkpoint-ancestor/
// merge-base helpers that used to live in this region moved to `store` along with `ArtifactEnvelope`
// (`checkpoint_ancestors`/`merge_base`/`reconcile_alternative` all take an envelope) — only the
// envelope-free id-minting primitive stays here.

/// @emoji 🔒️ Content-addressed checkpoint id: `ck-<hex16(blake3(parent_id || ordered_change_content_
/// hashes || message || authors || timestamp [|| ordered_pin_content]))>`, replacing the old fully-
/// random counter-string scheme (`create_document_vcs_id("checkpoint")`) — two peers that
/// independently commit the identical checkpoint content (same parent, same changes in the same
/// order, same message/authors/timestamp, same composition pins) now converge on the identical id
/// instead of minting two different ones. `changes` must already contain every entry `change_ids`
/// references (including one freshly created by this same commit, if any) — callers push a new
/// `Change` before calling this.
///
/// 🎯️ `pins` extension (composition-aware checkpoints): appended to the hash input ONLY when
/// non-empty, so a non-composite checkpoint (the overwhelming majority, and every checkpoint ever
/// minted before this ticket) hashes to EXACTLY the pre-existing bytes — this is what keeps old ids
/// stable, not a version bump. `pins` is re-sorted by `child_ref.to_uri()` (see [`CompositionPin`])
/// inside this function rather than trusted in caller-supplied order: a caller
/// building the pin list from a `HashMap`/parallel-dispatch fan-out over owned children has no
/// natural deterministic order of its own, and two peers committing the identical pin SET must
/// still converge on the identical id regardless of which order their local dispatch happened to
/// discover the children in.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PendingChangeRef<'a> {
    id: &'a str,
    edit_ids: &'a [String],
    description: Option<&'a str>,
    saved_at: &'a str,
}

fn content_addressed_checkpoint_id_core(
    parent_id: Option<&str>,
    change_ids: &[String],
    changes: &ArtifactHistoryLedger<Change>,
    pending: Option<PendingChangeRef<'_>>,
    message: Option<&str>,
    authors: &[Author],
    timestamp: &str,
    pins: &[CompositionPin],
) -> String {
    let mut input = Vec::new();
    input.extend_from_slice(parent_id.unwrap_or("").as_bytes());
    input.push(0);
    for change_id in change_ids {
        let change_hash = if let Some(change) = changes.iter().find(|change| change.id == *change_id) {
            *semio_framework_hash::hash(&serde_json::to_vec(change).unwrap_or_default()).as_bytes()
        } else if let Some(change) = pending.as_ref().filter(|change| change.id == change_id.as_str()) {
            *semio_framework_hash::hash(&serde_json::to_vec(change).unwrap_or_default()).as_bytes()
        } else {
            [0u8; 32]
        };
        input.extend_from_slice(&change_hash);
    }
    input.push(0);
    input.extend_from_slice(message.unwrap_or("").as_bytes());
    input.push(0);
    for author in authors {
        input.extend_from_slice(author.id.as_bytes());
        input.push(0);
    }
    input.push(0);
    input.extend_from_slice(timestamp.as_bytes());
    if !pins.is_empty() {
        // 🪡️ `to_uri` (🚪️io, out of this packet's scope) is async — `Iterator::map`'s closure is
        // sync (E0728), so the await is hoisted into a plain loop before the sort (R10 residue #1).
        let mut ordered: Vec<(String, &CompositionPin)> = Vec::with_capacity(pins.len());
        for pin in pins {
            ordered.push((pin.child_ref.to_uri(), pin));
        }
        ordered.sort_by(|(a, _), (b, _)| a.cmp(b));
        input.push(0);
        for (uri, pin) in ordered {
            input.extend_from_slice(uri.as_bytes());
            input.push(0);
            input.extend_from_slice(pin.checkpoint_id.as_bytes());
            input.push(0);
        }
    }
    let digest = *semio_framework_hash::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("ck-{hex16}")
}

pub async fn content_addressed_checkpoint_id(parent_id: Option<&str>, change_ids: &[String], changes: &ArtifactHistoryLedger<Change>, message: Option<&str>, authors: &[Author], timestamp: &str, pins: &[CompositionPin]) -> String {
    content_addressed_checkpoint_id_core(parent_id, change_ids, changes, None, message, authors, timestamp, pins)
}

pub fn content_addressed_checkpoint_id_with_pending_change(
    parent_id: Option<&str>,
    change_ids: &[String],
    changes: &ArtifactHistoryLedger<Change>,
    pending_change_id: &str,
    pending_edit_ids: &[String],
    pending_description: Option<&str>,
    pending_saved_at: &str,
    message: Option<&str>,
    authors: &[Author],
    timestamp: &str,
    pins: &[CompositionPin],
) -> String {
    content_addressed_checkpoint_id_core(parent_id, change_ids, changes, Some(PendingChangeRef { id: pending_change_id, edit_ids: pending_edit_ids, description: pending_description, saved_at: pending_saved_at }), message, authors, timestamp, pins)
}
//#endregion 🔖️MergeStrategy

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct DemoItem {
        id: String,
        value: i32,
    }

    impl Identified<String> for DemoItem {
        fn id(&self) -> &String {
            &self.id
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoItemPatch {
        value: Option<i32>,
    }

    impl Patchable<DemoItemPatch> for DemoItem {
        fn apply_patch(&mut self, patch: &DemoItemPatch) {
            if let Some(value) = patch.value {
                self.value = value;
            }
        }

        fn diff_patch(&self, other: &Self) -> Option<DemoItemPatch> {
            (self.value != other.value).then_some(DemoItemPatch { value: Some(other.value) })
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_diff_from_op_projects_each_variant() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }];
        let added = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Add { index: 0, item: DemoItem { id: "c".into(), value: 3 } });
        assert_eq!(added.added.len(), 1);
        assert!(added.removed.is_empty() && added.modified.is_empty());

        let removed = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Remove { id: "a".into() });
        assert_eq!(removed.removed, vec!["a".to_string()]);

        let patched = collection_diff_from_mutation(&items, &CollectionMutation::Patch { id: "b".into(), patch: DemoItemPatch { value: Some(9) } });
        assert_eq!(patched.modified.len(), 1);
        assert_eq!(patched.modified[0].id, "b");

        let moved = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Move { id: "a".into(), to_index: 1 });
        assert_eq!(moved.removed, vec!["a".to_string()], "move is encoded as remove + re-add by identity");
        assert_eq!(moved.added.len(), 1);
        assert_eq!(moved.added[0].id, "a");
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_op_add_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let operation = CollectionMutation::Add { index: 1, item: DemoItem { id: "b".into(), value: 2 } };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation);
        assert_eq!(applied.len(), 2);
        assert_eq!(applied[1].id, "b");
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_op_move_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }, DemoItem { id: "c".into(), value: 3 }];
        let operation = CollectionMutation::Move { id: "a".into(), to_index: 2 };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation);
        assert_eq!(applied.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c", "a"]);
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_op_patch_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let operation = CollectionMutation::Patch { id: "a".into(), patch: DemoItemPatch { value: Some(9) } };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation);
        assert_eq!(applied[0].value, 9);
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[semio_framework_async_macros::async_test]
    async fn collection_op_remove_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }];
        let operation = CollectionMutation::Remove { id: "a".into() };
        let mut applied = items.clone();
        apply_collection_mutation(&mut applied, &operation);
        assert_eq!(applied.len(), 1);
        let inverse = inverse_collection_mutation(&items, &operation);
        apply_collection_mutation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    //#endregion 🔖️ReconcileAlternative

    //#region 🔖️ContentAddressedCheckpointAndMergeBase
    #[semio_framework_async_macros::async_test]
    async fn fixed_history_ledger_preserves_order_capacity_and_aba_rejection() {
        let mut ledger = ArtifactHistoryLedger::new();
        let mut keys = Vec::with_capacity(ARTIFACT_HISTORY_LEDGER_CAPACITY);
        for index in 0..ARTIFACT_HISTORY_LEDGER_CAPACITY {
            keys.push(ledger.try_push(format!("history-{index:02}")).expect("fixed ledger admits its exact capacity"));
        }
        let rejected = ledger.try_push("history-overflow".to_string()).expect_err("capacity + 1 returns the exact rejected owner");
        assert_eq!(rejected, "history-overflow");
        assert_eq!(ledger.iter().next().map(String::as_str), Some("history-00"));
        assert_eq!(ledger.iter().next_back().map(String::as_str), Some("history-63"));

        let removed = ledger.remove_key(keys[17]).expect("live generation removes its exact owner");
        assert_eq!(removed, "history-17");
        assert_eq!(ledger.remove_key(keys[17]), Err(keys[17]), "a stale generation cannot remove the reused slot");
        let replacement = ledger.try_push("history-replacement".to_string()).expect("one tombstone admits one replacement");
        assert_eq!(replacement.index, keys[17].index);
        assert!(replacement.generation > keys[17].generation);
        assert_eq!(ledger.last().map(String::as_str), Some("history-replacement"));

        let mut drained = 0;
        while ledger.pop().is_some() {
            drained += 1;
        }
        assert_eq!(drained, ARTIFACT_HISTORY_LEDGER_CAPACITY);
        assert!(ledger.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn fixed_history_reservation_returns_exact_rejected_owner_and_blocks_aba() {
        let mut first = ArtifactHistoryLedger::new();
        let mut second = ArtifactHistoryLedger::new();
        let reservation = first.reserve_one().expect("empty fixed ledger reserves one exact slot");
        let rejected = first.try_push("parallel-owner".to_string()).expect_err("an outstanding reservation excludes parallel adoption");
        assert_eq!(rejected, "parallel-owner");
        let (reservation, rejected) = second.insert_reserved(reservation, "wrong-ledger-owner".to_string()).expect_err("a reservation cannot cross ledger authority");
        assert_eq!(rejected, "wrong-ledger-owner");
        first.cancel_reservation(reservation).expect("the exact unconsumed token returns to its issuing ledger");

        let reservation = first.reserve_one().expect("cancelled reservation releases capacity");
        let key = first.insert_reserved(reservation, "committed-owner".to_string()).expect("matching token adopts exactly once");
        assert_eq!(first.remove_key(key).expect("live key returns its exact owner"), "committed-owner");
        assert!(first.terminal_is_empty());
        assert!(second.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn content_addressed_checkpoint_id_is_deterministic_and_content_sensitive() {
        let root_change = Change { id: "change-root".into(), edit_ids: vec!["edit-1".into()], description: Some("root".into()), saved_at: "2026-07-27T00:00:00Z".into() };
        let mut changes = ArtifactHistoryLedger::try_from_preflighted(vec![root_change]).expect("one change fits the fixed ledger");
        let change_ids = vec!["change-root".to_string()];
        let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];

        let id_a = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z", &[]).await;
        let id_b = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z", &[]).await;
        assert_eq!(id_a, id_b, "identical inputs converge on the identical id");
        assert!(id_a.starts_with("ck-"), "got {id_a}");

        let id_different_message = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("other message"), &authors, "2026-07-27T00:00:01Z", &[]).await;
        assert_ne!(id_a, id_different_message, "a different message must change the id");

        let id_different_parent = content_addressed_checkpoint_id(Some("ck-parent"), &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z", &[]).await;
        assert_ne!(id_a, id_different_parent, "a different parent must change the id");

        let id_different_timestamp = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:02Z", &[]).await;
        assert_ne!(id_a, id_different_timestamp, "a different timestamp must change the id");
        drop(changes.pop());
    }

    #[semio_framework_async_macros::async_test]
    async fn pending_change_checkpoint_hash_is_byte_identical_before_history_reservation() {
        let mut changes = ArtifactHistoryLedger::new();
        let change = Change { id: "change-pending".into(), edit_ids: vec!["edit-a".into(), "edit-b".into()], description: Some("pending".into()), saved_at: "2026-08-23T00:00:00Z".into() };
        let change_ids = vec![change.id.clone()];
        let authors = vec![Author { id: "actor".into(), name: "Actor".into(), avatar: None }];
        let before_reservation =
            content_addressed_checkpoint_id_with_pending_change(None, &change_ids, &changes, &change.id, &change.edit_ids, change.description.as_deref(), &change.saved_at, Some("checkpoint"), &authors, "2026-08-23T00:00:01Z", &[]);
        let reservation = changes.reserve_one().expect("hashing does not consume the fixed ledger reservation");
        changes.insert_reserved(reservation, change).expect("exact reservation adopts the pending change");
        let after_commit = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("checkpoint"), &authors, "2026-08-23T00:00:01Z", &[]).await;
        assert_eq!(before_reservation, after_commit, "borrowing the pending change before reservation preserves the exact wire hash");
        drop(changes.pop());
    }

    /// @emoji 🧩️ `composition_pins`/`CompositionPin` extension to `content_addressed_checkpoint_id`:
    /// the three properties the ticket calls for — pin-set changes flip the id, identical
    /// pins-in-identical-order converge, and (critically) an EMPTY pin list must hash to the exact
    /// same bytes `content_addressed_checkpoint_id` produced before this field existed, so every
    /// checkpoint id ever minted for a non-composite artifact stays valid.
    #[semio_framework_async_macros::async_test]
    async fn content_addressed_checkpoint_id_composition_pins_are_deterministic_and_backward_compatible() {
        let root_change = Change { id: "change-root".into(), edit_ids: vec!["edit-1".into()], description: Some("root".into()), saved_at: "2026-07-27T00:00:00Z".into() };
        let mut changes = ArtifactHistoryLedger::try_from_preflighted(vec![root_change]).expect("one change fits the fixed ledger");
        let change_ids = vec!["change-root".to_string()];
        let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];
        let args = (None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z");

        // (1) Empty pins must reproduce the pre-pins hash bytes EXACTLY — recomputed here via the
        // same blake3(parent||changes||message||authors||timestamp) formula
        // `content_addressed_checkpoint_id` used before the `pins` parameter was added, so this is
        // a byte-level backward-compatibility proof, not just "doesn't panic".
        let mut legacy_input = Vec::new();
        legacy_input.extend_from_slice(args.0.unwrap_or("").as_bytes());
        legacy_input.push(0);
        for change_id in args.1 {
            let change_hash = args.2.iter().find(|change| change.id == *change_id).map_or([0u8; 32], |change| *semio_framework_hash::hash(&serde_json::to_vec(change).unwrap_or_default()).as_bytes());
            legacy_input.extend_from_slice(&change_hash);
        }
        legacy_input.push(0);
        legacy_input.extend_from_slice(args.3.unwrap_or("").as_bytes());
        legacy_input.push(0);
        for author in args.4 {
            legacy_input.extend_from_slice(author.id.as_bytes());
            legacy_input.push(0);
        }
        legacy_input.push(0);
        legacy_input.extend_from_slice(args.5.as_bytes());
        let legacy_digest = *semio_framework_hash::hash(&legacy_input).as_bytes();
        let legacy_hex16: String = legacy_digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
        let legacy_id = format!("ck-{legacy_hex16}");
        let id_no_pins = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &[]).await;
        assert_eq!(id_no_pins, legacy_id, "an empty pin list must not change a single byte of the pre-existing hash input");

        // (2) A non-empty pin set changes the id relative to no pins at all.
        let child_a_ref = crate::os_io::ArtifactRef::parse_uri("child-a!s.stdio.mesh@87a/mesh").expect("valid test fixture uri");
        let child_b_ref = crate::os_io::ArtifactRef::parse_uri("child-b!s.stdio.image@87a/image").expect("valid test fixture uri");
        let pins_one = vec![CompositionPin { child_ref: child_a_ref.clone(), checkpoint_id: "ck-child-a-1".into() }];
        let id_with_pins = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_one).await;
        assert_ne!(id_no_pins, id_with_pins, "a non-empty pin list must change the id relative to no composition");

        // (3) Identical pins in identical order converge on the identical id.
        let id_with_pins_again = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_one).await;
        assert_eq!(id_with_pins, id_with_pins_again, "identical pins in identical order converge on the identical id");

        // (4) A different pin CONTENT (same child, different pinned checkpoint) changes the id.
        let pins_one_moved = vec![CompositionPin { child_ref: child_a_ref.clone(), checkpoint_id: "ck-child-a-2".into() }];
        let id_pin_moved = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_one_moved);
        assert_ne!(id_with_pins, id_pin_moved.await, "a different pinned checkpoint_id for the same child must change the id");

        // (5) Two peers that discover the same pin SET in different order (e.g. concurrent
        // parallel-child dispatch) still converge — the function sorts by `child_ref.to_uri()` internally.
        let pins_two_ordered = vec![CompositionPin { child_ref: child_a_ref.clone(), checkpoint_id: "ck-child-a-1".into() }, CompositionPin { child_ref: child_b_ref.clone(), checkpoint_id: "ck-child-b-1".into() }];
        let pins_two_reordered = vec![CompositionPin { child_ref: child_b_ref, checkpoint_id: "ck-child-b-1".into() }, CompositionPin { child_ref: child_a_ref, checkpoint_id: "ck-child-a-1".into() }];
        let id_ordered = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_two_ordered);
        let id_reordered = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_two_reordered);
        assert_eq!(id_ordered.await, id_reordered.await, "two peers discovering the same pin set in different incidental order must converge on the identical id");
        drop(changes.pop());
    }

    //#region 🆔️Ids
    #[semio_framework_async_macros::async_test]
    async fn content_addressed_entity_and_mint_helpers_are_deterministic() {
        assert_eq!(content_addressed_entity_id("x", b"payload").await, content_addressed_entity_id("x", b"payload").await);
        assert_ne!(content_addressed_entity_id("x", b"a").await, content_addressed_entity_id("x", b"b").await);
        assert_eq!(edit_scoped_id("edit-1", 0).await, edit_scoped_id("edit-1", 0).await);
        assert_ne!(edit_scoped_id("edit-1", 0).await, edit_scoped_id("edit-1", 1).await);
        assert!(edit_scoped_id("edit-1", 0).await.starts_with("scoped-"));
        assert_eq!(mint_edit_id(Some("alice"), 3, b"fwd").await, mint_edit_id(Some("alice"), 3, b"fwd").await);
        assert_ne!(mint_edit_id(Some("alice"), 3, b"fwd").await, mint_edit_id(Some("bob"), 3, b"fwd").await);
        assert_eq!(mint_change_id(&["e1".into(), "e2".into()], Some("msg")).await, mint_change_id(&["e1".into(), "e2".into()], Some("msg")).await);
        assert_eq!(mint_alternative_id("main", &["ck1".into()]).await, mint_alternative_id("main", &["ck1".into()]).await);
        assert_eq!(mint_mutation_id(b"op-bytes").await, mint_mutation_id(b"op-bytes").await);
        assert_eq!(create_document_vcs_id("draft").await, create_document_vcs_id("draft").await);
        assert!(create_document_vcs_id("draft").await.starts_with("draft-"));
    }
    //#endregion 🆔️Ids
}
//#endregion 🧪️Tests
