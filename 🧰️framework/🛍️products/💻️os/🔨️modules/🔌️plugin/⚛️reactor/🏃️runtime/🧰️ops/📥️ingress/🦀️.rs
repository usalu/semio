//! 📥️ Opening relays, command ingress, the exchange output and the custody manifest receiving.

use semio_plugin_decl_emission::ActionMeta;
use semio_plugin_decl_app::{ArtifactApp, MediaArtifactError};
use semio_plugin_decl_media::{ArtifactMediaExportHandle, ArtifactMediaExportPoll};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework::kernel::{Effect, InvocationResult};
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, ViewModel};
use semio_framework::manifest::{CommandAddress as ManifestCommandAddress, CommandOwnerAddress as ManifestCommandOwnerAddress};
use semio_plugin_plugin_program::{Plugin, PluginProgram, decode_wire_serialized, encode_wire_serialized};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_job::{TransactionProposalDraft, TypedOperationResultPage};
use serde_json::Value;
use crate::ops_typed_operations::drive_self_waking_ready;
pub use semio_plugin_reactor_lifecycle::instance_lifetime::OriginalCommandOrigin;
use crate::ops_wire_limits::find_instance;
use semio_plugin_runtime_instances::inst_wire_frames::{instance_actor, with_instances_turn_mut};
use semio_plugin_reactor_app::app_runtime_helpers::plugin_internal_fault;
#[cfg(test)]
use std::mem::size_of;
fn funded<T,E>(result:Result<T,E>)->semio_framework_value::RetainedWorkOutcome<T,E>{let work=u64::from(result.is_ok());semio_framework_value::RetainedWorkOutcome{result,work}}
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
pub async fn relay_open_artifact(artifact_ref: &str, role_wire: u8, plugin_id: &str, app_id: &str) -> Result<Effect, Fault> {
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
pub async fn relay_set_default_app(artifact_kind: &str, standard: &str, subset: &str, role_wire: u8, plugin_id: &str, app_id: &str) -> Result<Effect, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    let dialect = opening_dialect(artifact_kind.into(), standard.into(), subset.into()).await?;
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
pub async fn relay_clear_default_app(artifact_kind: &str, standard: &str, subset: &str, role_wire: u8) -> Result<Effect, Fault> {
    opening_dialect(artifact_kind.into(), standard.into(), subset.into()).await?;
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

fn admit_place<T:semio_framework_job::OriginalDomainFailure>(place:&mut semio_framework_job::OriginalDomainReservation<T>,pending:&mut Option<T>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::OriginalTurnFailure>,semio_framework_job::OriginalTurnFailure>{if !place.ready(){place.prepare(cx)?;return Ok(None)}if !place.pending(){if pending.is_none(){return Ok(None)}if !place.capture_original(cx,||pending.take().expect("retained original cause precedes capture"))?{return Ok(None)}}semio_framework_job::admit_original_domain_failure(place,cx)}
pub enum PluginCommandIngress {
    Encoded(semio_framework::kernel::PagedCommand),
    Decoding(OriginalCommandDecode),
    Decoded(OriginalDecodedPluginCommand),
    RetiringDecoded(OriginalDecodedPluginCommand),
    Closing { cursor: protocol::PagedAppCommandDecodeCursor, fault: Fault },
    ClosingDecoded { owner: OriginalDecodedPluginCommand, fault: Fault },
    FailedDecoded{owner:OriginalDecodedPluginCommand,failure:Option<semio_framework_job::OriginalTurnFailure>,identity:(semio_framework_job::OperationId,semio_framework_job::Generation),close_failure:Option<semio_framework_job::OriginalTurnFailure>,compound:Option<semio_framework_job::OriginalClosureFailure>,raw_media:Option<MediaArtifactError>},
}
pub struct OriginalCommandDecode{cursor:protocol::PagedAppCommandDecodeCursor,output:Option<protocol::DecodedAppCommandOwner>,failure:Option<semio_framework_job::OriginalTurnFailure>,closing:bool}
enum OriginalCommandDecodeStep{Pending,Ready(protocol::DecodedAppCommandOwner),Refused(semio_framework_job::OriginalTurnFailure),Failed(semio_framework_job::OriginalTurnFailure),Closed}
impl OriginalCommandDecode{
 fn new(cursor:protocol::PagedAppCommandDecodeCursor)->Self{Self{cursor,output:None,failure:None,closing:false}}
 fn save_failure(&mut self,failure:semio_framework_job::OriginalTurnFailure){assert!(self.failure.is_none(),"original decoder retains its first full cause");self.failure=Some(failure);self.closing=true}
 fn step_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->OriginalCommandDecodeStep{
  if !self.closing&&self.output.is_none(){match self.cursor.step_original(&mut self.output,cx){Ok(_)=>{},Err(failure)=>{self.closing=true;return OriginalCommandDecodeStep::Refused(failure)}}return OriginalCommandDecodeStep::Pending}
  if self.closing{if let Some(output)=self.output.as_mut(){match cx.with_original_retirement(|cx|output.close_original_step(cx)){Ok(false)=>return OriginalCommandDecodeStep::Pending,Err(failure)=>return OriginalCommandDecodeStep::Refused(failure),Ok(true)=>{}}let bytes=std::mem::size_of::<Option<protocol::DecodedAppCommandOwner>>();match original_ingress_metadata(cx,bytes){Ok(true)=>{},Ok(false)=>return OriginalCommandDecodeStep::Pending,Err(failure)=>return OriginalCommandDecodeStep::Refused(failure)}self.output.take();return OriginalCommandDecodeStep::Pending}}
  match cx.with_original_retirement(|cx|self.cursor.close_original_step(cx)){Ok(false)=>return OriginalCommandDecodeStep::Pending,Err(failure)=>return OriginalCommandDecodeStep::Refused(failure),Ok(true)=>{}}
  if self.closing{if self.failure.is_none(){return OriginalCommandDecodeStep::Closed}match original_ingress_metadata(cx,std::mem::size_of::<Option<semio_framework_job::OriginalTurnFailure>>()){Ok(true)=>{},Ok(false)=>return OriginalCommandDecodeStep::Pending,Err(failure)=>return OriginalCommandDecodeStep::Refused(failure)}return OriginalCommandDecodeStep::Failed(self.failure.take().expect("original decoder closes before returning its full cause"))}
  match original_ingress_metadata(cx,std::mem::size_of::<Option<protocol::DecodedAppCommandOwner>>()){Ok(true)=>{},Ok(false)=>return OriginalCommandDecodeStep::Pending,Err(failure)=>return OriginalCommandDecodeStep::Refused(failure)}OriginalCommandDecodeStep::Ready(self.output.take().expect("actual original cursor retained its output until close"))
 }
}
fn original_ingress_metadata(cx:&mut semio_framework_job::StepContext<'_>,bytes:usize)->Result<bool,semio_framework_job::OriginalTurnFailure>{let grant=cx.retained_grant();if cx.retirement_should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
#[derive(semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub struct OriginalCommandExchangeState{
 pub frames:Vec<protocol::AppFrame>,pub effects:Vec<Vec<u8>>,pub events:Vec<Vec<u8>>,
 pub frame_bytes:Vec<Vec<u8>>,pub pending_effects:Option<Vec<Effect>>,
 pub dispatch:Option<protocol::AppCommand>,
 pub manifest:Option<(u64,InvocationResult)>,pub preview:Option<(InvocationResult,semio_plugin_job::EmitWire)>,pub configuration:Option<InvocationResult>,
 pub archive_admission:semio_plugin_document_media::OriginalDocumentArchiveAdmission,
 pub media_artifact:Option<semio_plugin_decl_app::MediaArtifact>,pub media_input:Option<(protocol::AppCommand,semio_plugin_decl_app::MediaArtifactDescriptor)>,
 pub typed_output:Option<PluginExchangeOutput>,pub typed_leftover:Option<semio_plugin_job::TypedOperationLeftover>,
 pub applied:bool,pub mutated:bool,pub started_operation:Option<u64>,pub effects_done:bool,pub typed_done:bool,pub snapshots_done:bool,
}
impl OriginalCommandExchangeState{
 pub const fn empty()->Self{Self{frames:Vec::new(),effects:Vec::new(),events:Vec::new(),frame_bytes:Vec::new(),pending_effects:None,dispatch:None,manifest:None,preview:None,configuration:None,archive_admission:semio_plugin_document_media::OriginalDocumentArchiveAdmission::empty(),media_artifact:None,media_input:None,typed_output:None,typed_leftover:None,applied:false,mutated:false,started_operation:None,effects_done:false,typed_done:false,snapshots_done:false}}
}
#[doc(hidden)]
pub enum OriginalCommandIngressRetirement{Refused{owner:PluginCommandIngress,failure:semio_framework_job::OriginalTurnFailure},Pending(PluginCommandIngress),Failed(semio_framework_job::OriginalTurnFailure),Closed}
#[doc(hidden)]
pub enum PluginCommandIngressStep {
    Refused{owner:PluginCommandIngress,failure:semio_framework_job::OriginalTurnFailure},
    Closed,
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
    pub fn retire_original(self,cx:&mut semio_framework_job::StepContext<'_>)->OriginalCommandIngressRetirement{let transition=matches!(&self,Self::Encoded(_)|Self::Decoded(_))||matches!(&self,Self::Decoding(decode)if !decode.closing);let progress=RetainedCloneProgress{copied_items:usize::from(transition),copied_bytes:if transition{std::mem::size_of::<Self>()}else{0},..Default::default()};if transition{if cx.retirement_should_yield()||cx.fuel_remaining()==0||cx.retained_grant().maximum_depth==0||!progress.fits(cx.retained_grant()){return OriginalCommandIngressRetirement::Pending(self)}if let Err(error)=cx.retained_progress().checked_add(progress){return OriginalCommandIngressRetirement::Refused{owner:self,failure:error.into()}}}let closing=match self{Self::Encoded(command)=>{let mut decode=OriginalCommandDecode::new(protocol::PagedAppCommandDecodeCursor::new(command));decode.closing=true;Self::Decoding(decode)},Self::Decoding(mut decode)=>{decode.closing=true;Self::Decoding(decode)},Self::Decoded(owner)=>Self::RetiringDecoded(owner),owner=>owner};if transition{let posted=cx.consume_retained(progress);cx.consume_fuel(1);if let Err(error)=posted{return OriginalCommandIngressRetirement::Refused{owner:closing,failure:error.into()}}}match closing.advance_original(1,cx){PluginCommandIngressStep::Refused{owner,failure}=>OriginalCommandIngressRetirement::Refused{owner,failure},PluginCommandIngressStep::Pending(owner)=>OriginalCommandIngressRetirement::Pending(owner),PluginCommandIngressStep::TerminalOriginalFailure(failure)=>OriginalCommandIngressRetirement::Failed(failure),PluginCommandIngressStep::TerminalFault(fault)=>OriginalCommandIngressRetirement::Failed(fault.into()),PluginCommandIngressStep::Ready(owner)=>OriginalCommandIngressRetirement::Pending(Self::RetiringDecoded(owner)),PluginCommandIngressStep::Closed=>OriginalCommandIngressRetirement::Closed}}
    pub fn retire_failed_step_on_original(self,cx:&mut semio_framework_job::StepContext<'_>)->OriginalCommandIngressRetirement{
        if !matches!(self,Self::FailedDecoded{..}){return OriginalCommandIngressRetirement::Pending(self)}
        match self.advance_original(1,cx){PluginCommandIngressStep::Refused{owner,failure}=>OriginalCommandIngressRetirement::Refused{owner,failure},PluginCommandIngressStep::Pending(owner)=>OriginalCommandIngressRetirement::Pending(owner),PluginCommandIngressStep::TerminalOriginalFailure(failure)=>OriginalCommandIngressRetirement::Failed(failure),_=>unreachable!("failed original source only closes and returns its full original cause")}
    }

    pub fn advance_original(self,moves:usize,cx:&mut semio_framework_job::StepContext<'_>)->PluginCommandIngressStep{
        if let Self::FailedDecoded{mut owner,mut failure,identity,mut close_failure,mut compound,mut raw_media}=self{
            if identity!=(cx.operation(),cx.generation())||cx.retirement_should_yield(){return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})}
            if let Some(actual)=close_failure.take(){return PluginCommandIngressStep::Refused{owner:Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media},failure:actual}}
            if raw_media.is_some()||!owner.media_place.terminal_is_empty(){
                match cx.with_original_retirement(|cx|admit_place(&mut owner.media_place,&mut raw_media,cx)){Ok(Some(original))=>{if failure.is_none(){failure=Some(original)}else{close_failure=Some(original)}},Ok(None)=>{},Err(actual)=>return PluginCommandIngressStep::Refused{owner:Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media},failure:actual}}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            if compound.is_some()||!owner.compound_place.terminal_is_empty(){
                match cx.with_original_retirement(|cx|admit_place(&mut owner.compound_place,&mut compound,cx)){Ok(Some(original))=>{if failure.is_none(){failure=Some(original)}else{close_failure=Some(original)}},Ok(None)=>{},Err(actual)=>return PluginCommandIngressStep::Refused{owner:Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media},failure:actual}}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            let grant=cx.retained_grant();
            if !owner.terminal_is_empty(){
                match owner.close_original_step(cx){Ok(_)=>{},Err(actual)=>return PluginCommandIngressStep::Refused{owner:Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media},failure:actual}}
                return PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
            }
            if grant.maximum_items>0&&grant.maximum_depth>0{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()}).expect("original failure donation was preflighted");cx.consume_fuel(1);return PluginCommandIngressStep::TerminalOriginalFailure(failure.take().expect("failed decoded initial source remains original"))}
            PluginCommandIngressStep::Pending(Self::FailedDecoded{owner,failure,identity,close_failure,compound,raw_media})
        }else{let mut current=self;for _ in 0..moves{current=match current{
            Self::Encoded(command)=>{let bytes=std::mem::size_of::<Self>();let progress=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};if cx.should_yield()||cx.is_cancelled()||cx.fuel_remaining()==0||cx.retained_grant().maximum_depth==0||!progress.fits(cx.retained_grant()){return PluginCommandIngressStep::Pending(Self::Encoded(command))}if let Err(error)=cx.retained_progress().checked_add(progress){return PluginCommandIngressStep::Refused{owner:Self::Encoded(command),failure:error.into()}}let next=Self::Decoding(OriginalCommandDecode::new(protocol::PagedAppCommandDecodeCursor::new(command)));let posted=cx.consume_retained(progress);cx.consume_fuel(1);if let Err(error)=posted{return PluginCommandIngressStep::Refused{owner:next,failure:error.into()}}next},
            Self::Decoding(mut decode)=>match decode.step_original(cx){OriginalCommandDecodeStep::Refused(failure)=>return PluginCommandIngressStep::Refused{owner:Self::Decoding(decode),failure},OriginalCommandDecodeStep::Pending=>Self::Decoding(decode),OriginalCommandDecodeStep::Ready(wire)=>return PluginCommandIngressStep::Ready(OriginalDecodedPluginCommand::from_wire(wire)),OriginalCommandDecodeStep::Failed(failure)=>return PluginCommandIngressStep::TerminalOriginalFailure(failure),OriginalCommandDecodeStep::Closed=>return PluginCommandIngressStep::Closed},
            Self::Decoded(owner)=>return PluginCommandIngressStep::Ready(owner),
            Self::RetiringDecoded(mut owner)=>match cx.with_original_retirement(|cx|owner.close_original_step(cx)){Ok(true)=>return PluginCommandIngressStep::Closed,Ok(false)=>Self::RetiringDecoded(owner),Err(failure)=>Self::FailedDecoded{owner,failure:Some(failure),identity:(cx.operation(),cx.generation()),close_failure:None,compound:None,raw_media:None}},
            Self::Closing{cursor,fault}=>{let mut decode=OriginalCommandDecode::new(cursor);decode.save_failure(fault.into());Self::Decoding(decode)},
            Self::ClosingDecoded{mut owner,fault}=>match cx.with_original_retirement(|cx|owner.close_original_step(cx)){Ok(true)=>return PluginCommandIngressStep::TerminalFault(fault),Ok(false)=>Self::ClosingDecoded{owner,fault},Err(failure)=>Self::FailedDecoded{owner,failure:Some(fault.into()),identity:(cx.operation(),cx.generation()),close_failure:Some(failure),compound:None,raw_media:None}},
            failed@Self::FailedDecoded{..}=>return failed.advance_original(1,cx),
        };if cx.retirement_should_yield(){break}}PluginCommandIngressStep::Pending(current)}
    }
    /// 📥️ Drives the owner until it is dispatchable, terminal, or the turn's move grant is spent.
    #[cfg(test)]
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
    #[cfg(test)]
    pub fn retire_step(self) -> Option<Self> {
        if matches!(self,Self::FailedDecoded{..}){return Some(self)}
        match self.cancel(plugin_internal_fault("command lifetime is closing")).step() {
            PluginCommandIngressStep::Pending(owner) => Some(owner),
            PluginCommandIngressStep::TerminalFault(_) => None,
            PluginCommandIngressStep::Refused{..}=>unreachable!("ordinary fixture retirement cannot invoke an original refusal"),PluginCommandIngressStep::Closed=>None,
            PluginCommandIngressStep::TerminalOriginalFailure(_)=>unreachable!("whole failed decoded sources require their original retirement caller"),
            PluginCommandIngressStep::Ready(_) => unreachable!("cancelled ingress cannot dispatch"),
        }
    }
    #[cfg(test)]
    pub fn cancel(self, fault: Fault) -> Self {
        match self {
            Self::Encoded(command) => Self::Closing { cursor: protocol::PagedAppCommandDecodeCursor::new(command), fault },
            Self::Decoding(mut decode)=>{decode.save_failure(fault.into());Self::Decoding(decode)},
            Self::Closing { cursor, .. } => Self::Closing { cursor, fault },
            Self::Decoded(owner) | Self::ClosingDecoded { owner, .. } => match owner.receiving.as_ref(){Some(receiving)=>Self::FailedDecoded{identity:receiving.identity(),owner,failure:Some(fault.into()),close_failure:None,compound:None,raw_media:None},None=>Self::ClosingDecoded { owner, fault }},
            original @ Self::FailedDecoded{..}=>original,
            original @ Self::RetiringDecoded(_)=>original,
        }
    }
    #[cfg(test)]
    pub fn step(self) -> PluginCommandIngressStep {
        match self {
            Self::Encoded(encoded) => {
                let cursor = protocol::PagedAppCommandDecodeCursor::new(encoded);
                if cursor.kind() == 28 {
                    PluginCommandIngressStep::Pending(Self::Closing { cursor, fault: plugin_internal_fault("Presence commands require the reserved presence ingress authority") })
                } else {
                    PluginCommandIngressStep::Pending(Self::Decoding(OriginalCommandDecode::new(cursor)))
                }
            }
            Self::Decoding(mut decode) => match decode.cursor.step() {
                Ok(Some(command)) => PluginCommandIngressStep::Pending(Self::Decoded(OriginalDecodedPluginCommand::new(command))),
                Ok(None) => PluginCommandIngressStep::Pending(Self::Decoding(decode)),
                Err(fault) => PluginCommandIngressStep::Pending(Self::Closing { cursor:decode.cursor, fault }),
            },
            Self::Decoded(owner) => PluginCommandIngressStep::Ready(owner),
            original @ Self::FailedDecoded{..}=>PluginCommandIngressStep::Pending(original),
            original @ Self::RetiringDecoded(_)=>PluginCommandIngressStep::Pending(original),
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
#[derive(Default,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub struct PluginExchangeOutput {
    pub original_receiving:Option<OriginalPublicationReceiving>,
    pub constructor_receipt: Option<(semio_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorKind, semio_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)>,
    pub preparation_receipt: Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)>,
    pub retirement_receipt: Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)>,
    pub history_command_receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    pub tool_run_receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    pub frames: Vec<Vec<u8>>,
    pub effects: Vec<Vec<u8>>,
    pub events: Vec<Vec<u8>>,
    pub command_terminal_fault: Option<Vec<u8>>,
    pub presence_pending: Option<u64>,
    pub presence_terminal: Option<u64>,
    pub presence_terminal_fault: Option<Vec<u8>>,
    pub typed_operation_pages: usize,
}
/// 🏷️ Names the one actual producer whose full physical receipt was posted in the initiating context.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalPublicationFamily{Constructor,Preparation,Retirement,History,Tool}
semio_framework_value::artifact_retire_leaf!(OriginalPublicationFamily);
/// 🧾️ Captures actual already-posted effects, exact initiating grant and recipient before another loan.
#[derive(Clone,Copy)]
pub struct OriginalPublicationReceiving{pub family:Option<OriginalPublicationFamily>,pub before:RetainedCloneProgress,pub posted:RetainedCloneProgress,pub grant:RetainedCloneGrant,pub recipient:usize}
semio_framework_value::artifact_retire_leaf!(OriginalPublicationReceiving);

#[cfg(test)]
#[path="../../🎞️command/📥️custody/🧪️tests/🦀️.rs"]
mod original_media_command_custody_tests;
