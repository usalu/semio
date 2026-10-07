//! 📝️ Text representation codec surface for `stdio.md` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type MdSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod pack_codec {
use crate::standards::v_commonmark::subsets::any::schema::snapshot::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::NativeEncodeControl;
use semio_framework_value::NativeDecodeControl;
use crate::standards::v_commonmark::subsets::any::io::binary::snapshot::owned_pack::*;
impl store::ArtifactDsl for MdSnapshot{
 const EXTENSION:&'static str="md";
 fn envelope_id()->&'static str{"stdio.md"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let body=match store::semio_format::split_text_preamble(text){Ok((envelope,body))=>{if !envelope.matches_identity("stdio.md",store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("CommonMark text identity differs").to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))}body},Err(_)=>text};let record=semio_framework_dsl_record::parse(body,&Snapshot::__dsl_spec(),&semio_framework_dsl_record::ParseOptions::default())?;let snapshot=Snapshot::__dsl_from_record(&record)?;MdSnapshot::try_from(snapshot).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&Snapshot::from(self).__dsl_to_record(),&Snapshot::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.md",store::semio_format::Component::Dsl,1).expect("valid CommonMark identity");store::semio_format::wrap_text(&envelope,&body)}
}
}

impl crate::MdSnapshot {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_text(text: &str) -> Self {
        let blocks = crate::standards::v_commonmark::subsets::any::io::import::deserializers::parse_markdown_blocks(text);
        Self { schema: crate::STDIO_MD_DOCUMENT_SCHEMA.into(), blocks }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_text(&self) -> String {
        crate::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks(&self.blocks)
    }
}
