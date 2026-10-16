//! 🖼️ Kernel UI delta: dirty scopes and the per-turn UI patch owners with their transports.

use semio_framework_diagnostic::Fault;
use semio_framework_ui_contract::UiValueRetirementStep;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeSeq;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::sync::Mutex;

/// 🐢️ What part of the shell's rendered UI an action actually invalidates — lets `refresh-ui`
/// skip re-rendering/re-fetching sections nothing touched. Absent from JSON (older/unmodified plugins)
/// deserializes to `Full`, so any program that never sets this keeps today's whole-shell-refresh
/// behavior exactly. `None` means "nothing to re-render at all" (e.g. a pure telemetry/heartbeat action).
// 🐢️ `rename_all = "camelCase"` alone only renames the *variant* names (Full/None/Partial ->
// full/none/partial via `tag = "kind"`) — it does NOT cascade into a struct variant's own fields, which
// would otherwise serialize as snake_case (`window_bodies`) and silently desync from the TS
// `UiDirtyScope` type's camelCase `windowBodies`. `rename_all_fields` is the attribute that renames
// fields *within* variants; both are needed together.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[derive(semio_framework_value::RetireOwned)]
pub enum UiDirtyScope {
    #[default]
    Full,
    None,
    Partial {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        window_bodies: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        panel_bodies: Vec<String>,
        #[serde(default)]
        #[value(default)]
        utilities: bool,
        #[serde(default)]
        #[value(default)]
        tools: bool,
        #[serde(default)]
        #[value(default)]
        engagements: bool,
        #[serde(default)]
        #[value(default)]
        measures: bool,
        #[serde(default)]
        #[value(default)]
        labels: bool,
    },
}

/// 🔖️ One flag-addressed section of a batched `refresh-ui` — the boolean fields of
/// [`UiDirtyScope::Partial`] named by value so ONE predicate serves all of them, on both renderers.
/// Windows and panels are addressed by body key instead ([`UiDirtyScope::wants_window_body`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UiDirtySection {
    Utilities,
    Tools,
    Engagements,
    Measures,
    Labels,
}

/// 🐢️ The selection law both shells answer to, and the union one coalesced refresh pass owes.
///
/// The React shell has always read a scope (`🛠️ShellHelpers/🟦️.tsx`'s `uiRefreshWants*` +
/// `buildUiRefreshRequest`); the wgpu shell's `refresh_ui` walked EVERY window and panel on every
/// settle and threw `InvocationResult::ui_scope` away — 116 of 137 renders per converging edit
/// answered `patched=0`, and the flow window was re-minted eight times for nothing
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-edit-convergence-perf-2026-09-14.md` §7).
/// These predicates are that shell's half of the same law, driven by the same fixture
/// (`🧫️fixtures/🐢️ui-dirty-scope/🔣️.json`) as the TypeScript twin in `🎠️kernel/🟦️.ts`.
impl UiDirtyScope {
    /// 🚫️ Nothing to re-render at all — the caller must not even open a refresh pass.
    pub fn asks_for_nothing(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn wants_window_body(&self, body_key: &str) -> bool {
        match self {
            Self::Full => true,
            Self::None => false,
            Self::Partial { window_bodies, .. } => window_bodies.iter().any(|body| body == body_key),
        }
    }

    pub fn wants_panel_body(&self, body_key: &str) -> bool {
        match self {
            Self::Full => true,
            Self::None => false,
            Self::Partial { panel_bodies, .. } => panel_bodies.iter().any(|body| body == body_key),
        }
    }

    pub fn wants_section(&self, section: UiDirtySection) -> bool {
        match self {
            Self::Full => true,
            Self::None => false,
            Self::Partial { utilities, tools, engagements, measures, labels, .. } => match section {
                UiDirtySection::Utilities => *utilities,
                UiDirtySection::Tools => *tools,
                UiDirtySection::Engagements => *engagements,
                UiDirtySection::Measures => *measures,
                UiDirtySection::Labels => *labels,
            },
        }
    }

    /// 🛍️ The app-static operator/palette catalogue never goes stale inside an app instance, so it
    /// carries no flag of its own: only a full scope — a session switch, or the first fetch — asks
    /// for it. Mirrors `uiRefreshWantsCatalogue`.
    pub fn wants_catalogue(&self) -> bool {
        matches!(self, Self::Full)
    }

    /// 🤝️ The scope ONE pass must cover when a second was asked for while the first was crossing
    /// into the guest — the union, never the newer alone. Twin of `mergeUiDirtyScopeV1`.
    pub fn merged_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Full, _) | (_, Self::Full) => Self::Full,
            (Self::None, scope) | (scope, Self::None) => scope,
            (
                Self::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels },
                Self::Partial {
                    window_bodies: other_window_bodies,
                    panel_bodies: other_panel_bodies,
                    utilities: other_utilities,
                    tools: other_tools,
                    engagements: other_engagements,
                    measures: other_measures,
                    labels: other_labels,
                },
            ) => Self::Partial {
                window_bodies: union_body_keys(window_bodies, other_window_bodies),
                panel_bodies: union_body_keys(panel_bodies, other_panel_bodies),
                utilities: utilities || other_utilities,
                tools: tools || other_tools,
                engagements: engagements || other_engagements,
                measures: measures || other_measures,
                labels: labels || other_labels,
            },
        }
    }
}

/// 🤝️ First-seen order, no duplicates — the `[...new Set([...first, ...second])]` the TypeScript
/// twin builds, spelled without a hash set so the two orders cannot drift.
fn union_body_keys(first: Vec<String>, second: Vec<String>) -> Vec<String> {
    let mut union = first;
    for key in second {
        if !union.contains(&key) {
            union.push(key);
        }
    }
    union
}

//#region 🔖️UiPatch

/// 🩹️ `wit-flip` (26/08/20): re-exported from `semio-framework-ui-contract`, the language-neutral
/// contract crate's own `UiPatch`/`UiPatchOp` (`🦀️document.rs`) — this file no longer declares its
/// own copy, so there is exactly one definition to keep in sync with `component.wit`'s
/// `ui-patch`/`patch-op` (node-id addressed, not path addressed) rather than two that could drift.
/// Requires a `semio-framework-ui-contract` dependency on every crate that `#[path]`-mounts this
/// file — see this packet's report for the exact registrar-request lines (this crate is not on the
/// registrar-only list for `Cargo.toml`, so the dependency itself is not added here).
pub use semio_framework_ui_contract::{PresenceUpdate, UiPatch, UiPatchOp};

//#endregion 🔖️UiPatch
//#region 🔖️TurnResult
/// 🏁️ Result of one `reactor::poll` call — `📜️wit/📜️reactor.wit`'s `turn-result` record.
///
/// 📏️ Inline extent of ONE exact patch owner — the block every arena slot that parks a publication
/// reserves by value, and the number [`UI_TURN_PATCHES_MAXIMUM`] is read off.
pub const UI_TURN_PATCH_OWNER_BYTES: usize = 4_096;

/// 📏️ How many surfaces one turn result may carry. This is the fixed CAPACITY of [`UiTurnPatches`],
/// not the admission rule — admission is the per-turn BYTE budget (`Budget::max_patch_bytes`, bounded
/// by [`UI_TURN_PATCH_BUDGET_BYTES`]), so a turn carries every patch that is ready and fits.
///
/// 🐛️ Until 2026-09-15 this was `1`, and the wire contract "one patch per crossing" made N published
/// surfaces cost N + 1 host round trips by construction — the floor
/// `📓️reactor-reconcile-spin-2026-09-14.md` §7 named and could not move. It is DERIVED, never chosen:
/// the page is an inline exact owner parked by value in every transport slot and every handback, so
/// the guest's declared contiguous-request ceiling is what one such block may cost, at
/// [`UI_TURN_PATCH_OWNER_BYTES`] per publication.
pub const UI_TURN_PATCHES_MAXIMUM: usize = semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / UI_TURN_PATCH_OWNER_BYTES;

/// 📏️ Ceiling every lane's `Budget::max_patch_bytes` declaration must stay under — a turn result is
/// an ASSEMBLED answer crossing the boundary, and a single assembled answer may claim a quarter of
/// what the declared budget admits (`🧮️memory/🦀️.rs`'s `GUEST_HOST_ANSWER_CEILING_BYTES`). The floor
/// is the contiguous ceiling itself: a lane that declared less than one contiguous page could not
/// carry one document-scaled publication at all.
pub const UI_TURN_PATCH_BUDGET_BYTES: usize = semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES / 4;

/// 📏️ Fixed wire envelope of one patch, and of one of its operations — the same two numbers the
/// native host's own `patch_wire_bytes` charges (`🔌️plugin/🖥️host/📥️ui-patch/🦀️.rs`), so the guest
/// that admits a batch and the host that refuses one price the envelope identically.
pub const UI_TURN_PATCH_WIRE_ENVELOPE_BYTES: usize = 32;

pub const UI_TURN_PATCH_OP_WIRE_ENVELOPE_BYTES: usize = 16;

/// 📐️ What ONE patch costs the turn it joins: its wire envelope, its surface name, one operation
/// envelope per operation, and the physical backing its operation storage actually holds. The last
/// term is what makes this an UPPER bound of the host's wire measure — an operation's pack payload is
/// never larger than the inline `UiPatchOp` that produced it — so a batch the guest admits can never
/// be a batch the host refuses.
pub fn ui_patch_turn_bytes(patch: &UiPatch) -> usize {
    UI_TURN_PATCH_WIRE_ENVELOPE_BYTES
        .saturating_add(patch.surface.as_ref().len())
        .saturating_add(patch.ops.len().saturating_mul(UI_TURN_PATCH_OP_WIRE_ENVELOPE_BYTES))
        .saturating_add(patch.ops.allocated_bytes())
}

pub const UI_TURN_PATCH_RETIRE_SLOTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UiTurnPatchRetireKey {
    slot: usize,
    epoch: u64,
}

#[derive(Debug, Default)]
struct UiTurnPatchRetireSlot {
    epoch: u64,
    reserved: bool,
    contents: Option<UiTurnPatchContents>,
}

#[derive(Debug, Default)]
struct UiTurnPatchContents {
    pending: semio_framework_ui_contract::UiPendingPatch,
}

impl UiTurnPatchContents {
    fn terminal_is_empty(&self) -> bool {
        self.pending.terminal_is_empty()
    }

    fn close_step(&mut self, items: usize, bytes: usize) -> Result<semio_framework_ui_contract::UiValueRetirementStep, &'static str> {
        use semio_framework_ui_contract::UiValueRetirementStep;
        if items == 0 || bytes == 0 {
            return Ok(UiValueRetirementStep::default());
        }
        if !self.pending.terminal_is_empty() {
            return self.pending.close_step(1, bytes);
        }
        Ok(UiValueRetirementStep { complete: true, ..Default::default() })
    }
}

struct UiTurnPatchHandback {
    ready: std::sync::atomic::AtomicBool,
    owner: std::cell::UnsafeCell<std::mem::MaybeUninit<(UiTurnPatchRetireKey, UiTurnPatchContents)>>,
}

/// 🔒️ A reserved key has one non-cloneable producer; its slot cannot be reused before the sole arena consumer retires the returned owner.
unsafe impl Sync for UiTurnPatchHandback {}

impl UiTurnPatchHandback {
    const fn new() -> Self {
        Self { ready: std::sync::atomic::AtomicBool::new(false), owner: std::cell::UnsafeCell::new(std::mem::MaybeUninit::uninit()) }
    }

    fn publish(&self, key: UiTurnPatchRetireKey, contents: UiTurnPatchContents) {
        unsafe {
            (*self.owner.get()).write((key, contents));
        }
        self.ready.store(true, std::sync::atomic::Ordering::Release);
    }

    fn take(&self) -> Option<(UiTurnPatchRetireKey, UiTurnPatchContents)> {
        if !self.ready.load(std::sync::atomic::Ordering::Acquire) {
            return None;
        }
        let owner = unsafe { (*self.owner.get()).assume_init_read() };
        self.ready.store(false, std::sync::atomic::Ordering::Release);
        Some(owner)
    }
}

const _: () = assert!(size_of::<(UiTurnPatchRetireKey, UiTurnPatchContents)>() <= UI_TURN_PATCH_OWNER_BYTES);

static UI_TURN_PATCH_HANDBACKS: [UiTurnPatchHandback; UI_TURN_PATCH_RETIRE_SLOTS] = [const { UiTurnPatchHandback::new() }; UI_TURN_PATCH_RETIRE_SLOTS];

static UI_TURN_PATCH_RETIRE_ARENA: std::sync::Mutex<UiTurnPatchRetireArena> =
    std::sync::Mutex::new(UiTurnPatchRetireArena { slots: [const { UiTurnPatchRetireSlot { epoch: 0, reserved: false, contents: None } }; UI_TURN_PATCH_RETIRE_SLOTS], next_epoch: 1, epoch_exhausted: false, close_cursor: 0 });

#[derive(Debug)]
struct UiTurnPatchRetireArena {
    slots: [UiTurnPatchRetireSlot; UI_TURN_PATCH_RETIRE_SLOTS],
    next_epoch: u64,
    epoch_exhausted: bool,
    close_cursor: usize,
}

impl Default for UiTurnPatchRetireArena {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| UiTurnPatchRetireSlot::default()), next_epoch: 1, epoch_exhausted: false, close_cursor: 0 }
    }
}

impl UiTurnPatchRetireArena {
    fn reserve(&mut self) -> Option<UiTurnPatchRetireKey> {
        if self.epoch_exhausted {
            return None;
        }
        let slot = self.slots.iter().position(|slot| !slot.reserved)?;
        let epoch = self.next_epoch;
        let target = &mut self.slots[slot];
        target.epoch = epoch;
        target.reserved = true;
        match epoch.checked_add(1) {
            Some(next) => self.next_epoch = next,
            None => self.epoch_exhausted = true,
        }
        Some(UiTurnPatchRetireKey { slot, epoch })
    }

    fn release_empty(&mut self, key: UiTurnPatchRetireKey) -> bool {
        let Some(slot) = self.slots.get_mut(key.slot) else { return false };
        if !slot.reserved || slot.epoch != key.epoch || slot.contents.is_some() {
            return false;
        }
        slot.reserved = false;
        true
    }

    #[expect(clippy::result_large_err, reason = "A refused retirement handback returns the exact patch contents without allocation.")]
    fn handback(&mut self, key: UiTurnPatchRetireKey, contents: UiTurnPatchContents) -> Result<(), UiTurnPatchContents> {
        let Some(slot) = self.slots.get_mut(key.slot).filter(|slot| slot.reserved && slot.epoch == key.epoch && slot.contents.is_none()) else {
            return Err(contents);
        };
        slot.contents = Some(contents);
        Ok(())
    }

    fn close_one(&mut self, items: usize, bytes: usize) -> bool {
        for offset in 0..UI_TURN_PATCH_RETIRE_SLOTS {
            let Some(index) = self.close_cursor.checked_add(offset).map(|index| index % UI_TURN_PATCH_RETIRE_SLOTS) else { return false };
            let slot = &mut self.slots[index];
            if !slot.reserved || slot.contents.is_none() {
                continue;
            }
            let Some(next) = index.checked_add(1) else { return false };
            self.close_cursor = next % UI_TURN_PATCH_RETIRE_SLOTS;
            let contents = slot.contents.as_mut().expect("selected returned patch owner");
            if contents.close_step(items, bytes).expect("exact returned patch retirement").complete && contents.terminal_is_empty() {
                slot.contents = None;
                slot.reserved = false;
            }
            return true;
        }
        false
    }
}

#[cfg(test)]
fn with_ui_turn_patch_retire_arena<T>(f: impl FnOnce(&mut UiTurnPatchRetireArena) -> T) -> T {
    let mut arena = UI_TURN_PATCH_RETIRE_ARENA.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut arena)
}

pub fn close_ui_turn_patch_owner_one() -> bool {
    close_ui_turn_patch_owner_with_grant(1, 4096)
}

/// ♻️ One retirement unit of a returned turn patch, against the caller's grant.
///
/// The grant is what keeps a document-scaled publication off the round-trip ladder: the patch a
/// 180-object world-3d document swap returns here needs 1 091 retirement units, and the reactor
/// spends one turn — one host round trip — per unit it cannot finish (ticket 26/09/02 W-S2, measured
/// 2026-09-10: 24.3 s and 8 799 worker messages for one example switch).
pub fn close_ui_turn_patch_owner_with_grant(items: usize, bytes: usize) -> bool {
    if items == 0 || bytes == 0 {
        return false;
    }
    let mut arena = match UI_TURN_PATCH_RETIRE_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return false,
        Err(std::sync::TryLockError::Poisoned(_)) => return false,
    };
    for offset in 0..UI_TURN_PATCH_RETIRE_SLOTS {
        let index = (arena.close_cursor + offset) % UI_TURN_PATCH_RETIRE_SLOTS;
        if let Some((key, contents)) = UI_TURN_PATCH_HANDBACKS[index].take() {
            if let Err(contents) = arena.handback(key, contents) {
                UI_TURN_PATCH_HANDBACKS[index].publish(key, contents);
                return false;
            }
            arena.close_cursor = (index + 1) % UI_TURN_PATCH_RETIRE_SLOTS;
            return true;
        }
    }
    arena.close_one(items, bytes)
}

pub const UI_TURN_PATCH_TRANSPORT_SLOTS: usize = 64;

const UI_TURN_PATCH_TRANSPORT_TOKEN_BYTES: usize = 32;

const UI_TURN_PATCH_TRANSPORT_MAGIC: [u8; 8] = *b"semui005";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UiTurnPatchTransportKey {
    slot: usize,
    epoch: u64,
    session: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum UiTurnPatchTransportState {
    #[default]
    Vacant,
    Building,
    Published,
    CheckedOut,
    Closing,
}

#[derive(Debug, Default)]
struct UiTurnPatchTransportSlot {
    epoch: u64,
    session: u64,
    state: UiTurnPatchTransportState,
    owner: Option<UiTurnPatches>,
    external: bool,
}

#[derive(Debug)]
struct UiTurnPatchTransportArena {
    slots: [UiTurnPatchTransportSlot; UI_TURN_PATCH_TRANSPORT_SLOTS],
    close_cursor: usize,
}

impl Default for UiTurnPatchTransportArena {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| UiTurnPatchTransportSlot::default()), close_cursor: 0 }
    }
}

impl UiTurnPatchTransportArena {
    #[expect(clippy::result_large_err, reason = "Admission refusal preserves the exact patch owner without allocating.")]
    fn reserve(&mut self, session: u64, owner: UiTurnPatches) -> Result<UiTurnPatchTransportKey, UiTurnPatches> {
        let Some(slot) = self.slots.iter().position(|slot| slot.state == UiTurnPatchTransportState::Vacant) else { return Err(owner) };
        let Some(epoch) = self.slots[slot].epoch.checked_add(1) else { return Err(owner) };
        self.slots[slot] = UiTurnPatchTransportSlot { epoch, session, state: UiTurnPatchTransportState::Building, owner: Some(owner), external: true };
        Ok(UiTurnPatchTransportKey { slot, epoch, session })
    }

    fn slot_mut(&mut self, key: UiTurnPatchTransportKey) -> Option<&mut UiTurnPatchTransportSlot> {
        let slot = self.slots.get_mut(key.slot)?;
        (slot.epoch == key.epoch && slot.session == key.session && slot.state != UiTurnPatchTransportState::Vacant).then_some(slot)
    }

    fn close_one(&mut self, items: usize, bytes: usize) -> Result<UiTurnPatchTransportProgress, &'static str> {
        for offset in 0..UI_TURN_PATCH_TRANSPORT_SLOTS {
            let index = (self.close_cursor + offset) % UI_TURN_PATCH_TRANSPORT_SLOTS;
            if self.slots[index].state != UiTurnPatchTransportState::Closing {
                continue;
            }
            self.close_cursor = (index + 1) % UI_TURN_PATCH_TRANSPORT_SLOTS;
            if self.slots[index].external {
                return Ok(UiTurnPatchTransportProgress::Blocked);
            }
            let owner = self.slots[index].owner.as_mut().ok_or("closing turn patch transport lost its exact owner")?;
            let step = owner.close_step_with_grant(items.max(1), bytes.max(1))?;
            if step.complete {
                let epoch = self.slots[index].epoch;
                self.slots[index] = UiTurnPatchTransportSlot { epoch, ..UiTurnPatchTransportSlot::default() };
                return Ok(UiTurnPatchTransportProgress::Pending { released_items: step.released_items.max(1), released_bytes: step.released_bytes });
            }
            return Ok(if step.progressed { UiTurnPatchTransportProgress::Pending { released_items: step.released_items, released_bytes: step.released_bytes } } else { UiTurnPatchTransportProgress::Blocked });
        }
        Ok(UiTurnPatchTransportProgress::Idle)
    }

    #[cfg(test)]
    fn request_session_close(&mut self, session: u64) -> bool {
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.session == session && slot.state != UiTurnPatchTransportState::Vacant) else { return false };
        if slot.state != UiTurnPatchTransportState::CheckedOut {
            slot.state = UiTurnPatchTransportState::Closing;
        }
        true
    }
}

static UI_TURN_PATCH_TRANSPORT_ARENA: std::sync::Mutex<UiTurnPatchTransportArena> =
    std::sync::Mutex::new(UiTurnPatchTransportArena { slots: [const { UiTurnPatchTransportSlot { epoch: 0, session: 0, state: UiTurnPatchTransportState::Vacant, owner: None, external: false } }; UI_TURN_PATCH_TRANSPORT_SLOTS], close_cursor: 0 });

struct UiTurnPatchTransportHandback {
    ready: std::sync::atomic::AtomicBool,
    value: std::cell::UnsafeCell<std::mem::MaybeUninit<(UiTurnPatchTransportKey, Option<UiTurnPatches>)>>,
}

/// 🔒️ A reserved slot has exactly one producer or checked-out lease; its epoch cannot be reused before the sole arena consumer takes this handback.
unsafe impl Sync for UiTurnPatchTransportHandback {}

impl UiTurnPatchTransportHandback {
    const fn new() -> Self {
        Self { ready: std::sync::atomic::AtomicBool::new(false), value: std::cell::UnsafeCell::new(std::mem::MaybeUninit::uninit()) }
    }
    fn publish(&self, key: UiTurnPatchTransportKey, owner: Option<UiTurnPatches>) {
        unsafe {
            (*self.value.get()).write((key, owner));
        }
        self.ready.store(true, std::sync::atomic::Ordering::Release);
    }
    fn take(&self) -> Option<(UiTurnPatchTransportKey, Option<UiTurnPatches>)> {
        if !self.ready.load(std::sync::atomic::Ordering::Acquire) {
            return None;
        }
        let value = unsafe { (*self.value.get()).assume_init_read() };
        self.ready.store(false, std::sync::atomic::Ordering::Release);
        Some(value)
    }
}

const _: () = assert!(size_of::<(UiTurnPatchTransportKey, Option<UiTurnPatches>)>() <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);

static UI_TURN_PATCH_TRANSPORT_HANDBACKS: [UiTurnPatchTransportHandback; UI_TURN_PATCH_TRANSPORT_SLOTS] = [const { UiTurnPatchTransportHandback::new() }; UI_TURN_PATCH_TRANSPORT_SLOTS];

#[cfg(test)]
fn with_ui_turn_patch_transport_arena<T>(f: impl FnOnce(&mut UiTurnPatchTransportArena) -> T) -> T {
    let mut arena = UI_TURN_PATCH_TRANSPORT_ARENA.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut arena)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiTurnPatchTransportProgress {
    Idle,
    Blocked,
    Pending { released_items: usize, released_bytes: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiTurnPatchTransportStep {
    MoreWork,
    Blocked,
    Fault(&'static str),
    Ready,
    Cancelled,
    Stale,
}

pub struct UiTurnPatchTransportProducer {
    key: UiTurnPatchTransportKey,
    patch: usize,
    operation: usize,
    ready: bool,
    transferred: bool,
    closing: bool,
}

impl UiTurnPatchTransportProducer {
    #[expect(clippy::result_large_err, reason = "A contended or full transport arena returns the exact patch owner for retry.")]
    pub fn try_new(session: u64, owner: UiTurnPatches) -> Result<Self, UiTurnPatches> {
        let Ok(mut arena) = UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() else {
            return Err(owner);
        };
        let key = arena.reserve(session, owner)?;
        Ok(Self { key, patch: 0, operation: 0, ready: false, transferred: false, closing: false })
    }

    pub fn drive_one(&mut self, session: u64, cancelled: bool, deadline_expired: bool) -> UiTurnPatchTransportStep {
        if session != self.key.session || self.transferred {
            return UiTurnPatchTransportStep::Stale;
        }
        if cancelled || self.closing {
            if !self.closing {
                UI_TURN_PATCH_TRANSPORT_HANDBACKS[self.key.slot].publish(self.key, None);
                self.closing = true;
            }
            return UiTurnPatchTransportStep::Cancelled;
        }
        if deadline_expired {
            return UiTurnPatchTransportStep::MoreWork;
        }
        let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return UiTurnPatchTransportStep::Blocked,
            Err(std::sync::TryLockError::Poisoned(_)) => return UiTurnPatchTransportStep::Fault("turn patch transport arena is poisoned"),
        };
        if arena.slot_mut(self.key).is_some_and(|slot| slot.state == UiTurnPatchTransportState::Closing) {
            UI_TURN_PATCH_TRANSPORT_HANDBACKS[self.key.slot].publish(self.key, None);
            self.closing = true;
            return UiTurnPatchTransportStep::Cancelled;
        }
        let step = (|| {
            let slot = arena.slot_mut(self.key)?;
            if slot.state != UiTurnPatchTransportState::Building {
                return None;
            }
            let owner = slot.owner.as_ref()?;
            let Some(patch) = owner.iter().nth(self.patch) else {
                slot.state = UiTurnPatchTransportState::Published;
                return Some(UiTurnPatchTransportStep::Ready);
            };
            if self.operation < patch.ops.len() {
                self.operation = self.operation.checked_add(1)?;
                return Some(UiTurnPatchTransportStep::MoreWork);
            }
            self.patch = self.patch.checked_add(1)?;
            self.operation = 0;
            Some(UiTurnPatchTransportStep::MoreWork)
        })();
        let Some(step) = step else { return UiTurnPatchTransportStep::Stale };
        self.ready |= step == UiTurnPatchTransportStep::Ready;
        step
    }

    pub fn take_ready(&mut self) -> Result<Option<[u8; UI_TURN_PATCH_TRANSPORT_TOKEN_BYTES]>, &'static str> {
        if !self.ready || self.transferred || self.closing {
            return Ok(None);
        }
        let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(None),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch transport arena is poisoned"),
        };
        let slot = arena.slot_mut(self.key).filter(|slot| slot.state == UiTurnPatchTransportState::Published && slot.owner.is_some() && slot.external).ok_or("exact turn patch publication is no longer available")?;
        let mut token = [0u8; UI_TURN_PATCH_TRANSPORT_TOKEN_BYTES];
        token[..8].copy_from_slice(&UI_TURN_PATCH_TRANSPORT_MAGIC);
        token[8..16].copy_from_slice(&(self.key.slot as u64).to_le_bytes());
        token[16..24].copy_from_slice(&self.key.epoch.to_le_bytes());
        token[24..32].copy_from_slice(&self.key.session.to_le_bytes());
        slot.external = false;
        self.transferred = true;
        Ok(Some(token))
    }
}

impl Drop for UiTurnPatchTransportProducer {
    fn drop(&mut self) {
        if self.transferred || self.closing {
            return;
        }
        UI_TURN_PATCH_TRANSPORT_HANDBACKS[self.key.slot].publish(self.key, None);
        self.closing = true;
    }
}

pub struct UiTurnPatchTransportLease {
    key: UiTurnPatchTransportKey,
    owner: Option<UiTurnPatches>,
}

impl UiTurnPatchTransportLease {
    pub fn try_from_token(token: &[u8], expected_session: u64) -> Result<Self, &'static str> {
        if token.len() != UI_TURN_PATCH_TRANSPORT_TOKEN_BYTES || token[..8] != UI_TURN_PATCH_TRANSPORT_MAGIC {
            return Err("invalid turn patch transport token");
        }
        let slot = usize::try_from(u64::from_le_bytes(token[8..16].try_into().map_err(|_| "invalid turn patch slot")?)).map_err(|_| "invalid turn patch slot")?;
        let epoch = u64::from_le_bytes(token[16..24].try_into().map_err(|_| "invalid turn patch epoch")?);
        let session = u64::from_le_bytes(token[24..32].try_into().map_err(|_| "invalid turn patch session")?);
        if session != expected_session {
            return Err("stale turn patch session");
        }
        let key = UiTurnPatchTransportKey { slot, epoch, session };
        let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return Err("turn patch transport arena is busy"),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch transport arena is poisoned"),
        };
        let owner = (|| {
            let slot = arena.slot_mut(key)?;
            if slot.state != UiTurnPatchTransportState::Published || slot.external || slot.owner.is_none() {
                return None;
            }
            slot.state = UiTurnPatchTransportState::CheckedOut;
            slot.external = true;
            slot.owner.take()
        })()
        .ok_or("stale or duplicate turn patch token")?;
        Ok(Self { key, owner: Some(owner) })
    }

    #[expect(clippy::result_large_err, reason = "A refused transfer preserves the exact lease and its retirement authority for retry.")]
    pub fn take_owner(mut self) -> Result<UiTurnPatches, Self> {
        let Ok(mut arena) = UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() else {
            return Err(self);
        };
        let Some(slot) = arena.slot_mut(self.key).filter(|slot| slot.state == UiTurnPatchTransportState::CheckedOut && slot.external && slot.owner.is_none()) else {
            return Err(self);
        };
        let Some(owner) = self.owner.take() else { return Err(self) };
        let epoch = slot.epoch;
        *slot = UiTurnPatchTransportSlot { epoch, ..UiTurnPatchTransportSlot::default() };
        Ok(owner)
    }
}

impl Drop for UiTurnPatchTransportLease {
    fn drop(&mut self) {
        let Some(owner) = self.owner.take() else { return };
        UI_TURN_PATCH_TRANSPORT_HANDBACKS[self.key.slot].publish(self.key, Some(owner));
    }
}

pub fn close_ui_turn_patch_transport_one() -> Result<UiTurnPatchTransportProgress, &'static str> {
    close_ui_turn_patch_transport_with_grant(1, 4096)
}

/// ♻️ One retirement unit of a closing turn-patch transport, against the caller's PAGE grant.
///
/// The transport owns the patch the turn hands to the host, so a document-scaled publication parks a
/// document-scaled owner here; retiring it one item per unit is one host round trip per item, because
/// the reactor answers `MoreWork` for as long as anything is outstanding (ticket 26/09/02, W-S2 §7
/// left this ladder ungranted, W-B2 grants it).
pub fn close_ui_turn_patch_transport_with_grant(items: usize, bytes: usize) -> Result<UiTurnPatchTransportProgress, &'static str> {
    let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(UiTurnPatchTransportProgress::Blocked),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch transport arena is poisoned"),
    };
    for offset in 0..UI_TURN_PATCH_TRANSPORT_SLOTS {
        let index = (arena.close_cursor + offset) % UI_TURN_PATCH_TRANSPORT_SLOTS;
        let Some((key, owner)) = UI_TURN_PATCH_TRANSPORT_HANDBACKS[index].take() else {
            continue;
        };
        let Some(slot) = arena.slot_mut(key).filter(|slot| {
            slot.external
                && if owner.is_some() {
                    slot.state == UiTurnPatchTransportState::CheckedOut && slot.owner.is_none()
                } else {
                    matches!(slot.state, UiTurnPatchTransportState::Building | UiTurnPatchTransportState::Published | UiTurnPatchTransportState::Closing) && slot.owner.is_some()
                }
        }) else {
            UI_TURN_PATCH_TRANSPORT_HANDBACKS[index].publish(key, owner);
            return Err("exact turn patch handback does not match its reserved slot");
        };
        if owner.is_some() {
            slot.owner = owner;
        }
        slot.external = false;
        slot.state = UiTurnPatchTransportState::Closing;
        arena.close_cursor = (index + 1) % UI_TURN_PATCH_TRANSPORT_SLOTS;
        return Ok(UiTurnPatchTransportProgress::Pending { released_items: 0, released_bytes: 0 });
    }
    arena.close_one(items, bytes)
}

pub fn close_ui_turn_patch_transport_session_one(session: u64) -> Result<UiTurnPatchTransportProgress, &'static str> {
    let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(UiTurnPatchTransportProgress::Blocked),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch transport arena is poisoned"),
    };
    if let Some(slot) = arena.slots.iter_mut().find(|slot| slot.session == session && matches!(slot.state, UiTurnPatchTransportState::Building | UiTurnPatchTransportState::Published)) {
        slot.state = UiTurnPatchTransportState::Closing;
        return Ok(UiTurnPatchTransportProgress::Pending { released_items: 0, released_bytes: 0 });
    }
    Ok(if arena.slots.iter().any(|slot| slot.session == session && slot.state != UiTurnPatchTransportState::Vacant) { UiTurnPatchTransportProgress::Blocked } else { UiTurnPatchTransportProgress::Idle })
}

/// 🧰️ One publication slot of the turn page: an exact patch owner and the retirement reservation it
/// hands back when it is dropped.
#[derive(Debug, Default)]
struct UiTurnPatchEntry {
    contents: UiTurnPatchContents,
    retirement: Option<UiTurnPatchRetireKey>,
}

/// 🧰️ The fixed exact-owner patch page emitted by one turn — every patch that was ready when the turn
/// assembled its result, in publication order, bounded by [`UI_TURN_PATCHES_MAXIMUM`] slots and by the
/// caller's own byte budget.
///
/// 📐️ `length` is the pushed prefix and `cursor` the next slot a reader takes, so `[cursor, length)`
/// is exactly the patches still in the page and slots below `cursor` hold nothing but the retirement
/// reservation their patch left behind.
#[derive(Debug)]
pub struct UiTurnPatches {
    entries: [UiTurnPatchEntry; UI_TURN_PATCHES_MAXIMUM],
    length: usize,
    cursor: usize,
}

impl Default for UiTurnPatches {
    fn default() -> Self {
        Self { entries: std::array::from_fn(|_| UiTurnPatchEntry::default()), length: 0, cursor: 0 }
    }
}

/// 🧨️ The page is parked BY VALUE in every transport slot and every transport handback, so its own
/// extent is the one block the guest's declared contiguous-request ceiling has to fund.
const _: () = assert!(size_of::<UiTurnPatches>() <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);

pub enum UiTurnPatchTransfer<T> {
    Empty,
    Transferred(T),
    Refused,
}

impl PartialEq for UiTurnPatches {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(left, right)| left == right)
    }
}

impl UiTurnPatches {
    /// 📥️ Appends one patch to the page's publication order, reserving that slot's own retirement.
    ///
    /// 🐛️ Until 2026-09-15 the page held exactly ONE patch, so the second surface a turn had ready was
    /// refused here and cost a whole host round trip of its own.
    #[expect(clippy::result_large_err, reason = "Refusal returns the exact fixed patch so the caller retains its retirement obligation.")]
    pub fn try_push_ui_patch(&mut self, patch: UiPatch) -> Result<(), UiPatch> {
        if self.length == UI_TURN_PATCHES_MAXIMUM {
            return Err(patch);
        }
        let index = self.length;
        if !self.entries[index].contents.terminal_is_empty() || self.entries[index].contents.pending.source_mut().is_err() {
            return Err(patch);
        }
        if self.entries[index].retirement.is_none() {
            let Ok(mut arena) = UI_TURN_PATCH_RETIRE_ARENA.try_lock() else {
                return Err(patch);
            };
            let Some(retirement) = arena.reserve() else { return Err(patch) };
            self.entries[index].retirement = Some(retirement);
        }
        *self.entries[index].contents.pending.source_mut().expect("preflighted pending patch slot") = Some(patch);
        self.length = index + 1;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &UiPatch> {
        self.entries[self.cursor..self.length].iter().filter_map(|entry| entry.contents.pending.get())
    }

    pub fn len(&self) -> usize {
        self.length - self.cursor
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 📏️ What this page costs its turn's byte budget — the sum every admitting caller compares.
    pub fn turn_bytes(&self) -> usize {
        self.iter().fold(0usize, |total, patch| total.saturating_add(ui_patch_turn_bytes(patch)))
    }

    /// 📤️ Hands out the NEXT patch in publication order, so a reader that loops until `Empty` sees
    /// every surface this turn published exactly once and in the order the guest published it.
    pub fn try_transfer_one<T>(&mut self, transfer: impl FnOnce(UiPatch) -> Result<T, UiPatch>) -> UiTurnPatchTransfer<T> {
        if self.cursor == self.length {
            return UiTurnPatchTransfer::Empty;
        }
        let index = self.cursor;
        let Ok(source) = self.entries[index].contents.pending.source_mut() else {
            return UiTurnPatchTransfer::Refused;
        };
        let Some(patch) = source.take() else { return UiTurnPatchTransfer::Empty };
        match transfer(patch) {
            Ok(value) => {
                self.cursor = index + 1;
                UiTurnPatchTransfer::Transferred(value)
            }
            Err(patch) => {
                *self.entries[index].contents.pending.source_mut().expect("preflighted pending patch slot") = Some(patch);
                UiTurnPatchTransfer::Refused
            }
        }
    }

    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("exact turn patch retirement").complete
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<semio_framework_ui_contract::UiValueRetirementStep, &'static str> {
        use semio_framework_ui_contract::UiValueRetirementStep;
        if items == 0 || bytes == 0 {
            return Ok(UiValueRetirementStep::default());
        }
        for index in 0..self.length {
            if !self.entries[index].contents.terminal_is_empty() {
                let mut step = self.entries[index].contents.close_step(items, bytes)?;
                step.complete = false;
                return Ok(step);
            }
            let Some(retirement) = self.entries[index].retirement else { continue };
            let mut arena = match UI_TURN_PATCH_RETIRE_ARENA.try_lock() {
                Ok(arena) => arena,
                Err(std::sync::TryLockError::WouldBlock) => return Ok(UiValueRetirementStep::default()),
                Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch retirement arena is poisoned"),
            };
            if !arena.release_empty(retirement) {
                return Err("exact turn patch retirement reservation missing");
            }
            self.entries[index].retirement = None;
            return Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() });
        }
        self.length = 0;
        self.cursor = 0;
        Ok(UiValueRetirementStep { complete: true, ..Default::default() })
    }
}

/// 📤️ Drains the page in publication order; whatever the reader leaves behind retires through the
/// page's own [`Drop`], exactly as a partially transferred page does.
pub struct UiTurnPatchesIntoIter {
    owner: UiTurnPatches,
}

impl Iterator for UiTurnPatchesIntoIter {
    type Item = UiPatch;

    fn next(&mut self) -> Option<UiPatch> {
        match self.owner.try_transfer_one(Ok::<UiPatch, UiPatch>) {
            UiTurnPatchTransfer::Transferred(patch) => Some(patch),
            UiTurnPatchTransfer::Empty | UiTurnPatchTransfer::Refused => None,
        }
    }
}

impl IntoIterator for UiTurnPatches {
    type Item = UiPatch;
    type IntoIter = UiTurnPatchesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        UiTurnPatchesIntoIter { owner: self }
    }
}

impl Drop for UiTurnPatches {
    fn drop(&mut self) {
        for index in 0..self.entries.len() {
            let Some(retirement) = self.entries[index].retirement.take() else { continue };
            let contents = std::mem::take(&mut self.entries[index].contents);
            UI_TURN_PATCH_HANDBACKS[retirement.slot].publish(retirement, contents);
        }
    }
}

impl Serialize for UiTurnPatches {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for patch in self.iter() {
            sequence.serialize_element(patch)?;
        }
        sequence.end()
    }
}

impl<'de> Deserialize<'de> for UiTurnPatches {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct UiTurnPatchesVisitor;

        impl<'de> serde::de::Visitor<'de> for UiTurnPatchesVisitor {
            type Value = UiTurnPatches;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a fixed turn patch page")
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut patches = UiTurnPatches::default();
                while let Some(patch) = access.next_element::<UiPatch>()? {
                    if patches.try_push_ui_patch(patch).is_err() {
                        return Err(serde::de::Error::custom("turn patch page capacity exceeded"));
                    }
                }
                Ok(patches)
            }
        }

        deserializer.deserialize_seq(UiTurnPatchesVisitor)
    }
}

//#endregion 🔖️TurnResult

#[cfg(test)]
use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;

#[cfg(test)]
#[path = "../🧪️tests/🐢️ui-dirty-scope/🦀️.rs"]
mod ui_dirty_scope_tests;

//#region 🔖️TurnResult
#[cfg(test)]
#[path = "../🧪️tests/🔬️ui-turn-patch/🦀️.rs"]
mod ui_turn_patch_tests;
//#endregion 🔖️TurnResult
