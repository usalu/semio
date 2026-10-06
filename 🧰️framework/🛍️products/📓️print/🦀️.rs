//! 🖨️ Print chart artifact with canonical mutation and inference execution.
extern crate semio_framework_os_kernel as protocol;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
#[path = "🧬️schema/📸️snapshot/🦀️.rs"]
pub mod snapshot;
#[path = "🧬️schema/🔀️diff/🦀️.rs"]
pub mod diff;
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
#[path = "🧬️schema/💡️inferences/🦀️.rs"]
pub mod inferences;

pub use snapshot::ChartSnapshot;
pub use diff::{ChartDiff, ChartEdit};
pub use mutations::ChangeChartValue;
pub use inferences::ChartInference;
pub const CHART_ARTIFACT_KIND:&str="s.print.chart";

pub fn chart_artifact_schema_descriptor()->semio_framework_schema_registry::ArtifactSchemaDescriptor{
    use semio_framework_schema_registry::{ArtifactSchemaDescriptor,FacetLeaves};
    let leaves=|rust,typescript,json_schema|FacetLeaves{rust,typescript,json_schema,graphql:"",proto:""};
    ArtifactSchemaDescriptor{id:"framework.print.chart",artifact:leaves(include_str!("🦀️.rs"),"",include_str!("🧬️schema/🔣️.json")),snapshot:leaves(include_str!("🧬️schema/📸️snapshot/🦀️.rs"),include_str!("🧬️schema/📸️snapshot/🟦️.ts"),include_str!("🧬️schema/📸️snapshot/🔣️.json")),diff:leaves(include_str!("🧬️schema/🔀️diff/🦀️.rs"),include_str!("🧬️schema/🔀️diff/🟦️.ts"),include_str!("🧬️schema/🔀️diff/🔣️.json")),mutations:leaves(include_str!("🧬️schema/🧬️mutations/🦀️.rs"),include_str!("🧬️schema/🧬️mutations/🟦️.ts"),include_str!("🧬️schema/🧬️mutations/🔣️.json"))}
}
pub fn chart_artifact_inference_descriptor()->semio_framework_schema_registry::ArtifactInferenceDescriptor{
    semio_framework_schema_registry::ArtifactInferenceDescriptor{id:"framework.print.chart.inference",inference:semio_framework_schema_registry::FacetLeaves{rust:include_str!("🧬️schema/💡️inferences/🦀️.rs"),typescript:include_str!("🧬️schema/💡️inferences/🟦️.ts"),json_schema:include_str!("🧬️schema/💡️inferences/🔣️.json"),graphql:"",proto:""}}
}

/// 🧷️ Publishes the real native service to the existing OS inference registry.
pub fn register_chart_inference() -> Result<(), semio_framework_plugin::ArtifactInferenceRegistrationError> {
    semio_framework_plugin::app::register_artifact_inference_service(chart_inference_service())
}

pub fn register_chart_artifact()->Result<(),String>{
    print_plugin().map(|_|()).map_err(|error|error.to_string())
}

pub fn chart_artifact_declaration()->Result<semio_framework_plugin::app::ArtifactDeclaration,semio_framework_plugin::app::ArtifactDefinitionError>{
    use semio_framework_plugin::app::{ArtifactDefinition,ArtifactIdentity,ArtifactCapability,ArtifactCapabilityKind,ArtifactIdentityClaim,ArtifactIdentityNamespace,ArtifactDeclaration};
    let capability=|id,kind,claim,descriptor:&str|ArtifactCapability::new(ArtifactIdentity::parse(id)?,kind).descriptor(descriptor.as_bytes().to_vec())?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(),claim)?);
    let definition=ArtifactDefinition::new(ArtifactIdentity::parse(CHART_ARTIFACT_KIND)?)
        .capability(capability("s.print.chart.schema",ArtifactCapabilityKind::schema(),"framework.print.chart",include_str!("🧬️schema/📸️snapshot/🔣️.json"))?)?
        .capability(capability("s.print.chart.inference",ArtifactCapabilityKind::inference(),"framework.print.chart.inference",include_str!("🧬️schema/💡️inferences/🔣️.json"))?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.print.chart.codec")?,ArtifactCapabilityKind::codec()).descriptor(include_str!("🧬️schema/📸️snapshot/🔣️.json").as_bytes().to_vec())?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(),"print.chart")?)?.claim(ArtifactIdentityClaim::codec_extension("print.chart","chart")?)?)?;
    let dialect=protocol::io_schema::Dialect{artifact_kind:CHART_ARTIFACT_KIND,standard:protocol::io_schema::StandardId("v1"),subset:protocol::io_schema::SubsetId("any")};
    ArtifactDeclaration::builder(definition).schema(chart_artifact_schema_descriptor()).inferences([chart_artifact_inference_descriptor()]).inference_services([chart_inference_service()]).document_codec_bare::<ChartSnapshot,ChangeChartValue>("print.chart",dialect).try_build()
}

/// 🚀️ The print domain boots through the existing transactional plugin assembly.
pub fn print_plugin()->Result<semio_framework_plugin::app::Plugin<semio_framework_plugin::app::NoPluginApp>,semio_framework_plugin::app::PluginAssemblyError>{
    use semio_framework_plugin::app::{Plugin,NoPluginApp,PluginAssemblyError};
    let declaration=chart_artifact_declaration().map_err(|error|PluginAssemblyError::new("print.chart.declaration",error.to_string()))?;
    let plugin=Plugin::<NoPluginApp>::builder("print").label("Print").version(env!("CARGO_PKG_VERSION")).package_id("semio:print").artifact(declaration).try_build()?;
    register_chart_inference().map_err(|error|PluginAssemblyError::new("print.chart.inference-registration",error.to_string()))?;
    semio_framework_schema_registry::register_referenced_schema_documents(&[include_str!("🧬️schema/🔣️.json"),include_str!("🧬️schema/🧬️mutations/🔣️.json")]);
    Ok(plugin)
}

pub fn chart_inference_service() -> semio_framework_plugin::ArtifactInferenceService {
    use semio_framework_plugin::{ArtifactInferenceService, ArtifactInferenceServiceMetadata, ArtifactInferencePayloadContract};
    ArtifactInferenceService::new(ArtifactInferenceServiceMetadata {
        owner: "print", artifact_kind: CHART_ARTIFACT_KIND, artifact_schema: "framework.print.chart", artifact_schema_version: 1, inference_schema: "framework.print.chart.inference", inference_schema_version: 1, algorithm_version: 1, policy_version: 1,
        payload: Some(ArtifactInferencePayloadContract { payload_schema_id: "framework.print.chart.inference.payload", input_schema: include_str!("🧬️schema/📸️snapshot/🔣️.json"), output_schema: include_str!("🧬️schema/💡️inferences/🔣️.json"), progress_unit: "chart-values", artifact_binding: None, commit: None }),
    }, execute_chart_inference)
}

pub fn execute_chart_inference(request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
    execute_chart_inference_controlled(request, &mut |_| {
        if semio_framework_plugin::app::inference_cancelled(request.cancellation_id)?{Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("artifact-inference.cancelled","chart inference cancelled"))}else{Ok(())}
    })
}

/// 🚦️ The controlled service binds progress, cancellation and allocation/work budgets.
pub fn execute_chart_inference_controlled(request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>, checkpoint: &mut dyn FnMut(u64) -> Result<(), semio_framework_plugin::ArtifactInferenceExecutionError>) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
    use semio_framework_value::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactInferenceExecution, ArtifactInferenceExecutionError as Error, WireArtifactInferenceCacheMode};
    if request.cancellation_id.trim().is_empty() || request.budgets.work_units == 0 || request.budgets.recursion_depth == 0 || request.canonical_payload.len() as u64 > request.budgets.allocation_bytes {
        return Err(Error::new("print.chart.inference.admission", "cancellation identity and sufficient non-zero budgets are required"));
    }
    if request.requested_cache_mode == WireArtifactInferenceCacheMode::Incremental { return Err(Error::new("print.chart.inference.cache", "incremental inference is not declared")); }
    checkpoint(0)?;
    let mut decode_options = protocol::PackDecodeOptions::default();
    decode_options.limits.max_total_alloc = request.budgets.allocation_bytes;
    let value = protocol::pack_rt::decode_wire_value_with_options(request.canonical_payload,&decode_options).map_err(|error| Error::new("print.chart.inference.decode", error.to_string()))?;
    let mut stack=vec![(&value,0u32)];
    let mut decode_work=0;
    while let Some((value,depth))=stack.pop(){
        if depth>request.budgets.recursion_depth{return Err(Error::new("print.chart.inference.depth","chart value nesting exceeds recursion budget"));}
        decode_work+=1;
        if decode_work>request.budgets.work_units{return Err(Error::new("print.chart.inference.work","chart values exceed work budget"));}
        checkpoint(decode_work)?;
        match value{semio_framework_value::DslValue::Array(items)=>stack.extend(items.iter().map(|value|(value,depth+1))),semio_framework_value::DslValue::Object(items)=>stack.extend(items.iter().map(|(_,value)|(value,depth+1))),_=>{}}
    }
    let snapshot = ChartSnapshot::from_value(value).map_err(|error| Error::new("print.chart.inference.snapshot", error.to_string()))?;
    let mut cancellation=None;
    let result = inferences::infer_chart_controlled(&snapshot, &mut |work| {
        if work.saturating_add(decode_work) > request.budgets.work_units { return Err("inference work budget exhausted".into()); }
        checkpoint(work.saturating_add(decode_work)).map_err(|error| {let message=error.to_string();cancellation=Some(error);message})
    });
    if let Some(error)=cancellation{return Err(error);}
    let inference=match result{
        Ok(tikz)=>ChartInference{tikz,diagnostics:Vec::new(),complete:true},
        Err(message)=>ChartInference{tikz:String::new(),diagnostics:vec![inferences::ChartDiagnostic{code:"print.chart.inference".into(),path:"chart".into(),message}],complete:false}
    };
    let canonical_payload = protocol::pack_rt::encode_wire_value(&inference.to_value());
    if canonical_payload.len() as u64 > request.budgets.allocation_bytes { return Err(Error::new("print.chart.inference.allocation", "output exceeds allocation budget")); }
    Ok(ArtifactInferenceExecution { canonical_payload, diagnostics: Vec::new(), validity: if inference.complete{"valid"}else{"invalid"}.into(), quality: "exact".into(), complete: inference.complete, actual_cache_mode: WireArtifactInferenceCacheMode::Cold })
}

#[cfg(test)]
#[path = "🧪️tests/🧬️chart-mutations/🦀️.rs"]
mod tests;
