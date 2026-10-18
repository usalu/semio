//! 🧮️ Owned constants and codec origin: the result, checkpoint, fuel and deadline limits, the compiled guest and the codec origin with its cell, use and assembly. See [`super`].

use super::*;

//#region 🧠️OwnedRuntime
/// 📦️ The largest result bytes one owned export may return.
pub(super) const OWNED_RESULT_LIMIT: usize = 64 * 1024 * 1024;

/// 🪪️ The header of an owned host checkpoint frame.
pub(super) const OWNED_CHECKPOINT_MAGIC: &[u8; 8] = b"SMOWNH01";

/// ⛽️ The instructions one interpreter step may run before the host looks at its deadline, cancellation and progress.
pub const OWNED_STEP_FUEL: u64 = 4_096;

/// ⏱️ The wall bound of one owned job cancellation slice.
pub(super) const OWNED_SLICE_DEADLINE_MS: u32 = 8;

/// 🧊️ The artifact schema a codec origin's assembly asks `codec.pack-schema-hash` for: one no app declares, so the
/// export assembles the bundle, answers its refusal and touches no app's codec.
pub(super) const OWNED_CODEC_ORIGIN_SCHEMA: &str = "";

/// ⏲️ How often a codec call waiting for another call's origin assembly relays that assembly's fuel and looks at its
/// own cancellation.
pub(super) const OWNED_CODEC_ORIGIN_WAIT_POLL: std::time::Duration = std::time::Duration::from_millis(50);

/// 🧠️ One component compiled for the owned interpreter: its parsed artifact, and the origin its codec calls start from
/// once one of them assembled it. A clone shares both, so every holder of one compiled guest starts its codec calls from the
/// same origin.
#[derive(Clone)]
pub struct OwnedCompiledGuest {
    pub(super) artifact: Arc<OwnedSemioArtifact>,
    pub(super) codec_origin: Arc<OwnedCodecOriginCell>,
}

impl OwnedCompiledGuest {
    /// 📏️ The bytes this compiled guest holds beyond its parsed component: its codec origin once a codec call assembled it
    /// ([`OwnedCodecOriginCell`]), else none. A residency budget charges a compiled guest this on top of its component.
    pub fn codec_origin_bytes(&self) -> u64 {
        self.codec_origin.resident_bytes()
    }
}

/// 🧊️ A compiled guest's codec origin: its owned instance right after a first codec export assembled the plugin bundle
/// (`__semio_ensure_plugin_runtime`: every artifact declared, every mutation roster, the built bundle) and completed.
/// Measured on catalog B3 (H12, 2026-09-27): a fresh instance spends 521.5 M of a puzzle codec call's 524.7 M fuel on that
/// assembly and the operation itself ≈ 3.2 M, and every codec call used to pay it on a throwaway instance. Each codec call
/// now starts from a copy of this state, so every call still sees exactly the same guest — the assembled bundle and
/// nothing another call did — and answers byte for byte what a fresh instance answers.
pub(super) struct OwnedCodecOrigin {
    pub(super) actor: OwnedSemioInstance,
    pub(super) diagnostics: Vec<u8>,
    pub(super) context: i32,
    pub(super) next_resource: i32,
    pub(super) assembly_fuel: u64,
}

impl OwnedCodecOrigin {
    /// 📏️ What the origin holds: the guest's linear memory and what it printed while assembling.
    fn resident_bytes(&self) -> u64 {
        u64::try_from(self.actor.memory().map_or(0, <[u8]>::len).saturating_add(self.diagnostics.len())).unwrap_or(u64::MAX)
    }
}

/// 🚦️ Where one compiled guest's codec origin stands; `Assembling` carries the fuel the assembly reported so far.
pub(super) enum OwnedCodecOriginState {
    Absent,
    Assembling { fuel: u64 },
    Ready(Arc<OwnedCodecOrigin>),
}

/// 🧊️ The codec origin of one compiled guest, assembled once, by the first codec call that needs it. A call that arrives
/// while another assembles waits for that assembly instead of running a second one, relays its fuel as its own progress —
/// a stall bound watching the waiter sees the assembly move, and sees it stop — and observes its own cancellation. An
/// assembly that faults, is cancelled or panics leaves the origin absent, and the next call assembles it itself.
pub(super) struct OwnedCodecOriginCell {
    pub(super) state: Mutex<OwnedCodecOriginState>,
    pub(super) settled: std::sync::Condvar,
}

impl OwnedCodecOriginCell {
    pub(crate) fn new() -> Self {
        Self { state: Mutex::new(OwnedCodecOriginState::Absent), settled: std::sync::Condvar::new() }
    }

    pub(super) fn lock(&self) -> std::sync::MutexGuard<'_, OwnedCodecOriginState> {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn resident_bytes(&self) -> u64 {
        match &*self.lock() {
            OwnedCodecOriginState::Ready(origin) => origin.resident_bytes(),
            OwnedCodecOriginState::Absent | OwnedCodecOriginState::Assembling { .. } => 0,
        }
    }

    /// 📈️ Records the running assembly's fuel for the calls waiting on it.
    pub(super) fn assembled_so_far(&self, fuel: u64) {
        if let OwnedCodecOriginState::Assembling { fuel: reported } = &mut *self.lock() {
            *reported = fuel;
        }
        self.settled.notify_all();
    }
}

/// 🧊️ A codec call's use of its compiled guest's origin: the origin, the fuel THIS call spent assembling it (the part of its
/// budget it used), and the fuel it already reported as progress — the assembly's own, or the one it relayed while it waited
/// for another call's — so its operation's progress continues from there instead of starting over.
pub(super) struct OwnedCodecOriginUse {
    pub(super) origin: Arc<OwnedCodecOrigin>,
    pub(super) spent_fuel: u64,
    pub(super) reported_fuel: u64,
}

/// 🧯️ One origin assembly in flight: however it ends — with an origin, a fault, a cancellation or a panic — the cell
/// settles (ready, or absent again) and every waiting call wakes.
pub(super) struct OwnedCodecOriginAssembly<'a> {
    pub(super) cell: &'a OwnedCodecOriginCell,
    pub(super) origin: Option<Arc<OwnedCodecOrigin>>,
}

impl Drop for OwnedCodecOriginAssembly<'_> {
    fn drop(&mut self) {
        *self.cell.lock() = self.origin.take().map_or(OwnedCodecOriginState::Absent, OwnedCodecOriginState::Ready);
        self.cell.settled.notify_all();
    }
}
//#endregion 🧠️OwnedRuntime
