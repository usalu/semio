//! 🧳️ Text DSL codec of the persisted snapshot root.

#![allow(unused_imports)]

use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use crate::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_dsl_record::DslField;
use crate::standards::v2_0::subsets::any::io::binary::snapshot::owned_pack::*;
use crate::standards::v2_0::subsets::any::io::snapshot_codecs::pack_codec::*;
impl pack::snapshot::ArtifactDsl for GltfSnapshot{
 const EXTENSION:&'static str="gltf";
 fn envelope_id()->&'static str{"stdio.gltf"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let body=match store::semio_format::split_text_preamble(text){Ok((envelope,body))=>{if !envelope.matches_identity("stdio.gltf",store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("GLTF logical text identity differs").to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))}body},Err(_)=>text};let record=semio_framework_dsl_record::parse_exact(body,&Snapshot::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;Snapshot::__dsl_from_record(&record).map(Into::into)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&record(self),&Snapshot::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.gltf",store::semio_format::Component::Dsl,1).expect("valid GLTF owned identity");store::semio_format::wrap_text(&envelope,&body)}
}
