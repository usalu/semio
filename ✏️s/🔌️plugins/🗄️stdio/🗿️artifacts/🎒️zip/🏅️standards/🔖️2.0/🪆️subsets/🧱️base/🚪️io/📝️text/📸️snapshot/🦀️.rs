//! 📝️ Text representation codec surface for `stdio.zip` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ZipSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v2_0::subsets::base::schema::snapshot::*;
use crate::STDIO_ZIP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl store::ArtifactDsl for ZipSnapshot {
    const EXTENSION:&'static str="zip";
    fn envelope_id()->&'static str{"stdio.zip"}
    fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
        let body=store::semio_format::split_text_preamble(text).map(|(_,body)|body).unwrap_or(text);
        let record=semio_framework_dsl_record::parse(body,&Self::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self)->String{
        let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);
        let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.zip",store::semio_format::Component::Dsl,1).expect("valid ZIP snapshot identity");
        store::semio_format::wrap_text(&envelope,&body)
    }
}
}
pub use snapshot_codec::*;
