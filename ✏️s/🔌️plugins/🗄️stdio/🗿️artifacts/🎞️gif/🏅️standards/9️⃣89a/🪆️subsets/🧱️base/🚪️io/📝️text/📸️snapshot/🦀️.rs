//! 📝️ Text representation codec surface for `stdio.gif` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type GifSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v89a::subsets::any::schema::snapshot::*;
use framework_schema::ArtifactSchema;

impl store::ArtifactDsl for GifSnapshot {
    const EXTENSION: &'static str = "gif";
    fn envelope_id() -> &'static str {
        STDIO_GIF89A_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
        let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
        if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "GIF owned Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)));}
        Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(body,&Self::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits{max_bytes:32*1024*1024,..semio_framework_diagnostic::Limits::default()},..semio_framework_dsl_record::ParseOptions::default()})?)
    }
    fn print_dsl(&self)->String{
        let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);
        let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared GIF envelope");store::semio_format::wrap_text(&envelope,&body)
    }
}
}
pub use snapshot_codec::*;
