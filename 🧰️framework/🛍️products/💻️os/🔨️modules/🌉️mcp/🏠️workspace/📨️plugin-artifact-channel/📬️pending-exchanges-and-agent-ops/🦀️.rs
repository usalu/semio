//! 📬️ Pending exchanges of the plugin artifact channel: the pending command exchange, its response page, the headless agent operation filter, the abandoned exchange close and the exchange registry. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
#[cfg(not(target_arch = "wasm32"))]
pub struct PendingExchange {
    pub instance: u32,
    pub seq: u64,
    pub response: PendingResponsePage,
    /// 🧾️ The guest's own reason for a faulted ingress, kept from the turn that reported it until
    /// the bounded cleanup that follows reaches its terminal and can answer the caller with it.
    pub ingress_fault: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingResponseFault {
    Oversized,
    Duplicate,
}

/// 📬️ One command's answer, as it arrives: the guest's own encoded `AppFrame` bytes off the
/// `Shell{instance}` message lane.
///
/// 🧭️ This used to hold a single [`semio_framework::kernel::FixedCommandPage`], because the answer
/// was read off `Effect::Respond` and a `RequestOutcome`'s bytes are one page. An `AppFrame` is not:
/// `ReadDocument` answers a whole `Document{pack, spr}`, which for any real artifact is far past
/// 4 KiB, so that ceiling would have refused every genuine answer the moment the lane was read
/// correctly. The ceiling that does apply is the declared one for a single host↔guest answer,
/// [`semio_framework::kernel::COMMAND_MAXIMUM_BYTES`] — 8 MiB — and it is still a hard bound, so an
/// unbounded guest answer is a typed fault rather than an allocation.
#[cfg(not(target_arch = "wasm32"))]
pub enum PendingResponsePage {
    Empty,
    Frame { bytes: Vec<u8>, duplicate: bool },
    /// 🏁️ The command settled and the guest published no frame naming it — the same reading the
    /// React host takes (`🔌️PluginRuntime/🟦️.tsx`'s `commandIngressNeedsReplyStampV1`, which stamps
    /// `AppFrame::Done{in_reply_to}` on exactly this shape so a caller never waits on an answer a
    /// completed command was never going to publish). A caller that needs a real frame reports the
    /// `Done` as the unexpected variant it is; a caller whose verb genuinely answers `Done`
    /// (`LoadDocumentArchive`'s admission and acknowledgement, `TransactionUndo`/`Redo`) reads its own success.
    Stamped,
    Fault(PendingResponseFault),
}

#[cfg(not(target_arch = "wasm32"))]
impl PendingResponsePage {
    pub fn admit_frame(&mut self, bytes: &[u8]) {
        match self {
            Self::Empty => {
                *self = if bytes.len() > semio_framework::kernel::COMMAND_MAXIMUM_BYTES { Self::Fault(PendingResponseFault::Oversized) } else { Self::Frame { bytes: bytes.to_vec(), duplicate: false } };
            }
            Self::Frame { duplicate, .. } => *duplicate = true,
            Self::Stamped | Self::Fault(_) => *self = Self::Fault(PendingResponseFault::Duplicate),
        }
    }

    pub fn stamp_settled(&mut self) {
        if matches!(self, Self::Empty) {
            *self = Self::Stamped;
        }
    }

    /// 🏁️ Whether a real answer is already held — the test that tells a redundant `AppFrame::Done`
    /// stamp apart from a genuine second answer (see [`RoutingArtifactChannel`]'s sibling
    /// `admit_reply_frame`).
    pub(super) fn holds_frame(&self) -> bool {
        matches!(self, Self::Frame { .. })
    }

    pub fn take(&mut self, seq: u64) -> Result<store::AppFrame, Fault> {
        match std::mem::replace(self, Self::Empty) {
            Self::Empty => Err(PluginArtifactChannel::not_wired("execute_turn", format!("no AppFrame answering seq {seq} before terminal acknowledgement"))),
            Self::Frame { duplicate: true, .. } => Err(PluginArtifactChannel::not_wired("AppFrame response", format!("the guest published more than one frame answering seq {seq}"))),
            Self::Frame { bytes, duplicate: false } => ::semio_framework_async::poll::resolve_ready(store::decode_app_frame(&bytes)).map_err(|error| PluginArtifactChannel::not_wired("decoding AppFrame", error)),
            Self::Stamped => Ok(store::AppFrame::Done { in_reply_to: seq }),
            Self::Fault(PendingResponseFault::Oversized) => Err(PluginArtifactChannel::not_wired("AppFrame response", format!("the guest's answer exceeds the declared {}-byte host answer ceiling", semio_framework::kernel::COMMAND_MAXIMUM_BYTES))),
            Self::Fault(PendingResponseFault::Duplicate) => Err(PluginArtifactChannel::not_wired("AppFrame response", format!("the guest published more than one frame answering seq {seq}"))),
        }
    }

    /// 🧹️ A frame larger than one retirement grant is released a grant at a time, so a
    /// faulted command's cleanup stays bounded however big the answer the guest had queued.
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let released = match self {
            Self::Frame { bytes, .. } if bytes.len() <= maximum_bytes => bytes.len(),
            Self::Frame { bytes, .. } => {
                let remaining = bytes.len() - maximum_bytes;
                bytes.truncate(remaining);
                return (false, maximum_bytes);
            }
            Self::Empty | Self::Stamped | Self::Fault(_) => 0,
        };
        *self = Self::Empty;
        (true, released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

/// 🪟️ What a HEADLESS agent's transaction carries of one previewed verb: its document operations and owned children, never
/// its config or draft lanes — those are the view state (a selection, a viewport, a mode) of a shell the agent does not
/// have, and the real wire `TransactionPrepare` has no lane for them (coordinator decision, ticket 26/09/23 session 14c: a
/// headless agent edits DOCUMENTS). Beside a document change they are omitted and the preview says so
/// ([`VIEW_STATE_OMITTED_WARNING`]); a verb whose whole effect is view state is refused
/// [`crate::actions::AGENT_LANE_UNCARRIED_FAULT_CODE`] at prepare — it runs only from the shell — instead of failing at
/// invoke with "no wire representation" (live hub coverage on 7800/p33: procedural's add/rename/removeGeneration publish
/// `SetSelectedGeneration` beside their document op).
#[cfg(not(target_arch = "wasm32"))]
pub fn headless_agent_ops(capability_id: &str, ops: PreparedOps) -> Result<(PreparedOps, Vec<String>), Fault> {
    let view_lanes = [("config", !ops.config.is_empty()), ("draft", !ops.draft.is_empty())].into_iter().filter_map(|(lane, held)| held.then_some(lane)).collect::<Vec<_>>();
    if view_lanes.is_empty() {
        return Ok((ops, Vec::new()));
    }
    if ops.document.is_empty() && ops.children.is_empty() {
        return Err(Fault {
            code: AGENT_LANE_UNCARRIED_FAULT_CODE.to_string(),
            message: format!("action '{capability_id}' changes only {} view state, which a headless agent's transaction cannot carry; it runs only from the shell", view_lanes.join(" and ")),
        });
    }
    Ok((PreparedOps { config: Vec::new(), draft: Vec::new(), ..ops }, vec![VIEW_STATE_OMITTED_WARNING.to_string()]))
}

/// 🪟️ The preview warning of a verb whose config/draft view state a headless agent's transaction leaves in the shell.
#[cfg(not(target_arch = "wasm32"))]
pub const VIEW_STATE_OMITTED_WARNING: &str = "view-state-omitted: the action's config/draft view state (a selection, a viewport, a mode) stays in the shell; this transaction commits its document operations only";

/// 🧹️ Begins closing the exchange an ABANDONED command left on `instance` — a pending exchange whose seq is not `seq`'s —
/// so the bounded teardown in `exchange_one_turn` retires it before the next command on the instance is admitted.
#[cfg(not(target_arch = "wasm32"))]
pub fn begin_closing_abandoned_exchange(pending_exchanges: &PendingExchangeRegistry<1>, closes: &mut semio_framework::kernel::CommandDriverRegistry<1>, instance: u32, seq: u64) -> Result<(), Fault> {
    let Some(abandoned) = pending_exchanges.get(instance).map(|pending| pending.seq).filter(|pending_seq| *pending_seq != seq) else { return Ok(()) };
    if closes.is_active(u64::from(instance), abandoned) {
        closes.begin_close(u64::from(instance), abandoned).map_err(|fault| Fault { code: "channel.not-wired".to_string(), message: format!("abandoned command owner: {}: {}", fault.code.0, fault.message) })?;
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub struct PendingExchangeRegistry<const CAPACITY: usize> {
    pub slots: [Option<PendingExchange>; CAPACITY],
}

#[cfg(not(target_arch = "wasm32"))]
impl<const CAPACITY: usize> PendingExchangeRegistry<CAPACITY> {
    pub fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None) }
    }

    pub fn can_insert(&self, instance: u32) -> bool {
        self.slots[instance as usize % CAPACITY].is_none()
    }

    pub fn insert_admitted(&mut self, pending: PendingExchange) {
        let index = pending.instance as usize % CAPACITY;
        assert!(self.slots[index].is_none(), "fixed pending-exchange admission changed before insert");
        self.slots[index] = Some(pending);
    }

    pub fn get_mut(&mut self, instance: u32) -> Option<&mut PendingExchange> {
        self.slots[instance as usize % CAPACITY].as_mut().filter(|pending| pending.instance == instance)
    }

    pub fn get(&self, instance: u32) -> Option<&PendingExchange> {
        self.slots[instance as usize % CAPACITY].as_ref().filter(|pending| pending.instance == instance)
    }

    pub fn remove(&mut self, instance: u32) -> Option<PendingExchange> {
        let index = instance as usize % CAPACITY;
        if self.slots[index].as_ref().is_some_and(|pending| pending.instance == instance) {
            self.slots[index].take()
        } else {
            None
        }
    }

    pub fn close_response_step(&mut self, instance: u32, maximum_bytes: usize) -> (bool, usize) {
        self.get_mut(instance).map_or((true, 0), |pending| pending.response.close_step(maximum_bytes))
    }
}
//#endregion 🔖️ArtifactChannel
