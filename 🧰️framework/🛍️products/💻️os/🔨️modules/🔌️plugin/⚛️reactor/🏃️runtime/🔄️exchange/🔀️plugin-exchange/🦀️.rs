//! 🔀️ The plugin exchange dispatcher: one function over every command, action and operation.

use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_decl_app::{ArtifactApp, EphemeralSnapshot};
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportPoll;
use semio_framework_plugin_runtime_ops::ops_ingress::{COMMAND_INGRESS_MOVES_PER_TURN, PluginCommandIngress, PluginCommandIngressStep, PluginExchangeOutput, encode_exchange_frames, push_invocation_side_frames, relay_clear_default_app, relay_open_artifact, relay_set_default_app, transaction_proposal_frame};
use semio_framework_plugin_runtime_ops::ops_surface_render::ContextMenuWireRequest;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework_plugin_job::EmitWire;
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
use semio_framework_plugin_plugin_program::{Plugin, PluginProgram, decode_wire_serialized, encode_wire_serialized};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_framework_plugin_runtime_ops::ops_typed_operations::{TYPED_OPERATION_REPLY_LEFTOVER_PUMPS, advance_typed_operation_output, advance_typed_operation_output_with_leftover, drive_self_waking_ready, media_export_handle_from_wire, media_export_handle_into_wire, merge_typed_operation_leftover_into_reply};
use semio_framework_plugin_window_config::WindowConfigPack;
use semio_framework_plugin_reactor_app::app_runtime_helpers::{debug_runtime_line, plugin_internal_fault};
use semio_framework_plugin_runtime_instances::inst_wire_frames::{decode_wire_serialized_or, instance_actor, push_app_fault, push_dispatch_fault, push_os_fault, read_conflicts_frames, resolve_conflict_frames, set_merge_policy_frames, with_instances_mut, with_instances_original_mut};
use semio_framework_plugin_runtime_ops::ops_wire_limits::find_instance;
use semio_framework_plugin_runtime_ops::ops_media::{plugin_cancel_media_export, plugin_consume_media, plugin_poll_media_export, plugin_produce_media, plugin_retain_media_export_result, plugin_submit_media_export, plugin_take_media_export_chunk};
use semio_framework_plugin_runtime_ops::ops_document::{plugin_document_pack, plugin_ingest_operations};

#[path = "🎯️document-and-child-commands/🦀️.rs"]
mod document_and_child_commands;
use document_and_child_commands::{arm_local_interaction_query, arm_context_menu, arm_apply_envelopes, arm_load_children, arm_read_children, arm_read_child_heads, arm_read_history};

#[path = "📦️document-archive-commands/🦀️.rs"]
mod document_archive_commands;
use document_archive_commands::{arm_load_document_archive, arm_merge_document_archive, arm_poll_document_archive_load, arm_cancel_document_archive_load, arm_acknowledge_document_archive_load, arm_read_document_identity, arm_read_document_archive};

#[path = "⚙️config-commands/🦀️.rs"]
mod config_commands;
use config_commands::{arm_load_config, arm_read_config, arm_load_window_config, arm_read_window_configs};

#[path = "🎞️media-commands/🦀️.rs"]
mod media_commands;
use media_commands::{arm_media_in, arm_submit_media_export, arm_poll_media_export, arm_cancel_media_export, arm_media_fingerprint};

#[path = "🔁️transaction-commands/🦀️.rs"]
mod transaction_commands;
use transaction_commands::{arm_transaction_prepare, arm_transaction_commit, arm_transaction_rollback, arm_transaction_undo, arm_transaction_redo};

#[path = "🌐️relay-and-conflict-commands/🦀️.rs"]
mod relay_and_conflict_commands;
use relay_and_conflict_commands::{arm_open_artifact, arm_set_default_app, arm_clear_default_app, arm_set_merge_policy, arm_resolve_conflict, arm_read_conflicts};

#[path = "🏁️exchange-finish/🦀️.rs"]
mod exchange_finish;
use exchange_finish::finish_exchange;

/// 🧾️ The two facts the command arms of one exchange record for its tail: whether the document mutated and which typed operation the command started.
#[derive(Default)]
struct ExchangeFlags {
    mutated: bool,
    started_typed_operation: Option<u64>,
}

/// 🕹️ The typed operation an `AppCommand::Command` of this exchange STARTED (its reply's
/// `output.operationId`) — the operation whose inline interaction fold this exchange still
/// tries to land before the reply is sealed (`TYPED_OPERATION_REPLY_LEFTOVER_PUMPS`).
///
/// 🧮️ B1: `command` is a binary-encoded `store::os_store::store_schema::ArtifactCommand<A::ConfigMutation>` —
/// real dispatch against the config store (replaces the deleted `apply_config_bytes`
/// whole-record-replace legacy path); undo/redo/checkpoint all work on config now.
///
/// 🔀️ Contract §5.1: a gesture whose mutations carried foreign steps
/// applied nothing (`dispatch_emit` stashed a proposal instead) — frame
/// `AppFrame::TransactionProposal` in place of the ordinary
/// `Invocation` frame whenever one is pending.
///
/// 🚧️ Wave 1 stub — see this function's doc comment.
///
/// 🗂️ Decodes straight into the typed wire shape and encodes the typed response straight
/// back out — no intermediate `Value`/JSON-string hop through `plugin_context_menu`
/// (which stays as the separate string-in/string-out entry point the WIT boundary needs).
///
/// 🔀️ Same proposal check as `AppCommand::Command` above — the six
/// history verbs never carry foreign steps in practice, but
/// `dispatch_emit` is the shared choke point, so this arm is covered too.
///
/// ⚔️ Pushed unsolicited after every ingest, next to `DocumentChanged`
/// (contract §C8): one `MergeReport` per ingested envelope, then the
/// authoritative open-conflict projection once the whole batch has landed.
///
/// 🧸️ Composed children are their own envelopes with their own histories, so they
/// need their own load/read pair — a parent's own pack+spr (`AppFrame::Document`) carries none
/// of them, and before this existed a genesis child lived only until the process
/// ended.
///
/// 🌉️ `with_instances_mut`'s callback is deliberately sync (single-threaded
/// `RefCell::borrow_mut` reentrancy guard, see its own doc comment) —
/// bridged via `resolve_ready` exactly like `media_fingerprint` above.
///
/// 🧮️ B1: real work — loads straight into the config `ArtifactStore` (was routed
/// through the deleted `apply_config_bytes` whole-record-replace legacy path).
///
/// 🧮️ B1: real work — the config store's current pack+spr+ops (was a stub always
/// returning empty bytes).
///
/// 🔀️ Member state machine (contract §5.3-§5.10) — see `📓️w1-b-report.md` for the
/// full frame/rejection-code table.
///
/// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the operation this command started owes
/// its inline interaction fold's leftover `InteractionView` to THIS reply (`output` is the only
/// lane `applyLeftoverInteractionView` reads for a typed command), so the ladder is driven a
/// few more bounded units here — exactly the units the reactor's own continuation would drive
/// next — until that fold lands, a result page is presented (the host must ACK it first),
/// nothing is runnable, or the bound is spent. A fold that lands later still rides the `Ui`
/// progress frame's `output`.
///
/// 🌉️ Rewritten from a `.map(protocol::encode_app_frame).collect()` into an explicit loop
/// (sync — `Iterator::map` cannot take an async closure, and `encode_app_frame` is genuinely async).
pub async fn plugin_exchange<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, command: Option<(u64, PluginCommandIngress)>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginExchangeOutput, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    debug_runtime_line(format_args!("[TRACE] plugin_exchange entry instance={instance_id} command={}", command.is_some()));
    let constructing = with_instances_mut(runtime, |list| { let instance = find_instance(list, instance_id)?; Ok(instance.app.store_constructor_is_pending()) }).await?;
    if constructing {
        let mut output = with_instances_original_mut(runtime, |list| { let mut instance = find_instance(list, instance_id)?; let cell=instance.cell;let mut pages=match cell.result_pages.try_lock(){Ok(pages)=>pages,Err(std::sync::TryLockError::WouldBlock)=>return Ok(PluginExchangeOutput::default()),Err(std::sync::TryLockError::Poisoned(_))=>return Err(plugin_internal_fault("original result receiving authority is poisoned").into())}; advance_typed_operation_output(&mut instance.app, instance_id, &mut pages, identity,cx) }).await?;
        output.retry_command = command;
        return Ok(output);
    }
    let mut frames: Vec<protocol::AppFrame> = Vec::new();
    let mut effect_bytes: Vec<Vec<u8>> = Vec::new();
    let mut event_bytes: Vec<Vec<u8>> = Vec::new();
    let retry_command = None;
    let mut command_terminal_fault = None;
    let presence_pending = None;
    let mut presence_terminal = None;
    let mut presence_terminal_fault = None;
    let mut flags = ExchangeFlags::default();
    let command = match command {
        Some((envelope_seq, ingress)) => match ingress.advance_original(COMMAND_INGRESS_MOVES_PER_TURN,cx) {
            PluginCommandIngressStep::Pending(ingress) => {
                return Ok(PluginExchangeOutput {
                    original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
                    frames: Vec::new(),
                    effects: effect_bytes,
                    events: event_bytes,
                    retry_command: Some((envelope_seq, ingress)),
                    command_terminal_fault,
                    presence_pending,
                    presence_terminal,
                    presence_terminal_fault,
                    typed_operation_pages: 0,
                });
            }
            PluginCommandIngressStep::Ready(owner) => Some((envelope_seq, owner)),
            PluginCommandIngressStep::TerminalOriginalFailure(failure)=>return Err(failure.into()),
            PluginCommandIngressStep::TerminalFault(fault) => {
                command_terminal_fault = Some(semio_framework_diagnostic::encode_fault_bytes(&fault));
                push_app_fault(&mut frames, Some(envelope_seq), fault).await;
                return Ok(PluginExchangeOutput {
                    original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
                    frames: encode_exchange_frames(&frames).await,
                    effects: effect_bytes,
                    events: event_bytes,
                    retry_command,
                    command_terminal_fault,
                    presence_pending,
                    presence_terminal,
                    presence_terminal_fault,
                    typed_operation_pages: 0,
                });
            }
        },
        None => None,
    };
    let presence_outcome = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        Ok(instance.app.take_presence_outcome())
    })
    .await;
    let presence_outcome = match presence_outcome {
        Ok(outcome) => outcome,
        Err(fault) => {
            if let Some((envelope_seq, owner)) = command {
                let ingress = PluginCommandIngress::Decoded(owner);
                let ingress = if fault.message == "runtime instance authority is busy" { ingress } else { ingress.cancel(fault) };
                return Ok(PluginExchangeOutput {
                    original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
                    frames: Vec::new(),
                    effects: effect_bytes,
                    events: event_bytes,
                    retry_command: Some((envelope_seq, ingress)),
                    command_terminal_fault,
                    presence_pending,
                    presence_terminal,
                    presence_terminal_fault,
                    typed_operation_pages: 0,
                });
            }
            return Err(fault.into());
        }
    };
    if let Some(outcome) = presence_outcome {
        presence_terminal = Some(outcome.seq);
        match outcome.fault {
            Some(fault) => {
                presence_terminal_fault = Some(semio_framework_diagnostic::encode_fault_bytes(&fault));
                push_app_fault(&mut frames, Some(outcome.seq), fault).await;
            }
            None => frames.push(protocol::AppFrame::Done { in_reply_to: outcome.seq }),
        }
    }
    if let Some((envelope_seq, mut owner)) = command {
        let manifest_result=if owner.command_seq.is_some(){
            match owner.advance_manifest_step(runtime,instance_id,identity,cx).await{
                Ok(Some(result))=>Some(result),
                result=>{let ingress=match result{Ok(None)=>PluginCommandIngress::Decoded(owner),Err(failure)=>{let identity=owner.receiving.as_ref().map_or((cx.operation(),cx.generation()),|owner|owner.identity());PluginCommandIngress::FailedDecoded{owner,failure:Some(failure),identity,close_failure:None,compound:None,raw_media:None}},Ok(Some(_))=>unreachable!()};return Ok(PluginExchangeOutput{frames:encode_exchange_frames(&frames).await,effects:effect_bytes,events:event_bytes,retry_command:Some((envelope_seq,ingress)),command_terminal_fault,presence_pending,presence_terminal,presence_terminal_fault,..Default::default()})}
            }
        }else{None};
        if let Some((seq,result))=manifest_result{
                        let (proposal, report) = with_instances_mut(runtime, |list| {
                            let mut instance = find_instance(list, instance_id)?;
                            Ok((::semio_framework_async::poll::resolve_ready(instance.app.take_pending_transaction_proposal()), ::semio_framework_async::poll::resolve_ready(instance.app.dispatch_report()).clone()))
                        })
                        .await?;
                        if let Some(proposal) = proposal {
                            frames.push(transaction_proposal_frame(instance_id, seq, proposal).await);
                            push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
                        } else {
                            let output = encode_wire_serialized(&result.output);
                            let diagnostics = encode_wire_serialized(&result.diagnostics);
                            let mutations = encode_wire_serialized(&result.mutations);
                            let inverse_group = encode_wire_serialized(&result.inverse_group);
                            if mutations.len() > ::replication::wire::command_ingress::INVOCATION_RESULT_PACK_MAXIMUM_BYTES || inverse_group.len() > ::replication::wire::command_ingress::INVOCATION_RESULT_PACK_MAXIMUM_BYTES {
                                push_dispatch_fault(&mut frames, seq, Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.invocation-result-capacity"), "packed mutations or inverse group exceed command transport authority"), &report).await;
                            } else {
                                flags.mutated = true;
                                flags.started_typed_operation = result.output.get("operationId").and_then(DslValue::as_str).and_then(|operation| operation.parse::<u64>().ok());
                                frames.push(protocol::AppFrame::Invocation {
                                    in_reply_to: seq,
                                    output,
                                    diagnostics,
                                    ui_scope: encode_wire_serialized(&result.ui_scope),
                                    history_patch: encode_wire_serialized(&result.history_patch),
                                    messages: encode_wire_serialized(&report),
                                    mutations,
                                    inverse_group,
                                });
                                push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
                            }
                        }
        }else{
        let mut original_media_submission_ready=false;
        let mut original_media_poll_ready=false;
        let mut original_media_failure:Option<semio_framework_job::OriginalTurnFailure>=None;
        let mut original_media_domain=None;
        let mut original_media_artifact=None;
        let mut original_preview=None;
        let mut original_configuration=None;
        let media_pending=match owner.original_command(){
            Some(protocol::AppCommand::MediaIn{..})=>{
                let result=with_instances_original_mut(runtime,|list|{let mut instance=find_instance(list,instance_id)?;::semio_framework_async::poll::resolve_ready(instance.app.prepare_media_import(cx)).map_err(Into::into)}).await;
                match result{Ok(ready)=>!ready,Err(failure)=>{original_media_failure=Some(failure.into());true}}
            },
            Some(protocol::AppCommand::SubmitMediaExport{..})=>{
                let(port,parent,revision,checkpoint,receiver)=owner.media_submission_parts()?;
                match plugin_submit_media_export(runtime,instance_id,port,parent,revision,checkpoint,receiver,cx).await{Ok(ready)=>original_media_submission_ready=ready,Err(failure)=>original_media_failure=Some(failure.into())};
                !original_media_submission_ready
            },
            Some(protocol::AppCommand::PollMediaExport{..})=>{
                let(loan,poll,status)=owner.media_poll_parts()?;
                if poll.is_none()&&status.is_none(){match plugin_poll_media_export(runtime,instance_id,loan,poll,cx).await{Ok(ready)=>original_media_poll_ready=ready,Err(failure)=>original_media_failure=Some(failure)}}else{original_media_poll_ready=true}
                if original_media_failure.is_none()&&original_media_poll_ready&&(status.is_some()||matches!(poll.as_ref(),Some(ArtifactMediaExportPoll::Complete(_)))){match plugin_retain_media_export_result(runtime,instance_id,loan,poll,status,cx).await{Ok(ready)=>original_media_poll_ready=ready,Err(failure)=>{original_media_failure=Some(failure);original_media_poll_ready=false}}}
                !original_media_poll_ready
            },
            Some(protocol::AppCommand::TakeMediaExportChunk{..})=>{
                let(loan,receiver,terminal)=owner.media_chunk_parts()?;match plugin_take_media_export_chunk(runtime,instance_id,loan,receiver,terminal,cx).await{Ok(ready)=>!ready,Err(failure)=>{original_media_failure=Some(failure);true}}
            },
            Some(protocol::AppCommand::MediaOut{port,..})=>{
                let grant=cx.retained_grant();let artifact=identity.encode(|native|{let mut encoder=semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner::new(native,grant);let result=::semio_framework_async::poll::resolve_ready(plugin_produce_media(runtime,instance_id,port,"",&mut encoder,cx));cx.consume_retained(encoder.progress()).map_err(semio_framework_plugin_decl_app::MediaArtifactOperationFailure::from)?;result});
                match artifact{Ok(result)=>original_media_artifact=result,Err(semio_framework_plugin_decl_app::MediaArtifactOperationFailure::Original(failure))=>original_media_failure=Some(failure.into()),Err(semio_framework_plugin_decl_app::MediaArtifactOperationFailure::Domain(source))=>original_media_domain=Some(source)};
                original_media_artifact.is_none()
            },
            Some(protocol::AppCommand::LoadWindowConfig{..})=>{
                let(entry,load)=owner.window_load_parts()?;
                let result=with_instances_original_mut(runtime,|list|{let mut instance=find_instance(list,instance_id)?;instance.app.advance_window_config_load(load,entry,cx).map_err(Into::into)}).await;
                match result{Ok(ready)=>!ready,Err(failure)=>{original_media_failure=Some(failure.into());true}}
            },
            Some(protocol::AppCommand::ConfigCommand{..})=>{
                let meta=ActionMeta{actor:instance_actor(runtime,instance_id).await?.into(),instance_id,view_state:None};let command=owner.configuration_bytes_mut()?;
                let result=with_instances_original_mut(runtime,|list|{let mut instance=find_instance(list,instance_id)?;::semio_framework_async::poll::resolve_ready(instance.app.dispatch_config_command(command,&meta,identity,cx)).map_err(Into::into)}).await;
                match result{Ok(result)=>original_configuration=result,Err(failure)=>original_media_failure=Some(failure.into())}original_configuration.is_none()
            },
            Some(protocol::AppCommand::PureCommand{command,head,..})=>{
                let meta=ActionMeta{actor:instance_actor(runtime,instance_id).await?.into(),instance_id,view_state:None};
                let result=with_instances_original_mut(runtime,|list|{let mut instance=find_instance(list,instance_id)?;if !instance.app.original_command_frame_pending()&&!::semio_framework_async::poll::resolve_ready(instance.app.hydrate_pure_head(head,cx))?{return Ok(None)}let result=::semio_framework_async::poll::resolve_ready(instance.app.handle_command_frame(command,&meta,identity,cx))?;Ok(result.map(|result|(result,::semio_framework_async::poll::resolve_ready(instance.app.take_last_emit_wire()).unwrap_or_default())))}).await;
                match result{Ok(result)=>original_preview=result,Err(failure)=>original_media_failure=Some(failure.into())};original_preview.is_none()
            },
            _=>false,
        };
        if media_pending{
            let ingress=match original_media_failure{Some(failure)=>PluginCommandIngress::FailedDecoded{owner,failure:Some(failure),identity:(cx.operation(),cx.generation()),close_failure:None,compound:None,raw_media:original_media_domain},None=>if original_media_domain.is_some(){PluginCommandIngress::FailedDecoded{owner,failure:None,identity:(cx.operation(),cx.generation()),close_failure:None,compound:None,raw_media:original_media_domain}}else{PluginCommandIngress::Decoded(owner)}};
            return Ok(PluginExchangeOutput{frames:encode_exchange_frames(&frames).await,effects:effect_bytes,events:event_bytes,retry_command:Some((envelope_seq,ingress)),command_terminal_fault,presence_pending,presence_terminal,presence_terminal_fault,..Default::default()});
        }
        let command = owner.take_for_dispatch().ok_or_else(|| plugin_internal_fault("decoded command owner is terminal before dispatch"))?;
        match command {
            protocol::AppCommand::Presence { .. } => unreachable!("Presence commands are admitted from their fixed header before whole-command decoding"),
            protocol::AppCommand::LocalInteractionQuery { .. } => arm_local_interaction_query(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::ConfigCommand{seq,command}=>{
                assert_eq!(command.capacity(),0,"configuration native job must own the exact original raw allocation before dispatch donation");let result=original_configuration.take().expect("original configuration completes before decoded command donation");flags.mutated=true;frames.push(protocol::AppFrame::Done{in_reply_to:seq});push_invocation_side_frames(&mut effect_bytes,&mut event_bytes,&result).await;
            }
            protocol::AppCommand::Command { .. } => unreachable!("original manifest receiver dispatches before source consumption"),
            protocol::AppCommand::CommandText { seq, line: _ } => {
                push_os_fault(&mut frames, Some(seq), "unsupported", "CommandText not yet wired".into()).await;
            }
            protocol::AppCommand::ContextMenu { .. } => arm_context_menu(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::ArtifactCommand { .. } => unreachable!("original artifact receiver dispatches before source consumption"),
            protocol::AppCommand::ApplyEnvelopes { .. } => arm_apply_envelopes(runtime, instance_id, identity, cx, &mut frames, &mut flags, command).await?,
            protocol::AppCommand::ReadDocument { seq } => match plugin_document_pack(runtime, instance_id).await {
                Ok(files) => frames.push(protocol::AppFrame::Document { in_reply_to: seq, pack: files.pack, spr: files.spr, ops: files.ops }),
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::LoadChildren { .. } => arm_load_children(runtime, instance_id, identity, &mut frames, &mut flags, command).await?,
            protocol::AppCommand::ReadChildren { .. } => arm_read_children(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::ReadChildHeads { .. } => arm_read_child_heads(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::LoadDocumentArchive { .. } => arm_load_document_archive(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::MergeDocumentArchive { .. } => arm_merge_document_archive(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::PollDocumentArchiveLoad { .. } => arm_poll_document_archive_load(runtime, instance_id, identity, cx, &mut frames, &mut flags, command).await?,
            protocol::AppCommand::CancelDocumentArchiveLoad { .. } => arm_cancel_document_archive_load(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::AcknowledgeDocumentArchiveLoad { .. } => arm_acknowledge_document_archive_load(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::ReadDocumentIdentity { .. } => arm_read_document_identity(runtime, instance_id, identity, &mut frames, command).await?,
            protocol::AppCommand::ReadDocumentArchive { .. } => arm_read_document_archive(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::ReadHistory { .. } => arm_read_history(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::LoadConfig { .. } => arm_load_config(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::ReadConfig { .. } => arm_read_config(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::LoadWindowConfig { .. } => arm_load_window_config(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::ReadWindowConfigs { .. } => arm_read_window_configs(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::MediaIn { .. } => arm_media_in(runtime, instance_id, cx, &mut frames, &mut flags, command).await?,
            protocol::AppCommand::MediaOut{seq,port,..}=>{let artifact=original_media_artifact.take().expect("original media encoding completes before command donation");frames.push(protocol::AppFrame::Media{in_reply_to:seq,port,descriptor:encode_wire_serialized(&artifact.descriptor),data:artifact.data});},
            protocol::AppCommand::SubmitMediaExport { .. } => arm_submit_media_export(&mut frames, &mut owner, command).await?,
            protocol::AppCommand::PollMediaExport { .. } => arm_poll_media_export(&mut frames, &mut owner, command).await?,
            protocol::AppCommand::CancelMediaExport { .. } => arm_cancel_media_export(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::TakeMediaExportChunk{seq,handle}=>{let(data,terminal)=owner.take_media_chunk();frames.push(protocol::AppFrame::MediaExportChunk{in_reply_to:seq,handle,data:data.unwrap_or_default(),terminal});},
            protocol::AppCommand::MediaFingerprint { .. } => arm_media_fingerprint(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::PureCommand{seq,..}=>{let(result,EmitWire{document:document_ops,config:config_ops,draft:draft_ops,children:child_ops})=original_preview.take().expect("original preview completes before decoded command donation");flags.mutated=true;frames.push(protocol::AppFrame::Emit{in_reply_to:seq,document_ops,config_ops,draft_ops,output:encode_wire_serialized(&result.output),diagnostics:encode_wire_serialized(&result.diagnostics),child_ops});push_invocation_side_frames(&mut effect_bytes,&mut event_bytes,&result).await;},
            protocol::AppCommand::TransactionPrepare { .. } => arm_transaction_prepare(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::TransactionCommit { .. } => arm_transaction_commit(runtime, instance_id, identity, &mut frames, &mut flags, command).await?,
            protocol::AppCommand::TransactionRollback { .. } => arm_transaction_rollback(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::TransactionUndo { .. } => arm_transaction_undo(runtime, instance_id, identity, &mut frames, &mut flags, command).await?,
            protocol::AppCommand::TransactionRedo { .. } => arm_transaction_redo(runtime, instance_id, identity, &mut frames, &mut flags, command).await?,
            protocol::AppCommand::OpenArtifact { .. } => arm_open_artifact(&mut frames, &mut effect_bytes, command).await?,
            protocol::AppCommand::SetDefaultApp { .. } => arm_set_default_app(&mut frames, &mut effect_bytes, command).await?,
            protocol::AppCommand::ClearDefaultApp { .. } => arm_clear_default_app(&mut frames, &mut effect_bytes, command).await?,
            protocol::AppCommand::SetMergePolicy { .. } => arm_set_merge_policy(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::ResolveConflict { .. } => arm_resolve_conflict(runtime, instance_id, &mut frames, command).await?,
            protocol::AppCommand::ReadConflicts { .. } => arm_read_conflicts(runtime, instance_id, &mut frames, command).await?,
        }
        }
    }
    finish_exchange(runtime, instance_id, identity, cx, frames, effect_bytes, event_bytes, retry_command, command_terminal_fault, presence_pending, presence_terminal, presence_terminal_fault, flags).await
}
