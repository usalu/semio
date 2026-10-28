//! 📄️ Document operations, codecs, archive loads, backbone bindings and codec registration.

use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework::kernel::{Effect, MessageEndpoint};
use semio_framework::{Fault, FaultFrom};
use semio_framework_value::{FromValue, ToValue};
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::{PLUGIN_RUNTIME_INSTANCE_SLOTS, PluginRuntime, RuntimeDocumentBackboneBindingV1};
use semio_framework_plugin_plugin_program::{Plugin, PluginProgram, encode_wire_serialized};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use crate::ops_typed_operations::drive_self_waking_ready;
use crate::ops_wire_limits::find_instance;
use semio_framework_plugin_reactor_app::app_runtime_helpers::plugin_internal_fault;
use semio_framework_plugin_runtime_instances::inst_wire_frames::with_instances_mut;
/// 📥️ Ingests binary-encoded remote `MutationEnvelope`s (`protocol::decode_envelopes`) into
/// the instance's document store (idempotent — duplicate mutation ids are dropped by the causal
/// DAG / edit-id dedupe), returning one `protocol::MergeReport` per envelope for the caller
/// (`ApplyEnvelopes`'s handler) to frame as unsolicited `AppFrame::MergeReport` frames.
pub async fn plugin_ingest_operations<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.ingest_operations(mutations, identity, cx))
    })
    .await
}
/// 📜️ Text-DSL counterpart of {@link plugin_ingest_operations}: one already-authored operation
/// per non-blank op-text line instead of a binary `MutationEnvelope` array.
pub async fn plugin_ingest_operations_text<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operations_text: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.ingest_operations_text(operations_text, identity))
    })
    .await
}
/// 📜️ Text-DSL counterpart of {@link plugin_document_pack}.
pub async fn plugin_document_text<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<store::os_store::text_format::ArtifactTextFiles, Fault> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.document_text())
    })
    .await
}
/// 🧬️ Resolves the `codec` answers of the ONE app of the installed bundle that owns `artifact_schema`. This is
/// the guest half of `world actor`'s `codec` interface: a host that links no Rust codec for this package selects
/// a document kind by the same `schema` string `store::os_store::serialization::ArtifactCodec` is keyed by, and the bundle answers from its
/// own registered apps. An editor is preferred over a viewer because only an editor's snapshot is the kind's
/// creation authority; an ambiguous schema is refused rather than resolved by order. Nothing is constructed: the
/// answers are the functions of the owner's type its registration recorded ([`crate::app::ArtifactCodecTableV1`]).
fn plugin_artifact_codec<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str) -> Result<semio_framework_plugin_decl_surface::ArtifactCodecTableV1, Fault> {
    if artifact_schema.is_empty() || artifact_schema.len() > 256 || artifact_schema.chars().any(char::is_control) {
        return Err(plugin_internal_fault("artifact codec schema identity is empty, oversized or control-bearing"));
    }
    if let Some(fault) = runtime.plugin_assembly_error.try_borrow().map_err(|_| plugin_internal_fault("plugin assembly authority busy"))?.clone() {
        return Err(fault);
    }
    let program = runtime.plugin.try_borrow().map_err(|_| plugin_internal_fault("plugin factory authority busy"))?;
    let program = program.as_ref().ok_or_else(|| plugin_internal_fault("plugin not initialized"))?;
    let definition = artifact_codec_owner(program, artifact_schema)?;
    program.app_codec(&definition.id).ok_or_else(|| plugin_internal_fault("artifact codec owner has no registered codec"))
}
/// 🪪️ The apps of `program` that own `artifact_schema`: those whose registered type opens that document
/// schema — the primary key `store::os_store::serialization::ArtifactCodec` and the hub's trusted catalog use — and those whose
/// dialect names it as an ARTIFACT KIND, the identity build tooling with nothing but a compiled
/// descriptor asks by (it reads the schema back out of `codec.genesis` and asks by schema after that; a
/// kind `s.<plugin>.<artifact>` and a `DOCUMENT_SCHEMA` never collide). Pure: nothing is constructed.
#[doc(hidden)]
pub fn artifact_codec_candidates<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> impl Iterator<Item = &'a semio_framework::AppDefinition> + 'a {
    program.manifest.apps.iter().filter(move |definition| program.app_document_schema(&definition.id) == Some(artifact_schema) || definition.dialect.artifact_kind == artifact_schema)
}
/// 🎯️ The ONE app whose codec answers `artifact_schema`: the owning editor — only an editor's snapshot is the
/// kind's creation authority — else the owning viewer. Two owners of the same role are refused rather than
/// resolved by order, and no owner is refused; all of it decided on the declarations.
#[doc(hidden)]
pub fn artifact_codec_owner<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> Result<&'a semio_framework::AppDefinition, Fault> {
    let (mut editor, mut viewer) = (None, None);
    for definition in artifact_codec_candidates(program, artifact_schema) {
        let slot = if definition.role == semio_framework::AppRole::Editor { &mut editor } else { &mut viewer };
        if slot.replace(definition).is_some() {
            return Err(plugin_internal_fault("artifact codec schema resolves more than one app of the same role"));
        }
    }
    editor.or(viewer).ok_or_else(|| plugin_internal_fault("artifact codec schema is owned by no app of this bundle"))
}
/// 🧬️ `codec.pack-schema-hash` — the kind's own 32-byte snapshot-record fingerprint.
pub async fn plugin_artifact_pack_schema_hash<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str) -> Result<[u8; 32], Fault> {
    let codec = plugin_artifact_codec(runtime, artifact_schema)?;
    (codec.pack_schema_hash)().ok_or_else(|| plugin_internal_fault("artifact codec schema has no structural record specification"))
}
/// 🏛️ `codec.sqlite-schema` names a registered dialect's handwritten relational declaration.
pub fn plugin_snapshot_sqlite_schema(dialect: &str) -> Result<String, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};
    let dialect = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(dialect).map_err(|error| plugin_internal_fault(&error))?;
    semio_framework_os_kernel::io::io_mechanism::native_snapshot_sqlite_schema(&dialect).map_err(|error| plugin_internal_fault(&error.to_string()))
}
/// 📤️ `codec.sqlite-export` executes the exact declared native snapshot provider.
pub async fn plugin_snapshot_sqlite_export(owner: &mut semio_framework_plugin_abi::sqlite_wire::SnapshotInputOwner, native: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>, snapshot: &mut semio_framework::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<semio_framework_plugin_abi::sqlite_wire::SnapshotFileResult, Fault> {
    use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework::io_schema::{IoPayload,SQLITE_SNAPSHOT}};
    let input=owner.wire.as_mut().ok_or_else(||plugin_internal_fault("snapshot original request is absent"))?;
    let limits=input.limits.native().map_err(plugin_internal_fault)?;
    snapshot.restrict_limits(limits).map_err(|error|plugin_internal_fault(error.into_message()))?;
    owner.dialect=Some(ArtifactDialect::parse_coordinate(&input.dialect).map_err(plugin_internal_fault)?);
    let encoding=semio_framework::sqlite_snapshot::SnapshotEncoding::parse(&input.encoding).map_err(|error|plugin_internal_fault(error.into_message()))?;
    if matches!(encoding,semio_framework::sqlite_snapshot::SnapshotEncoding::Text)&&std::str::from_utf8(&input.payload).is_err(){return Err(plugin_internal_fault("snapshot original text is invalid UTF8"))}
    owner.payload=Some(match encoding{semio_framework::sqlite_snapshot::SnapshotEncoding::Binary=>IoPayload::Binary(std::mem::take(&mut input.payload)),semio_framework::sqlite_snapshot::SnapshotEncoding::Text=>IoPayload::Text(String::from_utf8(std::mem::take(&mut input.payload)).unwrap())});
    owner.route=Some(semio_framework_os_kernel::io::io_mechanism::io_route(owner.dialect.as_ref().unwrap(),&ArtifactDialect::from(SQLITE_SNAPSHOT),1).await);
    let route=match owner.route.as_mut().unwrap(){Ok(route)=>&route.value,Err(error)=>{let kind=error.cause.kind;return Err(plugin_internal_fault(std::mem::replace(&mut error.cause,semio_framework_value::ValueError::literal(kind,"snapshot route refused")).into_message()))}};
    owner.hop=Some(semio_framework_os_kernel::io::io_mechanism::io_run_with_snapshot_control(route,&mut owner.payload,native,snapshot).await);
    match owner.hop.as_mut().unwrap(){
        Err(error)=>{let kind=error.cause.kind;let diagnostics=semio_framework_plugin_abi::sqlite_wire::encode_diagnostics(&error.diagnostics);let message=std::mem::replace(&mut error.cause,semio_framework_value::ValueError::literal(kind,"snapshot hop refused")).into_message();Ok(semio_framework_plugin_abi::sqlite_wire::SnapshotFileResult::Rejected(semio_framework_plugin_abi::sqlite_wire::SnapshotRejection{kind,message,diagnostics}))},
        Ok(output)=>match &mut output.value{IoPayload::Binary(bytes)=>Ok(semio_framework_plugin_abi::sqlite_wire::SnapshotFileResult::Done(semio_framework_plugin_abi::sqlite_wire::SnapshotFile{bytes:std::mem::take(bytes),diagnostics:semio_framework_plugin_abi::sqlite_wire::encode_diagnostics(&output.diagnostics)})),IoPayload::Text(_)=>Err(plugin_internal_fault("semantic SQLite export must produce binary"))},
    }
}
/// 📥️ `codec.sqlite-import` validates exact dialect and DDL before reconstructing native values.
pub async fn plugin_snapshot_sqlite_import(owner: &mut semio_framework_plugin_abi::sqlite_wire::SnapshotInputOwner, native: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>, snapshot: &mut semio_framework::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<semio_framework_plugin_abi::sqlite_wire::SnapshotPayloadResult, Fault> {
    use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework::io_schema::{IoPayload,SQLITE_SNAPSHOT}};
    let input=owner.wire.as_mut().ok_or_else(||plugin_internal_fault("snapshot original request is absent"))?;
    let limits=input.limits.native().map_err(plugin_internal_fault)?;
    snapshot.restrict_limits(limits).map_err(|error|plugin_internal_fault(error.into_message()))?;
    owner.dialect=Some(ArtifactDialect::parse_coordinate(&input.dialect).map_err(plugin_internal_fault)?);
    owner.payload=Some(IoPayload::Binary(std::mem::take(&mut input.payload)));
    owner.route=Some(semio_framework_os_kernel::io::io_mechanism::io_route(&ArtifactDialect::from(SQLITE_SNAPSHOT),owner.dialect.as_ref().unwrap(),1).await);
    let route=match owner.route.as_mut().unwrap(){Ok(route)=>&route.value,Err(error)=>{let kind=error.cause.kind;return Err(plugin_internal_fault(std::mem::replace(&mut error.cause,semio_framework_value::ValueError::literal(kind,"snapshot route refused")).into_message()))}};
    owner.hop=Some(semio_framework_os_kernel::io::io_mechanism::io_run_with_snapshot_control(route,&mut owner.payload,native,snapshot).await);
    match owner.hop.as_mut().unwrap(){
        Err(error)=>{let kind=error.cause.kind;let diagnostics=semio_framework_plugin_abi::sqlite_wire::encode_diagnostics(&error.diagnostics);let message=std::mem::replace(&mut error.cause,semio_framework_value::ValueError::literal(kind,"snapshot hop refused")).into_message();Ok(semio_framework_plugin_abi::sqlite_wire::SnapshotPayloadResult::Rejected(semio_framework_plugin_abi::sqlite_wire::SnapshotRejection{kind,message,diagnostics}))},
        Ok(output)=>{let diagnostics=semio_framework_plugin_abi::sqlite_wire::encode_diagnostics(&output.diagnostics);Ok(semio_framework_plugin_abi::sqlite_wire::SnapshotPayloadResult::Done(match &mut output.value{IoPayload::Binary(bytes)=>semio_framework_plugin_abi::sqlite_wire::SnapshotPayload{encoding:semio_framework_plugin_abi::sqlite_wire::SnapshotEncoding::Binary,bytes:std::mem::take(bytes),diagnostics},IoPayload::Text(text)=>semio_framework_plugin_abi::sqlite_wire::SnapshotPayload{encoding:semio_framework_plugin_abi::sqlite_wire::SnapshotEncoding::Text,bytes:std::mem::take(text).into_bytes(),diagnostics}}))},
    }
}
/// 🌱️ `codec.genesis` — the canonical empty document of `artifact_schema` at `document_id`.
pub async fn plugin_artifact_genesis<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str, document_id: &str) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
    (plugin_artifact_codec(runtime, artifact_schema)?.genesis)(document_id).await
}
/// 📥️ `codec.print-mirror` — the host's pair-validation fence for an unlinked package.
pub async fn plugin_artifact_print_mirror<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str, pack: &[u8], spr: &[u8]) -> Result<store::os_store::text_format::ArtifactTextFiles, Fault> {
    (plugin_artifact_codec(runtime, artifact_schema)?.print_mirror)(pack, spr).await
}
/// 🧩️ `codec.apply-ops` — the host-authoritative edit apply for an unlinked package.
pub async fn plugin_artifact_apply_ops<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
    (plugin_artifact_codec(runtime, artifact_schema)?.apply_ops)(pack, spr, ops, identity).await
}
/// 📜️ `codec.replay-envelopes` — the hub's Check In fold for an unlinked package.
pub async fn plugin_artifact_replay_envelopes<PA: PluginApp>(runtime: &PluginRuntime<PA>, artifact_schema: &str, pack: &[u8], spr: &[u8], envelopes: &[u8]) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
    (plugin_artifact_codec(runtime, artifact_schema)?.replay_envelopes)(pack, spr, envelopes).await
}
/// 📦️ Serializes the instance's full persistent document as pack+spr bytes
/// ({@link store::os_store::serialization::ArtifactPackFiles}) via `store::os_store::text_format::print_document_pack`.
pub async fn plugin_document_pack<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.document_pack())
    })
    .await
}
/// 🏗️ Whether instance `instance_id` still owes store constructor turns the reactor drives before any command.
pub async fn plugin_store_constructor_pending<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<bool, Fault> {
    with_instances_mut(runtime, |list| Ok(find_instance(list, instance_id)?.app.store_constructor_is_pending())).await
}
/// 🪪️ The archive a whole-document load of `files` into instance `instance_id` admits, stamped with that instance's store
/// identity ([`PluginApp::document_load_archive`]).
pub async fn plugin_document_load_archive<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<protocol::DocumentArchivePack, Fault> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        instance.app.document_load_archive(files)
    })
    .await
}
/// 🛬️ Admits one whole-document archive load under `operation` on instance `instance_id` (`📓️api-stepped-document-load.md`)
/// — the in-guest twin of `AppCommand::LoadDocumentArchive`.
pub async fn plugin_begin_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64, archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.begin_document_archive_load(operation, archive)
    })
    .await
}
/// 📊️ Advances one whole-document archive load for at most one poll wall and answers its status — the in-guest twin of
/// `AppCommand::PollDocumentArchiveLoad`.
pub async fn plugin_poll_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        drive_self_waking_ready(instance.app.poll_document_archive_load(operation, identity, cx))
    })
    .await
}
/// 🛑️ Requests the cancellation of one running whole-document archive load — the in-guest twin of
/// `AppCommand::CancelDocumentArchiveLoad`; the load reaches `Cancelled` over later polls.
pub async fn plugin_cancel_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.cancel_document_archive_load(operation)
    })
    .await
}
/// 📨️ Releases one terminal whole-document archive load — the in-guest twin of `AppCommand::AcknowledgeDocumentArchiveLoad`.
pub async fn plugin_acknowledge_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.acknowledge_document_archive_load(operation)
    })
    .await
}
/// 📡️ One inbound backbone delivery's turn output. `document_changed` is true when the delivery
/// merged remote edits into the document, so every view of it must be re-projected.
pub struct DocumentBackboneTurnOutputV1 {
    pub instance_id: u32,
    pub document_changed: bool,
    pub frames: Vec<Vec<u8>>,
    pub effects: Vec<Effect>,
}
#[doc(hidden)]
pub fn document_backbone_effects(owner: &store::os_store::backbone::ActorBackboneChannelOwner, uri: &str) -> Result<Vec<Effect>, Fault> {
    let mut effects = Vec::new();
    for _ in 0..store::os_store::backbone::BACKBONE_CHANNEL_MAXIMUM_MESSAGES {
        let Some(payload) = owner.take_outbound().map_err(|error| plugin_internal_fault(error.to_string()))? else { break };
        effects.push(Effect::SendMessage { target: MessageEndpoint::Backbone { uri: uri.into() }, payload });
    }
    Ok(effects)
}
fn live_document_backbone_for_uri<PA: PluginApp>(runtime: &PluginRuntime<PA>, uri: &str) -> Result<Option<(u32, u64, store::os_store::backbone::ActorBackboneChannelOwner)>, Fault> {
    let bindings = runtime.document_backbones.try_borrow().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
    for index in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS {
        let Some((instance_id, binding)) = bindings.entry_at(index) else { continue };
        if binding.uri.as_deref() == Some(uri) {
            if let Some(channel) = &binding.channel {
                return Ok(Some((instance_id, binding.generation, channel.clone())));
            }
        }
    }
    Ok(None)
}
fn document_backbone_receipt_effect(receipt: &semio_framework_plugin_actor_host::document_backbone_binding::DocumentBackboneBindingReceiptV1) -> Effect {
    Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(receipt.instance_id.to_string()) }, payload: receipt.encode() }
}
pub async fn plugin_handle_document_backbone_binding<PA: PluginApp>(runtime: &PluginRuntime<PA>, source_instance: &str, payload: &[u8]) -> Result<Option<Vec<Effect>>, Fault> {
    use semio_framework_plugin_actor_host::document_backbone_binding::{decide_document_backbone_binding_v1, decode_document_backbone_binding_command_v1, DocumentBackboneBindingDecisionV1};
    let Some(command) = decode_document_backbone_binding_command_v1(payload).map_err(plugin_internal_fault)? else { return Ok(None) };
    let source_matches = source_instance.parse::<u32>().ok() == Some(command.instance_id);
    let instance_live = runtime.guest_lifetimes.try_borrow().map_err(|_| plugin_internal_fault("lifecycle authority busy"))?.get(command.instance_id).is_some_and(|slot| slot.cell.is_live());
    if !source_matches || !instance_live {
        return Ok(Some(vec![document_backbone_receipt_effect(&semio_framework_plugin_actor_host::document_backbone_binding::DocumentBackboneBindingReceiptV1::refused(&command, "plugin.document-backbone.instance-refused"))]));
    }
    let state = runtime.document_backbones.try_borrow().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?.get(command.instance_id).map(RuntimeDocumentBackboneBindingV1::state).unwrap_or_default();
    match decide_document_backbone_binding_v1(&state, &command) {
        DocumentBackboneBindingDecisionV1::Replay(receipt) | DocumentBackboneBindingDecisionV1::Refuse(receipt) => Ok(Some(vec![document_backbone_receipt_effect(&receipt)])),
        DocumentBackboneBindingDecisionV1::Bind(receipt) => {
            if live_document_backbone_for_uri(runtime, &command.uri)?.is_some() {
                return Ok(Some(vec![document_backbone_receipt_effect(&semio_framework_plugin_actor_host::document_backbone_binding::DocumentBackboneBindingReceiptV1::refused(&command, "plugin.document-backbone.binding-collision"))]));
            }
            let (channel, owner) = store::os_store::backbone::ActorBackboneChannelOwner::pair(&command.uri);
            let backbone = store::os_store::backbone::Backbones::Port(store::os_store::backbone::PortBackbone::with_channel(&command.uri, channel).await);
            let attached = with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, command.instance_id)?;
                ::semio_framework_async::poll::resolve_ready(instance.app.attach_hot_backbone(backbone))
            })
            .await;
            if attached.is_err() {
                owner.begin_retire().map_err(|error| plugin_internal_fault(error.to_string()))?;
                return Ok(Some(vec![document_backbone_receipt_effect(&semio_framework_plugin_actor_host::document_backbone_binding::DocumentBackboneBindingReceiptV1::refused(&command, "plugin.document-backbone.attach-refused"))]));
            }
            let rejected = {
                let mut bindings = runtime.document_backbones.try_borrow_mut().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
                if let Some(binding) = bindings.get_mut(command.instance_id) {
                    if binding.state() != state {
                        Some(("plugin.document-backbone.binding-collision", owner))
                    } else {
                        binding.generation = command.binding_generation;
                        binding.uri = Some(command.uri.clone());
                        binding.channel = Some(owner);
                        None
                    }
                } else if !bindings.can_insert(command.instance_id) {
                    Some(("plugin.document-backbone.binding-capacity", owner))
                } else {
                    bindings.insert_admitted(command.instance_id, RuntimeDocumentBackboneBindingV1 { generation: command.binding_generation, uri: Some(command.uri.clone()), channel: Some(owner) });
                    None
                }
            };
            if let Some((code, owner)) = rejected {
                let _ = plugin_detach_backbone(runtime, command.instance_id).await;
                owner.begin_retire().map_err(|error| plugin_internal_fault(error.to_string()))?;
                return Ok(Some(vec![document_backbone_receipt_effect(&semio_framework_plugin_actor_host::document_backbone_binding::DocumentBackboneBindingReceiptV1::refused(&command, code))]));
            }
            Ok(Some(vec![document_backbone_receipt_effect(&receipt)]))
        }
        DocumentBackboneBindingDecisionV1::Retire(receipt) => {
            let owner = runtime
                .document_backbones
                .try_borrow()
                .map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?
                .get(command.instance_id)
                .and_then(|binding| binding.channel.clone())
                .ok_or_else(|| plugin_internal_fault("live document backbone channel is missing"))?;
            owner.begin_retire().map_err(|error| plugin_internal_fault(error.to_string()))?;
            plugin_detach_backbone(runtime, command.instance_id).await?;
            if !owner.terminal_is_empty().map_err(|error| plugin_internal_fault(error.to_string()))? {
                return Err(plugin_internal_fault("retired document backbone channel did not drain"));
            }
            let mut bindings = runtime.document_backbones.try_borrow_mut().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
            let binding = bindings.get_mut(command.instance_id).ok_or_else(|| plugin_internal_fault("document backbone binding disappeared during retirement"))?;
            binding.uri = None;
            binding.channel = None;
            Ok(Some(vec![document_backbone_receipt_effect(&receipt)]))
        }
    }
}
pub async fn plugin_receive_document_backbone<PA: PluginApp>(runtime: &PluginRuntime<PA>, uri: &str, payload: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<DocumentBackboneTurnOutputV1, Fault> {
    let (instance_id, generation, owner) = live_document_backbone_for_uri(runtime, uri)?.ok_or_else(|| plugin_internal_fault("backbone message has no exact live document binding"))?;
    owner.push_inbound(uri, payload).map_err(|error| plugin_internal_fault(error.to_string()))?;
    let (reports, conflicts) = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        let reports = ::semio_framework_async::poll::resolve_ready(instance.app.tick_backbone(identity, cx))?;
        let conflicts = if reports.is_empty() { Vec::new() } else { ::semio_framework_async::poll::resolve_ready(instance.app.open_conflicts()) };
        Ok((reports, conflicts))
    })
    .await?;
    let current = live_document_backbone_for_uri(runtime, uri)?;
    if !current.as_ref().is_some_and(|(current_instance, current_generation, current_owner)| *current_instance == instance_id && *current_generation == generation && current_owner.admits_same_channel(&owner)) {
        return Err(plugin_internal_fault("document backbone authority changed during inbound delivery"));
    }
    let mut frames = Vec::new();
    for report in &reports {
        frames.push(protocol::encode_app_frame(&protocol::AppFrame::MergeReport { in_reply_to: None, report: encode_wire_serialized(report) }).await);
    }
    if !reports.is_empty() {
        frames.push(protocol::encode_app_frame(&protocol::AppFrame::Conflicts { in_reply_to: None, conflicts: encode_wire_serialized(&conflicts) }).await);
    }
    Ok(DocumentBackboneTurnOutputV1 { instance_id, document_changed: !reports.is_empty(), frames, effects: document_backbone_effects(&owner, uri)? })
}
pub fn plugin_drain_document_backbones<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> Result<Vec<Effect>, Fault> {
    let bindings = runtime.document_backbones.try_borrow().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
    let mut effects = Vec::new();
    for index in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS {
        let Some((_, binding)) = bindings.entry_at(index) else { continue };
        let (Some(uri), Some(owner)) = (binding.uri.as_deref(), binding.channel.as_ref()) else { continue };
        effects.extend(document_backbone_effects(owner, uri)?);
    }
    Ok(effects)
}
/// 🗂️ Registers `A::Snapshot`'s pack↔dsl codec under `schema` in the process-wide
/// `store::os_store::serialization::ArtifactCodec` registry — the one-liner every app's own native registration fn
/// (`register_<app>_exports()`-style) calls once per document kind so `framework/sync`'s
/// `FolderEndpoint` (and any other schema-string-keyed caller) can print/parse that kind without
/// depending on its concrete `Snapshot`/`Mutation` types.
pub async fn register_document_codec_for_app<A: ArtifactApp>(schema: impl Into<String>) -> Result<(), PluginAssemblyError>
where
    A::Mutation: Sync,
{
    let codec = store::native_artifact_codec::<A::Snapshot>(schema);
    let app = A::default();
    let owner = app.instance_id().await.to_string();
    let document_bindings = vec![semio_framework_os_kernel::io::ArtifactCodecBinding::from_codec(semio_framework_os_kernel::io::ArtifactCodecBindingChannel::DirectDocumentNative, owner.clone(), Some(A::DIALECT.artifact_kind.to_string()), vec![owner], Some(A::DIALECT.into()), None, &codec)];
    let plan = semio_framework_os_kernel::io::ArtifactAssemblyRegistryPlan {
        document_codecs: vec![codec.clone()],
        document_bindings,
        native_snapshots: semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration::from_capability(A::DIALECT.into(), codec).into_iter().collect(),
        ..Default::default()
    };
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|error| PluginAssemblyError::new("plugin-assembly.unavailable", error.to_string()))?;
    semio_framework_os_kernel::io::commit_artifact_assembly_registry_plan(&assembly, plan).map_err(|error| PluginAssemblyError::new("plugin-assembly.snapshot", error.to_string()))
}
/// 🔗️ Attaches a backbone channel by URI. The URI is resolved to a `store::os_store::backbone::PortBackbone`
/// (a pure queue relayed across the wasm sandbox to the host); the host owns the real IO endpoint.
pub async fn plugin_attach_backbone<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, uri: &str) -> Result<(), Fault> {
    let backbone: store::os_store::backbone::Backbones = store::os_store::backbone::Backbones::Port(store::os_store::backbone::PortBackbone::new(uri).await);
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.attach_backbone(backbone))
    })
    .await
}
/// ✂️ Detaches the instance's backbone channel; the document graph stays in memory.
pub async fn plugin_detach_backbone<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.detach_backbone())
    })
    .await
}
