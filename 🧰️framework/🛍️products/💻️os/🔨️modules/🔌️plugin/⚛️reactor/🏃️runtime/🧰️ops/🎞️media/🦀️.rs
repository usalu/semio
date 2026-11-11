//! 🎞️ Media consumption, export and production operations of the runtime.

use semio_plugin_decl_app::{ArtifactApp, MediaArtifactDescriptor, MediaArtifactError, MediaConsumption};
use semio_plugin_decl_media::{ArtifactMediaExportHandle, ArtifactMediaExportPoll};
use semio_framework::{Fault, FaultFrom, MediaError};
use semio_framework_value::{FromValue, ToValue};
use semio_plugin_document_media::lifted_app::MediaArtifact;
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_plugin_program::PluginProgram;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use serde_json::Value;
use crate::ops_wire_limits::{find_instance, runtime_instance_cell};
use semio_plugin_reactor_app::app_runtime_helpers::plugin_internal_fault;
use semio_plugin_runtime_instances::inst_wire_frames::{with_instances_mut, with_instances_original_mut};
/// 🎞️ Answers how one media import settled: applied at once, a stepped archive load pending for the host to poll, still pending until the next funded turn (app consumption or archive admission), or refused.
pub enum MediaImport{Applied,Loading(protocol::DocumentArchiveLoadStatus),Pending,Refused(Fault)}
/// 🎞️ WIT `consume-media` glue — decodes the incoming `media-artifact` (`descriptor-json` + `data`)
/// and dispatches to `PluginApp::consume_media`. A whole document parks in the owner's resident `archive` and is admitted as a stepped archive load
/// under `operation` through the owner's resident `admission` (the host sequence of the import, `📓️api-stepped-document-load.md` §4); re-entry with a parked archive resumes its admission.
pub async fn plugin_consume_media<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64, port_id: &str, descriptor: &MediaArtifactDescriptor, data: &mut Vec<u8>, archive: &mut Option<protocol::DocumentArchivePack>, admission: &mut semio_plugin_document_media::OriginalDocumentArchiveAdmission, cx:&mut semio_framework_job::StepContext<'_>) -> Result<MediaImport, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    if archive.is_none() {
        let mut artifact = Some(MediaArtifact { descriptor: descriptor.clone(), data: std::mem::take(data) });
        let consumed = with_instances_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            Ok(::semio_framework_async::poll::resolve_ready(instance.app.consume_media(port_id, &mut artifact, cx)))
        })
        .await;
        if let Some(unconsumed) = artifact { *data = unconsumed.data; }
        match consumed {
            Err(fault) => return Ok(MediaImport::Refused(fault)),
            Ok(Ok(Some(MediaConsumption::Applied))) => return Ok(MediaImport::Applied),
            Ok(Ok(Some(MediaConsumption::DocumentLoad(pack)))) => *archive = Some(pack),
            Ok(Ok(None)) => return Ok(MediaImport::Pending),
            Ok(Err(semio_plugin_decl_app::MediaArtifactOperationFailure::Domain(error))) => return Ok(MediaImport::Refused(semio_plugin_decl_app::media_artifact_fault(error))),
            Ok(Err(semio_plugin_decl_app::MediaArtifactOperationFailure::Original(failure))) => return Err(failure.into()),
        }
    }
    let pack = archive.as_mut().expect("parked media archive is resident");
    let admitted = with_instances_original_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        Ok(instance.app.begin_document_archive_load(operation, pack, admission, cx)?)
    })
    .await?;
    Ok(if admitted { MediaImport::Loading(protocol::DocumentArchiveLoadStatus { operation, state: protocol::DocumentArchiveLoadState::Pending, completed: 0, total: 0, ahead: 0, fault: Vec::new() }) } else { MediaImport::Pending })
}
/// 🎬️ Interactive workflow entry: submits without waiting for the exporter to finish.
pub async fn plugin_submit_media_export<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, port_id: &str, expected_parent_document_id: &str, expected_base_revision: u64,checkpoint_id:Option<&str>,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let cell = runtime_instance_cell(runtime, instance_id)?;
    let mut instance = cell.host_instance().ok_or_else(|| plugin_internal_fault("media export original instance is busy or poisoned"))?;
    instance.app.submit_media_export_checked(port_id, expected_parent_document_id, expected_base_revision,checkpoint_id,receiver,cx).await
}
/// 📡️ Interactive workflow entry receives the actual initiating context and its complete failure.
pub async fn plugin_poll_media_export<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, handle: semio_plugin_decl_media::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<ArtifactMediaExportPoll>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let cell = runtime_instance_cell(runtime, instance_id)?;
    let mut instance = cell.host_instance().ok_or_else(|| plugin_internal_fault("media export original instance is busy or poisoned"))?;
    instance.app.poll_media_export(handle,receiver,cx).await
}
/// 🛑️ Interactive workflow entry: cancels the exact live operation named by the handle.
pub async fn plugin_cancel_media_export<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
    let cell = runtime_instance_cell(runtime, instance_id).map_err(|error| MediaError::Payload(handle.parent_document_id.clone(), error.message))?;
    let mut instance = cell.host_instance().ok_or_else(|| MediaError::Payload(handle.parent_document_id.clone(), format!("instance busy or poisoned: {instance_id}")))?;
    instance.app.cancel_media_export(handle).await
}
pub async fn plugin_retain_media_export_result<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,handle:semio_plugin_decl_media::ArtifactMediaExportHandleLoan<'_>,source:&mut Option<ArtifactMediaExportPoll>,status:&mut Option<semio_plugin_decl_media::ArtifactMediaExportPublishedStatus>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let cell=runtime_instance_cell(runtime,instance_id)?;let mut instance=cell.host_instance().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"download publication requires its original instance loan"))?;instance.app.retain_media_export_result(handle,source,status,cx).await
}
pub async fn plugin_take_media_export_chunk<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,handle:semio_plugin_decl_media::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<Vec<u8>>,terminal:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let cell=runtime_instance_cell(runtime,instance_id)?;let mut instance=cell.host_instance().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"media chunk requires its original instance loan"))?;instance.app.take_media_export_chunk(handle,receiver,terminal,cx).await
}
/// 🧵️ Takes one already-produced bounded action-download chunk without blocking another instance.
pub async fn plugin_take_segmented_download_chunk<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation_id: u64) -> Result<Option<Vec<u8>>, Fault> {
    let cell = runtime_instance_cell(runtime, instance_id)?;
    let mut instance = cell.host_instance().ok_or_else(|| plugin_internal_fault(format!("instance busy or poisoned: {instance_id}")))?;
    instance.app.take_segmented_download_chunk(operation_id).await
}
/// 🎞️ WIT `produce-media` glue — dispatches to `PluginApp::produce_media` and encodes the result back
/// into `(descriptor-json, data)` for the `media-artifact` WIT record. `_request_json` is reserved for
/// future parameterized requests; unused by the SDK default (see `PluginApp::produce_media`).
pub async fn plugin_produce_media<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, port_id: &str, _request_json: &str, owner: &mut semio_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<MediaArtifact>,semio_plugin_decl_app::MediaArtifactOperationFailure> {
    let cell = runtime_instance_cell(runtime, instance_id).map_err(MediaArtifactError::Import)?;
    let mut instance = cell.host_instance().ok_or_else(|| MediaArtifactError::Value(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original media instance is busy or poisoned")))?;
    instance.app.produce_media(port_id, owner,cx).await
}
