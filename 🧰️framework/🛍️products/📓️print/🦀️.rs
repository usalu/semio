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
pub const CHART_ARTIFACT_KIND:&str="framework.print.chart";

/// 📦️ Runtime channel for the print chart's authored schema documents. The host reads them and hands them to the plugin assembly, so no document is compiled into the crate. The schema registries keep `&'static str` leaves for the process lifetime, so [`ChartSchemaAssets::read`] leaks each document once.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ChartSchemaAssets{
    pub artifact:semio_framework_schema_registry::FacetLeaves,
    pub snapshot:semio_framework_schema_registry::FacetLeaves,
    pub diff:semio_framework_schema_registry::FacetLeaves,
    pub mutations:semio_framework_schema_registry::FacetLeaves,
    pub inferences:semio_framework_schema_registry::FacetLeaves,
    pub inference_output_schema:&'static str,
}

/// 🚫️ A schema document the host could not read.
#[derive(Debug)]
pub struct ChartSchemaAssetError{pub path:std::path::PathBuf,pub cause:std::io::Error}
impl std::fmt::Display for ChartSchemaAssetError{
    fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(formatter,"print chart schema document {}: {}",self.path.display(),self.cause)}
}
impl std::error::Error for ChartSchemaAssetError{}

impl ChartSchemaAssets{
    /// 📂️ Reads every document below `root`, the print product directory that holds this crate's `🦀️.rs`.
    pub fn read(root:&std::path::Path)->Result<Self,ChartSchemaAssetError>{
        Ok(Self{
            artifact:semio_framework_schema_registry::FacetLeaves{rust:read_document(root,"🦀️.rs")?,typescript:"",json_schema:read_document(root,"🧬️schema/🔣️.json")?,graphql:"",proto:""},
            snapshot:read_facet(root,"🧬️schema/📸️snapshot")?,
            diff:read_facet(root,"🧬️schema/🔀️diff")?,
            mutations:read_facet(root,"🧬️schema/🧬️mutations")?,
            inferences:read_facet(root,"🧬️schema/💡️inferences")?,
            inference_output_schema:read_document(root,"🚪️io/📝️text/💡️inferences/🔣️.json")?,
        })
    }
}

fn read_document(root:&std::path::Path,relative:&str)->Result<&'static str,ChartSchemaAssetError>{
    let path=root.join(relative);
    match std::fs::read_to_string(&path){Ok(text)=>Ok(&*Box::leak(text.into_boxed_str())),Err(cause)=>Err(ChartSchemaAssetError{path,cause})}
}
fn read_facet(root:&std::path::Path,directory:&str)->Result<semio_framework_schema_registry::FacetLeaves,ChartSchemaAssetError>{
    Ok(semio_framework_schema_registry::FacetLeaves{rust:read_document(root,&format!("{directory}/🦀️.rs"))?,typescript:read_document(root,&format!("{directory}/🟦️.ts"))?,json_schema:read_document(root,&format!("{directory}/🔣️.json"))?,graphql:"",proto:""})
}

pub fn chart_artifact_schema_descriptor(assets:&ChartSchemaAssets)->semio_framework_schema_registry::ArtifactSchemaDescriptor{
    semio_framework_schema_registry::ArtifactSchemaDescriptor{id:"framework.print.chart",artifact:assets.artifact,snapshot:assets.snapshot,diff:assets.diff,mutations:assets.mutations}
}
fn chart_artifact_inference_descriptor(assets:&ChartSchemaAssets)->semio_framework_schema_registry::ArtifactInferenceDescriptor{
    semio_framework_schema_registry::ArtifactInferenceDescriptor{id:"framework.print.chart.inference",inference:assets.inferences}
}

/// 🧷️ Publishes the real native service to the existing OS inference registry.
pub fn register_chart_inference(assets: &ChartSchemaAssets) -> Result<(), semio_framework_plugin::ArtifactInferenceRegistrationError> {
    semio_framework_plugin::app::register_artifact_inference_service(chart_inference_service(assets))
}

pub fn register_chart_artifact(assets:&ChartSchemaAssets)->Result<(),String>{
    print_plugin(assets).map(|_|()).map_err(|error|error.to_string())
}

pub fn chart_artifact_declaration(assets:&ChartSchemaAssets)->Result<semio_framework_plugin::app::ArtifactDeclaration,semio_framework_plugin::app::ArtifactDefinitionError>{
    use semio_framework_plugin::app::{ArtifactDefinition,ArtifactIdentity,ArtifactCapability,ArtifactCapabilityKind,ArtifactIdentityClaim,ArtifactIdentityNamespace,ArtifactDeclaration};
    let capability=|id,kind,claim,descriptor:&str|ArtifactCapability::new(ArtifactIdentity::parse(id)?,kind).descriptor(descriptor.as_bytes().to_vec())?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(),claim)?);
    let definition=ArtifactDefinition::new(ArtifactIdentity::parse(CHART_ARTIFACT_KIND)?)
        .capability(capability("framework.print.chart.schema",ArtifactCapabilityKind::schema(),"framework.print.chart",assets.snapshot.json_schema)?)?
        .capability(capability("framework.print.chart.inference",ArtifactCapabilityKind::inference(),"framework.print.chart.inference",assets.inferences.json_schema)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("framework.print.chart.codec")?,ArtifactCapabilityKind::codec()).descriptor(assets.snapshot.json_schema.as_bytes().to_vec())?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(),"print.chart")?)?.claim(ArtifactIdentityClaim::codec_extension("print.chart","chart")?)?)?;
    let dialect=semio_framework_artifact_reference::Dialect{artifact_kind:CHART_ARTIFACT_KIND,standard:semio_framework_artifact_reference::StandardId("v1"),subset:semio_framework_artifact_reference::SubsetId("any")};
    ArtifactDeclaration::builder(definition).schema(chart_artifact_schema_descriptor(assets)).inferences([chart_artifact_inference_descriptor(assets)]).inference_services([chart_inference_service(assets)]).document_codec_bare::<ChartSnapshot,ChangeChartValue>("print.chart",dialect).try_build()
}

/// 🚀️ The print domain boots through the existing transactional plugin assembly.
pub fn print_plugin(assets:&ChartSchemaAssets)->Result<semio_framework_plugin::app::Plugin<semio_framework_plugin::app::NoPluginApp>,semio_framework_plugin::app::PluginAssemblyError>{
    use semio_framework_plugin::app::{Plugin,NoPluginApp,PluginAssemblyError};
    use semio_framework_plugin::app::PluginBuilder;
    let declaration=chart_artifact_declaration(assets).map_err(|error|PluginAssemblyError::new("print.chart.declaration",error.to_string()))?;
    let plugin=PluginBuilder::<_, NoPluginApp>::new("print").label("Print").version(env!("CARGO_PKG_VERSION")).package_id("semio:print").artifact(declaration).try_build()?;
    register_chart_inference(assets).map_err(|error|PluginAssemblyError::new("print.chart.inference-registration",error.to_string()))?;
    semio_framework_schema_registry::register_referenced_schema_documents(&[assets.artifact.json_schema,assets.mutations.json_schema]);
    Ok(plugin)
}

pub fn chart_inference_service(assets: &ChartSchemaAssets) -> semio_framework_plugin::ArtifactInferenceService {
    use semio_framework_plugin::{ArtifactInferenceService, ArtifactInferenceServiceMetadata, ArtifactInferencePayloadContract};
    ArtifactInferenceService::new(ArtifactInferenceServiceMetadata {
        owner: "print", artifact_kind: CHART_ARTIFACT_KIND, artifact_schema: "framework.print.chart", artifact_schema_version: 1, inference_schema: "framework.print.chart.inference", inference_schema_version: 1, algorithm_version: 1, policy_version: 1,
        payload: Some(ArtifactInferencePayloadContract { payload_schema_id: "framework.print.chart.inference.payload", input_schema: assets.snapshot.json_schema, output_schema: assets.inference_output_schema, progress_unit: "chart-values", artifact_binding: None, commit: None }),
    }, io::binary::inferences::execute_chart_inference,chart_inference_demands)
}


#[cfg(test)]
#[path = "🧪️tests/🧬️chart-mutations/🦀️.rs"]
mod tests;

/// ♻️ This synchronous executable retains no callback child between invocations.
fn chart_inference_demands(_request:&semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>,_copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand::default())}
