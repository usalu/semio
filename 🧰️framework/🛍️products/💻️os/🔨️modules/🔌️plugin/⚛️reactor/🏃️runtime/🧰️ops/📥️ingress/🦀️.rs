//! 📥️ Opening relays, command ingress, the exchange output and the custody manifest receiving.

use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_decl_app::{ArtifactApp, MediaArtifactError};
use semio_framework_plugin_decl_views::media_contract::{ArtifactMediaExportHandle, ArtifactMediaExportPoll};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework::kernel::{Effect, InvocationResult};
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use semio_framework::manifest::{CommandAddress as ManifestCommandAddress, CommandOwnerAddress as ManifestCommandOwnerAddress};
use semio_framework_plugin_plugin_program::{Plugin, PluginProgram, decode_wire_serialized, encode_wire_serialized};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_plugin_job::{TransactionProposalDraft, TypedOperationResultPage};
use serde_json::Value;
use crate::ops_typed_operations::drive_self_waking_ready;
use crate::ops_wire_limits::find_instance;
use semio_framework_plugin_runtime_instances::inst_wire_frames::{instance_actor, with_instances_turn_mut};
use semio_framework_plugin_reactor_app::app_runtime_helpers::plugin_internal_fault;
#[cfg(test)]
use std::mem::size_of;
/// 🎯️ `AppCommand::ArtifactCommand`'s Wave 1 mapping — the same magic action-name strings
/// `VcsArtifactApp::dispatch_action` already intercepts for `store::os_store::store_schema::ArtifactCommand` verbs (duplicated
/// here rather than imported since `app`'s `HISTORY_ACTION_IDS` const is private — see
/// `plugin_exchange`'s doc for why this frame is scoped to history verbs only in Wave 1).
const DOCUMENT_COMMAND_ACTION_IDS: [&str; 6] = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"];
/// 📤️ Collects `result`'s side-channel effects/events as wire-encoded bytes — channel v12 (A4)
/// dropped `AppFrame::Effects`/`AppFrame::Events`, since effects/events now travel straight into
/// `TurnResult` (design-abi.md §2/§4) instead of being wrapped as a frame; `PluginExchangeOutput`
/// carries them alongside `frames`. Shared by `AppCommand::Command`'s and
/// `AppCommand::ArtifactCommand`'s handling below.
#[doc(hidden)]
pub async fn push_invocation_side_frames(effects: &mut Vec<Vec<u8>>, events: &mut Vec<Vec<u8>>, result: &InvocationResult) {
    for effect in result.requested_effects.iter() {
        effects.push(encode_wire_serialized(effect));
    }
    for event in result.events.iter() {
        events.push(encode_wire_serialized(event));
    }
}
async fn opening_role(role: u8) -> Result<semio_framework::AppRole, Fault> {
    match role {
        0 => Ok(semio_framework::AppRole::Viewer),
        1 => Ok(semio_framework::AppRole::Editor),
        value => Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-role"), format!("opening role {value} must be 0 (viewer) or 1 (editor)"))),
    }
}
async fn opening_dialect(artifact_kind: String, standard: String, subset: String) -> Result<semio_framework_artifact_reference::ArtifactDialect, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    for (field, value) in [("artifact_kind", &artifact_kind), ("standard", &standard), ("subset", &subset)] {
        if value.trim().is_empty() {
            return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-dialect"), format!("opening {field} must be non-empty")));
        }
    }
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind, standard, subset };
    semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&dialect.to_coordinate()).map_err(|error| Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-dialect"), error))?;
    Ok(dialect)
}
async fn relay_opening_command(action_id: &'static str, arguments: Value) -> Result<Effect, Fault> {
    Ok(Effect::ReplayShellCommand { action_id: action_id.into(), args: Some(DslValue::from(&arguments)) })
}
#[doc(hidden)]
pub async fn relay_open_artifact(artifact_ref: String, role_wire: u8, plugin_id: String, app_id: String) -> Result<Effect, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    let role = opening_role(role_wire).await?;
    let (dialect, artifact_role) = semio_framework::parse_surface_app_id(&artifact_ref).map_err(|error| Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-artifact-ref"), error))?;
    if role != artifact_role {
        return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.role-mismatch"), format!("artifact ref {artifact_ref} declares {} but command declares {}", artifact_role.as_str(), role.as_str())));
    }
    match (plugin_id.trim().is_empty(), app_id.trim().is_empty()) {
        (true, true) => {}
        (false, false) => {
            let (app_dialect, app_role) = semio_framework::parse_surface_app_id(&app_id).map_err(|error| Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-app-ref"), error))?;
            if app_dialect != dialect || app_role != role {
                return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.app-mismatch"), format!("explicit app {app_id} does not serve {}#{}", dialect.to_coordinate(), role.as_str())));
            }
        }
        _ => return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.partial-app-ref"), "plugin_id and app_id must either both be empty or both be set")),
    }
    relay_opening_command("os.open-artifact", serde_json::json!({ "artifactRef": artifact_ref, "role": role_wire, "pluginId": plugin_id, "appId": app_id })).await
}
#[doc(hidden)]
pub async fn relay_set_default_app(artifact_kind: String, standard: String, subset: String, role_wire: u8, plugin_id: String, app_id: String) -> Result<Effect, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    let dialect = opening_dialect(artifact_kind.clone(), standard.clone(), subset.clone()).await?;
    let role = opening_role(role_wire).await?;
    if plugin_id.trim().is_empty() {
        return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-app-ref"), "plugin_id must be non-empty when setting a default app"));
    }
    let (app_dialect, app_role) = semio_framework::parse_surface_app_id(&app_id).map_err(|error| Fault::new(FaultOrigin::Os, FaultCode::new("opening.invalid-app-ref"), error))?;
    if app_dialect != dialect || app_role != role {
        return Err(Fault::new(FaultOrigin::Os, FaultCode::new("opening.app-mismatch"), format!("default app {app_id} does not serve {}#{}", dialect.to_coordinate(), role.as_str())));
    }
    relay_opening_command("os.set-default-app", serde_json::json!({ "artifactKind": artifact_kind, "standard": standard, "subset": subset, "role": role_wire, "pluginId": plugin_id, "appId": app_id })).await
}
#[doc(hidden)]
pub async fn relay_clear_default_app(artifact_kind: String, standard: String, subset: String, role_wire: u8) -> Result<Effect, Fault> {
    opening_dialect(artifact_kind.clone(), standard.clone(), subset.clone()).await?;
    opening_role(role_wire).await?;
    relay_opening_command("os.clear-default-app", serde_json::json!({ "artifactKind": artifact_kind, "standard": standard, "subset": subset, "role": role_wire })).await
}
#[cfg(test)]
include!("../../../../🧪️tests/🔬️plugin-runtime-opening-command-relay/🦀️.rs");
/// 🔀️ Frames a `TransactionProposalDraft` (contract §5.1) as `AppFrame::TransactionProposal`,
/// wire-encoding each foreign step via `encode_wire_serialized` (contract §2's per-element
/// `ForeignStep` encoding note) and minting a proposal id scoped to this dispatch
/// (`"<instance_id>:<seq>"`, unique enough for one instance's own in-flight proposals — all a
/// single-command dispatch ever needs).
#[doc(hidden)]
pub async fn transaction_proposal_frame(instance_id: u32, seq: u64, proposal: TransactionProposalDraft) -> protocol::AppFrame {
    let mut foreign = Vec::with_capacity(proposal.foreign.len());
    for step in proposal.foreign.iter() {
        foreign.push(encode_wire_serialized(step));
    }
    protocol::AppFrame::TransactionProposal { in_reply_to: seq, proposal_id: format!("{instance_id}:{seq}"), local_ops: proposal.local_ops, foreign }
}
#[doc(hidden)]
pub async fn encode_exchange_frames(frames: &[protocol::AppFrame]) -> Vec<Vec<u8>> {
    let mut encoded = Vec::with_capacity(frames.len());
    for frame in frames {
        encoded.push(protocol::encode_app_frame(frame).await);
    }
    encoded
}
/// 🔀️ `PluginExchangeOutput` — `plugin_exchange`'s return shape. `frames` still carries the
/// surviving `protocol::AppFrame` wire encodings (routed through `route_app_frame` by the
/// reactor's `poll`); `effects`/`events` are wire-encoded `kernel::Effect`/`kernel::AppEvent`
/// bytes collected directly — channel v12 (A4) removed `AppFrame::Effects`/`AppFrame::Events`,
/// since design-abi.md §2/§4 has effects/events travel straight into `TurnResult`, never wrapped
/// as a frame.

#[doc(hidden)]
#[path="../../🎞️command/📥️custody/🛂️manifest/🦀️.rs"]
pub mod original_manifest_receiving;
pub use crate::ops_ingress::original_manifest_receiving::OriginalDecodedPluginCommand;
use crate::ops_ingress::original_manifest_receiving::OriginalManifestCommandReceiving;
#[cfg(feature="original-actor-context-testing")]
/// 🧪️ Qualifies the canonical receiving body without mounting unrelated app fixture suites.
pub fn original_manifest_receiving_law(){original_manifest_receiving::tests::original_manifest_fields_keep_paid_source_and_denied_transfer()}

pub enum PluginCommandIngress {
    Encoded(semio_framework::kernel::PagedCommand),
    Decoding(protocol::PagedAppCommandDecodeCursor),
    Decoded(OriginalDecodedPluginCommand),
    Closing { cursor: protocol::PagedAppCommandDecodeCursor, fault: Fault },
    ClosingDecoded { owner: OriginalDecodedPluginCommand, fault: Fault },
    FailedDecoded{owner:OriginalDecodedPluginCommand,failure:Option<semio_framework_job::OriginalTurnFailure>,identity:(semio_framework_job::OperationId,semio_framework_job::Generation),close_failure:Option<semio_framework_job::OriginalTurnFailure>,compound:Option<semio_framework_job::OriginalClosureFailure>,raw_media:Option<MediaArtifactError>},
}
#[doc(hidden)]
pub enum OriginalCommandIngressRetirement{Pending(PluginCommandIngress),Failed(semio_framework_job::OriginalTurnFailure)}
#[doc(hidden)]
pub enum PluginCommandIngressStep {
    Pending(PluginCommandIngress),
    Ready(OriginalDecodedPluginCommand),
    TerminalFault(Fault),
    TerminalOriginalFailure(semio_framework_job::OriginalTurnFailure),
}
/// 📥️ Ingress moves one turn may spend driving a command from its assembled pages to dispatch.
///
/// Every state [`PluginCommandIngress::step`] reaches is a bounded move: a decode state reads one
/// `read_bounded_bytes` field, a closing state releases exactly one page. Answering `Pending`
/// after each of them cost the HOST a whole turn round trip per move — measured 4.006 turns for a
/// ONE-page `AppCommand::CommandText` (`a_long_command_stream_never_pins_the_retained_ingress_
/// authority`: 320 commands, 1 282 turns), five for a one-page `AppCommand::Command`, and one more
/// for every further page. The grant is the whole close ladder — `COMMAND_MAXIMUM_PAGES` page
/// releases plus the three decoded field stages plus the header — so a command assembles, decodes
/// and dispatches inside the turn that carried its LAST page, and a cancelled one retires inside a
/// single turn too. Ticket 26/09/02 wave B24.
#[doc(hidden)]
pub const COMMAND_INGRESS_MOVES_PER_TURN: usize = semio_framework::kernel::COMMAND_MAXIMUM_PAGES + 8;
impl PluginCommandIngress {
    pub fn retire_failed_step_on_original(self,cx:&mut semio_framework_job::StepContext<'_>)->OriginalCommandIngressRetirement{
        if !matches!(self,Self::FailedDecoded{..}){return OriginalCommandIngressRetirement::Pending(self)}
        match self.advance_original(1,cx){PluginCommandIngressStep::Pending(owner)=>OriginalCommandIngressRetirement::Pending(owner),PluginCommandIngressStep::TerminalOriginalFailure(failure)=>OriginalCommandIngressRetirement::Failed(failure),_=>unreachable!("failed original source only closes and returns its full original cause")}
    }

    pub fn advance_original(self,moves:usize,cx:&mut semio_framework_job::StepContext<'_>)->PluginCommandIngressStep{
        if let Self::FailedDecoded{mut owner,mut failure,identity,mut close_failure,mut compound,mut raw_media}=self{
            if identity!=(cx.operation(),cx.generation())||cx.should_yield(){return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})}
            if raw_media.is_some(){
                match semio_framework_job::admit_original_domain_failure(&mut raw_media,cx){Ok(Some(original))=>failure=Some(original),Ok(None)=>{},Err(refusal)=>close_failure=Some(refusal)}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            if compound.is_some(){
                match semio_framework_job::admit_original_domain_failure(&mut compound,cx){Ok(Some(original))=>failure=Some(original),Ok(None)=>{},Err(refusal)=>close_failure=Some(refusal)}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            let grant=cx.retained_grant();
            if close_failure.is_some(){
                if grant.maximum_items>0&&grant.maximum_depth>0{compound=Some(semio_framework_job::OriginalClosureFailure{initial:failure.take().expect("failed decoded initial source remains original"),closure:close_failure.take().expect("original closure cause remains original")});cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).expect("original closure donation was preflighted");cx.consume_fuel(1)}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            if !owner.terminal_is_empty(){
                match owner.close_original_step(cx){Ok(_)=>{},Err(refusal)=>close_failure=Some(refusal)}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            if grant.maximum_items>0&&grant.maximum_depth>0{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).expect("original failure donation was preflighted");cx.consume_fuel(1);return PluginCommandIngressStep::TerminalOriginalFailure(failure.take().expect("failed decoded initial source remains original"))}
            PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
        }else{self.advance(moves)}
    }
    /// 📥️ Drives the owner until it is dispatchable, terminal, or the turn's move grant is spent.
    pub fn advance(self, moves: usize) -> PluginCommandIngressStep {
        let mut owner = self;
        for _ in 0..moves {
            match owner.step() {
                PluginCommandIngressStep::Pending(next) => owner = next,
                terminal => return terminal,
            }
        }
        PluginCommandIngressStep::Pending(owner)
    }
    pub fn retire_step(self) -> Option<Self> {
        if matches!(self,Self::FailedDecoded{..}){return Some(self)}
        match self.cancel(plugin_internal_fault("command lifetime is closing")).step() {
            PluginCommandIngressStep::Pending(owner) => Some(owner),
            PluginCommandIngressStep::TerminalFault(_) => None,
            PluginCommandIngressStep::TerminalOriginalFailure(_)=>unreachable!("whole failed decoded sources require their original retirement caller"),
            PluginCommandIngressStep::Ready(_) => unreachable!("cancelled ingress cannot dispatch"),
        }
    }
    pub fn cancel(self, fault: Fault) -> Self {
        match self {
            Self::Encoded(command) => Self::Closing { cursor: protocol::PagedAppCommandDecodeCursor::new(command), fault },
            Self::Decoding(cursor) | Self::Closing { cursor, .. } => Self::Closing { cursor, fault },
            Self::Decoded(owner) | Self::ClosingDecoded { owner, .. } => match owner.receiving.as_ref(){Some(receiving)=>Self::FailedDecoded{identity:receiving.identity(),owner,failure:Some(fault.into()),close_failure:None,compound:None,raw_media:None},None=>Self::ClosingDecoded { owner, fault }},
            original @ Self::FailedDecoded{..}=>original,
        }
    }
    pub fn step(self) -> PluginCommandIngressStep {
        match self {
            Self::Encoded(encoded) => {
                let cursor = protocol::PagedAppCommandDecodeCursor::new(encoded);
                if cursor.kind() == 28 {
                    PluginCommandIngressStep::Pending(Self::Closing { cursor, fault: plugin_internal_fault("Presence commands require the reserved presence ingress authority") })
                } else {
                    PluginCommandIngressStep::Pending(Self::Decoding(cursor))
                }
            }
            Self::Decoding(mut cursor) => match cursor.step() {
                Ok(Some(command)) => PluginCommandIngressStep::Pending(Self::Decoded(OriginalDecodedPluginCommand::new(command))),
                Ok(None) => PluginCommandIngressStep::Pending(Self::Decoding(cursor)),
                Err(fault) => PluginCommandIngressStep::Pending(Self::Closing { cursor, fault }),
            },
            Self::Decoded(owner) => PluginCommandIngressStep::Ready(owner),
            original @ Self::FailedDecoded{..}=>PluginCommandIngressStep::Pending(original),
            Self::Closing { mut cursor, fault } => {
                let (complete, _) = cursor.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if complete && cursor.terminal_is_empty() {
                    PluginCommandIngressStep::TerminalFault(fault)
                } else {
                    PluginCommandIngressStep::Pending(Self::Closing { cursor, fault })
                }
            }
            Self::ClosingDecoded { mut owner, fault } => {
                let (complete, _, _) = owner.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if complete && owner.terminal_is_empty() {
                    PluginCommandIngressStep::TerminalFault(fault)
                } else {
                    PluginCommandIngressStep::Pending(Self::ClosingDecoded { owner, fault })
                }
            }
        }
    }
}
#[cfg(test)]
include!("../../../../🧪️tests/🔬️plugin-runtime-paged-command-ingress/🦀️.rs");
#[derive(Default)]
pub struct PluginExchangeOutput {
    pub original_receiving:Option<OriginalPublicationReceiving>,
    pub constructor_receipt: Option<(semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorKind, semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)>,
    pub preparation_receipt: Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)>,
    pub retirement_receipt: Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)>,
    pub history_command_receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    pub tool_run_receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    pub frames: Vec<Vec<u8>>,
    pub effects: Vec<Vec<u8>>,
    pub events: Vec<Vec<u8>>,
    pub retry_command: Option<(u64, PluginCommandIngress)>,
    pub command_terminal_fault: Option<Vec<u8>>,
    pub presence_pending: Option<u64>,
    pub presence_terminal: Option<u64>,
    pub presence_terminal_fault: Option<Vec<u8>>,
    pub typed_operation_results: Vec<TypedOperationResultPage>,
}
/// 🏷️ Names the one actual producer whose full physical receipt was posted in the initiating context.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalPublicationFamily{Constructor,Preparation,Retirement,History,Tool}
/// 🧾️ Captures actual already-posted effects, exact initiating grant and recipient before another loan.
#[derive(Clone,Copy)]
pub struct OriginalPublicationReceiving{pub family:Option<OriginalPublicationFamily>,pub before:RetainedCloneProgress,pub posted:RetainedCloneProgress,pub grant:RetainedCloneGrant,pub recipient:usize}

#[cfg(test)]
#[path="../../🎞️command/📥️custody/🧪️tests/🦀️.rs"]
mod original_media_command_custody_tests;
