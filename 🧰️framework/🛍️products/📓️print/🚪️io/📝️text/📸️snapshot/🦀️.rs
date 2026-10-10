//! 📝️ Authored chart text representation.
use crate::ChartSnapshot;
use semio_framework_value::DslValue;
semio_framework_value_derive::value_codec!{#[value(deny_unknown_fields)]struct ChartSnapshot{pub chart:DslValue}}
semio_framework_dsl_record_derive::record_binding!{struct ChartSnapshot{pub chart:DslValue}}
use protocol as store;
use semio_framework_diagnostic::{TextError,TextSpan};
use semio_framework_value::ValueRefusalKind as K;
impl store::ArtifactDsl for ChartSnapshot{
 const EXTENSION:&'static str="chart";
 fn envelope_id()->&'static str{"print.chart"}
 fn parse_dsl(text:&str)->Result<Self,TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|TextError::from_value_error(error.into_value_error(),TextSpan::at(1,1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(TextError::new(K::InvalidValue,"Chart logical Text envelope mismatch",TextSpan::at(1,1)))}Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(body,&Self::__dsl_spec(),&Default::default())?)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared Chart envelope");store::semio_format::wrap_text(&envelope,&body)}
}
