//! 📜️ Owned plugin descriptor registration data, independent of mounted app execution.
use semio_framework::{AssetDeclaration,ExecutionMode,ExtensionPointDeclaration};
use semio_framework::kernel::{ActivationEvent,CapabilityRequest,QuotaSchema};
/// 🧺️ Consumed descriptor values transferred once into the assembled plugin.
pub struct PluginDescriptorParts{
pub activation_events: Vec<ActivationEvent>,
pub capability_requests: Vec<CapabilityRequest>,
pub extension_points: Vec<ExtensionPointDeclaration>,
pub execution: ExecutionMode,
pub quotas: QuotaSchema,
pub assets: Vec<AssetDeclaration>,
}
/// 📇️ Authoring data retained by a fluent plugin builder until assembly.
#[derive(Default)]
pub struct PluginDescriptorRegistration{
activation_events: Vec<ActivationEvent>,
capability_requests: Vec<CapabilityRequest>,
extension_points: Vec<ExtensionPointDeclaration>,
execution: ExecutionMode,
quotas: QuotaSchema,
assets: Vec<AssetDeclaration>,
}
impl PluginDescriptorRegistration{
pub fn activation(&mut self,event:ActivationEvent){if !self.activation_events.contains(&event){self.activation_events.push(event)}}
pub fn requests(&mut self,request:CapabilityRequest){if !self.capability_requests.contains(&request){self.capability_requests.push(request)}}
pub fn extension_point(&mut self,declaration:ExtensionPointDeclaration){if !self.extension_points.contains(&declaration){self.extension_points.push(declaration)}}
pub fn execution(&mut self,mode:ExecutionMode){self.execution=mode}
pub fn quota(&mut self,schema:QuotaSchema){self.quotas=merge_quota_schema(self.quotas,schema)}
pub fn asset(&mut self,declaration:AssetDeclaration){if !self.assets.contains(&declaration){self.assets.push(declaration)}}
pub fn into_parts(self)->PluginDescriptorParts{PluginDescriptorParts{activation_events:self.activation_events,capability_requests:self.capability_requests,extension_points:self.extension_points,execution:self.execution,quotas:self.quotas,assets:self.assets}}
}
/// 📏️ Incoming explicit quotas override only the corresponding existing fields.
fn merge_quota_schema(base: QuotaSchema, incoming: QuotaSchema) -> QuotaSchema {
    QuotaSchema {
        memory_bytes: incoming.memory_bytes.or(base.memory_bytes),
        fuel_per_turn: incoming.fuel_per_turn.or(base.fuel_per_turn),
        turn_deadline_ms: incoming.turn_deadline_ms.or(base.turn_deadline_ms),
        tables: incoming.tables.or(base.tables),
        mailbox_len: incoming.mailbox_len.or(base.mailbox_len),
        message_bytes: incoming.message_bytes.or(base.message_bytes),
        outstanding_requests: incoming.outstanding_requests.or(base.outstanding_requests),
        timers: incoming.timers.or(base.timers),
        storage_bytes: incoming.storage_bytes.or(base.storage_bytes),
        network_bytes_per_min: incoming.network_bytes_per_min.or(base.network_bytes_per_min),
        ui_nodes: incoming.ui_nodes.or(base.ui_nodes),
        patch_bytes: incoming.patch_bytes.or(base.patch_bytes),
        patch_hz: incoming.patch_hz.or(base.patch_hz),
        blob_resident_bytes: incoming.blob_resident_bytes.or(base.blob_resident_bytes),
        gpu_ms_per_frame: incoming.gpu_ms_per_frame.or(base.gpu_ms_per_frame),
        background_ms_per_min: incoming.background_ms_per_min.or(base.background_ms_per_min),
        log_bytes_per_min: incoming.log_bytes_per_min.or(base.log_bytes_per_min),
    }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
