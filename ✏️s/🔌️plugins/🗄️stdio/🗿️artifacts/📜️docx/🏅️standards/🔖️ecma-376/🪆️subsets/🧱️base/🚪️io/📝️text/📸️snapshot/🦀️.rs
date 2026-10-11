//! 📝️ Text representation codec surface for `stdio.docx` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type DocxSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use crate::standards::v_ecma_376::subsets::base::io::binary::snapshot::native;
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::*;
use crate::{
    standards::v_ecma_376::subsets::base::{schema::{refusal::{DocxError},vocabulary::{REL_TYPE_STYLES,STRICT_REL_TYPE_OFFICE_DOCUMENT,STRICT_REL_TYPE_STYLES}}},
    STDIO_DOCX_DOCUMENT_SCHEMA,
};
use framework_schema::ArtifactSchema;
use semio_framework_value::list::PagedList;
use semio_s_artifact_stdio_xml::schema::snapshot::{retained::RetainedXmlDocument, ownership::{retire_xml_document, retire_xml_document_with_frontier}, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, retained::RetainedOpcPackage, OpcPackage, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};
use std::collections::HashSet;

impl store::ArtifactDsl for DocxSnapshot {
 const EXTENSION:&'static str="docx";
 fn envelope_id()->&'static str{"stdio.docx"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{native::decode_text(text,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default())).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))}
 fn print_dsl(&self)->String{match native::encode_standalone(self,semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Text,&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default())).expect("DOCX native ownership admission"){semio_framework_os_kernel::io_schema::IoPayload::Text(text)=>text,semio_framework_os_kernel::io_schema::IoPayload::Binary(_)=>unreachable!()}}
}
}
pub use snapshot_codec::*;



impl crate::schema::snapshot::DocxSnapshot {
    pub fn part_text(&self, path: &str) -> Option<String> {
        let key = path.trim_start_matches('/');
        self.xml_part(key)
            .and_then(|part| part.document.materialize_exact().ok())
            .map(|document| semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text(&document))
            .or_else(|| self.opc.part_bytes(key).and_then(|bytes| String::from_utf8(bytes.to_vec_owner()).ok()))
    }
}
