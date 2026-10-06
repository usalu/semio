//! 📝️ Text representation codec surface for `stdio.obj` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ObjSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod pack_codec {
use crate::standards::v3_0::subsets::any::schema::snapshot::ObjSnapshot;
use semio_framework_diagnostic::{TextError,TextSpan};
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v3_0::subsets::any::io::binary::snapshot::native_pack::*;
impl store::ArtifactDsl for ObjSnapshot{
 const EXTENSION:&'static str="obj";
 fn envelope_id()->&'static str{"stdio.obj"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(error.to_string()).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;if !envelope.matches_identity("stdio.obj",store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("owned snapshot Text identity differs").to_string(),semio_framework_diagnostic::TextSpan::at(1,1)));}let record=semio_framework_dsl_record::parse_exact(body,&ObjSnapshot::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;ObjSnapshot::__dsl_from_record(&record)}
 fn print_dsl(&self)->String{let snapshot=self;let body=semio_framework_dsl_record::print(&snapshot.__dsl_to_record(),&ObjSnapshot::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.obj",store::semio_format::Component::Dsl,1).expect("valid owned snapshot identity");store::semio_format::wrap_text(&envelope,&body)}
}
}
