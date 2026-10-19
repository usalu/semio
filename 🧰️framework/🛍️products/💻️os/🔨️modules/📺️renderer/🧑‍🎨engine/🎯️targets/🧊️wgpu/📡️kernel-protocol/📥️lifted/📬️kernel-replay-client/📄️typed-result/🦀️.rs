//! 📄️ Typed result and exchange: the typed operation result page, the exchange surface document and outcome, the kernel outcome and the response slot. See [`super`].

use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️Requests/Outcomes
//#region 📬️TypedOperationResultPage
/// 📬️ One typed-operation result page a guest publishes to its shell (`semio.typed-operation-page.v1`),
/// and the exact ACK (`semio.typed-operation-ack.v1`) the guest waits for before it retires the page
/// and continues the operation. The kernel thread acknowledges every page inside the turn settle
/// that received it ([`KernelPoolState::run_turn`]), exactly as the React host acknowledges inside
/// `settlePluginTurn` (`typedOperationAcknowledgements`), and hands the pages to the caller on
/// [`ExchangeOutcome::typed_results`]. Before this, the page waited in a renderer exchange no
/// native consumer ever read, so every reserved tool job (undo, redo, checkpoint, copy, paste…)
/// stalled its guest in `MoreWork` (ticket 26/09/23 slice WG8, measured on block2d's undo).
pub const TYPED_OPERATION_RESULT_PAGE_BYTES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedOperationResultToken {
    pub receiver: u32,
    pub operation: u64,
    pub generation: u64,
    pub sequence: u32,
    pub attempt: u8,
}

#[derive(Clone)]
pub struct TypedOperationResultPage {
    pub token: TypedOperationResultToken,
    pub lane: u8,
    pub bytes: [u8; TYPED_OPERATION_RESULT_PAGE_BYTES],
    pub len: usize,
}

impl TypedOperationResultPage {
    pub(super) const PAGE_MAGIC: &'static [u8] = b"semio.typed-operation-page.v1\0";
    pub(super) const ACK_MAGIC: &'static [u8] = b"semio.typed-operation-ack.v1\0";
    /// 🛤️ Highest `TypedOperationResultLane` discriminant the guest writes at byte 25 of a
    /// page (`🔌️plugin/🦀️.rs`: Artifact 0 … Fault 11, Interaction 12, WindowTransient 13,
    /// WindowConfig 14). Capped at 12 this demux refused the guest's OWN mesh publication lane and
    /// fell the page through to `decode_app_frame`, which cannot read it either
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END); the neutral lane fixture is the oracle.
    const LANE_MAX: u8 = 14;

    pub fn try_copy_from(token: TypedOperationResultToken, lane: u8, bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err("typed-operation result exceeds renderer fixed page authority");
        }
        let mut page = Self { token, lane, bytes: [0; TYPED_OPERATION_RESULT_PAGE_BYTES], len: bytes.len() };
        page.bytes[..bytes.len()].copy_from_slice(bytes);
        Ok(page)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn decode_guest_message(bytes: &[u8]) -> Option<Self> {
        let body = bytes.strip_prefix(Self::PAGE_MAGIC)?;
        if body.len() < 30 {
            return None;
        }
        let token = TypedOperationResultToken {
            receiver: u32::from_le_bytes(body[0..4].try_into().ok()?),
            operation: u64::from_le_bytes(body[4..12].try_into().ok()?),
            generation: u64::from_le_bytes(body[12..20].try_into().ok()?),
            sequence: u32::from_le_bytes(body[20..24].try_into().ok()?),
            attempt: body[24],
        };
        let lane = body[25];
        if lane > Self::LANE_MAX {
            return None;
        }
        let len = u32::from_le_bytes(body[26..30].try_into().ok()?) as usize;
        if len > TYPED_OPERATION_RESULT_PAGE_BYTES || body.len() != 30 + len {
            return None;
        }
        Self::try_copy_from(token, lane, &body[30..]).ok()
    }

    pub fn encode_ack(token: TypedOperationResultToken) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::ACK_MAGIC.len() + 25);
        bytes.extend_from_slice(Self::ACK_MAGIC);
        bytes.extend_from_slice(&token.receiver.to_le_bytes());
        bytes.extend_from_slice(&token.operation.to_le_bytes());
        bytes.extend_from_slice(&token.generation.to_le_bytes());
        bytes.extend_from_slice(&token.sequence.to_le_bytes());
        bytes.push(token.attempt);
        bytes
    }
}
//#endregion 📬️TypedOperationResultPage

pub struct ExchangeSurfaceDocument {
    pub surface: SurfaceId,
    pub document: UiDocumentLease,
}

pub struct ExchangeOutcome {
    pub frames: Vec<protocol::AppFrame>,
    /// 🖼️ Surfaces this turn repainted or retained on desync — reconciled against the kernel
    /// pool state machine's retained tree (`KernelPoolState::retained`); see that field's doc for the
    /// full-body-vs-desync policy.
    pub surfaces: UiFixedList<ExchangeSurfaceDocument, UI_DOCUMENT_LEASE_SLOTS>,
    /// 🧾️ Every effect this turn produced that was NOT one of the `Effect::SendMessage{target:
    /// Shell{..}}` entries already unpacked into `frames` above — `📓️design-abi.md` §2's
    /// replacement for the deleted `AppFrame::Effects` wrapper: effects now travel as real
    /// `kernel::Effect` values on `TurnResult.effects` directly, not re-encoded as an `AppFrame`.
    pub effects: Vec<Effect>,
    pub command_ingress: semio_framework::kernel::CommandIngressStatus,
    /// 📬️ Every typed-operation result page this exchange's turns published to the shell, in
    /// publication order — each already acknowledged to the guest inside the settle.
    pub typed_results: Vec<TypedOperationResultPage>,
}

impl ExchangeOutcome {
    /// 🤫️ Whether this turn carried nothing a caller could act on — no frame, effect, typed result
    /// or surface document and an idle command ingress. The native twin of the browser drive's
    /// `shardTurnCarriesNothingV1` (`🎭️actor/🖼️wire-turn/🟦️.ts`) at this host's level, where
    /// patches have already become surface documents and shell messages frames.
    pub fn carries_nothing(&self) -> bool {
        self.frames.is_empty() && self.effects.is_empty() && self.surfaces.is_empty() && self.typed_results.is_empty() && matches!(self.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle)
    }

    /// ➕️ Folds a continuation turn's outcome into this one: frames, effects and typed results in
    /// order, the latest command-ingress status a turn actually reported, and each surface's LATER
    /// document (a continuation repainting a surface supersedes the earlier lease, which is handed
    /// back by its own `Drop`). An `Idle` ingress reports nothing: the acknowledgement turn after a
    /// page turn answers `Idle`, and letting it overwrite the page's `CommandComplete` made
    /// [`KernelPoolState::exchange_commands`] resend the page for ever — measured as one
    /// `addHandleKind` applied 64 times, then 6 466 resends in 83 min, on native block2d (ticket
    /// 26/09/23 slice WG8, `wp-wg8/generated/journey-{9,10}`).
    pub fn absorb(&mut self, later: ExchangeOutcome) -> Result<(), String> {
        self.frames.extend(later.frames);
        self.effects.extend(later.effects);
        self.typed_results.extend(later.typed_results);
        if !matches!(later.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle) {
            self.command_ingress = later.command_ingress;
        }
        for entry in later.surfaces {
            if let Some(index) = self.surfaces.iter().position(|earlier| earlier.surface == entry.surface) {
                let _ = self.surfaces.swap_remove(index);
            }
            self.surfaces.try_push(entry).map_err(|_| "kernel: continuation surfaces exceed the exchange's fixed surface capacity".to_string())?;
        }
        Ok(())
    }

    pub fn take_surface(&mut self, surface: &str) -> Option<UiDocumentLease> {
        let index = self.surfaces.iter().position(|entry| entry.surface.as_ref() == surface)?;
        self.surfaces.swap_remove(index).map(|entry| entry.document)
    }

    pub fn take_product_replay_authority(&mut self, instance: u32, permit: MountedProductReplayAdmissionPermit) -> MountedProductReplayAdmission {
        let Some(index) = self.effects.iter().position(|effect| matches!(effect, Effect::SpawnJob { .. })) else {
            permit.release();
            return MountedProductReplayAdmission::None;
        };
        let (kind, input) = match &self.effects[index] {
            Effect::SpawnJob { kind, input, .. } => (kind.as_str(), input.as_slice()),
            _ => unreachable!("selected product replay effect remains SpawnJob"),
        };
        let cause = if kind.len() > PRODUCT_REPLAY_KIND_BYTES {
            Some(MountedProductReplayRefusalCause::KindCapacity)
        } else if input.len() > PRODUCT_REPLAY_INPUT_BYTES {
            Some(MountedProductReplayRefusalCause::InputCapacity)
        } else {
            None
        };
        let recovery = cause.is_none().then(|| mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").reserve()).flatten();
        if let Some(recovery) = recovery {
            let spawn = self.effects.remove(index);
            let request = MountedProductReplayRequest::from_admitted_effect(instance, index, spawn, recovery);
            permit.release();
            return MountedProductReplayAdmission::Admitted(request);
        }
        let cause = cause.unwrap_or(MountedProductReplayRefusalCause::RecoveryCapacity);
        let spawn = self.effects.remove(index);
        let remaining_effects = std::mem::take(&mut self.effects);
        MountedProductReplayAdmission::Refused(permit.refuse(cause, index, spawn, remaining_effects))
    }
}

pub enum KernelOutcome {
    Created(Result<u32, ProgramFault>),
    Exchanged(Result<ExchangeOutcome, ProgramFault>),
    ProductReplayMounted(Result<(), MountedProductReplayRequest>),
}
//#endregion

//#region 🔖️KernelFuture — the leaf `Future` every `ProgramBridgeEntry` async method awaits
#[derive(Default)]
struct ResponseState {
    pub result: Option<KernelOutcome>,
    pub waker: Option<Waker>,
}

#[derive(Default)]
pub struct ResponseSlot {
    pub state: Mutex<ResponseState>,
    pub abandoned: std::sync::atomic::AtomicBool,
}

impl ResponseSlot {
    /// 🛑️ Whether the requester dropped its future before an outcome reached it — the explicit cancel of
    /// a request whose owner follows it with progress and a cancel (a document open's create request).
    pub fn abandoned(&self) -> bool {
        self.abandoned.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn deliver(&self, outcome: KernelOutcome) {
        let waker = {
            let mut state = self.state.lock().expect("response slot lock");
            state.result = Some(outcome);
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}
//#endregion 🔖️KernelFuture — the leaf `Future` every `ProgramBridgeEntry` async method awaits
//#endregion 🎠️KernelRuntime
