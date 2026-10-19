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
        let mut output = with_instances_original_mut(runtime, |list| { let mut instance = find_instance(list, instance_id)?; advance_typed_operation_output(&mut instance.app, instance_id, identity,cx) }).await?;
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
    let mut mutated = false;
    let mut started_typed_operation: Option<u64> = None;
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
                                mutated = true;
                                started_typed_operation = result.output.get("operationId").and_then(DslValue::as_str).and_then(|operation| operation.parse::<u64>().ok());
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
                let(port,parent,revision,receiver)=owner.media_submission_parts()?;
                match plugin_submit_media_export(runtime,instance_id,port,parent,revision,receiver,cx).await{Ok(ready)=>original_media_submission_ready=ready,Err(failure)=>original_media_failure=Some(failure.into())};
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
            protocol::AppCommand::LocalInteractionQuery { seq, command } => {
                frames.try_reserve_exact(2).map_err(|_| plugin_internal_fault("local interaction receipt and rejection slots could not be admitted"))?;
                let reply = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    Ok(match command {
                        protocol::LocalInteractionQueryCommand::Read { request_id } => match runtime.local_interaction_query_generation.next() {
                            Some(generation) => instance.app.begin_local_interaction_query(request_id, generation),
                            None => Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::GenerationExhausted }),
                        },
                        protocol::LocalInteractionQueryCommand::Acknowledge { token } => {
                            instance.app.acknowledge_local_interaction_query(&token);
                            None
                        }
                        protocol::LocalInteractionQueryCommand::Cancel { token } => {
                            instance.app.cancel_local_interaction_query(&token);
                            None
                        }
                    })
                })
                .await?;
                if let Some(reply) = reply {
                    frames.push(protocol::AppFrame::LocalInteractionQuery { reply });
                }
                frames.push(protocol::AppFrame::Done { in_reply_to: seq });
            }
            protocol::AppCommand::ConfigCommand{seq,command}=>{
                assert_eq!(command.capacity(),0,"configuration native job must own the exact original raw allocation before dispatch donation");let result=original_configuration.take().expect("original configuration completes before decoded command donation");mutated=true;frames.push(protocol::AppFrame::Done{in_reply_to:seq});push_invocation_side_frames(&mut effect_bytes,&mut event_bytes,&result).await;
            }
            protocol::AppCommand::Command { .. } => unreachable!("original manifest receiver dispatches before source consumption"),
            protocol::AppCommand::CommandText { seq, line: _ } => {
                push_os_fault(&mut frames, Some(seq), "unsupported", "CommandText not yet wired".into()).await;
            }
            protocol::AppCommand::ContextMenu { seq, request } => match decode_wire_serialized::<ContextMenuWireRequest>(&request).await.and_then(ContextMenuWireRequest::into_parts) {
                Ok((request, view_state)) => {
                    let outcome = with_instances_mut(runtime, |list| {
                        let mut instance = find_instance(list, instance_id)?;
                        Ok(::semio_framework_async::poll::resolve_ready(instance.app.context_menu(&request, &view_state)))
                    });
                    match outcome.await {
                        Ok(items) => frames.push(protocol::AppFrame::ContextMenu { in_reply_to: seq, items: encode_wire_serialized(&items) }),
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                    }
                }
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::ArtifactCommand { .. } => unreachable!("original artifact receiver dispatches before source consumption"),
            protocol::AppCommand::ApplyEnvelopes { seq, envelopes } => {
                let encoded = protocol::encode_envelopes(&envelopes);
                match plugin_ingest_operations(runtime, instance_id, &encoded, identity, cx).await {
                    Ok(reports) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                        for report in &reports {
                            frames.push(protocol::AppFrame::MergeReport { in_reply_to: None, report: encode_wire_serialized(report) });
                        }
                        if !reports.is_empty() {
                            let open: Vec<protocol::Conflict> = with_instances_mut(runtime, |list| {
                                let instance = find_instance(list, instance_id)?;
                                Ok(::semio_framework_async::poll::resolve_ready(instance.app.open_conflicts()))
                            })
                            .await?;
                            frames.push(protocol::AppFrame::Conflicts { in_reply_to: None, conflicts: encode_wire_serialized(&open) });
                        }
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadDocument { seq } => match plugin_document_pack(runtime, instance_id).await {
                Ok(files) => frames.push(protocol::AppFrame::Document { in_reply_to: seq, pack: files.pack, spr: files.spr, ops: files.ops }),
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::LoadChildren { seq, entries } => {
                let loaded = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    for entry in &entries {
                        if !entry.owner.is_empty() {
                            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), format!("child pack {}/{} belongs to member {}: a member's own children load with the document archive", entry.slot, entry.child_id, entry.owner)));
                        }
                        let dialect = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&entry.dialect).map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), error))?;
                        ::semio_framework_async::poll::resolve_ready(instance.app.load_child_pack(&entry.slot, &entry.child_id, dialect, &entry.envelope_pack, identity))?;
                    }
                    Ok(())
                });
                match loaded.await {
                    Ok(()) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadChildren { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.child_packs())
                });
                match read.await {
                    Ok(entries) => frames.push(protocol::AppFrame::Children { in_reply_to: seq, entries }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadChildHeads { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.child_head_packs())
                });
                match read.await {
                    Ok(entries) => frames.push(protocol::AppFrame::ChildHeads { in_reply_to: seq, entries }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::LoadDocumentArchive { seq, archive } => {
                let admitted = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    instance.app.begin_document_archive_load(seq, archive)
                });
                match admitted.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::MergeDocumentArchive { seq, archive } => {
                let admitted = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    instance.app.begin_document_archive_merge(seq, archive)
                });
                match admitted.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::PollDocumentArchiveLoad { seq, operation } => {
                let status = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    drive_self_waking_ready(instance.app.poll_document_archive_load(operation, identity, cx))
                });
                match status.await {
                    Ok(status) => {
                        mutated |= status.state == protocol::DocumentArchiveLoadState::Ready;
                        frames.push(protocol::AppFrame::DocumentArchiveLoad { in_reply_to: seq, status });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::CancelDocumentArchiveLoad { seq, operation } => {
                let cancelled = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    instance.app.cancel_document_archive_load(operation)
                });
                match cancelled.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation } => {
                let acknowledged = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    instance.app.acknowledge_document_archive_load(operation)
                });
                match acknowledged.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadDocumentIdentity { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    Ok(protocol::AppDocumentIdentity { app_instance_id: instance_id, parent_document_id: instance.app.document_identity() })
                });
                match read.await {
                    Ok(identity) => frames.push(protocol::AppFrame::DocumentIdentity { in_reply_to: seq, identity }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadDocumentArchive { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.document_archive())
                });
                match read.await {
                    Ok(archive) => frames.push(protocol::AppFrame::DocumentArchive { in_reply_to: seq, archive }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadHistory { seq } => {
                let snapshot = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.history_snapshot())
                });
                match snapshot.await {
                    Ok(history_patch) => frames.push(protocol::AppFrame::HistorySnapshot { in_reply_to: seq, history_patch: encode_wire_serialized(&history_patch) }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::LoadConfig { seq, pack, spr } => {
                let applied = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.load_config_pack(&store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }))
                });
                match applied.await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadConfig { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.config_pack())
                });
                match read.await {
                    Ok(files) => frames.push(protocol::AppFrame::Config { in_reply_to: seq, pack: files.pack, spr: files.spr, ops: files.ops }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::LoadWindowConfig { seq, entry } => match store::os_store::serialization::decode_document_pack_bytes(&entry.envelope_pack).await {
                Ok((pack, spr)) => {
                    let loaded = with_instances_mut(runtime, |list| {
                        let mut instance = find_instance(list, instance_id)?;
                        ::semio_framework_async::poll::resolve_ready(instance.app.load_window_config_pack(WindowConfigPack {
                            window_id: entry.window_id,
                            window_kind_id: entry.window_kind_id,
                            files: store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() },
                        }))
                    });
                    match loaded.await {
                        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                    }
                }
                Err(error) => push_app_fault(&mut frames, Some(seq), error.into_fault()).await,
            },
            protocol::AppCommand::ReadWindowConfigs { seq } => {
                let read = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.window_config_packs())
                });
                match read.await {
                    Ok(packs) => {
                        let mut entries = Vec::with_capacity(packs.len());
                        for pack in packs {
                            entries.push(protocol::WindowConfigPackEntry { window_id: pack.window_id, window_kind_id: pack.window_kind_id, envelope_pack: store::os_store::serialization::encode_document_pack_bytes(&pack.files.pack, &pack.files.spr).await });
                        }
                        frames.push(protocol::AppFrame::WindowConfigs { in_reply_to: seq, entries });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::MediaIn { seq, port, descriptor, data } => {
                let descriptor_value: DslValue = decode_wire_serialized_or(&descriptor, DslValue::Null).await;
                let descriptor_json = serde_json::to_string(&descriptor_value).unwrap_or_else(|_| "{}".into());
                match plugin_consume_media(runtime, instance_id, seq, &port, &descriptor_json, data, cx).await {
                    Ok(None) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Ok(Some(status)) => frames.push(protocol::AppFrame::DocumentArchiveLoad { in_reply_to: seq, status }),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::MediaOut{seq,port,..}=>{let artifact=original_media_artifact.take().expect("original media encoding completes before command donation");frames.push(protocol::AppFrame::Media{in_reply_to:seq,port,descriptor:encode_wire_serialized(&artifact.descriptor),data:artifact.data});},
            protocol::AppCommand::SubmitMediaExport { seq, .. } => {
                let handle=owner.take_media_submission().expect("original media submission is complete before command donation");
                frames.push(protocol::AppFrame::MediaExportSubmitted{in_reply_to:seq,handle:media_export_handle_into_wire(handle)});
            },
            protocol::AppCommand::PollMediaExport { seq, handle } => {
                let handle = media_export_handle_from_wire(handle);
                if let Some(status)=owner.take_media_status(){frames.push(protocol::AppFrame::MediaExportStatus{in_reply_to:seq,handle:media_export_handle_into_wire(handle),state:protocol::MediaExportStateWire::Complete,applied_progress:status.total_bytes,checkpoint_available:false,mime_type:status.mime_type,total_bytes:status.total_bytes,detail:String::new()});}else{
                match owner.take_media_poll().expect("original media poll is retained before command donation") {
                    ArtifactMediaExportPoll::Running { applied_progress, checkpoint_available } => frames.push(protocol::AppFrame::MediaExportStatus {
                        in_reply_to: seq,
                        handle: media_export_handle_into_wire(handle),
                        state: protocol::MediaExportStateWire::Running,
                        applied_progress,
                        checkpoint_available,
                        mime_type: String::new(),
                        total_bytes: 0,
                        detail: String::new(),
                    }),
                    ArtifactMediaExportPoll::Complete(_) => unreachable!("terminal result donates only to its original download registry before decoded command consumption"),
                    ArtifactMediaExportPoll::Cancelled => frames.push(protocol::AppFrame::MediaExportStatus {
                        in_reply_to: seq,
                        handle: media_export_handle_into_wire(handle),
                        state: protocol::MediaExportStateWire::Cancelled,
                        applied_progress: 0,
                        checkpoint_available: false,
                        mime_type: String::new(),
                        total_bytes: 0,
                        detail: String::new(),
                    }),
                    ArtifactMediaExportPoll::Failed(detail) => frames.push(protocol::AppFrame::MediaExportStatus {
                        in_reply_to: seq,
                        handle: media_export_handle_into_wire(handle),
                        state: protocol::MediaExportStateWire::Failed,
                        applied_progress: 0,
                        checkpoint_available: false,
                        mime_type: String::new(),
                        total_bytes: 0,
                        detail,
                    }),
                }
            }}
            protocol::AppCommand::CancelMediaExport { seq, handle } => {
                let handle = media_export_handle_from_wire(handle);
                match plugin_cancel_media_export(runtime, instance_id, &handle).await {
                    Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                    Err(error) => push_app_fault(&mut frames, Some(seq), plugin_internal_fault(error.to_string())).await,
                }
            }
            protocol::AppCommand::TakeMediaExportChunk{seq,handle}=>{let(data,terminal)=owner.take_media_chunk();frames.push(protocol::AppFrame::MediaExportChunk{in_reply_to:seq,handle,data:data.unwrap_or_default(),terminal});},
            protocol::AppCommand::MediaFingerprint { seq, port } => {
                let fingerprint = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.media_fingerprint(&port)).map_err(|error| plugin_internal_fault(error.to_string()))
                });
                match fingerprint.await {
                    Ok(fingerprint) => {
                        let value = DslValue::String(fingerprint.0);
                        frames.push(protocol::AppFrame::MediaFingerprint { in_reply_to: seq, port, fingerprint: encode_wire_serialized(&value) });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::PureCommand{seq,..}=>{let(result,EmitWire{document:document_ops,config:config_ops,draft:draft_ops,children:child_ops})=original_preview.take().expect("original preview completes before decoded command donation");mutated=true;frames.push(protocol::AppFrame::Emit{in_reply_to:seq,document_ops,config_ops,draft_ops,output:encode_wire_serialized(&result.output),diagnostics:encode_wire_serialized(&result.diagnostics),child_ops});push_invocation_side_frames(&mut effect_bytes,&mut event_bytes,&result).await;},
            protocol::AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, origin, prepared_child_ops } => {
                let decoded_origin = if origin.is_empty() { Ok(None) } else { decode_wire_serialized::<protocol::MutationOrigin>(&origin).await.map(Some) };
                match decoded_origin {
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                    Ok(decoded_origin) => {
                        let outcome = with_instances_mut(runtime, |list| {
                            let mut instance = find_instance(list, instance_id)?;
                            Ok(::semio_framework_async::poll::resolve_ready(instance.app.transaction_prepare(&txn_id, &mutation_id, &payload, &prepared_ops, &prepared_child_ops, decoded_origin)))
                        });
                        match outcome.await {
                            Ok(outcome) => {
                                let mut foreign = Vec::with_capacity(outcome.foreign.len());
                                for step in outcome.foreign.iter() {
                                    foreign.push(encode_wire_serialized(step));
                                }
                                let rejection = match outcome.rejection {
                                    Some(fault) => encode_wire_serialized(&fault),
                                    None => Vec::new(),
                                };
                                frames.push(protocol::AppFrame::TransactionPrepared { txn_id, foreign, rejection });
                                frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                            }
                            Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                        }
                    }
                }
            }
            protocol::AppCommand::TransactionCommit { seq, txn_id } => {
                let meta = ActionMeta { actor: instance_actor(runtime, instance_id).await?.into(), instance_id, view_state: None };
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.transaction_commit(&txn_id, &meta, identity))
                });
                match outcome.await {
                    Ok(edit_id) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::TransactionCommitted { txn_id, edit_id });
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::TransactionRollback { seq, txn_id } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.transaction_rollback(&txn_id))
                });
                match outcome.await {
                    Ok(()) => {
                        frames.push(protocol::AppFrame::TransactionRolledBack { txn_id });
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::TransactionUndo { seq, group_id } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.transaction_undo(&group_id, identity))
                });
                match outcome.await {
                    Ok(()) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::TransactionRedo { seq, group_id } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(instance.app.transaction_redo(&group_id, identity))
                });
                match outcome.await {
                    Ok(()) => {
                        mutated = true;
                        frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    }
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } => match relay_open_artifact(artifact_ref, role, plugin_id, app_id).await {
                Ok(effect) => {
                    frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    effect_bytes.push(encode_wire_serialized(&effect));
                }
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } => match relay_set_default_app(artifact_kind, standard, subset, role, plugin_id, app_id).await {
                Ok(effect) => {
                    frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    effect_bytes.push(encode_wire_serialized(&effect));
                }
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } => match relay_clear_default_app(artifact_kind, standard, subset, role).await {
                Ok(effect) => {
                    frames.push(protocol::AppFrame::Done { in_reply_to: seq });
                    effect_bytes.push(encode_wire_serialized(&effect));
                }
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            },
            protocol::AppCommand::SetMergePolicy { seq, policy } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(set_merge_policy_frames(&mut instance.app, seq, policy))
                });
                match outcome.await {
                    Ok(outcome) => frames.extend(outcome),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ResolveConflict { seq, conflict_id, resolution } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    ::semio_framework_async::poll::resolve_ready(resolve_conflict_frames(&mut instance.app, seq, conflict_id, resolution))
                });
                match outcome.await {
                    Ok(outcome) => frames.extend(outcome),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
            protocol::AppCommand::ReadConflicts { seq } => {
                let outcome = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    Ok(::semio_framework_async::poll::resolve_ready(read_conflicts_frames(&instance.app, seq)))
                });
                match outcome.await {
                    Ok(outcome) => frames.extend(outcome),
                    Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
                }
            }
        }
        }
    }
    if mutated {
        let effects = with_instances_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            let view = instance.surface_contexts.view().cloned();
            ::semio_framework_async::poll::resolve_ready(instance.app.pending_effects(view.as_ref(), cx))
        })
        .await
        ?;
        for effect in effects.iter() {
            effect_bytes.push(encode_wire_serialized(effect));
        }
    }
    let (mut typed_output, mut typed_leftover) = with_instances_original_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        let cell=instance.cell;let mut pages=match cell.result_pages.try_lock(){Ok(pages)=>pages,Err(std::sync::TryLockError::WouldBlock)=>return Ok((PluginExchangeOutput::default(),None)),Err(std::sync::TryLockError::Poisoned(_))=>return Err(plugin_internal_fault("original result receiving authority is poisoned"))};
        advance_typed_operation_output_with_leftover(&mut instance.app, instance_id, &mut pages, identity,cx)
    })
    .await?;
    if let Some(operation) = started_typed_operation {
        let mut pumps = 0;
        while pumps < TYPED_OPERATION_REPLY_LEFTOVER_PUMPS && typed_leftover.as_ref().is_none_or(|leftover| leftover.operation != operation) && typed_output.typed_operation_results.is_empty() && !typed_output.ownership_receipt_pending() {
            let (more, leftover) = with_instances_original_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                if !instance.app.has_runnable_typed_operations() {
                    return Ok((None, None));
                }
                let cell=instance.cell;let mut pages=match cell.result_pages.try_lock(){Ok(pages)=>pages,Err(std::sync::TryLockError::WouldBlock)=>return Ok((None,None)),Err(std::sync::TryLockError::Poisoned(_))=>return Err(plugin_internal_fault("original result receiving authority is poisoned"))};
                advance_typed_operation_output_with_leftover(&mut instance.app, instance_id, &mut pages, identity,cx).map(|(output, leftover)| (Some(output), leftover))
            })
            .await?;
            let Some(more) = more else { break };
            typed_output.original_receiving=more.original_receiving;
            typed_output.constructor_receipt=more.constructor_receipt;
            typed_output.preparation_receipt=more.preparation_receipt;
            typed_output.retirement_receipt=more.retirement_receipt;
            typed_output.history_command_receipt=more.history_command_receipt;
            typed_output.tool_run_receipt=more.tool_run_receipt;
            typed_output.frames.extend(more.frames);
            typed_output.effects.extend(more.effects);
            typed_output.events.extend(more.events);
            typed_output.typed_operation_pages+=more.typed_operation_pages;
            if leftover.is_some() {
                typed_leftover = leftover;
            }
            pumps += 1;
        }
    }
    effect_bytes.extend(typed_output.effects);
    event_bytes.extend(typed_output.events);
    if let Some(leftover) = typed_leftover.as_ref() {
        let merged = merge_typed_operation_leftover_into_reply(&mut frames, leftover);
        debug_runtime_line(format_args!("[TRACE] typed-operation inline leftover operation={} merged-into-reply={merged}", leftover.operation));
    }
    let ephemeral = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(::semio_framework_async::poll::resolve_ready(instance.app.ephemeral_snapshot()))
    });
    if let Ok(EphemeralSnapshot { presence, presence_generation, transient_generation, interaction, tool_run, history_edit }) = ephemeral.await {
        let tool_run = tool_run.map_or_else(Vec::new, |tool_run| {
            let mut bytes = Vec::new();
            protocol::encode_presence_tool_run(&tool_run, &mut bytes);
            bytes
        });
        let history_edit = history_edit.map_or_else(Vec::new, |history_edit| {
            let mut bytes = Vec::new();
            protocol::encode_presence_history_edit(&history_edit, &mut bytes);
            bytes
        });
        frames.push(protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run, history_edit });
    }
    if let Some(outcome) = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        Ok(instance.app.take_presence_outcome())
    })
    .await?
    {
        if let Some(fault) = outcome.fault {
            push_app_fault(&mut frames, Some(outcome.seq), fault).await;
        }
    }
    let mut frame_bytes = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        frame_bytes.push(protocol::encode_app_frame(frame).await);
    }
    frame_bytes.extend(typed_output.frames);
    Ok(PluginExchangeOutput {
        original_receiving: typed_output.original_receiving,
        constructor_receipt: typed_output.constructor_receipt,
        preparation_receipt: typed_output.preparation_receipt,
        retirement_receipt: typed_output.retirement_receipt,
        history_command_receipt: typed_output.history_command_receipt,
        tool_run_receipt: typed_output.tool_run_receipt,
        frames: frame_bytes,
        effects: effect_bytes,
        events: event_bytes,
        retry_command,
        command_terminal_fault,
        presence_pending,
        presence_terminal,
        presence_terminal_fault,
        typed_operation_pages: typed_output.typed_operation_pages,
    })
}
