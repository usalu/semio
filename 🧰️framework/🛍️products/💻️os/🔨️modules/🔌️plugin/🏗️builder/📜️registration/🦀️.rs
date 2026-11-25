//! 📜️ Owned plugin descriptor registration data, independent of mounted app execution.
use semio_framework::{AssetDeclaration,ComponentDeclaration,ExecutionMode,ExtensionPointDeclaration};
use semio_framework::kernel::{ActivationEvent,CapabilityRequest,QuotaSchema};
/// 🧺️ Consumed descriptor values transferred once into the assembled plugin.
pub struct PluginDescriptorParts{
pub activation_events: Vec<ActivationEvent>,
pub capability_requests: Vec<CapabilityRequest>,
pub extension_points: Vec<ExtensionPointDeclaration>,
pub execution: ExecutionMode,
pub quotas: QuotaSchema,
pub assets: Vec<AssetDeclaration>,
pub components: Vec<ComponentDeclaration>,
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
components: Vec<(String,usize)>,
}
impl PluginDescriptorRegistration{
pub fn activation(&mut self,event:ActivationEvent){if !self.activation_events.contains(&event){self.activation_events.push(event)}}
pub fn requests(&mut self,request:CapabilityRequest){if !self.capability_requests.contains(&request){self.capability_requests.push(request)}}
pub fn extension_point(&mut self,declaration:ExtensionPointDeclaration){if !self.extension_points.contains(&declaration){self.extension_points.push(declaration)}}
pub fn execution(&mut self,mode:ExecutionMode){self.execution=mode}
pub fn quota(&mut self,schema:QuotaSchema){self.quotas=merge_quota_schema(self.quotas,schema)}
pub fn asset(&mut self,declaration:AssetDeclaration){if !self.assets.contains(&declaration){self.assets.push(declaration)}}
pub fn component(&mut self,id:String,first_app:usize){self.components.push((id,first_app))}
pub fn declares_components(&self)->bool{!self.components.is_empty()}
pub fn into_parts(self,app_ids:&[String])->Result<PluginDescriptorParts,String>{let components=component_declarations(self.components,app_ids)?;Ok(PluginDescriptorParts{activation_events:self.activation_events,capability_requests:self.capability_requests,extension_points:self.extension_points,execution:self.execution,quotas:self.quotas,assets:self.assets,components})}
}
/// 🧩️ Every app registered from one `component` call to the next belongs to that component; a package that declares components
/// assigns each app to exactly one, by a unique non-empty id.
fn component_declarations(starts:Vec<(String,usize)>,app_ids:&[String])->Result<Vec<ComponentDeclaration>,String>{
    if starts.is_empty(){return Ok(Vec::new())}
    if starts[0].1!=0{return Err(format!("app {:?} is registered before the first component",app_ids[0]))}
    let mut declarations:Vec<ComponentDeclaration>=Vec::with_capacity(starts.len());
    for (index,(id,first)) in starts.iter().enumerate(){
        let end=starts.get(index+1).map_or(app_ids.len(),|next|next.1);
        if id.is_empty()||!id.bytes().all(|byte|byte.is_ascii_lowercase()||byte.is_ascii_digit()||byte==b'-'||byte==b'_'){return Err(format!("component id {id:?} must be non-empty lowercase ascii, digits, '-' or '_'"))}
        if declarations.iter().any(|declaration|&declaration.id==id){return Err(format!("component {id:?} is declared twice"))}
        if *first>=end{return Err(format!("component {id:?} serves no app"))}
        declarations.push(ComponentDeclaration{id:id.clone(),apps:app_ids[*first..end].to_vec()});
    }
    Ok(declarations)
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
