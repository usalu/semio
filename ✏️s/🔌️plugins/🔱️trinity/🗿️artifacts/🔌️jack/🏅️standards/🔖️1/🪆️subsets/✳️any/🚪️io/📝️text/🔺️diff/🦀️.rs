//! 📝️ Physical text diff representation.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JackDiffText = String;

use crate::{Camera,JackContentChild};
use crate::standards::v1::subsets::any::schema::diff::JackDiff;
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct JackDiffDsl {
    schema: Option<String>,
    name: Option<String>,
    manifest_id: Option<Option<String>>,
    manifest: Option<crate::Manifest>,
    camera: Option<Camera>,
    content: Option<JackContentChild>,
    root_node_id: Option<Option<String>>,
    query: Option<String>,
}

impl From<&JackDiff> for JackDiffDsl { fn from(value:&JackDiff)->Self { Self {schema:value.schema.clone(), name:value.name.clone(), manifest_id:value.manifest_id.clone(), manifest:value.manifest.clone(), camera:value.camera.clone(), content:value.content.clone(), root_node_id:value.root_node_id.clone(), query:value.query.clone()} } }
impl From<JackDiffDsl> for JackDiff { fn from(value:JackDiffDsl)->Self { Self {schema:value.schema, name:value.name, manifest_id:value.manifest_id, manifest:value.manifest, camera:value.camera, content:value.content, root_node_id:value.root_node_id, query:value.query} } }
pub(crate) fn jack_diff_record_spec()->semio_framework_dsl_record::RecordSpec {JackDiffDsl::__dsl_spec()}
pub(crate) fn jack_diff_to_record(value:&JackDiff)->semio_framework_dsl_record::RecordValue {JackDiffDsl::from(value).__dsl_to_record()}
pub(crate) fn jack_diff_from_record(value:&semio_framework_dsl_record::RecordValue)->Result<JackDiff,semio_framework_diagnostic::TextError> {JackDiffDsl::__dsl_from_record(value).map(Into::into)}
impl protocol::DiffText for JackDiff {
 fn parse_diff(text:&str)->Result<Self,semio_framework_diagnostic::TextError> {let record=semio_framework_dsl_record::parse_exact(text,&jack_diff_record_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;jack_diff_from_record(&record)}
 fn print_diff(&self)->String {semio_framework_dsl_record::print(&jack_diff_to_record(self),&jack_diff_record_spec(),semio_framework_dsl_record::JoinMode::Document)}
}

