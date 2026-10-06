//! 📝️ Text representation codec surface for `stdio.png` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PngSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod pack_codec {
use crate::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot;
use crate::store;
use crate::standards::v1_2::subsets::any::io::binary::snapshot::owned_codecs::*;
impl store::ArtifactDsl for PngSnapshot{
 const EXTENSION:&'static str="png";
 fn envelope_id()->&'static str{"stdio.png"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(),semio_framework_diagnostic::TextSpan::at(1,1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PNG logical Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(body,&Self::__dsl_spec(),&semio_framework_dsl_record::ParseOptions::default())?)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared PNG logical envelope");store::semio_format::wrap_text(&envelope,&body)}
}
}
