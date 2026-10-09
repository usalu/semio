//! 📤️ Borrowed generated return values expose exact native allocation counts before canonical lifting.
use super::return_types::{reactor,effects,ui,codec,types,pure};
use semio_framework_value::{ValueError,ValueRefusalKind};
type Reserve<'a>=dyn FnMut(pure::OperationReturnAllocation,usize)->Result<(),ValueError>+'a;
fn count(kind:pure::OperationReturnAllocation,count:usize,reserve:&mut Reserve<'_>)->Result<(),ValueError>{if count==0{Ok(())}else{reserve(kind,count)}}
fn bytes(value:&[u8],reserve:&mut Reserve<'_>)->Result<(),ValueError>{count(pure::OperationReturnAllocation::Bytes,value.len(),reserve)}
fn text(value:&str,reserve:&mut Reserve<'_>)->Result<(),ValueError>{count(pure::OperationReturnAllocation::StringBytes,value.len(),reserve)}
fn optional_bytes(value:&Option<Vec<u8>>,reserve:&mut Reserve<'_>)->Result<(),ValueError>{value.as_ref().map_or(Ok(()),|value|bytes(value,reserve))}
fn optional_text(value:&Option<String>,reserve:&mut Reserve<'_>)->Result<(),ValueError>{value.as_ref().map_or(Ok(()),|value|text(value,reserve))}
fn effect(value:&effects::Effect,reserve:&mut Reserve<'_>)->Result<(),ValueError>{use effects::Effect;match value{
    Effect::SendMessage(value)=>{match &value.target{types::MessageEndpoint::Backbone(value)|types::MessageEndpoint::Extension(value)|types::MessageEndpoint::Topic(value)=>text(value,reserve)?,types::MessageEndpoint::Shell(_)|types::MessageEndpoint::PluginInstance(_)=>{}}bytes(&value.payload,reserve)},
    Effect::PublishEvent(value)=>{text(&value.topic,reserve)?;bytes(&value.payload,reserve)},
    Effect::BlobLoad(value)=>text(&value.params.hash,reserve),
    Effect::BlobWrite(value)=>{bytes(&value.params.media_type,reserve)?;bytes(&value.params.bytes,reserve)},
    Effect::HttpRequest(value)=>{text(&value.params.method,reserve)?;text(&value.params.url,reserve)?;count(pure::OperationReturnAllocation::StringPairList,value.params.headers.len(),reserve)?;for(name,value)in &value.params.headers{text(name,reserve)?;text(value,reserve)?;}optional_bytes(&value.params.body,reserve)},
    Effect::ArtifactRead(value)=>text(&value.params.lane,reserve),
    Effect::ArtifactWrite(value)=>{text(&value.params.lane,reserve)?;bytes(&value.params.ops,reserve)},
    Effect::LinkResolve(value)=>bytes(&value.link,reserve),
    Effect::RegistryQuery(value)=>{text(&value.params.kind,reserve)?;bytes(&value.params.filter,reserve)},
    Effect::IoCompose(value)=>{bytes(&value.params.key,reserve)?;bytes(&value.params.sources,reserve)},
    Effect::IoRun(value)=>{text(&value.params.source,reserve)?;text(&value.params.target,reserve)?;bytes(&value.params.payload,reserve)},
    Effect::CacheDerive(value)=>{text(&value.params.engine_id,reserve)?;bytes(&value.params.input,reserve)},
    Effect::CacheRead(value)=>{text(&value.params.engine_id,reserve)?;bytes(&value.params.key,reserve)},
    Effect::OpenWindow(value)=>{text(&value.params.kind,reserve)?;bytes(&value.params.params,reserve)},
    Effect::DispatchAction(value)=>{text(&value.params.action,reserve)?;optional_bytes(&value.params.args,reserve)},
    Effect::InvokeExtension(value)=>{text(&value.params.extension_id,reserve)?;text(&value.params.capability,reserve)?;bytes(&value.params.payload,reserve)},
    Effect::Notify(value)=>text(&value.message,reserve),
    Effect::ClipboardWrite(value)=>bytes(&value.fragment,reserve),
    Effect::Navigate(value)=>text(&value.uri,reserve),
    Effect::OpenExternalUrl(value)=>text(&value.url,reserve),
    Effect::SetPanel(value)=>text(&value.panel_json,reserve),
    Effect::SetActiveUtility(value)=>{text(&value.window_id,reserve)?;text(&value.utility_id,reserve)},
    Effect::SetActiveTool(value)=>text(&value.tool_id,reserve),
    Effect::ReplayShellCommand(value)=>{text(&value.action_id,reserve)?;optional_bytes(&value.args,reserve)},
    Effect::SpawnPluginInstance(value)=>{text(&value.params.plugin_id,reserve)?;text(&value.params.app_id,reserve)?;optional_text(&value.params.os_instance_id,reserve)?;optional_text(&value.params.label,reserve)?;optional_text(&value.params.artifact_json,reserve)},
    Effect::OpenPluginInstance(value)=>{text(&value.plugin_id,reserve)?;text(&value.app_id,reserve)?;optional_text(&value.os_instance_id,reserve)},
    Effect::OpenDialog(value)=>{text(&value.params.dialog_id,reserve)?;optional_bytes(&value.params.args,reserve)},
    Effect::IconRenderExport(value)=>bytes(&value.items,reserve),
    Effect::DownloadMediaExport(value)=>{text(&value.filename,reserve)?;text(&value.mime_type,reserve)?;text(&value.data,reserve)?;optional_text(&value.encoding,reserve)},
    Effect::VideoRenderExport(value)=>{text(&value.filename,reserve)?;bytes(&value.program,reserve)},
    Effect::RequestFileOpen(value)=>{text(&value.params.accept,reserve)?;optional_text(&value.params.read_as,reserve)?;text(&value.params.import_action,reserve)?;optional_bytes(&value.params.args,reserve)},
    Effect::RequestMediaFrames(value)=>{text(&value.params.accept,reserve)?;text(&value.params.frame_action,reserve)?;text(&value.params.done_action,reserve)?;text(&value.params.fallback_action,reserve)?;optional_text(&value.params.payload,reserve)?;optional_bytes(&value.params.args,reserve)},
    Effect::LoadDocument(value)=>{bytes(&value.doc_pack,reserve)?;bytes(&value.spr,reserve)},
    Effect::SpawnJob(value)=>{text(&value.kind,reserve)?;bytes(&value.input,reserve)},
    Effect::Respond(value)=>match &value.outcome{effects::RespondResult::Ok(value)|effects::RespondResult::Fault(value)=>bytes(value,reserve)},
    Effect::StorageRead(value)=>text(&value.params.key,reserve),
    Effect::StorageWrite(value)=>{text(&value.params.key,reserve)?;bytes(&value.params.value,reserve)},
    Effect::StorageDelete(value)=>text(&value.params.key,reserve),
    Effect::RequestCapability(value)=>{text(&value.params.id,reserve)?;text(&value.params.scope,reserve)?;text(&value.params.reason,reserve)},
    Effect::ReleaseCapability(value)=>text(&value.id,reserve),
    Effect::Subscribe(value)|Effect::Unsubscribe(value)=>text(&value.topic,reserve),
    Effect::RequestServiceOperation(value)=>{text(&value.owner,reserve)?;text(&value.service_id,reserve)?;text(&value.action,reserve)?;bytes(&value.payload,reserve)},
    Effect::CloseWindow(_)|Effect::RequestSync|Effect::SetTimer(_)|Effect::CancelJob(_)=>Ok(())
}}
fn patch(value:&ui::UiPatch,reserve:&mut Reserve<'_>)->Result<(),ValueError>{text(&value.surface.surface,reserve)?;count(pure::OperationReturnAllocation::PatchOpList,value.ops.len(),reserve)?;for operation in &value.ops{match operation{
    ui::PatchOp::Upsert(value)=>bytes(&value.node,reserve)?,ui::PatchOp::SetComponent(value)=>bytes(&value.component,reserve)?,ui::PatchOp::SetLayout(value)=>bytes(&value.layout,reserve)?,ui::PatchOp::SetActivity(value)=>bytes(&value.activity,reserve)?,ui::PatchOp::SetChildren(value)=>count(pure::OperationReturnAllocation::U64List,value.children.len(),reserve)?,ui::PatchOp::SetStyle(value)=>bytes(&value.style,reserve)?,ui::PatchOp::SetAccessibility(value)=>bytes(&value.accessibility,reserve)?,ui::PatchOp::SetBindings(value)=>bytes(&value.bindings,reserve)?,ui::PatchOp::SetMenu(value)=>bytes(&value.menu,reserve)?,ui::PatchOp::Remove(_)|ui::PatchOp::SetRoot(_)=>{}
}}Ok(())}
/// 🔎️ Walks actual generated turn fields without cloning their buffers, strings or list frames.
pub fn walk_turn(value:&reactor::TurnResult,reserve:&mut Reserve<'_>)->Result<(),ValueError>{
    count(pure::OperationReturnAllocation::UiPatchList,value.ui_patches.len(),reserve)?;for value in &value.ui_patches{patch(value,reserve)?;}
    count(pure::OperationReturnAllocation::EffectList,value.effects.len(),reserve)?;for value in &value.effects{effect(value,reserve)?;}
    count(pure::OperationReturnAllocation::PresenceUpdateList,value.presence.len(),reserve)?;for value in &value.presence{bytes(&value.update,reserve)?;}
    match &value.status{reactor::TurnStatus::CheckpointReady(value)=>bytes(&value.state,reserve)?,reactor::TurnStatus::Faulted(value)=>bytes(value,reserve)?,reactor::TurnStatus::Idle|reactor::TurnStatus::MoreWork=>{}}
    bytes(&value.command_ingress.fault,reserve)?;
    match &value.cold_pair_ingress{reactor::ColdPairIngressStatus::Applied(value)=>{text(&value.baseline_frontier.document_id,reserve)?;text(&value.baseline_frontier.head_edit_id,reserve)?;bytes(&value.baseline_frontier.chain_sha256,reserve)?;bytes(&value.aggregate_sha256,reserve)?;},reactor::ColdPairIngressStatus::Fault(value)=>bytes(&value.fault,reserve)?,reactor::ColdPairIngressStatus::Idle|reactor::ColdPairIngressStatus::PageAccepted(_)|reactor::ColdPairIngressStatus::Backpressure(_)|reactor::ColdPairIngressStatus::Loading(_)=>{}}
    Ok(())
}
/// 📦️ Reserves both actual borrowed native document containers before returning their pair.
pub fn walk_document_pair(value:&codec::DocumentPair,reserve:&mut Reserve<'_>)->Result<(),ValueError>{bytes(&value.pack,reserve)?;bytes(&value.spr,reserve)}
fn rejection(value:&codec::SnapshotRejection,reserve:&mut Reserve<'_>)->Result<(),ValueError>{text(&value.message,reserve)?;bytes(&value.diagnostics,reserve)}
/// 📤️ Walks the actual SQLite file or its typed rejection fields before host lifting.
pub fn walk_snapshot_file_result(value:&codec::SnapshotFileResult,reserve:&mut Reserve<'_>)->Result<(),ValueError>{match value{codec::SnapshotFileResult::Done(value)=>{bytes(&value.bytes,reserve)?;bytes(&value.diagnostics,reserve)},codec::SnapshotFileResult::Rejected(value)=>rejection(value,reserve),codec::SnapshotFileResult::Pending(_)=>Ok(())}}
/// 📥️ Walks the actual reconstructed SQLite native payload or typed rejection before host lifting.
pub fn walk_snapshot_payload_result(value:&codec::SnapshotPayloadResult,reserve:&mut Reserve<'_>)->Result<(),ValueError>{match value{codec::SnapshotPayloadResult::Done(value)=>{bytes(&value.bytes,reserve)?;bytes(&value.diagnostics,reserve)},codec::SnapshotPayloadResult::Rejected(value)=>rejection(value,reserve),codec::SnapshotPayloadResult::Pending(_)=>Ok(())}}
/// 🧯️ Reserves the actual packed plugin failure as a native byte-list return.
pub fn walk_error(value:&types::PluginError,reserve:&mut Reserve<'_>)->Result<(),ValueError>{match value{types::PluginError::Fault(value)=>bytes(value,reserve),types::PluginError::OperationRefusal(code)if(1..=8).contains(code)=>Ok(()),types::PluginError::OperationRefusal(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical operation refusal code is invalid"))}}
